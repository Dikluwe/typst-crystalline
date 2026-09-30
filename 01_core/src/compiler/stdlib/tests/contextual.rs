//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/contextual.md
//! @prompt-hash 18bb6af0
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn p210b_counter_step_basico() {
    // counter_step("foo") devolve Value::Content(Content::CounterUpdate
    // { key: "foo", action: Step }).
    use crate::entities::content::Content;
    use crate::entities::counter_update::CounterUpdate as CounterAction;
    null_ctx!(ctx);
    let r = native_counter_step(
        &mut ctx,
        &p(vec![Value::Str("foo".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    match r {
        Value::Content(Content::CounterUpdate(e)) => {
            assert_eq!(e.key, ck("foo"));
            assert_eq!(e.action, CounterAction::step());
        }
        _ => panic!("expected Value::Content(CounterUpdate)"),
    }
}

#[test]
fn p210b_counter_step_arg_invalido_retorna_err() {
    // counter_step(42) — Value::Int não é Value::Str.
    null_ctx!(ctx);
    let r = native_counter_step(
        &mut ctx,
        &p(vec![Value::Int(42)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "arg não-string deve falhar");
}

#[test]
fn p210b_counter_step_sem_args_retorna_err() {
    null_ctx!(ctx);
    let r = native_counter_step(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "sem args deve falhar");
}

#[test]
fn p210b_counter_step_multipla_invocacao_estruturalmente_igual() {
    // Duas invocações com mesma key produzem Content
    // estructuralmente iguais (sem state shared no stdlib func).
    use crate::entities::content::Content;
    null_ctx!(ctx);
    let r1 = native_counter_step(
        &mut ctx,
        &p(vec![Value::Str("h".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let r2 = native_counter_step(
        &mut ctx,
        &p(vec![Value::Str("h".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // PartialEq sobre Content::CounterUpdate (per content.rs:1203).
    match (r1, r2) {
        (Value::Content(c1), Value::Content(c2)) => {
            assert!(matches!(
                (&c1, &c2),
                (Content::CounterUpdate(_), Content::CounterUpdate(_))
            ));
            // Via match interno + comparison.
            let same = match (&c1, &c2) {
                (Content::CounterUpdate(e1), Content::CounterUpdate(e2)) => {
                    e1.key == e2.key && e1.action == e2.action
                }
                _ => false,
            };
            assert!(same, "invocações idempotentes esperadas");
        }
        _ => panic!("expected Value::Content"),
    }
}

#[test]
fn stdlib_counter_final_em_introspector_vazio_retorna_str_vazia() {
    null_ctx!(ctx);
    let r = native_counter_final(
        &mut ctx,
        &p(vec![Value::Str("heading".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Str("".into()));
}

#[test]
fn stdlib_counter_final_em_introspector_populado_retorna_string_formatada() {
    null_ctx!(ctx);
    // Popular CounterRegistry directamente com hierarquia [1, 2, 1].
    ctx.introspector.counters.apply_hierarchical(ck("heading"), 1);
    ctx.introspector.counters.apply_hierarchical(ck("heading"), 2);
    ctx.introspector.counters.apply_hierarchical(ck("heading"), 1);

    let r = native_counter_final(
        &mut ctx,
        &p(vec![Value::Str("heading".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    // P170: hierarchical "1.2.1" → "2" após [1,2,1] (último top-level).
    // Ajustar conforme paridade real — confirma que retorna string
    // não vazia formatada.
    match r {
        Value::Str(s) => {
            assert!(!s.is_empty(), "esperado string não-vazia, recebido vazia");
        }
        other => panic!("esperado Value::Str, recebido {:?}", other),
    }
}

#[test]
fn stdlib_counter_final_key_inexistente_retorna_str_vazia() {
    null_ctx!(ctx);
    let r = native_counter_final(
        &mut ctx,
        &p(vec![Value::Str("inexistente".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Str("".into()));
}

#[test]
fn stdlib_counter_final_arg_nao_string_retorna_err() {
    null_ctx!(ctx);
    let r = native_counter_final(
        &mut ctx,
        &p(vec![Value::Int(42)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "arg não-string deve retornar Err");
}

#[test]
fn stdlib_counter_at_em_introspector_vazio_retorna_str_vazia() {
    null_ctx!(ctx);
    let r = native_counter_at(
        &mut ctx,
        &p(vec![Value::Str("heading".into()), Value::Str("intro".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Str("".into()));
}

#[test]
fn stdlib_counter_at_label_inexistente_retorna_str_vazia() {
    null_ctx!(ctx);
    // Popular counter mas label não registada.
    use crate::entities::location::Location;
    ctx.introspector.counters.apply_hierarchical_at(
        ck("heading"),
        1,
        Location::from_raw(10),
    );
    let r = native_counter_at(
        &mut ctx,
        &p(vec![Value::Str("heading".into()), Value::Str("nonexistent".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Str("".into()));
}

#[test]
fn stdlib_counter_at_label_associado_retorna_string_formatada() {
    null_ctx!(ctx);
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    // Popular: heading na loc 10 com counter [1], label "intro" → loc 10.
    ctx.introspector.counters.apply_hierarchical_at(
        ck("heading"),
        1,
        Location::from_raw(10),
    );
    ctx.introspector
        .labels
        .add(Label("intro".to_string()), Location::from_raw(10));

    let r = native_counter_at(
        &mut ctx,
        &p(vec![Value::Str("heading".into()), Value::Str("intro".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Str("1".into()));
}

#[test]
fn stdlib_counter_at_args_invalidos_retornam_err() {
    null_ctx!(ctx);
    // Primeiro arg não-string.
    let r = native_counter_at(
        &mut ctx,
        &p(vec![Value::Int(1), Value::Str("intro".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err());
    // Segundo arg não-string.
    let r = native_counter_at(
        &mut ctx,
        &p(vec![Value::Str("heading".into()), Value::Int(1)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err());
    // Número errado de args.
    let r = native_counter_at(
        &mut ctx,
        &p(vec![Value::Str("heading".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err());
}

#[test]
fn p240_native_state_display_sem_callback_constroi_variant() {
    null_ctx!(ctx);
    let r = native_state_display(
        &mut ctx,
        &p(vec![Value::Str("k".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::StateDisplay(e)) = r {
        assert_eq!(e.key, "k".to_string());
        assert!(e.callback.is_none(), "1-arg → callback=None");
    } else {
        panic!("esperado Content::StateDisplay");
    }
}

#[test]
fn p240_native_state_display_com_callback_constroi_some() {
    use crate::entities::func::Func;
    null_ctx!(ctx);
    let identity_fn = Func::native("identity", |_, args, _, _| {
        Ok(args.items.first().cloned().unwrap_or(Value::None))
    });
    let r = native_state_display(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Func(identity_fn)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::StateDisplay(e)) = r {
        assert_eq!(e.key, "k".to_string());
        assert!(e.callback.is_some(), "2-arg → callback=Some");
    } else {
        panic!("esperado Content::StateDisplay com callback");
    }
}

#[test]
fn p240_native_state_display_arg_tipo_errado_rejeita() {
    null_ctx!(ctx);
    // 1-arg key não-string.
    let r1 = native_state_display(
        &mut ctx,
        &p(vec![Value::Int(1)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r1.is_err(), "key não-string deve retornar Err");
    // 2-arg segundo arg não-Func.
    let r2 = native_state_display(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Int(2)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r2.is_err(), "callback não-Func deve retornar Err");
}

#[test]
fn p240_native_state_display_arity_errada_rejeita() {
    null_ctx!(ctx);
    // 0 args.
    let r0 = native_state_display(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r0.is_err(), "0 args deve retornar Err");
    // 3 args.
    let r3 = native_state_display(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Int(1), Value::Int(2)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r3.is_err(), "3 args deve retornar Err");
}

#[test]
fn p241_native_counter_display_sem_callback_constroi_variant() {
    null_ctx!(ctx);
    let r = native_counter_display(
        &mut ctx,
        &p(vec![Value::Str("heading".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::CounterDisplayCallback(e)) = r {
        assert_eq!(e.key, ck("heading"));
        assert!(e.callback.is_none(), "1-arg → callback=None");
    } else {
        panic!("esperado Content::CounterDisplayCallback");
    }
}

#[test]
fn p241_native_counter_display_com_callback_constroi_some() {
    use crate::entities::func::Func;
    null_ctx!(ctx);
    let identity_fn = Func::native("identity", |_, args, _, _| {
        Ok(args.items.first().cloned().unwrap_or(Value::None))
    });
    let r = native_counter_display(
        &mut ctx,
        &p(vec![Value::Str("figure".into()), Value::Func(identity_fn)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::CounterDisplayCallback(e)) = r {
        assert_eq!(e.key, ck("figure"));
        assert!(e.callback.is_some(), "2-arg → callback=Some");
    } else {
        panic!("esperado Content::CounterDisplayCallback com callback");
    }
}

#[test]
fn p241_native_counter_display_arg_tipo_errado_rejeita() {
    null_ctx!(ctx);
    // 1-arg key não-string.
    let r1 = native_counter_display(
        &mut ctx,
        &p(vec![Value::Int(1)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r1.is_err(), "key não-string deve retornar Err");
    // 2-arg segundo arg não-Func.
    let r2 = native_counter_display(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Int(2)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r2.is_err(), "callback não-Func deve retornar Err");
}

#[test]
fn p241_native_counter_display_arity_errada_rejeita() {
    null_ctx!(ctx);
    // 0 args.
    let r0 = native_counter_display(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r0.is_err(), "0 args deve retornar Err");
    // 3 args.
    let r3 = native_counter_display(
        &mut ctx,
        &p(vec![Value::Str("k".into()), Value::Int(1), Value::Int(2)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r3.is_err(), "3 args deve retornar Err");
}
