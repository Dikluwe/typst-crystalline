# typst-passo-281 — Text+Glyph+Line emit em Group via unificação `PageContext` (β-completa)

**Magnitude**: M com código (cap LOC L3 hard 220 / soft 170; cap testes hard 80 / soft 50). Net LOC esperado **negativo ou neutro** — consolidação dos 3 stream-builders compensa adições.
**Cluster**: Cluster Gradient residual / Render real Groups / Refactor arquitectural L3.
**Origem**: relatório P279 §5 pendência específica `P280.X-bis-text-emit-em-group-3-font-scenarios`; relatório P280 §6 "agora desbloqueado porque `collect_codepoints` + `collect_glyph_ids` já recursam"; humano confirma β-completa 2026-05-18 após análise dos 3 critérios literais (atomicidade, manutenção futura, performance bit-exact).
**Tipo**: passo principal P281 — unificação dos 3 stream-builders (`build_page_stream_type1/cidfont/multifont`) num pipeline central `build_page_stream` + `draw_item_local` com despacho via `PageContext { font_scenario: FontScenario, ptr_to_idx, img_refs }`. Fix funcional Text/Glyph/Line em Group entrega-se como **consequência natural** da unificação.
**Sequência**: P276 (DEBT-35b OBSOLETED) → P277 (DEBT-33 CLOSED) → P278 (cleanup; 1 sub-op reformulada) → P279 (Image em Group; narrow) → P280 (auditoria walkers; estabilização) → **P281 (unificação β-completa + Text/Glyph/Line fix)**.
**Estratégia decidida**: opção β-completa fixada **antes da Fase A** com base em análise dos 3 critérios em conversa. Fase A confirma inventário factual + dimensiona LOC; **não** re-discute α/β/γ. NÃO criar ADR nova per anti-padrão over-formalização — sub-padrão "Agregador de contexto em L3" N=1 inaugural fica abaixo do limiar de formalização. L0 `infra/export.md` documenta arquitectura.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L3 (zero alteração L1); testes-primeiro; bit-exact preservation **obrigatória** para casos pré-existentes.

2. **ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-p281-unificacao-stream-builders.md` imutável. **35º consumo** (continuação P280 N=39; 34º consumo).

3. **NÃO criar ADRs novas** — decisão arquitectural já tomada em conversa; tipos `PageContext` + `FontScenario` ficam `pub(crate)` em L3; L0 documenta. Anti-padrão over-formalização P273.17 preserved (sub-padrão N=1 inaugural não merece ADR).

4. **ADR-0029 pureza física L1** preserved absoluto — passo é puramente L3 `03_infra/src/export.rs`. Zero alteração L1.

5. **ADR-0033 paridade vanilla** parcial — o cristalino tem 3 caminhos (Type1/CidFont/Multifont); vanilla tem arquitectura própria. Paridade observacional (bytes PDF) é o critério, não paridade estrutural de código. Verificar em Fase A.

6. **ADR-0054 graded "menor mudança suficiente"** — neste passo, "menor mudança suficiente para alcançar manutenção futura + atomicidade + performance" é β-completa. Trade-off explicitado em conversa.

7. **Crystalline-lint zero violations** obrigatório. L0 `infra/export.md` actualizado; hash propagado.

8. **Tests workspace 2615 → 2640-2660 esperado** (~15-25 testes novos: Text/Glyph/Line em Group × 3 scenarios + casos nested + sanidade unificação).

9. **Sub-padrões aplicados / esperados**:
   - **Agregador de contexto em L3** N=1 inaugural (precedente conceptual ADR-0044 `Engine<'a>` em L1).
   - **Extract helper de replicação inline** N=4 → **N=5 cumulativo** (consolidação cross-funcional dos 3 stream-builders).
   - **Render real Groups** N=2 → **N=3 cumulativo** (P273.13 Shape + P279 Image + P281 Text/Glyph/Line).
   - **Refactor preservando bit-exact** N=1 (refactor estrutural com testes de regressão garantindo paridade output).
   - Disciplina anti-over-formalização preservada: nenhum dos 3 acima formaliza ADR.

10. **Caps documentais** (ADR-0094 Pattern 1):
    - Diagnóstico Fase A: hard 800 / soft 600.
    - Relatório consolidado: hard 1200 / soft 900.

11. **Sub-padrão "Decisão arquitectural fixada antes de Fase A"** — diferente de P277/P279 que deixaram opções abertas na Fase A. P281 fixa β-completa antes; Fase A apenas inventaria factualmente. Pattern N=1 inaugural.

