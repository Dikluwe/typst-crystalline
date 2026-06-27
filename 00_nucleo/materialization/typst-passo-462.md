# P462 — `ref<x>` / `@x`: Resolução de destino e texto da referência

> **Passo:** 462  
> **Data:** 2026-06-25  
> **Foco:** Materializar a função nativa `ref(name)` e a sintaxe `@x` para referências cruzadas internas, resolvendo para o número do elemento (heading, figure, equation, table) associado ao `label`.  
> **Trilha:** 2 — Referências cruzadas e navegação interna (depende de P460 `label`, Trilha 1 completa).  
> **ADR-0110:** Hyperlinks e referências cruzadas em documentos estruturados (continuação de P460).

---

## Contexto

O Typst vanilla suporta `#ref<sec1>` ou `@sec1` para referenciar um destino nomeado criado por `#label<sec1>`. A referência resolve para o número do elemento associado (heading, figure, equation, table) e cria um link clicável para o destino. O cristalino tem `Content::Label` (P460) e `/Dests` no PDF, mas **não tem** mecanismo de resolução de referências nem texto substituto com número. Este passo materializa `ref` como o segundo lado do par `label`/`ref`.

**Nota:** P460 implementou `label` (destino nomeado). P462 implementa `ref` (resolução + texto). P463 (PDF `/GoTo`) conecta `ref` a `/Dests` via annotations de link interno.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Label` existe (P460)? | Sim — `LabelElem { name, body }` | ✅ |
| `/Dests` no PDF existe (P460)? | Sim — `PagedDocument.extracted_label_positions` | ✅ |
| `CounterRegistry` com chaves `"heading"`, `"figure"`, `"equation"`, `"table"`? | Sim — P451/P454/P456/P461 | ✅ |
| `format_counter` existe? | Sim — P451 | ✅ |
| `ref`/`@x` existe em stdlib? | Não | ❌ |
| Introspecção de elementos numerados por label? | Parcial — label tem `body`, mas não tem associação explícita a counter key | 🟡 |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** S-M (~25 min; entidade `RefElem` + resolução de número + layout de texto + tests).

---

## Toques pontuais

### 1. Entidade `Ref` (`entities/elements/ref.rs` ou `entities/content.rs`)

```rust
pub struct RefElem {
    pub name: EcoString,      // "sec1", "fig1", "eq1", "tab1"
    pub supplement: Option<Content>,  // Texto antes do número (ex: "Fig. ", "Section ")
}

