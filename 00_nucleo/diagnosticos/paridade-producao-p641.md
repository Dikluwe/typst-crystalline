# Relatório de Paridade — P641

**Passo:** 641  
**Data:** 2026-07-09  
**Foco:** Confirmar se `#context` adia avaliação do corpo e como isso afecta o método de teste do caso 4 (`state.update` callback com erro descartado).  
**Dependências:** P640 (descoberta do adiamento de `#context` ao reescrever testes de `counter.display`).

---

## 1. Comportamento de `#context` no cristalino

### 1.1 Eval

`01_core/src/rules/stdlib/context.rs:23-48`:

```rust
pub fn native_context(...) -> SourceResult<Value> {
    ...
    [Value::Func(func)] => {
        let id = ctx.next_context_id();
        Ok(Value::Content(Content::ContextBlock(Arc::new(
            ContextBlockElem { id, closure: func.clone() },
        ))))
    }
    ...
}
```

`#context { body }` avalia o body como uma **closure** e devolve `Content::ContextBlock`. O corpo **não é avaliado imediatamente**.

### 1.2 Pipeline

`03_infra/src/pipeline.rs:105-156`:

```rust
pub fn expand_context_blocks(
    content: Content,
    intr: &TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Content> {
    ...
    for (id, loc) in &intr.context_block_locations {
        ...
        let result = apply_func(
            elem.closure.clone(),
            Args::positional(vec![]),
            &mut scopes,
            &mut ctx,
            &mut engine,
        )?;
        resolved.insert(*id, value_to_content(&result));
    }
    Ok(substitute_context_blocks(content, &resolved))
}
```

A expansão acontece **após a introspeção** (`pipeline.rs:345-350`). O corpo do context só é avaliado nesta fase, com `in_context = true` e a localização capturada pelo walk de introspeção.

### 1.3 Layout

`01_core/src/rules/layout/mod.rs:1163-1165`:

```rust
// ── P506 — ContextBlock (delayed evaluation). Deve ter sido
// expandido antes do layout; se chegou aqui, é defensive no-op.
Content::ContextBlock(_) => {}
```

Ao chegar ao layouter, qualquer `ContextBlock` residual é ignorado — a expansão deve já ter acontecido.

---

## 2. Comportamento do vanilla

`lab/typst-original/crates/typst-eval/src/code.rs:386-409`:

```rust
impl Eval for ast::Contextual<'_> {
    type Output = Content;

    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output> {
        let body = self.body();
        let captured = { ... };
        let closure = Closure {
            node: ClosureNode::Context(self.body().to_untyped().clone()),
            defaults: vec![],
            captured,
            num_pos_params: 0,
        };
        let func = Func::from(closure).spanned(body.span());
        Ok(ContextElem::new(func).pack().spanned(body.span()))
    }
}
```

No vanilla, `#context { body }` também cria uma closure dentro de `ContextElem`. O corpo **não é avaliado imediatamente** no eval; é avaliado mais tarde, durante o layout, quando o contexto é necessário.

Conclusão: o adiamento é **intencional e igual ao vanilla**, não uma diferença de arquitectura do cristalino.

---

## 3. Onde `state.update` callback é aplicado

O caso 4 de P633 referia-se a `from_tags.rs:64`. O ficheiro correcto é `01_core/src/rules/introspect/from_tags.rs:64` (não `eval/from_tags.rs`, que não existe):

```rust
if let StateUpdate::Func(func) = update {
    if let Some(curr) = intr.state.value_at(key, *loc).cloned() {
        let args = Args::positional(vec![curr]);
        if let Ok(new_value) =
            apply_func(func.clone(), args, &mut scopes, ctx, engine)
        {
            intr.state.update(key.clone(), new_value, *loc);
        }
        // Err: defensive ignore — refino futuro pode
        // propagar via Sink.
    }
    // value_at == None: defensive ignore (P171 padrão
    // "update sem init").
}
```

Esta função faz parte do processamento de tags durante a **introspecção** (walk de `TagIntrospector`). O callback de `state.update` só é executado nesta fase.

---

## 4. Implicação para testes

| Construção | Fase em que o erro se manifesta | Teste adequado |
|---|---|---|
| `counter.display(...)` com argumentos inválidos | Eval (se `at:` estiver presente) ou layout/expansão de context | `eval/tests.rs` com `at:` fora de context, como feito em P640 |
| `state.update(callback)` com callback que dá erro | Introspecção (`from_tags.rs:64`) | Não basta `eval/tests.rs`; é preciso correr a pipeline até à introspeção (testes em `introspect/` ou `03_infra/src/integration_tests.rs`) |
| `#context(state.update(...))` | Expansão de context, **após** introspeção | Provavelmente o update não será processado pela introspeção; não é o cenário correcto para testar o caso 4 |

Portanto, o teste do caso 4 deve usar `#state("x", 0).update(x => ...)` **fora de contexto** e correr até à fase de introspeção, para que `from_tags.rs:64` seja atingido.

---

## 5. Decisão

- A correcção do caso 4 (`state.update` callback descartando erro) deve ser acompanhada por um teste de **integração/pipeline**, não por um teste isolado em `eval/tests.rs`.
- O teste deve colocar o `state.update` fora de `#context`, para que o `TagIntrospector` o processe durante o walk de introspeção.
- A correção em si é simples: propagar o `Err` de `apply_func` em `from_tags.rs:64` em vez de o ignorar.

---

## 6. Estado de fecho

- [x] Comportamento de `#context` confirmado: cria `Content::ContextBlock` no eval; corpo avaliado em `expand_context_blocks` (`03_infra/src/pipeline.rs:105`), após introspecção.
- [x] Vanilla confirmado com comportamento semelhante (`Contextual::eval` cria `ContextElem` com closure).
- [x] `state.update` callback confirmado como processado durante introspecção (`01_core/src/rules/introspect/from_tags.rs:64`).
- [x] Método de teste certo decidido: teste de pipeline/introspecção, com `state.update` fora de `#context`.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p641.md`.

---

## 7. Nota sobre o path em P633

O relatório de P633 indicava `01_core/src/rules/eval/from_tags.rs:64`. O ficheiro correcto é `01_core/src/rules/introspect/from_tags.rs:64`. O path no relatório de P633 está desactualizado (provavelmente resultado de uma reorganização de módulos). P641 regista esta correção.
