# Passo 146 — Multi-font per document (extensão voluntária pós-DEBT-1)

**Série**: 146 (passo **substantivo** L1 + L3 + L0; extensão
voluntária da decisão 5 de ADR-0055).
**Precondição**: Passo 144 encerrado; gap 7 (hyphenation)
fechado voluntariamente; 1104 tests; zero violations; 57 ADRs
(ADR-0057 `IMPLEMENTADO`); 10 DEBTs abertos; DEBT-1 + DEBT-52
fechados.

**Numeração**: passo **146**. Decisão deliberada de **não usar
142A** apesar das notas operacionais de 142, 143 e 145
referirem-no como tal:
- 142 já foi executado como passo de fecho administrativo de
  DEBT-1.
- Sufixo `A` na grelha actual significa **diagnóstico-primeiro
  associado ao mesmo número** (precedente: 131A, 132A, 140A —
  cada um foi diagnóstico para um trabalho substantivo
  distinto).
- 142A neste contexto seria ambíguo (associado a 142
  administrativo? a algum diagnóstico futuro?).
- 146 é o seguinte natural após 145, sem colisão com 144 (já
  executado).

Esta decisão de numeração é registada no relatório do passo
para preservar coerência da história.

**Natureza**: passo **substantivo**. Toca:
- L3 (`03_infra/src/`): pipeline expandido para multi-font;
  `build_cidfont` ou função vizinha estendida para múltiplos
  faces.
- L1 (`01_core/`): possível accessor novo se a iteração de
  `PagedDocument` exigir; sem mudança de domínio.
- L0 (prompts): spec do dispatch multi-font.
- ADR-0055: anotação pós-IMPLEMENTADO documentando que
  decisão 5 foi materializada.
- `DEBT-52` (encerrado): nova actualização ao histórico
  análoga às de DEBT-1.
- `Cargo.toml` / `crystalline.toml`: provavelmente nenhuma
  mudança (sem crates novas — confirmar em 146.1).

**ADRs aplicáveis**:
- **ADR-0055** (`IMPLEMENTADO`) — decisão 5 (multi-font per
  document) declarada explicitamente "fora do escopo mínimo
  para fechar DEBT-1" e "Passo 142, opcional". Este passo
  materializa-a. **ADR-0055 ganha anotação pós-IMPLEMENTADO**
  análoga à de ADR-0019 (anotada em 140A) — documenta
  factualmente a materialização de decisão 5 sem alterar
  status nem revisar.
- **ADR-0054** (perfil observacional graded) — multi-font
  está dentro do perfil; não há contradição.
- **ADR-0033** (paridade funcional) — output observacional:
  cada span de texto usa a font que o seu `TextStyle.font`
  declara.
- **ADR-0029** (pureza física L1) — preservada;
  alterações em L3.
- **ADR-0019** + nota factual de 140A — `ttf-parser` continua
  a ser a infra; `rustybuzz` continua sem uso.

---

## Contexto

Passo 140B materializou consumer single-font: primeira
`FontList` encontrada no documento é usada para o documento
inteiro; spans posteriores com font diferente são
**silenciosamente descartados** (caem no embedding da primeira).

`first_font_from_doc` itera `doc.pages → items →
FrameItem::Text.style.font` e devolve a primeira `FontList`
não-None. Comportamento MVP aceite por ADR-0055 decisão 3.

Passo 141 estendeu single-font com array fallback **dentro de
uma `FontList`**; multi-font per document continuou MVP.

Diferença material observável: documento com `*ênfase em
Inria Serif*` num parágrafo de Helvetica produz PDF com **só
Helvetica** (ou **só Inria Serif**, dependendo de qual o
eval encontra primeiro). Vanilla emite ambas as fonts.

**`build_cidfont` em `03_infra/src/export.rs:423`** assume
**uma** font, com nome interno `CrystallineFont`. Resource
dict tem entrada única (`/F4 <objref>` ou similar). Para
multi-font: iterar conjunto de fonts distintas no documento;
uma entrada de resource dict por font; emit do glyph escolhe
o `/Fn` correcto consoante o span.

**Pipeline actual** (`compile_to_pdf_bytes`):

