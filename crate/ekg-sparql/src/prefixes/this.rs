#[cfg(not(feature = "_rdfox"))]
use lazy_static::__Deref;
#[cfg(feature = "_rdfox")]
use {
    crate::prefixes::handle::CPrefixesHandle,
    std::sync::{Arc, Mutex},
    std::{ffi::CString, ops::Deref},
};
use {
    crate::prefixes::{PrefixesBuilder, PrefixesDeclareResult},
    ekg_identifier::{NS_OWL, NS_RDF, NS_RDFS, NS_XSD, Namespace, TBoxNamespaceIRI},
    ekg_metadata::{Class, Predicate},
    ekg_util::log::LOG_TARGET_DATABASE,
    std::{
        collections::HashSet,
        fmt::{Display, Formatter},
    },
};

/// A set of namespace prefixes, used to declare consts in SPARQL queries.
/// The prefixes are stored in a vector of [`Namespace`] structs.
/// The `_rdfox` feature uses the `rdfox` library to manage the prefixes and
/// also uses the Rust vector to store the consts because RDFox does not
/// have an API that allows the retrieval of all the prefixes from a given
/// instance of `CPrefixes`.
#[derive(Debug, Clone)]
pub struct Prefixes {
    #[cfg(feature = "_rdfox")]
    inner:    Arc<Mutex<CPrefixesHandle>>,
    prefixes: HashSet<Namespace>,
}

unsafe impl Sync for Prefixes {}

unsafe impl Send for Prefixes {}

impl Eq for Prefixes {}

impl PartialEq for Prefixes {
    #[cfg(feature = "_rdfox")]
    fn eq(&self, _other: &Self) -> bool { Arc::ptr_eq(&self.inner, &_other.inner) }

    #[cfg(not(feature = "_rdfox"))]
    fn eq(&self, _other: &Self) -> bool { self.prefixes == _other.prefixes }
}

impl Iterator for Prefixes {
    type Item = Namespace;

    fn next(&mut self) -> Option<Self::Item> { self.prefixes.iter().next().cloned() }
}

impl ExactSizeIterator for Prefixes {
    fn len(&self) -> usize { self.prefixes.len() }
}

/// Show the consts in SPARQL format
impl Display for Prefixes {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for namespace in self.prefixes.iter() {
            writeln!(f, "{}", namespace.as_sparql_prefix())?;
        }
        Ok(())
    }
}

impl From<Vec<Namespace>> for Prefixes {
    fn from(namespaces: Vec<Namespace>) -> Self {
        Self::builder()
            .declare_namespaces(&namespaces)
            .build()
            .unwrap()
    }
}

impl From<&Vec<Namespace>> for Prefixes {
    fn from(namespaces: &Vec<Namespace>) -> Self {
        Self::builder()
            .declare_namespaces(namespaces)
            .build()
            .unwrap()
    }
}

impl Prefixes {
    pub fn builder() -> PrefixesBuilder { PrefixesBuilder::default_builder() }

    pub fn empty() -> Result<Self, ekg_error::Error> {
        #[allow(unused_mut)]
        let mut prefixes = Self {
            #[cfg(feature = "_rdfox")]
            inner:                            Arc::new(Mutex::new(CPrefixesHandle::new().unwrap())),
            prefixes:                         HashSet::new(),
        };

        #[cfg(feature = "_rdfox")]
        {
            let mut c_prefixes = prefixes.inner.lock().unwrap(); // TODO: error handling
            rdfox_sys::database_call!(
                "Registering default prefixes",
                rdfox_sys::CPrefixes_newDefaultPrefixes(&mut c_prefixes.cast_mut())
            )?;
        }
        Ok(prefixes)
    }

    /// Return the default consts: `RDF`, `RDFS`, `OWL` and `XSD`
    pub fn try_default() -> Result<Self, ekg_error::Error> {
        Self::builder()
            .declare(NS_RDF.deref())
            .declare(NS_RDFS.deref())
            .declare(NS_OWL.deref())
            .declare(NS_XSD.deref())
            .build()
    }

    pub fn declare_namespaces(
        &mut self,
        namespaces: Vec<Namespace>,
    ) -> Result<Self, ekg_error::Error> {
        for namespace in namespaces {
            self.declare_namespace(&namespace)?;
        }
        Ok(self.clone())
    }

