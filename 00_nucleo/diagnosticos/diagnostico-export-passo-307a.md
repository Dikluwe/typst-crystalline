# Diagnóstico — P307a: inventário factual de `export.rs`

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-307.md`
**Sub-passo**: P307a (diagnóstico-primeiro, **sem código tocado**)
**Magnitude**: M documental
**Tipo**: inventário empírico cross-codebase (cristalino vs vanilla)

---

## §1 — Métrica corrigida vs auditoria de paridade

A auditoria de paridade de 2026-05-19 declarou `export.rs` como
"9.856 LOC num único ficheiro" e classificou-o como "PDF export
monolítico — violação do espírito da arquitectura cristalina".

**Inventário factual corrige a interpretação**:

| Banda | LOC | % |
|---|---:|---:|
| Produção (linhas 1-2826) | **2.826** | 28,7% |
| Testes inline (linhas 2827-9856) | **7.029** | 71,3% |
| **Total** | 9.856 | 100% |

O ficheiro **não é** "10k LOC de PDF export". É **2.826 LOC de
produção + 7.029 LOC de testes inline**. A auditoria comparou contra
`typst-pdf` vanilla de 7.559 LOC (que provavelmente também exclui
testes — vanilla typst-pdf não tem `#[cfg(test) mod tests]` extenso
embutido).

**Comparação corrigida**:

| Métrica | Crystalline `export.rs` | Vanilla `typst-pdf/` |
|---|---:|---:|
| LOC produção | 2.826 | 7.559 |
| LOC testes (inline + integration) | 7.029 (inline) | ~280 (separado em `tests/`) |
| Ficheiros produção | 1 | ~30 |
| Ratio LOC/ficheiro | 2.826/1 | 7.559/30 ≈ 252 |

**Conclusão revista**: crystalline tem **menos código de produção
PDF** do que vanilla (37% de vanilla), mas concentrado num único
ficheiro. O problema **não é tamanho absoluto** — é **densidade
estrutural** e **ratio testes/produção**.

O ficheiro continua sendo um candidato legítimo à decomposição per
ADR-0037 Regra 2 (>800 linhas), mas a urgência é qualitativamente
menor do que a auditoria sugeria.

---

## §2 — Inventário por cluster (produção)

Bandas reais por marker `// ── ... ──` no ficheiro:

| Linhas | LOC | Cluster | Conteúdo principal |
|---:|---:|---|---|
| 1-62 | 62 | **API + headers** | `export_pdf` / `_with_font` / `_multifont` |
| 63-373 | **311** | **Imagens** | `detect_format`, `compress_zlib`, `process_png_for_pdf`, `scan_all_images`, `process_image_item`, `xobject_resources_for_page`, `build_jpeg_xobject`, `build_png_smask/rgb_xobject` |
| 374-763 | **390** | **Gradient Linear (P263)** | `scan_all_gradients`, `pattern_resources_for_page`, `compute_axial_coords`, `multispace_sample_stops` |
| 764-846 | 83 | Gradient CMYK helpers (P270.2) | `rgb_to_cmyk`, `multispace_sample_stops_linear_cmyk`, `_radial_cmyk` |
| 847-899 | 53 | Gradient relative (P273) | `resolve_relative`, `apply_parent_transform` |
| 900-963 | 64 | Adaptive N (P274) | `perceptual_distance_in_space`, `adaptive_n_for_stops` |
| 964-1408 | **445** | **Conic Coons (P272)** | `bezier_control_points_for_arc`, `compute_coons_patches_n_stops`, `emit_conic_coons_stream_rgb/cmyk`, `emit_function_dict` |
| 1409-2038 | **630** | **Builder** | `struct PdfBuilder`, `build_helvetica`, `build_cidfont`, `build_multifont`, `emit_gradient_objects`, `emit_image_xobjects`, `serialize` |
| 2039-2040 | 2 | (divider only) | — |
| 2041-2708 | **668** | **PageContext + emit (P281)** | `struct PageContext`, `struct FontScenario`, `emit_text_pdf`, `emit_glyph_pdf`, `line_rg_prefix`, `emit_stroke_paint`, `build_page_stream`, `emit_shape_path_local`, `emit_rounded_rect_ops`, `draw_item_local` |
| 2709-2826 | 118 | CIDFont helpers | `escape_pdf_string`, `collect_codepoints`, `collect_glyph_ids`, `map_chars_to_glyphs`, `widths_array`, `to_unicode_cmap`, `text_to_hex_string` |
| **Total** | **2.826** | | |

