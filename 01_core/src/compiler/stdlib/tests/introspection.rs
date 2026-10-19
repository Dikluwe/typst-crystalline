//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/introspection.md
//! @prompt-hash b2360b9b
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn stdlib_query_em_introspector_vazio_retorna_array_vazio() {
    // P175 → P179 upgrade: query retorna Value::Array, não Int.
    null_ctx!(ctx);
    let r = native_query(
        &mut ctx,
        &p(vec![Value::Str("heading".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Array(vec![]));
}

#[test]
fn stdlib_query_em_introspector_populado_retorna_array_de_locations() {
    // P179 upgrade: array com Value::Location entries.
    null_ctx!(ctx);
    use crate::entities::element_kind::ElementKind;
    use crate::entities::location::Location;
    ctx.introspector
        .kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .extend(vec![
            Location::from_raw(1),
            Location::from_raw(2),
            Location::from_raw(3),
        ]);

    let r = native_query(
        &mut ctx,
        &p(vec![Value::Str("heading".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let expected = Value::Array(vec![
        Value::Location(Location::from_raw(1)),
        Value::Location(Location::from_raw(2)),
        Value::Location(Location::from_raw(3)),
    ]);
    assert_eq!(r, expected);
}

#[test]
fn stdlib_query_kind_invalido_retorna_err() {
    null_ctx!(ctx);
    let r = native_query(
        &mut ctx,
        &p(vec![Value::Str("nao_existe".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "kind inválido deve retornar Err");
}

#[test]
fn stdlib_query_arg_nao_string_retorna_err() {
    null_ctx!(ctx);
    let r =
        native_query(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id());
    assert!(r.is_err(), "arg não-string deve retornar Err");
}

#[test]
fn stdlib_query_outline_funciona_pos_p178_p179() {
    // P178: query("outline") agora aceita kind.
    // P179: retorna array, não count.
    null_ctx!(ctx);
    use crate::entities::element_kind::ElementKind;
    use crate::entities::location::Location;
    // Vazio.
    let r = native_query(
        &mut ctx,
        &p(vec![Value::Str("outline".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Array(vec![]));
    // Populado.
    ctx.introspector
        .kind_index
        .entry(ElementKind::Outline)
        .or_default()
        .push(Location::from_raw(10));
    let r = native_query(
        &mut ctx,
        &p(vec![Value::Str("outline".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Array(vec![Value::Location(Location::from_raw(10))]));
}

#[test]
fn stdlib_query_p179_upgrade_value_location_type_name() {
    // P179: Value::Location existe e tem type_name correcto.
    use crate::entities::location::Location;
    let v = Value::Location(Location::from_raw(42));
    assert_eq!(v.type_name(), "location");
}

#[test]
fn p1151_query_preserva_locations_distintas_sem_mudar_morfologia() {
    use crate::compiler::eval::operators::eval_binary_op;
    use crate::entities::ast::expr::BinOp;
    use crate::entities::content::Content;
    use crate::entities::element_kind::ElementKind;
    use crate::entities::location::Location;

    null_ctx!(ctx);
    let a = Location::from_raw(41);
    let b = Location::from_raw(42);
    let content = Content::strong(Content::text("Same"));
    ctx.introspector
        .kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .extend([a, b]);
    ctx.introspector.elements.insert(
        a,
        crate::entities::value::IntrospectedContent::new(content.clone(), None),
    );
    ctx.introspector.elements.insert(
        b,
        crate::entities::value::IntrospectedContent::new(content.clone(), None),
    );

    let Value::Array(values) = native_query(
        &mut ctx,
        &p(vec![Value::Str("heading".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap() else {
        panic!("query deve devolver array")
    };

    assert!(matches!(&values[0], Value::LocatedContent(_, loc) if *loc == a));
    assert!(matches!(&values[1], Value::LocatedContent(_, loc) if *loc == b));
    assert_eq!(values[0].type_of(), crate::entities::value::Type::Content);
    assert_eq!(
        crate::compiler::eval::repr::repr_value(&values[0]),
        crate::compiler::eval::repr::repr_value(&Value::Content(content.clone()))
    );
    assert_eq!(
        eval_binary_op(BinOp::Eq, values[0].clone(), values[1].clone()),
        Ok(Value::Bool(true))
    );
    let Value::LocatedContent(first, first_loc) = &values[0] else { unreachable!() };
    assert!(matches!(
        crate::compiler::eval::bindings::eval_content_method_at(
            first.content(),
            Some(*first_loc),
            "location",
            Args::positional(vec![]),
            crate::entities::span::Span::detached(),
        ),
        Ok(Value::Location(loc)) if loc == a
    ));
    assert!(matches!(
        crate::compiler::eval::bindings::eval_content_method(
            &content,
            "location",
            Args::positional(vec![]),
            crate::entities::span::Span::detached(),
        ),
        Ok(Value::None)
    ));
}

#[test]
fn p208b_here_sem_current_location_retorna_err_contextual() {
    // Pre-population: EvalContext::new() default current_location = None.
    // here() retorna erro contextual coerente (não panic).
    null_ctx!(ctx);
    let r = native_here(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "here() sem current_location deve falhar");
    // Erro contém menção a "here()" ou similar — não validar texto exacto.
}

#[test]
fn p208b_here_com_current_location_retorna_value_location() {
    // Setter conveniente: with_current_location.
    use crate::entities::location::Location;
    let mut ctx = EvalContext::new().with_current_location(Location::from_raw(42));
    let r = native_here(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    assert_eq!(r, Value::Location(Location::from_raw(42)));
    assert_eq!(r.type_name(), "location");
}

#[test]
fn p208b_here_com_args_retorna_err() {
    // here() não aceita argumentos (paridade vanilla: sem args).
    use crate::entities::location::Location;
    let mut ctx = EvalContext::new().with_current_location(Location::from_raw(1));
    let r =
        native_here(&mut ctx, &p(vec![Value::Int(99)]), &null_world(), test_file_id());
    assert!(r.is_err(), "here(99) deve falhar — sem args");
}

#[test]
fn p208b_here_field_writable_directamente() {
    // Pattern alternativo: write directo ao field (sem setter).
    // Útil quando consumer já tem &mut EvalContext.
    use crate::entities::location::Location;
    null_ctx!(ctx);
    assert!(ctx.current_location.is_none());

    ctx.current_location = Some(Location::from_raw(7));
    let r = native_here(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    assert_eq!(r, Value::Location(Location::from_raw(7)));
}

#[test]
fn p772w_target_devolve_paged() {
    // P821 — `target()` é contextual (vanilla `#[func(contextual)]`):
    // o caso feliz exige `in_context = true` (como na expansão de
    // `#context` em L3). Fora de contexto erra — ver testes P821 em
    // `compiler/eval/tests.rs`.
    null_ctx!(ctx);
    ctx.in_context = true;
    let r = native_target(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    assert_eq!(r, Value::Str("paged".into()));
}

#[test]
fn p772w_target_com_args_retorna_err() {
    null_ctx!(ctx);
    let r =
        native_target(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id());
    assert!(r.is_err(), "target(1) deve falhar — sem args, paridade vanilla");
}

#[test]
fn p208c_locate_kind_existente_retorna_some_location() {
    // Popula introspector com 2 headings; locate("heading")
    // retorna a PRIMEIRA Location (Vec::first).
    use crate::entities::element_kind::ElementKind;
    use crate::entities::location::Location;
    null_ctx!(ctx);
    ctx.introspector
        .kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .push(Location::from_raw(10));
    ctx.introspector
        .kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .push(Location::from_raw(20));

    let r = native_locate(
        &mut ctx,
        &p(vec![Value::Str("heading".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Location(Location::from_raw(10)));
    assert_eq!(r.type_name(), "location");
}

#[test]
fn p208c_locate_kind_inexistente_retorna_none() {
    // Introspector vazio; locate("figure") retorna Value::None
    // (não Err — kind é válido, só não há matches).
    null_ctx!(ctx);
    let r = native_locate(
        &mut ctx,
        &p(vec![Value::Str("figure".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::None);
}

#[test]
fn p208c_locate_kind_invalido_retorna_err() {
    // Kind inválido → erro contextual coerente.
    null_ctx!(ctx);
    let r = native_locate(
        &mut ctx,
        &p(vec![Value::Str("inexistente".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "kind inválido deve falhar");
}

#[test]
fn p208c_locate_arg_nao_string_retorna_err_com_hint_p209() {
    // Arg não-string e não-Location → erro com hint.
    // **Actualizado em P209B**: `Value::Location` agora é
    // dispatched (não é mais erro). `Value::Int` continua a
    // ser erro — hint actual menciona Regex pendente (P209D)
    // e And/Or Rust-only.
    null_ctx!(ctx);
    let r =
        native_locate(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id());
    assert!(r.is_err(), "arg não-string/location deve falhar");
}

#[test]
fn p209b_locate_label_syntax_retorna_some_location() {
    // <nome> syntax → Selector::Label. Popula introspector
    // com label "intro" → loc(7); locate("<intro>") deve
    // retornar Value::Location(loc(7)).
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    null_ctx!(ctx);
    ctx.introspector
        .labels
        .add(Label("intro".to_string()), Location::from_raw(7));
    let r = native_locate(
        &mut ctx,
        &p(vec![Value::Str("<intro>".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Location(Location::from_raw(7)));
}

#[test]
fn p209b_locate_label_inexistente_retorna_none() {
    // <nome> que não existe → query devolve Vec vazio →
    // locate() devolve Value::None.
    null_ctx!(ctx);
    let r = native_locate(
        &mut ctx,
        &p(vec![Value::Str("<ausente>".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::None);
}

#[test]
fn p209b_query_location_arg_devolve_singleton() {
    // Value::Location → Selector::Location → singleton
    // [loc]. query() retorna Value::Array com 1 entry.
    use crate::entities::location::Location;
    null_ctx!(ctx);
    let r = native_query(
        &mut ctx,
        &p(vec![Value::Location(Location::from_raw(42))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Array(vec![Value::Location(Location::from_raw(42))]),);
}

#[test]
fn p209b_locate_location_arg_retorna_propria_location() {
    // Selector::Location é singleton trivial → locate(loc)
    // retorna Value::Location(loc).
    use crate::entities::location::Location;
    null_ctx!(ctx);
    let r = native_locate(
        &mut ctx,
        &p(vec![Value::Location(Location::from_raw(99))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Location(Location::from_raw(99)));
}

#[test]
fn p209b_query_label_via_introspector_directo() {
    // Test API directo: Introspector::query(Selector::Label)
    // delega a query_by_label.
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    use crate::entities::selector::Selector;
    null_ctx!(ctx);
    ctx.introspector
        .labels
        .add(Label("a".to_string()), Location::from_raw(1));
    let r = ctx.introspector.query(&Selector::Label(Label("a".to_string())));
    assert_eq!(r, vec![Location::from_raw(1)]);
    // Label inexistente → vazio.
    let r2 = ctx.introspector.query(&Selector::Label(Label("b".to_string())));
    assert!(r2.is_empty());
}

#[test]
fn p209c_query_and_vazio_devolve_empty() {
    // Opção A: And(vec![]) → empty Vec.
    use crate::entities::introspector::Introspector;
    use crate::entities::location::Location;
    use crate::entities::selector::Selector;
    use ecow::EcoVec;
    let ctx = EvalContext::new();
    let r = ctx.introspector.query(&Selector::And(EcoVec::new()));
    assert_eq!(r, Vec::<Location>::new());
}

#[test]
fn p209c_query_or_vazio_devolve_empty() {
    use crate::entities::introspector::Introspector;
    use crate::entities::location::Location;
    use crate::entities::selector::Selector;
    use ecow::EcoVec;
    let ctx = EvalContext::new();
    let r = ctx.introspector.query(&Selector::Or(EcoVec::new()));
    assert_eq!(r, Vec::<Location>::new());
}

#[test]
fn p209c_query_and_interseccao_de_dois() {
    // Popular: heading na loc 7 com label "a"; figure na loc 10
    // sem label. And([Kind(heading), Label(a)]) → [loc(7)].
    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    use crate::entities::selector::Selector;
    use ecow::EcoVec;
    null_ctx!(ctx);
    ctx.introspector
        .kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .push(Location::from_raw(7));
    ctx.introspector
        .kind_index
        .entry(ElementKind::Figure)
        .or_default()
        .push(Location::from_raw(10));
    ctx.introspector
        .labels
        .add(Label("a".to_string()), Location::from_raw(7));

    let r = ctx.introspector.query(&Selector::And(EcoVec::from(vec![
        Selector::Kind(ElementKind::Heading),
        Selector::Label(Label("a".to_string())),
    ])));
    // Apenas loc(7) está em ambos.
    assert_eq!(r, vec![Location::from_raw(7)]);

    // Intersecção vazia: label que não existe.
    let r_empty = ctx.introspector.query(&Selector::And(EcoVec::from(vec![
        Selector::Kind(ElementKind::Heading),
        Selector::Label(Label("zzz".to_string())),
    ])));
    assert!(r_empty.is_empty());
}

#[test]
fn p209c_query_or_uniao_dedupliquada() {
    // Popular: 2 headings (loc 1, 2), 1 figure (loc 3).
    // Or([Kind(heading), Kind(figure)]) → união ordenada
    // [1, 2, 3].
    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;
    use crate::entities::location::Location;
    use crate::entities::selector::Selector;
    use ecow::EcoVec;
    null_ctx!(ctx);
    ctx.introspector
        .kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .push(Location::from_raw(1));
    ctx.introspector
        .kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .push(Location::from_raw(2));
    ctx.introspector
        .kind_index
        .entry(ElementKind::Figure)
        .or_default()
        .push(Location::from_raw(3));

    let r = ctx.introspector.query(&Selector::Or(EcoVec::from(vec![
        Selector::Kind(ElementKind::Heading),
        Selector::Kind(ElementKind::Figure),
    ])));
    assert_eq!(
        r,
        vec![Location::from_raw(1), Location::from_raw(2), Location::from_raw(3),],
    );

    // Dedup: Or com mesma kind 2× não duplica.
    let r_dedup = ctx.introspector.query(&Selector::Or(EcoVec::from(vec![
        Selector::Kind(ElementKind::Heading),
        Selector::Kind(ElementKind::Heading),
    ])));
    assert_eq!(r_dedup, vec![Location::from_raw(1), Location::from_raw(2)],);
}

#[test]
fn p209d_introspector_query_regex_devolve_empty_stub() {
    // Stub documentado: Regex variant retorna vec![] em query.
    use crate::entities::introspector::Introspector;
    use crate::entities::location::Location;
    use crate::entities::regex::Regex;
    use crate::entities::selector::Selector;
    let ctx = EvalContext::new();
    let r = ctx.introspector.query(&Selector::Regex(Regex::new("\\d+").unwrap()));
    assert_eq!(r, Vec::<Location>::new());
}

#[test]
fn p209d_introspector_query_regex_in_or_compoe_com_kind() {
    // Composição estrutural funciona: Or([Regex, Kind]) →
    // união. Regex retorna empty (stub); Kind devolve seus
    // matches. Result = matches do Kind apenas.
    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;
    use crate::entities::location::Location;
    use crate::entities::regex::Regex;
    use crate::entities::selector::Selector;
    use ecow::EcoVec;
    null_ctx!(ctx);
    ctx.introspector
        .kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .push(Location::from_raw(1));

    let r = ctx.introspector.query(&Selector::Or(EcoVec::from(vec![
        Selector::Regex(Regex::new("abc").unwrap()),
        Selector::Kind(ElementKind::Heading),
    ])));
    // Regex stub: empty. Kind: [loc(1)]. Union dedup: [loc(1)].
    assert_eq!(r, vec![Location::from_raw(1)]);
}

#[test]
fn p209c_query_nested_or_dentro_de_and() {
    // And([Or([Kind(heading), Kind(figure)]), Label("a")]).
    // Popular: heading loc 7 com label "a"; figure loc 10 sem
    // label. Resultado: [loc(7)] (a ∈ {heading, figure} E label a).
    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    use crate::entities::selector::Selector;
    use ecow::EcoVec;
    null_ctx!(ctx);
    ctx.introspector
        .kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .push(Location::from_raw(7));
    ctx.introspector
        .kind_index
        .entry(ElementKind::Figure)
        .or_default()
        .push(Location::from_raw(10));
    ctx.introspector
        .labels
        .add(Label("a".to_string()), Location::from_raw(7));

    let inner_or = Selector::Or(EcoVec::from(vec![
        Selector::Kind(ElementKind::Heading),
        Selector::Kind(ElementKind::Figure),
    ]));
    let nested = Selector::And(EcoVec::from(vec![
        inner_or,
        Selector::Label(Label("a".to_string())),
    ]));
    let r = ctx.introspector.query(&nested);
    assert_eq!(r, vec![Location::from_raw(7)]);
}

#[test]
fn p393_regex_construtor_valido_devolve_value_regex() {
    null_ctx!(ctx);
    let r = native_regex(
        &mut ctx,
        &p(vec![Value::Str("\\d+".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert!(
        matches!(r, Value::Regex(_)),
        "regex(\"\\d+\") deve devolver Value::Regex: {:?}",
        r
    );
}

#[test]
fn p393_regex_construtor_invalido_erro() {
    null_ctx!(ctx);
    let r = native_regex(
        &mut ctx,
        &p(vec![Value::Str("[".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "regex(\"[\") deve falhar");
}

#[test]
fn p393_regex_construtor_tipo_errado_erro() {
    null_ctx!(ctx);
    let r =
        native_regex(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id());
    assert!(r.is_err(), "regex(42) deve falhar");
}

#[test]
fn p393_regex_construtor_arg_nomeado_rejeitado() {
    null_ctx!(ctx);
    let r = native_regex(
        &mut ctx,
        &pn(vec![Value::Str("\\d+".into())], "foo", Value::Int(1)),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "regex com arg nomeado deve falhar");
}
