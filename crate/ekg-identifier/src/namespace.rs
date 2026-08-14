use {
    crate::NamespaceIRI,
    std::{
        fmt::{Display, Formatter},
        hash::{Hash, Hasher},
    },
};

/// A `Namespace` represents a namespace IRI that can also be shown
/// in abbreviated format, also known as "prefix".
///
/// For instance, the namespace IRI <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
/// can also be shown (in [RDF Turtle](https://www.w3.org/TR/turtle/#prefixed-name)
/// or SPARQL for instance) as `rdf:`.
/// A "local name" such as "type" in such a namespace would look
/// like <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> or like `rdf:type`.
#[derive(Debug, Clone)]
pub struct Namespace {
    /// assumed to end with ':'
    pub name: String,
    /// assumed to end with either '/' or '#'
    pub iri:  crate::TBoxNamespaceIRI,
}

impl Eq for Namespace {}

impl PartialEq for Namespace {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.iri.as_str() == other.iri.as_str()
    }
}

impl Hash for Namespace {
    fn hash<H: Hasher>(&self, state: &mut H) { self.name.hash(state); }
}

impl Display for Namespace {
    // noinspection SpellCheckingInspection
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name.as_str(), self.iri)
    }
}

impl Namespace {
    pub fn declare(name: &str, iri: crate::TBoxNamespaceIRI) -> Result<Self, ekg_error::Error> {
        match iri.to_string().chars().last() {
            Some('/') | Some('#') => Ok(Self { name: name.to_string(), iri }),
            _ => {
                tracing::error!("{} does not end with either / or #", iri);
                Err(ekg_error::Error::IncorrectBaseIRI { iri: iri.to_string() })
            },
        }
    }

    /// Variation of [`declare`] that takes an [`iref::Iri`]. We may want to
    /// move back to iref since iref 3 is now available which seems to have
    /// everything we need.
    pub fn declare_iref_iri(name: &str, iri: &iref::Iri) -> Result<Self, ekg_error::Error> {
        match iri.to_string().chars().last() {
            Some('/') | Some('#') => Ok(Self { name: name.to_string(), iri: iri.try_into()? }),
            _ => {
                tracing::error!("{} does not end with either / or #", iri);
                Err(ekg_error::Error::IncorrectBaseIRI { iri: iri.to_string() })
            },
        }
    }

    pub fn declare_from_str(name: &str, iri: &str) -> Result<Self, ekg_error::Error> {
        Self::declare(name, iri.try_into()?)
    }

    // noinspection SpellCheckingInspection
    /// Return an identifier based on the current namespace IRI and the given
    /// local name within that namespace.
    pub fn with_local_name(
        &self,
        name: &str,
    ) -> Result<iri_string::types::IriReferenceString, ekg_error::Error> {
        let iri_str = match self.iri.to_string().chars().last().unwrap() {
            '/' | '#' => format!("{}{name}", self.iri),
            _ => {
                panic!("{} does not end with either / or #", self.iri)
            },
        };

        Ok(iri_string::types::IriReferenceString::try_from(
            iri_str,
        )?)
    }

    #[inline]
    pub fn is_in_namespace(&self, iri: &str) -> bool { self.iri.is_in_namespace(iri) }

    // noinspection SpellCheckingInspection
    pub fn as_sparql_prefix(&self) -> String { format!("PREFIX {} <{}>", self.name, self.iri) }

    // noinspection SpellCheckingInspection
    pub fn as_turtle_prefix(&self) -> String { format!("@prefix {} <{}> .", self.name, self.iri) }
}

#[cfg(test)]
mod tests {
    #[test_log::test]
    fn test_a_prefix() -> Result<(), ekg_error::Error> {
        let namespace = crate::Namespace::declare(
            "test:",
            iri_string::types::IriReferenceString::try_from("http://whatever.kom/test#")
                .unwrap()
                .into(),
        )
        .unwrap();
        let x = namespace.with_local_name("abc")?;

        assert_eq!(
            x.to_string().as_str(),
            "http://whatever.kom/test#abc"
        );
        Ok(())
    }

    #[test_log::test]
    fn test_b_prefix() -> Result<(), ekg_error::Error> {
        let namespace = crate::Namespace::declare(
            "test:",
            iri_string::types::IriReferenceString::try_from("http://whatever.kom/test/")
                .unwrap()
                .into(),
        )
        .unwrap();
        let x = namespace.with_local_name("abc")?;

        assert_eq!(
            x.to_string().as_str(),
            "http://whatever.kom/test/abc"
        );
        Ok(())
    }

    #[test_log::test]
    fn test_iri_string() -> Result<(), ekg_error::Error> {
        let x = iri_string::types::IriReferenceString::try_from("http://whatever.kom/test/abc#")?;
        assert_eq!(x.as_str(), "http://whatever.kom/test/abc#");

        let x = iri_string::types::IriReferenceString::try_from("http://whatever.kom/test/abc/")?;
        assert_eq!(x.as_str(), "http://whatever.kom/test/abc/");
        Ok(())
    }
}
