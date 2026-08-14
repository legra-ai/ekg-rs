use {
    crate::{DataType, LiteralIdUrlDisplay, LiteralUrlDisplay, LiteralValue, Term},
    ekg_identifier::{ABoxNamespaceIRI, iri::NamespaceIRI},
    ekg_util::log::LOG_TARGET_DATABASE,
    std::{
        fmt::{Debug, Display, Formatter},
        mem::ManuallyDrop,
        ops::Deref,
        str::FromStr,
    },
};

/// Literals are used for values such as strings, numbers, and dates.
/// It consists of a [`DataType`] and a [`LiteralValue`].
///
/// (See also [RDF 1.1 Concepts and Abstract Syntax](https://www.w3.org/TR/rdf11-concepts/#section-Graph-Literal)).
///
/// A literal in an RDF graph consists of two or three elements:
///
/// 1. a lexical form, being a Unicode string, which SHOULD be in [Normal Form C](http://www.unicode.org/reports/tr15/)
///
/// 2. a datatype IRI, being an IRI identifying a datatype that determines how
///    the lexical form maps to a literal value,
///
/// 3. and if and only if the datatype IRI is <http://www.w3.org/1999/02/22-rdf-syntax-ns#langString>,
///    a non-empty language tag as defined by [BCP47](https://www.rfc-editor.org/info/bcp47).
///    The language tag MUST be well-formed according to section 2.2.9 of [BCP47](https://www.rfc-editor.org/info/bcp47).
#[derive(Default)]
pub struct Literal {
    pub data_type: DataType,
    literal_value: LiteralValue,
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        let data_type = self.data_type;
        if data_type != other.data_type {
            return false;
        }
        unsafe {
            if data_type.is_iri() {
                self.literal_value.iri.as_str() == other.literal_value.iri.as_str()
            } else if data_type.is_string() {
                self.literal_value.string == other.literal_value.string
            } else if data_type.is_boolean() {
                self.literal_value.boolean == other.literal_value.boolean
            } else if data_type.is_signed_integer() {
                self.literal_value.signed_integer == other.literal_value.signed_integer
            } else if data_type.is_unsigned_integer() {
                self.literal_value.unsigned_integer == other.literal_value.unsigned_integer
            } else if data_type.is_blank_node() {
                self.literal_value.blank_node == other.literal_value.blank_node
            } else if data_type.is_decimal() {
                self.literal_value.string == other.literal_value.string
            } else if data_type.is_date() {
                self.literal_value.date == other.literal_value.date
            } else if data_type.is_date_time() {
                self.literal_value.date_time == other.literal_value.date_time
            } else {
                panic!("Cannot compare, unimplemented datatype {data_type:?}")
            }
        }
    }
}

impl Eq for Literal {}

