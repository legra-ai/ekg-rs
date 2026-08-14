use {
    crate::prefixes::this::Prefixes,
    ekg_identifier::{NS_OWL, NS_RDF, NS_RDFS, NS_XSD},
    ekg_metadata::{Class, Namespace, Predicate},
    ekg_util::log::LOG_TARGET_DATABASE,
    std::ops::Deref,
};

#[derive(Default, Clone)]
pub struct PrefixesBuilder {
    prefixes: Vec<Namespace>,
}

impl PrefixesBuilder {
    pub fn default_builder() -> Self { PrefixesBuilder { prefixes: Vec::new() } }

    pub fn declare(mut self, namespace: &Namespace) -> Self {
        self.prefixes.push(namespace.clone());
        self
    }

    /// Return the default consts: `RDF`, `RDFS`, `OWL` and `XSD`
    pub fn default_namespaces(self) -> Self {
        tracing::trace!(target: LOG_TARGET_DATABASE, "Declaring default namespaces");
        self.declare(NS_RDF.deref())
            .declare(NS_RDFS.deref())
            .declare(NS_OWL.deref())
            .declare(NS_XSD.deref())
    }

    pub fn declare_namespaces(mut self, namespaces: &[Namespace]) -> Self {
        self.prefixes.extend_from_slice(namespaces);
        self
    }

    pub fn add_class(mut self, clazz: &Class) -> Self {
        self.prefixes.push(clazz.namespace.clone());
        self
    }

    pub fn add_predicate(mut self, predicate: &Predicate) -> Self {
        self.prefixes.push(predicate.namespace.clone());
        self
    }

    pub fn build(self) -> Result<Prefixes, ekg_error::Error> {
        let mut to_build = Prefixes::empty()?;
        for namespace in self.prefixes.iter() {
            to_build.declare_namespace(namespace)?;
        }
        Ok(to_build)
    }
}
