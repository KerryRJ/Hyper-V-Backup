#![allow(non_snake_case)]

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct CreateReferencePointInput<'a> {
    pub(super) AffectedSystem: &'a str,
    pub(super) ReferencePointSettings: &'a str,
    pub(super) ReferencePointType: u16,
    pub(super) ResultingReferencePoint: &'a str,
}
