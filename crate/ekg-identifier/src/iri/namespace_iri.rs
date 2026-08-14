pub trait NamespaceIRI {
    /// Returns the namespace IRI as a string slice
    fn as_str(&self) -> &str;
    /// Returns the length in characters of the namespace IRI
    fn len(&self) -> usize { self.as_str().len() }
    /// Returns whether the namespace IRI is empty.
    fn is_empty(&self) -> bool { self.as_str().is_empty() }
    /// Returns true if the given IRI is in the namespace
    fn is_in_namespace(&self, iri: &str) -> bool { iri.starts_with(self.as_str()) }

    /// Returns the authority part of the namespace IRI, if any (and if it's not
    /// `localhost` or `127.0.0.1`)
    fn authority(&self) -> Option<&str> { None }
}