---

## §1 — Sub-passo P281.A — Fase A diagnóstico empírico

Produz `00_nucleo/diagnosticos/diagnostico-p281-unificacao-stream-builders.md`.

**Foco**: factual, não decisional. Decisão arquitectural já fixada em §0; Fase A confirma inventário + dimensiona caps.

### §A.1 — Inventário dos 3 stream-builders actuais

Para cada um (`build_page_stream_type1`, `build_page_stream_cidfont`, `build_page_stream_multifont`):

```bash
# Localizar e inspeccionar
rg -n -A 80 "fn build_page_stream_type1" 03_infra/src/export.rs
rg -n -A 80 "fn build_page_stream_cidfont" 03_infra/src/export.rs
rg -n -A 80 "fn build_page_stream_multifont" 03_infra/src/export.rs
```

**Output §A.1** — tabela linha-por-linha:

| Linha (Type1) | Linha (CidFont) | Linha (Multifont) | O que faz | Idêntico? |
|---|---|---|---|---|
| ... | ... | ... | BT | sim |
| ... | ... | ... | Td (cursor) | sim |
| ... | ... | ... | Tf (font select) | **divergente** |
| ... | ... | ... | Tj (text emit) | **divergente** |
| ... | ... | ... | ET | sim |
| ... | ... | ... | Line/Shape/Image/Group arms | sim (via `draw_item_local`) |

Confirmar literal: divergência só em **fonte selection (Tf) + text emit (Tj)** para arms Text/Glyph; tudo o resto é compartilhável.

### §A.2 — Parâmetros que cada scenario precisa

Inventário exaustivo dos params específicos:

| Scenario | Params específicos para Text | Params específicos para Glyph |
|---|---|---|
| Type1 | (apenas `style.bold/italic` que vem do FrameItem) | (não emite — silently ignored) |
| CidFont | `char_to_gid: &HashMap<char, u16>` | (glyph_id directo do FrameItem) |
| Multifont | `font_maps: &[HashMap<char, u16>]` + `font_lookup: &HashMap<&FontList, usize>` | idem para glyph + selecção de fonte |

Params compartilhados (já consolidados em `draw_item_local` desde P279):
- `ptr_to_idx: &HashMap<usize, usize>` (Image XObject lookup).
- `img_refs: &[ImageRef]` (Image XObject names).

### §A.3 — Estrutura proposta de `PageContext`

Fixar literal a forma do agregador (decisão arquitectural já tomada; Fase A confirma forma exacta):

```rust
pub(crate) enum FontScenario<'a> {
    Type1,
    CidFont {
        char_to_gid: &'a HashMap<char, u16>,
    },
    Multifont {
        font_maps:   &'a [HashMap<char, u16>],
        font_lookup: &'a HashMap<&'a FontList, usize>,
    },
}

pub(crate) struct PageContext<'a> {
    pub ptr_to_idx:    &'a HashMap<usize, usize>,
    pub img_refs:      &'a [ImageRef],
    pub font_scenario: FontScenario<'a>,
}
```

**Verificação Fase A**: confirmar que tipos referenciados (`ImageRef`, `FontList`, etc.) têm visibilidade compatível e que lifetimes funcionam (`'a` propagado é suficiente).

### §A.4 — Pipeline unificado proposto

```rust
// Top-level entry points (preserved):
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8> { ... }
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8> { ... }
pub fn export_pdf_multifont(doc: &PagedDocument, fonts: &[(FontList, Vec<u8>)]) -> Vec<u8> { ... }

// Cada entry-point constrói o seu PageContext + chama build_page_stream

// Unified stream builder (NEW):
fn build_page_stream(page: &Page, ctx: &PageContext) -> String { ... }

// Unified recursive helper (existing, refactored):
fn draw_item_local(item: &FrameItem, ops: &mut String, ctx: &PageContext) { ... }
```

**Decisão Fase A**: validar que esta estrutura é factualmente viável (lifetimes; visibility; thread-safety).

### §A.5 — Casos de teste planeados

**Regressão bit-exact (críticos; ~5-8 testes)**:

