extern crate swc_common;
extern crate swc_ecma_parser;

use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

use crate::es_features::EsFeature;

#[derive(Default)]
pub struct FeatureFinder {
    // used to detect top level ´await´
    in_function: bool,
    set: HashSet<EsFeature>,
}

impl Debug for FeatureFinder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for e in &self.set {
            write!(f, "{}", e)?
        }
        Ok(())
    }
}

impl FeatureFinder {
    pub fn get_result(&self) -> HashSet<EsFeature> {
        self.set.to_owned()
    }
}

impl Visit for FeatureFinder {
    fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
        let prev_in_function = self.in_function;
        self.in_function = true;
        n.visit_children_with(self);
        self.in_function = prev_in_function;
    }

    fn visit_assign_op(&mut self, n: &AssignOp) {
        if let AssignOp::ExpAssign = &n {
            self.set.insert(EsFeature::ExponentiationOperator);
        }
        if let AssignOp::NullishAssign = &n {
            self.set.insert(EsFeature::LogicalAssignmentOperators);
        }
        if let AssignOp::AndAssign = &n {
            self.set.insert(EsFeature::LogicalAssignmentOperators);
        }
        if let AssignOp::OrAssign = &n {
            self.set.insert(EsFeature::LogicalAssignmentOperators);
        }
        n.visit_children_with(self)
    }

    fn visit_await_expr(&mut self, n: &AwaitExpr) {
        if !self.in_function {
            // wait on top level
            self.set.insert(EsFeature::TopLevelAwait);
        } else {
            // wait inside a function
            self.set.insert(EsFeature::AsyncFunctions);
        }
        n.visit_children_with(self)
    }

    fn visit_bin_expr(&mut self, n: &BinExpr) {
        if let Expr::PrivateName(_) = n.left.deref() {
            // Check for the 'in' operator which is commonly used for brand checks
            if let BinaryOp::In = &n.op {
                self.set
                    .insert(EsFeature::ErgonomicBrandChecksForPrivateFields);
            }
        }
        n.visit_children_with(self)
    }

    fn visit_binary_op(&mut self, n: &BinaryOp) {
        if let BinaryOp::Exp = &n {
            self.set.insert(EsFeature::ExponentiationOperator);
        }
        if let BinaryOp::NullishCoalescing = &n {
            self.set.insert(EsFeature::NullishCoalescingOperator);
        }

        n.visit_children_with(self)
    }

    fn visit_call_expr(&mut self, n: &CallExpr) {
        if let Callee::Expr(e) = &n.callee {
            if let Expr::Member(m) = e.deref() {
                if let Expr::Ident(a) = &m.obj.deref() {
                    if let MemberProp::Ident(i) = &m.prop {
                        match (i.sym.deref(), a.sym.deref()) {
                            ("values" | "entries", "Object") => {
                                self.set.insert(EsFeature::ObjectValuesEntries);
                            }
                            _ => {}
                        }
                    }
                }

                if let Expr::Array(_) = &m.obj.deref() {
                    if let MemberProp::Ident(i) = &m.prop {
                        match i.sym.deref() {
                            "groupBy" => {
                                self.set.insert(EsFeature::ArrayGrouping);
                            }
                            _ => {}
                        }
                    }
                }

                if let Expr::Ident(a) = &m.obj.deref() {
                    if let MemberProp::Ident(i) = &m.prop {
                        match (i.sym.deref(), a.sym.deref()) {
                            ("groupBy", "Object" | "Map") => {
                                self.set.insert(EsFeature::ArrayGrouping);
                            }
                            _ => {}
                        }
                    }
                }

                if let Expr::Ident(a) = &m.obj.deref() {
                    if let MemberProp::Ident(i) = &m.prop {
                        if &i.sym == "getOwnPropertyDescriptors" && &a.sym == "Object" {
                            self.set.insert(EsFeature::ObjectGetOwnPropertyDescriptors);
                        }
                    }
                }

                if let Expr::Ident(a) = &m.obj.deref() {
                    if let MemberProp::Ident(i) = &m.prop {
                        if &i.sym == "hasOwn" && &a.sym == "Object" {
                            self.set
                                .insert(EsFeature::AccessibleObjectPrototypeHasOwnProperty);
                        }
                    }
                }

                if let Expr::Ident(_) = &m.obj.deref() {
                    if let MemberProp::Ident(i) = &m.prop {
                        if &i.sym == "fromEntries" {
                            self.set.insert(EsFeature::ObjectFromEntries);
                        }
                    }
                }

                if let Expr::Ident(a) = &m.obj.deref() {
                    if let MemberProp::Ident(i) = &m.prop {
                        if &a.sym == "Promise" {
                            if &i.sym == "allSettled" {
                                self.set.insert(EsFeature::PromiseAllSettled);
                            }
                            if &i.sym == "any" {
                                self.set.insert(EsFeature::PromiseAny);
                            }
                            if &i.sym == "withResolvers" {
                                self.set.insert(EsFeature::PromiseWithResolvers);
                            }
                        }
                    }
                }

                if let Expr::Ident(a) = &m.obj.deref() {
                    if let MemberProp::Ident(i) = &m.prop {
                        if &a.sym == "Atomics" {
                            if &i.sym == "waitAsync" {
                                self.set.insert(EsFeature::AtomicsWaitAsync);
                            }
                        }
                    }
                }
            }
            if let Expr::Ident(a) = e.deref() {
                if &a.sym == "BigInt" {
                    if let Some(args) = n.args.first() {
                        if let Expr::Lit(Lit::Str(_)) = &args.expr.deref() {
                            self.set.insert(EsFeature::BigInt);
                        } else if let Expr::Lit(Lit::Num(_)) = &args.expr.deref() {
                            self.set.insert(EsFeature::BigInt);
                        }
                    }
                }
            }
        }
        n.visit_children_with(self)
    }

    fn visit_class_member(&mut self, n: &ClassMember) {
        match n {
            ClassMember::PrivateMethod(_)
            | ClassMember::PrivateProp(_)
            | ClassMember::ClassProp(_) => {
                self.set.insert(EsFeature::ClassFields);
            }
            ClassMember::Method(method) if method.is_static => {
                self.set.insert(EsFeature::ClassFields);
            }
            _ => {}
        }
        n.visit_children_with(self)
    }

    fn visit_expr(&mut self, n: &Expr) {
        if let Expr::OptChain(_) = n {
            self.set.insert(EsFeature::OptionalChaining);
        }

        n.visit_children_with(self)
    }

    fn visit_fn_decl(&mut self, n: &FnDecl) {
        if n.function.is_async {
            self.set.insert(EsFeature::AsyncFunctions);
        }
        n.visit_children_with(self)
    }

    fn visit_for_in_stmt(&mut self, n: &ForInStmt) {
        self.set.insert(EsFeature::ForInMechanics);
        n.visit_children_with(self)
    }

    fn visit_function(&mut self, n: &Function) {
        let prev_in_function = self.in_function;
        self.in_function = true;
        n.visit_children_with(self);
        self.in_function = prev_in_function;
    }

    fn visit_lit(&mut self, n: &Lit) {
        if let Lit::BigInt(_) = n {
            self.set.insert(EsFeature::BigInt);
        }
        if let Lit::Num(i) = n {
            if let Some(v) = &i.raw {
                if v.contains("_") {
                    self.set.insert(EsFeature::NumericSeparators);
                }
            }
        }
        n.visit_children_with(self)
    }

    fn visit_member_expr(&mut self, n: &MemberExpr) {
        if let Expr::Ident(m) = n.obj.deref() {
            if &m.sym == "globalThis" {
                self.set.insert(EsFeature::GlobalThis);
            }
        }
        n.visit_children_with(self)
    }

    fn visit_new_expr(&mut self, n: &NewExpr) {
        if let Expr::Ident(i) = n.callee.deref() {
            if &i.sym == "SharedArrayBuffer" {
                self.set.insert(EsFeature::SharedMemoryAndAtomics);
            } else if &i.sym == "RegExp" {
                if let Some(args) = &n.args {
                    if let Some(flags) = args.get(1) {
                        if let Expr::Lit(Lit::Str(str)) = flags.expr.deref() {
                            if str.value.contains("s") {
                                self.set.insert(EsFeature::SDotAllFlagForRegularExpressions);
                            }
                            if str.value.contains("d") {
                                self.set.insert(EsFeature::RegExpMatchIndices);
                            }
                            if str.value.contains("v") {
                                self.set.insert(
                                    EsFeature::RegexpVFlagWithSetNotationAndPropertiesOfStrings,
                                );
                            }
                        }
                    }

                    if let Some(flags) = args.first() {
                        if let Expr::Lit(Lit::Str(str)) = flags.expr.deref() {
                            if str.value.contains("(?<=") || str.value.contains("(?<!") {
                                self.set.insert(EsFeature::RegExpLookbehindAssertions);
                            }
                            if str.value.contains("\\p{") || str.value.contains("\\P{") {
                                if let Expr::Lit(Lit::Str(str)) = flags.expr.deref() {
                                    if str.value.contains("u") {
                                        self.set.insert(EsFeature::RegExpUnicodePropertyEscapes);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        n.visit_children_with(self)
    }

    fn visit_regex(&mut self, n: &Regex) {
        if n.flags.contains("s") {
            self.set.insert(EsFeature::SDotAllFlagForRegularExpressions);
        }
        if n.flags.contains("d") {
            self.set.insert(EsFeature::RegExpMatchIndices);
        }
        if n.flags.contains("v") {
            self.set
                .insert(EsFeature::RegexpVFlagWithSetNotationAndPropertiesOfStrings);
        }
        if n.exp.contains("(?<=") || n.exp.contains("(?<!") {
            self.set.insert(EsFeature::RegExpLookbehindAssertions);
        }
        if (n.exp.contains("\\p{") || n.exp.contains("\\P{")) && n.flags.contains("u") {
            self.set.insert(EsFeature::RegExpUnicodePropertyEscapes);
        }
        n.visit_children_with(self)
    }

    fn visit_rest_pat(&mut self, n: &RestPat) {
        self.set.insert(EsFeature::RestSpreadProperties);
        n.visit_children_with(self)
    }

    fn visit_spread_element(&mut self, n: &SpreadElement) {
        self.set.insert(EsFeature::RestSpreadProperties);
        n.visit_children_with(self)
    }

    fn visit_static_block(&mut self, n: &StaticBlock) {
        self.set.insert(EsFeature::ClassStaticBlock);
        n.visit_children_with(self)
    }

    fn visit_try_stmt(&mut self, n: &TryStmt) {
        if let Some(handler) = &n.handler {
            if handler.param.is_none() {
                self.set.insert(EsFeature::OptionalCatchBinding);
            }
        }
        n.visit_children_with(self)
    }
}
