// syn::Type
// syn::Type::Path
// syn::TypePath

pub trait ReplaceTypeIdent {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool;
}

/// Replaces all and any identifier `Self` into `to_ty`, recursively
/// searching every place in which that substitution could happen.
pub fn replace_ident_from_self_to_type<T: ReplaceTypeIdent>(t: &mut T, to_ty: &syn::Type) {
    use proc_macro2::Span;
    let _self = syn::Ident::new("Self", Span::call_site());
    t.replace_type_ident(&_self, to_ty);
}

// /// Replaces all and any identifier `Self` into `_State`, recursively
// /// searching every place in which that substitution could happen.
// pub fn replace_type_ident_from_self_to_state<T: ReplaceTypeIdent>(t: &mut T) {
//     use proc_macro2::Span;
//     let _state = syn::Ident::new("_State", Span::call_site());
//     replace_type_ident_from_self_to_ident(t, &_state)
// }

// /// Replaces all and any identifier `Self` into `_State`, recursively
// /// searching every place in which that substitution could happen.
// pub fn replace_type_ident_from_self_to_ident<T: ReplaceTypeIdent>(t: &mut T, ident: &syn::Ident) {
//     use proc_macro2::Span;
//     let _self = syn::Ident::new("Self", Span::call_site());
//     t.replace_type_ident(&_self, ident);
// }

