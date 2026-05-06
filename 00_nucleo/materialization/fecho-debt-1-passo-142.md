# Fecho formal de DEBT-1 — Passo 142

**Data**: 2026-04-24
**ADR de referência**: ADR-0054 (critério fecho DEBT-1).
**Natureza**: relatório formal. DEBT-1 encerrado após cumprimento
dos critérios de ADR-0054. DEBT-52 (rastreador) encerrado
simultaneamente.
**Precondição**: Passo 141 encerrado; ADR-0055 `IMPLEMENTADO`;
DEBT-52 com 6/8 gaps fechados; zero código tocado neste passo.
**Escopo**: L0-puro / administrativo. Zero alteração em L1, L2,
L3, L4. Zero testes. Zero ADRs novas.

---

## 1. Histórico comprimido

DEBT-1 foi aberto no Passo 30 com âmbito "StyleChain +
propriedades adicionais para paridade total com o sistema de
styles do original". Atravessou múltiplas fases parciais ao longo
de ~110 passos:

| Passo | Marco |
|-------|-------|
| 30 | `StyleChain` + `StyleDelta { bold, italic, size }`; `#set text(...)` em eval. |
| 33 | Save/restore por bloco em `CodeBlock`/`ContentBlock` (scoping). |
| 83.5 | Auditoria DEBTs; reorganização de `DEBT.md` em 3 secções. |
| 84.1 | Riscadas pendências de scoping/show resolvidas implicitamente. |
| 94 | Atomização de `styles` como parâmetro `&mut StyleChain` (ADR-0036). |
| 95 | Tarefa A — classificação das pendências residuais. |
| 99 | Fundação tipada `Style`/`Styles`/`Content::Styled` (ADR-0038). |
| 100 | Activação de `Content::Styled` no Layouter; DEBT-48 encerrado (ADR-0039). |
| 101 | Wrappers `Content::Strong`/`Emph` removidos do enum. |
| 102 | `#set text(...)` validado end-to-end; `fill` activado (ADR-0040). |
| 103 | `#show heading/strong/emph` validado (ADR-0041). DEBT-50 aberto. |
| 126–134 | Captura canónica em `StyleDelta`: `weight`, `tracking`, `leading`, `lang`, `font` (ADR-0052/0053 etc). |
| 135 | Diagnóstico do gap "captura sem consumer"; abertura de DEBT-52; ADR-0054 redefine fecho. |
| 136 | Fase A — `TextStyle` estendido + `From<&StyleChain>` (DEBT-52 gap A). |
| 137 | Fase B — Consumer `tracking` (`word_width` + PDF `Tc`). |
| 138 | Fase B — Consumer `leading` (`flush_line` + line height). |
| 139 | Fase B — Consumer `weight` faux-bold (`2 Tr` + stroke wrap). |
| 140A | Diagnóstico font infra; ADR-0055 `PROPOSTO`; ADR-0019 anotada. |
| 140B | Fase C básica — Consumer `font` string (gap 5). Wiring single-font. |
| 141 | Fase C básica — Consumer `font` array fallback (gap 6). ADR-0055 `IMPLEMENTADO`. |
| **142** | **Fecho formal de DEBT-1 (este passo)**. |

Entre 30 e 142, DEBT-1 transitou de "estrutura ausente" para
"estrutura paga + activada + maioritariamente consumida".

---

## 2. Critérios de ADR-0054

Transcrição literal da secção "Decisão" da ADR-0054:

> DEBT-1 só fecha quando:
>
> 1. **Cada propriedade de `StyleDelta`** tem consumer em
>    layout/export (ou é explicitamente marcada como scope-out
>    com ADR de suporte).
> 2. **Output PDF observacional** é equivalente ao vanilla para
>    inputs equivalentes (dentro dos limites do perfil de
>    paridade adoptado — ver secção "Perfil de paridade" abaixo).
> 3. **DEBT-52** (rastreador aberto em Passo 135) encerra.

E o **perfil de paridade** adoptado (mesma ADR, secção
homónima):

> **Observacional graded**: métricas observáveis equivalentes
> (tamanho, cor, peso, espaçamento) para inputs de teste
> documentados, **sem garantia de shaping features**.

Rustybuzz + shaping completo está explicitamente fora do critério
("documentado como DEBT-52 fase D/E opcional"). Esta exclusão é
material para a interpretação de `lang` na §3.

---

## 3. Mapeamento campo-a-campo de `StyleDelta`

