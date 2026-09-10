#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MethodResult {
    pub ReturnValue: u16,
    pub Job: Option<String>,
}