```
doc → first_font_from_doc → resolve_font → Some(bytes)?
                                        → Yes: export_pdf_with_font(doc, bytes)
                                        → No:  export_pdf(doc)
```

**Pipeline target**:

```
doc → collect_fonts_from_doc → resolve_each → Vec<(FontId, bytes)>
    → export_pdf_multifont(doc, fonts_resolved)   [novo]
```

`export_pdf_with_font` permanece para compatibilidade;
single-font é caso particular de multi-font (uma entrada).

---

## Objectivo

Ao fim do passo:

1. **`collect_fonts_from_doc(doc) → Vec<FontList>`** em L3 (ou
   L1 se mais natural — confirmar em 146.1):
   - Itera `doc.pages → items → FrameItem::Text.style.font`.
   - Devolve **todas** as `FontList` distintas do documento,
     em ordem de primeira ocorrência.
   - Deduplica por valor de `FontList` (igualdade
     estrutural; primeira instância de cada valor único é
     mantida).

2. **`resolve_fonts(font_lists, font_book, world) → Vec<(FontList,
   Vec<u8>)>`** em L3:
   - Para cada `FontList`, aplica `resolve_font` (o helper
     do 141).
   - **Filtra** entries que não resolvem (silent drop análogo
     ao 140B).
   - Devolve par `(FontList, bytes)` para preservar a
     associação entre o input style e o output embed.

3. **`export_pdf_multifont(doc, fonts) → Vec<u8>`** em L3:
   - Constrói resource dict com **uma entrada por font**:
     `/CrystallineFont1`, `/CrystallineFont2`, ...
   - Cada entrada é Type0 + CIDFontType2 + FontFile2 (forma
     idêntica ao `build_cidfont` actual).
   - Emit do PDF: para cada `FrameItem::Text`, decide qual
     `/CrystallineFontN` usar com base no `style.font`
     (matching contra `Vec<(FontList, _)>`); operador `Tf` no
     content stream activa o font correcto.
   - Quando `style.font` é `None` ou `style.font` não tem
     match nas fonts resolvidas: fallback Helvetica
     (preserva comportamento existente do `export_pdf`).

4. **`compile_to_pdf_bytes` actualizado** para dispatch:
   - `collect_fonts_from_doc` + `resolve_fonts` → `Vec<(_,
     bytes)>`.
   - Se vec vazio → `export_pdf(doc)` (sem fonts, fallback).
   - Se vec.len() == 1 → `export_pdf_with_font(doc, &bytes)`
     **OU** `export_pdf_multifont(doc, vec![entry])` —
     decisão em 146.1 (preservar `export_pdf_with_font`
     como caso especial vs converger para multi-font).
   - Se vec.len() > 1 → `export_pdf_multifont(doc, vec)`.

5. **Tests**:
   - Unit: `collect_fonts_from_doc` em 4 cenários (vazio,
     uma font, duas distintas, duas iguais com ocorrências
     dispersas).
   - Unit: `resolve_fonts` em 3 cenários (todas resolvem,
     algumas resolvem, nenhuma resolve).
   - Integração L3: 3 cenários cobertos com `discover_any_system_fonts`
     + early-return:
     - Documento bilíngue com 2 fonts: PDF embute ambas
       (assert: 2 ocorrências de `/Subtype /Type0`).
     - Documento com 3 fonts (uma não disponível): PDF
       embute 2; spans da terceira caem em fallback.
     - Regressão: documento single-font produz output
       equivalente ao pré-146 (1 `/Subtype /Type0`).

6. **ADR-0055 anotada**:
   - Cabeçalho ganha linha:
     `**Anotação Passo 146**: decisão 5 (multi-font per
     document) materializada — ver
     `00_nucleo/materialization/typst-passo-146-relatorio.md`.`
   - Secção "Materialização" (introduzida em 141) ganha
     entrada nova para Passo 146.
   - **Status permanece `IMPLEMENTADO`**. Sem revisão.

7. **DEBT-52 actualizado** em Secção 2 (encerrados):
   actualização análoga à de 144 (DEBT-52 não reabre; só
   anotação ao histórico).

8. **README dos ADRs**: entrada P146 em "Passos-chave da
   história dos ADRs". Sem mudança na tabela "Estado por
   ADR" (ADR-0055 mantém status; sem ADR nova).

9. **Relatório do passo**.

