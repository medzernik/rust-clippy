use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Body, HirIdMap, ImplItemKind, ItemKind, Node, TraitItemKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    /// This is a test lint
    ///
    /// ### Why is this bad?
    /// The method should receive an owned value per spec.
    ///
    /// ### Example
    /// ```norun
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

impl_lint_pass!(FnParamRefClonedLate => [FN_PARAM_REF_CLONED_INFO]);

#[derive(Default)]
pub struct FnParamRefClonedLate {
    fn_decl_obj: HirIdMap<()>,
}

impl<'tcx> LateLintPass<'tcx> for FnParamRefClonedLate {
    fn check_body(&mut self, cx: &LateContext<'tcx>, body: &Body<'tcx>) {
        let mut parents = cx.tcx.hir_parent_iter(body.value.hir_id);
        let (item_id, sig, is_trait_item) = match parents.next() {
            Some((_, Node::Item(i))) => {
                if let ItemKind::Fn { sig, .. } = &i.kind {
                    (i.owner_id, sig, false)
                } else {
                    return;
                }
            },
            Some((_, Node::ImplItem(i))) => {
                if !matches!(parents.next(),
                    Some((_, Node::Item(i))) if matches!(&i.kind, ItemKind::Impl(i) if i.of_trait.is_none())
                ) {
                    return;
                }
                if let ImplItemKind::Fn(sig, _) = &i.kind {
                    (i.owner_id, sig, false)
                } else {
                    return;
                }
            },
            Some((_, Node::TraitItem(i))) => {
                if let TraitItemKind::Fn(sig, _) = &i.kind {
                    (i.owner_id, sig, true)
                } else {
                    return;
                }
            },
            _ => return,
        };


        let inputs = sig.decl.inputs;
        let body_params = body.params;
        dbg!(inputs);
        dbg!(body_params);


        span_lint_and_help(
            cx,
            FN_PARAM_REF_CLONED_INFO,
            body_params.first().unwrap().span,
            "function gets a parameter by reference, but you later clone it",
            None,
            "consider passing by value instead",
        );
    }

    // fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
    //     // Check our expr is calling a method with pattern matching
    //     if let ExprKind::MethodCall(path, _, _, span) = &expr.kind
    //         // Check if the name of this method is `our_fancy_method`
    //         && path.ident.name == sym::clone
    //         // Check if the method belongs to the `sym::OurFancyTrait` trait.
    //         // (for example, a `map` method could belong to user-defined trait instead of to
    // `Iterator`)         // See the next section for more information.
    //         && (
    //         cx.ty_based_def(expr).opt_parent(cx).is_diag_item(cx, sym::Clone)
    //         || cx.ty_based_def(expr).opt_parent(cx).is_diag_item(cx, sym::Copy))
    //     {
    //         println!("`expr` is a method call for `our_fancy_method`");
    //         span_lint_and_help(
    //             cx,
    //             FN_PARAM_REF_CLONED_INFO,
    //             *span,
    //             "function gets a parameter by reference, but you later clone it",
    //             None,
    //             "consider passing by value instead",
    //         );
    //     }
    // }
}
// fn has_matching_args(kind: FnKind, args: GenericArgsRef<'_>) -> bool {
//     match kind {
//         FnKind::Fn => true,
//         FnKind::TraitFn => args.iter().enumerate().all(|(idx, subst)| match subst.kind() {
//             GenericArgKind::Lifetime(_) => true,
//             GenericArgKind::Type(ty) => matches!(*ty.kind(), ty::Param(ty) if ty.index as usize
// == idx),             GenericArgKind::Const(c) => matches!(c.kind(), ConstKind::Param(c) if
// c.index as usize == idx),         }),
//         FnKind::ImplTraitFn(expected_args) => std::ptr::from_ref(args) as usize == expected_args,
//     }
// }
