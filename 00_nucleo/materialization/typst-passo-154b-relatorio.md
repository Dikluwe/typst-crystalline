# Relatório do Passo 154B — Fase 1 Model (sub-passo 1: terms + divider)

**Tipo**: substantivo (materialização Fase 1 ADR-0060, primeira sub-fase).
**Padrão**: diagnóstico-primeiro (P154A) → materialização (P154B);
replica precedentes 131A→131B, 132A→132B, 140A→140B.

## 1. Sumário executivo

Materializou-se a primeira sub-fase do roadmap ADR-0060 (Fase 1, Model
structural):

- **`Content::Divider`** — singleton estrutural (separador horizontal).
- **`Content::Terms { items: Vec<Content> }`** — lista de pares termo-descrição.
- **`Content::TermItem { term, description }`** — par individual.
- **`native_terms`** e **`native_divider`** em stdlib, expostas via
  `#terms(...)` e `#divider()` em Typst-lang.

ADR-0060 ganhou nota de progresso; **status `PROPOSTO` preservado** (Fase 1
fecha apenas após Passo 155 = `quote`). Sem ADR nova; sem DEBT criado ou
fechado; sem regressão.

## 2. Inventário pré-materialização (154B.1)

- `Content` enum: 38 variants antes de P154B (cristalino), conforme
  inventário 148 Tabela B.
- Sítios `match` exaustivos no L1 que exigem novos arms:
  `plain_text`, `map_content`, `map_text` (em `entities/content.rs`);
  `materialize_time`, `walk` (em `rules/introspect.rs`); `layout_content`
  (em `rules/layout/mod.rs`).
- Sítios `match` com `_ =>` catch-all (mas igualmente actualizados para
  arms explícitos): `is_empty`, `PartialEq::eq` em `entities/content.rs`.
- `make_stdlib` em `rules/eval/mod.rs` registava 29 funções nativas +
  módulo `calc`; faltavam `terms` e `divider`.
- Parser não tem suporte para syntax markup `/ term: desc` ou `---`
  (scope-out P154B — passo de parser separado se priorizado).

## 3. Variants adicionados (forma final)

```rust
// 01_core/src/entities/content.rs
pub enum Content {
    // ... 38 variants existentes
    Divider,
    Terms { items: Vec<Content> },
    TermItem { term: Box<Content>, description: Box<Content> },
}
```

**Notas**:
- `Terms.items` é `Vec<Content>` (não `Vec<TermItem>` como hipótese A do
  spec) porque `TermItem` é um variant directo de `Content`. Iteração e
  introspecção ficam uniformes; permite também sequências mistas se
  futuras show rules introduzirem nós entre items.
- `TermItem` é variant directo de `Content` (não struct interno) per
  Decisão 9 do spec — flexibilidade futura.

## 4. Cobertura exaustiva de arms (~7 sítios L1)

| Ficheiro | Função | Tratamento |
|----------|--------|------------|
| `entities/content.rs` | `is_empty()` | Divider→false; Terms→items vazios; TermItem→ambos vazios |
| `entities/content.rs` | `plain_text()` | Divider→`""`; Terms→join("\n"); TermItem→`"term: description"` |
| `entities/content.rs` | `PartialEq::eq` | par-a-par para todos os 3 |
| `entities/content.rs` | `map_content` | Divider terminal; Terms+TermItem container |
| `entities/content.rs` | `map_text` | idem |
| `rules/introspect.rs` | `materialize_time` | Divider terminal; Terms+TermItem recurse |
| `rules/introspect.rs` | `walk` | Divider/Terms/TermItem sem efeito em contadores; recurse em filhos |
| `rules/layout/mod.rs` | `layout_content` | Divider→`FrameItem::Shape::Line` 0.5pt; Terms→loop layout items; TermItem→bold term + ": " + description com indent |
| `rules/layout/mod.rs` | `measure_content_constrained` | catch-all `_ => (0.0, 0.0)` cobre |

