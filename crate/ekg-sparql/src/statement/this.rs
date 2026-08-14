#[cfg(feature = "_rdfox")]
use {
    crate::rdfox::{Cursor, DataStoreConnection},
    std::ffi::CString,
};
use {
    crate::{Parameters, Prefixes},
    ekg_error::Error,
    ekg_metadata::DEFAULT_GRAPH_RDFOX,
    indoc::formatdoc,
    std::{
        borrow::Cow,
        collections::HashMap,
        fmt::{Display, Formatter},
        ops::Deref,
    },
};

pub const RDFOX_QUERY_VALIDATION: &str = "rdfox-query-validation";
pub const RDFOX_QUERY_VALIDATION_STANDARD_COMPLIANT: &str = "standard-compliant";

/// SPARQL Statement
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statement {
    pub(crate) prefixes: Prefixes,
    pub(crate) text:     String,
    /// Parameters to be added to the SPARQL HTTP request that were recognized
    /// in the comments of the SPARQL statement.
    pub params:          HashMap<&'static str, &'static str>,
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // writeln!(f, "SPARQL Statement:")?;
        for (number, line) in self.text.lines().enumerate() {
            writeln!(f, "{:0>4}: {line}", number + 1)?;
        }
        Ok(())
    }
}

impl Statement {
    pub fn new(prefixes: crate::Prefixes, statement: Cow<str>) -> Result<Self, ekg_error::Error> {
        Self::new_with_params(prefixes, statement, HashMap::new())
    }

    pub fn new_with_params(
        prefixes: crate::Prefixes,
        statement: Cow<str>,
        params: HashMap<&'static str, &'static str>,
    ) -> Result<Self, ekg_error::Error> {
        Ok(Self {
            prefixes: prefixes.clone(),
            text:     format!("{}\n{}", prefixes, statement.trim()),
            params:   Self::scan_for_params(statement.as_ref(), params)?,
        })
    }

    /// Scan the comment lines in the given SPARQL statement for special
    /// key/value pairs that we should add to the SPARQL HTTP request such
    /// as:
    ///
    /// - rdfox-query-validation: standards-compliant
    fn scan_for_params(
        statement: &str,
        mut params: HashMap<&'static str, &'static str>,
    ) -> Result<HashMap<&'static str, &'static str>, ekg_error::Error> {
        for line in statement.lines().filter(|l| l.starts_with("#")) {
            if let Some(stripped_line) = line.strip_prefix("# ") &&
                let Some((key, value)) = Self::scan_for_param(stripped_line)
            {
                params.insert(key, value);
            }
        }
        Ok(params)
    }

    fn scan_for_param(line: &str) -> Option<(&'static str, &'static str)> {
        let mut parts = line.splitn(2, ':');
        if let Some(RDFOX_QUERY_VALIDATION) = parts.next() &&
            let Some(RDFOX_QUERY_VALIDATION_STANDARD_COMPLIANT) = parts.next().map(str::trim)
        {
            return Some((
                RDFOX_QUERY_VALIDATION,
                RDFOX_QUERY_VALIDATION_STANDARD_COMPLIANT,
            ));
        }
        None
    }

    #[cfg(feature = "_rdfox")]
    pub(crate) fn as_c_string(&self) -> Result<CString, ekg_error::Error> {
        Ok(CString::new(self.text.as_str())?)
    }

    pub fn as_str(&self) -> &str { self.text.as_str() }

    pub fn no_comments(&self) -> String { crate::no_comments(self.text.as_str()) }

    /// Return a Statement that can be used to export all data in
    /// `application/nquads` format
    pub fn nquads_query(prefixes: crate::Prefixes) -> Result<Statement, ekg_error::Error> {
        let default_graph = DEFAULT_GRAPH_RDFOX.deref().as_display_iri();
        let statement = Statement::new(
            prefixes,
            formatdoc!(
                r##"
                SELECT ?S ?P ?O ?G
                WHERE {{
                    {{
                        GRAPH ?G {{ ?S ?P ?O }}
                    }} UNION {{
                        ?S ?P ?P .
                        BIND({default_graph} AS ?G)
                    }}
                }}
            "##
            )
            .into(),
        )?;
        Ok(statement)
    }

    pub fn default_parameters(&self) -> Result<Parameters, Error> {
        Parameters::builder().fact_domain_all().build()
    }

    /// Complete the given parameters with any parameters that are found in the
    /// comments of the SPARQL statement.
    pub fn complete_parameters(&self, parameters: Parameters) -> Result<Parameters, Error> {
        let fixed_params = if let Some(&RDFOX_QUERY_VALIDATION_STANDARD_COMPLIANT) =
            self.params.get(RDFOX_QUERY_VALIDATION)
        {
            let mut tmp_params = parameters.clone();
            tmp_params.set_string(
                RDFOX_QUERY_VALIDATION.strip_prefix("rdfox-").unwrap(),
                RDFOX_QUERY_VALIDATION_STANDARD_COMPLIANT,
            )?;
            tmp_params
        } else {
            parameters
        };
        Ok(fixed_params)
    }

    #[cfg(feature = "_rdfox")]
    pub fn cursor_with_default_parameters(
        &self,
        connection: &std::sync::Arc<DataStoreConnection>,
    ) -> Result<Cursor, ekg_error::Error> {
        self.cursor(connection, self.default_parameters()?)
    }

    #[cfg(feature = "_rdfox")]
    pub fn cursor(
        &self,
        connection: &std::sync::Arc<DataStoreConnection>,
        parameters: Parameters,
    ) -> Result<Cursor, ekg_error::Error> {
        Cursor::create(connection, parameters, self)
    }
}
