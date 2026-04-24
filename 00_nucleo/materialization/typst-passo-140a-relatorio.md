# Passo 140A — Relatório (diagnóstico font infra + ADR-0055 + ADR-0019 anotada)

**Data**: 2026-04-24
**Precondição**: Passo 139 encerrado; 1100 total tests; zero
violations; 54 ADRs activas; 12 DEBTs abertos (DEBT-52 com 4
gaps). Fase B completa.
**Natureza**: passo L0-puro. **Sem código**. **Sem testes**.
Quarta aplicação do padrão "diagnóstico + ADR" (após 131A,
132A, 135).
**ADR**: **ADR-0055** criada em `PROPOSTO`; **ADR-0019**
anotada com nota factual.

---

## Sumário

**Descoberta central**: a infra de PDF font embedding
**já está materializada** em cristalino. Roadmap 135
sobrestimou o trabalho de Fase C significativamente.

Três artefactos L0:
1. **Diagnóstico** `diagnostico-font-infra-passo-140a.md`
   (~360 linhas) com 9 secções factuais.
2. **ADR-0055** `font-consumer-cidfont.md` em `PROPOSTO`
   (~170 linhas).
3. **ADR-0019** (TTF+RustyBuzz) **anotada** com nota factual:
   `ttf-parser` integralmente implementado; `rustybuzz`
   declarado mas sem uso.

**Zero código tocado**. Tests estáveis em **1100**. Lint clean.

---

## 140A.1/A.2 — Inventário L1 + L3 descoberta

### Infra completa descoberta

| Componente | Localização | Status |
|------------|-------------|--------|
| `FontBook::select` | `01_core/src/entities/font_book.rs:183` | ✅ implementado |
| `Font(Vec<u8>)` runtime | `01_core/src/entities/world_types.rs:30` | ✅ |
| `World::font(index)` trait | `01_core/src/contracts/world.rs:55` | ✅ |
| `discover_fonts(paths)` | `03_infra/src/fonts.rs:51` | ✅ |
| `FontSlot::get()` lazy loader | `03_infra/src/fonts.rs:36` | ✅ |
| `SystemWorld::with_fonts` | `03_infra/src/world.rs:142` | ✅ |
| CLI `--font-path` + env `TYPST_FONT_PATHS` | Passos 122-123 | ✅ |
| `build_font_book(slots)` | `03_infra/src/fonts.rs:145` | ✅ |

---

## 140A.3 — PDF embedding (descoberta-chave)

### `build_cidfont` existe e é completo

`03_infra/src/export.rs:423` — **full CIDFont pipeline**:

- Type0 font com Identity-H encoding.
- CIDFontType2 + CIDSystemInfo.
- FontDescriptor + FontFile2 (TTF embutido).
- Widths array (`/W [...]`).
- ToUnicode CMap para texto copiável.
- `collect_codepoints` + `map_chars_to_glyphs(face, chars)`.
- Suporte para glyphs variantes (math + DEBT-9).

Entry points públicos:
- `export_pdf(doc)` — Helvetica fallback (default actual).
- `export_pdf_with_font(doc, font_data)` — **CIDFont embed**.

### Gap real

Pipeline em `03_infra/src/pipeline.rs:78` usa `export_pdf`
(fallback). **`TextStyle.font` é ignorado**.

Fechar esta lacuna = wiring, não refactor.

---

## 140A.4 — Vanilla referência

Vanilla `typst-pdf/` tem pipeline sofisticado:
- Subsetting via `subsetter` crate.
- CFF + TTF support.
- Multi-font per document.

Cristalino é **sub-conjunto válido** com trade-offs explícitos
documentados (ADR-0054 perfil "observacional graded"). PDFs
maiores que vanilla mas observacionalmente correctos.

---

## 140A.5 — ADR-0019 estado empírico

**Verificação**:
- `ttf-parser` (0.25): usado em `font_metrics.rs`, `fonts.rs`.
  **5+ ficheiros L3**.
- `rustybuzz` (0.20): declarado em Cargo.toml + 03_infra; **zero
  ficheiros usam**.

**ADR-0019 status revisado**: **parcialmente implementada**.
`ttf-parser` OK, `rustybuzz` intenção sem materialização.

**Acção tomada**: nota factual adicionada à ADR-0019
referenciando diagnóstico 140A. **Sem revogação** — ADR
permanece válida; rustybuzz fica como intenção futura
(candidato DEBT-53).

---

## 140A.6 — Crates

### Autorizações actuais

- L1: `ecow`, `rustc_hash`, `indexmap`, `thiserror`, `comemo`,
  `unicode_*`, `time`, `clap`.
- L3: `ttf-parser`, `rustybuzz`, `image`.
- Via linter `crystalline.toml [l1_allowed_external]` regula L1.

### Decisão ADR-0055

**Zero crates novas para Fase C básica**. Infra existente
suficiente. Se subsetting/shaping forem priorizados no futuro,
**ADR-0056 (subsetting)** ou **DEBT-53 (rustybuzz shaping)**
em passos dedicados.

---

## 140A.7 — Diagnóstico escrito

9 secções:
1. L1 domínio font runtime.
2. L3 descoberta de fontes.
3. L3 PDF embedding (descoberta-chave).
4. Vanilla referência.
5. ADR-0019 estado empírico.
6. Crates.
7. Roadmap proposto Fase C.
8. DEBTs adjacentes.
9. Resumo executivo.

---

## 140A.8 — ADR-0055 proposta

Status: **`PROPOSTO`**. 7 decisões numeradas:
1. Zero crates novas.
2. Pipeline consumer em L3/L4 (iterar doc → selectFirstFont → embed).
3. Single font per document (140B MVP).
4. Array fallback chain (141).
5. Multi-font per document (142 opcional).
6. Lang hyphenation (143 opcional; ADR-0055bis futura se
   crate autorizada).