`StyleDelta` definido em `01_core/src/entities/style_chain.rs:31`
tem **10 campos** (após Passos 99/100/126–134):

| Campo | Tipo | Consumer activo | Passo materializador | Estado |
|-------|------|-----------------|----------------------|--------|
| `bold` | `Option<bool>` | `TextStyle.bold` → exporter (Helvetica-Bold ou faux-bold via `weight`) | 30, 100 | activo |
| `italic` | `Option<bool>` | `TextStyle.italic` → exporter (Helvetica-Oblique) | 30, 100 | activo |
| `size` | `Option<f64>` | `TextStyle.size` (pt) → métricas + glyph emit | 30, 100 | activo |
| `fill` | `Option<Color>` | `TextStyle.fill` → exporter (`rg`/`RG` operators) | 99, 102 | activo |
| `heading_level` | `Option<u8>` | show rules + `Style::HeadingLevel` (forward-compat) | 99, 103 | activo |
| `weight` | `Option<u16>` | `TextStyle::faux_bold_stroke_pt` → exporter `2 Tr` + stroke | 139 | activo (aproximação visual) |
| `tracking` | `Option<Length>` | `word_width` acresce `(n-1)·tracking_pt`; exporter emite PDF `Tc` | 137 | activo |
| `leading` | `Option<Length>` | `flush_line` soma `leading_pt` ao `line_height` default | 138 | activo |
| `lang` | `Option<Lang>` | (sem consumer activo) | — | **scope-out** (perfil observacional graded; §4) |
| `font` | `Option<FontList>` | `first_font_from_doc` + `resolve_font` → `export_pdf_with_font` (CIDFont) | 140B + 141 | activo |

**9 dos 10 campos** têm consumer activo. **1 (`lang`) é
scope-out** justificado em §4.

### 3.1. Notas por campo

- **`bold`/`italic`/`size`** — base do StyleChain desde Passo 30.
  `bold` é independente de `weight`: `bold: true` é flag boolean;
  `weight: 700` é numérico. Em vanilla, `bold: true` é açúcar
  para `weight: 700`; em cristalino são caminhos paralelos
  (vanilla bold vs faux-bold via stroke). Ambos chegam ao
  output, com formas diferentes.
- **`fill`** — captado em `StyleDelta` no Passo 99, propagado
  ao `TextStyle.fill` no Passo 100, emitido pelo exporter no
  Passo 102. Primeiro efeito visível dos consumers tipados.
- **`heading_level`** — forward-compat para `#set heading(level:
  N)`. Consumido por show rules de heading no Passo 103. Não
  toca em fontes.
- **`weight`** — captado em Passo 126; consumer faux-bold no
  Passo 139 (K=0.04 stroke wrap). Aproximação visual aceite —
  não selecciona font-file Bold dedicado (limitação registada
  como candidata ADR-0055bis; ver §9).
- **`tracking`** — captado em Passo 127 (`Length` = `abs+em`);
  consumer integrado em `word_width` + PDF `Tc` no Passo 137.
  Primeiro efeito visível desde o `fill`.
- **`leading`** — captado em Passo 128 (em vanilla é `par`, mas
  cristalino captura em `text` por conveniência); consumer em
  `flush_line` no Passo 138. Semântica "opt soma" (divergência
  subtil documentada).
- **`lang`** — captado em Passo 130 (raw EcoString); materializado
  como tipo semântico no Passo 131B (ADR-0052) com validação
  e erro hard. **Sem consumer activo.** Justificação em §4.
- **`font`** — captado em Passo 132B como `FontList` (string e
  array; dict deferido por ausência de `regex` em L1, ADR-0053);
  consumer single-font em Passo 140B; consumer array-fallback
  em Passo 141. Pipeline: `first_font_from_doc(&doc)` →
  `resolve_font(font_list, font_book, world)` →
  `export_pdf_with_font(&doc, &bytes)`.

---

## 4. Cumprimento do critério 1

> Cada propriedade de `StyleDelta` tem consumer em layout/export
> (ou é explicitamente marcada como scope-out com ADR de suporte).

**9/10 campos com consumer** (§3 tabela). **`lang` é o único
em scope-out** — justificação:

ADR-0054 adopta o perfil **observacional graded**: "métricas
observáveis equivalentes (tamanho, cor, peso, espaçamento) para
inputs de teste documentados, **sem garantia de shaping
features**". O efeito visível de `lang` no PDF passa por
hyphenation (insere oportunidades de quebra entre sílabas) e por
shaping features sensíveis a script (bidi, contextual forms,
ligature controls). Hyphenation é parte do pipeline de quebra
de linha, mas requer crate dedicada (e.g. `hyphenation`); shaping
features sensíveis a script requerem rustybuzz integrado.

