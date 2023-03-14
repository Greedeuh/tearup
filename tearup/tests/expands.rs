#[test]
#[cfg(not(feature = "async"))]
fn it_expands_as_expected_without_default_features() {
    macrotest::expand("tests/expands/waiting/sync.rs");
    macrotest::expand("tests/expands/waiting/async-no-feature-fail.rs");
}

#[test]
#[cfg(feature = "async")]
fn it_expands_as_expected() {
    macrotest::expand("tests/expands/waiting/sync.rs");
    macrotest::expand("tests/expands/waiting/async.rs");
}