Este passo **não**:

- Toca DEBT-1 (encerrado).
- Cria ADRs novas.
- Materializa selecção variant-aware (ADR-0055bis candidata).
- Materializa subsetting (ADR-0056 candidata).
- Materializa shaping (DEBT-53).
- Materializa gap 8 (font dict; ADR-0054bis).
- Adiciona crates novas (a confirmar em 146.1).
- Toca pipeline de hyphenation do 144 (independente).

---

## Decisões já tomadas

1. **Numeração 146** (não 142A). Decisão registada no
   relatório.
2. **Sem ADR nova**. ADR-0055 já cobre decisão 5; este passo
   materializa-a com anotação pós-IMPLEMENTADO. Modelo
   precedente: ADR-0019 anotada em 140A.
3. **Sem alteração de status de ADR-0055**. Anotação não é
   revisão.
4. **Helvetica continua a ser fallback** quando `style.font` é
   `None` ou não resolve.
5. **Silent drop** quando font está no documento mas não
   resolve (consistente com 140B).
6. **Ordem dos `/CrystallineFontN`**: ordem de primeira
   ocorrência no documento. Determinismo preservado.
7. **`build_cidfont` actual permanece**. Multi-font é
   construído por composição (chamar a lógica de single
   embedding N vezes), não por refactor de `build_cidfont`.

## Decisões diferidas (resolvidas neste passo)

8. **Localização de `collect_fonts_from_doc`**: L1 (módulo
   util) vs L3 (junto a `pipeline.rs` ou `export.rs`).
   Análoga a `first_font_from_doc` (que vive em L3 segundo
   relatório 140B). Decisão padrão: **L3**, junto a
   `pipeline.rs`. Confirmar em 146.1 que é o ponto certo.

9. **Convergência ou preservação de `export_pdf_with_font`**:
   - Opção A: `export_pdf_with_font(doc, bytes)` continua
     no API público; `export_pdf_multifont` é nova função
     paralela. Caso single-font usa A; multi-font usa nova.
     **Mais conservador**.
   - Opção B: `export_pdf_with_font` torna-se fina
     casca sobre `export_pdf_multifont(doc, vec![(FontList,
     bytes)])`. Single API; código antigo continua a
     funcionar. **Mais limpo**.
   - Decisão default: **B** — converger. Reduz duplicação;
     `export_pdf_with_font` continua a existir como helper.
     Confirmar em 146.1 por inspecção do uso real.

10. **Critério de igualdade de `FontList`**: igualdade
    estrutural (`PartialEq` derivado). Duas `FontList`
    são "iguais" se têm exactamente as mesmas famílias na
    mesma ordem. Documento com `["Inria"]` e `["Inria",
    "Arial"]` resolveriam **ambos** para "Inria" mas seriam
    `FontList` distintos — **2 entradas no resource dict**.
    Optimização (deduplicar pelo bytes resolvidos) é candidato
    futuro; aceite como trade-off.

11. **Limite máximo de fonts por documento**: nenhum imposto.
    PDF spec não tem limite prático. Documentar no
    relatório.

12. **Span sem `style.font` em documento multi-font**: emite
    com Helvetica (resource já tem entradas Helvetica do
    `export_pdf` original). Consistente com fallback.

---

## Escopo

**Dentro**:

- Edição de `03_infra/src/pipeline.rs` (nova função
  `collect_fonts_from_doc`, função estendida
  `resolve_fonts`, dispatch novo).
- Edição de `03_infra/src/export.rs` (nova função
  `export_pdf_multifont`; possível convergência de
  `export_pdf_with_font` se opção B em 146.1).
- Possível accessor novo em L1 (improvável; APIs do 140B já
  expõem o necessário).
- Tests unit + integração.
- Edição de prompt L0 que descreve o pipeline (`prompts/infra/
  pipeline.md` segundo relatório 140B).
- Anotação em ADR-0055.
- Actualização de DEBT.md (Secção 2, entrada DEBT-52).
- Actualização de README dos ADRs (Passos-chave).
- Recálculo + propagação de hash L0.

**Fora**:

- Refactor de `build_cidfont` (composição preferida).
- Refactor de `export_pdf` (Helvetica fallback intacto).
- Mudança em `first_font_from_doc` ou `resolve_font` (helpers
  do 140B/141 — preservados; multi-font é extensão por adição).
