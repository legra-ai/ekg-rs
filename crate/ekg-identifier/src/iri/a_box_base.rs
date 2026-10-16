use {
    crate::{TBoxNamespaceIRI, iri::NamespaceIRI},
    ekg_error::Error,
    iref::Iri,
    serde::{Deserialize, Serialize},
};

/// An `ABoxNamespaceIRI` is a namespace IRI for ABox resources in an EKG where
/// we always want to use the `/` separator for local names since anything that
/// comes after a `#` separator is not passed through to the server-side in an
/// HTTP request.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ABoxNamespaceIRI(
    #[serde(deserialize_with = "ekg_util::serde_util::deserialize_uri")]
    #[serde(serialize_with = "ekg_util::serde_util::serialize_base_uri")]
    pub iri_string::types::IriReferenceString,
);

impl ABoxNamespaceIRI {
    #[inline]
    pub fn as_str(&self) -> &str { self.0.as_str() }

    // noinspection DuplicatedCode
    pub fn as_base_iri(&self) -> String {
        let str = self.as_str();
        let last_char = str.chars().last().unwrap();
        if last_char == '/' {
            str.to_string()
        } else {
            format!("{str}/")
        }
    }

    pub fn as_default_id_base_iri(&self) -> Result<Self, Error> {
        use std::str::FromStr;
        let str = self.as_str();
        let last_char = str.chars().last().unwrap();
        if last_char == '/' {
            Self::from_str(format!("{str}{}/", crate::DEFAULT_ID_PATH).as_str())
        } else {
            Self::from_str(format!("{str}/{}/", crate::DEFAULT_ID_PATH).as_str())
        }
    }
}

impl NamespaceIRI for ABoxNamespaceIRI {
    fn as_str(&self) -> &str { self.0.as_str() }

    fn authority(&self) -> Option<&str> {
        let iri = unsafe { Iri::new_unchecked(self.0.as_str()) };
        iri.authority()
            .and_then(|authority| {
                match authority.host().as_str() {
                    "127.0.0.1" | "localhost" => None,
                    _ => Some(authority),
                }
            })
            .map(|authority| authority.as_str())
    }
}

impl std::fmt::Display for ABoxNamespaceIRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl From<iri_string::types::IriReferenceString> for ABoxNamespaceIRI {
    fn from(uri: iri_string::types::IriReferenceString) -> Self { Self(uri) }
}

impl std::str::FromStr for ABoxNamespaceIRI {
    type Err = ekg_error::Error;

    fn from_str(uri_str: &str) -> Result<Self, Self::Err> {
        let last_char = uri_str.chars().last().unwrap();
        if last_char == '/' {
            Ok(Self(
                iri_string::types::IriReferenceString::try_from(uri_str.to_owned())?,
            ))
        } else {
            Err(ekg_error::Error::ABoxNamespaceIRIDoesNotEndWithSlash { iri: uri_str.to_owned() })
        }
    }
}

/// Convert a TBoxNamespaceIRI into an ABoxNamespaceIRI which only works when
/// the TBoxNamespaceIRI ends with a slash (`/`).
impl TryFrom<TBoxNamespaceIRI> for ABoxNamespaceIRI {
    type Error = ekg_error::Error;

    fn try_from(value: TBoxNamespaceIRI) -> Result<Self, Self::Error> { Ok(Self(value.0)) }
}
