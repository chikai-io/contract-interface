pub trait ReplaceIdent {
    fn replace_ident(&mut self, from: &syn::Ident, to: &syn::Ident);
}

/// Replaces all and any identifier `Self` into `_State`, recursively
/// searching every place in which that substitution could happen.
pub fn replace_ident_from_self_to_state<T: ReplaceIdent>(t: &mut T) {
    use proc_macro2::Span;
    let _self = syn::Ident::new("Self", Span::call_site());
    let _state = syn::Ident::new("_State", Span::call_site());
    t.replace_ident(&_self, &_state);
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
            Member::Unnamed(u) => {}
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
            Pat::Macro(_p) => unimplemented!(),
            Pat::Or(p) => p.replace_ident(from, to),
            Pat::Path(p) => p.replace_ident(from, to),
            Pat::Range(p) => p.replace_ident(from, to),
            Pat::Reference(p) => p.replace_ident(from, to),
            Pat::Rest(_p) => unimplemented!(),
            Pat::Slice(p) => p.replace_ident(from, to),
            Pat::Struct(p) => p.replace_ident(from, to),
            Pat::Tuple(p) => p.replace_ident(from, to),
            Pat::TupleStruct(p) => p.replace_ident(from, to),
            Pat::Type(p) => p.replace_ident(from, to),
            Pat::Verbatim(_p) => unimplemented!(),
            Pat::Wild(_p) => unimplemented!(),
            _ => unimplemented!(),
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

//  (todo)
impl ReplaceIdent for syn::Expr {
    fn replace_ident(&mut self, _from: &syn::Ident, _to: &syn::Ident) {
        todo!()
        // use syn::Expr;
        // match self {
        //     Expr::Array(e) => e.replace_ident(from, to),
        //     Expr::Assign(e) => e.replace_ident(from, to),
        //     Expr::AssignOp(e) => e.replace_ident(from, to),
        //     Expr::Async(e) => e.replace_ident(from, to),
        //     Expr::Await(e) => e.replace_ident(from, to),
        //     Expr::Binary(e) => e.replace_ident(from, to),
        //     Expr::Block(e) => e.replace_ident(from, to),
        //     Expr::Box(e) => e.replace_ident(from, to),
        //     Expr::Break(e) => e.replace_ident(from, to),
        //     Expr::Call(e) => e.replace_ident(from, to),
        //     Expr::Cast(e) => e.replace_ident(from, to),
        //     Expr::Closure(e) => e.replace_ident(from, to),
        //     Expr::Continue(e) => e.replace_ident(from, to),
        //     Expr::Field(e) => e.replace_ident(from, to),
        //     Expr::ForLoop(e) => e.replace_ident(from, to),
        //     Expr::Group(e) => e.replace_ident(from, to),
        //     Expr::If(e) => e.replace_ident(from, to),
        //     Expr::Index(e) => e.replace_ident(from, to),
        //     Expr::Let(e) => e.replace_ident(from, to),
        //     Expr::Lit(e) => e.replace_ident(from, to),
        //     Expr::Loop(e) => e.replace_ident(from, to),
        //     Expr::Macro(e) => e.replace_ident(from, to),
        //     Expr::Match(e) => e.replace_ident(from, to),
        //     Expr::MethodCall(e) => e.replace_ident(from, to),
        //     Expr::Paren(e) => e.replace_ident(from, to),
        //     Expr::Path(e) => e.replace_ident(from, to),
        //     Expr::Range(e) => e.replace_ident(from, to),
        //     Expr::Reference(e) => e.replace_ident(from, to),
        //     Expr::Repeat(e) => e.replace_ident(from, to),
        //     Expr::Return(e) => e.replace_ident(from, to),
        //     Expr::Struct(e) => e.replace_ident(from, to),
        //     Expr::Try(e) => e.replace_ident(from, to),
        //     Expr::TryBlock(e) => e.replace_ident(from, to),
        //     Expr::Tuple(e) => e.replace_ident(from, to),
        //     Expr::Type(e) => e.replace_ident(from, to),
        //     Expr::Unary(e) => e.replace_ident(from, to),
        //     Expr::Unsafe(e) => e.replace_ident(from, to),
        //     Expr::Verbatim(e) => e.replace_ident(from, to),
        //     Expr::While(e) => e.replace_ident(from, to),
        //     Expr::Yield(e) => e.replace_ident(from, to),
        //     _ => unimplemented!(),
        // }
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
            Type::Verbatim(_t) => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}
