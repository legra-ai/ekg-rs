#![deny(unused_crate_dependencies)]

use thiserror::Error;

#[allow(missing_docs, dead_code)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown Error")]
    Unknown,

    #[error("Not Implemented")]
    NotImplemented,

    #[error("Cannot start a new transaction")]
    CannotStartNewTransaction,

    #[error("Invalid Input")]
    InvalidInput,

    #[error("No Input Records")]
    NoInputRecords,

    #[error("Timeout")]
    Timeout,

    #[error("Mandatory Environment Variable {0} is empty")]
    EnvironmentVariableEmpty(String),

    #[error("Mandatory Environment Variable {0} missing")]
    MandatoryEnvironmentVariableMissing(String),

    #[error("Mandatory Environment Variable {0} is not a valid IRI")]
    MandatoryEnvironmentVariableIsNotIRI(String),

    #[error("Service error {0}")]
    ServiceError(String),

    #[error("No event")]
    NoEvent,

    #[error("No subject")]
    NoSubject,

    #[error("No predicate")]
    NoPredicate,

    #[error("Path {0} does not exist")]
    PathDoesNotExist(String),

    #[error("No root project found")]
    NoRootProjectFound,

    #[error(
        "Detected an unknown story input parameter [{param}] for story [{story_key}], expected \
         parameters are: {expected_params:?}"
    )]
    DetectedUnknownStoryInputParameter {
        story_key:       String,
        param:           String,
        expected_params: Vec<String>,
    },

    #[error("No base IRI specified")]
    NoBaseIRISpecified,

    #[error("No identifier namespace specified")]
    NoIdentifierNamespaceSpecified,

    #[error("Incorrect base IRI: {iri}")]
    IncorrectBaseIRI { iri: String },

    #[error("The A-box namespace IRI {iri} does not end with a slash")]
    ABoxNamespaceIRIDoesNotEndWithSlash { iri: String },

    #[error("Parse Error")]
    Parse,

    #[cfg(feature = "uuid")]
    #[error(transparent)]
    UuidParseError(#[from] uuid::Error),

    #[error("Could not fetch context and objects")]
    MissingContext,

    #[error("Could not generate metadata")]
    CouldNotGenerateMetadata,

    #[error("Could not find root project")]
    CouldNotFindRootProject,

    #[error("Could not rewrite IRI [{iri}]")]
    CouldNotRewriteIRI { iri: String },

    #[error("Invalid Docker Image ID")]
    InvalidDockerImageId,

    #[error("Could not lock a resource: {msg}")]
    CouldNotLock { msg: String },

    #[error("Missing Identifier Base IRI")]
    MissingIdentifierBaseIRI,

    #[error("Could not create the story service client")]
    CouldNotCreateClient,

    #[error("Could not connect to the database server")]
    CouldNotConnectToServer,

    #[error("There's no context")]
    NoContextProvided,

    #[error("Invalid Story Service IRI")]
    InvalidClientIri,

    #[error("Persona {persona_key} does not exist")]
    PersonaDoesNotExist { persona_key: String },

    #[error("Story {use_case_key}/{story_key} does not exist")]
    StoryDoesNotExist { story_key: String, use_case_key: String },

    #[error("Could not find story implementation {implementation_file} for story {story_id}")]
    CouldNotFindStory {
        implementation_file: String,
        story_id:            String,
    },

    #[error("Could not find use case {0}")]
    CouldNotFindUseCase(String),

    #[error("UseCase {use_case_key} does not exist")]
    UseCaseDoesNotExist { use_case_key: String },

    #[cfg(feature = "serde")]
    #[error("JSON Parsing Error")]
    JSONParseError(serde_path_to_error::Error<serde_json::Error>),

    #[cfg(feature = "serde")]
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Invalid Story IRI")]
    InvalidStoryIri,

    #[error("Could not get story results")]
    CouldNotGetStoryResults,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    /// Encountered a syntax error in a SPARQL statement
    #[cfg(all(feature = "sparql", not(target_arch = "wasm32")))]
    #[error("Encountered SPARQL error \"{source:}\" in\n{statement:}")]
    SPARQLStatementError {
        #[source]
        source:    spargebra::SparqlSyntaxError,
        statement: String,
    },

    #[error(transparent)]
    FormatError(#[from] core::fmt::Error),

    /// Represents all other cases of `ignore::Error`
    /// (see <https://docs.rs/ignore/latest/ignore/enum.Error.html>)
    #[cfg(feature = "fs")]
    #[error(transparent)]
    WalkError(#[from] ignore::Error),

    #[cfg(feature = "iref")]
    #[error(transparent)]
    IriErrorString(#[from] iref::IriError<String>),

    #[error(transparent)]
    IriStringParseError(#[from] iri_string::validate::Error),

    #[error(transparent)]
    IriStringCreationError(#[from] iri_string::types::CreationError<String>),

    #[error(transparent)]
    CApiError(#[from] std::ffi::NulError),

    #[cfg(all(feature = "rdftk-support", not(target_arch = "wasm32")))]
    #[error(transparent)]
    RDFTkError(#[from] rdftk_core::error::Error),

    #[cfg(all(feature = "rdftk-support", not(target_arch = "wasm32")))]
    #[error(transparent)]
    RDFTkIRIError(#[from] rdftk_iri::error::Error),

    #[error("Could not open database: {source:}")]
    CouldNotOpenDatabase { source: Box<Error> },

    #[cfg(all(not(target_arch = "wasm32"), feature = "xlsx"))]
    #[error(transparent)]
    ExcelWriterError(#[from] xlsxwriter::XlsxError),

    #[cfg(all(feature = "salvo", not(target_arch = "wasm32")))]
    #[error(transparent)]
    InvalidHeaderValue(#[from] salvo::http::header::InvalidHeaderValue),

    #[cfg(all(feature = "salvo", not(target_arch = "wasm32")))]
    #[error(transparent)]
    SalvoError(#[from] salvo::Error),

    #[cfg(all(feature = "salvo", not(target_arch = "wasm32")))]
    #[error(transparent)]
    ToStrError(#[from] salvo_core::http::header::ToStrError),

    #[cfg(feature = "tokio")]
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[cfg(feature = "tauri")]
    #[error(transparent)]
    TauriError(#[from] tauri::Error),

    #[cfg(feature = "gix")]
    #[error(transparent)]
    GitDiscoverUpwardsError2(#[from] gix_discover::upwards::Error),

    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[cfg(all(feature = "reqwest", not(target_arch = "wasm32")))]
    #[error(transparent)]
    StreamBodyError(#[from] reqwest_streams::error::StreamBodyError),

    #[cfg(feature = "aws-lambda")]
    #[error(transparent)]
    LambdaError(#[from] lambda_runtime::Error),

    #[cfg(feature = "aws-lambda")]
    #[error(transparent)]
    InvalidAppName(#[from] aws_types::app_name::InvalidAppName),

    #[cfg(not(target_arch = "wasm32"))]
    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[cfg(not(target_arch = "wasm32"))]
    #[error(transparent)]
    HttpError(#[from] hyper::http::Error),

    #[cfg(not(target_arch = "wasm32"))]
    #[error(transparent)]
    InvalidUri(#[from] hyper::http::uri::InvalidUri),

    #[error("Invalid IRI: {0}")]
    InvalidIri(String),

    #[error("Invalid base IRI: {0}")]
    InvalidBaseIri(String),

    #[error(transparent)]
    SerdeUrlEncodingError(#[from] serde_urlencoded::ser::Error),

    #[error("Unknown data type {data_type_id}")]
    UnknownDataType { data_type_id: u8 },
    #[error("Unknown value [{value}] for data type {data_type_xsd_iri:?}")]
    UnknownValueForDataType {
        data_type_xsd_iri: String,
        value:             String,
    },
    #[error("Unknown XSD data type {data_type_iri}")]
    UnknownXsdDataType { data_type_iri: String },
    #[error("Unknown literal value in N-Triples format: {value}")]
    UnknownNTriplesValue { value: String },

    #[cfg(feature = "tracing-subscriber")]
    #[error(transparent)]
    EnvFilterParseError(#[from] tracing_subscriber::filter::ParseError),

    #[cfg(feature = "tracing-subscriber")]
    #[error(transparent)]
    TracingSubscriberError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[cfg(feature = "tracing-subscriber")]
    #[error(transparent)]
    TracingSubscriberTryInitError(#[from] tracing_subscriber::util::TryInitError),

    #[cfg(feature = "tracing-subscriber")]
    #[error(transparent)]
    FromEnvError(#[from] tracing_subscriber::filter::FromEnvError),

    #[cfg(all(feature = "rdfox-support", not(target_arch = "wasm32")))]
    #[error(transparent)]
    R2D2Error(#[from] r2d2::Error),

    #[error("While {action}: {message}")]
    Exception { action: String, message: String },
    #[error(
        "The multiplicity ({multiplicity}) of a cursor row exceeded the maximum number of rows \
         ({maxrow}) for query:\n{query}"
    )]
    MultiplicityExceededMaximumNumberOfRows {
        maxrow:       usize,
        multiplicity: usize,
        query:        String,
    },
    #[error("Cannot get any argument indexes from the cursor of:\n{query}")]
    CannotGetAnyArgumentIndexes { query: String },
    #[error("Maximum number of rows ({maxrow}) has been exceeded for query:\n{query}")]
    ExceededMaximumNumberOfRows { maxrow: usize, query: String },
    #[error("Could not find a license key")]
    RDFoxLicenseFileNotFound,
    #[error("Unknown resource")]
    UnknownResourceException,
    #[error("Could not create RDFox server")]
    CouldNotCreateRDFoxServer,
    #[error("Could not import RDF File")]
    CouldNotImportRDFFile,
    #[error("Invalid prefix name")]
    InvalidPrefixName,
    #[error("Invalid literal value")]
    InvalidLiteral,
    #[error("Could not parse IRI: {0:?}")]
    IriParseError(String),

    #[cfg(not(target_arch = "wasm32"))]
    #[error(transparent)]
    DateParseError(#[from] chrono::ParseError),

    #[cfg(all(feature = "rdfox-support", not(target_arch = "wasm32")))]
    #[error(transparent)]
    RDFoxError(#[from] rdfox_sys::Error),

    #[error(transparent)]
    FromPathError(#[from] relative_path::FromPathError),

    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),
}

unsafe impl Send for Error {}

#[cfg(all(feature = "salvo", not(target_arch = "wasm32")))]
#[salvo::async_trait]
impl salvo::Writer for Error {
    async fn write(
        mut self,
        _req: &mut salvo::http::Request,
        _depot: &mut salvo::Depot,
        res: &mut salvo::http::Response,
    ) {
        res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
        res.render("custom error");
    }
}

impl From<iref::InvalidIri<String>> for Error {
    fn from(value: iref::InvalidIri<String>) -> Self { Error::InvalidIri(value.to_string()) }
}

impl From<iref::InvalidIri<&str>> for Error {
    fn from(value: iref::InvalidIri<&str>) -> Self { Error::InvalidIri(value.to_string()) }
}

#[cfg(not(target_arch = "wasm32"))]
impl<I: From<&'static str>> From<Error> for nom::Err<nom::error::Error<I>> {
    fn from(_: Error) -> Self {
        nom::Err::Error(nom::error::Error::new(
            "unknown datastore error".into(),
            nom::error::ErrorKind::Fail,
        ))
    }
}