- `p281_helvetica_top_level_preserved` — PDF identico ao baseline antes da unificação.
- `p281_cidfont_top_level_preserved` — Identity-H stream identico.
- `p281_multifont_top_level_preserved` — N fonts dispatch identico.
- `p281_image_top_level_preserved` — P279 fix preserved.
- `p281_image_em_group_preserved` — P279 cobertura preserved.
- `p281_documento_complexo_helvetica` — corpus existente PDF byte-byte.
- `p281_documento_complexo_cidfont` — idem CIDFont.
- `p281_documento_complexo_multifont` — idem multifont.

**Novos (cobertura fix funcional; ~10-15 testes)**:

- `p281_text_em_group_helvetica` — Text Latin-1 dentro de Group rotacionado emite no PDF.
- `p281_text_em_group_cidfont` — Text Unicode dentro de Group emite hex glyph IDs.
- `p281_text_em_group_multifont` — Text com `style.font` dentro de Group emite `/F{i+1}` correcto.
- `p281_text_unicode_em_group_cidfont_nao_perde_chars` — chars em Group estão no `char_to_gid` (regressão directa do bug latente que P280 fixou em `collect_codepoints`).
- `p281_glyph_em_group_cidfont` — Glyph (delimitador matemático) dentro de Group emite hex glyph_id.
- `p281_glyph_em_group_multifont` — Glyph com fonte específica em Group selecciona `/F{i+1}`.
- `p281_glyph_em_group_helvetica_continua_ignorado` — Glyph em Group + Type1 continua a ser silently ignored (paridade pré-fix; não introduzir comportamento novo).
- `p281_line_em_group_emite_path_ops` — Line dentro de Group emite path ops correctos.
- `p281_text_em_group_aninhado` — Text dentro de Group dentro de Group (recursão).
- `p281_misturado_text_image_em_group` — Text+Image no mesmo Group.

**Estimativa total**: 15-23 testes. Cap testes hard 80 confortável.

### §A.6 — Estimativa LOC produção

Inventário factual:

| Componente | LOC adicionados | LOC removidos | Net |
|---|---|---|---|
| `PageContext` + `FontScenario` enum (definições + impl Debug) | ~25 | 0 | +25 |
| `build_page_stream` unificado (substitui 3) | ~60 | ~140 | **-80** |
| `draw_item_local` arms Text/Glyph/Line (substituem stubs P278) | ~50 | ~12 (stubs) | **+38** |
| 3 entry-points refactored para construir PageContext | ~30 | ~15 | +15 |
| Adapter helpers (`PageContext::type1()`, `PageContext::cidfont(...)`, `PageContext::multifont(...)`) | ~20 | 0 | +20 |
| **Total estimado** | **~185** | **~167** | **~+18** |

Cap LOC L3 hard 220 / soft 170 — net ~+18 confortável; total adicionado ~185 dentro do cap.

**Observação importante**: net LOC pode ser **negativo** se a Fase A confirmar mais duplicação inter-stream-builders do que estimado. Verificar empíricamente.

### §A.7 — Verificar paridade vanilla (sanity check)

Vanilla não tem arquitectura idêntica ao cristalino; comparação observacional via testes de paridade existentes. Confirmar que **nenhum** teste de paridade vanilla é tocado neste passo (refactor estrutural; output PDF bit-exact garantido pelos testes regressão §A.5).

### §A.8 — Gates de paragem (§política condição)

Disparam paragem antes de §C:

1. **§A.1 detecta divergência inter-stream-builders além de Text+Glyph** (e.g. arms Line têm dispatch específico por scenario) — reformular para β-only-draw ou α.
2. **§A.2 revela params adicionais não previstos** — expandir `FontScenario` enum; verificar cap LOC ainda viável.
3. **§A.3 lifetimes ou visibility falham compilação inicial** — refactor estrutural não trivial; pausar para humano.
4. **§A.6 estimativa LOC excede hard 220** — reformular (e.g. extrair sub-op de Line para passo futuro).
5. **Cap doc Fase A hard 800 ameaçado** — reformular para diagnóstico mais conciso.
6. **Tests workspace ≠ 2615 baseline P280** — regressão pré-existente; investigar.

---

## §2 — Sub-passo P281.B — Anotação cumulativa (condicional)

**Default**: §2 não aplicado — refactor sem decisão arquitectural ADR-formalizada.

Anotação cumulativa **só dispara** se Fase A revelar:
- Aplicação concreta de ADR existente (e.g. ADR-0094 Pattern 1 cap LOC com estouro registado significativo) — provável; registar em §C.6 relatório §5.
- Sub-padrão N≥3-4 não-formalizado com aplicação nova — sim, "Extract helper de replicação inline" N=4 → N=5 (relatório §4); não formalizar ADR per Opção A consistente.

