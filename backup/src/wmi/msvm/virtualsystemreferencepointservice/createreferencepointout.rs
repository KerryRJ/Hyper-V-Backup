#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateReferencePointOut {
    pub ReturnValue: u32,
    pub ResultingReferencePoint: Option<String>,
    pub Job: Option<String>,
}
