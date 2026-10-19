---

# P494 — Expansão de `parse_selector` para Elementos de Documento (D1)

> **Passo:** 494
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os 7 gaps de selectors não registrados identificados no diagnóstico P490 (Grupo D1). Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação M-size com validação via bateria P490.
> **Tamanho:** M (~60 min de implementação + 20 min de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P491 (D2 fechado), P492 (D4/D5 fechado), P493 (D3 fechado), P470 (list/enum/par), P452/P463 (links), P155 (quotes), P304/P305 (footnotes).

---

## Metodologia

Para cada sub-tarefa D1a–D1b, registrar o selector no `parse_selector`, correr a bateria P490 (20 ficheiros), e comparar o output estrutural contra o baseline do diagnóstico.

```bash
# Baseline P493 (antes de tocar código):
cargo test --test structural_parity p493_field_access_colecoes

# Após cada sub-tarefa:
cargo test --test structural_parity p494_<subtarefa>
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## Categoria 1 — Selectors de Layout de Texto (D1a)

### 1.1 — Estado baseline (P493)

| Ficheiro | Selector | Vanilla | Cristalino pré-494 | Classificação P493 |
|---|---|---|---|---|
| test-list-marker-array.typ | `list` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** |
| test-enum-start.typ | `enum` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** |
| test-par.typ | `par` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** |

- **Vanilla:** `typst query test-list-marker-array.typ "list"` → conta 1.
- **Cristalino (pré-494):** `list` não é selector reconhecido; `parse_selector` não mapeia `"list"` → `Selector::Kind(List)`.
- **Classificação P493:** **AUSENTE** (selector não registrado)

### 1.2 — Diagnóstico de causa raiz

O cristalino expõe apenas: `heading`, `figure`, `citation`, `metadata`, `state`, `state_update`, `outline`, `bibliography`, `equation`, `counter_update`.

O gap é em `parse_selector` (ou `Introspector::query_to_selector`): o mapeamento de string → `Selector::Kind` está incompleto para elementos de layout de texto.

**Hipóteses:**
1. **H1:** `Selector::Kind(List)`, `Selector::Kind(Enum)`, `Selector::Kind(Par)` existem no enum mas não estão no mapa de parse.
2. **H2:** As variants `List`, `Enum`, `Par` não existem em `Selector::Kind` — precisam ser adicionadas ao enum.
3. **H3:** O `Introspector` não sabe como contar `List`/`Enum`/`Par` elementos no documento.

**Verificação rápida:**
```bash
rg -n "Selector::Kind\|parse_selector" src/ --type rs
rg -n "enum Selector" src/ --type rs
rg -n "Introspector::query" src/ --type rs
```

### 1.3 — Implementação

**Arquivo alvo:** `src/introspect/selector.rs` (ou `src/eval/selector.rs`)

**Mudança 1 (enum):** Adicionar variants ao enum `Selector::Kind` (se ainda não existirem):

```rust
pub enum Selector {
    Kind(ElementKind),
    // ... variants existentes
}

pub enum ElementKind {
    Heading,
    Figure,
    Citation,
    Metadata,
    State,
    StateUpdate,
    Outline,
    Bibliography,
    Equation,
    CounterUpdate,
    // Novos D1a:
    List,
    Enum,
    Par,
    // Novos D1b (se implementado junto):
    Link,
    Raw,
    Quote,
    Footnote,
}
```

**Mudança 2 (parse):** Adicionar mapeamento string → `ElementKind`:

```rust
pub fn parse_selector(s: &str) -> Option<Selector> {
    match s {
        "heading" => Some(Selector::Kind(ElementKind::Heading)),
        "figure" => Some(Selector::Kind(ElementKind::Figure)),
        // ... existentes
        "list" => Some(Selector::Kind(ElementKind::List)),
        "enum" => Some(Selector::Kind(ElementKind::Enum)),
        "par" => Some(Selector::Kind(ElementKind::Par)),
        "link" => Some(Selector::Kind(ElementKind::Link)),
        "raw" => Some(Selector::Kind(ElementKind::Raw)),
        "quote" => Some(Selector::Kind(ElementKind::Quote)),
        "footnote" => Some(Selector::Kind(ElementKind::Footnote)),
        _ => None,
    }
}
```

**Mudança 3 (introspector):** Adicionar contagem de elementos no `Introspector::query`:

```rust
fn query_selector(&self, selector: &Selector) -> Vec<Content> {
    match selector {
        Selector::Kind(ElementKind::List) => self.doc.iter().filter(|c| c.is_list()).collect(),
        Selector::Kind(ElementKind::Enum) => self.doc.iter().filter(|c| c.is_enum()).collect(),
        Selector::Kind(ElementKind::Par) => self.doc.iter().filter(|c| c.is_par()).collect(),
        Selector::Kind(ElementKind::Link) => self.doc.iter().filter(|c| c.is_link()).collect(),
        Selector::Kind(ElementKind::Raw) => self.doc.iter().filter(|c| c.is_raw()).collect(),
        Selector::Kind(ElementKind::Quote) => self.doc.iter().filter(|c| c.is_quote()).collect(),
        Selector::Kind(ElementKind::Footnote) => self.doc.iter().filter(|c| c.is_footnote()).collect(),
        // ... existentes
    }
}
```

**Nota:** Os métodos `is_list()`, `is_enum()`, etc. devem existir em `Content`. Se não existirem, adicionar em `src/model/content.rs`:

```rust
impl Content {
    pub fn is_list(&self) -> bool { matches!(self, Content::List(_)) }
    pub fn is_enum(&self) -> bool { matches!(self, Content::Enum(_)) }
    pub fn is_par(&self) -> bool { matches!(self, Content::Par(_)) }
    pub fn is_link(&self) -> bool { matches!(self, Content::Link(_)) }
    pub fn is_raw(&self) -> bool { matches!(self, Content::Raw(_)) }
    pub fn is_quote(&self) -> bool { matches!(self, Content::Quote(_)) }
    pub fn is_footnote(&self) -> bool { matches!(self, Content::Footnote(_)) }
}
```

### 1.4 — Validação

Rodar os 3 ficheiros contra vanilla e cristalino. Esperado:
- `test-list-marker-array.typ` → `list` → **MATCH** (count=1)
- `test-enum-start.typ` → `enum` → **MATCH** (count=1)
- `test-par.typ` → `par` → **MATCH** (count=1)

---

## Categoria 2 — Selectors de Conteúdo Inline (D1b)

### 2.1 — Estado baseline (P493)

| Ficheiro | Selector | Vanilla | Cristalino pré-494 | Classificação P493 |
|---|---|---|---|---|
| test-show-link.typ | `link` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** |
| test-raw.typ | `raw` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** |
| test-quote.typ | `quote` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** |
| test-footnote.typ | `footnote` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** |

- **Vanilla:** `typst query test-show-link.typ "link"` → conta 1.
- **Cristalino (pré-494):** `link` não é selector reconhecido.
- **Classificação P493:** **AUSENTE**

### 2.2 — Diagnóstico de causa raiz

Mesma causa que D1a: `parse_selector` não mapeia estas strings. As funcionalidades subjacentes (links, raw, quotes, footnotes) já estão implementadas (P452/P463, P304/P305, P155), mas o mecanismo de query não as expõe.

### 2.3 — Implementação

**Arquivo alvo:** Mesmo que D1a (`src/introspect/selector.rs`, `src/model/content.rs`).

As mudanças já foram descritas em D1a (se D1a e D1b forem implementados juntos). Se forem implementados separadamente, D1b requer apenas:
- Adicionar `Link`, `Raw`, `Quote`, `Footnote` ao `ElementKind` (se ainda não feito em D1a).
- Adicionar mapeamento no `parse_selector` (se ainda não feito em D1a).
- Adicionar contagem no `Introspector::query` (se ainda não feito em D1a).

### 2.4 — Validação

Rodar os 4 ficheiros contra vanilla e cristalino. Esperado:
- `test-show-link.typ` → `link` → **MATCH** (count=1)
- `test-raw.typ` → `raw` → **MATCH** (count=1)
- `test-quote.typ` → `quote` → **MATCH** (count=1)
- `test-footnote.typ` → `footnote` → **MATCH** (count=1)

---

## Formato do relatório de resultados

Para cada sub-tarefa, produzir uma linha:

```
| 494a list/enum/par | Vanilla: ok(1) | Cristalino: ok(1) | MATCH | 3 selectors registrados |
| 494b link/raw/quote/footnote | Vanilla: ok(1) | Cristalino: ok(1) | MATCH | 4 selectors registrados |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

**Classificações:**
- `MATCH` — output estruturalmente equivalente.
- `DIFF` — ambos produzem resultado mas diferente (investigar).
- `ERRO_DESCRITIVO` — cristalino retorna erro com mensagem clara (aceitável se scope-out).
- `PANIC` — cristalino crasha (bug prioritário, deve ser zero).
- `AUSENTE` — funcionalidade não reconhecida.

---

## Testes de não-regressão

Após as 2 sub-tarefas, rodar a bateria completa P490 (20 ficheiros):

| Ficheiro | Selector | Vanilla | Cristalino pré-494 | Esperado pós-494 | Δ |
|---|---|---|---|---|---|
| test-list-marker-array.typ | `list` | ok(1) | ERRO_DESCRITIVO | ok(1) | **AUSENTE → MATCH** |
| test-enum-start.typ | `enum` | ok(1) | ERRO_DESCRITIVO | ok(1) | **AUSENTE → MATCH** |
| test-par.typ | `par` | ok(1) | ERRO_DESCRITIVO | ok(1) | **AUSENTE → MATCH** |
| test-show-link.typ | `link` | ok(1) | ERRO_DESCRITIVO | ok(1) | **AUSENTE → MATCH** |
| test-raw.typ | `raw` | ok(1) | ERRO_DESCRITIVO | ok(1) | **AUSENTE → MATCH** |
| test-quote.typ | `quote` | ok(1) | ERRO_DESCRITIVO | ok(1) | **AUSENTE → MATCH** |
| test-footnote.typ | `footnote` | ok(1) | ERRO_DESCRITIVO | ok(1) | **AUSENTE → MATCH** |
| test-table.typ | `table` | ok(1) | ok(1) | ok(1) | MATCH (P493) |
| test-array.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P493) |
| test-show-where-multi.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH (P493) |
| test-stroke-sides.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH (P492) |
| test-show-regex.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH (P492) |
| test-calc.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P491) |
| test-str.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P491) |
| test-dict.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P491) |
| test-math.typ | `math.equation` | ok(5) | ok(5) | ok(5) | MATCH |
| test-set-local.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH |
| test-columns.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH |
| test-page.typ | `page` | ok(0) | ok(0) | ok(0) | MATCH |
| test-place.typ | `place` | ok(0) | ok(0) | ok(0) | MATCH |