### §2.1 — Concentração

Cinco clusters com >300 LOC absorvem **2.444/2.826 = 86,5%** do
código de produção:

1. **PageContext + emit** (668) — maior cluster.
2. **Builder** (630) — orquestração L3.
3. **Conic Coons** (445) — feature isolada (P272).
4. **Gradient Linear** (390) — feature isolada (P263).
5. **Imagens** (311) — domínio coerente.

Os outros 5 clusters (≤120 LOC cada) somam 382 LOC (13,5%) e são
candidatos a fusão ou submódulo único de "gradient helpers".

---

## §3 — Inventário de testes inline (P307a §C)

`#[test]` count em `export.rs`: **230** (estimativa por grep
`fn (p[0-9]+|pdf_|builds_|emits_|renders_)`).

Distribuição por prefixo (top 8):

| Prefixo | Tests | Cluster vinculado |
|---|---:|---|
| `p270_*` | 69 | CMYK gradient (P270.2) |
| `p273_*` | 56 | Gradient relative + emit Group + clip |
| `p272_*` | 15 | Conic Coons |
| `p274_*` | 14 | Adaptive N |
| `p269_*` | 13 | Gradient focal |
| `p281_*` | 10 | PageContext unificado |
| `p263_*` | 8 | Gradient linear inicial |
| `pdf_*` | 7 | Estrutura PDF genérica (header, EOF, xref) |
| Outros (p265-p305) | 38 | dispersos por feature |

**Observação crítica**: ~175/230 testes (76%) são de gradients.
Gradients são **a feature mais testada** em L3 — sinal de
complexidade real, não de over-testing.

### §3.1 — Decomposição de testes

Per ADR-0037 Regra 5 + Ajuste C, testes E2E cross-cutting podem
viver em `tests.rs` dedicado mesmo se >800 linhas. **Decisão
proposta P307b**: separar `export.rs` testes em ficheiros por
cluster (`tests/imagens.rs`, `tests/gradients_linear.rs`, etc.)
**ou** manter num único `tests.rs` agregador.

A opção "agregador único" é defensável per Regra 5 Ajuste C
porque os testes cruzam features (e.g. P273 testa gradient
relative dentro de Group, P281 testa PageContext via builders
distintos). Decompor pode duplicar setup ou perder cobertura
cross-feature.

**Recomendação P307a**: testes em ficheiro único `export/tests.rs`
agregador (~7k LOC documentado como Regra 6 categoria
"infraestrutura de testes E2E").

---

## §4 — Helpers privados reusados cross-cluster

Identificação de funções privadas usadas por múltiplos clusters:

| Helper | Definido em | Usado em (clusters) |
|---|---|---|
| `compress_zlib` | Imagens (134) | Imagens (PNG), CIDFont (font streams), Conic Coons (Pattern Shading streams) |
| `scan_all_images` / `xobject_resources_for_page` | Imagens | Builder (`build_helvetica/cidfont/multifont`) |
| `scan_all_gradients` / `pattern_resources_for_page` | Gradient Linear | Builder |
| `multispace_sample_stops_*` | Gradient Linear + CMYK | Conic Coons (`emit_conic_coons_stream_*`) — **divergência conceptual notable** |
| `escape_pdf_string` | CIDFont helpers (2695) | Builder (text emit), PageContext (`emit_text_pdf`) |
| `build_jpeg_xobject` / `build_png_*_xobject` | Imagens | Builder.emit_image_xobjects |
| `emit_shape_path_local` / `emit_rounded_rect_ops` | PageContext | `draw_item_local` (mesmo cluster), Builder (Group clip masks) |

### §4.1 — Implicação para decomposição

A presença destes helpers cross-cluster significa que **submódulos
não podem ser totalmente independentes** — `super::` ou
`pub(crate)` cross-submódulo será necessário. Isto é coerente com
ADR-0037 Ajuste D ("coesão não implica isolamento") já validado em
L1.

**Anti-padrão a evitar**: extrair tudo para `helpers.rs` global —
seria divisão por tipo de função (helpers vs core) em vez de por
domínio. Per Regra 1, dividir por domínio.

---

## §5 — Estrutura proposta — refinada vs spec §4

A spec §4 propôs estrutura em `pdf/`, `fonts/`, `images/`, `geometry/`,
`collectors/`. **Após inventário factual**, ajusto:

```
03_infra/src/export/
    mod.rs                  # API pública (export_pdf, _with_font, _multifont) — ~62 LOC
                            # + struct PdfBuilder + dispatch (build_helvetica/cidfont/multifont)
    builder.rs              # Métodos build_* + emit_gradient_objects + emit_image_xobjects + serialize  — ~570 LOC
    images.rs               # Detect + zlib + PNG/JPEG XObjects + scan + dedup — ~311 LOC
    fonts.rs                # collect_codepoints + collect_glyph_ids + cmap + widths + escape — ~118 LOC
    gradients/
        mod.rs              # scan_all_gradients + pattern_resources_for_page + tipos partilhados — ~80 LOC
        linear.rs           # compute_axial_coords + multispace_sample_stops (RGB) — ~280 LOC
        radial.rs           # compute_radial_coords + multispace_sample_stops_radial — ~80 LOC
        cmyk.rs             # rgb_to_cmyk + multispace_sample_stops_linear_cmyk + radial_cmyk + emit_function_dict_cmyk — ~150 LOC
        relative.rs         # resolve_relative + apply_parent_transform — ~53 LOC
        adaptive.rs         # perceptual_distance_in_space + adaptive_n_for_stops — ~64 LOC
        conic.rs            # bezier + compute_coons_patches + emit_conic_coons_stream_rgb/cmyk — ~445 LOC
        function_dict.rs    # emit_function_dict — ~60 LOC
    stream/
        mod.rs              # struct PageContext + struct FontScenario — ~150 LOC
        page.rs             # build_page_stream — ~180 LOC
        text.rs             # emit_text_pdf + emit_glyph_pdf — ~120 LOC
        shape.rs            # emit_shape_path_local + emit_rounded_rect_ops + emit_stroke_paint + line_rg_prefix — ~150 LOC
        draw.rs             # draw_item_local — ~70 LOC
    tests.rs                # 230 testes — ~7029 LOC (Regra 6 categoria "infraestrutura de testes")
```

### §5.1 — Diferenças vs spec §4

| Spec §4 propunha | Diagnóstico revisto |
|---|---|
| `pdf/objects.rs`, `catalog.rs`, `page.rs`, `stream.rs`, `escape.rs` | Tudo em `builder.rs` (não há separação real — PdfBuilder agrega objects/catalog/page/xref). `escape_pdf_string` vai para `fonts.rs` (uso primário) ou `stream/text.rs` |
| `fonts/helvetica.rs`, `cidfont.rs`, `multifont.rs` | Os 3 caminhos são **métodos** de `PdfBuilder` em `builder.rs`. Decompor inverte a estrutura natural |
| `fonts/cmap.rs`, `widths.rs`, `descriptor.rs` | Agregar em `fonts.rs` único (118 LOC totais) |
| `images/jpeg.rs`, `dedup.rs` | `images.rs` único (311 LOC) — sub-divisão prematura |
| `geometry/coords.rs` | Eliminado — não há cluster coords genérico (cada gradient tem o seu) |
| `collectors/*.rs` | Eliminado — `collect_codepoints`/`collect_glyph_ids` ficam em `fonts.rs` |

**Princípio guia**: 1 ficheiro por cluster ≥80 LOC; agregar
clusters pequenos. Total estimado: **~15 ficheiros + tests.rs**
(vs ~25 da spec original).

### §5.2 — Submódulo `gradients/` é a peça maior

`gradients/` consome ~1.135 LOC (linear+radial+cmyk+relative+
adaptive+conic+function_dict). É o **maior submódulo proposto**.
Sub-divisão dentro do submódulo justifica-se por:
- Conic (445 LOC) sozinho excede limite Regra 2.
- Linear/Radial são paralelos arquitecturais distintos.
- CMYK helpers são cross-cutting entre Linear/Radial.

---

## §6 — Snapshot binário — corpus canónico (gerado em P307a.2)

Spec §5 P307b requer snapshot test binário que faça
`assert_eq!(export_pdf_pre, export_pdf_pos)` para N inputs canónicos.

**Corpus canónico final** (9 fixtures após feedback humano sobre
cobertura completa, ancorado ao L0
`00_nucleo/prompts/infra/export-fixtures.md`):

