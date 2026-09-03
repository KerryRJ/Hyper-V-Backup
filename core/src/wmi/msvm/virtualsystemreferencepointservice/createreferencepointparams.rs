#![allow(non_snake_case)]

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct CreateReferencePointParams<'a> {
    pub(super) AffectedSystem: wmi::Variant,
    pub(super) ReferencePointSettings: &'a str,
    pub(super) ReferencePointType: u16,
    pub(super) ResultingReferencePoint: Option<&'a str>,
}
