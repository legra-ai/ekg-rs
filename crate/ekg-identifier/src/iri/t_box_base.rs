use {
    crate::{ABoxNamespaceIRI, iri::NamespaceIRI},
    serde::{Deserialize, Serialize},
    std::str::FromStr,
};

/// An `TBoxNamespaceIRI` is a namespace IRI for TBox resources in an EKG where
/// we can accept either a `#` or `/` at the end of the IRI.
/// See also [ABoxNamespaceIRI](crate::ABoxNamespaceIRI).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TBoxNamespaceIRI(
    #[serde(deserialize_with = "ekg_util::serde_util::deserialize_uri")]
    #[serde(serialize_with = "ekg_util::serde_util::serialize_base_uri")]
    pub iri_string::types::IriReferenceString,
);

impl TBoxNamespaceIRI {
    #[inline]
    pub fn as_str(&self) -> &str { self.0.as_str() }

    // noinspection DuplicatedCode
    pub fn as_base_iri(&self) -> String {
        let str = self.as_str();
        let last_char = str.chars().last().unwrap();
        if last_char == '/' || last_char == '#' {
            str.to_string()
        } else {
            format!("{str}/")
        }
    }
}

impl NamespaceIRI for TBoxNamespaceIRI {
    fn as_str(&self) -> &str { self.0.as_str() }
}

impl std::fmt::Display for TBoxNamespaceIRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl From<iri_string::types::IriReferenceString> for TBoxNamespaceIRI {
    fn from(uri: iri_string::types::IriReferenceString) -> Self { Self(uri) }
}

impl std::str::FromStr for TBoxNamespaceIRI {
    type Err = ekg_error::Error;

    fn from_str(uri_str: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            iri_string::types::IriReferenceString::try_from(uri_str.to_owned())?,
        ))
    }
}

impl TryFrom<&str> for TBoxNamespaceIRI {
    type Error = ekg_error::Error;

    fn try_from(iri_str: &str) -> Result<Self, Self::Error> { Self::from_str(iri_str) }
}

impl TryFrom<String> for TBoxNamespaceIRI {
    type Error = ekg_error::Error;

    fn try_from(iri_str: String) -> Result<Self, Self::Error> {
        Ok(Self(
            iri_string::types::IriReferenceString::try_from(iri_str)?,
        ))
    }
}

impl TryFrom<&iri_string::types::IriReferenceStr> for TBoxNamespaceIRI {
    type Error = ekg_error::Error;

    fn try_from(iri: &iri_string::types::IriReferenceStr) -> Result<Self, Self::Error> {
        iri.to_string().try_into()
    }
}

impl TryFrom<&iref::Iri> for TBoxNamespaceIRI {
    type Error = ekg_error::Error;

    fn try_from(iri: &iref::Iri) -> Result<Self, Self::Error> {
        Ok(Self(
            iri_string::types::IriReferenceString::try_from(iri.as_str())?.to_owned(),
        ))
    }
}

impl TryFrom<ABoxNamespaceIRI> for TBoxNamespaceIRI {
    type Error = ekg_error::Error;

    fn try_from(iri: ABoxNamespaceIRI) -> Result<Self, Self::Error> {
        TBoxNamespaceIRI::from_str(iri.as_str())
    }
}
