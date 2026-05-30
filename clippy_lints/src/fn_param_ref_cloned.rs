use clippy_utils::ty::implements_trait;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, FnDecl, PatKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyCtxt;
use rustc_session::impl_lint_pass;
use rustc_span::def_id::{DefId, LocalDefId};
use rustc_span::Span;

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
pub struct FnParamRefClonedLate {
    fn_decl_obj: Vec<ParameterIndex>,
}

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
        _ => false,
    }
}

pub fn get_param_id_span(cx: &LateContext<'_>, param: &rustc_hir::Param<'_>) -> Option<(rustc_hir::HirId, Span)> {
    match param.pat.kind {
        PatKind::Binding(a, b, c, d) => {
            if !c.span.from_expansion() {
                Some((b, param.ty_span))
            } else {
                None
            }
        },
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for FnParamRefClonedLate {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fn_kind: FnKind<'tcx>,
        fn_decl: &'tcx FnDecl<'tcx>,
        fn_body: &'tcx Body<'tcx>,
        _: Span,
        def_id: LocalDefId,
    ) {
        let cant_impl_trait = [cx.tcx.lang_items().copy_trait().unwrap()];
        let must_impl_trait = [
            cx.tcx.lang_items().clone_trait().unwrap(),
            cx.tcx.lang_items().drop_trait().unwrap(),
        ];

        // MIR signature
        let sig = TyCtxt::fn_sig(cx.tcx, def_id.to_def_id());
        let signature = sig.instantiate_identity().skip_binder();
        // TyCtxt::normalize_erasing_late_bound_regions()

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
            clippy_utils::diagnostics::span_lint_and_help(
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