**AUSENTEs restantes esperados:** 7 - 7 = **0** (todos os AUSENTEs do P490 resolvidos).  
**DIFFs esperados:** **0** (todos resolvidos em P493).  
**PANICs esperados:** **0** (preservado).

**Resultado final da bateria P490 pós-P494:**
- **MATCH:** 20/20 ficheiros
- **DIFF:** 0
- **AUSENTE:** 0
- **PANIC:** 0

---

## Critério de fecho

- [ ] 494a implementado: `list`, `enum`, `par` registrados como selectors.
- [ ] 494b implementado: `link`, `raw`, `quote`, `footnote` registrados como selectors.
- [ ] 7 testes unitários novos passam (`p494_selector_list`, `p494_selector_enum`, `p494_selector_par`, `p494_selector_link`, `p494_selector_raw`, `p494_selector_quote`, `p494_selector_footnote`).
- [ ] Bateria P490 completa: 7 AUSENTEs viraram MATCH.
- [ ] AUSENTEs restantes: **0** (todos os AUSENTEs do P490 resolvidos).
- [ ] DIFFs restantes: **0** (todos resolvidos em P493).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (`entities/selector.md` com lista completa de selectors; `rules/introspect.md` se existir).
- [ ] ADR-0107 checklist atualizado (7 itens marcados implementado).
- [ ] Sentinela `p494_selectores_elementos_documento` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p494.md` produzido com tabela de resultados.
- [ ] **Bateria P490: 20/20 MATCH** — paridade funcional completa alcançada.

---

## Próximo passo (P495)

Com P494 fechado, a bateria P490 atinge **20/20 MATCH**. A paridade funcional do diagnóstico inicial está completa.

**Recomendações para P495+:**

1. **P495 = Audit de cobertura stdlib expandida** — criar novos ficheiros `.typ` para funcionalidades não cobertas pela bateria P490 (ex: `calc` args nomeados restantes, `str` métodos avançados, `dict` métodos, `image` com `fit`, `page` com `header`/`footer`, `place` avançado).

2. **P495 = Performance benchmark** — comparar tempo de compilação cristalino vs vanilla em corpus maior (DEBT-42).

3. **P495 = Documentação de paridade** — produzir relatório final de paridade funcional consolidando P490-P494, com matriz de cobertura stdlib.

4. **P495 = DEBT-42 Benchmark Plan** — executar o plano de benchmark documentado em `typst-cobertura-vaistalino.md`.

---

## A. Apêndice — Referência rápida dos gaps D1

```typst
// 494a: list, enum, par
#list([Item A], [Item B])
#enum([Primeiro], [Segundo])
#set par(leading: 1.5em)
Parágrafo.

