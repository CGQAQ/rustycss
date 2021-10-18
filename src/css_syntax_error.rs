#![allow(dead_code)]

use crate::datatype::Number;
use std::any::TypeId;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CssSyntaxError {
    name: String,
    message: String,
    line: Number,
    column: Number,
    source: Option<String>,
    file: Option<String>,
    plugin: Option<String>,
}

impl Display for CssSyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for CssSyntaxError {
    fn description(&self) -> &str {
        match self.source.as_ref() {
            Some(msg) => &msg,
            None => "",
        }
    }
}

impl CssSyntaxError {
    pub fn new(
        &mut self,
        message: String,
        line: Number,
        column: Number,
        source: Option<String>,
        file: Option<String>,
        plugin: Option<String>,
    ) -> Self {
        Self {
            name: "CssSyntaxError".to_string(),
            message,
            line,
            column,
            source,
            file,
            plugin,
        }
    }
}
