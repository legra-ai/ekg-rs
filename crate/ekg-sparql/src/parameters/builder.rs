use {
    super::Parameters,
    crate::{DatastoreType, PersistenceMode, fact_domain::FactDomain},
    std::{
        default::Default,
        path::{Path, PathBuf},
    },
};

#[derive(Default)]
pub struct ParametersBuilder {
    fact_domain:            Option<FactDomain>,
    persist_datastore:      Option<PersistenceMode>,
    file_access_sandboxing: Option<bool>,
    license_dir:            Option<PathBuf>,
    server_directory:       Option<PathBuf>,
    datastore_type:         Option<DatastoreType>,
}

impl ParametersBuilder {
    pub fn default_builder() -> Self { Self::default() }

    pub fn fact_domain(&mut self, fact_domain: FactDomain) -> &mut Self {
        self.fact_domain = Some(fact_domain);
        self
    }

    pub fn fact_domain_all(&mut self) -> &mut Self {
        self.fact_domain = Some(FactDomain::ALL);
        self
    }

    pub fn fact_domain_asserted(&mut self) -> &mut Self {
        self.fact_domain = Some(FactDomain::ASSERTED);
        self
    }

    pub fn persist_datastore(&mut self, mode: PersistenceMode) -> &mut Self {
        self.persist_datastore = Some(mode);
        self
    }

    pub fn switch_off_file_access_sandboxing(&mut self) -> &mut Self {
        self.file_access_sandboxing = Some(false);
        self
    }

    pub fn set_license(&mut self, license_dir: &Path) -> &mut Self {
        self.license_dir = Some(license_dir.into());
        self
    }

    pub fn server_directory(&mut self, server_directory: &Path) -> &mut Self {
        self.server_directory = Some(server_directory.into());
        self
    }

    pub fn datastore_type(&mut self, datastore_type: DatastoreType) -> &mut Self {
        self.datastore_type = Some(datastore_type);
        self
    }

    pub fn build(&self) -> Result<Parameters, ekg_error::Error> {
        let mut to_build = Parameters::empty()?;
        if let Some(fact_domain) = &self.fact_domain {
            to_build.fact_domain(fact_domain)?;
        }
        if let Some(persist_datastore) = &self.persist_datastore {
            to_build.persist_datastore(persist_datastore)?;
        }
        if let Some(license_dir) = &self.license_dir {
            to_build.set_license(license_dir)?;
        }
        if let Some(server_directory) = &self.server_directory {
            to_build.server_directory(server_directory)?;
        }
        if let Some(datastore_type) = self.datastore_type {
            to_build.datastore_type(datastore_type)?;
        }
        // Do this one last in case sandbox directory has been set
        if let Some(file_access_sandboxing) = self.file_access_sandboxing &&
            !file_access_sandboxing
        {
            to_build.switch_off_file_access_sandboxing()?;
        }
        Ok(to_build)
    }
}
