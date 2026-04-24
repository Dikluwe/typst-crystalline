# ⚖️ ADR-0055: Font consumer via pipeline CIDFont existente

**Status**: `IMPLEMENTADO`
**Data**: 2026-04-24
**Autor**: Humano + IA
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-font-infra-passo-140a.md`](../diagnosticos/diagnostico-font-infra-passo-140a.md)

---

## Contexto

Fase C do roadmap DEBT-1 (ADR-0054) precisa de consumer para
`text.font`. Diagnóstico 140A revelou que **infra CIDFont
embedding já está materializada** em `03_infra/src/export.rs`
via função `export_pdf_with_font(doc, font_data)`:
- Type0 + CIDFontType2.
- FontFile2 stream (TTF embutido).
- Widths array + ToUnicode CMap.
- Identity-H encoding.

Pipeline actual usa `export_pdf` (Helvetica fallback) por
default. Fase C reduz a **wiring** `TextStyle.font` ao
pipeline CIDFont existente.

## Decisão

1. **Zero crates novas**. `ttf-parser` + CIDFont existentes
   suficientes para paridade básica.

2. **Pipeline consumer em L3/L4**:
   - Em `compile_to_pdf_bytes` (ou equivalente), após layout:
     - Itera `PagedDocument` procurando primeiro `TextStyle.font`
       não-None.
     - Para cada `FontFamily.name`, tenta `FontBook::select(name,
       variant)`.
     - Se match: `World::font(index) → Option<Font(bytes)>` →
       passa para `export_pdf_with_font`.
     - Se não match: fallback `export_pdf` (Helvetica).

3. **Single font per document** (Passo 140B):
   - Primeira font encontrada é usada para o documento inteiro.
   - Outros spans com font diferente: ignorados (silent) ou
     warning.
   - Simplicidade sobre correção — **MVP**.

4. **Array fallback chain** (Passo 141):
   - Para `FontList` com N famílias, iterar por ordem;
     primeira que `FontBook::select` resolve vence.
   - Se nenhuma resolve, fallback Helvetica.

5. **Multi-font per document (Passo 142, opcional)**:
   - Extender `build_cidfont` para aceitar múltiplos faces.
   - Resource dict com `/F1 /F2 /F3 ...` dinâmico.
   - Fora do escopo mínimo para fechar DEBT-1.

6. **Lang hyphenation (Passo 143, opcional)**:
   - Crate `hyphenation` se autorizada via ADR complementar.
   - Fora do escopo mínimo; adiável.

7. **`rustybuzz` mantém-se declarado mas sem uso**. ADR-0019
   anotada como "parcialmente implementada". Shaping real
   (rustybuzz) é escopo futuro (candidato DEBT-53), **fora**
   DEBT-1.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Paridade total (subsetting + CFF) | Bytes PDF mínimos; quality alta | XL; crates novas (subsetter, CFF) |
| **Paridade básica CIDFont (escolhida)** | Zero crates novas; infra existe; fecha DEBT-1 | PDFs maiores (full-font embed, não subset) |
| Paridade mínima (só Helvetica fallback) | XS | Não fecha DEBT-1; sem consumer real |
| Crate `pdf-writer` + refactor | Mais limpo que PdfBuilder manual | Refactor grande; ADR nova |

**Escolha**: paridade básica usando CIDFont existente.

## Consequências

### Positivas

- **Gap 5 + 6 de DEBT-52 resolvidos** em 2 passos (~3h).
- **DEBT-1 pode fechar após 140B + 141** com gap 7
  (hyphenation) adiado/opcional.
- **Zero custo de autorização** de crates.
- **Infra CIDFont já validada** por testes existentes
  (`export_pdf_with_font` em `integration_tests.rs:620`).
- **Fase C mais curta que Fase B** (2 passos vs 4).

### Negativas

- **Single/multi font limite**: 140B single-per-document é
  simplificação visível. Multi-font (142) é opcional.
- **Sem subsetting**: PDFs maiores que vanilla. Aceite sob
  perfil "observacional graded" ADR-0054.
- **rustybuzz unused em deps**: cargo tree fica poluído. Aceite
  para futuro shaping.

### Neutras

- **ADR-0019 ganha nota factual**: clarificar que `ttf-parser`
  está integrado mas `rustybuzz` não. Não invalida ADR.
- **Helvetica fallback preservado**: código existente continua
  a funcionar quando nenhuma font é resolvida.

## Plano de materialização

### Passo 140B — Wiring single-font (Passo S, ~2h)

1. Em `03_infra/src/pipeline.rs` (ou `04_wiring/main.rs`):
   - Após `layout(...) → PagedDocument`.
   - Itera `doc.pages → items → FrameItem::Text.style.font`.
   - Primeiro `Some(FontList)` não-None encontrado é usado.
   - Para cada `FontFamily.name` na lista:
     - `font_book.select(name, variant) → Option<usize>`.
     - Se match: `world.font(index) → bytes` → break.
   - Se algum match: `export_pdf_with_font(doc, bytes)`.
   - Senão: `export_pdf(doc)` (Helvetica fallback).

2. Tests L3: passo a passo integração font_path → embed PDF.

3. DEBT-52 gap 5 marcado resolvido.

### Passo 141 — Array fallback chain (Passo XS, ~45min)

1. FontList tem múltiplas famílias. Em 140B só a primeira.
   Em 141 iterar todas até achar match.

2. Comportamento: `font: ("Inria Serif", "Arial", "sans-serif")`
   tenta cada em ordem.

3. DEBT-52 gap 6 resolvido.

### Passo 142 (opcional, M) — Multi-font per document

Adia fecho DEBT-1. Decisão futura se necessário para caso real
de uso.

### Passo 143 (opcional, M) — Lang hyphenation

Gap 7 de DEBT-52. Independente de font. Adia-se se não urgente.

### Fecho DEBT-1

Após **140B + 141**. Gap 7 + 8 podem ficar documentados como
"not-blocking" com issue tracker / DEBT-52 entry.

## Referências

- **ADR-0019** (ttf-parser + rustybuzz) — parcialmente
  implementada.
- **ADR-0033** (paridade funcional) — perfil graded.
- **ADR-0034** (diagnóstico obrigatório) — cumprido em 140A.
- **ADR-0053** (FontList) — tipo capturado pronto para consumo.
- **ADR-0054** (critério fecho DEBT-1) — gate.
- **DEBT-52** (consumer integral) — gaps 5-8.
- **Passo 140A** (diagnóstico) — motivação e findings.
- **Vanilla**: `lab/typst-original/crates/typst-pdf/` (referência
  para subsetting futuro).

## Futuros candidatos

- **ADR-0055bis** (se lang hyphenation): autoriza `hyphenation`
  ou `hypher` em L1/L3.
- **ADR-0056** (se subsetting): autoriza `subsetter` + refactor
  `build_cidfont` para subset.
- **DEBT-53** (shaping): rustybuzz integration — escopo XL.
- **ADR-0019 R1**: revisão de status quando shaping chegar.

## Materialização

- **Passo 140B** (2026-04-24) — wiring single-font (decisão 3).
  `compile_to_pdf_bytes` despacha para `export_pdf_with_font`
  quando a primeira família encontrada no `PagedDocument` resolve
  via `FontBook::select` + `world.font(index)`. Fallback Helvetica
  preservado quando nenhuma família resolve. Gap 5 de DEBT-52
  fechado.
- **Passo 141** (2026-04-24) — array fallback chain (decisão 4).
  `resolve_font` itera `font_list.as_slice()` em ordem; primeira
  família a completar `select` + `world.font` vence. Cenário
  patológico (índice stale) não curto-circuita — continua a
  tentar famílias seguintes. Gap 6 de DEBT-52 fechado. **Paridade
  básica da ADR completa.**

Decisões 5 (multi-font per document — Passo 142 opcional),
6 (lang hyphenation — Passo 143 opcional, gap 7 DEBT-52) e
7 (rustybuzz / shaping — DEBT-53 candidato XL) permanecem
scope-out conforme definido originalmente. Não bloqueiam o
fecho desta ADR.

Selecção variant-aware (font-file "Bold"/"Italic" dedicado)
permanece como limitação conhecida — `FontVariant::default()`
é usada em `resolve_font`, e `weight`/`style` continuam a ser
renderizados via faux-bold do Passo 139. **ADR-0055bis** é o
candidato natural se paridade avançada for priorizada.