| # | Fixture | Cluster exercitado | Bytes |
|---:|---|---|---:|
| 01 | `01-markup-plain.typ` | API + Helvetica + escape_pdf_string | 941 |
| 02 | `02-markup-heading.typ` | Helvetica + emit_text | 1031 |
| 03 | `03-text-styling.typ` | Bold/italic via Styled → /F2/F3 | 1515 |
| 04 | `04-shapes.typ` | Shape kinds (rect, circle) + paint solid | 1149 |
| 05 | `05-gradient-linear.typ` | Linear scan + pattern_resources (P263) | 915 |
| 06 | `06-gradient-conic.typ` | Conic Coons (P272) | 1100 |
| 07 | `07-multi-feature.typ` | Integração multi-feature | 1527 |
| 08 | `08-image-jpeg.typ` | JPEG XObject + dedup + zlib | 1671 |
| 09 | `09-cidfont.typ` | CIDFont + Type0 + Identity-H | 559206 |

**Cobertura directa**: 6/9 clusters (2.562/2.826 LOC = 90,7%).
3 clusters menores (CMYK, relative, adaptive — 200 LOC totais)
ficam com cobertura indirecta via testes inline pré-existentes
em `export.rs::tests` que migram para `export/tests.rs` em
P307b.1.

Fixtures vivem em `03_infra/fixtures/p307b/sources/` + `reference/`.
Ver `MANIFEST.md` para tabela operacional bytes/md5 e
`00_nucleo/prompts/infra/export-fixtures.md` para L0 que ancora o
corpus.

Test harness em `03_infra/src/integration_tests.rs` (já existe):
adicionar módulo `mod p307b_snapshot` com `#[test]` por ficheiro:

```rust
#[test]
fn p307b_snapshot_markup_plain() {
    let pre  = include_bytes!("../fixtures/p307b/markup_plain.pdf");  // gerado em P307a
    let post = compile_and_export("markup/plain.typ");
    assert_eq!(post, pre);
}
```

Os bytes `.pdf` de referência são gerados em P307a (este passo)
correndo `cargo build --bin typst` pré-decomposição e guardando
output. P307b valida que pós-decomposição os bytes são idênticos.

### §6.1 — Riscos do snapshot binário

| Risco | Mitigação |
|---|---|
| PDF tem datas/IDs aleatórios | Verificar: `PdfBuilder` não usa SystemTime nem RNG. **Validar empiricamente** em P307a (gerar 2× o mesmo PDF, comparar bytes). |
| Mudança de versão Rust afecta floats | Pin à mesma toolchain em `rust-toolchain.toml`. |
| Compress_zlib varia | `flate2` é determinístico em mesma versão. |

**Pré-requisito P307b**: validar empiricamente em P307a que
`export_pdf(doc) == export_pdf(doc)` em 2 invocações consecutivas
sobre o mesmo input. Se sim, snapshot é fiável.

---

## §7 — Granularidade de P307b — recomendação

Spec §8 decisão 6 pediu recomendação sobre granularidade.

**Recomendação P307a**: **decompor P307b em 3 sub-movimentações
sequenciais**, não um único passo monolítico:

### P307b.1 — Extracção de submódulos por cluster grande (1 PR equivalente)

Movimenta para ficheiros novos:
- `images.rs`
- `fonts.rs`
- `gradients/{mod,linear,radial,cmyk,relative,adaptive,conic,function_dict}.rs`
- `stream/{mod,page,text,shape,draw}.rs`
- `builder.rs`
- `mod.rs` (ex-`export.rs` reduzido)
- `tests.rs` (move o `#[cfg(test) mod tests]` actual)

Tudo `pub(super)` ou `pub(crate)`. Snapshot binário deve verde.

### P307b.2 — Reorganização interna de gradients/ (refino)

Pós-validação binária P307b.1, sub-dividir `gradients/conic.rs` se
ainda exceder 800 LOC (provável: 445 LOC inicial mas com docs +
imports cresce).

### P307b.3 — Hashes propagados (mecânico)

`crystalline-lint --fix-hashes` em tudo. Validação cargo test +
snapshot final.

**Justificação**: P307b.1 é o passo "principal" (5-10 ficheiros
movidos); P307b.2 é só se necessário; P307b.3 é trivial. Esta
sub-divisão evita PR gigante e permite revisão incremental.

---

## §8 — ADR-0098 — substituição textual → binário

Spec §3 propôs:

> **Invariante reforçado**: para um conjunto de inputs canónicos
> (ficheiros `.typ` em corpus), o output binário PDF de cada um
> permanece bit-exact pós-decomposição.