- Selecção variant-aware (ADR-0055bis).
- Subsetting (ADR-0056).
- Shaping (DEBT-53).
- Pipeline de hyphenation (independente do 144).
- Crates novas.

---

## Sub-passos

### 146.1 — Inventário pré-materialização

**A.1.1 — Confirmar API actual**:

```
view 03_infra/src/pipeline.rs        # confirmar first_font_from_doc, resolve_font, dispatch
view 03_infra/src/export.rs          # confirmar build_cidfont, export_pdf_with_font
```

Registar:
- Assinatura exacta de `first_font_from_doc`,
  `resolve_font`, `export_pdf_with_font`.
- Estrutura interna de `build_cidfont`: que parametros
  toma; que partes podem ser reutilizadas para multi-font.
- Se `FontList` deriva `PartialEq` + `Eq` + `Hash`
  (necessário para deduplicação trivial).

**A.1.2 — Decidir convergência (opção A vs B)**:

Procurar usos externos de `export_pdf_with_font`:

```
grep -rn "export_pdf_with_font" 03_infra/ 04_wiring/
```

Se único uso é em `pipeline.rs` (esperado pelo 140B),
**opção B** (converger) é trivial e preferida. Se há mais
chamadas, ponderar.

**A.1.3 — Inspeccionar `build_cidfont`**:

`view 03_infra/src/export.rs:423-...` em range razoável (até
~520). Identificar:
- Que objectos PDF cria (Type0, CIDFontType2, FontFile2,
  Widths, ToUnicode CMap).
- Se algum estado é "global" ao documento (numeração de
  objectos, resource dict).
- Que parâmetros já são parametrizáveis vs hardcoded.

A multi-font precisa de **N** Font dictionaries no resource
dict + N FontDescriptor + N FontFile2 streams. Cada um pode
ser produzido pela mesma lógica de `build_cidfont` se a
função for parametrizada por `(font_data, name)`.

**A.1.4 — Decidir L1 vs L3 para `collect_fonts_from_doc`**:

Já decidido: L3, junto a `pipeline.rs`. Confirmar que não há
contra-indicação (ex: a iteração de `PagedDocument` não exige
trait que só viva em L1).

### 146.2 — `collect_fonts_from_doc`

**Forma**:

```rust
fn collect_fonts_from_doc(doc: &PagedDocument) -> Vec<FontList> {
    let mut seen: Vec<FontList> = Vec::new();
    for page in &doc.pages {
        collect_fonts_in_items(&page.items, &mut seen);
    }
    seen
}

fn collect_fonts_in_items(items: &[FrameItem], seen: &mut Vec<FontList>) {
    for item in items {
        match item {
            FrameItem::Text(text) => {
                if let Some(fl) = &text.style.font {
                    if !seen.contains(fl) {
                        seen.push(fl.clone());
                    }
                }
            }
            FrameItem::Group { items, .. } => {
                collect_fonts_in_items(items, seen);
            }
            _ => {}
        }
    }
}
```

Forma idêntica à de `first_font_from_doc` (relatório 140B)
mas acumulativa. Deduplicação via `Vec::contains` é O(N²) —
aceite porque N (fonts distintas no documento) é tipicamente
pequeno (<10).

**Tests unit (4)**:

- `collect_fonts_from_doc_documento_vazio_devolve_vazio`.
- `collect_fonts_from_doc_uma_font_devolve_unitario`.
- `collect_fonts_from_doc_duas_distintas_devolve_par_em_ordem`.
- `collect_fonts_from_doc_duas_iguais_dispersas_dedup`.

### 146.3 — `resolve_fonts`

**Forma**:

```rust
fn resolve_fonts(
    font_lists: &[FontList],
    font_book: &FontBook,
    world: &dyn World,
) -> Vec<(FontList, Vec<u8>)> {
    font_lists.iter()
        .filter_map(|fl| {
            resolve_font(fl, font_book, world).map(|bytes| (fl.clone(), bytes))
        })
        .collect()
}
```

Reutiliza `resolve_font` do 141 (já itera famílias na
`FontList`). Silent drop quando `resolve_font` devolve
`None`.