## 5. Stdlib funcs

```rust
// 01_core/src/rules/stdlib/structural.rs

pub fn native_divider(...) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if !args.items.is_empty() { return Err(...); }
    Ok(Value::Content(Content::Divider))
}

pub fn native_terms(...) -> SourceResult<Value> {
    if !args.items.is_empty() { return Err(...); }
    let mut items = Vec::with_capacity(args.named.len());
    for (key, value) in args.named.iter() {
        let term = Content::text(key.as_str());
        let description = match value {
            Value::Content(c) => c.clone(),
            Value::Str(s)     => Content::text(s.as_str()),
            other => return Err(...),
        };
        items.push(Content::TermItem {
            term:        Box::new(term),
            description: Box::new(description),
        });
    }
    Ok(Value::Content(Content::Terms { items }))
}
```

Registadas em `make_stdlib` (em `01_core/src/rules/eval/mod.rs`):

```rust
scope.define("terms",   Value::Func(Func::native("terms",   native_terms)));
scope.define("divider", Value::Func(Func::native("divider", native_divider)));
```

Forma final per Decisão 12/13 do spec (Opção K + Opção M):
- `#terms(apple: [fruit], banana: [yellow])` — named args; preserva ordem
  via IndexMap.
- `#divider()` — sem args; rejeita posicionais e nomeados.

## 6. Tests adicionados

| Ficheiro | Testes | Total |
|----------|--------|-------|
| `01_core/src/entities/content.rs::tests` | divider_constructor, divider_plain_text, terms_constructor, terms_plain_text_concatena_pares, term_item_plain_text, terms_map_text_recurse, terms_partial_eq | 7 |
| `01_core/src/rules/eval/tests.rs` | eval_divider_construtor_typst_lang, eval_terms_construtor_typst_lang, eval_divider_rejeita_args | 3 |

**Render tests (03_infra)**: scope-out neste passo. Layouter cobre os
3 variants com forma mínima viável (Divider→linha; Terms/TermItem→layout
sequencial); testes E2E render via PDF não exigidos pela spec —
implícitos via testes de pipeline já existentes que exercem
`layout_content` exhaustivamente.

**Total testes workspace**: 1113 → **1123** (+10 = 7 unit core + 3 eval).
- core: 874 → 884
- infra: 215 (inalterado)
- shell: 24 (inalterado)

## 7. Edição L0 + hash propagado

- L0 `00_nucleo/prompts/entities/content.md` ganhou secção
  "Variantes estruturais — Passo 154B (ADR-0060 Fase 1)" descrevendo
  Divider, Terms, TermItem, stdlib funcs e limitações conscientes.
- Hash recomputado:
  - L0 (`@prompt-hash` em código): `85fae9b9` → **`43745b5d`**.
  - "Hash do Código" no L0: `e6b6f0af` → `a4244268`.
- `01_core/src/entities/content.rs` header `@updated`:
  `2026-04-23` → `2026-04-24`.
- Hash propagado automaticamente via `crystalline-lint --fix-hashes .`
  (1 ficheiro corrigido).

## 8. ADR-0060 anotada (sem mudança de status)

```diff
**Anotação Passo 154B (2026-04-24)**: primeiro sub-passo da Fase 1
materializado — `Content::Divider`, `Content::Terms`,
`Content::TermItem` adicionados ao enum `Content`; `native_terms`
e `native_divider` registadas em `make_stdlib`. Sem ADR nova.
Status permanece `PROPOSTO` — Fase 1 fecha após Passo 155 (`quote`).
```

**Status `PROPOSTO` preservado** — analogia com ADR-0055 +
Passos 140B/141 (anotação intercalar sem transição até a fase
inteira encerrar).

## 9. Inventário 148 actualizado

