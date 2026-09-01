#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MethodResult {
    pub ReturnValue: u32,
    pub Job: Option<String>,
}
