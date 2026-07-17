---
# P642 — Propagar o erro do callback de `state.update`

> **Passo:** 642
> **Data:** 2026-07-09
> **Foco:** P633 confirmou (caso 4) que o callback de `state.update(func)`, quando `apply_func` devolve `Err`, é descartado com um comentário "defensive ignore" — a actualização de estado não acontece e o utilizador não recebe diagnóstico. P641 confirmou a fase certa (introspecção, `01_core/src/engine/introspect/from_tags.rs:64`, não `eval/`) e o método de teste certo (integração/pipeline, não `eval/tests.rs` isolado). Este passo corrige.
> **Tipo:** Implementação directa. Causa e localização já confirmadas por P633/P641.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P633 (caso confirmado), P641 (fase e método de teste confirmados, caminho de ficheiro corrigido).

---

## Contexto

`01_core/src/engine/introspect/from_tags.rs:64`:

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
}
```

O comentário já reconhecia que isto era provisório. Este passo faz o refinamento que o comentário previa.

---

## Sonda mínima

### Confirmar o comportamento do vanilla para o mesmo caso

```bash
cat > /tmp/p642-state-erro.typ <<'EOF'
#let s = state("x", 0)
#s.update(x => 1 / 0)
#context s.get()
EOF
lab/typst-original/target/release/typst compile /tmp/p642-state-erro.typ /tmp/p642-vanilla.pdf
```

Confirmar a mensagem de erro exacta e se o vanilla pára a compilação ou continua com um `Sink::warn`.

### Critério de fecho da sonda

- [ ] Comportamento do vanilla confirmado — erro que pára a compilação, ou aviso que continua.

---

## Implementação

Se o vanilla parar a compilação com erro: propagar o `Err` através do mecanismo já disponível no `Engine`/`Sink` (a confirmar qual é o caminho certo — `engine.sink.error(...)`, ou propagar como `SourceResult<()>` a partir da função que processa as tags).

Se o vanilla continuar com aviso: usar `engine.sink.warn(...)` em vez de erro que pára tudo.

```rust
// Esboço, a confirmar contra o comportamento do vanilla:
if let StateUpdate::Func(func) = update {
    if let Some(curr) = intr.state.value_at(key, *loc).cloned() {
        let args = Args::positional(vec![curr]);
        match apply_func(func.clone(), args, &mut scopes, ctx, engine) {
            Ok(new_value) => intr.state.update(key.clone(), new_value, *loc),
            Err(e) => {
                // propagar conforme confirmado pela sonda
            }
        }
    }
}
```

### Critério de fecho da implementação

- [ ] Erro do callback já não é descartado — propagado da forma confirmada pela sonda.
- [ ] Testado com um teste de integração/pipeline (não `eval/tests.rs` isolado), conforme P641 já confirmou ser o método certo.
- [ ] Uso correcto de `state.update` (callback sem erro) sem regressão.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Confirmar especificamente os testes de introspecção/state já existentes, sem regressão.

---

## Critério de fecho do passo

- [ ] Sonda mínima confirmando o comportamento do vanilla.
- [ ] Erro propagado, não descartado.
- [ ] Teste de integração/pipeline escrito, testando a fase certa.
- [ ] Uso correcto sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p642.md`, com hash do commit.

---

## Próximo passo

Casos 1, 2 (escape unicode inválido) e 5, 6 (bibliografia) — os últimos quatro da lista de P633.
