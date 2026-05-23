use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::intravisit::FnKind;
use rustc_hir::*;
use rustc_infer::infer::canonical::ir::TypeVisitableExt;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::Span;
use rustc_span::def_id::LocalDefId;


declare_clippy_lint! {
    /// ### What it does
    /// This is a test lint
    ///
    /// ### Why is this bad?
    /// Foo is not descriptive
    ///
    /// ### Example
    /// ```rust
    /// pub fn foo() -> String {
    ///     "hey".to_string()
    /// }
    /// ```
    #[clippy::version = "1.97.0"]
    pub FOO_FUNCTIONS_LATE,
    pedantic,
    "function named `foo`, which is not a descriptive name"
}

declare_lint_pass!(FooFunctionsLate => [FOO_FUNCTIONS_LATE]);

impl LateLintPass<'_> for FooFunctionsLate {
    fn check_fn(
        &mut self,
        cx: &LateContext<'_>,
        fn_kind: FnKind<'_>,
        fn_decl: &'_ FnDecl<'_>,
        _: &'_ Body<'_>,
        span: Span,
        _: LocalDefId,
    ) {
        if is_foo_fn(fn_kind) {
            span_lint_and_help(
                cx,
                FOO_FUNCTIONS_LATE,
                span,
                "function named `foo`",
                None,
                "consider using a more meaningful name",
            );
        }
    }
}

fn is_foo_fn(fn_kind: FnKind<'_>, fn_decl: FnDecl<'_>) -> bool {
    match fn_kind {
        FnKind::ItemFn(ident, _, header) => {
           let has_param = ident.has_param();
            // ident.span.references_error()
            fn_decl.inputs.iter()

            // check if `fn` name is `foo`
            ident.name.as_str() == "foo"
        },
        // ignore closures
        _ => false,
    }
}