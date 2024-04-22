#![allow(unused_imports)]
#![allow(unused_variables)]

mod visitor;
mod es_features;
mod es_version;

use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use swc_common::{BytePos, FileName, SourceFile};
use swc_common::input::StringInput;
use swc_ecma_ast::{ArrowExpr, AssignOp, AwaitExpr, BinaryOp, BinExpr, CallExpr, ClassMember, EsVersion, Expr, FnDecl, ForInStmt, Function, Lit, MemberExpr, NewExpr, Program, Regex, RestPat, SpreadElement, StaticBlock, TryStmt};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{EsConfig, Parser, PResult, Syntax};
use swc_ecma_visit::{Visit, VisitWith};

use crate::visitor::FeatureFinder;
use thiserror::Error;
use crate::es_features::EsFeature;

#[derive(Error, Clone, Debug)]
pub enum Error {
    #[error("Parse error")]
    ParserError,
    // ParserError(#[from] swc_ecma_parser::error::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

fn analyse(js: impl AsRef<str>) -> Result<HashSet<EsFeature>> {
    let f = SourceFile::new(
        FileName::Anon,
        false,
        FileName::Anon,
        String::from(js.as_ref()),
        BytePos(1),
    );

    let lexer = Lexer::new(
        Syntax::Es(EsConfig {
            jsx: false,
            ..Default::default()
        }),
        EsVersion::Es2022,
        StringInput::from(&f),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let program = parser.parse_program().map_err(|_| Error::ParserError)?;

    // println!("{:#?}", program);

    let mut visitor = FeatureFinder::default();
    program.visit_children_with(&mut visitor);

    let r = visitor.get_result();
    Ok(r)
}

pub fn get_ecma_features(js: impl AsRef<str>) -> Result<HashSet<EsFeature>> {
    analyse(js)
}

pub fn get_min_ecma_version(js: impl AsRef<str>) -> Result<es_version::EsVersion> {
    let r = analyse(js)?;
    let max = r.iter().max();

    // println!("{:?}", r);
    // println!("{:?}", max);

    Ok(max.unwrap().version())
}
