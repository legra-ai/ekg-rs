#![allow(missing_docs)]

use ekg_error::Error;
#[cfg(not(target_family = "wasm"))]
use owo_colors::OwoColorize;

pub const LOG_TARGET_TRANSFORM: &str = "transform";
pub const LOG_TARGET_DATABASE: &str = "database";
pub const LOG_TARGET_PROJECT: &str = "project";
pub const LOG_TARGET_NUMBERS: &str = "numbers";
pub const LOG_TARGET_COMMAND: &str = "command";
pub const LOG_TARGET_EXPORT: &str = "export";
pub const LOG_TARGET_CONFIG: &str = "config";
pub const LOG_TARGET_SPARQL: &str = "sparql";
pub const LOG_TARGET_SERVER: &str = "server";
pub const LOG_TARGET_FETCH: &str = "fetch";
pub const LOG_TARGET_FILES: &str = "files";
pub const LOG_TARGET_STORY: &str = "story";
pub const LOG_TARGET_TEST: &str = "test";

#[allow(missing_docs, unused_variables, clippy::cognitive_complexity)]
pub fn log_item<T: std::fmt::Display>(target: &'static str, item: &str, value: T) {
    #[cfg(target_family = "wasm")]
    let value = format!("{value:>10}");
    #[cfg(not(target_family = "wasm"))]
    let value = format!("{value:>10}").blue().to_string();
    match target {
        LOG_TARGET_TRANSFORM => tracing::info!(target: LOG_TARGET_TRANSFORM, "{item:<54}: {value}"),
        LOG_TARGET_DATABASE => tracing::info!(target: LOG_TARGET_DATABASE, "{item:<54}: {value}"),
        LOG_TARGET_PROJECT => tracing::info!(target: LOG_TARGET_PROJECT, "{item:<54}: {value}"),
        LOG_TARGET_NUMBERS => tracing::info!(target: LOG_TARGET_NUMBERS, "{item:<54}: {value}"),
        LOG_TARGET_COMMAND => tracing::info!(target: LOG_TARGET_COMMAND, "{item:<54}: {value}"),
        LOG_TARGET_EXPORT => tracing::info!(target: LOG_TARGET_EXPORT, "{item:<54}: {value}"),
        LOG_TARGET_CONFIG => tracing::info!(target: LOG_TARGET_CONFIG, "{item:<54}: {value}"),
        LOG_TARGET_SPARQL => tracing::info!(target: LOG_TARGET_SPARQL, "{item:<54}: {value}"),
        LOG_TARGET_FETCH => tracing::info!(target: LOG_TARGET_FETCH, "{item:<54}: {value}"),
        LOG_TARGET_FILES => tracing::info!(target: LOG_TARGET_FILES, "{item:<54}: {value}"),
        LOG_TARGET_TEST => tracing::info!(target: LOG_TARGET_TEST, "{item:<54}: {value}"),
        _ => tracing::info!("{item:<54}: {value}"),
    }
}

#[allow(missing_docs, unused_variables, clippy::cognitive_complexity)]
pub fn log_item_debug<T: std::fmt::Display>(target: &str, item: &str, value: T) {
    #[cfg(target_family = "wasm")]
    let value = format!("{value:>10}");
    #[cfg(not(target_family = "wasm"))]
    let value = format!("{value:>10}").blue().to_string();
    match target {
        LOG_TARGET_TRANSFORM => {
            tracing::debug!(target: LOG_TARGET_TRANSFORM, "{item:<54}: {value}")
        },
        LOG_TARGET_DATABASE => tracing::debug!(target: LOG_TARGET_DATABASE, "{item:<54}: {value}"),
        LOG_TARGET_PROJECT => tracing::debug!(target: LOG_TARGET_PROJECT, "{item:<54}: {value}"),
        LOG_TARGET_NUMBERS => tracing::debug!(target: LOG_TARGET_NUMBERS, "{item:<54}: {value}"),
        LOG_TARGET_COMMAND => tracing::debug!(target: LOG_TARGET_COMMAND, "{item:<54}: {value}"),
        LOG_TARGET_EXPORT => tracing::debug!(target: LOG_TARGET_EXPORT, "{item:<54}: {value}"),
        LOG_TARGET_CONFIG => tracing::debug!(target: LOG_TARGET_CONFIG, "{item:<54}: {value}"),
        LOG_TARGET_SPARQL => tracing::debug!(target: LOG_TARGET_SPARQL, "{item:<54}: {value}"),
        LOG_TARGET_FETCH => tracing::debug!(target: LOG_TARGET_FETCH, "{item:<54}: {value}"),
        LOG_TARGET_FILES => tracing::debug!(target: LOG_TARGET_FILES, "{item:<54}: {value}"),
        LOG_TARGET_TEST => tracing::debug!(target: LOG_TARGET_TEST, "{item:<54}: {value}"),
        _ => tracing::debug!("{item:<54}: {value}"),
    }
}