pub enum Content {
    // ... existing variants
    Ref(RefElem),  // NOVO
}
```

- `ref` é um **elemento ativo** — resolve para texto (número) no momento do layout/eval.
- `supplement` é opcional; se `None`, usa o supplement default do elemento referenciado (ex: "Fig. " para figure, "Table " para table).
- No Typst vanilla, `ref` também pode ter `form` (como exibir: "1.1", "1.1.1", etc.) — scope-out para este passo; usa o pattern do elemento.

**Decisão arquitetural:** `RefElem` não guarda o número resolvido — guarda apenas o `name` do label. A resolução acontece no layout (ou no eval, se o oráculo já estiver populado). Isto é coerente com o Typst vanilla, onde `ref` é resolvido em segunda passagem (ou no layout com oráculo).

### 2. `native_ref` (`rules/stdlib/interactive.rs` ou `rules/stdlib/structural.rs`)

```rust
fn native_ref(name: EcoString, supplement: Option<Content>) -> Content {
    Content::Ref(RefElem { name, supplement })
}
```

- Registar no stdlib scope como `"ref"`.
- Sintaxe sugar `@x` é lexer/parser — scope-out; usa-se `ref("x")` como função nativa.

### 3. Resolução de número (`rules/eval/rules.rs` ou `rules/layout/ref.rs`)

O `ref` precisa resolver o número do elemento associado ao `label`. O `label` tem um `body` que é o conteúdo ao qual está associado. Para obter o número, precisamos de:

- **Opção A:** No eval, fazer introspecção do documento para encontrar o `Content::Label` com `name` correspondente, identificar o tipo do `body` (heading, figure, equation, table), e ler o counter do `CounterRegistry`.
- **Opção B:** No layout, usar o `Introspector`/`CounterRegistry` já populado para obter o número do elemento na posição do label.

**Decisão:** Opção B — coerente com P451/P454/P456/P461, onde counters são lidos do oráculo no layout (ou no eval com oráculo populado). O `ref` é resolvido no momento do layout, quando o oráculo já tem os counters.

**Algoritmo de resolução:**
1. Procurar `label` com `name` em `PagedDocument.extracted_label_positions` (ou numa tabela de labels mantida pelo `Introspector`).
2. Identificar o tipo do elemento associado ao label (heading, figure, equation, table).
3. Obter o número do counter correspondente do `CounterRegistry`/`Introspector`:
   - Heading: `intr.counters["heading"]` na posição do label.
   - Figure: `intr.counters["figure"]` na posição do label.
   - Equation: `intr.counters["equation"]` na posição do label.
   - Table: `intr.counters["table"]` na posição do label.
4. Formatar o número via `format_counter`.
5. Combinar com `supplement` (ou default do tipo) → `"Fig. 1"`, `"Table 2"`, `"(1)"`, etc.

**Problema:** O `label` tem `body: Content`, mas não tem metadado explícito do tipo do elemento. Como saber se o `body` é um heading, figure, equation, ou table?

**Soluções:**
- **A:** O `label` é criado pelo utilizador como `label("fig1", figure(...))` — o `body` é `Content::Figure`. O layout pode inspecionar o tipo do `body`.
- **B:** Adicionar campo `kind: ElementKind` ao `LabelElem` (ou `ElementPayload`) quando o label é criado, para facilitar a resolução.
- **C:** No `Introspector`, manter uma tabela `label_to_kind: HashMap<Label, ElementKind>` que mapeia nome do label para o tipo do elemento.

**Recomendação:** Opção C — adicionar `label_to_kind` (ou `label_to_counter_key`) ao `Introspector`/`CounterRegistry` durante o walk de introspecção. Quando o walk encontra `Content::Label`, inspeciona o `body` e registra o tipo. Isto é mais robusto que inspecionar o `body` no momento da resolução.

### 4. Layout de `ref` (`rules/layout/ref.rs` ou `rules/layout/mod.rs`)

- Ao encontrar `Content::Ref`:
  1. Resolver número via `Introspector` (algoritmo acima).
  2. Se resolvido:
     - Formatar número: `format_counter(&number, pattern)`.
     - Combinar com supplement: `supplement + formatted` (ex: `"Fig. " + "1" = "Fig. 1"`).
     - Renderizar como `FrameItem::Text` com estilo do contexto.
  3. Se não resolvido (label não encontrado):
     - Renderizar como `FrameItem::Text` com `"?"` (ou texto de erro).
     - Ou emitir warning — scope-out para este passo.

**Supplement default por tipo:**
- Heading: `""` (número apenas, ex: `"1.1"`)
- Figure: `"Fig. "` (ou "Figure " — i18n scope-out)
- Equation: `""` (número apenas, ex: `"(1)"`)
- Table: `"Table "` (ou "Tabela " — i18n scope-out)

**Decisão:** Usar supplements fixos em inglês para subset minimal ("Fig. ", "Table ", "Section "). Divergência de i18n declarada como scope-out.

### 5. `Content::Label` vs `Content::Labelled` — nota de consolidação

O P460 introduziu `Content::Label` (user-created) e manteve `Content::Labelled` (P329, auto-generated). Para `ref`, ambos devem ser resolvíveis. No entanto, `Labelled` é usado internamente por figures/equations e pode não ter um nome de label explícito.

**Decisão:** `ref` resolve apenas `Content::Label` (user-created) neste passo. `Content::Labelled` é scope-out para consolidação futura (P463 ou P464). Se o utilizador quer referenciar uma figure, deve adicionar um label explicitamente: `figure(caption: "...") #label<fig1>`.