    #[cfg(not(feature = "_rdfox"))]
    pub fn declare_namespace(
        &mut self,
        namespace: &Namespace,
    ) -> Result<PrefixesDeclareResult, ekg_error::Error> {
        self._declare_namespace(namespace)
    }

    fn _declare_namespace(
        &mut self,
        namespace: &Namespace,
    ) -> Result<PrefixesDeclareResult, ekg_error::Error> {
        if !self.prefixes.insert(namespace.clone()) {
            tracing::trace!("Declaring PREFIX {namespace} (already declared)");
            return Ok(PrefixesDeclareResult::PREFIXES_NO_CHANGE);
        }
        tracing::trace!(target: LOG_TARGET_DATABASE, "Declaring PREFIX {namespace}");
        Ok(PrefixesDeclareResult::PREFIXES_DECLARED_NEW)
    }

    #[cfg(feature = "_rdfox")]
    pub fn declare_namespace(
        &mut self,
        namespace: &Namespace,
    ) -> Result<PrefixesDeclareResult, ekg_error::Error> {
        let result = self._declare_namespace(namespace)?;
        if result != PrefixesDeclareResult::PREFIXES_DECLARED_NEW {
            return Ok(result);
        }
        let c_name = CString::new(namespace.name.as_str()).unwrap();
        let c_iri = CString::new(namespace.iri.as_str()).unwrap();
        let mut result = rdfox_sys::CPrefixes_DeclareResult::PREFIXES_NO_CHANGE;
        let mut c_prefixes = self.inner.lock().unwrap(); // TODO: error handling
        rdfox_sys::database_call!(
            format!(
                "Registering PREFIX {} <{}>",
                namespace.name.as_str(),
                namespace.iri.as_str(),
            )
            .as_str(),
            rdfox_sys::CPrefixes_declarePrefix(
                c_prefixes.cast_mut(),
                c_name.as_ptr(),
                c_iri.as_ptr(),
                &mut result
            )
        )?;
        assert!(
            self.get_namespace(&namespace.name).is_some(),
            "Namespace not found"
        );
        match result {
            rdfox_sys::CPrefixes_DeclareResult::PREFIXES_INVALID_PREFIX_NAME => {
                tracing::error!(
                    target: LOG_TARGET_DATABASE,
                    "Invalid prefix name \"{}\" while registering namespace <{}>",
                    namespace.name.as_str(),
                    namespace.iri.as_str()
                );
                Err(ekg_error::Error::InvalidPrefixName)
            },
            rdfox_sys::CPrefixes_DeclareResult::PREFIXES_DECLARED_NEW => {
                tracing::trace!(
                    target: LOG_TARGET_DATABASE,
                    "Registered  PREFIX {} <{}>",
                    namespace.name.as_str(),
                    namespace.iri.as_str()
                );
                Ok(result.into())
            },
            rdfox_sys::CPrefixes_DeclareResult::PREFIXES_NO_CHANGE => {
                tracing::error!(
                    target: LOG_TARGET_DATABASE,
                    "Registered {namespace} twice"
                );
                Ok(result.into())
            },
            _ => {
                tracing::error!(
                    target: LOG_TARGET_DATABASE,
                    "Result of registering prefix {namespace} is {:?}",
                    result
                );
                Ok(result.into())
            },
        }
    }

    pub fn declare(
        &mut self,
        name: &str,
        iri: TBoxNamespaceIRI,
    ) -> Result<PrefixesDeclareResult, ekg_error::Error> {
        self.declare_namespace(&Namespace::declare(name, iri)?)
    }

    pub fn add_namespace(&mut self, namespace: &Namespace) -> Result<Self, ekg_error::Error> {
        let _ = self.declare_namespace(namespace);
        Ok(self.clone())
    }

    pub fn add_class(&mut self, clazz: &Class) -> Result<Self, ekg_error::Error> {
        self.add_namespace(&clazz.namespace)
    }

    pub fn add_predicate(&mut self, predicate: &Predicate) -> Result<Self, ekg_error::Error> {
        self.add_namespace(predicate.namespace)
    }

    pub fn for_each_namespace_do<F: FnMut(&str, &Namespace) -> Result<(), E>, E>(
        &self,
        mut f: F,
    ) -> Result<(), E> {
        for namespace in self.prefixes.iter() {
            f(namespace.name.as_str(), namespace)?;
        }
        Ok(())
    }

    pub fn get_namespace(&self, prefix: &str) -> Option<&Namespace> {
        self.prefixes.iter().find(|ns| ns.name == prefix)
    }
}
