use {
    crate::{SPARQLFlavor::SPARQL11, SPARQLStatementType, statement::Statement},
    ekg_metadata::Namespace,
    spargebra::{Query, SparqlParser, Update},
};

#[cfg(test)]
mod tests;

/// The `ParsedStatement` is a wrapper around the `OxiGraph` "spargebra" crate,
/// dealing with the distinction between SPARQL update-statements and SPARQL
/// query-statements which are handled separately by that library.
#[allow(missing_docs)]
#[derive(Clone)]
pub struct ParsedStatement {
    pub statement_type:   SPARQLStatementType,
    pub query_statement:  Option<Query>,
    pub update_statement: Option<Update>,
    pub statement:        Statement,
}

#[allow(missing_docs)]
impl ParsedStatement {
    fn parser(base_iri: Option<&str>) -> Result<SparqlParser, ekg_error::Error> {
        match base_iri {
            Some(base_iri) => {
                SparqlParser::new()
                    .with_base_iri(base_iri)
                    .map_err(|_| ekg_error::Error::InvalidBaseIri(base_iri.to_owned()))
            },
            None => Ok(SparqlParser::new()),
        }
    }

    pub fn parse(
        statement: &Statement,
        base_ns: Option<&Namespace>,
    ) -> Result<Self, ekg_error::Error> {
        let (statement_type, query_statement, update_statement) = Self::_parse(statement, base_ns)?;
        Ok(Self {
            statement_type,
            query_statement,
            update_statement,
            statement: statement.clone(),
        })
    }

    fn _parse(
        statement: &Statement,
        base_ns: Option<&Namespace>,
    ) -> Result<(SPARQLStatementType, Option<Query>, Option<Update>), ekg_error::Error> {
        let base_iri = base_ns.map(|ns| ns.iri.to_string());
        // First figure out whether we are dealing with a SPARQL update-statement by
        // attempting to parse it. If that fails, we'll treat it as a SPARQL
        // query-statement.
        match Self::parser(base_iri.as_deref())?.parse_update(statement.as_str()) {
            Ok(algebra) => {
                Ok((
                    SPARQLStatementType::UPDATE(SPARQL11),
                    None,
                    Some(algebra),
                ))
            },
            Err(_) => {
                tracing::debug!(
                    target: ekg_util::log::LOG_TARGET_SPARQL,
                    "SPARQL statement is not an update-statement, trying now to see if it is a \
                     query-statement"
                );
                match Self::parser(base_iri.as_deref())?.parse_query(statement.as_str()) {
                    Ok(query_algebra) => {
                        // tracing::error!("{}", query_algebra.to_sse());
                        match query_algebra {
                            Query::Select { .. } => {
                                Ok((
                                    SPARQLStatementType::SELECT(SPARQL11),
                                    Some(query_algebra),
                                    None,
                                ))
                            },
                            Query::Ask { .. } => {
                                Ok((
                                    SPARQLStatementType::ASK(SPARQL11),
                                    Some(query_algebra),
                                    None,
                                ))
                            },
                            Query::Construct { .. } => {
                                Ok((
                                    SPARQLStatementType::CONSTRUCT(SPARQL11),
                                    Some(query_algebra),
                                    None,
                                ))
                            },
                            Query::Describe { .. } => {
                                Ok((
                                    SPARQLStatementType::DESCRIBE(SPARQL11),
                                    Some(query_algebra),
                                    None,
                                ))
                            },
                        }
                    },
                    Err(err) => {
                        Err(ekg_error::Error::SPARQLStatementError {
                            source:    err,
                            statement: statement.to_string(),
                        })
                    },
                }
            },
        }
    }

    #[allow(unused)]
    pub fn query_algebra(&self) -> &Query { self.query_statement.as_ref().unwrap() }

    #[allow(unused)]
    pub fn update_algebra(&self) -> &Update { self.update_statement.as_ref().unwrap() }

    pub const fn is_select_statement(&self) -> bool {
        matches!(&self.query_statement, Some(Query::Select { .. }))
    }

    #[allow(unused)]
    pub const fn is_construct_statement(&self) -> bool {
        matches!(
            &self.query_statement,
            Some(Query::Construct { .. })
        )
    }

    #[allow(unused)]
    pub const fn is_ask_statement(&self) -> bool {
        matches!(&self.query_statement, Some(Query::Ask { .. }))
    }

    #[allow(unused)]
    pub const fn is_describe_statement(&self) -> bool {
        matches!(
            &self.query_statement,
            Some(Query::Describe { .. })
        )
    }

    pub const fn is_update_statement(&self) -> bool {
        matches!(&self.update_statement, Some(Update { .. }))
    }
}
