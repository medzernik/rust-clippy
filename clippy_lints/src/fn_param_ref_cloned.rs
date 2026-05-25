use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::res::{MaybeDef, MaybeTypeckRes};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    /// This is a test lint
    ///
    /// ### Why is this bad?
    /// The method should receive an owned value per spec.
    ///
    /// ### Example
    /// ```rust
    /// #[derive(Clone)]
    /// struct A;
    ///
    /// pub fn foo(item: &A) {
    ///     let cloned_ref = item.clone();
    /// }
    /// ```
    #[clippy::version = "1.97.0"]
    pub FN_PARAM_REF_CLONED_INFO,
    pedantic,
    "you should pass by value instead of cloning a passed reference"
}

declare_lint_pass!(FnParamRefClonedLate => [FN_PARAM_REF_CLONED_INFO]);

impl<'tcx> LateLintPass<'tcx> for FnParamRefClonedLate {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        // Check our expr is calling a method with pattern matching
        if let ExprKind::MethodCall(path, _, _, span) = &expr.kind
            // Check if the name of this method is `our_fancy_method`
            && path.ident.name == sym::clone
            // Check if the method belongs to the `sym::OurFancyTrait` trait.
            // (for example, a `map` method could belong to user-defined trait instead of to `Iterator`)
            // See the next section for more information.
            && cx.ty_based_def(expr).opt_parent(cx).is_diag_item(cx, sym::Clone)
        {
            println!("`expr` is a method call for `our_fancy_method`");
            span_lint_and_help(
                cx,
                FN_PARAM_REF_CLONED_INFO,
                *span,
                "function gets a parameter by reference, but you later clone it",
                None,
                "consider passing by value instead",
            );
        }
    }
}

// impl LateLintPass<'_> for FnParamRefClonedLate {
//     fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'_ Expr<'_>) {
//         // Check our expr is calling a method
//         if let hir::ExprKind::MethodCall(path, _, _self_arg, ..) = &expr.kind
//             // Check the name of this method is `some_method`
//             && path.ident.name == sym::clone
//         // Optionally, check the type of the self argument.
//         // - See "Checking for a specific type"
//         {
//             // ...
//         }
//     }
// fn check_fn(
//     &mut self,
//     cx: &LateContext<'_>,
//     fn_kind: FnKind<'_>,
//     fn_decl: &'_ FnDecl<'_>,
//     fn_body: &'_ Body<'_>,
//     span: Span,
//     _: LocalDefId,
// ) {
//
//     if is_foo_fn(cx, fn_kind, fn_decl, fn_body) {
//         span_lint_and_help(
//             cx,
//             FOO_FUNCTIONS_LATE,
//             span,
//             "function named `foo`",
//             None,
//             "consider using a more meaningful name",
//         );
//     }
// }
// }

// fn is_foo_fn(cx: &LateContext<'_>, fn_kind: FnKind<'_>, fn_decl: &FnDecl<'_>, fn_body: &Body<'_>)
// -> bool {     let mut header_items = HashMap::new();
//     match fn_kind {
//         FnKind::ItemFn(ident, _, header) => {
//
//             // the stuff should have been done by someone
//             for item in fn_decl.inputs.iter() {
//                 match item.kind {
//                     TyKind::Ref(_, param) => {
//                         header_items.insert(param.ty.hir_id, ());
//                     }
//                     _ => ()
//                 }
//             }
//
//             for item in fn_body.params.iter() {
//                 let id = item.hir_id;
//                 if header_items.get(&id).is_some() {
//                     // item.
//                     // item.pat.
//                    // cx.typeck_results().expr_ty(&ExprId::from(id))
//                 }
//             }
//
//             // check if `fn` name is `foo`
//             ident.name.as_str() == "foo"
//         },
//         // ignore closures
//         _ => false,
//     }
// }
