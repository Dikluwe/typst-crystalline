# Prompt L0 — `infra/query-helpers`
Hash do Código: c7ea6387

**Camada**: L3.
**Fase**: P206C / Vanilla integration.
**ADRs vinculantes**: ADR-0075 (vanilla integration via
pre-built CLI + comparação estrutural; PROPOSTO).
**Cross-references**: P206A diagnóstico (auditoria
empírica); P206C C2 = Caminho B (helper em workspace
cristalino); ADR-0073 §C6a fechada por F3 (P205B+C);
trait `Introspector` (M8 P204B); `entities/selector.md`
(P175 minimal).

---

## Contexto

P206A auditou vanilla integration empíricamente e fixou
ADR-0075 PROPOSTO. P206C materializa comparação
estrutural cristalino vs vanilla via JSON shape
compatível com `typst query`.

P206C C2 fixou **Caminho B** (helper em workspace
cristalino, não subcomando CLI exposto):

- Caminho A (subcomando CLI em `04_wiring/`) era **L**
  por exigir refactor cross-modular (`main.rs` +
  Selector parsing + JSON shape vanilla replication +
  L0 prompts updated).
- Caminho B é **M** — módulo dedicado L3 com selector
  parsing + dispatch a `Introspector::query_*` +
  output domain struct.
- Caminho C (helper em `lab/parity/`) contradiria
  clarificação inicial ("cristalino expõe helper").

C3 documenta resolução parcial da tensão: clarificação
"novo CLI cristalino" honrada via helper L3 (cristalino
expõe via API pública); CLI subcomando deferred para
sub-passo dedicado pós-P206.

---

## Decisão

`03_infra/src/query_helpers.rs` — módulo L3 que expõe:

1. **Selector parsing** — string → enum interno
   discriminando entre:
   - **Kind** (ex: `"heading"`, `"figure"`,
     `"metadata"`) → `Selector::Kind(ElementKind)`
     via `ElementKind::from_str`.
   - **Label** (ex: `"<fig-alfa>"`) → label string
     extraído entre `<...>` → `Introspector::query_by_label`.
   - **Inválido** → erro com mensagem.

2. **Query execution** — pipeline integrado:
   - Eval source via `eval_to_module_with_sink`.
   - Extrair Content via `module.content`.
   - Construir TagIntrospector via `introspect`.
   - Aplicar selector → dispatching a método correcto.
   - Retornar **`QuerySummary`** estrutura mínima.

3. **`QuerySummary`** — struct domain-level (sem
   serde):
   ```text
   pub struct QuerySummary {
       pub selector: String,         // input literal
       pub kind: SelectorKind,       // Kind | Label
       pub count: usize,             // count de matches
       pub kind_name: Option<String>, // nome do kind se Kind selector
       pub label_found: Option<String>, // label se Label selector com match
       pub metadata_values: Vec<String>, // se selector="metadata", plain text de cada value
   }
   ```

4. **`QueryError`** — enum:
   - `EvalFailed` (eval produziu erros).
   - `NoContent` (eval ok mas sem content).
   - `InvalidSelector(String)` (selector não parseável).
   - `WorldError(String)` (I/O ou source loading).

5. **Função pública principal**:
   ```text
   pub fn query_to_summary(
       world: &SystemWorld,
       source: &Source,
       selector: &str,
   ) -> Result<QuerySummary, QueryError>;
   ```

6. **Funções auxiliares pub** (úteis para callers
   isolados):
   - `parse_selector(s: &str) -> Result<ParsedSelector, QueryError>`.
   - `summarize_query(intr: &TagIntrospector, parsed: &ParsedSelector) -> QuerySummary`.

---

## Restrições

- **L3 zero deps externas novas** — sem `serde_json`
  em 03_infra (lab/parity converte via own dep).
- Domain struct (`QuerySummary`) não implementa
  `Serialize`; lab/parity usa `to_string()` ou
  manual JSON build.
- Selector parsing minimal: aceita Kind names existentes
  (10 variants ElementKind via `from_str`) + label
  syntax `<label>`. Outras formas vanilla
  (`heading.where(...)`, `figure.where(kind: image)`)
  → `InvalidSelector`. Documentado.
- Sem dependência circular: 03_infra usa 01_core; não
  invertido.

---

## Coerência arquitectónica

- Pattern paralelo a `pipeline.rs` (entry point para
  pipeline cristalino completo) — `query_helpers.rs` é
  entry point para pipeline + query.
- Reusa `eval_to_module_with_sink` (não duplicação).
- Reusa `introspect` (L1).
- L3 hosting é correcto: pipeline integration; sem I/O
  novo; complementa `pipeline.rs`.

---

## Tests

`#[cfg(test)] mod tests` com:

1. `parse_selector_kind_basico` — `"heading"` →
   `ParsedSelector::Kind(Heading)`.
2. `parse_selector_label_basico` — `"<fig-alfa>"` →
   `ParsedSelector::Label("fig-alfa")`.
3. `parse_selector_invalido` — `"weird.where()"` →
   `InvalidSelector`.
4. `parse_selector_kind_metadata` — `"metadata"` →
   `ParsedSelector::Kind(Metadata)`.
5. `summarize_kind_count` — corpus mínimo com 2
   headings → `count == 2`.
6. `summarize_label_match` — corpus com 1 label
   "fig-alfa" → `label_found == Some("fig-alfa")` +
   `count == 1`.
7. `summarize_metadata_values` — corpus com 3 metadata
   → `metadata_values.len() == 3`.

---

## Cross-impl considerations

- TagIntrospector field `metadata: MetadataStore` (P169);
  `query_metadata()` retorna `&[Value]`; cada Value
  com `Content(c).plain_text()` ou similar para output
  textual.
- `query_by_label(&Label)` retorna `Option<Location>`;
  `count` = `0` ou `1`.
- `query_by_kind(ElementKind)` retorna `Vec<Location>`;
  `count` = `len()`.

---

## Não-objectivos

`query_helpers.rs` não:

- Implementa subcomando CLI (Caminho A; deferred).
- Replica formato JSON vanilla literal (tarefa de lab/parity
  consumer).
- Estende `Selector` enum em L1 (P175 minimal mantém-se).
- Adiciona `Serialize` derive em tipos cristalinos
  (separação via domain struct).
- Materializa `Selector::Label` ou `Selector::Where`
  em L1 (futuro; out-of-scope P206).
