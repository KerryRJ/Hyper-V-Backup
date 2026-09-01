#![allow(non_snake_case)]

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct ReferencePointInput<'a> {
    pub(super) AffectedReferencePoint: &'a str,
}