**Sem anotação ADR concreta esperada**. §2 do relatório regista: "B sub-passo não aplicado — refactor sem decisão ADR nova; sub-padrões emergentes ficam §5 sem formalização."

---

## §3 — Sub-passo P281.C — Materialização (testes-primeiro + refactor)

### §C.1 — L0 `prompts/infra/export.md` (Protocolo de Nucleação)

Reescrever secções afectadas:

**Substituir** secção "Helpers Internos" — agora documenta apenas helpers verdadeiros (não os 3 stream-builders separados):

```markdown
## Helpers Internos (pub(crate))

| Função | Responsabilidade |
|--------|-----------------|
| ... helpers pré-existentes ... | ... |
| `build_page_stream(page, ctx)` | Stream BT/ET unificado para qualquer font scenario |
| `draw_item_local(item, ops, ctx)` | Emit recursivo de items em Group, despachado por scenario |
```

**Adicionar** secção nova "Pipeline unificado (P281)":

```markdown
## Secção: Pipeline unificado de stream-building (P281)

Os 3 caminhos de exportação (`export_pdf` Helvetica, `export_pdf_with_font`
CidFont, `export_pdf_multifont` Multifont) **partilham** o mesmo
`build_page_stream` interno, despachado por `PageContext { font_scenario }`.

### `PageContext` e `FontScenario`

[código literal §A.3]

### Pipeline

1. Entry-point top-level (`export_pdf*`) constrói o `PageContext`
   apropriado (Type1 / CidFont / Multifont) com refs aos resources
   pré-computados (image XObject map, char_to_gid map se aplicável,
   font maps + lookup se multifont).
2. Para cada página, chama `build_page_stream(page, &ctx) -> String`.
3. `build_page_stream` itera `page.items`; para cada item, chama
   `draw_item_local(item, &mut ops, &ctx)` que dispatcha por
   `FrameItem` variant + `FontScenario` quando necessário (Text/Glyph).
4. Arm `Group` em `draw_item_local` recurse com mesmo `ctx`.

### Invariante arquitectural (P281)

Single source of truth para emit PDF — modificações futuras a arms
de `FrameItem` editam **um sítio** (`draw_item_local`); modificações
a setup top-level editam **um sítio** (`build_page_stream`).
Adicionar scenario novo (e.g. PDF/A) é uma variante nova em
`FontScenario` enum.

### Histórico

- P273.13: render real de Shape em Group inaugurou recursão em
  `draw_item_local`.
- P279: Image em Group fix (narrow scope) preserved `draw_item_local`.
- P280: auditoria walkers; `collect_codepoints` + `collect_glyph_ids`
  desbloqueados para recursão Group.
- **P281**: unificação β-completa; Text/Glyph/Line em Group fix
  como consequência.
```

Hash propagado via `crystalline-lint --fix-hashes .`.

### §C.2 — Testes-primeiro

Adicionar testes da §A.5 ao módulo de testes. **Ordem importa**:

1. **Primeiro** os testes regressão bit-exact (~8 testes). **Devem passar** com o código actual (baseline P280). Estes são as guardas de paridade output.
2. **Depois** os testes funcionais novos (~10-15). **Devem falhar** com o código actual (P278 sub-op 3 stubs no-op para Text/Glyph/Line). Inversão é a evidência factual do bug.

### §C.3 — Implementar `PageContext` + `FontScenario`

Adicionar tipos `pub(crate)` em `03_infra/src/export.rs` conforme §A.3. Includes constructors helpers:

```rust
impl<'a> PageContext<'a> {
    pub(crate) fn type1(ptr_to_idx: &'a HashMap<usize, usize>, img_refs: &'a [ImageRef]) -> Self { ... }
    pub(crate) fn cidfont(...) -> Self { ... }
    pub(crate) fn multifont(...) -> Self { ... }
}
```

### §C.4 — Refactor `draw_item_local` para usar `PageContext`

Signature actualizada:

```rust
fn draw_item_local(item: &FrameItem, ops: &mut String, ctx: &PageContext)
```

Arms novos para Text/Glyph/Line (substituem stubs P278 sub-op 3); arm Image preserved bit-exact da P279; arm Shape preserved bit-exact da P273.13; arm Group recurse com `ctx` partilhado.

