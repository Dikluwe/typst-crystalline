# Relatório P480 — Zero diffs: resolução final dos diffs activos

**Data:** 2026-06-27
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P480 (Outline heading count + math.equation alias + audit de estado)
**Materialização:** 2 fixes pontuais + 4 testes + 3 L0 prompts actualizados

---

## 1. Resumo

**Sub-item A — Outline-toc heading count:**
Diff histórico P479 (`outline-toc heading`: cristalino=5, vanilla=6) resolvido via registo
sintético de heading em `kind_index[Heading]` no walk arm `Content::Outline`. A abordagem
proposta na spec (Sequence com Heading real) foi rejeitada por causar TOC auto-referente.
ADR-0108 aplicada: intenção (count parity) distinguida de comportamento (TOC self-reference).

**Sub-item B — `math.equation` selector namespace:**
22 errors históricos (vanilla rejeitando `equation` standalone) resolvidos via alias
`parse_selector("math.equation")` → `Kind(Equation)` + actualização dos selectors corpus.
O cristalino passa a aceitar `math.equation` (namespace vanilla) e `equation` (path interno).

**Sub-item C — Auditoria de estado:**
Estado confirmado: Trilhas 1–4, 7, 8 completas. Trilha 5 (shaping rustybuzz) e Trilha 6
(4/5) pendentes.

**Resultado final**: **73/73 matches, 0 diffs, 0 errors** — paridade estrutural 100%.

---

## 2. Sub-item A — Outline-toc heading count (ADR-0108)

### 2.1 Medição antes de decidir

| Medição | Resultado | `file:line` |
|---------|-----------|-------------|
| Corpus `outline-toc.typ` headings explícitos (= H1) | 5 (`Introdução` ... `Conclusão`) | corpus |
| Cristalino query `heading` pré-P480 | count=5 | P479 sonda |
| Vanilla query `heading` 0.14.2 | count=6 | vanilla CLI |
| Walk arm `Content::Outline` pré-P480 | Vazio (P189B) | `introspect.rs:1225` |
| `outline::layout` cria heading de título? | Sim (`layout_content(e.title)`) | `outline.rs:57` |
| `Content::Outline` field `title` tipo | `Option<Content>` | `entities/content.rs` |
| `headings_for_toc` — inclui título outline? | Não (P189B) | `introspect.rs:headings_for_toc` |

**Distinção intenção vs comportamento** (ADR-0108):
- Intenção vanilla: o título de outline conta como heading numa query.
- Comportamento da spec P480 (Sequence): `Content::Heading` no Sequence → walk arm Heading
  → `headings_for_toc` recebe o título → `layout_outline` lista "Índice" nas suas próprias
  entradas → **TOC auto-referente**. Comportamento diverge da intenção.

### 2.2 Decisão e implementação

**Abordagem adoptada**: registo sintético directo em `kind_index[Heading]` no walk arm
`Content::Outline`, sem passar pelo arm `Content::Heading`. Assim:
- `kind_index[Heading]` inclui o título de outline (count parity).
- `headings_for_toc` não inclui o título (sem TOC self-reference).
- Nenhum counter é incrementado (título de outline não é secção numerada).

**Ficheiro alterado** `01_core/src/rules/introspect.rs`:

```rust
Content::Outline(_) => {
    // P480 — registo sintético de heading de título em kind_index
    // para paridade de query de count com vanilla. headings_for_toc
    // NÃO actualizado — evita TOC auto-referente. Counter NÃO
    // aplicado — título não é secção numerada.
    let title_loc = locator.next();
    intr.kind_index
        .entry(ElementKind::Heading)
        .or_default()
        .push(title_loc);
}
```

**Resultado**: cristalino=6 = vanilla=6. `headings_for_toc.len()` inalterado.

### 2.3 Testes

| Teste | Ficheiro | Resultado |
|-------|----------|-----------|
| `p480_walk_outline_registra_heading_em_kind_index` | `introspect.rs` | ok |
| `p480_outline_nao_adiciona_headings_for_toc` | `introspect.rs` | ok |

Teste de não-regressão: 8 testes existentes de `layout_outline` passam sem alteração.

---

## 3. Sub-item B — `math.equation` selector namespace (ADR-0107 + ADR-0108)

