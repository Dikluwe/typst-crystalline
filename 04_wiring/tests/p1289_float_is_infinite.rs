//! @prompt 00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md
//! @prompt-hash 4d725b3f

use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_typst");

fn eval(expression: &str) -> Output {
    Command::new(BIN)
        .args(["eval", expression, "--format", "json"])
        .output()
        .expect("run crystalline eval")
}

#[test]
fn p1289_float_is_infinite_reports_finite_infinite_and_nan() {
    let output = eval(
        "(float.is-infinite(0.0), float.is-infinite(42.5), \
         float.is-infinite(float(\"1e999\")), float.is-infinite(-float(\"1e999\")), \
         float.is-infinite(float(\"NaN\")))",
    );
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "[false,false,true,true,false]"
    );
}

#[test]
fn p1289_float_is_infinite_preserves_public_signature_and_errors() {
    let identity = eval("repr((type(float.is-infinite), repr(float.is-infinite)))");
    assert!(identity.status.success());
    assert_eq!(
        String::from_utf8_lossy(&identity.stdout).trim(),
        "\"(function, \\\"is-infinite\\\")\""
    );

    for (expression, expected) in [
        ("float.is-infinite()", "missing argument"),
        ("float.is-infinite(1.0, 2.0)", "unexpected argument"),
        ("float.is-infinite(1.0, foo: 2)", "unexpected argument: foo"),
        ("float.is-infinite(\"x\")", "expected float"),
    ] {
        let output = eval(expression);
        assert!(!output.status.success(), "{expression} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected),
            "{expression}: expected {expected:?}, got {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