Dispatch interno via `match ctx.font_scenario` para Text/Glyph emit.

### §C.5 — Implementar `build_page_stream` unificado

Substituir as 3 funções `build_page_stream_type1/cidfont/multifont`. Pipeline:

```rust
fn build_page_stream(page: &Page, ctx: &PageContext) -> String {
    let mut ops = String::new();
    for item in &page.items {
        draw_item_local(item, &mut ops, ctx);
    }
    ops
}
```

### §C.6 — Refactor 3 entry-points top-level

`export_pdf`, `export_pdf_with_font`, `export_pdf_multifont` actualizados para:

1. Pré-computar resources (image map, char_to_gid, font maps) como antes.
2. Construir `PageContext` via helper apropriado.
3. Chamar `build_page_stream(page, &ctx)` em vez do helper específico antigo.

### §C.7 — Validação final

```bash
cargo test --workspace 2>&1 | grep "test result"
# Esperado: ~2640-2660 passed (2615 baseline + 15-25 P281)

cargo run -p crystalline-lint --quiet
# Esperado: ✓ No violations found

# Validação bit-exact:
# Os testes de regressão devem confirmar que PDFs gerados pré e pós-P281
# para os mesmos inputs são byte-byte idênticos.
```

### §C.8 — Actualização DEBT.md

**Não é fecho de DEBT numerado**. Cabeçalho cumulativo recebe linha:

```markdown
> **Passo 281 (2026-05-XX)**: unificação β-completa dos 3
> stream-builders em `build_page_stream + PageContext + FontScenario`.
> Text/Glyph/Line em Group fix como consequência natural (pendência
> `P280.X-bis-text-emit-em-group-3-font-scenarios` fechada). Sub-padrões
> emergentes: "Agregador de contexto em L3" N=1 inaugural; "Render real
> Groups" N=3 cumulativo; "Extract helper de replicação inline" N=5
> cumulativo; "Decisão arquitectural fixada antes de Fase A" N=1
> inaugural. Net LOC L3 ~[valor real]; tests 2615 → [valor real].
> Bit-exact preserved para todos os scenarios pré-existentes.
```

**Pendências fechadas neste passo**:
- `P280.X-bis-text-emit-em-group-3-font-scenarios` (Text+Glyph em Group; M-magnitude).
- `P280.X-bis-line-emit-em-group` (Line em Group; XS — fechado como consequência).
- `P280.X-bis-glyph-emit-em-group` (Glyph em Group; S — fechado como consequência).

3 pendências derivadas fechadas em 1 passo via consolidação arquitectural.

### §C.9 — Relatório consolidado

Produz `/mnt/user-data/outputs/typst-passo-281-relatorio.md`. Estrutura:

- §1 — Validação contra spec (tabela critérios §7).
- §2 — Resumo factual:
  - §2.1 — Unificação materializada (PageContext + FontScenario).
  - §2.2 — Text/Glyph/Line em Group fix funcional.
  - §2.3 — Bit-exact preserved verificado.
- §3 — Operações realizadas:
  - L0 `export.md` actualizado.
  - Tipos `PageContext` + `FontScenario` adicionados.
  - 3 stream-builders consolidados num.
  - `draw_item_local` arms Text/Glyph/Line implementados.
  - 3 entry-points refactored.
  - 15-23 testes adicionados.
  - DEBT.md cabeçalho actualizado.
- §4 — Sub-padrões emergentes (sem formalização ADR):
  - Agregador de contexto em L3 N=1.
  - Render real Groups N=3 cumulativo.
  - Extract helper de replicação inline N=5 cumulativo.
  - Decisão arquitectural fixada antes de Fase A N=1.
- §5 — Métricas (tabela pré/pós).
- §6 — Pendências fechadas (3 P280.X-bis em 1 passo).
- §7 — Próximos passos: P282+ decisão humana.
- §8 — Referências cross-passos.

---

## §4 — Caps e gates de protecção

- **LOC L3 produção net**: hard +30 / soft +20 (refactor consolidatório; net pode ser negativo).
- **LOC L3 produção total adicionado**: hard 220 / soft 170.
- **LOC testes**: hard 80 / soft 50.
- **Modificações `.rs`**: `03_infra/src/export.rs` apenas. Sem outras alterações L1/L2/L4.
- **Modificações L0**: `prompts/infra/export.md` (secção "Pipeline unificado" + reescrita "Helpers Internos"; hash propagado).
- **Modificações `DEBT.md`**: 1 linha cabeçalho.
- **Tests workspace**: 2615 → 2630-2660 (preserved bit-exact + novos).
- **Lint**: zero violations preserved.