impl std::hash::Hash for Literal {
    fn hash<H>(&self, state: &mut H)
    where H: std::hash::Hasher {
        let data_type = self.data_type;
        data_type.hash(state);
        unsafe {
            #[allow(clippy::if_same_then_else)]
            if data_type.is_iri() {
                self.literal_value.iri.as_str().hash(state)
            } else if data_type.is_string() {
                self.literal_value.string.hash(state)
            } else if data_type.is_blank_node() {
                self.literal_value.blank_node.hash(state)
            } else if data_type.is_boolean() {
                self.literal_value.boolean.hash(state)
            } else if data_type.is_signed_integer() {
                self.literal_value.signed_integer.hash(state)
            } else if data_type.is_unsigned_integer() {
                self.literal_value.unsigned_integer.hash(state)
            } else if data_type.is_decimal() {
                self.literal_value.string.hash(state)
            } else if data_type.is_duration() {
                self.literal_value.string.hash(state)
            } else if data_type.is_date() {
                self.literal_value.date.hash(state)
            } else if data_type.is_date_time() {
                self.literal_value.date_time.hash(state)
            } else {
                panic!("Cannot hash, unimplemented datatype {data_type:?}")
            }
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data_type = self.data_type;
        write!(f, "Literal({:?},", data_type)?;
        unsafe {
            #[allow(clippy::if_same_then_else)]
            if data_type.is_iri() {
                write!(f, "{}", self)?
            } else if data_type.is_string() {
                write!(f, "\"{}\"", self.literal_value.string.as_str())?
            } else if data_type.is_blank_node() {
                write!(f, "_:{}", self.literal_value.blank_node.as_str())?
            } else if data_type.is_boolean() {
                write!(f, "{}", self.literal_value.boolean)?
            } else if data_type.is_signed_integer() {
                write!(f, "{}", self.literal_value.signed_integer)?
            } else if data_type.is_unsigned_integer() {
                write!(f, "{}", self.literal_value.unsigned_integer)?
            } else if data_type.is_decimal() {
                write!(f, "{}", self.literal_value.string.as_str())?
            } else if data_type.is_duration() || data_type.is_date_time_stamp() {
                write!(f, "{}", self.literal_value.string.as_str())?
            } else if data_type.is_date() {
                write!(
                    f,
                    "{}",
                    self.literal_value.date.format("%Y-%m-%d")
                )?
            } else if data_type.is_date_time() {
                write!(f, "{:}", self.literal_value.date_time)?
            } else {
                panic!("Cannot format, unimplemented datatype {data_type:?}")
            }
        }
        write!(f, ")")
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data_type.is_iri() {
            write!(f, "<{}>", self.as_iri().unwrap())
        } else if self.data_type.is_blank_node() {
            write!(f, "_:{}", self.as_string().unwrap().as_str())
        } else if self.data_type.is_string() {
            if let Some(str) = self.as_string() {
                write!(f, "\"{}\"", str.as_str())
            } else {
                write!(f, "ERROR, could not convert to String")
            }
        } else if self.data_type.is_boolean() {
            write!(f, "{}", self.as_boolean().unwrap())
        } else if self.data_type.is_date() {
            write!(f, "{}", self.as_date().unwrap())
        } else if self.data_type.is_date_time() {
            write!(f, "{}", self.as_date_time().unwrap())
        } else if let Some(str) = self.as_string() {
            write!(f, "{} ({:?})", str.as_str(), self.data_type)
        } else {
            write!(
                f,
                "ERROR, could not convert to String ({:?})",
                self.data_type
            )
        }
    }
}

impl Clone for Literal {
    // noinspection RsUnreachableCode
    fn clone(&self) -> Self {
        if self.data_type.is_iri() {
            if let Some(ref iri) = self.as_iri() {
                Literal {
                    data_type:     self.data_type,
                    literal_value: LiteralValue::new_iri(iri),
                }
            } else {
                todo!("the situation where the iri in a lexical value is empty")
            }
        } else if self.data_type.is_blank_node() {
            if let Some(blank_node) = self.as_str() {
                Literal::new_blank_node_with_datatype(blank_node, self.data_type).unwrap()
            } else {
                todo!("the situation where the blank_node in a lexical value is empty")
            }
        } else if self.data_type.is_string() {
            if let Some(str) = self.as_str() {
                Literal::new_string_with_datatype(str, self.data_type).unwrap()
            } else {
                todo!("the situation where the string in a lexical value is empty")
            }
        } else if self.data_type.is_boolean() {
            if let Some(boolean) = self.as_boolean() {
                Literal::new_boolean_with_datatype(boolean, self.data_type).unwrap()
            } else {
                todo!("the situation where the boolean in a lexical value is not a boolean")
            }
        } else if self.data_type.is_date() {
            if let Some(date) = self.as_date() {
                Literal::new_date_with_datatype(date, self.data_type).unwrap()
            } else {
                todo!("the situation where the naive date in a lexical value is not a naive date")
            }
        } else if self.data_type.is_date_time() {
            if let Some(date_time) = self.as_date_time() {
                Literal::new_date_time_with_datatype(*date_time, self.data_type).unwrap()
            } else {
                todo!("the situation where the boolean in a lexical value is not a boolean")
            }
        } else if self.data_type.is_signed_integer() {
            if let Some(long) = self.as_signed_long() {
                Literal::new_signed_integer_with_datatype(long, self.data_type).unwrap()
            } else {
                todo!("the situation where the signed integer value is not a long")
            }
        } else if self.data_type.is_unsigned_integer() {
            if let Some(long) = self.as_unsigned_long() {
                Literal::new_unsigned_integer_with_datatype(long, self.data_type).unwrap()
            } else {
                todo!("the situation where the unsigned integer value is not a long")
            }
        } else if self.data_type.is_decimal() {
            if let Some(decimal) = self.as_decimal() {
                Literal::new_decimal_with_datatype(decimal, self.data_type).unwrap()
            } else {
                todo!("the situation where the decimal value is not a decimal")
            }
        } else if self.data_type.is_duration() {
            if let Some(duration) = self.as_duration() {
                Literal::new_duration_with_datatype(duration, self.data_type).unwrap()
            } else {
                todo!("the situation where the duration value is not a duration")
            }
        } else {
            todo!(
                "dealing with other datatypes: {:?}",
                self.data_type
            )
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Literal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let data_type = self.data_type;
        unsafe {
            if data_type.is_iri() {
                serializer.serialize_str(self.literal_value.iri.as_str())
            } else if data_type.is_string() {
                serializer.serialize_str(self.literal_value.string.as_str())
            } else if data_type.is_blank_node() {
                serializer.serialize_str(self.literal_value.blank_node.as_str())
            } else if data_type.is_boolean() {
                serializer.serialize_bool(self.literal_value.boolean)
            } else if data_type.is_signed_integer() {
                serializer.serialize_i64(self.literal_value.signed_integer)
            } else if data_type.is_unsigned_integer() {
                serializer.serialize_u64(self.literal_value.unsigned_integer)
            } else if data_type.is_decimal() {
                serializer.serialize_str(self.literal_value.string.as_str())
            } else if data_type.is_duration() || data_type.is_date_time_stamp() {
                serializer.serialize_str(self.literal_value.string.as_str())
            } else if data_type.is_date() {
                serializer.serialize_str(self.literal_value.date.to_string().as_str())
            } else if data_type.is_date_time() {
                serializer.serialize_str(self.literal_value.date_time.to_string().as_str())
            } else {
                panic!("Cannot serialize, unimplemented datatype {data_type:?}")
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for Literal {
    fn deserialize<D: serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(LiteralDeserializeVisitor)
    }
}

impl FromStr for Literal {
    type Err = ekg_error::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> { Self::new_plain_literal_string(str) }
}

#[cfg(feature = "oxigraph-support")]
impl From<oxrdf::Literal> for Literal {
    fn from(value: oxrdf::Literal) -> Self {
        // TODO: Temporary simplistic implementation
        Self::from_str(value.value()).unwrap()
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self { Literal::from_str(value).unwrap() }
}

impl Literal {
    pub fn as_term(&self) -> Term {
        match self.data_type {
            DataType::IriReference | DataType::AnyUri => Term::Iri(self.clone()),
            DataType::BlankNode => Term::BlankNode(self.clone()),
            _ => Term::Literal(self.clone()),
        }
    }

    pub fn as_iri(&self) -> Option<iri_string::types::IriReferenceString> {
        if self.data_type.is_iri() {
            Some(unsafe { self.literal_value.iri.deref().clone() })
        } else {
            None
        }
    }

    pub fn as_iri_ref(&self) -> Option<&iri_string::types::IriReferenceStr> {
        if self.data_type.is_iri() {
            Some(unsafe { self.literal_value.iri.as_ref() })
        } else {
            None
        }
    }

    pub fn as_iref_iri_ref(&self) -> Option<&iref::Iri> {
        if self.data_type.is_iri() {
            Some(unsafe { iref::Iri::new(self.literal_value.iri.as_str()).unwrap() })
        } else {
            None
        }
    }

    pub fn as_iref_iribuf(&self) -> Option<iref::IriBuf> {
        if self.data_type.is_iri() {
            iref::IriBuf::from_str(unsafe { self.literal_value.iri.as_str() }).ok()
        } else {
            None
        }
    }

    pub fn as_local_name(&self) -> Option<String> {
        self.as_iri_ref()
            .map(|iri| iri.to_string())
            .and_then(|ref iri| {
                match fancy_regex::Regex::new(r#"(?:.*)[#/](.*)"#) {
                    Ok(re) => {
                        if let Ok(Some(captures)) = re.captures(iri.as_str()) {
                            captures.get(1).map(|mat| String::from(mat.as_str()))
                        } else {
                            None
                        }
                    },
                    Err(_err) => {
                        tracing::error!(
                            target: LOG_TARGET_DATABASE,
                            "Literal::as_local_name failed with iri: {}", iri.as_str()
                        );
                        None
                    },
                }
            })
    }

    pub fn as_str(&self) -> Option<&str> {
        #[allow(clippy::if_same_then_else)]
        if self.data_type.is_string() {
            unsafe { Some(self.literal_value.string.as_str()) }
        } else if self.data_type.is_iri() {
            unsafe { Some(self.literal_value.iri.as_str()) }
        } else if self.data_type.is_signed_integer() {
            None
        } else if self.data_type.is_unsigned_integer() {
            None
        } else if self.data_type.is_blank_node() {
            unsafe { Some(self.literal_value.blank_node.as_str()) }
        } else if self.data_type.is_boolean() {
            unsafe {
                if self.literal_value.boolean {
                    Some("true")
                } else {
                    Some("false")
                }
            }
        } else if self.data_type.is_decimal() {
            unsafe { Some(self.literal_value.string.as_str()) }
        } else if self.data_type.is_duration() {
            unsafe { Some(self.literal_value.string.as_str()) }
        } else if self.data_type.is_date_time() {
            unsafe { Some(self.literal_value.string.as_str()) }
        } else {
            panic!("Data type {:?} not yet supported", self.data_type);
        }
    }

    pub fn as_string(&self) -> Option<String> { self.as_str().map(|v| v.to_owned()) }

    pub fn as_boolean(&self) -> Option<bool> {
        match self.data_type {
            DataType::Boolean => Some(unsafe { self.literal_value.boolean }),
            _ => None,
        }
    }

    pub fn as_signed_long(&self) -> Option<i64> {
        if self.data_type.is_signed_integer() {
            Some(unsafe { self.literal_value.signed_integer })
        } else {
            None
        }
    }

    pub fn as_unsigned_long(&self) -> Option<u64> {
        if self.data_type.is_unsigned_integer() {
            Some(unsafe { self.literal_value.unsigned_integer })
        } else {
            None
        }
    }

    pub fn as_date(&self) -> Option<chrono::NaiveDate> {
        match self.data_type {
            DataType::Date => Some(unsafe { *self.literal_value.date }),
            DataType::DateTime => self.as_date_time().map(|dt| dt.naive_utc().date()),
            _ => None,
        }
    }

    pub fn as_date_time(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        match self.data_type {
            DataType::DateTime => Some(unsafe { &self.literal_value.date_time }),
            _ => None,
        }
    }

    pub fn as_decimal(&self) -> Option<&str> {
        match self.data_type {
            DataType::Decimal => Some(unsafe { &self.literal_value.string }),
            _ => None,
        }
    }

    pub fn as_duration(&self) -> Option<&str> {
        match self.data_type {
            DataType::Duration => Some(unsafe { &self.literal_value.string }),
            _ => None,
        }
    }

    pub fn from_type_and_c_buffer(
        data_type: DataType,
        buffer: &[u8],
    ) -> Result<Option<Literal>, ekg_error::Error> {
        let str_buffer = std::ffi::CStr::from_bytes_until_nul(buffer)
            .map_err(|err| {
                tracing::error!(
                    target: LOG_TARGET_DATABASE,
                    "Cannot read buffer: {err:?}"
                );
                ekg_error::Error::Unknown // TODO
            })?
            .to_str()
            .map_err(|err| {
                tracing::error!(
                    target: LOG_TARGET_DATABASE,
                    "Cannot convert buffer to string: {err:?}"
                );
                ekg_error::Error::Unknown // TODO
            })?;
        Self::from_type_and_buffer(data_type, str_buffer, None)
    }

    pub fn from_type_and_buffer(
        data_type: DataType,
        buffer: &str,
        id_base_iri: Option<&ABoxNamespaceIRI>,
    ) -> Result<Option<Literal>, ekg_error::Error> {
        match data_type {
            DataType::AnyUri | DataType::IriReference => {
                if buffer.starts_with('<') && buffer.ends_with('>') {
                    return Self::from_type_and_buffer(
                        data_type,
                        &buffer[1..buffer.len() - 1],
                        id_base_iri,
                    );
                }
                if buffer.starts_with("https://spec.edmcouncil.org/fibo/ontology/FND/Accounting/ISO4217-CurrencyCodes/Bol") {
                    tracing::warn!(
                        "BolívarSoberano detected\n{}", buffer
                    );
                    tracing::info!("xxxxxxxx1");
                    let x = iri_string::types::IriReferenceString::try_from(buffer.to_string());
                    tracing::info!("xxxxxxxx2 {:?}", x);
                }

                if let Ok(ref iri) = iri_string::types::IriReferenceString::try_from(buffer) {
                    Ok(Some(Literal::new_iri_with_datatype(
                        iri, data_type,
                    )?))
                } else if id_base_iri.is_some() {
                    Ok(Some(Literal::new_iri_from_string_with_datatype(
                        buffer,
                        data_type,
                        id_base_iri,
                    )?))
                } else {
                    match iri_string::types::IriReferenceString::try_from(buffer) {
                        Ok(iri) => {
                            tracing::error!(
                                target: LOG_TARGET_DATABASE,
                                "Cannot convert [{:?}] to a valid IRI",
                                iri
                            );
                            Err(ekg_error::Error::UnknownValueForDataType {
                                data_type_xsd_iri: data_type.as_xsd_iri_str().to_string(),
                                value:             buffer.to_string(),
                            })
                        },
                        Err(error) => {
                            tracing::error!(
                                target: LOG_TARGET_DATABASE,
                                "Cannot convert [{buffer}] to an IRI"
                            );
                            Err(ekg_error::Error::from(error))
                        },
                    }
                }
            },
            DataType::BlankNode => {
                Ok(Some(Literal::new_blank_node_with_datatype(
                    buffer, data_type,
                )?))
            },
            DataType::Boolean => {
                match buffer {
                    "true" | "false" => {
                        Ok(Some(Literal::new_boolean_with_datatype(
                            buffer.starts_with("true"),
                            data_type,
                        )?))
                    },
                    _ => Err(ekg_error::Error::UnknownNTriplesValue { value: buffer.to_string() }),
                }
            },
            DataType::String | DataType::PlainLiteral => {
                Ok(Some(Literal::new_string_with_datatype(
                    buffer, data_type,
                )?))
            },
            DataType::Date | DataType::DateTime => Self::date_from_str(buffer),
            DataType::Int |
            DataType::Integer |
            DataType::NegativeInteger |
            DataType::NonPositiveInteger |
            DataType::Long |
            DataType::Short => {
                let signed_integer: i64 = buffer.parse()?; // TODO: Remove unwrap
                Ok(Some(Literal::new_signed_integer_with_datatype(
                    signed_integer,
                    data_type,
                )?))
            },
            DataType::PositiveInteger |
            DataType::NonNegativeInteger |
            DataType::UnsignedByte |
            DataType::UnsignedInt |
            DataType::UnsignedShort |
            DataType::UnsignedLong => {
                let unsigned_integer: u64 = buffer.parse().unwrap(); // TODO: Remove unwrap
                Ok(Some(Literal::new_unsigned_integer_with_datatype(
                    unsigned_integer,
                    data_type,
                )?))
            },
            DataType::Decimal => {
                Ok(Some(Literal::new_decimal_with_datatype(
                    buffer, data_type,
                )?))
            },
            DataType::Duration => {
                Ok(Some(Literal::new_duration_with_datatype(
                    buffer, data_type,
                )?))
            },
            DataType::UnboundValue => Ok(None),
            _ => {
                tracing::warn!(
                    target: LOG_TARGET_DATABASE,
                    "Unsupported datatype: {data_type:?} value={buffer}"
                );
                Err(ekg_error::Error::Unknown)
            },
        }
    }

    fn date_from_str(buffer: &str) -> Result<Option<Literal>, ekg_error::Error> {
        if let Ok(date_time) = chrono::DateTime::parse_from_rfc2822(buffer) {
            return Ok(Some(Literal::new_date_time_with_datatype(
                chrono::DateTime::from(date_time),
                DataType::DateTime,
            )?));
        }
        if let Ok(date_time) = chrono::DateTime::parse_from_rfc3339(buffer) {
            return Ok(Some(Literal::new_date_time_with_datatype(
                chrono::DateTime::from(date_time),
                DataType::DateTime,
            )?));
        }
        if let Ok(date_time) = chrono::DateTime::parse_from_str(buffer, "%Y-%m-%d %H:%M:%S %z") {
            return Ok(Some(Literal::new_date_time_with_datatype(
                chrono::DateTime::from(date_time),
                DataType::DateTime,
            )?));
        }
        if let Ok(date_time) = chrono::NaiveDateTime::parse_from_str(buffer, "%Y-%m-%d %H:%M:%S") {
            return Ok(Some(Literal::new_date_time_with_datatype(
                chrono::DateTime::from_naive_utc_and_offset(date_time, chrono::Utc),
                DataType::DateTime,
            )?));
        }
        if let Ok(date_time) = chrono::NaiveDateTime::parse_from_str(buffer, "%Y-%m-%d %H:%M") {
            return Ok(Some(Literal::new_date_time_with_datatype(
                chrono::DateTime::from_naive_utc_and_offset(date_time, chrono::Utc),
                DataType::DateTime,
            )?));
        }
        if let Ok(date) = chrono::NaiveDate::parse_from_str(buffer, "%Y-%m-%d") {
            return Ok(Some(Literal::new_date_with_datatype(
                date,
                DataType::Date,
            )?));
        }
        if let Ok(date) = chrono::NaiveDate::parse_from_str(buffer, "%Y/%m/%d") {
            return Ok(Some(Literal::new_date_with_datatype(
                date,
                DataType::Date,
            )?));
        }
        if let Ok(date) = chrono::NaiveDate::parse_from_str(buffer, "%m/%d/%Y") {
            return Ok(Some(Literal::new_date_with_datatype(
                date,
                DataType::Date,
            )?));
        }

        #[cfg(feature = "serde")]
        match serde_json::from_str::<chrono::DateTime<chrono::Utc>>(buffer) {
            Ok(date_time) => {
                Ok(Some(Literal::new_date_time_with_datatype(
                    date_time,
                    DataType::DateTime,
                )?))
            },
            Err(error) => {
                tracing::error!(
                    target: LOG_TARGET_DATABASE,
                    "Could not convert [{buffer}] to a DateTime Literal"
                );
                Err(ekg_error::Error::SerdeJsonError(error))
            },
        }
        #[cfg(not(feature = "serde"))]
        Err(ekg_error::Error::Unknown) // TODO
    }

    pub fn from_iri(iri: &iri_string::types::IriReferenceStr) -> Result<Self, ekg_error::Error> {
        Ok(Literal {
            data_type:     DataType::IriReference,
            literal_value: LiteralValue { iri: ManuallyDrop::new(iri.to_owned()) },
        })
    }

    pub fn from_iref_iribuf(iri: &iref::IriBuf) -> Result<Self, ekg_error::Error> {
        Ok(Literal {
            data_type:     DataType::IriReference,
            literal_value: LiteralValue {
                iri: ManuallyDrop::new(iri_string::types::IriReferenceString::try_from(
                    iri.as_str(),
                )?),
            },
        })
    }

    pub fn new_plain_literal_string(str: &str) -> Result<Self, ekg_error::Error> {
        Self::new_string_with_datatype(str, DataType::PlainLiteral)
    }

    pub fn new_plain_literal_boolean(boolean: bool) -> Result<Self, ekg_error::Error> {
        Self::new_string_with_datatype(
            boolean.to_string().as_str(),
            DataType::PlainLiteral,
        )
    }

    pub fn new_string_with_datatype(
        str: &str,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_string(),
            "{data_type:?} is not a string type"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_string(str),
        })
    }

    /// Use this only for naive dates
    /// (see <https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html>)
    pub fn new_date_with_datatype(
        date: chrono::NaiveDate,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_date(),
            "{data_type:?} is not a Date"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_date(date),
        })
    }

    pub fn new_date_time_with_datatype(
        date_time: chrono::DateTime<chrono::Utc>,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_date_time(),
            "{data_type:?} is not a dateTime"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_date_time(date_time),
        })
    }

