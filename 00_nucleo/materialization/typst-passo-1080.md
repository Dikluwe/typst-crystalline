# L0 — Passo 1080: Reclassificação N16[α/β/γ] — Lote 3 (`introspect/`, `layout/`, `export/`)

**Gate**: `ADR-0127` — toca ~37 pontos de código de produção (anotações de
comentário, não lógica). Mesma cautela de mudanças multi-arquivo já usada
nesta conversa.

**Base**: P1070 (amostragem + recomendação). O Lote 3 foi identificado como o
único dos três que exige "auditoria manual caso a caso obrigatória" — ao
contrário dos Lotes 1/2 (`entities/`, `stdlib/`/`eval/`), que a amostra do
P1070 mostrou serem quase 100% mecânicos (β).

---

## 1. Por que este lote é diferente dos outros dois

A amostra do P1070 já revelou 2 casos ambíguos dentro deste universo
(`introspect/labelled.rs:68`, β vs γ; e um caso de `export/stream.rs:1427`, α
vs β) — ambos resolvidos com justificação explícita, não por default. O
critério β/γ aqui depende de julgamento sobre risco de evolução futura do
enum, não de padrão sintáctico reconhecível — por isso não é candidato a
script/heurística em lote como os outros dois lotes podem ser.

## 2. Inventário do Lote 3

```bash
grep -rn "// neutro:" 01_core/src/compiler/introspect/ \
  01_core/src/compiler/layout/ 03_infra/src/export/
```

Reconciliar contra "~37" (o P1070 já usa "~", não número exacto — fechar isso
primeiro).

## 3. Classificação, caso a caso

Para cada um dos ~37: aplicar os critérios já citados no P1070 (α =
impossibilidade estrutural; β = comportamento uniforme genuíno; γ = fallback
deliberado aberto, maior risco). Não amostrar desta vez — são poucos o
suficiente (37) para cobrir 100%, ao contrário do universo de 204 original.

Para cada caso classificado como β ou α quando havia razão para hesitar,
registar por que não ficou γ — a classe γ é "vigilância contínua" per
`ADR-0017`; sub-classificar por conveniência (evitar γ para não gerar mais
avisos) seria trair o propósito da taxonomia.

## 4. Anotação

Mesmo formato já homologado — `// neutro: N16[α/β/γ] — <justificação
específica ao caso, não copiada de outro>.`

## 5. Critérios de verificação

1. 100% dos casos do Lote 3 classificados (não amostra).
2. Nenhum caso ambíguo resolvido sem registar as duas leituras e por que uma
   venceu (mesmo padrão dos 2 casos já resolvidos no P1070).
3. `crystalline-lint --checks v16` — confirmar que o linter aceita as tags
   (`wildcard_saturation.rs:136-151`, já citado no P1070).
4. `cargo test --workspace` — 100% pass.
5. `crystalline-lint .` — 0 erros.

## Critério de conclusão

- Inventário reconciliado (número exacto, não "~37").
- 37 (ou o número real) casos anotados, cada um com justificação própria.
- Nenhum caso γ rebaixado para α/β só para reduzir contagem de avisos.