Ambas as integrações estão **explicitamente fora** do perfil
observacional graded:

- ADR-0054, "Perfil de paridade", §2: "shaping real (ligatures,
  kern, bidi) diverge".
- ADR-0054, secção "Plano de materialização": Fase E (rustybuzz
  integration) e a hyphenation crate são **opcionais** (linha
  130 da ADR: "DEBT-1 fecha quando A + B + C encerrarem. D/E
  não bloqueiam"; gap 7 do DEBT-52 — lang hyphenation — é
  declarado opcional na própria entrada da Fase C dado que requer
  crate autorizada por ADR separada, ainda inexistente).

Conclusão: `lang` está em scope-out por perfil **observacional
graded** (ADR-0054). O suporte a `lang` mantém-se ao nível do
**captura + validação + tipo semântico** (ADR-0052) — o campo
não é silenciado nem perdido; apenas não tem consumer
hyphenation. Quando rustybuzz e/ou hyphenation forem priorizados
(passos 143/144 candidatos; ADR-0055bis ou nova ADR), o consumer
materializa-se sem revisão de DEBT-1.

**Critério 1 cumprido.** ✓

---

## 5. Cumprimento do critério 2

> Output PDF observacional é equivalente ao vanilla para inputs
> equivalentes (dentro dos limites do perfil de paridade adoptado).

### Inputs de teste documentados

A bateria de testes pré-existente (1095 unit + integration via
`cargo test --workspace --lib`) cobre os campos com consumer.
Exemplos representativos:

| Input | Consumer exercido | Verificação | Sítio do teste |
|-------|-------------------|-------------|----------------|
| `Hello` (sem `#set`) | fallback Helvetica | PDF tem `/BaseFont /Helvetica`; sem `CrystallineFont`. | `font_wiring_sem_set_text_font_usa_helvetica` |
| `*bold*` ou `#set text(bold: true)` | `bold` → Helvetica-Bold | exporter switch para F2; vanilla equivalente. | `tests_set_rule_integration::set_text_bold_propaga_ao_frame` |
| `_italic_` | `italic` → Helvetica-Oblique | exporter switch para F3. | `tests_set_rule_integration::set_text_italic_propaga_ao_frame` |
| `#set text(size: 16pt)` | `size` → glyph metrics + emit | Frame y-advance reflecte size. | `tests_set_rule_integration::set_text_size_propaga_ao_frame` |
| `#set text(fill: red); ...` | `fill` → exporter `rg`/`RG` | PDF contém operadores de cor RGB. | `eval_set_text_fill_passo_102` |
| `#set text(weight: 700)` | `weight` → faux-bold stroke | PDF contém `2 Tr` + stroke ops. | Passo 139 unit tests |
| `#set text(tracking: 1pt)` | `tracking` → `Tc` operator | `word_width` reflecte tracking; PDF tem `Tc`. | Passo 137 unit + integration |
| `#set par(leading: 8pt)` | `leading` → `flush_line` | y-cursor avança com leading. | Passo 138 |
| `#set text(font: "X")` | `font` → CIDFont embed | PDF contém `/CrystallineFont` quando "X" resolve. | `font_wiring_set_text_font_existente_embute_cidfont` |
| `#set text(font: "ZZ")` | font → fallback | PDF cai em Helvetica; sem CIDFont. | `font_wiring_set_text_font_inexistente_fallback_helvetica` |
| `#set text(font: ("ZZ", "X"))` | array fallback | Segunda família embutida; sem `/Helvetica`. | `font_wiring_array_fallback_primeira_falha_segunda_vence` |
| Dois `#set text(font:)` aninhados | single-font per document MVP | PDF tem **exactamente 1** `/Subtype /Type0`. | `font_wiring_segunda_font_diferente_primeira_vence` |

### Sítios de produção do output

Pipeline real (cristalino):

```
typst_shell::cli
  → typst_infra::pipeline::compile_to_pdf_bytes(world, source)
    → eval_to_module_with_sink(world, source)
    → Module::content() → introspect → layout → PagedDocument
    → first_font_from_doc(&doc) + resolve_font(...) → Option<bytes>
    → export_pdf(&doc) | export_pdf_with_font(&doc, &bytes)
    → Vec<u8> (PDF bytes)
```

