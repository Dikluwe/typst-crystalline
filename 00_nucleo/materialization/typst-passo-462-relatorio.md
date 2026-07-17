# Relatório — Passo 462: `ref<x>` / `@x`: Resolução de destino e texto da referência

## Resumo

Materializou-se o segundo lado do par `label`/`ref`: a função nativa
`ref(name, supplement: ?)` e a resolução da sintaxe `@x` para o número do
elemento associado a uma `label`. O layout de `Content::Ref` consulta o
`Introspector` (oráculo de counters) e renderiza:

- heading → `"1"` (ou hierárquico `"1.1"`);
- figure → `"Fig. 1"`;
- equation → `"(1)"`;
- table → `"Table 1"`;
- label inexistente → `"?"`.

A implementação separa claramente o caminho numérico (`Content::Label`, P460)
do caminho legacy (`Content::Labelled`, P329), preservando backwards
compatibilidade enquanto abre a Trilha 2 de referências cruzadas.

## Alterações

### 1. `01_core/src/entities/elements/ref.rs`

- `RefElem` remodelado: `name: EcoString` + `supplement: Option<Content>`.
- Remove `Eq`/`Hash` devido a `Content` no supplement; mantém `Clone`,
  `PartialEq`, `Hash`.
- Testes L1 de identidade e supplement.

### 2. `01_core/src/entities/content.rs`

- `Content::reference(name)` delega a `reference_with_supplement(name, None)`.
- `Content::reference_with_supplement(name, supplement)` cria `Content::Ref`.
- Call-sites antigos (`Content::reference(Label(...))`) actualizados para
  strings/`EcoString`.

### 3. `01_core/src/engine/stdlib/ref.rs` (novo)

- `native_ref(ctx, args, ...)` valida nome (`Str` não vazio) e supplement
  opcional (`Content`).
- Emite `Content::Ref(RefElem { name, supplement })`.
- Testes L1: emissão, supplement, nome vazio, arg não-string.

### 4. `01_core/src/engine/stdlib/mod.rs` e `01_core/src/engine/eval/mod.rs`

- Registo de `native_ref` no stdlib (`mod r#ref`, `pub use`).
- `@x` no parser mapeia para `Content::reference(name)` (sem wrap `Label`).
- `make_stdlib()` expõe `"ref"` no scope global.

### 5. `01_core/src/entities/introspector.rs`

- `TagIntrospector` ganha `label_to_counter_key: HashMap<Label, EcoString>`.
- Trait `Introspector` estendido com `counter_key_for_label(&Label) -> Option<&str>`.

### 6. `01_core/src/engine/introspect.rs`

- `populate_intr_from_tag_start` popula `label_to_counter_key` para:
  - Heading → `"heading"`;
  - Figure counted → `"figure:{kind}"`;
  - Equation block + numbered → `"equation"`;
  - Table counted → `"table"`.
- `Content::Styled` propaga `label_from_parent` (permite `Content::Label`
  alcançar figuras/equações embrulhadas em `Styled` de numbering).
- `Content::Labelled` remove a entrada de `label_to_counter_key`, forçando o
caminho legacy e evitando ambiguidade.

### 7. `01_core/src/engine/layout/references.rs`

- `layout_ref` resolução em 4 passos:
  1. `counter_key_for_label` + `query_by_label` + formatação do counter;
  2. Fallback figure legacy (`figure_number_for_label`);
  3. Fallback texto resolvido legacy (`resolved_label_for`);
  4. `"?"` se label não existe.
- `default_supplement_for_key` fornece `"Fig. "` / `"Table "` quando não há
  supplement explícito.

### 8. `01_core/src/engine/layout/mod.rs`

- Caller de `layout_ref` actualizado para passar `&RefElem` em vez de
  `&Label`.

### 9. `03_infra/src/measurements.rs`

- `CountingIntrospector` implementa `counter_key_for_label` com índice 26.
- Arrays `INTROSPECTOR_METHODS`/`CALL_COUNTERS` aumentados para 26 entradas.
- Teste P204G actualizado para `len() == 26`.

### 10. `01_core/src/engine/layout/tests.rs`

- Módulo `p462_ref_numeric` com 7 testes L2/L3:
  - heading, figure, equation, table;
  - supplement explícito;
  - label inexistente (`?`);
  - isolamento de `Content::Labelled` (caminho legacy).
- Fallback final alterado de `@nome` para `"?"` em testes P194B e de
  independência de estado.

### 11. Specs L0 (prompts)

- `00_nucleo/prompts/entities/elements/ref.md`
- `00_nucleo/prompts/entities/introspector.md`
- `00_nucleo/prompts/engine/introspect.md`
- `00_nucleo/prompts/engine/layout_references.md`
- `00_nucleo/prompts/engine/stdlib/ref.md` (novo)
- `00_nucleo/prompts/engine/layout/ref.md` (já existente, alinhado)

## Verificação

```bash
cargo test --workspace -- --skip p350c
```

Resultado: **todos os testes passam** (3253 no `typst-core`, 493 no
`typst-infra`, 21 no `typst-wiring`, 2 no `crystalline_lint`).

A excepção `p350c_flag_on_nao_convergente_classifica` (stack overflow) é
**pré-existente em HEAD** — confirmado via `git stash` antes das alterações.
Não é regressão do P462.

## Scope-out / próximos passos

- PDF `/GoTo` links internos (P463).
- `@x` syntax sugar já funciona via parser; não houve mudanças lexer.
- i18n de supplements.
- Warning em label não encontrada.