**Tests unit (3)**:

- `resolve_fonts_todos_resolvem`.
- `resolve_fonts_alguns_nao_resolvem_filtrados`.
- `resolve_fonts_nenhum_resolve_devolve_vazio`.

### 146.4 — `export_pdf_multifont`

**A.4.1 — Estrutura interna**:

```rust
pub fn export_pdf_multifont(
    doc: &PagedDocument,
    fonts: &[(FontList, Vec<u8>)],
) -> Vec<u8> {
    // 1. Construir N entradas no resource dict:
    //    /CrystallineFont1, /CrystallineFont2, ... (uma por (FontList, bytes))
    // 2. Para cada entrada: build_cidfont_object(font_bytes, name="CrystallineFontN")
    //    — função interna refactorizada a partir de build_cidfont actual.
    // 3. Para cada FrameItem::Text:
    //    - Procurar match: qual font_lists[i] == text.style.font?
    //    - Se match: emitir Tf /CrystallineFont{i+1} <size> antes do TJ.
    //    - Se não match (ou style.font is None): Helvetica fallback.
    // 4. Combinar em PDF final via PdfBuilder existente.
}
```

**A.4.2 — Refactor mínimo de `build_cidfont`**:

Se `build_cidfont` actual é monolítica, extrair função
auxiliar:

```rust
fn build_cidfont_object(
    pdf_builder: &mut PdfBuilder,
    font_data: &[u8],
    name: &str,
) -> ObjectRef;
```

Parametrizada por `name` ("CrystallineFont1", etc.).
`build_cidfont` actual passa a chamar
`build_cidfont_object(builder, data, "CrystallineFont")`.

Se já é parametrizada, nada a fazer.

**A.4.3 — Match span → font index**:

```rust
fn font_index_for_text(text: &Text, fonts: &[(FontList, _)]) -> Option<usize> {
    let fl = text.style.font.as_ref()?;
    fonts.iter().position(|(stored, _)| stored == fl)
}
```

`Some(i)` → emit `/CrystallineFont{i+1}`. `None` → Helvetica
fallback.

### 146.5 — Dispatch em `compile_to_pdf_bytes`

```diff
- let pdf = match first_font_from_doc(&doc)
-     .and_then(|fl| resolve_font(&fl, world.book(), world))
- {
-     Some(bytes) => export_pdf_with_font(&doc, &bytes),
-     None        => export_pdf(&doc),
- };
+ let font_lists = collect_fonts_from_doc(&doc);
+ let resolved = resolve_fonts(&font_lists, world.book(), world);
+ let pdf = match resolved.len() {
+     0 => export_pdf(&doc),
+     _ => export_pdf_multifont(&doc, &resolved),  // 1+ entradas
+ };
```

Se opção A (preservar `export_pdf_with_font`):

```diff
+ let pdf = match resolved.as_slice() {
+     []      => export_pdf(&doc),
+     [(_, b)] => export_pdf_with_font(&doc, b),  // optimização single
+     many    => export_pdf_multifont(&doc, many),
+ };
```

**Decisão preferida (opção B)**: simplicidade vence;
`export_pdf_multifont` lida bem com 1 entrada (caso especial
trivial).

### 146.6 — Tests de integração L3

Em `03_infra/src/integration_tests.rs`:

| Test | Cobre |
|------|-------|
| `font_wiring_multifont_dois_fonts_distintos_ambos_embebidos` | doc com 2 spans `#set text(font:)` em famílias diferentes (ambas resolvíveis) → PDF tem **2 ocorrências** de `/Subtype /Type0` |
| `font_wiring_multifont_uma_resolve_outra_falla_silent_drop` | 3 fonts no doc; 1 não disponível → PDF tem 2 `/Subtype /Type0`; spans da terceira caem em Helvetica |
| `font_wiring_multifont_regressao_single_font_inalterado` | doc single-font produz output funcionalmente equivalente ao pré-146 (1 `/Type0`); regressão zero |

Mesmo padrão `discover_any_system_fonts` + early-return do
140B/141.

**Test 4 (revisão dos tests do 140B)**: o test
`font_wiring_segunda_font_diferente_primeira_vence` do 140B
**precisa de revisão**. Pré-146 documenta limitação MVP;
pós-146 a primeira não vence — ambas embebidas. **Substituir
test** ou **renomear para reflectir comportamento pós-146**.

