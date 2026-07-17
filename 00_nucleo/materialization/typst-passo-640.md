---
# P640 — `counter.display` com argumentos inválidos, unificando a lógica duplicada

> **Passo:** 640
> **Data:** 2026-07-09
> **Foco:** P633 confirmou seis casos (itens 18–23) onde `counter.display(...)` ignora argumentos inválidos ou desconhecidos sem erro. P638 confirmou que o vanilla já produz erro claro em todos. Três dos seis casos são duplicados de outros três, porque a lógica está implementada duas vezes — uma para a chamada directa (`counter("x").display(...)`), outra para o despacho de método sobre `Value::Counter`. Este passo corrige os seis, e aproveita para unificar a duplicação, em vez de remendar os dois sítios em paralelo.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P633 (casos confirmados), P638 (mensagens de erro do vanilla já extraídas por teste directo).

---

## Contexto

Os seis casos, com a mensagem de erro do vanilla já confirmada por P638:

| # | Local | Problema | Mensagem do vanilla |
|---|---|---|---|
| 18 | `bindings.rs:141` | Argumento posicional não-Str ignorado | `expected string, function, or auto, found integer` |
| 19 | `bindings.rs:156` | `at:` inválido ignorado | `expected label, function, location, selector, or auto, found integer` |
| 20 | `bindings.rs:164` | Argumentos não reconhecidos ignorados | Erro de argumento inesperado |
| 21 | `bindings.rs:273` | Duplicado de 19 (despacho `Value::Counter`) | Mesmo que 19 |
| 22 | `bindings.rs:280` | Duplicado de 18 (despacho `Value::Counter`) | Mesmo que 18 |
| 23 | `bindings.rs:284` | Argumento posicional ignorado quando pattern já definido | Erro de argumento inesperado |

---

## Sonda

### Confirmar a duplicação exacta

```bash
sed -n '130,300p' 01_core/src/rules/eval/bindings.rs
```

Confirmar se as duas implementações (chamada directa vs. despacho de método) podem ser reduzidas a uma função só, chamada dos dois sítios, ou se há alguma diferença genuína entre os dois caminhos que impede a unificação total.

### Critério de fecho da sonda

- [ ] Duplicação confirmada linha a linha.
- [ ] Confirmado se cabe numa função única, ou se precisa de duas versões com uma pequena diferença documentada.

---

## Implementação

Se a sonda confirmar que os dois caminhos são genuinamente a mesma lógica: extrair uma função única (`counter_display_args(args) -> SourceResult<(Option<...>, Smart<...>)>`, ou equivalente), chamada dos dois sítios (`bindings.rs:141/156/164` e `bindings.rs:273/280/284`).

Produzir erro claro para os três problemas:

1. Argumento posicional de tipo inválido.
2. `at:` de tipo inválido.
3. Argumentos não reconhecidos (nomeados desconhecidos, ou posicionais a mais).

### Critério de fecho da implementação

- [ ] Função única criada, se a sonda confirmar que é possível.
- [ ] Os seis casos corrigidos, com uma só correcção em vez de seis remendos separados, se a unificação for possível.
- [ ] Mensagens de erro no mesmo formato do vanilla, já confirmado por P638.
- [ ] Testes de P633 (`p633_counter_display_invalid_arg_silent`, `p633_counter_display_at_invalid_silent`) invertidos.
- [ ] Uso correcto de `counter.display(...)` (as duas formas de chamada) sem regressão.

---

## Validação

```bash
cargo test -p typst-core p633_counter -- --nocapture
```

```bash
cat > /tmp/p640-uso-correcto.typ <<'EOF'
#context counter("x").display("1.")
#context counter("x").display("1.", at: heading.where(level: 1))
EOF
./target/release/typst /tmp/p640-uso-correcto.typ /tmp/p640.pdf
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa.
- [ ] Duplicação unificada, se possível, com razão registada se não for.
- [ ] Seis casos corrigidos, com erro claro.
- [ ] Testes de P633 invertidos e a passar.
- [ ] Uso correcto sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p640.md`, com hash do commit.

---

## Próximo passo

Casos 1, 2 (escape unicode), 4 (`state.update`), 5, 6 (bibliografia) — os últimos da lista de P633, todos confirmados como correcções de paridade genuínas por P638.
