/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;

use crate::backend::{CodeDeclaration, CodeOracle, CodeType, Literal};
use crate::interface::{CallbackInterface, ComponentInterface};
use askama::Template;

use super::filters;
pub struct CallbackInterfaceCodeType {
    id: String,
}

impl CallbackInterfaceCodeType {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    fn ffi_converter_name(&self, oracle: &dyn CodeOracle) -> String {
        format!("FfiConverter{}", &self.canonical_name(oracle))
    }
}

impl CodeType for CallbackInterfaceCodeType {
    fn type_label(&self, oracle: &dyn CodeOracle) -> String {
        oracle.class_name(&self.id)
    }

    fn canonical_name(&self, oracle: &dyn CodeOracle) -> String {
        format!("CallbackInterface{}", self.type_label(oracle))
    }

    fn literal(&self, _oracle: &dyn CodeOracle, _literal: &Literal) -> String {
        unreachable!();
    }

    fn lower(&self, oracle: &dyn CodeOracle, nm: &dyn fmt::Display) -> String {
        format!(
            "{}._lower({})",
            self.ffi_converter_name(oracle),
            oracle.var_name(nm)
        )
    }

    fn write(
        &self,
        oracle: &dyn CodeOracle,
        nm: &dyn fmt::Display,
        target: &dyn fmt::Display,
    ) -> String {
        format!(
            "{}._write({}, {})",
            self.ffi_converter_name(oracle),
            oracle.var_name(nm),
            target
        )
    }

    fn lift(&self, oracle: &dyn CodeOracle, nm: &dyn fmt::Display) -> String {
        format!("{}._lift({})", self.ffi_converter_name(oracle), nm)
    }

    fn read(&self, oracle: &dyn CodeOracle, nm: &dyn fmt::Display) -> String {
        format!("{}._read({})", self.ffi_converter_name(oracle), nm)
    }

    fn coerce(&self, _oracle: &dyn CodeOracle, nm: &dyn fmt::Display) -> String {
        nm.to_string()
    }
}

#[derive(Template)]
#[template(syntax = "py", escape = "none", path = "CallbackInterfaceTemplate.py")]
pub struct PythonCallbackInterface {
    inner: CallbackInterface,
}

impl PythonCallbackInterface {
    pub fn new(inner: CallbackInterface, _ci: &ComponentInterface) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &CallbackInterface {
        &self.inner
    }
}

impl CodeDeclaration for PythonCallbackInterface {
    fn definition_code(&self, _oracle: &dyn CodeOracle) -> Option<String> {
        Some(self.render().unwrap())
    }
}

#[derive(Template)]
#[template(syntax = "py", escape = "none", path = "CallbackInterfaceRuntime.py")]
pub struct PythonCallbackInterfaceRuntime {
    is_needed: bool,
}

impl PythonCallbackInterfaceRuntime {
    pub fn new(ci: &ComponentInterface) -> Self {
        Self {
            is_needed: !ci.iter_callback_interface_definitions().is_empty(),
        }
    }
}

impl CodeDeclaration for PythonCallbackInterfaceRuntime {
    fn definition_code(&self, _oracle: &dyn CodeOracle) -> Option<String> {
        if !self.is_needed {
            None
        } else {
            Some(self.render().unwrap())
        }
    }
}