Decisão (resolver em 146.6):
- **Renomear**: `font_wiring_segunda_font_diferente_ambas_embebidas`.
- Assertion muda: de `exactly 1 /Type0` para `exactly 2`.

Atenção: este é trabalho a ter em conta no relatório como
"regressão deliberada do MVP do 140B" — não bug.

### 146.7 — Anotar ADR-0055

```diff
+ **Anotação Passo 146**: decisão 5 (multi-font per document)
+ materializada — ver
+ `00_nucleo/materialization/typst-passo-146-relatorio.md`.
```

Adicionada após a linha `**Status**:` (ou no fim da secção de
cabeçalho, antes da `## Contexto`). Modelo: ADR-0019 com a
sua nota factual de 140A.

Secção "Materialização" (introduzida em 141) ganha entrada:

```markdown
- **Passo 146** (2026-04-24) — multi-font per document
  (decisão 5). `collect_fonts_from_doc` + `resolve_fonts` +
  `export_pdf_multifont`. **Anotação pós-IMPLEMENTADO** sem
  revisão de status; modelo análogo a ADR-0019 + 140A.
```

**Status permanece `IMPLEMENTADO`**. Sem revisão.

### 146.8 — Edição L0

**`prompts/infra/pipeline.md`** (hash actual: `00e4ebd3` desde
141):

Substituir secção "Helpers privados de dispatch" actual por
versão estendida:

```markdown
### `collect_fonts_from_doc(doc) -> Vec<FontList>`

Itera `doc.pages → items → FrameItem::Text.style.font`,
descendo recursivamente em `FrameItem::Group`. Devolve todas
as `FontList` distintas em ordem de primeira ocorrência;
deduplicação por igualdade estrutural.

### `resolve_fonts(font_lists, font_book, world) -> Vec<(FontList, Vec<u8>)>`

Map-filter sobre `resolve_font` do Passo 141. Filter silent
drop de entries que não resolvem.

### Dispatch em `compile_to_pdf_bytes`

```
let resolved = resolve_fonts(collect_fonts_from_doc(&doc), …);
match resolved.len() {
    0 → export_pdf(&doc)             # Helvetica fallback
    _ → export_pdf_multifont(&doc, &resolved)
}
```

`export_pdf_with_font` permanece no API como compat shim
sobre `export_pdf_multifont` (single-font é caso particular).
```

Recalcular hash; propagar a `03_infra/src/pipeline.rs` e
`03_infra/src/export.rs` (se este último também consumir o
prompt — confirmar em 146.1).

### 146.9 — Actualizar DEBT.md

`DEBT.md` Secção 2, entrada DEBT-52, sub-secção nova:

```markdown
### Actualização Passo 146 — Multi-font per document

- [x] Decisão 5 de ADR-0055 (multi-font per document)
  materializada pós-fecho de DEBT-1. ADR-0055 anotada com
  referência factual; **status permanece `IMPLEMENTADO`**;
  sem revisão. **DEBT-52 não reabre**: este passo é
  extensão voluntária do MVP single-font do 140B,
  consistente com perfil observacional graded de ADR-0054.
```

Contagem de DEBTs abertos: **inalterada (10)**.

### 146.10 — Actualizar README dos ADRs

Em `00_nucleo/adr/README.md`, secção "Passos-chave da
história dos ADRs":

```markdown
- **Passo 146** — Multi-font per document. Materialização
  da decisão 5 de ADR-0055 (declarada opcional na própria
  ADR). `collect_fonts_from_doc` + `resolve_fonts` +
  `export_pdf_multifont`. ADR-0055 anotada
  (pós-IMPLEMENTADO, modelo ADR-0019 + 140A); status
  inalterado.
```

**Sem mudança** na tabela "Estado por ADR" (ADR-0055
permanece `IMPLEMENTADO`). Sem mudança na distribuição de
status. Sem mudança no total (57 ADRs).

### 146.11 — Verificação automatizada + relatório

Comandos:

```bash
cargo test --workspace --lib
crystalline-lint .
git diff --stat 00_nucleo/adr/typst-adr-0055-*.md
git diff --stat 00_nucleo/DEBT.md
```