---

## §5 — Sub-padrões esperados aplicados

- **Agregador de contexto em L3 análogo a Engine L1** — N=1 inaugural. Precedente conceptual ADR-0044 (`Engine<'a>` em L1) mas em camada diferente. Aguardar N≥3-4 reaplicações antes de considerar formalização.
- **Render real Groups** — N=2 → **N=3 cumulativo** (P273.13 Shape + P279 Image + P281 Text/Glyph/Line). Limiar formalização N≥3-4 atingido. **Decisão**: registar §4 relatório sem formalizar (Opção A consistente; anti-padrão over-formalização).
- **Extract helper de replicação inline** — N=4 → **N=5 cumulativo**. Continuação Opção A.
- **Decisão arquitectural fixada antes de Fase A** — N=1 inaugural. Diferencia de P277/P279 que deixaram opções na Fase A. Aguardar reaplicação.
- **Diagnóstico imutável** — N=39 → **N=40 cumulativo** (35º consumo).
- **Refactor preservando bit-exact** — N=1 inaugural (refactor estrutural cross-cutting com tests de regressão garantindo paridade output).
- **Pendência específica derivada-fecha-derivada** — N=1 (P279) → **N=2 cumulativo** (P281 fecha 3 pendências P280.X-bis simultaneamente).

---

## §6 — Workflow operacional

1. Utilizador upload literal `00_nucleo/DEBT.md` + `00_nucleo/prompts/infra/export.md` + **`03_infra/src/export.rs` completo** (ou pelo menos as 3 funções stream-builder + `draw_item_local` + entry-points top-level).
2. Claude Code executa Fase A:
   - Produz `typst-passo-281A-diagnostico.md` em `/mnt/user-data/outputs/`.
   - §A.1 inventário 3 stream-builders.
   - §A.2 params específicos per scenario.
   - §A.3 forma exacta de `PageContext` + `FontScenario`.
   - §A.4 pipeline unificado validado.
   - §A.5 testes planeados.
   - §A.6 estimativa LOC produção (refinada com inventário real).
   - §A.7 paridade vanilla sanity.
3. Utilizador valida Fase A.
4. Claude Code executa §C em ordem:
   - C.1 L0 update + `--fix-hashes`.
   - C.2 testes-primeiro (regressão + novos).
   - C.3 tipos `PageContext` + `FontScenario`.
   - C.4 refactor `draw_item_local`.
   - C.5 `build_page_stream` unificado.
   - C.6 entry-points refactored.
   - C.7 validação final.
   - C.8 DEBT.md cabeçalho.
   - C.9 relatório.
5. Utilizador valida relatório.
6. Próximo passo: P282+ decisão humana.

---

## §7 — Critério de fecho

P281 fecha quando:

- [ ] Fase A produzida; §A.1-A.7 preenchidos empíricamente.
- [ ] §A.8 gates não dispararam.
- [ ] L0 `export.md` actualizado; hash propagado.
- [ ] Testes regressão bit-exact passam pré-refactor (baseline P280).
- [ ] Testes funcionais novos falham pré-refactor (stubs P278).
- [ ] `PageContext` + `FontScenario` implementados.
- [ ] `build_page_stream` unificado implementado; 3 stream-builders específicos removidos.
- [ ] `draw_item_local` arms Text/Glyph/Line implementados; stubs P278 sub-op 3 substituídos.
- [ ] 3 entry-points top-level refactored.
- [ ] Testes funcionais novos passam pós-refactor.
- [ ] Testes regressão bit-exact continuam a passar (paridade preserved).
- [ ] DEBT.md cabeçalho com linha P281.
- [ ] Tests workspace 2615 → ≥2630.
- [ ] Lint zero violations.
- [ ] Cap LOC L3 hard 220 respeitado.
- [ ] 3 pendências `P280.X-bis-*` fechadas (Text/Glyph/Line em Group).
- [ ] Relatório consolidado §1-§8 completos.

P281 NÃO fecha se:

- §A.1 detecta divergência inter-stream-builders além de Text+Glyph (β-completa inviável; reformular).
- Cap LOC L3 hard 220 estourado.
- Regressão tests baseline 2615.
- Algum teste regressão bit-exact falha pós-refactor (paridade quebrada).
- Algum teste funcional novo falha após implementação.
- Bug nova introduzida (e.g. multifont dispatch incorrecto, font index off-by-one).

