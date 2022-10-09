#[test]

fn it_expands_as_expected() {
    macrotest::expand("tests/expands/*.rs");
}

#[test]
fn it_compiles_as_expected() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compiles/*.rs");
}
