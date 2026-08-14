use ekg_error::Error;

pub fn mandatory_env_var(name: &str, suffix: Option<&'static str>) -> Result<String, Error> {
    let env_var_name = format!("{}{}", name, suffix.unwrap_or(""));
    match std::env::var(env_var_name.as_str()) {
        Ok(val) => {
            if val.trim().is_empty() {
                Err(Error::EnvironmentVariableEmpty(
                    env_var_name.to_string(),
                ))
            } else {
                Ok(val)
            }
        },
        Err(_) => {
            Err(Error::MandatoryEnvironmentVariableMissing(
                env_var_name.to_string(),
            ))
        },
    }
}

pub fn mandatory_env_var_static(
    name: &str,
    suffix: Option<&'static str>,
) -> Result<&'static str, Error> {
    Ok(mandatory_env_var(name, suffix)?.leak())
}
