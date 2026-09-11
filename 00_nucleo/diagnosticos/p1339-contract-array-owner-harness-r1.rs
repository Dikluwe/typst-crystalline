// Independent prospective owner oracle, not an Array implementation.
// Invoke unchanged with the real narrow control owner, then real final owner.
// The caller supplies a legitimate FileId and ABI closure; no fixture ID routing.
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::bytes::Bytes;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{Severity, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

pub fn assert_array_owner(
    file: FileId,
    mut owner: impl FnMut(&Args) -> SourceResult<Value>,
) {
    let aggregate = Span::from_range(file, 100..140);
    let input_span = Span::from_range(file, 101..110);
    let positional_span = Span::from_range(file, 120..125);
    let named_span = Span::from_range(file, 118..130);
    let detached = Span::detached();
    let bytes = || Value::Bytes(Bytes::new(vec![0, 255]));

    // Positive reachability and exact output variants, independent of errors.
    for octets in [vec![], vec![0, 255]] {
        let args = Args::from_parts(
            vec![Value::Bytes(Bytes::new(octets.clone()))],
            Default::default(),
            aggregate,
        );
        let result = owner(&args).expect("ARR-U00 real owner must accept Bytes");
        let Value::Array(values) = result else {
            panic!("ARR-U00 expected actual Array variant");
        };
        assert_eq!(values.len(), octets.len(), "ARR-U00 exact length");
        for (actual, expected) in values.iter().zip(octets.iter()) {
            assert!(matches!(actual, Value::Int(n) if *n == i64::from(*expected)),
                "ARR-U00 expected exact unsigned Int octet");
        }
        let Value::Bytes(original) = &args.items[0] else {
            panic!("ARR-U00 original input type changed");
        };
        assert_eq!(original.as_slice(), octets.as_slice(), "ARR-U00 immutable input");
    }

    // from_parts is a legitimate synthetic carrier: individual origins detached.
    let positional_synthetic = Args::from_parts(
        vec![bytes(), Value::Int(9)], Default::default(), aggregate,
    );
    assert_error(&mut owner, &positional_synthetic, "unexpected argument", aggregate,
        "ARR-U01-detached-positional");

    // Known arg span wins even when value span is detached (the mapped case).
    let positional_known = Args::from_occurrences(aggregate, vec![
        ArgOccurrence { name: None, value: bytes(), span: input_span, value_span: input_span },
        ArgOccurrence { name: None, value: Value::Int(9), span: positional_span, value_span: detached },
    ]);
    assert_error(&mut owner, &positional_known, "unexpected argument", positional_span,
        "ARR-U02-known-positional");

    let mut named_synthetic = Args::from_parts(vec![bytes()], Default::default(), aggregate);
    named_synthetic.named.insert("other".into(), Value::Int(9));
    assert_error(&mut owner, &named_synthetic, "unexpected argument: other", aggregate,
        "ARR-U03-detached-named");

    let named_known = Args::from_occurrences(aggregate, vec![
        ArgOccurrence { name: None, value: bytes(), span: input_span, value_span: input_span },
        ArgOccurrence { name: Some("other".into()), value: Value::Int(9), span: named_span, value_span: detached },
    ]);
    assert_error(&mut owner, &named_known, "unexpected argument: other", named_span,
        "ARR-U04-known-named");
}

fn assert_error(
    owner: &mut impl FnMut(&Args) -> SourceResult<Value>,
    args: &Args,
    message: &str,
    span: Span,
    id: &str,
) {
    let diagnostics = owner(args).expect_err(id);
    assert_eq!(diagnostics.len(), 1, "{id}: exactly one diagnostic");
    let diagnostic = &diagnostics[0];
    assert!(matches!(diagnostic.severity, Severity::Error), "{id}: error severity");
    assert_eq!(diagnostic.message, message, "{id}: exact message");
    assert_eq!(diagnostic.span, span, "{id}: exact occurrence or aggregate span");
    assert!(diagnostic.hints.is_empty(), "{id}: exact empty hints");
    assert!(diagnostic.trace.is_empty(), "{id}: direct owner has no eval trace");
}