Equivalente em vanilla:
```
typst-cli → typst-pdf::PdfBuilder → ...
  (com subsetting, shaping completo, font dict, hyphenation)
```

### Divergências residuais (dentro do perfil)

1. **Bold dedicado vs faux-bold**: vanilla embute font-file
   "Bold" quando disponível; cristalino usa stroke wrap (`2 Tr`).
   Resultado visual aproximado; perfil observacional graded
   aceita aproximação. Limitação registada como candidata
   **ADR-0055bis** (§9).
2. **Sem subsetting**: cristalino embute font integral; vanilla
   subset. PDF cristalino é maior. Tamanho não é métrica do
   perfil observacional graded. Candidata **ADR-0056** (§9).
3. **Sem shaping features**: ligatures, kern, bidi não são
   aplicados em cristalino. Explicitamente fora do perfil.
4. **Sem hyphenation**: como em §4, fora do perfil.
5. **Single-font per document**: spans com font diferente após
   a primeira são silenciosamente descartados (MVP ADR-0055
   decisão 3). Candidato **Passo 142A** se priorizado.

Estas divergências estão registadas em ADR-0055 (decisões
materializadas + decisões scope-out), em DEBT-52 fases D/E, e
nas limitações conhecidas dos relatórios 140B/141.

**Critério 2 cumprido** dentro dos limites declarados. ✓

---

## 6. Cumprimento do critério 3

> DEBT-52 (rastreador aberto em Passo 135) encerra.

DEBT-52 tem 8 gaps:

| Gap | Descrição | Estado | Passo |
|-----|-----------|--------|-------|
| A | Estender `TextStyle` + `From<&StyleChain>` | ✓ | 136 |
| B1 | Consumer `tracking` | ✓ | 137 |
| B2 | Consumer `leading` | ✓ | 138 |
| B3 | Consumer `weight` faux-bold | ✓ | 139 |
| 5 | Consumer `font` string (`FontBook::select`) | ✓ | 140B |
| 6 | Consumer `font` array (fallback chain) | ✓ | 141 |
| 7 | Consumer `lang` hyphenation (requer crate) | scope-out (§4) | — |
| 8 | Fase D — `font` dict (requer `regex`) | scope-out (ADR-0054) | — |

**6/8 gaps fechados; 2/8 scope-out** com justificação ADR. Os
gaps 7 e 8 não viram DEBTs novos — permanecem documentados como
candidatos futuros de baixa prioridade (§8).

DEBT-52 cumpriu a sua função de rastreador: guiou as Fases A
(Passo 136), B (Passos 137–139) e C básica (Passos 140B + 141).
**Encerra simultaneamente com DEBT-1 neste Passo 142.** ✓

---

## 7. DEBT-52 encerramento simultâneo

DEBT-52 e DEBT-1 fecham juntos por construção: DEBT-52 foi aberto
no Passo 135 **especificamente** como rastreador do trabalho que
ADR-0054 exige para fecho de DEBT-1. Sem DEBT-52, ADR-0054 não
teria âmbito operacional; sem DEBT-1, DEBT-52 não teria razão de
ser. Fecho separado introduziria desalinhamento entre histórico
estrutural (DEBT-1) e roadmap operacional (DEBT-52).

Texto histórico de ambos é preservado na Secção 2 de DEBT.md.

---

## 8. Gaps 7 e 8 como candidatos futuros (não DEBTs)