### 6. Tests

- **L1 (eval):** 2 testes — `native_ref("sec1")` emite `Content::Ref`; `native_ref("sec1", supplement: Some("Section "))` preserva supplement.
- **L2 (layout):** 3 testes — `ref("sec1")` resolve para número de heading; `ref("fig1")` resolve para número de figure; `ref("unknown")` renderiza `"?"`.
- **L3 (E2E):** 2 testes — documento com heading `#label<h1>` e `ref("h1")` renderiza número correcto; documento com figure `#label<f1>` e `ref("f1")` renderiza "Fig. 1".

### 7. Spec L0

- `00_nucleo/prompts/entities/elements/ref.md` — `RefElem { name, supplement }`.
- `00_nucleo/prompts/rules/stdlib/interactive.md` — `native_ref(name, supplement?)`.
- `00_nucleo/prompts/rules/layout/ref.md` — resolução de número e rendering de texto.

---

## Scope-out explícito

- **Sintaxe sugar `@x`** — scope-out; requer lexer/parser. Usa-se `ref("x")`.
- **Links internos `/GoTo`** — scope-out; P463. `ref` é texto apenas neste passo.
- **`form` (formato de exibição do número)** — scope-out; usa pattern do elemento.
- **Supplement automático por i18n** — scope-out; supplements fixos em inglês.
- **Referência a `Content::Labelled` (P329)** — scope-out; apenas `Content::Label`.
- **Warning em label não encontrado** — scope-out; renderiza `"?"`.
- **Referência a múltiplos labels** — scope-out; apenas um label por `ref`.
- **Referência a texto arbitrário (não numerado)** — scope-out; apenas elementos numerados.
- **Page numbers em `ref`** — scope-out; `ref` resolve para número do elemento, não page number.

---

## Critério de fecho

- [ ] `RefElem` criado com `name: EcoString` e `supplement: Option<Content>`.
- [ ] `Content::Ref` adicionado como variante.
- [ ] `native_ref` registada no stdlib como `"ref"`.
- [ ] `Introspector`/`CounterRegistry` ganha mapeamento `label_to_counter_key` (ou similar) durante o walk.
- [ ] Layout resolve número do elemento associado ao label via oráculo.
- [ ] Texto renderizado: `supplement + formatted_number` (ou default por tipo).
- [ ] Label não encontrado renderiza `"?"`.
- [ ] 7 tests verdes (2 L1 + 3 L2 + 2 L3).
- [ ] Spec L0 actualizada (3 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| Resolução no eval (L1) em vez de no layout (L3) | O eval não tem acesso ao oráculo de counters populado; o layout tem. Coerente com P451/P454/P456/P461. |
| `RefElem` guarda número resolvido (eager) | Perde a capacidade de re-layout com oráculo atualizado; diverge do Typst vanilla (2-pass). |
| Inspecionar `body` do label no momento da resolução | Funciona, mas é mais frágil que manter tabela `label_to_kind` no oráculo. A tabela é computada uma vez durante o walk. |
| `ref` como `Content::Text` (número pré-computado) | Perde semântica de referência; dificulta futura adição de `/GoTo` links. |

---

## Próximo passo (Trilha 2 continua)

Com P462, `ref` está materializado. O próximo passo é:
- **P463** — PDF links internos `/GoTo`: Conectar `ref` a `/Dests` via annotations `/GoTo`, tornando as referências clicáveis.
- **P464** — Consolidação `Content::Label` vs `Content::Labelled`: Unificar os dois tipos num único `Content::Label`.

**Aguardando sua indicação:**

1. **Executar o P462** (ref, ~25 min)?
2. **Escrever o P463** (PDF `/GoTo` links internos)?
3. **Pivotar para outra trilha** (Trilha 3: Selector::Where, Trilha 4: visuais, Trilha 6: bibliografia Fase 2, Trilha 8: refinos)?
4. **Ajustar o escopo** do P462 (adicionar `@x` syntax sugar, ou page numbers em ref)?
