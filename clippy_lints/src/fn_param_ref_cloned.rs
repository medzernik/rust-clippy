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

pub fn is_candidate_ty<'a>(cx: &LateContext<'a>, ty: &rustc_middle::ty::Ty<'a>, clone_trait_id: DefId) -> bool {
    match ty.kind() {
        rustc_middle::ty::TyKind::Ref(a, b, c) => {
            if clippy_utils::ty::implements_trait(cx, *b, clone_trait_id, &[]) {
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
                Some((b, c.span))
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
        let mut ref_position = FnParamRefClonedLate::default();
        let clone_trait_id = cx.tcx.lang_items().clone_trait().unwrap();

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
                    && is_candidate_ty(cx, ty, clone_trait_id)
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

        // for (iter, item) in fn_decl.inputs.iter().enumerate() {
        //     match item.kind {
        //         TyKind::Ref(_, reference) if !item.span.from_expansion() => {
        //             dbg!(signature.inputs().get(iter).unwrap());
        //             let clone_trait_id = cx.tcx.lang_items().clone_trait().unwrap();
        //
        //             if clippy_utils::ty::implements_trait(
        //                 cx,
        //                 *signature.inputs().get(iter).unwrap(),
        //                 clone_trait_id,
        //                 &[],
        //             ) {
        //                 dbg!(true);
        //                 ref_position.fn_decl_obj.push(iter);
        //             }
        //         },
        //         _ => (),
        //     }
        // }

        // for iter in ref_position.fn_decl_obj.iter() {
        //     let param = fn_body.params.get(*iter).unwrap();
        //
        //     match param.pat.kind {
        //         PatKind::Binding(a, b, c, d) => {
        //             dbg!({ a });
        //             dbg!({ b });
        //             dbg!({ c });
        //             dbg!({ d });
        //
        //             clippy_utils::diagnostics::span_lint_and_help(
        //                 cx,
        //                 FN_PARAM_REF_CLONED_INFO,
        //                 fn_decl.inputs.get(*iter).unwrap().span,
        //                 "function gets a parameter by reference, but you later clone it",
        //                 None,
        //                 "consider passing by value instead",
        //             );
        //         },
        //         _ => {},
        //     }
        // }
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
