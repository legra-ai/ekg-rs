#![cfg(not(target_family = "wasm"))]
#![deny(unused_crate_dependencies)]
// #![feature(rustc_private)]
// #![feature(ptr_metadata)]

pub use {
    client::SPARQLClient,
    datastore_type::DatastoreType,
    fact_domain::FactDomain,
    flavor::SPARQLFlavor,
    parameters::Parameters,
    parser::ParsedStatement,
    persistence_mode::PersistenceMode,
    prefixes::Prefixes,
    statement::{
        RDFOX_QUERY_VALIDATION,
        RDFOX_QUERY_VALIDATION_STANDARD_COMPLIANT,
        SPARQLStatementType,
        Statement,
        no_comments,
    },
};

mod client;
mod flavor;
mod parser;
mod prefixes;
mod statement;
#[cfg(test)]
mod tests;

mod datastore_type;
mod fact_domain;
mod parameters;
mod persistence_mode;
#[cfg(feature = "_rdfox")]
pub mod rdfox;

#[cfg(all(feature = "fs", not(feature = "_rdfox")))]
use ignore as _;