**Recomendação P307a**: aceitar a substituição como **versão
estendida** do invariante, não como deprecação.

ADR-0098 actual (pós-P281) preserva hash textual `66cb8ac3` de
`export.rs`. A semântica era: **"não mexer no emit"**. O hash
textual era o **proxy** da invariante real, que era observable
behaviour preservation.

Pós-P307:
- A invariante observável **continua** (PDF bytes preservados).
- O proxy (hash textual de `export.rs`) **deixa de fazer sentido**
  porque o ficheiro deixa de existir como entidade única.
- Novo proxy: snapshot binário do corpus canónico.

**Substituição é honesta**: melhora o invariante (proxy → directo)
e remove o falso positivo (hash textual mudaria com qualquer
refactor benigno).

ADR-0098 deve ser **anotada** (não substituída) com secção
"Evolução pós-P307: proxy textual → snapshot binário" que documenta
a transição.

---

## §9 — Decisões fixadas para P307b

Per spec §8, este diagnóstico fixa as 6 decisões críticas:

| # | Decisão crítica | Resposta P307a |
|---:|---|---|
| 1 | Aceitação substituição ADR-0098 textual → binário | **Aceite** — anotação na ADR-0098 (não nova ADR) + ADR-0100 nova para extensão L3 |
| 2 | Numeração ADR para extensão L3 | **ADR-0100** (próximo slot; ADR-0099 já existe) |
| 3 | Estrutura submódulos final | **Refinada** vs spec §4 (ver §5 acima): ~15 ficheiros + tests.rs; agrupados em `gradients/`, `stream/`, e ficheiros simples no topo (`builder.rs`, `images.rs`, `fonts.rs`) |
| 4 | Snapshot binário — corpus canónico | **7 ficheiros** cobrindo os 5 clusters grandes; 3 já existem em corpus (markup/plain, heading, cite-bibliography), 4 a criar com gradient.linear/conic/image. Validação determinismo em P307a antes de P307b |
| 5 | Hash naming convention | **Manter padrão actual** `@prompt-hash XXXXXX`; paths longos são aceitáveis. L0 prompt por submódulo conforme P307c |
| 6 | Granularidade P307b | **3 sub-movimentações sequenciais**: P307b.1 (extracção), P307b.2 (gradients refino opcional), P307b.3 (hashes mecânico). Cada uma com snapshot binário verde |

---

## §10 — Não-objectivos confirmados (P307a fecho)

P307a:
- **Não** toca código L1 nem L3.
- **Não** propõe novas dependências.
- **Não** decide sobre estilo de imports cross-submódulo (P307b
  resolve caso a caso).
- **Não** abre DEBT-46-L3 formal — recomenda-se fazê-lo só em
  P307b ou P307d para mapear o fim do processo.

---

## §11 — Critério de fecho de P307a

P307a fechado quando:

- [x] Inventário factual de produção (5 clusters > 300 LOC; 5
      clusters menores; helpers cross-cluster identificados).
- [x] Inventário factual de testes (230 testes; 76% gradient).
- [x] Estrutura de submódulos refinada (§5).
- [x] Corpus canónico de snapshot binário identificado (§6).
- [x] Granularidade P307b recomendada (§7).
- [x] Tratamento ADR-0098 fixado (§8).
- [x] Decisões críticas spec §8 todas respondidas (§9).
- [ ] **ADR-0100 redigida em status `PROPOSTO`**.
- [ ] **Anotação a ADR-0098 redigida**.
- [ ] **Aceitação humana das ADRs e da granularidade P307b.1/2/3**.

Os 3 últimos itens são acções subsequentes deste diagnóstico no
mesmo P307a.

---

## §12 — Próximas acções após P307a fechar

Depois de o humano aceitar este diagnóstico + ADRs P307a:

1. **P307b.1**: extracção de 15 submódulos via `git mv` + reorganização
   de imports. Magnitude L. Pré-requisito: corpus canónico gerado e
   determinismo validado.
2. **P307b.2** (opcional): sub-dividir `gradients/conic.rs` se exceder
   limite.
3. **P307b.3**: `crystalline-lint --fix-hashes` propagação.
4. **P307c**: L0 prompts por submódulo (substitui `infra/export.md`
   monolítico por ~15 prompts).
5. **P307d**: promoção ADR-0100 `PROPOSTO → EM VIGOR/IMPLEMENTADO`
   após validação empírica completa.

Cada sub-passo tem critério de fecho próprio (spec §5 P307).
