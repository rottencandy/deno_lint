// Copyright 2020 the Deno authors. All rights reserved. MIT license.
use super::Context;
use super::LintRule;
use swc_ecma_ast::{Function, Pat};
use swc_ecma_visit::Node;
use swc_ecma_visit::Visit;

pub struct DefaultParamLast;

impl LintRule for DefaultParamLast {
  fn new() -> Box<Self> {
    Box::new(DefaultParamLast)
  }

  fn code(&self) -> &'static str {
    "defaultParamLast"
  }

  fn lint_module(&self, context: Context, module: swc_ecma_ast::Module) {
    let mut visitor = DefaultParamLastVisitor::new(context);
    visitor.visit_module(&module, &module);
  }
}

struct DefaultParamLastVisitor {
  context: Context,
}

impl DefaultParamLastVisitor {
  pub fn new(context: Context) -> Self {
    Self { context }
  }
}

impl Visit for DefaultParamLastVisitor {
  fn visit_function(&mut self, function: &Function, _parent: &dyn Node) {
    let mut has_normal_param = false;
    let pat = function
      .params
      .iter()
      .rev()
      .find_map(|param| match &param.pat {
        Pat::Assign(pat) => Some(pat),
        _ => {
          has_normal_param = true;
          None
        }
      });
    if has_normal_param {
      if let Some(pat) = pat {
        self.context.add_diagnostic(
          pat.span,
          "defaultParamLast",
          "default parameters should be at last",
        );
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::*;

  #[test]
  fn default_param_last_test() {
    assert_lint_err::<DefaultParamLast>("function fn(a = 2, b) {}", 12);
    assert_lint_ok_n::<DefaultParamLast>(vec![
      "function fn(a = 2, b = 3) {}",
      "function fn(a, b = 2) {}",
      "function fn(a, b) {}",
    ]);
  }
}
