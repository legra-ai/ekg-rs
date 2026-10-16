use serde::{Deserialize, Serializer, de::Error};

pub fn serialize_uri<S>(
    uri: &iri_string::types::IriReferenceString,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(uri.as_str())
}

/// Serialize a `fluent_uri::Uri` that represents a base URI making sure
/// that it either ends with a slash or a hash (the latter can be the last
/// character of a namespace IRI in RDF).
pub fn serialize_base_uri<S>(
    uri: &iri_string::types::IriReferenceString,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let last_char = uri.as_str().chars().last().unwrap();
    if last_char == '/' || last_char == '#' {
        serializer.serialize_str(uri.as_str())
    } else {
        serializer.serialize_str(format!("{}/", uri.as_str()).as_str())
    }
}

pub fn deserialize_uri<'de, D>(
    deserializer: D,
) -> Result<iri_string::types::IriReferenceString, D::Error>
where D: serde::Deserializer<'de> {
    let s: String = Deserialize::deserialize(deserializer)?;
    iri_string::types::IriReferenceString::try_from(s).map_err(Error::custom)
}

pub fn serialize_bool_as_uppercase<S>(b: &bool, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    serializer.serialize_str(if *b { "TRUE" } else { "FALSE" })
}

pub fn deserialize_bool_as_uppercase<'de, D>(deserializer: D) -> Result<bool, D::Error>
where D: serde::Deserializer<'de> {
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        _ => {
            Err(Error::custom(format!(
                "Expected 'TRUE' or 'FALSE' but got '{}'",
                s
            )))
        },
    }
}