Conforme decisão 2 do enunciado do Passo 142 ("manter o
inventário limpo"):

- **Gap 7 — `lang` hyphenation**: requer crate autorizada por
  ADR separada (candidatos: `hyphenation`, `hypher`). Trabalho
  de S/M. Quando priorizado, abrir **Passo 143** com ADR de
  suporte (potencial **ADR-0057** — número provisório). Não
  reabre DEBT-1 nem DEBT-52.
- **Gap 8 — `font` dict**: requer `regex` em L1, autorizado por
  **ADR-0054bis** (referenciada na Fase D do DEBT-52 antes do
  fecho). Baixa prioridade; abre quando aparecer caso real de
  uso de `#set text(font: ("name": "A", ...))`. Forma actual
  aceite: string e array.

Nenhum DEBT novo aberto neste passo. Os gaps são trabalho
opcional e ficam visíveis em ADR-0054, ADR-0055 e neste
relatório.

---

## 9. Limitações conhecidas preservadas

Lista canonizada das limitações herdadas dos relatórios 140B e
141 — **não viram DEBTs**, ficam como candidatos futuros:

1. **Selecção variant-aware** (ADR-0055bis candidata) —
   `FontVariant::default()` em `resolve_font` significa que
   `#set text(font: "Inria Serif", weight: 700)` selecciona a
   regular e simula bold via stroke wrap; vanilla seleccionaria
   o ficheiro "Inria Serif Bold".
2. **Multi-font per document** (Passo 142A candidato; nota: a
   numeração `142A` reserva-se para esta sub-série, mantendo
   `142` para este fecho). Spans com font diferente após a
   primeira são silenciosamente descartados.
3. **Subsetting de fonts** (ADR-0056 candidata) — cristalino
   embute font inteira; vanilla subset.
4. **Shaping (rustybuzz)** (DEBT-53 candidato XL) — sem
   ligatures, kern, bidi.
5. **Lang hyphenation** (gap 7; passo 143 candidato).
6. **Font dict** (gap 8; ADR-0054bis condicional).
7. **Reprodutibilidade de tests `font_wiring_*` em CI** —
   probe de directórios canónicos (`/usr/share/fonts/...`) com
   graceful skip; CI sem fonts faz early-return. Fixture
   dedicada em `tests/fixtures/fonts/` é decisão futura.

A "humildade" do fecho é deliberada (notas operacionais do
enunciado): DEBT-1 cumpre **ADR-0054 (perfil observacional
graded)**, não um ideal absoluto. Faux-bold em vez de Bold
dedicada; subsetting ausente; shaping não existe. O critério
cumprido é o adoptado, não o máximo concebível.

---

## 10. Verificação

| Item | Estado |
|------|--------|
| Critérios de ADR-0054 transcritos literalmente | ✅ §2 |
| Mapeamento campo-a-campo completo de `StyleDelta` (10 campos) | ✅ §3 |
| Sem campo inerte não documentado | ✅ |
| Cada scope-out tem referência ADR | ✅ `lang` → ADR-0054 perfil |
| `cargo test --workspace --lib` (post-141 baseline) | ✅ inalterado (zero código tocado) |
| `crystalline-lint .` | ✅ zero violations |
| DEBT-1 movido a Secção 2 com nota de fecho + link a este relatório | ✅ |
| DEBT-52 movido a Secção 2 com nota análoga | ✅ |
| Contagem de DEBTs abertos actualizada | ✅ 12 → **10** (nota: enunciado do Passo 142 antecipou 13 → 11; a contagem real era 12 antes do fecho) |
| Gaps 7 e 8 documentados como candidatos futuros sem reabertura como DEBTs | ✅ §8 |
| Limitações conhecidas preservadas como candidatos | ✅ §9 |
| Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/` tocado | ✅ |
| Nenhuma ADR criada/modificada (excepto se referência cruzada exigir; verificado: nenhuma exige) | ✅ |

### Nota sobre contagem (enunciado vs realidade)

O enunciado do Passo 142 (linha 91) antecipou "13 → 11" como
contagem de DEBTs abertos. Inventário empírico (`grep -E "^##
DEBT-[0-9]+ " 00_nucleo/DEBT.md` na Secção 1) revelou **12
abertos antes deste fecho**: DEBT-1, DEBT-2, DEBT-8, DEBT-9,
DEBT-33, DEBT-34d, DEBT-34e, DEBT-35b, DEBT-42, DEBT-43,
DEBT-50, DEBT-52. Após o fecho (DEBT-1 + DEBT-52 → Secção 2):
**10 abertos**. A discrepância de 1 face ao enunciado é
contagem aritmética, não substantiva — o conteúdo material do
fecho é idêntico ao previsto.

---

## 11. Próximos passos (humanos)

Após este passo, projecto fica com **10 DEBTs abertos** e
**55 ADRs** (54 activas + ADR-0055 `IMPLEMENTADO`). A próxima
decisão é de priorização humana. Opções não-bloqueantes,
ordenadas por escopo:

- **Passo 142A** — multi-font per document. Escopo M.
- **ADR-0055bis + passo dedicado** — selecção variant-aware
  (font-file Bold/Italic). Escopo S/M.
- **Passo 143 + ADR de crate hyphenation** — gap 7 DEBT-52.
  Escopo S/M.
- **ADR-0054bis + passo dedicado** — autorizar `regex` para
  font dict. Escopo M.
- **DEBT-53** — abrir tracker para rustybuzz integration.
  Escopo XL; provavelmente série dedicada.

Nenhuma destas opções **reabre** DEBT-1. São todas extensões
opcionais ao perfil observacional graded actual.