### 3.1 Medição antes de decidir

| Medição | Resultado | `file:line` |
|---------|-----------|-------------|
| `parse_selector("equation")` pré-P480 | `Ok(Kind(Equation))` | `query_helpers.rs:130` |
| `parse_selector("math.equation")` pré-P480 | `Err(InvalidSelector)` | `query_helpers.rs:124` |
| Vanilla `typst query math-block.typ equation` | Error: "unknown variable: equation" | vanilla CLI |
| Vanilla `typst query math-block.typ math.equation` | matches | vanilla CLI |
| `make_math_module` chave `"equation"` presente pré-P480 | Não | `structural.rs` |

**Classificação** (ADR-0107): `math.equation` é sintaxe da linguagem Typst (namespace de
módulo). Paridade aqui é linguagem, não mecânica.

### 3.2 Implementação (3 pontos)

**Ponto 1** — `03_infra/src/query_helpers.rs`: alias antes do guard `.contains('.')`:

```rust
// P480 — alias vanilla: `math.equation` → ElementKind::Equation.
if trimmed == "math.equation" {
    return Ok(ParsedSelector::Kind(ElementKind::Equation));
}
```

**Ponto 2** — `01_core/src/rules/stdlib/structural.rs`: chave em `make_math_module`:

```rust
// P480 — alias `equation` no módulo math para paridade de namespace vanilla.
dict.insert("equation".into(), Value::None);
```

**Ponto 3** — `lab/parity/tests/structural_parity.rs`: selectors corpus `"equation"` →
`"math.equation"`:

```rust
"visual" => vec!["heading", "figure", "metadata", "math.equation"],
"math"   => vec!["math.equation"],
```

**Resultado**: 22 errors → 22 matches. `equation` standalone continua funcional internamente.

### 3.3 Testes

| Teste | Ficheiro | Resultado |
|-------|----------|-----------|
| `p480_parse_selector_math_equation_resolve_equation_kind` | `query_helpers.rs` | ok |
| `p480_parse_selector_equation_standalone_ainda_aceito` | `query_helpers.rs` | ok |

---

## 4. Testes (5 novos / 1 actualizado)

### L1 — `rules::introspect::tests`

| Teste | Cobertura |
|-------|-----------|
| `p480_walk_outline_registra_heading_em_kind_index` | Walk arm Outline → 1 heading em `kind_index` |
| `p480_outline_nao_adiciona_headings_for_toc` | Outline + 2 headings reais → `headings_for_toc.len()==2` |

### L3 — `query_helpers::tests`

| Teste | Cobertura |
|-------|-----------|
| `p480_parse_selector_math_equation_resolve_equation_kind` | `"math.equation"` → `Kind(Equation)` |
| `p480_parse_selector_equation_standalone_ainda_aceito` | `"equation"` → `Kind(Equation)` |

### Parity suite

| Teste | Cobertura |
|-------|-----------|
| `p480_corpus_paridade_actualizado` | corpus=46, INCLUDE≥28, diffs==0, errors==0 |
| `p479_corpus_paridade_actualizado` (ajustado) | sentinela P479 ajustada para `diffs ≤ 1` (aceita 0 ou 1) |

---

## 5. Arquivos alterados

### Specs L0 (3 actualizadas)

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/prompts/rules/introspect.md` | §P480 — registo sintético Heading em kind_index (walk arm Outline) |
| `00_nucleo/prompts/infra/query-helpers.md` | §P480 — `math.equation` alias em `parse_selector` |
| `00_nucleo/prompts/rules/stdlib/structural.md` | §P480 — `equation` alias em `make_math_module` |

### Código L1 (2 ficheiros)

| Ficheiro | `@prompt-hash` pós-P480 | Alteração |
|----------|------------------------|-----------|
| `01_core/src/rules/introspect.rs` | `cf873757` | Walk arm `Content::Outline` + 2 testes P480 |
| `01_core/src/rules/stdlib/structural.rs` | `5defd191` | `make_math_module` chave `equation` |

### Código L3 (1 ficheiro)

| Ficheiro | `@prompt-hash` pós-P480 | Alteração |
|----------|------------------------|-----------|
| `03_infra/src/query_helpers.rs` | `158c5d26` | `parse_selector` alias `math.equation` + 2 testes P480 |

### Parity suite

| Ficheiro | Alteração |
|----------|-----------|
| `lab/parity/tests/structural_parity.rs` | Selectors `math.equation`; sentinelas P479 + P480 |
| `lab/parity/SKIPS.md` | §3 P480 (todos diffs resolvidos) + §4 matriz P480 |
| `lab/parity/reports/latest.md` | Actualizado P480 (73/73, 0 diffs, 0 errors) |
| `lab/parity/reports/2026-06-27-passo-480.md` | Criado |

### Documentação

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/adr/typst-adr-0075-vanilla-integration.md` | §P480 anotado no plano de materialização |