impl ReplaceTypeIdent for syn::PatBox {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.pat.as_mut().replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::PatIdent {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        if let Some((_, p)) = self.subpat.as_mut() {
            r |= p.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::PatLit {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.expr.as_mut().replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::PatOr {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for c in self.cases.iter_mut() {
            r |= c.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::PatPath {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        if let Some(q) = self.qself.as_mut() {
            r |= q.replace_type_ident(ident, to_ty);
        }
        r |= self.path.replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::PatRange {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        r |= self.lo.as_mut().replace_type_ident(ident, to_ty);
        r |= self.hi.as_mut().replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::PatReference {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.pat.as_mut().replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::PatSlice {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for e in self.elems.iter_mut() {
            r |= e.replace_type_ident(ident, to_ty)
        }
        r
    }
}

impl ReplaceTypeIdent for syn::Member {
    fn replace_type_ident(&mut self, _ident: &syn::Ident, _to_ty: &syn::Type) -> bool {
        use syn::Member;
        match self {
            Member::Named(_n) => false,
            Member::Unnamed(_u) => false,
        }
    }
}

impl ReplaceTypeIdent for syn::FieldPat {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        r |= self.member.replace_type_ident(ident, to_ty);
        r |= self.pat.as_mut().replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::PatStruct {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        r |= self.path.replace_type_ident(ident, to_ty);
        for f in self.fields.iter_mut() {
            r |= f.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::PatTuple {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for e in self.elems.iter_mut() {
            r |= e.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::PatTupleStruct {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        r |= self.path.replace_type_ident(ident, to_ty);
        r |= self.pat.replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::Pat {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        use syn::Pat;
        match self {
            Pat::Box(p) => p.replace_type_ident(ident, to_ty),
            Pat::Ident(p) => p.replace_type_ident(ident, to_ty),
            Pat::Lit(p) => p.replace_type_ident(ident, to_ty),
            Pat::Macro(p) => {
                unimplemented!(
                    "ReplaceTypeIdent for Pat::Macro `{:?}` not yet implemented",
                    p
                )
            }
            Pat::Or(p) => p.replace_type_ident(ident, to_ty),
            Pat::Path(p) => p.replace_type_ident(ident, to_ty),
            Pat::Range(p) => p.replace_type_ident(ident, to_ty),
            Pat::Reference(p) => p.replace_type_ident(ident, to_ty),
            Pat::Rest(p) => {
                unimplemented!(
                    "ReplaceTypeIdent for Pat::Rest `{:?}` not yet implemented",
                    p
                )
            }
            Pat::Slice(p) => p.replace_type_ident(ident, to_ty),
            Pat::Struct(p) => p.replace_type_ident(ident, to_ty),
            Pat::Tuple(p) => p.replace_type_ident(ident, to_ty),
            Pat::TupleStruct(p) => p.replace_type_ident(ident, to_ty),
            Pat::Type(p) => p.replace_type_ident(ident, to_ty),
            Pat::Verbatim(p) => {
                unimplemented!(
                    "ReplaceTypeIdent for Pat::Verbatim `{:?}` not yet implemented",
                    p
                )
            }
            Pat::Wild(p) => {
                unimplemented!(
                    "ReplaceTypeIdent for Pat::Wild `{:?}` not yet implemented",
                    p
                )
            }
            p => unimplemented!(
                "ReplaceTypeIdent for unknown Pat `{:?}` not yet implemented",
                p
            ),
        }
    }
}

impl ReplaceTypeIdent for syn::PatType {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        let p = self.pat.as_mut();
        r |= p.replace_type_ident(ident, to_ty);
        r |= self.ty.as_mut().replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::TypeParam {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for b in self.bounds.iter_mut() {
            r |= b.replace_type_ident(ident, to_ty);
        }
        if let Some(d) = self.default.as_mut() {
            r |= d.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::ConstParam {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        r |= self.ty.replace_type_ident(ident, to_ty);
        if let Some(d) = self.default.as_mut() {
            r |= d.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::GenericParam {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        use syn::GenericParam;
        match self {
            GenericParam::Type(t) => t.replace_type_ident(ident, to_ty),
            GenericParam::Lifetime(_l) => false,
            GenericParam::Const(c) => c.replace_type_ident(ident, to_ty),
        }
    }
}

impl ReplaceTypeIdent for syn::PredicateType {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        r |= self.bounded_ty.replace_type_ident(ident, to_ty);
        for b in self.bounds.iter_mut() {
            r |= b.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::PredicateLifetime {
    fn replace_type_ident(&mut self, _ident: &syn::Ident, _to_ty: &syn::Type) -> bool {
        false
    }
}

impl ReplaceTypeIdent for syn::PredicateEq {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        r |= self.lhs_ty.replace_type_ident(ident, to_ty);
        r |= self.rhs_ty.replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::WherePredicate {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        use syn::WherePredicate;
        match self {
            WherePredicate::Type(t) => t.replace_type_ident(ident, to_ty),
            WherePredicate::Lifetime(l) => l.replace_type_ident(ident, to_ty),
            WherePredicate::Eq(e) => e.replace_type_ident(ident, to_ty),
        }
    }
}

impl ReplaceTypeIdent for syn::WhereClause {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for s in self.predicates.iter_mut() {
            r |= s.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::Generics {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for p in self.params.iter_mut() {
            r |= p.replace_type_ident(ident, to_ty);
        }
        if let Some(c) = self.where_clause.as_mut() {
            r |= c.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::Binding {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.ty.replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::Path {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for s in self.segments.iter_mut() {
            r |= s.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::TraitBound {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.path.replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::TypeParamBound {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        use syn::TypeParamBound;
        match self {
            TypeParamBound::Trait(t) => t.replace_type_ident(ident, to_ty),
            TypeParamBound::Lifetime(_l) => false,
        }
    }
}

impl ReplaceTypeIdent for syn::Constraint {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for b in self.bounds.iter_mut() {
            r |= b.replace_type_ident(ident, to_ty);
        }
        r
    }
}

pub mod expr {
    use super::ReplaceTypeIdent;

    impl ReplaceTypeIdent for syn::ExprArray {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            for e in self.elems.iter_mut() {
                r |= e.replace_type_ident(ident, to_ty);
            }
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprAssign {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.left.replace_type_ident(ident, to_ty);
            r |= self.right.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprAssignOp {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.left.replace_type_ident(ident, to_ty);
            r |= self.right.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprAwait {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            self.base.replace_type_ident(ident, to_ty)
        }
    }

    impl ReplaceTypeIdent for syn::ExprBinary {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.left.replace_type_ident(ident, to_ty);
            r |= self.right.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprBox {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            self.expr.replace_type_ident(ident, to_ty)
        }
    }

    impl ReplaceTypeIdent for syn::ExprBreak {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            if let Some(ref mut expr) = self.expr {
                r |= expr.replace_type_ident(ident, to_ty);
            }
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprCall {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.func.replace_type_ident(ident, to_ty);
            for a in self.args.iter_mut() {
                r |= a.replace_type_ident(ident, to_ty);
            }
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprCast {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.expr.replace_type_ident(ident, to_ty);
            r |= self.ty.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprClosure {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            for i in self.inputs.iter_mut() {
                r |= i.replace_type_ident(ident, to_ty);
            }
            r |= self.output.replace_type_ident(ident, to_ty);
            r |= self.body.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprContinue {
        fn replace_type_ident(&mut self, _ident: &syn::Ident, _to_ty: &syn::Type) -> bool {
            false
        }
    }

    impl ReplaceTypeIdent for syn::ExprField {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.base.replace_type_ident(ident, to_ty);
            r |= self.member.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprGroup {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            self.expr.replace_type_ident(ident, to_ty)
        }
    }

    impl ReplaceTypeIdent for syn::ExprIndex {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.expr.replace_type_ident(ident, to_ty);
            r |= self.index.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprLet {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.pat.replace_type_ident(ident, to_ty);
            r |= self.expr.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::Lit {
        fn replace_type_ident(&mut self, _ident: &syn::Ident, _to_ty: &syn::Type) -> bool {
            use syn::Lit;
            match self {
                Lit::Str(_l) => false,
                Lit::ByteStr(_l) => false,
                Lit::Byte(_l) => false,
                Lit::Char(_l) => false,
                Lit::Int(_l) => false,
                Lit::Float(_l) => false,
                Lit::Bool(_l) => false,
                Lit::Verbatim(l) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Lit::Verbatim `{:?}` not yet implemented",
                        l
                    )
                }
            }
        }
    }

    impl ReplaceTypeIdent for syn::ExprLit {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            self.lit.replace_type_ident(ident, to_ty)
        }
    }

    impl ReplaceTypeIdent for syn::Arm {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.pat.replace_type_ident(ident, to_ty);
            if let Some((_, ref mut expr)) = self.guard {
                r |= expr.replace_type_ident(ident, to_ty);
            }
            r |= self.body.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprMatch {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.expr.replace_type_ident(ident, to_ty);
            for a in self.arms.iter_mut() {
                r |= a.replace_type_ident(ident, to_ty);
            }
            r
        }
    }

    impl ReplaceTypeIdent for syn::GenericMethodArgument {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            match self {
                syn::GenericMethodArgument::Type(t) => t.replace_type_ident(ident, to_ty),
                syn::GenericMethodArgument::Const(e) => e.replace_type_ident(ident, to_ty),
            }
        }
    }

    impl ReplaceTypeIdent for syn::MethodTurbofish {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            for a in self.args.iter_mut() {
                r |= a.replace_type_ident(ident, to_ty);
            }
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprMethodCall {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.receiver.replace_type_ident(ident, to_ty);
            if let Some(ref mut turbo) = self.turbofish {
                r |= turbo.replace_type_ident(ident, to_ty);
            }
            for a in self.args.iter_mut() {
                r |= a.replace_type_ident(ident, to_ty);
            }
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprParen {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            self.expr.replace_type_ident(ident, to_ty)
        }
    }

    impl ReplaceTypeIdent for syn::ExprPath {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            if let Some(ref mut q) = self.qself {
                r |= q.replace_type_ident(ident, to_ty);
            }
            r |= self.path.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprRange {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            if let Some(ref mut f) = self.from {
                r |= f.replace_type_ident(ident, to_ty);
            }
            if let Some(ref mut t) = self.to {
                r |= t.replace_type_ident(ident, to_ty);
            }
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprReference {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            self.expr.replace_type_ident(ident, to_ty)
        }
    }

    impl ReplaceTypeIdent for syn::ExprRepeat {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.expr.replace_type_ident(ident, to_ty);
            r |= self.len.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprReturn {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            if let Some(ref mut r) = self.expr {
                r.replace_type_ident(ident, to_ty)
            } else {
                false
            }
        }
    }

    impl ReplaceTypeIdent for syn::FieldValue {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.member.replace_type_ident(ident, to_ty);
            r |= self.expr.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprStruct {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            self.path.replace_type_ident(ident, to_ty);
            for f in self.fields.iter_mut() {
                r |= f.replace_type_ident(ident, to_ty);
            }
            if let Some(ref mut rest) = self.rest {
                r |= rest.replace_type_ident(ident, to_ty);
            }
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprTry {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            self.expr.replace_type_ident(ident, to_ty)
        }
    }

    impl ReplaceTypeIdent for syn::ExprTuple {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            for e in self.elems.iter_mut() {
                r |= e.replace_type_ident(ident, to_ty);
            }
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprType {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            let mut r = false;
            r |= self.expr.replace_type_ident(ident, to_ty);
            r |= self.ty.replace_type_ident(ident, to_ty);
            r
        }
    }

    impl ReplaceTypeIdent for syn::ExprUnary {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            self.expr.replace_type_ident(ident, to_ty)
        }
    }

    impl ReplaceTypeIdent for syn::ExprYield {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            if let Some(ref mut expr) = self.expr {
                expr.replace_type_ident(ident, to_ty)
            } else {
                false
            }
        }
    }

    // todo
    impl ReplaceTypeIdent for syn::Expr {
        fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
            use syn::Expr;
            match self {
                Expr::Array(e) => e.replace_type_ident(ident, to_ty),
                Expr::Assign(e) => e.replace_type_ident(ident, to_ty),
                Expr::AssignOp(e) => e.replace_type_ident(ident, to_ty),
                Expr::Async(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::Async `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Await(e) => e.replace_type_ident(ident, to_ty),
                Expr::Binary(e) => e.replace_type_ident(ident, to_ty),
                Expr::Block(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::Block `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Box(e) => e.replace_type_ident(ident, to_ty),
                Expr::Break(e) => e.replace_type_ident(ident, to_ty),
                Expr::Call(e) => e.replace_type_ident(ident, to_ty),
                Expr::Cast(e) => e.replace_type_ident(ident, to_ty),
                Expr::Closure(e) => e.replace_type_ident(ident, to_ty),
                Expr::Continue(e) => e.replace_type_ident(ident, to_ty),
                Expr::Field(e) => e.replace_type_ident(ident, to_ty),
                Expr::ForLoop(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::ForLoop `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Group(e) => e.replace_type_ident(ident, to_ty),
                Expr::If(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::If `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Index(e) => e.replace_type_ident(ident, to_ty),
                Expr::Let(e) => e.replace_type_ident(ident, to_ty),
                Expr::Lit(e) => e.replace_type_ident(ident, to_ty),
                Expr::Loop(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::Loop `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Macro(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::Macro `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Match(e) => e.replace_type_ident(ident, to_ty),
                Expr::MethodCall(e) => e.replace_type_ident(ident, to_ty),
                Expr::Paren(e) => e.replace_type_ident(ident, to_ty),
                Expr::Path(e) => e.replace_type_ident(ident, to_ty),
                Expr::Range(e) => e.replace_type_ident(ident, to_ty),
                Expr::Reference(e) => e.replace_type_ident(ident, to_ty),
                Expr::Repeat(e) => e.replace_type_ident(ident, to_ty),
                Expr::Return(e) => e.replace_type_ident(ident, to_ty),
                Expr::Struct(e) => e.replace_type_ident(ident, to_ty),
                Expr::Try(e) => e.replace_type_ident(ident, to_ty),
                Expr::TryBlock(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::TryBlock `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Tuple(e) => e.replace_type_ident(ident, to_ty),
                Expr::Type(e) => e.replace_type_ident(ident, to_ty),
                Expr::Unary(e) => e.replace_type_ident(ident, to_ty),
                Expr::Unsafe(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::Unsafe `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Verbatim(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::Verbatim `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::While(e) => {
                    unimplemented!(
                        "ReplaceTypeIdent for Expr::While `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Yield(e) => e.replace_type_ident(ident, to_ty),

                e => unimplemented!(
                    "ReplaceTypeIdent for unknown Expr `{:?}` not yet implemented",
                    e
                ),
            }
        }
    }
}

impl ReplaceTypeIdent for syn::GenericArgument {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        use syn::GenericArgument;
        match self {
            GenericArgument::Lifetime(_l) => false,
            GenericArgument::Type(t) => t.replace_type_ident(ident, to_ty),
            GenericArgument::Binding(b) => b.replace_type_ident(ident, to_ty),
            GenericArgument::Constraint(c) => c.replace_type_ident(ident, to_ty),
            GenericArgument::Const(c) => c.replace_type_ident(ident, to_ty),
        }
    }
}

impl ReplaceTypeIdent for syn::TypeArray {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        r |= self.elem.as_mut().replace_type_ident(ident, to_ty);
        r |= self.len.replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::BareFnArg {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.ty.replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::TypeBareFn {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for i in self.inputs.iter_mut() {
            r |= i.replace_type_ident(ident, to_ty);
        }
        r |= self.output.replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::TypeGroup {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.elem.as_mut().replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::TypeImplTrait {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for b in self.bounds.iter_mut() {
            r |= b.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::TypeParen {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.elem.as_mut().replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::QSelf {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.ty.as_mut().replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::AngleBracketedGenericArguments {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for a in self.args.iter_mut() {
            r |= a.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::ReturnType {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        use syn::ReturnType;
        match self {
            ReturnType::Default => false,
            ReturnType::Type(_arrow, t) => t.as_mut().replace_type_ident(ident, to_ty),
        }
    }
}

impl ReplaceTypeIdent for syn::ParenthesizedGenericArguments {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for i in self.inputs.iter_mut() {
            r |= i.replace_type_ident(ident, to_ty);
        }
        r |= self.output.replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::PathSegment {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        use syn::PathArguments;
        match &mut self.arguments {
            PathArguments::None => false,
            PathArguments::AngleBracketed(a) => a.replace_type_ident(ident, to_ty),
            PathArguments::Parenthesized(a) => a.replace_type_ident(ident, to_ty),
        }
    }
}

impl ReplaceTypeIdent for syn::TypePath {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        if let Some(ref mut q) = self.qself {
            r |= q.replace_type_ident(ident, to_ty);
        }
        r |= self.path.replace_type_ident(ident, to_ty);
        r
    }
}

impl ReplaceTypeIdent for syn::TypePtr {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.elem.as_mut().replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::TypeReference {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.elem.as_mut().replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::TypeSlice {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        self.elem.as_mut().replace_type_ident(ident, to_ty)
    }
}

impl ReplaceTypeIdent for syn::TypeTraitObject {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for b in self.bounds.iter_mut() {
            r |= b.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::TypeTuple {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        let mut r = false;
        for e in self.elems.iter_mut() {
            r |= e.replace_type_ident(ident, to_ty);
        }
        r
    }
}

impl ReplaceTypeIdent for syn::Type {
    fn replace_type_ident(&mut self, ident: &syn::Ident, to_ty: &syn::Type) -> bool {
        use syn::Type;
        match self {
            Type::Array(t) => t.replace_type_ident(ident, to_ty),
            Type::BareFn(t) => t.replace_type_ident(ident, to_ty),
            Type::Group(t) => t.replace_type_ident(ident, to_ty),
            Type::ImplTrait(t) => t.replace_type_ident(ident, to_ty),
            Type::Infer(_t) => false,
            Type::Macro(_t) => false,
            Type::Never(_t) => false,
            Type::Paren(t) => t.replace_type_ident(ident, to_ty),
            Type::Path(t) => {
                if t.qself.is_none() && t.path.is_ident(ident) {
                    *self = to_ty.clone();
                    true
                } else {
                    t.replace_type_ident(ident, to_ty)
                }
            }
            Type::Ptr(t) => t.replace_type_ident(ident, to_ty),
            Type::Reference(t) => t.replace_type_ident(ident, to_ty),
            Type::Slice(t) => t.replace_type_ident(ident, to_ty),
            Type::TraitObject(t) => t.replace_type_ident(ident, to_ty),
            Type::Tuple(t) => t.replace_type_ident(ident, to_ty),
            Type::Verbatim(t) => {
                unimplemented!(
                    "ReplaceTypeIdent for Type::Verbatim `{:?}` not yet implemented",
                    t
                )
            }
            t => unimplemented!(
                "ReplaceTypeIdent for unknown Type `{:?}` not yet implemented",
                t
            ),
        }
    }
}
