use super::Result;
use crate::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use types::*;

pub(crate) fn read_manifest<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let manifest = serde_json::from_reader(reader)?;
    Ok(manifest)
}

pub(crate) fn validate_name(name: &str) -> Result<()> {
    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\w+(\.\w+)*$"#).unwrap());
    if REGEX.is_match(name) {
        Ok(())
    } else {
        Err(Error::InvalidName)
    }
}

pub(crate) enum ManifestType {
    NativeMessagingHosts,
    ManagedStorage,
    Pkcs11Modules,
}

pub(crate) trait Manifest: Serialize {
    fn name(&self) -> &str;
}

impl Manifest for NativeMessagingManifest {
    fn name(&self) -> &str {
        &self.name
    }
}

impl<T: Serialize> Manifest for ManagedStorageManifest<T> {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Manifest for Pkcs11Manifest {
    fn name(&self) -> &str {
        &self.name
    }
}