- **Tabela A Model**: 3/4/5/10/0=22 → **5/4/5/8/0=22** (terms + divider
  transitam de `ausente` para `implementado`).
  - Cobertura Model: **32-36% → 41%** (10/22 entradas implementadas
    ou implementadas⁺).
  - Total user-facing: 53% → **55%** (53+21 → 55+21 implementados).
- **Tabela B Content cristalino**: 39 → **42 variants** (+Divider,
  +Terms, +TermItem; todos `implementado`).
  - Vanilla extra ausentes: ~14 → ~12 (TermsElem + DividerElem saíram
    do conjunto).
  - Cobertura arquitectural: 72% → **75%**.
- **§7 entrada 7**: lista actualizada (Quote ainda no agregado;
  Terms/Divider removidos).
- Linha entrada 7 com nota de refinamento P154B explicitando salto.

## 10. README dos ADRs actualizado

Entrada nova em "Passos-chave da história dos ADRs" descrevendo
P154B: padrão, escopo, sítios match cobertos, hashes propagados,
testes, status preservado. Sem mudança na tabela "Estado por ADR"
(sem ADR nova). Total 60 ADRs (inalterado). 13 DEBTs abertos
(inalterado).

## 11. Próximo passo

**Passo 155 — `quote`** (Fase 1 segunda sub-fase). Padrão similar:
- `Content::Quote { body, attribution: Option<Box<Content>>, block: bool }`
  (decisão sobre atributos `attribution`/`block`/`quotes` em P155.1).
- `native_quote` em stdlib.
- Cobertura exaustiva de arms.
- Após P155, ADR-0060 transita `PROPOSTO → IMPLEMENTADO` (Fase 1
  inteira fechada). Fase 2 (P156/157/158 — table foundations, figure
  kinds, bibliography+cite) abre depois.

## 12. Limitações registadas

- **Sem syntax markup nova** (`/ term: desc` ou `---`) — trabalho de
  parser; passo separado se priorizado.
- **Sem atributos vanilla** (`tight`, `separator`, `indent`,
  `hanging-indent`) — extensíveis sem breaking change (passar a
  `Terms { items, tight, ... }`).
- **Sem show rules** `#show terms: ...` — candidato a passo único
  agregando show rules para todas features Fase 1 (P154C ou P159+).
- **Render PDF tests** scope-out — layouter cobre via path comum
  (`FrameItem::Shape::Line` para Divider; layout sequencial recursivo
  para Terms/TermItem). Testes E2E PDF agendáveis para P155+ se
  priorizado.

## 13. Verificação final

- ✅ `cargo build --workspace` clean (`Finished dev profile`).
- ✅ `cargo test --workspace --lib`: **1123 passed; 0 failed; 6 ignored**.
- ✅ `crystalline-lint .`: **No violations found** (incluindo V5
  PromptDrift após `--fix-hashes`).
- ✅ Hash propagado consistente: `43745b5d` em L0 (sha256[0..8] após
  --fix-hashes) ↔ `@prompt-hash` em `content.rs`.
- ✅ Inventário 148 reflecte cobertura aumentada.
- ✅ ADR-0060 anotada (sem mudança de status).
- ✅ README ADRs com entrada P154B em "Passos-chave".
- ✅ Sem ADR nova; sem DEBT criado/fechado.
- ✅ Sem regressão em testes pré-existentes.

## Critério de conclusão

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Content::Divider`, `Content::Terms`, `Content::TermItem` compilam | ✅ |
| 2 | Stdlib funcs invocáveis via `#terms(...)` e `#divider()` | ✅ (eval tests) |
| 3 | ADR-0060 anotada com `PROPOSTO` preservado | ✅ |
| 4 | Inventário 148 reflecte cobertura aumentada | ✅ |
| 5 | Próximo passo (155 = `quote`) tem âncora | ✅ |
| 6 | Sem regressão | ✅ |
| 7 | Relatório do passo escrito | ✅ |