    pub fn new_decimal_with_datatype(
        str: &str,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_decimal(),
            "{data_type:?} is not a decimal"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_string(str),
        })
    }

    pub fn new_duration_with_datatype(
        str: &str,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_duration(),
            "{data_type:?} is not a duration"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_string(str),
        })
    }

    pub fn new_iri_from_string_with_datatype(
        iri_string: &str,
        data_type: DataType,
        id_base_iri: Option<&ABoxNamespaceIRI>,
    ) -> Result<Self, ekg_error::Error> {
        match iri_string::types::IriReferenceString::try_from(iri_string) {
            Ok(ref iri) => Self::new_iri_with_datatype(iri, data_type),
            Err(error) => {
                if let Some(id_base_iri) = id_base_iri {
                    // If we passed a base IRI and the given IRI string is just an identifier,
                    // then stick the base IRI in front of it
                    let iri_str = iri_string::types::IriReferenceString::try_from(format!(
                        "{}/{}",
                        id_base_iri, iri_string
                    ))?;
                    return Self::from_iri(iri_str.as_ref());
                }
                Err(ekg_error::Error::from(error))
            },
        }
    }

    pub fn new_iri_reference_from_str(iri: &str) -> Result<Self, ekg_error::Error> {
        let iri = iri_string::types::IriReferenceString::try_from(iri)?;
        Self::new_iri_with_datatype(&iri, DataType::IriReference)
    }

    pub fn new_iref_iri_with_datatype(
        iri: &iref::Iri,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_iri(),
            "{data_type:?} is not an IRI type"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_iref_iri(iri)?,
        })
    }

    pub fn new_iri_with_datatype(
        iri: &iri_string::types::IriReferenceStr,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_iri(),
            "{data_type:?} is not an IRI type"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_iri(iri),
        })
    }

    pub fn new_blank_node_with_datatype(
        id: &str,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_blank_node(),
            "{data_type:?} is not a blank node type"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_blank_node(id),
        })
    }

    pub fn new_boolean(boolean: bool) -> Result<Self, ekg_error::Error> {
        Self::new_boolean_with_datatype(boolean, DataType::Boolean)
    }

    pub fn new_boolean_from_string(boolean_string: &str) -> Result<Self, ekg_error::Error> {
        Self::new_boolean_from_string_with_datatype(boolean_string, DataType::Boolean)
    }

    pub fn new_boolean_from_string_with_datatype(
        boolean_string: &str,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        match boolean_string {
            "true" => Self::new_boolean_with_datatype(true, data_type),
            "false" => Self::new_boolean_with_datatype(false, data_type),
            &_ => {
                Err(ekg_error::Error::UnknownValueForDataType {
                    data_type_xsd_iri: data_type.as_xsd_iri_str().to_string(),
                    value:             boolean_string.to_string(),
                })
            },
        }
    }

    pub fn new_boolean_with_datatype(
        boolean: bool,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_boolean(),
            "{data_type:?} is not a boolean type"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_boolean(boolean),
        })
    }

    pub fn new_signed_integer(signed_integer: i64) -> Result<Self, ekg_error::Error> {
        if signed_integer >= 0 {
            Self::new_unsigned_integer(signed_integer as u64)
        } else {
            Self::new_signed_integer_with_datatype(signed_integer, DataType::NegativeInteger)
        }
    }

    pub fn new_signed_integer_with_datatype(
        signed_integer: i64,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_signed_integer(),
            "{data_type:?} is not an signed integer type"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_signed_integer(signed_integer),
        })
    }

    pub fn new_unsigned_integer(unsigned_integer: u64) -> Result<Self, ekg_error::Error> {
        Self::new_unsigned_integer_with_datatype(unsigned_integer, DataType::PositiveInteger)
    }

    pub fn new_unsigned_integer_with_datatype(
        unsigned_integer: u64,
        data_type: DataType,
    ) -> Result<Self, ekg_error::Error> {
        assert!(
            &data_type.is_unsigned_integer(),
            "{data_type:?} is not an unsigned integer type"
        );
        Ok(Literal {
            data_type,
            literal_value: LiteralValue::new_unsigned_integer(unsigned_integer),
        })
    }

    pub fn display_turtle<'a, 'b>(&'a self) -> impl Display + 'a + 'b
    where 'a: 'b {
        struct TurtleLexVal<'b>(&'b Literal);
        impl<'b> Display for TurtleLexVal<'b> {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                let data_type = self.0.data_type;
                unsafe {
                    if data_type.is_iri() {
                        write!(f, "<{}>", self.0.literal_value.iri.as_str())?
                    } else if data_type.is_string() {
                        write!(f, "\"{}\"", self.0.literal_value.string.as_str())?
                    } else if data_type.is_blank_node() {
                        write!(
                            f,
                            "_:{}",
                            self.0.literal_value.blank_node.as_str()
                        )?
                    } else if data_type.is_boolean() {
                        write!(f, "{}", self.0.literal_value.boolean)?
                    } else if data_type.is_signed_integer() {
                        write!(f, "{}", self.0.literal_value.signed_integer)?
                    } else if data_type.is_unsigned_integer() {
                        write!(f, "{}", self.0.literal_value.unsigned_integer)?
                    } else if data_type.is_date_time() {
                        write!(
                            f,
                            "\"{}\"^^xsd:dateTime",
                            self.0.literal_value.string.as_str()
                        )?
                    } else if data_type.is_decimal() {
                        write!(f, "{}", self.0.literal_value.string.as_str())?
                    } else if data_type.is_duration() {
                        write!(
                            f,
                            "\"{}\"^^xsd:duration",
                            self.0.literal_value.string.as_str()
                        )?
                    } else {
                        panic!("Cannot format for turtle, unimplemented datatype {data_type:?}")
                    }
                }
                Ok(())
            }
        }
        TurtleLexVal(self)
    }

    pub fn display_json<'a, 'b>(&'a self) -> impl Display + 'a + 'b
    where 'a: 'b {
        struct JsonLexVal<'b>(&'b Literal);
        impl<'b> Display for JsonLexVal<'b> {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                let data_type = self.0.data_type;
                unsafe {
                    if data_type.is_iri() {
                        write!(f, "\"{}\"", *self.0.literal_value.iri)?
                    } else if data_type.is_string() {
                        write!(
                            f,
                            "\"{}\"",
                            self.0.literal_value.string.replace('\"', "\\\"").as_str()
                        )?
                    } else if data_type.is_blank_node() {
                        write!(
                            f,
                            "\"_:{}\"",
                            self.0.literal_value.blank_node.as_str()
                        )?
                    } else if data_type.is_boolean() {
                        write!(f, "{}", self.0.literal_value.boolean)?
                    } else if data_type.is_signed_integer() {
                        write!(f, "{}", self.0.literal_value.signed_integer)?
                    } else if data_type.is_unsigned_integer() {
                        write!(f, "{}", self.0.literal_value.unsigned_integer)?
                    } else if data_type.is_date_time() {
                        write!(f, "\"{}\"", self.0.literal_value.string.as_str())?
                    } else if data_type.is_decimal() {
                        write!(f, "{}", self.0.literal_value.string.as_str())?
                    } else if data_type.is_duration() {
                        write!(f, "\"{}\"", self.0.literal_value.string.as_str())?
                    } else {
                        panic!("Cannot format for JSON, unimplemented datatype {data_type:?}")
                    }
                }
                Ok(())
            }
        }
        JsonLexVal(self)
    }

    pub fn as_url_display(&self) -> LiteralUrlDisplay<'_> { LiteralUrlDisplay { literal: self } }

    pub fn as_id_url_display<'a, T: NamespaceIRI>(
        &'a self,
        id_base_iri: &'a T,
    ) -> LiteralIdUrlDisplay<'a, T> {
        LiteralIdUrlDisplay { literal: self, id_base_iri }
    }

    /// Is the given Literal an IRI whose base is the given IRI?
    pub fn is_id_iri(&self, id_base_iri: &ABoxNamespaceIRI) -> bool {
        match self.data_type {
            DataType::AnyUri | DataType::IriReference => unsafe {
                id_base_iri.is_in_namespace(self.literal_value.iri.as_str())
            },
            _ => false,
        }
    }

    pub fn is_in_namespace<T: NamespaceIRI>(&self, namespace: &T) -> bool {
        match self.data_type {
            DataType::AnyUri | DataType::IriReference => unsafe {
                namespace.is_in_namespace(self.literal_value.iri.as_str())
            },
            _ => false,
        }
    }

    pub fn as_id<T: NamespaceIRI>(&self, id_base_iri: &T) -> Result<String, ekg_error::Error> {
        match self.data_type {
            DataType::AnyUri | DataType::IriReference => unsafe {
                let len = id_base_iri.len();
                let str = self.literal_value.iri.to_string();
                let (_first, last) = str.split_at(len);
                Ok(last.to_string())
            },
            _ => Err(ekg_error::Error::UnknownDataType { data_type_id: self.data_type as u8 }),
        }
    }
}

#[cfg(feature = "serde")]
struct LiteralDeserializeVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for LiteralDeserializeVisitor {
    type Value = Literal;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a correct literal value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: serde::de::Error {
        if let Ok(Some(iri)) = Literal::from_type_and_buffer(DataType::AnyUri, v, None) {
            return Ok(iri);
        }
        if let Ok(Some(integer)) = Literal::from_type_and_buffer(DataType::Integer, v, None) {
            return Ok(integer);
        }
        if let Ok(Some(date_time)) = Literal::from_type_and_buffer(DataType::DateTime, v, None) {
            return Ok(date_time);
        }
        if let Ok(Some(decimal)) = Literal::from_type_and_buffer(DataType::Decimal, v, None) {
            return Ok(decimal);
        }
        match Literal::from_str(v) {
            Ok(literal) => Ok(literal),
            Err(rdf_store_error) => Err(E::custom(rdf_store_error.to_string())),
        }
    }
}
