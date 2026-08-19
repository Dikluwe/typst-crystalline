# L0 — Passo 1083: Reclassificação N16[α/β/γ] — Lote 2 Completo (`stdlib/`, `eval/`, `math/`, `parse/`)

**Gate**: `ADR-0127` — toca 28 pontos de código de produção (anotação, não
lógica).

**Base**: P1070 (amostragem) + P1081 (inventário real e reconciliação —
Lote 2 Completo = stdlib 19 + eval 6 + math 2 + parse 1 = 28, opção escolhida
sobre a alternativa de separar math/parse num sub-passo).

---

## 1. Classificar os 28 individualmente

Mesma disciplina do P1082 — não aplicar β em lote sem checar caso a caso.
Casos que merecem atenção extra (não projeção simples óbvia):

- `stdlib/layout.rs:217` — `None => vec![]` (colunas em grid/layout) —
  confirmar se é fallback aberto (γ) ou uniforme (β); listas vazias por
  ausência podem esconder um caso de evolução futura.
- `eval/rules.rs:1014` — `None => {}` (numbering omitido em set-rule) — no-op
  silencioso, mesma dúvida.
- `math/layout/mod.rs:279` — "tratamento especial de layout math" (`_ =>
  false`) — descrição vaga na origem (P1081), confirmar o que realmente faz
  antes de classificar.

## 2. `math/layout/attach.rs:157` — o caso que ficou de fora do Lote 3

Confirmado no P1081 como pendente real (filtro de `grep` do P1080 não
alcançou `math/layout/`, não é duplicação). Incluir aqui.

## 3. Anotação

Mesmo formato já homologado. Usar a lista do P1081 (§3.2, 28 itens) como
checklist.

## 4. Critérios de verificação

1. 28/28 casos anotados.
2. `crystalline-lint --checks v16` aceita todas as tags.
3. `cargo test --workspace` — 100% pass.
4. `crystalline-lint .` — 0 erros.
5. Confirmar contagem final: com P1082+P1083, os 92 casos totais do
   repositório (P1081) devem estar 100% tageados — reconciliar o número final,
   não presumir.

## Critério de conclusão

- 28 casos classificados individualmente, com atenção aos 3 casos do §1.
- `attach.rs:157` (§2) incluído, confirmado não-duplicado.
- Reconciliação final dos 92 casos (todo o repositório) confirmada.
