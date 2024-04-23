Detects the minimum required ECMAScript Version needed for a given Javascript file.

Due to the dynamic nature of Javascript, not all language features can be reliably detected.
So, the result is an approximation.

Can detect the following [stage 4 proposals](https://github.com/tc39/proposals/blob/HEAD/finished-proposals.md):

| Proposal                                         | Detected? |
|--------------------------------------------------|-----------|
| ArrayPrototypeIncludes                           |           |
| ExponentiationOperator                           | ✅         |
| ObjectValuesEntries                              | ✅         |
| StringPadding                                    |           |
| ObjectGetOwnPropertyDescriptors                  | ✅         |
| TrailingCommasInFunctionParameterListsAndCalls   |           |
| AsyncFunctions                                   | ✅         |
| SharedMemoryAndAtomics                           | ✅         |
| LiftingTemplateLiteralRestriction                |           |
| SDotAllFlagForRegularExpressions                 | ✅         |
| RegExpNamedCaptureGroups                         |           |
| RestSpreadProperties                             | ✅         |
| RegExpLookbehindAssertions                       | ✅         |
| RegExpUnicodePropertyEscapes                     | ✅         |
| PromisePrototypeFinally                          |           |
| AsynchronousIteration                            |           |
| OptionalCatchBinding                             | ✅         |
| JSONSuperset                                     |           |
| SymbolPrototypeDescription                       |           |
| FunctionPrototypeToStringRevision                |           |
| ObjectFromEntries                                | ✅         |
| WellFormedJSONStringify                          |           |
| StringPrototypeTrimStartTrimEnd                  |           |
| ArrayPrototypeFlatFlatMap                        |           |
| StringPrototypeMatchAll                          |           |
| Import                                           |           |
| BigInt                                           | ✅         |
| PromiseAllSettled                                | ✅         |
| GlobalThis                                       | ✅         |
| ForInMechanics                                   | ✅         |
| OptionalChaining                                 | ✅         |
| NullishCoalescingOperator                        | ✅         |
| ImportMeta                                       |           |
| StringPrototypeReplaceAll                        |           |
| PromiseAny                                       | ✅         |
| WeakRefs                                         |           |
| LogicalAssignmentOperators                       | ✅         |
| NumericSeparators                                | ✅         |
| ClassFields                                      | ✅         |
| RegExpMatchIndices                               | ✅         |
| TopLevelAwait                                    | ✅         |
| ErgonomicBrandChecksForPrivateFields             | ✅         |
| At                                               |           |
| AccessibleObjectPrototypeHasOwnProperty          | ✅         |
| ClassStaticBlock                                 | ✅         |
| ErrorCause                                       |           |
| ArrayFindFromLast                                |           |
| HashbangGrammar                                  |           |
| SymbolsAsWeakMapKeys                             |           |
| ChangeArrayByCopy                                |           |
| WellFormedUnicodeStrings                         |           |
| AtomicsWaitAsync                                 | ✅         |
| RegexpVFlagWithSetNotationAndPropertiesOfStrings | ✅         |
| ResizableAndGrowableArrayBuffers                 |           |
| ArrayGrouping                                    | ✅         |
| PromiseWithResolvers                             | ✅         |
| ArrayBufferTransfer                              |           |
| DuplicateNamedCaptureGroups                      |           |
| NetSetMethods                                    |           |