Relatório: `materialization/typst-passo-146-relatorio.md`
com secções:

1. Sumário executivo.
2. Decisão de numeração (146 não 142A) — registar com
   justificação.
3. Inventário pré-materialização (resultado de 146.1).
4. `collect_fonts_from_doc` (assinatura + tests).
5. `resolve_fonts` (assinatura + tests).
6. `export_pdf_multifont` (forma + refactor mínimo de
   `build_cidfont` se houve).
7. Dispatch em `compile_to_pdf_bytes`.
8. Tests adicionados; revisão do test do 140B (renomeado).
9. Anotação em ADR-0055.
10. Edição L0 + hash propagado.
11. DEBT-52 actualizado.
12. README dos ADRs actualizado.
13. Limitações registadas (sem mudança):
    - Variant-aware ainda ausente.
    - Subsetting ausente.
    - Shaping ausente.
    - Gap 8 (font dict) opcional.
14. Verificação final.

---

## Verificação

1. ✅ `collect_fonts_from_doc` + 4 unit tests.
2. ✅ `resolve_fonts` + 3 unit tests.
3. ✅ `export_pdf_multifont` materializada.
4. ✅ `compile_to_pdf_bytes` dispatch novo.
5. ✅ 3 tests integração L3 (multi-font + regressão
   single-font).
6. ✅ Test do 140B renomeado e assertion ajustada.
7. ✅ ADR-0055 anotada (status `IMPLEMENTADO` preservado).
8. ✅ DEBT-52 secção 2 com actualização Passo 146.
9. ✅ README dos ADRs com entrada P146 em "Passos-chave".
10. ✅ L0 `prompts/infra/pipeline.md` actualizado; hash
    propagado.
11. ✅ Tests pré-existentes inalterados (excepto o
    renomeado do 140B).
12. ✅ `cargo test --workspace --lib`: 1104 → ~1110-1112
    (acréscimo +7 unit + 3 integração; -1 substituído ≈
    +9). Ajustar no relatório.
13. ✅ `crystalline-lint .`: zero violations.
14. ✅ L1 de domínio intacto (apenas L3 + L0 + ADRs +
    DEBT.md + README ADR).
15. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Documento com N fonts distintas produz PDF com N
   `/Subtype /Type0` embebidos.
2. Spans com `style.font` cuja família não resolve caem
   silenciosamente em Helvetica (preserva fallback).
3. Documento single-font tem comportamento funcionalmente
   equivalente ao pré-146 (regressão zero).
4. ADR-0055 anotada com decisão 5 materializada;
   `IMPLEMENTADO` preservado.
5. DEBT-52 não reabre; anotação ao histórico apenas.
6. Tests verdes; lint zero; hash propagado.
7. Relatório do passo escrito.

---

## O que pode sair errado

- **`build_cidfont` é monolítica e não parametrizada por
  nome de font**: refactor mínimo necessário em 146.4.A.2.
  Se refactor cresce além de "extrair função interna",
  registar como dívida documental e materializar
  multi-font com nomes hardcoded `/CrystallineFont1` etc.
  parametrizados de forma ad-hoc. **Pausar** se refactor
  de `build_cidfont` ultrapassa ~50 linhas — pode justificar
  passo dedicado.

- **Resource dict do `export_pdf` (Helvetica) entra em
  conflito com resource dict de `export_pdf_multifont`**:
  ambos definem `/F1`, `/F2`, `/F3` para Helvetica
  variants. Em multi-font, manter `/F1`–`/F3` para
  Helvetica fallback **e** adicionar `/CrystallineFont1`+
  como entries paralelas. Confirmar em 146.1.

- **`FontList` não deriva `PartialEq`**: bloqueia
  deduplicação. Adicionar derive em L1 — accessor mínimo
  (sem lógica). Justifica nota no relatório como única
  alteração em L1 do passo.

- **`PagedDocument` ou `FrameItem::Group` não expõem o
  necessário**: improvável (relatório 140B/141 confirmaram
  acesso público). Se acontecer, accessor puro em L1.

- **Match `text.style.font == stored` falha em casos
  edge**: ex: `FontList::single("Inria")` capturado em
  contextos diferentes pode produzir instâncias com
  ordenação interna distinta? Pouco provável dado o
  determinismo de eval, mas verificar com test caso seja
  útil.