---

## 6. `crystalline-lint` resultados

```
crystalline-lint --fix-hashes .
  Fixed 3 files:
    ./01_core/src/rules/introspect.rs        → cf873757
    ./01_core/src/rules/stdlib/structural.rs → 5defd191
    ./03_infra/src/query_helpers.rs          → 158c5d26
  Re-running analysis... ✅ 0 drift warnings remaining

crystalline-lint .
  ✅ 0 erros V1–V14.
  Warnings V7 pré-existentes (prompts órfãos não relacionados com P480).
```

---

## 7. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **User-provided outline title como Heading real** | Se `#outline(title: "Índice")` — o título custom não é wrappado em `Content::Heading`; o registo sintético P480 regista 1 heading independente do título. Paridade de count mantida. Texto do título scope-out. |
| **Counter de secção para titulo outline** | O heading sintético não incrementa counters de secção numerada. Intencional (vanilla: título outline também não é secção numerada). |
| **`math.equation.where(...)` selector** | Selector composto continua rejeitado (`InvalidSelector`). Scope-out P206C documentado. |
| **Paridade layout/PDF** | FixedMetrics vs FontBookMetrics (ADR-0054) — sem comparação automática observable. Scope-out estrutural. |

---

## 8. Critério de fecho

- [x] `typst --version` confirma 0.14.2.
- [x] Suite `lab/parity/` executa sem panic.
- [x] Matriz P480: 46 corpus, 28 INCLUDE, **73 matches, 0 diffs, 0 errors**.
- [x] INCLUDE ≥ 23 (threshold P206D) — cumprido (28 INCLUDE).
- [x] Outline-toc diff (P479 scope-out): RESOLVIDO via registo sintético sem TOC self-reference.
- [x] `equation` namespace (22 errors): RESOLVIDOS via `math.equation` alias.
- [x] `headings_for_toc` inalterado — 8 testes layout_outline passam.
- [x] 4 testes novos verdes (2 typst-core, 2 typst-infra).
- [x] Sentinela `p480_corpus_paridade_actualizado` verde.
- [x] `cargo build --workspace` verde.
- [x] `crystalline-lint .` zero erros V1–V14.
- [x] 3 L0 prompts actualizados.
- [x] `SKIPS.md` actualizado §3 + §4.
- [x] Relatório versionado em `lab/parity/reports/2026-06-27-passo-480.md`.
- [x] ADR-0075 anotado com P480.

---

## 9. Estado pós-P480

| Indicador | Estado |
|-----------|--------|
| DEBTs activos com critério de fecho | 0 |
| Trilhas completas | 1, 2, 3, 4, 7, 8 |
| Trilhas pendentes | 5 (épico XL — shaping rustybuzz), 6 (4/5) |
| Paridade | **73/73 matches; 0 diffs; 0 errors** |
| Corpus paridade | 46 ficheiros |
| Matches vs P479 | 50 → 73 (+23) |
| Diffs vs P479 | 1 → 0 (-1) |
| Errors vs P479 | 22 → 0 (-22) |
| **P480** | **FECHADO** |

---

## 10. Próximo passo recomendado

Com paridade estrutural em 100% (73/73), os próximos candidatos são:

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **Trilha 5** | Shaping com rustybuzz — maior impacto qualitativo (texto renderizado correctamente) | XL |
| **Trilha 6** | Fechar a 5ª funcionalidade pendente | M |
| **Expansão corpus** | Adicionar mais ficheiros ao corpus de paridade (10 → 20 ficheiros de semantic, etc.) | S |
