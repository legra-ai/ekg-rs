#![cfg(feature = "_rdfox")]

use {
    crate::{
        Parameters,
        fact_domain::FactDomain,
        prefixes::Prefixes,
        rdfox::{DataStoreConnection, Transaction},
        statement::Statement,
    },
    ekg_metadata::Graph,
    ekg_util::log::LOG_TARGET_DATABASE,
    indoc::formatdoc,
    owo_colors::OwoColorize,
    std::{
        fmt::{Display, Formatter},
        path::Path,
        sync::Arc,
        time::Instant,
    },
};

/// A `GraphConnection` is a wrapper around a
/// [`DataStoreConnection`](DataStoreConnection) with a specific
/// [`Graph`](Graph) and an optional ontology [`Graph`](Graph).
#[derive(Debug)]
pub struct GraphConnection {
    pub datastore_connection: Arc<DataStoreConnection>,
    started_at:               Instant,
    pub graph:                Graph,
    pub ontology_graph:       Option<Graph>,
}

impl Display for GraphConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "graph-connection to {:} (on {:})",
            self.graph, self.datastore_connection
        )
    }
}

impl Drop for GraphConnection {
    fn drop(&mut self) {
        let duration = self.started_at.elapsed();
        tracing::trace!(
            target: LOG_TARGET_DATABASE,
            duration = ?duration,
            "Dropped {self}",
        )
    }
}

impl GraphConnection {
    pub fn new(
        datastore_connection: Arc<DataStoreConnection>,
        graph: Graph,
        ontology_graph: Option<Graph>,
    ) -> Arc<Self> {
        let result = Self {
            datastore_connection,
            started_at: Instant::now(),
            graph,
            ontology_graph,
        };
        tracing::trace!(target: LOG_TARGET_DATABASE, "Created {}", result.green());
        Arc::new(result)
    }

    /// Create a clone with another `DataStoreConnection`
    pub fn with_data_store_connection(
        &self,
        datastore_connection: &Arc<DataStoreConnection>,
    ) -> Arc<Self> {
        Arc::new(Self {
            datastore_connection: datastore_connection.clone(),
            started_at:           self.started_at,
            graph:                self.graph.clone(),
            ontology_graph:       self.ontology_graph.clone(),
        })
    }

    pub fn import_data_from_file<P>(&self, file: P) -> Result<(), ekg_error::Error>
    where P: AsRef<Path> {
        self.datastore_connection
            .import_data_from_file(file, &self.graph)
    }

    pub fn import_axioms(&self) -> Result<(), ekg_error::Error> {
        assert!(
            self.ontology_graph.is_some(),
            "no ontology graph specified"
        );
        self.datastore_connection
            .import_axioms_from_triples(self.ontology_graph.as_ref().unwrap(), &self.graph)
    }

    /// Read all RDF files (currently it supports .ttl and .nt files) from
    /// the given directory, applying ignore files like `.gitignore`.
    ///
    /// Returns the number of loaded files.
    ///
    /// TODO: Support all the types that RDFox supports (and more)
    /// TODO: Support '*.gz' files
    /// TODO: Parallelize appropriately in sync with number of threads that
    /// RDFox uses
    #[cfg(feature = "fs")]
    pub fn import_rdf_from_directory(&self, root: &Path) -> Result<u16, ekg_error::Error> {
        self.datastore_connection
            .import_rdf_from_directory(root, &self.graph)
    }

    /// Get the number of triples using the given transaction.
    ///
    /// TODO: Implement this with SPARQL COUNT (and compare performance)
    pub fn get_triples_count(
        &self,
        tx: &Arc<Transaction>,
        fact_domain: FactDomain,
    ) -> Result<usize, ekg_error::Error> {
        Statement::new(
            Prefixes::builder().build()?,
            formatdoc!(
                r##"
                SELECT ?s ?p ?o
                FROM {:}
                WHERE {{
                    ?s ?p ?o .
                }}
            "##,
                self.graph.as_display_iri()
            )
            .into(),
        )?
        .cursor(
            &self.datastore_connection,
            Parameters::builder().fact_domain(fact_domain).build()?,
        )?
        .count(tx)
    }

    // pub fn get_subjects_count(&self, fact_domain: FactDomain) ->
    // Result<std::os::raw::c_ulong, ekg_error::Error> {     Statement::query(
    //         &Namespaces::default()?,
    //         indoc! {r##"
    //             SELECT DISTINCT ?subject
    //             WHERE {
    //                 {
    //                     GRAPH ?graph {
    //                         ?subject ?p ?o
    //                     }
    //                 } UNION {
    //                     ?subject ?p ?o .
    //                     BIND("default" AS ?graph)
    //                 }
    //             }
    //         "##},
    //     )?
    //         .cursor(self, &Parameters::builder().fact_domain(fact_domain)?)?
    //         .count()
    // }
    //
    // pub fn get_predicates_count(&self, fact_domain: FactDomain) ->
    // Result<std::os::raw::c_ulong, ekg_error::Error> {     Statement::query(
    //         &Namespaces::default()?,
    //         indoc! {r##"
    //             SELECT DISTINCT ?predicate
    //             WHERE {
    //                 {
    //                     GRAPH ?graph {
    //                         ?s ?predicate ?o
    //                     }
    //                 } UNION {
    //                     ?s ?predicate ?o .
    //                     BIND("default" AS ?graph)
    //                 }
    //             }
    //         "##},
    //     )?
    //         .cursor(self, &Parameters::builder().fact_domain(fact_domain)?)?
    //         .count()
    // }
    //
    // pub fn get_ontologies_count(&self, fact_domain: FactDomain) ->
    // Result<std::os::raw::c_ulong, ekg_error::Error> {     Statement::query(
    //         &Namespaces::default()?,
    //         indoc! {r##"
    //             SELECT DISTINCT ?ontology
    //             WHERE {
    //                 {
    //                     GRAPH ?graph {
    //                         ?ontology a <http://www.w3.org/2002/07/owl#Ontology>
    //                     }
    //                 } UNION {
    //                         ?ontology a <http://www.w3.org/2002/07/owl#Ontology>
    //                     BIND("default" AS ?graph)
    //                 }
    //             }
    //         "##},
    //     )?
    //         .cursor(self, &Parameters::builder().fact_domain(fact_domain)?)?
    //         .count()
    // }
}
