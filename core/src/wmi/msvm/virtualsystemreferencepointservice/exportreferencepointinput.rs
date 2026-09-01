#![allow(non_snake_case)]

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct ExportReferencePointInput<'a> {
    pub(super) ReferencePoint: &'a str,
    pub(super) ExportDirectory: &'a str,
    pub(super) ExportSettingData: &'a str,
}
