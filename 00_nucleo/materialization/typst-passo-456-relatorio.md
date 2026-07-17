:warning: # Relatório de Execução — P456 — Equation numbering

> **Passo:** 456  
> **Data de execução:** 2026-06-25  
> **Executor:** Kimi Code CLI  
> **Estado:** Concluído

---

## 1. Resumo

O passo P456 materializa a numeração automática de equações matemáticas de
bloco (`$...$`) com contador independente e número renderizado ao lado da
equação. O repositório já possuía a base: `Content::Equation`, contador
`"equation"` no `CounterRegistry`/Introspector e um layout que adicionava o
número inline, mas **usava um gate booleano** e **não suportava patterns**
customizáveis.

Esta execução:

1. Migrou o gate de `equation.numbering` de `Value::Bool` para `Value::Str`,
   análogo a `figure.numbering` (P454) e `heading.numbering.pattern` (P451).
2. Conectou o layout de equation a `format_counter` para suportar `"(1)"`,
   `"[I]"`, `"(a)"`, etc.
3. Posicionou o número à **direita da página**, alinhado com a baseline da
   equação (em vez de inline na linha seguinte).
4. Manteve equações inline não numeradas.

---

## 2. Decisões de arquitectura

Seguindo a arquitectura estabelecida em P365/P454 e a Cláusula 4 da
ADR-0117 (P455), optou-se por:

- **Não adicionar campo `numbering` em `EquationElem`** — o padrão vive só na
  `StyleChain`.
- **Não criar `native_equation(body, numbering?)`** — o Typst vanilla expõe
  `math.equation` via `set` rule, não função nativa.
- **Reaproveitar `Introspector::flat_counter_at("equation", loc)`** — já
  populado pelo walk de introspect.
- **Usar `format_counter` para formatação** — consistência com P451/P454.

---

## 3. Arquivos alterados

| Arquivo | Mudança |
|---------|---------|
| `01_core/src/engine/eval/rules.rs` | `#set math.equation(numbering: ...)` guarda `Value::Str(pattern)` na chain; `Value::None` limpa. |
| `01_core/src/engine/introspect.rs` | Gate `numbering_active` de `Equation` passa a verificar `Value::Str(_)`; fixture de teste actualizada. |
| `01_core/src/engine/layout/equation.rs` | Lê pattern da chain; formata número via `format_counter`; posiciona `FrameItem::Text` à direita; `@prompt` actualizado. |
| `01_core/src/entities/content.rs` | `Content::equation_numbered` passa a usar `Value::Str("(1)")` em vez de `Value::Bool(true)`. |
| `01_core/src/entities/elements/equation.rs` | `@prompt-hash` actualizado para reflexo do prompt L0. |
| `01_core/src/engine/eval/tests.rs` | `find_equation_numbered` adaptada a `Option<String>`; 1 teste novo (pattern romano); fixture `f339t` ajustada a `("(1)")`. |
| `01_core/src/engine/layout/tests.rs` | 3 testes novos (pattern romano, sequencial, inline não numerada). |
| `00_nucleo/prompts/entities/elements/equation.md` | Actualizado para documentar gate `equation.numbering` como `Value::Str`. |
| `00_nucleo/prompts/engine/layout/equation.md` | Criado — especifica layout de equation numbering. |
| `00_nucleo/materialization/typst-passo-456-relatorio.md` | Este relatório. |

---

## 4. Testes adicionados

- **Eval:**
  - `f2s2_b1_set_equation_numbering_pattern_romano_transportado` — pattern
    `"[I]"` transportado na chain.
- **Layout:**
  - `layout_equation_pattern_romano` — pattern `"[I]"` produz `[I]`.
  - `layout_equation_sequencial_numerada` — duas equações block numeradas
    produzem `(1)` e `(2)`.
  - `layout_equation_inline_nao_numerada` — equação inline com gate activo
    não recebe número.

Testes pré-existentes de equation foram adaptados ao novo gate `Value::Str`.

---

## 5. Resultados

### 5.1 `cargo test --workspace`

Executado com `RUST_MIN_STACK=16777216` (stack overflow pré-existente em
`p350c_flag_on_nao_convergente_classifica`).

Resultado: **3222 passed; 0 failed**.

### 5.2 `crystalline-lint .`

Resultado: apenas warnings pré-existentes de prompts órfãos
(`adr-stub-vs-fallback.md`, `show-regex.md`). **Zero violações V5/V7 novas**.

---

## 6. Checklist de fecho

- [x] Eval lê `math.equation.numbering` da `StyleChain` como `Value::Str`.
- [x] Block equation com pattern computa número via Introspector
      (`flat_counter_at("equation", loc)`).
- [x] Layout renderiza número formatado por `format_counter`.
- [x] Número posicionado à direita da página (alinhado com baseline da equação).
- [x] Inline equation ignora numbering.
- [x] 4 novos tests verdes + todos os tests de equation pré-existentes verdes.
- [x] Spec L0 actualizada (`entities/elements/equation.md` + novo
      `engine/layout/equation.md`).
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint` sem violações relacionadas ao P456.

---

## 7. Notas

- O posicionamento à direita é feito calculando `page_width - margin -
  number_width` para o eixo X e usando a baseline da equação para Y. Não é
  centrado verticalmente perfeito (scope-out aceitável para subset minimal).
- O plain_text continua a incluir o número formatado, preservando
  observabilidade textual.
- Cross-reference (`@eq1`), sub-equations e numbering em math mode permanecem
  scope-out, conforme o passo original.

---

**Fim do relatório.**
