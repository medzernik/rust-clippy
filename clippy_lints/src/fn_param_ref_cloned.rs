use clippy_utils::ty::implements_trait;
use rustc_hir::intravisit::HirTyCtxt;
use rustc_hir::{Body, PatKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyKind;
use rustc_session::impl_lint_pass;
use rustc_span::def_id::DefId;
use rustc_span::{sym, Span};

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

type ParameterIndex = usize;

#[derive(Default)]
pub struct FnParamRefClonedLate {}

pub fn is_candidate_ty<'a>(
    cx: &LateContext<'a>,
    ty: &rustc_middle::ty::Ty<'a>,
    must_impl_trait: &[DefId],
    cant_impl_trait: &[DefId],
) -> bool {
    match ty.kind() {
        rustc_middle::ty::TyKind::Ref(a, b, c) => {
            if must_impl_trait
                .iter()
                .any(|def_id| implements_trait(cx, *b, *def_id, &[]))
                && cant_impl_trait
                    .iter()
                    .all(|def_id| !implements_trait(cx, *b, *def_id, &[]))
            {
                true
            } else {
                false
            }
        },
        TyKind::FnDef(x, y) => {
            let def_id = cx.tcx.lang_items().clone_trait().unwrap();
            dbg!(def_id == *x);
            true
        },
        _ => false,
    }
}

pub fn get_param_id_span(cx: &LateContext<'_>, param: &rustc_hir::Param<'_>) -> Option<(rustc_hir::HirId, Span)> {
    match param.pat.kind {
        PatKind::Binding(a, b, c, _) => {
            if !c.span.from_expansion() && !c.is_reserved() {
                Some((b, param.ty_span))
            } else {
                None
            }
        },
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for FnParamRefClonedLate {
    fn check_body(&mut self, cx: &LateContext<'tcx>, fn_body: &Body<'tcx>) {
        let cant_impl_trait = [cx.tcx.lang_items().copy_trait().unwrap()];
        let must_impl_trait = [
            cx.tcx.lang_items().clone_trait().unwrap(),
            cx.tcx.lang_items().drop_trait().unwrap(),
        ];

        let def_id = cx.tcx.hir_body_owner_def_id(fn_body.id());

        // MIR signature
        let candidates: Vec<_> = cx
            .tcx
            .fn_sig(def_id)
            .instantiate_identity()
            .skip_binder()
            .inputs()
            .into_iter()
            .zip(fn_body.params)
            .filter_map(|(ty, param)| {
                if let Some((id, span)) = get_param_id_span(cx, param)
                    && is_candidate_ty(cx, ty, &must_impl_trait, &cant_impl_trait)
                {
                    Some((id, span))
                } else {
                    None
                }
            })
            .collect();

        for (id, span) in candidates.iter() {
            let id1 = cx.tcx.hir_body(fn_body.id()).value;
            dbg!(id1);
            // if id1.hir_id == *id {
                if let rustc_hir::ExprKind::MethodCall(path, _, _, _) = id1.kind
                    && path.ident.name == sym::Clone
                {
                    clippy_utils::diagnostics::span_lint_and_help(
                        cx,
                        FN_PARAM_REF_CLONED_INFO,
                        *span,
                        "function gets a parameter by reference, but you later clone it",
                        None,
                        "consider passing by value instead",
                    );
                }
            // }
        }
    }

    // fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
    //     if let rustc_hir::ExprKind::MethodCall(path, _, _, span) = &expr.kind
    //         && path.ident.name == sym::Clone
    //         && cx.ty_based_def(expr).opt_parent(cx).is_diag_item(cx, sym::Clone)
    //     {
    //         clippy_utils::diagnostics::span_lint_and_help(
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