/// Log a path as an item at info level, relative to the current directory
pub fn log_path(target: &'static str, item: &str, path: &std::path::Path) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;

    match path.strip_prefix(current_dir.clone()) {
        Ok(p) => {
            log_item(target, item, format!("./{}", p.display()));
        },
        Err(_) => {
            log_item(target, item, path.display());
        },
    }

    Ok(())
}

/// Log a path as an item at debug level, relative to the current directory
pub fn log_path_debug(
    target: &'static str,
    item: &str,
    path: &std::path::Path,
) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;

    match path.strip_prefix(current_dir.clone()) {
        Ok(p) => {
            log_item_debug(target, item, format!("./{}", p.display()));
        },
        Err(_) => {
            log_item_debug(target, item, path.display());
        },
    }

    Ok(())
}

// noinspection ALL
#[allow(unused_variables, clippy::cognitive_complexity)]
pub fn log_duration<'a, T, U>(target: &str, task: &str, f: T) -> U
where T: FnOnce() -> U + 'a {
    let start = std::time::Instant::now();
    let result = f();
    #[cfg(target_family = "wasm")]
    let value = format!("{:}", start.elapsed().as_millis());
    #[cfg(not(target_family = "wasm"))]
    let value = format!("{:}", start.elapsed().as_millis())
        .blue()
        .to_string();
    match target {
        LOG_TARGET_TRANSFORM => {
            tracing::info!(
                target: LOG_TARGET_TRANSFORM,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_DATABASE => {
            tracing::info!(
                target: LOG_TARGET_DATABASE,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_PROJECT => {
            tracing::info!(
                target: LOG_TARGET_PROJECT,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_NUMBERS => {
            tracing::info!(
                target: LOG_TARGET_NUMBERS,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_COMMAND => {
            tracing::info!(
                target: LOG_TARGET_COMMAND,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_EXPORT => {
            tracing::info!(
                target: LOG_TARGET_EXPORT,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_CONFIG => {
            tracing::info!(
                target: LOG_TARGET_CONFIG,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_SPARQL => {
            tracing::info!(
                target: LOG_TARGET_SPARQL,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_FETCH => {
            tracing::info!(
                target: LOG_TARGET_FETCH,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_FILES => {
            tracing::info!(
                target: LOG_TARGET_FILES,
                "{task:49} took: {value:>10}ms"
            );
        },
        LOG_TARGET_TEST => {
            tracing::info!(
                target: LOG_TARGET_TEST,
                "{task:49} took: {value:>10}ms"
            );
        },
        _ => tracing::info!("{task:49} took: {value:>10}ms"),
    }
    result
}

#[cfg(target_family = "wasm")]
pub fn does_console_support_color() -> bool { return false; }

#[cfg(not(target_family = "wasm"))]
pub fn does_console_support_color() -> bool {
    if let Ok(term) = std::env::var("TERM") &&
        term.as_str() == "dumb"
    {
        return false;
    }
    if let Ok(no_color) = std::env::var("NO_COLOR") &&
        no_color.as_str() == "1"
    {
        return false;
    }
    if let Ok(style) = std::env::var("RUST_LOG_STYLE") &&
        style.as_str() == "never"
    {
        return false;
    }

    true
}