- **Test renomeado do 140B**: substituir vs deletar vs
  renomear — preferir **renomear** para preservar
  rastreabilidade (mesmo nome de teste anterior continua
  pesquisável; mudou só o sufixo do nome e a assertion).

- **Documento muito grande com >50 fonts distintas**:
  `Vec::contains` O(N²) no `collect_fonts_from_doc` torna-se
  perceptível. Aceite por agora; se surgir caso real,
  optimização para `HashSet` é trivial. Limitação registada.

- **Múltiplas instâncias do mesmo font bytes mas
  `FontList` diferentes**: ex: `["Inria"]` e `["Inria",
  "Arial"]` — mesma resolução real, FontList distinta.
  Resultado: 2 entradas no resource dict apontando para os
  mesmos bytes. PDF maior; sem incorrecção observacional.
  Aceite; optimização futura.

- **`export_pdf_multifont` com vec.len() == 1 produz PDF
  diferente de `export_pdf_with_font`**: opção B
  (convergência) requer que `export_pdf_multifont(doc,
  vec![entry])` produza output **byte-equivalente** (ou
  observacionalmente equivalente) ao
  `export_pdf_with_font(doc, bytes)`. Test de regressão do
  146.6 valida. Se houver diferença subtil (ex: nome
  `/CrystallineFont1` vs `/CrystallineFont`), registar e
  decidir: aceitar ou alinhar.

- **`export_pdf_with_font` chamado externamente em código
  não-pipeline**: opção B preserva o ponto de entrada como
  shim. Se há chamadas que assumem o nome `/CrystallineFont`
  específico, alinhar shim. Confirmado em 146.1.A.2.

---

## Notas operacionais

- **Decisão de numeração 146 (não 142A)**: registada
  explicitamente no cabeçalho deste passo. O sufixo `A` na
  grelha actual significa "diagnóstico-primeiro associado a
  trabalho futuro com o mesmo número". 142 já foi executado
  como passo administrativo de fecho de DEBT-1; reusar 142A
  introduziria ambiguidade. 146 é o seguinte número
  natural após 145.

- **ADR-0055 anotada, não revista**. Modelo ADR-0019 +
  140A. Anotação documenta facto histórico (decisão 5
  materializada) sem alterar status.

- **DEBT-52 não reabre**. Padrão consistente com 144 (gap 7).
  Histórico ganha actualizações; estado de "encerrado"
  preserva-se. Se mais gaps forem materializados pós-fecho
  (gap 8 via ADR-0054bis seria o próximo candidato), padrão
  repete-se.

- **Pós-146**: Fase C de DEBT-1 está completa **incluindo
  multi-font**. Limitações remanescentes do mapa do
  relatório 142 §9:
  - 1. Variant-aware (ADR-0055bis candidata).
  - 2. ~~Multi-font~~ — fechada por 146.
  - 3. Subsetting (ADR-0056 candidata).
  - 4. Shaping rustybuzz (DEBT-53 candidato XL).
  - 5. ~~Lang/hyphenation~~ — fechada por 144.
  - 6. Font dict (ADR-0054bis condicional, gap 8).
  - 7. Reprodutibilidade tests CI.

  Restam **5 candidatos** não-bloqueantes.

- **Optimização O(N²) → O(N)** em `collect_fonts_from_doc`
  é trivial via `HashSet<FontList>`, mas exige `Hash` em
  `FontList`. Por agora, `Vec::contains` é suficiente para
  documentos típicos. Optimização registada como candidato
  futuro sem urgência.

- **Sem ADR-0058**. Anotação em ADR-0055 cobre
  materialização. Se a frequência de "ADR anotada
  pós-IMPLEMENTADO" aumentar, considerar formalizar
  convenção ("anotação factual") na secção "Convenções
  estruturais" do README dos ADRs — meta-decisão futura
  fora deste passo.

- **Pipeline pós-146 invariável quanto a hyphenation
  (Passo 144)**. Multi-font opera no embedding; hyphenation
  opera na quebra de linha. Independentes. Documento
  hyphenated + multi-font produz PDF que combina ambos
  (hífenes inseridos onde aplicável; cada span emite na
  sua font correcta).
