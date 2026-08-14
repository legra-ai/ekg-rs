use {
    crate::{iri::ABoxNamespaceIRI, mandatory_env_var_base_iri},
    ekg_error::Error,
};

pub struct EkgIdentifierContext {
    pub ekg_base:          ABoxNamespaceIRI,
    pub ekg_id_base:       ABoxNamespaceIRI,
    pub ekg_graph_base:    ABoxNamespaceIRI,
    pub ekg_ontology_base: ABoxNamespaceIRI,
}

impl EkgIdentifierContext {
    pub fn from_env(suffix: &'static str) -> Result<Self, Error> {
        Ok(Self {
            ekg_base:          mandatory_env_var_base_iri("EKG_BASE", Some(suffix))?,
            ekg_id_base:       mandatory_env_var_base_iri("EKG_ID_BASE", Some(suffix))?,
            ekg_graph_base:    mandatory_env_var_base_iri("EKG_GRAPH_BASE", Some(suffix))?,
            ekg_ontology_base: mandatory_env_var_base_iri("EKG_ONTOLOGY_BASE", Some(suffix))?,
        })
    }
}

pub struct EkgIdentifierContexts {
    pub internal: EkgIdentifierContext,
    pub external: EkgIdentifierContext,
}

impl EkgIdentifierContexts {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            internal: EkgIdentifierContext::from_env("_INTERNAL")?,
            external: EkgIdentifierContext::from_env("_EXTERNAL")?,
        })
    }
}
