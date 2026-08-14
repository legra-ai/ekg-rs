use {super::prefixes::*, crate::Namespace, lazy_static::lazy_static};

// Prefixes
#[rustfmt::skip]
lazy_static! {
    pub static ref NS_BN: Namespace = Namespace::declare(NS_PREFIX_BN, NS_IRI_BN.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_SD: Namespace = Namespace::declare(NS_PREFIX_SD, NS_IRI_SD.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_WF: Namespace = Namespace::declare(NS_PREFIX_WF, NS_IRI_WF.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_UI: Namespace = Namespace::declare(NS_PREFIX_UI, NS_IRI_UI.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_SBE: Namespace = Namespace::declare(NS_PREFIX_SBE, NS_IRI_SBE.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_API: Namespace = Namespace::declare(NS_PREFIX_API, NS_IRI_API.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_DCT: Namespace = Namespace::declare(NS_PREFIX_DCT, NS_IRI_DCT.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_OWL: Namespace = Namespace::declare(NS_PREFIX_OWL, NS_IRI_OWL.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_RAW: Namespace = Namespace::declare(NS_PREFIX_RAW, NS_IRI_RAW.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_RDF: Namespace = Namespace::declare(NS_PREFIX_RDF, NS_IRI_RDF.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_XSD: Namespace = Namespace::declare(NS_PREFIX_XSD, NS_IRI_XSD.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_DCAT: Namespace = Namespace::declare(NS_PREFIX_DCAT, NS_IRI_DCAT.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_JIRA: Namespace = Namespace::declare(NS_PREFIX_JIRA, NS_IRI_JIRA.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_PROV: Namespace = Namespace::declare(NS_PREFIX_PROV, NS_IRI_PROV.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_RDFS: Namespace = Namespace::declare(NS_PREFIX_RDFS, NS_IRI_RDFS.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_SDLC: Namespace = Namespace::declare(NS_PREFIX_SDLC, NS_IRI_SDLC.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_SKOS: Namespace = Namespace::declare(NS_PREFIX_SKOS, NS_IRI_SKOS.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_EKGMM: Namespace = Namespace::declare(NS_PREFIX_EKGMM, NS_IRI_EKGMM.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_RDFOX: Namespace = Namespace::declare(NS_PREFIX_RDFOX, NS_IRI_RDFOX.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_STORY: Namespace = Namespace::declare(NS_PREFIX_STORY, NS_IRI_STORY.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_DATAOPS: Namespace = Namespace::declare(NS_PREFIX_DATAOPS, NS_IRI_DATAOPS.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_CONCEPT: Namespace = Namespace::declare(NS_PREFIX_CONCEPT, NS_IRI_CONCEPT.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_DATASET: Namespace = Namespace::declare(NS_PREFIX_DATASET, NS_IRI_DATASET.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_PERSONA: Namespace = Namespace::declare(NS_PREFIX_PERSONA, NS_IRI_PERSONA.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_USE_CASE: Namespace = Namespace::declare(NS_PREFIX_USE_CASE, NS_IRI_USE_CASE.as_str().try_into().unwrap()).unwrap();
    pub static ref NS_STORY_IMPL_SPARQL: Namespace = Namespace::declare(NS_PREFIX_STORY_IMPL_SPARQL, NS_IRI_STORY_IMPL_SPARQL.as_str().try_into().unwrap()).unwrap();
}
