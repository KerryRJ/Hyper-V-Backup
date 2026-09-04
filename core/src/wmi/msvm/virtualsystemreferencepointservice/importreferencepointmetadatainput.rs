#![allow(non_snake_case)]

use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(super) struct ImportReferencePointMetadataInput<'a> {
    pub(super) AffectedSystem: &'a str,
    pub(super) ConfigFilePath: &'a str,
    pub(super) RuntimeStateFilePath: &'a str,
}
