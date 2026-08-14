use crate::{Literal, Namespace};

/// The `Class` struct represents an RDFS or OWL class identifier
/// consisting of a [`Namespace`] (i.e. a namespace) and a "local name".
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Class {
    pub namespace:  Namespace,
    pub local_name: String,
}

impl std::fmt::Display for Class {
    // noinspection DuplicatedCode
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.namespace.name.as_str(),
            self.local_name.as_str()
        )
    }
}

impl Class {
    pub fn declare(namespace: Namespace, local_name: &str) -> Self {
        Self { namespace, local_name: local_name.to_string() }
    }

    // noinspection SpellCheckingInspection
    pub fn as_iri(&self) -> Result<iri_string::types::IriReferenceString, ekg_error::Error> {
        Ok(iri_string::types::IriReferenceString::try_from(
            format!("{}{}", self.namespace.iri, self.local_name),
        )?)
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn display_turtle<'a>(&'a self) -> impl std::fmt::Display + 'a {
        struct TurtleClass<'a>(&'a Class);
        impl<'a> std::fmt::Display for TurtleClass<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{}{}",
                    self.0.namespace.name, self.0.local_name
                )
            }
        }
        TurtleClass(self)
    }

    pub fn plural_label(&self) -> String {
        if self.local_name.ends_with('y') {
            let regex = fancy_regex::Regex::new(r"y$").expect("Failed to create regex");
            regex.replace(self.local_name.as_str(), "ies").to_string()
        } else {
            format!("{}s", self.local_name)
        }
    }

    // TODO: Make this slightly smarter

    pub fn is_literal(&self, literal: &Literal) -> bool {
        if let Some(that_iri) = literal.as_iri_ref() {
            if let Ok(this_iri) = self.as_iri() {
                that_iri == this_iri
            } else {
                let iri = self.to_string();
                literal.to_string() == iri
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::Namespace,
        crate::{DataType, Literal, class::Class},
    };

    #[test]
    fn test_a_class_01() {
        let namespace = Namespace::declare(
            "test:",
            "https://whatever.com/test#".to_string().try_into().unwrap(),
            // &iri_string::types::IriReferenceString::try_from("https://whatever.com/test#").unwrap(),
        )
        .unwrap();
        let class = Class::declare(namespace, "SomeClass");
        let s = format!("{:}", class);
        assert_eq!(s, "test:SomeClass")
    }

    #[test]
    fn test_a_class_02() {
        let namespace = Namespace::declare(
            "test:",
            iri_string::types::IriReferenceString::try_from("https://whatever.com/test#")
                .unwrap()
                .into(),
        )
        .unwrap();
        let class = Class::declare(namespace, "SomeClass");
        let s = format!("{}", class.as_iri().unwrap());
        assert_eq!(s, "https://whatever.com/test#SomeClass");
    }

    #[test]
    fn test_is_literal() {
        let namespace = Namespace::declare(
            "test:",
            iri_string::types::IriReferenceString::try_from("https://whatever.com/test#")
                .unwrap()
                .into(),
        )
        .unwrap();
        let class = Class::declare(namespace, "SomeClass");
        let literal = Literal::from_type_and_buffer(
            DataType::AnyUri,
            "https://whatever.com/test#SomeClass",
            None,
        )
        .unwrap();
        assert!(literal.is_some());
        assert_eq!(
            class.as_iri().unwrap().to_string().as_str(),
            "https://whatever.com/test#SomeClass"
        );
        assert!(class.is_literal(&literal.unwrap()))
    }
}