7. rustybuzz mantém-se sem uso (shaping = DEBT-53 futuro).

4 alternativas em tabela. Plano de materialização detalhado
em 4 passos com estimativas.

---

## 140A.9 — ADR-0019 anotada (não revisão)

Adicionada nota factual ao final de ADR-0019:
- Estado empírico: ttf-parser OK, rustybuzz declarado sem uso.
- Acção: nota factual — **sem revogação**.
- Referência a diagnóstico 140A.

Precedente: ADR-0038 ganhou 5 notas; ADR-0019 ganha 1.

---

## 140A.10 — DEBTs

### DEBT-52 (rastreador)

Mapeamento ADR-0055 → gaps:
- Gap 5 (font string) → resolvido por **140B**.
- Gap 6 (font array) → resolvido por **141**.
- Gap 7 (lang hyphenation) → **143 opcional**.
- Gap 8 (font dict) → **ADR-0055bis futura**, não bloqueia.

### DEBT novo?

**Nenhum aberto neste passo**.

**Candidato DEBT-53** registado: "Shaping via rustybuzz real".
Escopo XL. Fora DEBT-1. Abertura quando priorizado.

---

## Roadmap de Fase C revisto

| Passo | Descrição | Tamanho | Cumulativo |
|------:|-----------|:-------:|:----------:|
| 140B | Wiring single-font | S, ~2h | 2h |
| 141 | Array fallback chain | XS, ~45min | ~3h |
| 142 | Multi-font per doc | M, ~3-5h (opcional) | ~7h |
| 143 | Lang hyphenation | M, ~2-3h (opcional) | ~10h |

**Fecho DEBT-1 básico** = 140B + 141 = **~3h**.

**Comparação com Roadmap 135**:
- 135 estimava Fase C em 4-5 passos, 6-10h.
- 140A revela: 2 passos, ~3h (versão básica).
- Diferença: infra existente não foi identificada em 135.

---

## Verificação

1. ✅ Diagnóstico com 9 secções factuais.
2. ✅ ADR-0055 proposta em PROPOSTO com 7 decisões.
3. ✅ ADR-0019 anotada; acção documentada.
4. ✅ Crates decisão: zero novas para Fase C básica.
5. ✅ Roadmap estimado: 2 passos + 3h.
6. ✅ Zero ficheiros L1/L2/L3/L4 tocados (git confirma).
7. ✅ `cargo test --workspace`: 1100 inalterado.
8. ✅ `crystalline-lint`: zero violations.

---

## Ficheiros produzidos

| Ficheiro | Natureza | Linhas |
|----------|----------|-------:|
| `00_nucleo/diagnosticos/diagnostico-font-infra-passo-140a.md` | L0 diagnóstico | ~360 |
| `00_nucleo/adr/typst-adr-0055-font-consumer-cidfont.md` | L0 ADR PROPOSTO | ~170 |
| `00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md` | L0 modificado (nota) | +25 |
| `00_nucleo/materialization/typst-passo-140a-relatorio.md` | L0 relatório | ~190 |

### Números finais

| Métrica | Antes (139) | Depois |
|---------|------:|-------:|
| L1 tests | 869 | 869 |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1100** | **1100** |
| Violations | 0 | 0 |
| ADRs activas | 54 | **55** (ADR-0055 PROPOSTO) + ADR-0019 anotada |
| DEBTs abertos | 12 | 12 |

---

## Lições

1. **Inventário empírico paga-se extraordinariamente**:
   Roadmap 135 estimou Fase C em 4-5 passos ~6-10h baseado em
   assunção "CIDFont não existe". Realidade: CIDFont
   implementado integralmente, gap é só wiring. **140A
   poupou 3-7h cumulativamente**.

2. **"Não adivinhar" é disciplina forte**: a mesma lição
   que 131A (Lang simpler than expected) e 132A (regex
   expensive for gain). Sempre inventário antes de estimar.

3. **ADR-0019 nota factual é valor colateral**: sem
   diagnóstico 140A, `rustybuzz` declarado-mas-unused
   passaria despercebido. Documentar facto (não
   revogar) preserva histórico da intenção + torna
   estado actual transparente.

4. **Quarta aplicação do padrão L0-puro**: 131A, 132A, 135,
   140A. Padrão reusado com confiança. ADR-0034 formalização
   do pattern continua candidato futuro (registado em 132A).

5. **Zero crates novas = zero autorização needed**: Fase C
   básica fecha DEBT-1 sem burocracia nova. A "ausência" é
   resultado honesto, não atalho.

6. **Single-font-per-document é MVP aceite**: simplificação
   consciente. Multi-font é extensão opcional (142), não
   pré-requisito. ADR-0054 perfil graded permite.

---

## Próximos passos

### 140B — Wiring single-font (S, ~2h)

Consumer `TextStyle.font` ao pipeline existente:
1. Iterar `PagedDocument` procurando primeiro font non-None.
2. `FontBook::select` + `World::font` → bytes.
3. Chamar `export_pdf_with_font` em vez de `export_pdf`.

### 141 — Array fallback chain (XS, ~45min)

Extensão de 140B: FontList iter até match.

### Fecho DEBT-1

Após 140B + 141. Gaps 7 + 8 fica como "post-DEBT-1" ou
opcionais.

### Candidatos futuros

- **ADR-0056** (subsetting) quando qualidade PDF for priorizada.
- **DEBT-53** (rustybuzz shaping) XL; série dedicada.
- **ADR-0055bis** se `hyphenation` crate autorizada.
