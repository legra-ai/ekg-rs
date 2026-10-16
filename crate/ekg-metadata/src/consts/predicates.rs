#![allow(missing_docs)]
#![allow(clippy::wildcard_imports)]

use {
    crate::{Predicate, consts::*},
    ekg_identifier::{
        NS_API,
        NS_BN,
        NS_CONCEPT,
        NS_DATASET,
        NS_PERSONA,
        NS_SBE,
        NS_SDLC,
        NS_STORY,
        NS_STORY_IMPL_SPARQL,
        NS_USE_CASE,
    },
    lazy_static::lazy_static,
};

// Predicates
#[rustfmt::skip]
lazy_static! {
    pub static ref PREDICATE_API_KEY: Predicate<'static> = Predicate::declare(&NS_API, LN_KEY);
    pub static ref PREDICATE_BN_PREDICATE: Predicate<'static> = Predicate::declare(&NS_BN, LN_PREDICATE);
    pub static ref PREDICATE_BN_SUBJECT: Predicate<'static> = Predicate::declare(&NS_BN, LN_SUBJECT);
    pub static ref PREDICATE_CONCEPT_HAS_CONCEPT: Predicate<'static> = Predicate::declare(&NS_CONCEPT, LN_HAS_CONCEPT);
    pub static ref PREDICATE_CONCEPT_KEY: Predicate<'static> = Predicate::declare(&NS_CONCEPT, LN_KEY);
    pub static ref PREDICATE_CONCEPT_TYPE: Predicate<'static> = Predicate::declare(&NS_CONCEPT, LN_TYPE);
    pub static ref PREDICATE_CONCEPT_PREDICATE: Predicate<'static> = Predicate::declare(&NS_CONCEPT, LN_PREDICATE);
    pub static ref PREDICATE_CONCEPT_RDFS_CLASS: Predicate<'static> = Predicate::declare(&NS_CONCEPT, LN_RDFS_CLASS);
    pub static ref PREDICATE_DATASET_DEFAULT_GRAPH: Predicate<'static> = Predicate::declare(&NS_DATASET, LN_DEFAULT_GRAPH);
    pub static ref PREDICATE_DATASET_INCLUDE: Predicate<'static> = Predicate::declare(&NS_DATASET, LN_INCLUDE);
    pub static ref PREDICATE_PERSONA_IS_SUB_PERSONA_OF: Predicate<'static> = Predicate::declare(&NS_PERSONA, LN_IS_SUB_PERSONA_OF);
    pub static ref PREDICATE_PERSONA_KEY: Predicate<'static> = Predicate::declare(&NS_PERSONA, LN_KEY);
    pub static ref PREDICATE_SBE_CONCEPT_VALUE: Predicate<'static> = Predicate::declare(&NS_SBE, LN_CONCEPT_VALUE);
    pub static ref PREDICATE_SBE_GIVEN: Predicate<'static> = Predicate::declare(&NS_SBE, LN_GIVEN);
    pub static ref PREDICATE_SBE_HAS_CONCEPT: Predicate<'static> = Predicate::declare(&NS_SBE, LN_HAS_CONCEPT);
    pub static ref PREDICATE_SBE_HAS_DATASET: Predicate<'static> = Predicate::declare(&NS_SBE, LN_HAS_DATASET);
    pub static ref PREDICATE_SBE_HAS_GRAPH: Predicate<'static> = Predicate::declare(&NS_SBE, LN_HAS_GRAPH);
    pub static ref PREDICATE_SBE_HAS_SCENARIO: Predicate<'static> = Predicate::declare(&NS_SBE, LN_HAS_SCENARIO);
    pub static ref PREDICATE_SBE_THEN: Predicate<'static> = Predicate::declare(&NS_SBE, LN_THEN);
    pub static ref PREDICATE_SBE_VALUE: Predicate<'static> = Predicate::declare(&NS_SBE, LN_VALUE);
    pub static ref PREDICATE_SBE_WHEN: Predicate<'static> = Predicate::declare(&NS_SBE, LN_WHEN);
    pub static ref PREDICATE_SDLC_STATUS: Predicate<'static> = Predicate::declare(&NS_SDLC, LN_STATUS);
    pub static ref PREDICATE_STORY_CAN_USE_STORY: Predicate<'static> = Predicate::declare(&NS_STORY, LN_CAN_USE_STORY);
    pub static ref PREDICATE_STORY_CREATES: Predicate<'static> = Predicate::declare(&NS_STORY, LN_CREATES);
    pub static ref PREDICATE_STORY_HAS_INPUT: Predicate<'static> = Predicate::declare(&NS_STORY, LN_HAS_INPUT);
    pub static ref PREDICATE_STORY_HAS_OUTPUT: Predicate<'static> = Predicate::declare(&NS_STORY, LN_HAS_OUTPUT);
    pub static ref PREDICATE_STORY_HAS_PERSONA: Predicate<'static> = Predicate::declare(&NS_STORY, LN_HAS_PERSONA);
    pub static ref PREDICATE_STORY_IMPLEMENTED_BY: Predicate<'static> = Predicate::declare(&NS_STORY, LN_IMPLEMENTED_BY);
    pub static ref PREDICATE_STORY_IMPL_SPARQL_FILE_NAME: Predicate<'static> = Predicate::declare(&NS_STORY_IMPL_SPARQL, LN_FILE_NAME);
    pub static ref PREDICATE_STORY_IMPL_SPARQL_FLAVOR: Predicate<'static> = Predicate::declare(&NS_STORY_IMPL_SPARQL, LN_FLAVOR);
    pub static ref PREDICATE_STORY_IMPL_SPARQL_STATEMENT: Predicate<'static> = Predicate::declare(&NS_STORY_IMPL_SPARQL, LN_SPARQL_STATEMENT);
    pub static ref PREDICATE_STORY_IS_SERVER_PROVIDED: Predicate<'static> = Predicate::declare(&NS_STORY, LN_IS_SERVER_PROVIDED);
    pub static ref PREDICATE_USE_CASE_IS_COMPONENT_OF: Predicate<'static> = Predicate::declare(&NS_USE_CASE, LN_IS_COMPONENT_OF);
    pub static ref PREDICATE_USE_CASE_USED_IN: Predicate<'static> = Predicate::declare(&NS_USE_CASE, LN_USED_IN);
    pub static ref PREDICATE_USE_CASE_USES_CONCEPT: Predicate<'static> = Predicate::declare(&NS_USE_CASE, LN_USES_CONCEPT);
}
