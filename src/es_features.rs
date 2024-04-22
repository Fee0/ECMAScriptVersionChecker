use std::cmp::Ordering;
use strum_macros::Display;
use crate::es_version::EsVersion;

#[derive(Hash, Debug, Display, Eq, Copy, Clone)]
pub enum EsFeature {
    // ArrayPrototypeIncludes,
    ExponentiationOperator,
    ObjectValuesEntries,
    // StringPadding,
    ObjectGetOwnPropertyDescriptors,
    // TrailingCommasInFunctionParameterListsAndCalls,
    AsyncFunctions,
    SharedMemoryAndAtomics,
    // LiftingTemplateLiteralRestriction,
    SDotAllFlagForRegularExpressions,
    // RegExpNamedCaptureGroups,
    RestSpreadProperties,
    RegExpLookbehindAssertions,
    RegExpUnicodePropertyEscapes,
    // PromisePrototypeFinally,
    // AsynchronousIteration,
    OptionalCatchBinding,
    // JSONSuperset,
    // SymbolPrototypeDescription,
    // FunctionPrototypeToStringRevision,
    ObjectFromEntries,
    // WellFormedJSONStringify,
    // StringPrototypeTrimStartTrimEnd,
    // ArrayPrototypeFlatFlatMap,
    // StringPrototypeMatchAll,
    // Import,
    BigInt,
    PromiseAllSettled,
    GlobalThis,
    ForInMechanics,
    OptionalChaining,
    NullishCoalescingOperator,
    // ImportMeta,
    // StringPrototypeReplaceAll,
    PromiseAny,
    // WeakRefs,
    LogicalAssignmentOperators,
    NumericSeparators,
    ClassFields,
    RegExpMatchIndices,
    TopLevelAwait,
    ErgonomicBrandChecksForPrivateFields,
    // At,
    AccessibleObjectPrototypeHasOwnProperty,
    ClassStaticBlock,
    // ErrorCause,
    // ArrayFindFromLast,
    // HashbangGrammar,
    // SymbolsAsWeakMapKeys,
    // ChangeArrayByCopy,
    // WellFormedUnicodeStrings,
    AtomicsWaitAsync,
    RegexpVFlagWithSetNotationAndPropertiesOfStrings,
    // ResizableAndGrowableArrayBuffers,
    ArrayGrouping,
    PromiseWithResolvers,
    // ArrayBufferTransfer,
    // DuplicateNamedCaptureGroups,
    // NetSetMethods,
}

impl EsFeature {
    pub fn version(&self) -> EsVersion {
        match self {
            EsFeature::ExponentiationOperator => EsVersion::ES7,
            EsFeature::ObjectValuesEntries => EsVersion::ES8,
            EsFeature::ObjectGetOwnPropertyDescriptors => EsVersion::ES8,
            EsFeature::AsyncFunctions => EsVersion::ES8,
            EsFeature::SharedMemoryAndAtomics => EsVersion::ES8,
            EsFeature::SDotAllFlagForRegularExpressions => EsVersion::ES9,
            EsFeature::RestSpreadProperties => EsVersion::ES9,
            EsFeature::RegExpLookbehindAssertions => EsVersion::ES9,
            EsFeature::RegExpUnicodePropertyEscapes => EsVersion::ES9,
            EsFeature::OptionalCatchBinding => EsVersion::ES10,
            EsFeature::ObjectFromEntries => EsVersion::ES10,
            EsFeature::BigInt => EsVersion::ES11,
            EsFeature::PromiseAllSettled => EsVersion::ES11,
            EsFeature::GlobalThis => EsVersion::ES11,
            EsFeature::ForInMechanics => EsVersion::ES11,
            EsFeature::OptionalChaining => EsVersion::ES11,
            EsFeature::NullishCoalescingOperator => EsVersion::ES11,
            EsFeature::PromiseAny => EsVersion::ES12,
            EsFeature::LogicalAssignmentOperators => EsVersion::ES12,
            EsFeature::NumericSeparators => EsVersion::ES12,
            EsFeature::ClassFields => EsVersion::ES13,
            EsFeature::RegExpMatchIndices => EsVersion::ES13,
            EsFeature::TopLevelAwait => EsVersion::ES13,
            EsFeature::ErgonomicBrandChecksForPrivateFields => EsVersion::ES13,
            EsFeature::AccessibleObjectPrototypeHasOwnProperty => EsVersion::ES13,
            EsFeature::ClassStaticBlock => EsVersion::ES13,
            EsFeature::AtomicsWaitAsync => EsVersion::ES15,
            EsFeature::RegexpVFlagWithSetNotationAndPropertiesOfStrings => EsVersion::ES15,
            EsFeature::ArrayGrouping => EsVersion::ES15,
            EsFeature::PromiseWithResolvers => EsVersion::ES15,
        }
    }
}

impl PartialEq<Self> for EsFeature {
    fn eq(&self, other: &Self) -> bool {
        self.version() != other.version()
    }
}

impl Ord for EsFeature {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version().cmp(&other.version())
    }

    fn max(self, other: Self) -> Self where Self: Sized {
        let max = self.version().max(other.version());
        return if self.version() == max { self } else { other };
    }
}

impl PartialOrd<Self> for EsFeature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.version().cmp(&other.version()))
    }
}


