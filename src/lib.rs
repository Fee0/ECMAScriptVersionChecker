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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn assert_features(js: &str, expected_features: &[EsFeature]) {
        let features_found = get_ecma_features(js).expect("Failed to parse JavaScript");
        let expected: HashSet<EsFeature> = expected_features.iter().cloned().collect();
        println!("Found: {:?}", features_found);
        println!("Expected: {:?}", expected);
        expected
            .iter()
            .for_each(|expected_feature| assert!(features_found.contains(expected_feature)));
    }

    #[test]
    fn test_exponentiation_operator() {
        assert_features("let x = 2 ** 3;", &[EsFeature::ExponentiationOperator]);
    }

    #[test]
    fn test_object_values_entries() {
        assert_features(
            "Object.values(obj); Object.entries(obj);",
            &[EsFeature::ObjectValuesEntries],
        );
    }

    #[test]
    fn test_object_get_own_property_descriptors() {
        assert_features(
            "Object.getOwnPropertyDescriptors(obj);",
            &[EsFeature::ObjectGetOwnPropertyDescriptors],
        );
    }

    #[test]
    fn test_async_functions() {
        assert_features(
            "async function foo() {}; const bar = async () => {};",
            &[EsFeature::AsyncFunctions],
        );
    }

    #[test]
    fn test_shared_memory_and_atomics() {
        assert_features(
            "const sharedBuffer = new SharedArrayBuffer(1024); Atomics.store(sharedBuffer, 0, 42);",
            &[EsFeature::SharedMemoryAndAtomics],
        );
    }

    #[test]
    fn test_dotall_flag_for_regular_expressions() {
        assert_features(
            "/foo.bar/s;",
            &[EsFeature::SDotAllFlagForRegularExpressions],
        );
    }

    #[test]
    fn test_rest_spread_properties() {
        assert_features(
            "const { a, ...rest } = obj; const obj2 = { ...obj, c: 3 };",
            &[EsFeature::RestSpreadProperties],
        );
    }

    #[test]
    fn test_regexp_lookbehind_assertions() {
        assert_features("/(?<=foo)bar/;", &[EsFeature::RegExpLookbehindAssertions]);
    }

    #[test]
    fn test_regexp_unicode_property_escapes() {
        assert_features(
            "/\\p{Script=Greek}/u;",
            &[EsFeature::RegExpUnicodePropertyEscapes],
        );
    }

    #[test]
    fn test_optional_catch_binding() {
        assert_features(
            "try { throw 'error'; } catch { console.error('Caught'); }",
            &[EsFeature::OptionalCatchBinding],
        );
    }

    #[test]
    fn test_object_from_entries() {
        assert_features(
            "const obj = Object.fromEntries(entries);",
            &[EsFeature::ObjectFromEntries],
        );
    }

    #[test]
    fn test_bigint() {
        assert_features("const bigInt = 123n;", &[EsFeature::BigInt]);
    }

    #[test]
    fn test_promise_all_settled() {
        assert_features(
            "Promise.allSettled(promises);",
            &[EsFeature::PromiseAllSettled],
        );
    }

    #[test]
    fn test_global_this() {
        assert_features("globalThis.foo = 'bar';", &[EsFeature::GlobalThis]);
    }

    #[test]
    fn test_for_in_mechanics() {
        assert_features(
            "for (const key in obj) { console.log(key); }",
            &[EsFeature::ForInMechanics],
        );
    }

    #[test]
    fn test_optional_chaining() {
        assert_features("const value = obj?.prop;", &[EsFeature::OptionalChaining]);
    }

    #[test]
    fn test_nullish_coalescing_operator() {
        assert_features(
            "const value = foo ?? 'default';",
            &[EsFeature::NullishCoalescingOperator],
        );
    }

    #[test]
    fn test_promise_any() {
        assert_features("Promise.any(promises);", &[EsFeature::PromiseAny]);
    }

    #[test]
    fn test_logical_assignment_operators() {
        assert_features(
            "a ||= b; c &&= d; e ??= f;",
            &[EsFeature::LogicalAssignmentOperators],
        );
    }

    #[test]
    fn test_numeric_separators() {
        assert_features("const num = 1_000_000;", &[EsFeature::NumericSeparators]);
    }

    #[test]
    fn test_class_fields() {
        assert_features(
            "class MyClass { #privateField; static staticField = 42; }",
            &[EsFeature::ClassFields],
        );
    }

    #[test]
    fn test_regexp_match_indices() {
        assert_features("/foo/d;", &[EsFeature::RegExpMatchIndices]);
    }

    #[test]
    fn test_top_level_await() {
        assert_features("await fetchData();", &[EsFeature::TopLevelAwait]);
    }

    #[test]
    fn test_ergonomic_brand_checks_for_private_fields() {
        assert_features(
            "class MyClass { #privateField; isPrivate() { return #privateField in this; } }",
            &[EsFeature::ErgonomicBrandChecksForPrivateFields],
        );
    }

    #[test]
    fn test_accessible_object_prototype_has_own_property() {
        assert_features(
            "Object.hasOwn(obj, 'prop');",
            &[EsFeature::AccessibleObjectPrototypeHasOwnProperty],
        );
    }

    #[test]
    fn test_class_static_block() {
        assert_features(
            "class MyClass { static { console.log('Static block!'); } }",
            &[EsFeature::ClassStaticBlock],
        );
    }

    #[test]
    fn test_atomics_wait_async() {
        assert_features(
            "Atomics.waitAsync(sharedArrayBuffer, 0, 0);",
            &[EsFeature::AtomicsWaitAsync],
        );
    }

    #[test]
    fn test_regexp_v_flag_with_set_notation_and_properties_of_strings() {
        assert_features(
            "/[\\p{L}&&[^a]]/v;",
            &[EsFeature::RegexpVFlagWithSetNotationAndPropertiesOfStrings],
        );
    }

    #[test]
    fn test_array_grouping() {
        assert_features(
            "const grouped = [1, 2, 3].groupBy(x => x % 2);",
            &[EsFeature::ArrayGrouping],
        );
    }

    #[test]
    fn test_promise_with_resolvers() {
        assert_features(
            "const { promise, resolve, reject } = Promise.withResolvers();",
            &[EsFeature::PromiseWithResolvers],
        );
    }
}
