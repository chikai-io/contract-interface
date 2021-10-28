pub trait ReplaceIdent {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident);
}

/// Replaces all and any identifier `Self` into `_State`, recursively
/// searching every place in which that substitution could happen.
pub fn replace_ident_from_self_to_state<T: ReplaceIdent>(t: &mut T) {
    use proc_macro2::Span;
    let _state = syn::Ident::new("_State", Span::call_site());
    replace_ident_from_self_to_ident(t, &_state)
}

/// Replaces all and any identifier `Self` into `_State`, recursively
/// searching every place in which that substitution could happen.
pub fn replace_ident_from_self_to_ident<T: ReplaceIdent>(t: &mut T, ident: &syn::Ident) {
    use proc_macro2::Span;
    let _self = syn::Ident::new("Self", Span::call_site());
    t.replace_ident(&_self, ident);
}

impl ReplaceIdent for syn::PatBox {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.pat.as_mut().replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::PatIdent {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.ident.replace_ident(from, to);
        if let Some((_, p)) = self.subpat.as_mut() {
            p.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::PatLit {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.expr.as_mut().replace_ident(from, to)
    }
}

impl ReplaceIdent for syn::PatOr {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for c in self.cases.iter_mut() {
            c.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::PatPath {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        if let Some(q) = self.qself.as_mut() {
            q.replace_ident(from, to);
        }
        self.path.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::PatRange {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.lo.as_mut().replace_ident(from, to);
        self.hi.as_mut().replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::PatReference {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.pat.as_mut().replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::PatSlice {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for e in self.elems.iter_mut() {
            e.replace_ident(from, to)
        }
    }
}

impl ReplaceIdent for syn::Member {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        use syn::Member;
        match self {
            Member::Named(n) => n.replace_ident(from, to),
            Member::Unnamed(_u) => {}
        }
    }
}

impl ReplaceIdent for syn::FieldPat {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.member.replace_ident(from, to);
        self.pat.as_mut().replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::PatStruct {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.path.replace_ident(from, to);
        for f in self.fields.iter_mut() {
            f.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::PatTuple {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for e in self.elems.iter_mut() {
            e.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::PatTupleStruct {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.path.replace_ident(from, to);
        self.pat.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::Pat {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        use syn::Pat;
        match self {
            Pat::Box(p) => p.replace_ident(from, to),
            Pat::Ident(p) => p.replace_ident(from, to),
            Pat::Lit(p) => p.replace_ident(from, to),
            Pat::Macro(p) => {
                unimplemented!("ReplaceIdent for Pat::Macro `{:?}` not yet implemented", p)
            }
            Pat::Or(p) => p.replace_ident(from, to),
            Pat::Path(p) => p.replace_ident(from, to),
            Pat::Range(p) => p.replace_ident(from, to),
            Pat::Reference(p) => p.replace_ident(from, to),
            Pat::Rest(p) => {
                unimplemented!("ReplaceIdent for Pat::Rest `{:?}` not yet implemented", p)
            }
            Pat::Slice(p) => p.replace_ident(from, to),
            Pat::Struct(p) => p.replace_ident(from, to),
            Pat::Tuple(p) => p.replace_ident(from, to),
            Pat::TupleStruct(p) => p.replace_ident(from, to),
            Pat::Type(p) => p.replace_ident(from, to),
            Pat::Verbatim(p) => {
                unimplemented!(
                    "ReplaceIdent for Pat::Verbatim `{:?}` not yet implemented",
                    p
                )
            }
            Pat::Wild(p) => {
                unimplemented!("ReplaceIdent for Pat::Wild `{:?}` not yet implemented", p)
            }
            p => unimplemented!("ReplaceIdent for unknown Pat `{:?}` not yet implemented", p),
        }
    }
}

impl ReplaceIdent for syn::PatType {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        let p = self.pat.as_mut();
        p.replace_ident(from, to);
        self.ty.as_mut().replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::TypeParam {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.ident.replace_ident(from, to);
        for b in self.bounds.iter_mut() {
            b.replace_ident(from, to);
        }
        if let Some(d) = self.default.as_mut() {
            d.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::ConstParam {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.ident.replace_ident(from, to);
        self.ty.replace_ident(from, to);
        if let Some(d) = self.default.as_mut() {
            d.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::GenericParam {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        use syn::GenericParam;
        match self {
            GenericParam::Type(t) => t.replace_ident(from, to),
            GenericParam::Lifetime(l) => l.replace_ident(from, to),
            GenericParam::Const(c) => c.replace_ident(from, to),
        }
    }
}

impl ReplaceIdent for syn::PredicateType {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        if let Some(l) = self.lifetimes.as_mut() {
            l.replace_ident(from, to);
        }
        self.bounded_ty.replace_ident(from, to);
        for b in self.bounds.iter_mut() {
            b.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::PredicateLifetime {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.lifetime.replace_ident(from, to);
        for b in self.bounds.iter_mut() {
            b.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::PredicateEq {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.lhs_ty.replace_ident(from, to);
        self.rhs_ty.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::WherePredicate {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        use syn::WherePredicate;
        match self {
            WherePredicate::Type(t) => t.replace_ident(from, to),
            WherePredicate::Lifetime(l) => l.replace_ident(from, to),
            WherePredicate::Eq(e) => e.replace_ident(from, to),
        }
    }
}

impl ReplaceIdent for syn::WhereClause {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for s in self.predicates.iter_mut() {
            s.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::Generics {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for p in self.params.iter_mut() {
            p.replace_ident(from, to);
        }
        if let Some(c) = self.where_clause.as_mut() {
            c.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::Ident {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        if self == from {
            *self = to.clone();
        }
    }
}

impl ReplaceIdent for syn::Lifetime {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.ident.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::Binding {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.ident.replace_ident(from, to);
        self.ty.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::LifetimeDef {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.lifetime.replace_ident(from, to);
        for b in self.bounds.iter_mut() {
            b.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::BoundLifetimes {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for l in self.lifetimes.iter_mut() {
            l.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::Path {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for s in self.segments.iter_mut() {
            s.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::TraitBound {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        if let Some(l) = &mut self.lifetimes {
            l.replace_ident(from, to);
        };
        self.path.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::TypeParamBound {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        use syn::TypeParamBound;
        match self {
            TypeParamBound::Trait(t) => t.replace_ident(from, to),
            TypeParamBound::Lifetime(l) => l.replace_ident(from, to),
        }
    }
}

impl ReplaceIdent for syn::Constraint {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.ident.replace_ident(from, to);
        for b in self.bounds.iter_mut() {
            b.replace_ident(from, to);
        }
    }
}

pub mod expr {
    use super::ReplaceIdent;

    impl ReplaceIdent for syn::ExprArray {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            for e in self.elems.iter_mut() {
                e.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::ExprAssign {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.left.replace_ident(from, to);
            self.right.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprAssignOp {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.left.replace_ident(from, to);
            self.right.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprAwait {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.base.replace_ident(from, to)
        }
    }

    impl ReplaceIdent for syn::ExprBinary {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.left.replace_ident(from, to);
            self.right.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprBox {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to)
        }
    }

    impl ReplaceIdent for syn::ExprBreak {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            if let Some(ref mut label) = self.label {
                label.replace_ident(from, to);
            }
            if let Some(ref mut expr) = self.expr {
                expr.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::ExprCall {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.func.replace_ident(from, to);
            for a in self.args.iter_mut() {
                a.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::ExprCast {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to);
            self.ty.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprClosure {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            for i in self.inputs.iter_mut() {
                i.replace_ident(from, to);
            }
            self.output.replace_ident(from, to);
            self.body.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprContinue {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            if let Some(ref mut label) = self.label {
                label.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::ExprField {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.base.replace_ident(from, to);
            self.member.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprGroup {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to)
        }
    }

    impl ReplaceIdent for syn::ExprIndex {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to);
            self.index.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprLet {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.pat.replace_ident(from, to);
            self.expr.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::Lit {
        fn replace_ident(&mut self, _from: &syn::Ident, _to: &syn::Ident) {
            use syn::Lit;
            match self {
                Lit::Str(_l) => {}
                Lit::ByteStr(_l) => {}
                Lit::Byte(_l) => {}
                Lit::Char(_l) => {}
                Lit::Int(_l) => {}
                Lit::Float(_l) => {}
                Lit::Bool(_l) => {}
                Lit::Verbatim(l) => {
                    unimplemented!(
                        "ReplaceIdent for Lit::Verbatim `{:?}` not yet implemented",
                        l
                    )
                }
            }
        }
    }

    impl ReplaceIdent for syn::ExprLit {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.lit.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::Arm {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.pat.replace_ident(from, to);
            if let Some((_, ref mut expr)) = self.guard {
                expr.replace_ident(from, to);
            }
            self.body.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprMatch {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to);
            for a in self.arms.iter_mut() {
                a.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::GenericMethodArgument {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            match self {
                syn::GenericMethodArgument::Type(t) => t.replace_ident(from, to),
                syn::GenericMethodArgument::Const(e) => e.replace_ident(from, to),
            }
        }
    }

    impl ReplaceIdent for syn::MethodTurbofish {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            for a in self.args.iter_mut() {
                a.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::ExprMethodCall {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.receiver.replace_ident(from, to);
            self.method.replace_ident(from, to);
            if let Some(ref mut turbo) = self.turbofish {
                turbo.replace_ident(from, to);
            }
            for a in self.args.iter_mut() {
                a.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::ExprParen {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to)
        }
    }

    impl ReplaceIdent for syn::ExprPath {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            if let Some(ref mut q) = self.qself {
                q.replace_ident(from, to);
            }
            self.path.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprRange {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            if let Some(ref mut f) = self.from {
                f.replace_ident(from, to);
            }
            if let Some(ref mut t) = self.to {
                t.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::ExprReference {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to)
        }
    }

    impl ReplaceIdent for syn::ExprRepeat {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to);
            self.len.replace_ident(from, to)
        }
    }

    impl ReplaceIdent for syn::ExprReturn {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            if let Some(ref mut r) = self.expr {
                r.replace_ident(from, to)
            }
        }
    }

    impl ReplaceIdent for syn::FieldValue {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.member.replace_ident(from, to);
            self.expr.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprStruct {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.path.replace_ident(from, to);
            for f in self.fields.iter_mut() {
                f.replace_ident(from, to);
            }
            if let Some(ref mut rest) = self.rest {
                rest.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::ExprTry {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprTuple {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            for e in self.elems.iter_mut() {
                e.replace_ident(from, to);
            }
        }
    }

    impl ReplaceIdent for syn::ExprType {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to);
            self.ty.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprUnary {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            self.expr.replace_ident(from, to);
        }
    }

    impl ReplaceIdent for syn::ExprYield {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            if let Some(ref mut expr) = self.expr {
                expr.replace_ident(from, to);
            }
        }
    }

    // todo
    impl ReplaceIdent for syn::Expr {
        fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
            use syn::Expr;
            match self {
                Expr::Array(e) => e.replace_ident(from, to),
                Expr::Assign(e) => e.replace_ident(from, to),
                Expr::AssignOp(e) => e.replace_ident(from, to),
                Expr::Async(e) => {
                    unimplemented!("ReplaceIdent for Expr::Async `{:?}` not yet implemented", e)
                }
                Expr::Await(e) => e.replace_ident(from, to),
                Expr::Binary(e) => e.replace_ident(from, to),
                Expr::Block(e) => {
                    unimplemented!("ReplaceIdent for Expr::Block `{:?}` not yet implemented", e)
                }
                Expr::Box(e) => e.replace_ident(from, to),
                Expr::Break(e) => e.replace_ident(from, to),
                Expr::Call(e) => e.replace_ident(from, to),
                Expr::Cast(e) => e.replace_ident(from, to),
                Expr::Closure(e) => e.replace_ident(from, to),
                Expr::Continue(e) => e.replace_ident(from, to),
                Expr::Field(e) => e.replace_ident(from, to),
                Expr::ForLoop(e) => {
                    unimplemented!(
                        "ReplaceIdent for Expr::ForLoop `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Group(e) => e.replace_ident(from, to),
                Expr::If(e) => {
                    unimplemented!("ReplaceIdent for Expr::If `{:?}` not yet implemented", e)
                }
                Expr::Index(e) => e.replace_ident(from, to),
                Expr::Let(e) => e.replace_ident(from, to),
                Expr::Lit(e) => e.replace_ident(from, to),
                Expr::Loop(e) => {
                    unimplemented!("ReplaceIdent for Expr::Loop `{:?}` not yet implemented", e)
                }
                Expr::Macro(e) => {
                    unimplemented!("ReplaceIdent for Expr::Macro `{:?}` not yet implemented", e)
                }
                Expr::Match(e) => e.replace_ident(from, to),
                Expr::MethodCall(e) => e.replace_ident(from, to),
                Expr::Paren(e) => e.replace_ident(from, to),
                Expr::Path(e) => e.replace_ident(from, to),
                Expr::Range(e) => e.replace_ident(from, to),
                Expr::Reference(e) => e.replace_ident(from, to),
                Expr::Repeat(e) => e.replace_ident(from, to),
                Expr::Return(e) => e.replace_ident(from, to),
                Expr::Struct(e) => e.replace_ident(from, to),
                Expr::Try(e) => e.replace_ident(from, to),
                Expr::TryBlock(e) => {
                    unimplemented!(
                        "ReplaceIdent for Expr::TryBlock `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Tuple(e) => e.replace_ident(from, to),
                Expr::Type(e) => e.replace_ident(from, to),
                Expr::Unary(e) => e.replace_ident(from, to),
                Expr::Unsafe(e) => {
                    unimplemented!(
                        "ReplaceIdent for Expr::Unsafe `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::Verbatim(e) => {
                    unimplemented!(
                        "ReplaceIdent for Expr::Verbatim `{:?}` not yet implemented",
                        e
                    )
                }
                Expr::While(e) => {
                    unimplemented!("ReplaceIdent for Expr::While `{:?}` not yet implemented", e)
                }
                Expr::Yield(e) => e.replace_ident(from, to),

                e => unimplemented!(
                    "ReplaceIdent for unknown Expr `{:?}` not yet implemented",
                    e
                ),
            }
        }
    }
}

impl ReplaceIdent for syn::GenericArgument {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        use syn::GenericArgument;
        match self {
            GenericArgument::Lifetime(l) => {
                l.replace_ident(from, to);
            }
            GenericArgument::Type(t) => {
                t.replace_ident(from, to);
            }
            GenericArgument::Binding(b) => {
                b.replace_ident(from, to);
            }
            GenericArgument::Constraint(c) => {
                c.replace_ident(from, to);
            }
            GenericArgument::Const(c) => {
                c.replace_ident(from, to);
            }
        }
    }
}

impl ReplaceIdent for syn::TypeArray {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.elem.as_mut().replace_ident(from, to);
        self.len.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::BareFnArg {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        if let Some((n, _)) = &mut self.name {
            n.replace_ident(from, to);
        }
        self.ty.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::TypeBareFn {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        if let Some(l) = &mut self.lifetimes {
            l.replace_ident(from, to);
        }
        for i in self.inputs.iter_mut() {
            i.replace_ident(from, to);
        }
        self.output.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::TypeGroup {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.elem.as_mut().replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::TypeImplTrait {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for b in self.bounds.iter_mut() {
            b.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::TypeParen {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.elem.as_mut().replace_ident(from, to)
    }
}

impl ReplaceIdent for syn::QSelf {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.ty.as_mut().replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::AngleBracketedGenericArguments {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for a in self.args.iter_mut() {
            a.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::ReturnType {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        use syn::ReturnType;
        match self {
            ReturnType::Default => {}
            ReturnType::Type(_arrow, t) => {
                t.as_mut().replace_ident(from, to);
            }
        }
    }
}

impl ReplaceIdent for syn::ParenthesizedGenericArguments {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for i in self.inputs.iter_mut() {
            i.replace_ident(from, to);
        }
        self.output.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::PathSegment {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        use syn::PathArguments;
        self.ident.replace_ident(from, to);
        match &mut self.arguments {
            PathArguments::None => {}
            PathArguments::AngleBracketed(a) => {
                a.replace_ident(from, to);
            }
            PathArguments::Parenthesized(a) => {
                a.replace_ident(from, to);
            }
        }
    }
}

impl ReplaceIdent for syn::TypePath {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        if let Some(ref mut q) = self.qself {
            q.replace_ident(from, to);
        }
        self.path.replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::TypePtr {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.elem.as_mut().replace_ident(from, to)
    }
}

impl ReplaceIdent for syn::TypeReference {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        if let Some(l) = &mut self.lifetime {
            l.replace_ident(from, to);
        }
        self.elem.as_mut().replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::TypeSlice {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        self.elem.as_mut().replace_ident(from, to);
    }
}

impl ReplaceIdent for syn::TypeTraitObject {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for b in self.bounds.iter_mut() {
            b.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::TypeTuple {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        for e in self.elems.iter_mut() {
            e.replace_ident(from, to);
        }
    }
}

impl ReplaceIdent for syn::Type {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident) {
        use syn::Type;
        match self {
            Type::Array(t) => t.replace_ident(from, to),
            Type::BareFn(t) => t.replace_ident(from, to),
            Type::Group(t) => t.replace_ident(from, to),
            Type::ImplTrait(t) => t.replace_ident(from, to),
            Type::Infer(_t) => {}
            Type::Macro(_t) => {}
            Type::Never(_t) => {}
            Type::Paren(t) => t.replace_ident(from, to),
            Type::Path(t) => t.replace_ident(from, to),
            Type::Ptr(t) => t.replace_ident(from, to),
            Type::Reference(t) => t.replace_ident(from, to),
            Type::Slice(t) => t.replace_ident(from, to),
            Type::TraitObject(t) => t.replace_ident(from, to),
            Type::Tuple(t) => t.replace_ident(from, to),
            Type::Verbatim(t) => {
                unimplemented!(
                    "ReplaceIdent for Type::Verbatim `{:?}` not yet implemented",
                    t
                )
            }
            t => unimplemented!(
                "ReplaceIdent for unknown Type `{:?}` not yet implemented",
                t
            ),
        }
    }
}