---

## §8 — Referências cross-passos

- **P273.13** — render real Shape em Group inaugurou recursão em `draw_item_local`.
- **P278 sub-op 3 reformulada** — origem das pendências P279.X-bis e P280.X-bis; deferimento intencional documentado.
- **P279** — Image em Group narrow scope; preserved aqui sem mudança bit-exact.
- **P280** — auditoria walkers; `collect_codepoints` + `collect_glyph_ids` fixos (P281 depende destes).
- **L0 `infra/export.md`** — actualizado neste passo (secção "Pipeline unificado").
- **L0 `infra/pipeline.md`** — referência ao dispatch font-aware top-level (preserved).
- **ADR-0027** — CIDFont + Identity-H (1 dos 3 scenarios).
- **ADR-0029** — Pureza física L1 (preserved absoluto).
- **ADR-0033** — Paridade vanilla (verificada via tests existentes; output bit-exact garantido).
- **ADR-0044** — `Engine<'a>` agregador em L1 (precedente conceptual; P281 é análogo em L3).
- **ADR-0054** — Critério fecho graded (menor mudança suficiente per objectivo declarado).
- **ADR-0055** — Font consumer single-font + multifont decisão 5 (preserved).
- **ADR-0085** — Diagnóstico imutável (35º consumo).
- **ADR-0094** — Meta-operacional specs (Pattern 1 cap LOC; aplicado neste passo).
- **Anti-padrão over-formalização P273.17 §0** — preserved (zero ADR nova; sub-padrões emergentes ficam §4 relatório).

---

## §9 — Notas de execução para Claude Code

- **Decisão arquitectural já fixada**: β-completa. Fase A inventaria; **não** re-discute α/β/γ.
- **Bit-exact é o critério mais crítico**: cada arm Text/Glyph/Line/Image/Shape/Group após refactor **deve** produzir bytes PDF idênticos aos da arquitectura pré-P281 para o mesmo input. Testes regressão são o guard.
- **Ordem testes-primeiro**:
  1. Escrever testes regressão (esperado passar baseline).
  2. Correr — confirmar passam.
  3. Escrever testes funcionais novos (esperado falhar baseline).
  4. Correr — confirmar falham.
  5. Refactor.
  6. Correr — confirmar **todos** passam.
- **NÃO unificar entry-points top-level** — `export_pdf`/`export_pdf_with_font`/`export_pdf_multifont` continuam separados (API pública estável; unificação top-level seria scope creep arquitectural fora deste passo).
- **`PageContext` é `pub(crate)`** — não exposto fora da crate. Mudanças futuras não quebram callers externos.
- **Match exaustivo em `FontScenario`** preserved — compilador força revisão se variant scenario novo for adicionado.
- **Match exaustivo em `FrameItem` arms de `draw_item_local`** preserved (P278/P279/P280 estabeleceu).
- **Anti-padrão a evitar**: NÃO criar ADR formal para `PageContext` neste passo. Sub-padrão "Agregador de contexto em L3" N=1 inaugural fica abaixo do limiar formalização per anti-padrão over-formalização P273.17.
- **Outputs**: 2 ficheiros em `/mnt/user-data/outputs/` (`typst-passo-281A-diagnostico.md` + `typst-passo-281-relatorio.md`).
- **Tempo estimado**: 150-240 min (refactor estrutural M; bit-exact validation crítica).
- **Confirmação visual final**: `rg "build_page_stream_" 03_infra/src/export.rs` retorna **zero matches** após refactor (as 3 variantes específicas foram consolidadas).

---

*Spec produzida em 2026-05-XX para unificação β-completa dos 3 stream-builders em `build_page_stream + PageContext + FontScenario`. Decisão arquitectural fixada antes de Fase A (em conversa) com base em análise factual contra 3 critérios literais: atomicidade, manutenção futura, performance bit-exact. 3 pendências P280.X-bis (Text/Glyph/Line em Group) fechadas simultaneamente como consequência da unificação. Sub-padrão "Agregador de contexto em L3" N=1 inaugural não formalizado per anti-padrão over-formalização P273.17. Net LOC estimado neutro ou ligeiramente positivo; cap hard 220 confortável. Bit-exact preservation é critério crítico — testes regressão garantem paridade output para todos os scenarios pré-existentes.*