// 494b: link, raw, quote, footnote
#link("https://example.com")[Sítio]
```rust
fn main() {}
```
#quote(attribution: [Autor])[Citação]
Texto com nota.#footnote[Nota de rodapé.]
```

---

## B. Apêndice — Mapeamento completo de selectors (pós-P494)

| Selector | ElementKind | Status pré-P494 | Status pós-P494 |
|---|---|---|---|
| `heading` | `Heading` | ✓ | ✓ |
| `figure` | `Figure` | ✓ | ✓ |
| `citation` | `Citation` | ✓ | ✓ |
| `metadata` | `Metadata` | ✓ | ✓ |
| `state` | `State` | ✓ | ✓ |
| `state_update` | `StateUpdate` | ✓ | ✓ |
| `outline` | `Outline` | ✓ | ✓ |
| `bibliography` | `Bibliography` | ✓ | ✓ |
| `equation` | `Equation` | ✓ | ✓ |
| `counter_update` | `CounterUpdate` | ✓ | ✓ |
| `list` | `List` | ✗ | ✓ |
| `enum` | `Enum` | ✗ | ✓ |
| `par` | `Par` | ✗ | ✓ |
| `link` | `Link` | ✗ | ✓ |
| `raw` | `Raw` | ✗ | ✓ |
| `quote` | `Quote` | ✗ | ✓ |
| `footnote` | `Footnote` | ✗ | ✓ |
| `table` | `Table` | ✓ | ✓ |
| `math.equation` | `MathEquation` | ✓ | ✓ |
