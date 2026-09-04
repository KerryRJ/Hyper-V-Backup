#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateReferencePointResult {
    pub ReturnValue: u16,
    pub ResultingReferencePoint: Option<String>,
    pub Job: Option<String>,
}
