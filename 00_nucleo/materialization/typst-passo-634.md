---
# P634 — Catch-all de `eval_expr` faz `#break`/`#continue`/`#return` desaparecer sem erro

> **Passo:** 634
> **Data:** 2026-07-09
> **Foco:** P633 confirmou, com quatro testes directos, que `eval_expr` (`01_core/src/rules/eval/mod.rs:819`) devolve `Ok(Value::None)` para qualquer variante de `Expr` ainda não migrada — incluindo `#break`, `#continue`, e `#return` usados fora de um ciclo/função, e até erros de sintaxe (`#let x = 0xZZ` vira `x = none`). É o caso mais grave da lista, porque afecta construções fundamentais da linguagem, não uma propriedade opcional.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P633 (onde a causa foi confirmada com `file:line` e quatro testes já escritos).

---

## Contexto

O braço catch-all de `eval_expr` foi criado como medida temporária, presumivelmente durante a migração incremental de variantes de `Expr` (histórico do projecto, atomização gradual). O problema: qualquer variante ainda não migrada, ou qualquer erro de parsing que produza uma expressão malformada, cai neste braço e vira silenciosamente `Value::None`, sem nenhum sinal de que algo correu mal.

---

## Sonda

### Confirmar exactamente que variantes caem no catch-all hoje

```bash
grep -n "match.*self\|match expr\|_ =>" 01_core/src/rules/eval/mod.rs | grep -A2 -B20 "^819:"
```

Listar, com o `match` completo à volta da linha 819, todas as variantes de `Expr` já tratadas explicitamente, e confirmar por eliminação quais caem no catch-all.

### Confirmar se `#break`/`#continue`/`#return` têm braços próprios nalgum sítio (dentro de ciclos/funções) que simplesmente não cobrem o caso de uso no topo do documento

```bash
grep -n "Expr::Break\|Expr::Continue\|Expr::Return" 01_core/src/rules/eval/mod.rs
```

### Critério de fecho da sonda

- [ ] Lista completa de variantes que caem no catch-all hoje, com `file:line` do `match`.
- [ ] Confirmado se `Break`/`Continue`/`Return` já têm tratamento dentro de contexto (ciclo/função) e só falta o caso de uso fora desse contexto.
- [ ] Confirmado o comportamento do vanilla para os mesmos casos — `#break` no topo do documento deve produzir erro claro, não `none`.

---

## Implementação

Substituir o catch-all genérico por tratamento explícito, ou por um erro claro quando a variante não é reconhecida.

Para `#break`/`#continue` fora de ciclo, e `#return` fora de função: produzir um erro específico ("break outside of loop", ou a mensagem que o vanilla já usa — confirmar com sonda antes de escolher o texto).

Para erros de parsing (`0xZZ`): confirmar que o parser já devia ter reportado o erro antes de chegar a `eval_expr` — se `eval_expr` está a receber uma expressão malformada, o problema pode estar mais cedo no pipeline (parser), não só na avaliação.

### Critério de fecho da implementação

- [ ] `#break`/`#continue` fora de ciclo produzem erro claro.
- [ ] `#return` fora de função produz erro claro.
- [ ] `#let x = 0xZZ` produz erro (no parser, ou na avaliação, confirmado onde faz mais sentido depois da sonda).
- [ ] Testes já escritos por P633 (`p633_break_top_level_silently_none`, etc.) invertidos — devem passar a esperar `Err`, não `Ok(Value::None)`.
- [ ] Confirmar que nenhum uso legítimo de `#break`/`#continue`/`#return` dentro do seu contexto próprio (ciclo, função) regride.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Correr especificamente os testes de P633 relacionados, confirmando que passaram de "confirmam a falha silenciosa" para "confirmam o erro correcto":

```bash
cargo test -p typst-core p633_break_top_level
cargo test -p typst-core p633_continue_top_level
cargo test -p typst-core p633_return_top_level
cargo test -p typst-core p633_parse_error_expr
```

Testar também os casos legítimos, para confirmar que não regrediram:

```bash
cat > /tmp/p634-break-legitimo.typ <<'EOF'
#for i in range(5) {
  if i == 2 { break }
}
EOF
./target/release/typst /tmp/p634-break-legitimo.typ /tmp/p634.pdf
```

---

## Critério de fecho do passo

- [ ] Sonda completa, todas as variantes do catch-all mapeadas.
- [ ] `#break`/`#continue`/`#return` fora de contexto produzem erro claro.
- [ ] Testes de P633 invertidos (de "confirma falha" para "confirma erro correcto").
- [ ] Uso legítimo dentro de ciclo/função sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p634.md`, com hash do commit.

---

## Próximos passos da lista de P633

Depois deste, seguir pelos grupos já identificados no relatório de P633, um de cada vez:

1. **Regras `#set` que ignoram tipo inválido** (itens 8–16 de P633) — nove casos com o mesmo padrão, provavelmente corrigíveis juntos por serem a mesma causa estrutural repetida em `rules.rs`.
2. **`counter.display` com argumentos inválidos** (itens 18–23) — seis casos, dois deles duplicados por a lógica estar implementada duas vezes (chamada directa vs. método sobre `Value::Counter`); considerar unificar essa duplicação ao corrigir, não só corrigir os dois sítios em paralelo.
3. **Bibliografia e estado** (itens 4–6) — entradas/actualizações descartadas sem aviso.
