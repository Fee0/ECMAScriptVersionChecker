use std::collections::HashSet;
use std::fmt::Debug;

use swc_common::{BytePos, FileName, SourceFile};
use swc_common::input::StringInput;
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{EsConfig, Parser, Syntax};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_visit::VisitWith;
use thiserror::Error;

use crate::es_features::EsFeature;
use crate::visitor::FeatureFinder;

mod es_features;
mod es_version;
mod visitor;

#[derive(Error, Clone, Debug)]
pub enum Error {
    #[error("Parse error")]
    ParserError,
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

    let program = Parser::new_from(lexer)
        .parse_program()
        .map_err(|_| Error::ParserError)?;

    let mut visitor = FeatureFinder::default();
    program.visit_children_with(&mut visitor);

    Ok(visitor.get_result())
}

/// Analyses the given Javascript and returns a set of recognized language features
pub fn get_ecma_features(js: impl AsRef<str>) -> Result<HashSet<EsFeature>> {
    analyse(js)
}

/// Analyses the given Javascript and returns the minimum ECMAScript version required
pub fn get_min_ecma_version(js: impl AsRef<str>) -> Result<es_version::EsVersion> {
    let r = analyse(js)?;
    let max = r.iter().max();
    Ok(max.ok_or(Error::ParserError)?.version())
}
