# Passo 140B — Wiring single-font (consumer `text.font` via CIDFont existente)

**Série**: 140B (passo **S** em L3/L4; primeira materialização
da Fase C do roadmap DEBT-1).
**Precondição**: Passo 140A encerrado; 1100 total tests; zero
violations; **55 ADRs** (ADR-0055 em `PROPOSTO`); ADR-0019
anotada com nota factual; 12 DEBTs abertos (DEBT-52 com 4 gaps
— Fase B completa).

**ADRs aplicáveis**:
- **ADR-0055** (`PROPOSTO` → permanece `PROPOSTO`; transita a
  `IMPLEMENTADO` apenas após Passo 141 fechar o gap 6). Este
  passo **materializa a decisão 3** (single font per document,
  MVP).
- **ADR-0033** (paridade funcional) — primeira font encontrada
  é usada para o documento; outras são ignoradas silenciosamente
  (MVP aceite).
- **ADR-0054** (critério fecho DEBT-1) — este passo ataca o
  gap 5 de DEBT-52.
- **ADR-0053** (FontList materializado) — tipo consumido aqui.
- **ADR-0019** (TTF+RustyBuzz, `IMPLEMENTADO` com nota factual
  do 140A) — `ttf-parser` é a infra usada; `rustybuzz`
  permanece sem uso.

**Natureza**: passo **L3/L4** com pequena adição em L1
(accessor se necessário). **Zero alteração em L1 de domínio**
(FontBook, World, Font já existem — ver 140A). **Zero crates
novas** (ADR-0055 decisão 1).

**Tamanho estimado**: **S**, ~2h (conforme roadmap 140A secção
"Próximos passos").

---

## Contexto

Diagnóstico 140A revelou que a infra CIDFont embedding já
existe em `03_infra/src/export.rs:423` (`build_cidfont`),
exposta via `export_pdf_with_font(doc, font_data)`. O pipeline
actual em `03_infra/src/pipeline.rs:78` usa `export_pdf`
(Helvetica fallback), ignorando o campo `TextStyle.font`
capturado pelo eval.

O gap é **wiring**, não refactor:

1. Iterar `PagedDocument` procurando primeiro
   `TextStyle.font` não-None.
2. Para cada `FontFamily.name` na `FontList`, tentar
   `FontBook::select(name, variant)`.
3. Se match: `World::font(index) → bytes` →
   `export_pdf_with_font`.
4. Se nenhum match: fallback `export_pdf` (comportamento
   actual).

Este passo fecha o **gap 5 de DEBT-52** (consumer `font`
string via `FontBook::select`). O **gap 6** (array fallback
chain) é atacado no Passo 141.

---

## Objectivo

Ao fim do passo:

1. **Pipeline consumer** materializado em `03_infra/src/pipeline.rs`
   (ou `04_wiring/` se for o ponto de composição correcto —
   decidir em 140B.1):
   - Função de extracção `first_font_from_doc(doc) →
     Option<FontList>`.
   - Função de resolução `resolve_font(font_list, font_book,
     world) → Option<Vec<u8>>`.
   - Dispatch: se resolve retorna bytes →
     `export_pdf_with_font`; senão → `export_pdf`.

2. **Testes L3** de integração:
   - Documento com `#set text(font: "Inria Serif")` com font
     disponível → PDF embute a font (CIDFont presente).
   - Documento com `#set text(font: "NonExistentFont")` →
     fallback Helvetica (comportamento actual preservado).
   - Documento sem `#set text(font:)` → fallback Helvetica.
   - Documento com segunda font diferente no meio:
     **primeira vence** (MVP single-font).

3. **DEBT-52 gap 5 marcado resolvido** em `DEBT.md`.

4. **ADR-0055 permanece `PROPOSTO`**. Transição a
   `IMPLEMENTADO` é diferida para Passo 141 (após gap 6
   fechar) — o par 140B+141 é a unidade de materialização da
   ADR-0055.

Este passo **não**:

- Toca em L1 de domínio (`FontBook`, `Font`, `World`,
  `StyleChain`, `StyleDelta`). Podem existir pequenos
  accessors em L1 se a iteração do `PagedDocument` exigir —
  nesse caso documentar no relatório.
- Adiciona crates novas (ADR-0055 decisão 1).
- Implementa multi-font per document (ADR-0055 decisão 5,
  Passo 142 opcional).
- Implementa subsetting (fora de DEBT-1; candidato ADR-0056
  futura).
- Toca `rustybuzz` (candidato DEBT-53, escopo XL).
- Ataca gap 6 (array fallback chain — Passo 141).
- Ataca gap 7 (lang hyphenation — Passo 143 opcional).

---

## Decisões já tomadas (herdadas de 140A + ADR-0055)

1. **Single font per document**: primeira font encontrada no
   documento é usada para o documento inteiro. Spans
   posteriores com font diferente são ignorados silenciosamente.
   MVP consciente (ADR-0055 decisão 3).
2. **Fallback Helvetica** quando nenhuma família resolve
   (comportamento actual preservado).
3. **Zero crates novas** (ADR-0055 decisão 1).
4. **`export_pdf_with_font` é o entry point correcto**. Não
   refactor de `build_cidfont`.
5. **Sem warning/erro** quando segunda font é ignorada. Silent
   drop é MVP; telemetria é decisão futura.
6. **ADR-0055 permanece `PROPOSTO`** até Passo 141 fechar.

## Decisões diferidas (resolvidas neste passo)

7. **Localização do wiring**: `03_infra/src/pipeline.rs` vs
   `04_wiring/`. Decisão em 140B.1 após inspecção de quem
   constrói `SystemWorld` + `FontBook` + invoca export.
8. **Forma de iteração do `PagedDocument`**: `doc.pages →
   items → FrameItem::Text.style.font`. Confirmar em 140B.1
   que o acesso é possível sem alteração de L1. Se exigir
   accessor novo em L1, é accessor puro (sem lógica), sem
   tocar domínio.
9. **Resolução `FontFamily.name → FontBook::select`**:
   qual variant (weight, style) passar? MVP: usar
   `FontVariant::default()` (regular). `#set text(weight:
   700)` continua a ser renderizado via faux-bold do Passo 139
   — não há tentativa de selecção de font-file "Bold" dedicado.
   Esta é uma simplificação consciente; paridade total com
   vanilla exige ADR-0055bis (selecção variant-aware) futura
   — **registar como limitação conhecida**, não gap novo.

---

## Escopo

**Dentro**:

- Leitura de `03_infra/src/pipeline.rs:78` para localizar
  ponto de troca.
- Leitura de `03_infra/src/export.rs:423` para confirmar
  assinatura `export_pdf_with_font(doc, font_data)`.
- Leitura de `01_core/src/entities/font_book.rs:183` para
  confirmar assinatura `FontBook::select`.
- Leitura de `01_core/src/entities/world_types.rs:30` para
  confirmar `Font(Vec<u8>)` runtime.
- Leitura de `01_core/src/contracts/world.rs:55` para
  confirmar trait `World::font(index)`.
- Escrita de:
  - Função `first_font_from_doc` (L3 ou L4).
  - Função `resolve_font` (L3 ou L4).
  - Dispatch no pipeline.
  - Testes de integração L3.
- Pequeno accessor em L1 **apenas se** a iteração exigir —
  documentar no relatório.
- Actualização de `DEBT.md` (gap 5 marcado `[x]`).

**Fora**:

- Refactor de `build_cidfont` (assinatura actual preservada).
- Refactor de `FontBook` ou `Font` em L1.
- Multi-font per document (Passo 142).
- Array fallback chain (Passo 141 — **próximo passo**).
- Subsetting, CFF, shaping, hyphenation.
- Selecção variant-aware (font-file "Bold" dedicado).
- Warning/erro quando font não resolve.
- `rustybuzz` integration (DEBT-53 futuro).

---

## Sub-passos

### 140B.1 — Localizar ponto de wiring

**B.1.1 — Inspeccionar pipeline actual**:

`view 03_infra/src/pipeline.rs` — localizar função que chama
`export_pdf(doc)`. Esperado na linha 78 (referência do 140A).

Registar:
- Assinatura da função contentor (que recebe `doc`, `font_book`,
  `world`?).
- Se `font_book` e `world` já estão acessíveis nesse ponto.
- Se não: subir até ao ponto onde estão — candidato a
  `04_wiring/`.

**B.1.2 — Inspeccionar `export_pdf_with_font`**:

`view 03_infra/src/export.rs` view_range 420-460 — confirmar:
- Assinatura pública exacta.
- Tipo de `font_data` (`Vec<u8>`? `&[u8]`?).
- Se há testes existentes em `integration_tests.rs:620` (140A
  mencionou).

**B.1.3 — Decisão**:

Localização do wiring: `pipeline.rs` se `font_book`+`world`
estão disponíveis aí, senão `04_wiring/`. Registar no
relatório a decisão e a razão.

### 140B.2 — Escrever `first_font_from_doc`

**B.2.1 — Forma da função**:

```rust
fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList> {
    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Text(text) = item {
                if let Some(fl) = &text.style.font {
                    return Some(fl.clone());
                }
            }
        }
    }
    None
}
```

Forma final depende da API real de `PagedDocument` — confirmar
em `01_core/src/entities/` antes de escrever.

**B.2.2 — Testes unitários**:

- `first_font_from_doc_documento_vazio_devolve_none`.
- `first_font_from_doc_sem_font_devolve_none`.
- `first_font_from_doc_com_font_primeira_vence`.
- `first_font_from_doc_font_em_pagina_segunda_encontrada`.

### 140B.3 — Escrever `resolve_font`

**B.3.1 — Forma da função**:

```rust
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world: &dyn World,
) -> Option<Vec<u8>> {
    let first_family = font_list.families().first()?;
    let variant = FontVariant::default();
    let index = font_book.select(first_family.name(), variant)?;
    let font = world.font(index)?;
    Some(font.data().to_vec())
}
```

**MVP: só primeira família** (single-font). Array fallback
chain é Passo 141.

**B.3.2 — Testes unitários**:

- `resolve_font_match_primeiro_devolve_bytes`.
- `resolve_font_nao_match_devolve_none`.
- `resolve_font_font_book_vazio_devolve_none`.

### 140B.4 — Dispatch no pipeline

**B.4.1 — Substituição do entry point**:

Substituir:

```rust
let pdf_bytes = export_pdf(&doc);
```

Por:

```rust
let pdf_bytes = match first_font_from_doc(&doc)
    .and_then(|fl| resolve_font(&fl, font_book, world))
{
    Some(font_data) => export_pdf_with_font(&doc, &font_data),
    None => export_pdf(&doc),
};
```

Forma final depende de B.1.3 (localização) e B.2/B.3
(assinaturas reais).

**B.4.2 — Preservar comportamento actual**:

Todos os testes existentes que não usam `#set text(font:)`
devem continuar a passar (cai no ramo `None → export_pdf`).

### 140B.5 — Testes L3 de integração

**Ficheiro**: `03_infra/tests/` (ou local canónico — confirmar
em B.1).

**Testes propostos**:

1. `font_wiring_set_text_font_existente_embute_cidfont`:
   - Setup: descobrir font de teste via `--font-path`
     (`tests/fixtures/fonts/`).
   - Documento: `#set text(font: "<nome da font fixture>"); Hello`.
   - Assert: PDF bytes contêm marker CIDFont
     (`/Subtype /Type0` ou similar).

2. `font_wiring_set_text_font_inexistente_fallback_helvetica`:
   - Documento: `#set text(font: "FontQueNaoExiste"); Hello`.
   - Assert: PDF bytes **não** contêm marker CIDFont.
   - Assert: PDF válido (fallback limpo).

3. `font_wiring_sem_set_text_font_usa_helvetica`:
   - Documento: `Hello`.
   - Assert: comportamento actual preservado (nenhum CIDFont).

4. `font_wiring_segunda_font_diferente_primeira_vence`:
   - Documento com dois `#set text(font:)` aninhados de
     famílias diferentes (ambas disponíveis).
   - Assert: font embutida é a **primeira encontrada** na
     iteração do frame.
   - **Nota**: este teste documenta a limitação MVP. Passo
     142 (opcional) alteraria o comportamento para multi-font.

**Fixture de fonts**: usar fonts livres de licença restritiva
(ex: DejaVu, Inria Serif se já em `tests/fixtures/`).
Confirmar em B.1 se fixture existe; se não, adicionar em
subdir dedicado com README de proveniência.

### 140B.6 — Actualizar DEBT.md

Em `DEBT-52` secção "Âmbito":

```diff
- - [ ] Consumer `font` string (nome via `FontBook::select`).
+ - [x] Consumer `font` string (nome via `FontBook::select`).
+       **Resolvido no Passo 140B** — wiring single-font per
+       document via `export_pdf_with_font` existente. MVP
+       primeira font vence; variant-aware selecção é
+       limitação conhecida (ADR-0055bis candidato futuro).
```

Actualizar cabeçalho de DEBT-52 se necessário (contagem de
gaps resolvidos: 4 → 5).

### 140B.7 — Relatório

Ficheiro: `00_nucleo/materialization/typst-passo-140b-relatorio.md`.

Secções:

1. Sumário executivo.
2. Localização do wiring escolhida (resultado de B.1.3).
3. Funções criadas (`first_font_from_doc`, `resolve_font`).
4. Dispatch no pipeline.
5. Testes adicionados.
6. DEBT-52 gap 5 resolvido.
7. Limitações registadas (variant-aware; multi-font;
   rustybuzz).
8. Próximo passo: 141 (array fallback chain, XS ~45min).
9. Verificação final (tests + lint + paridade).

---

## Verificação

1. ✅ `first_font_from_doc` escrita + 4 testes unitários.
2. ✅ `resolve_font` escrita + 3 testes unitários.
3. ✅ Dispatch substituído no ponto correcto.
4. ✅ 4 testes L3 de integração a passar.
5. ✅ Todos os testes pré-existentes continuam a passar
   (regressão zero).
6. ✅ `cargo test --workspace`: **1100 → ~1107** (acréscimo
   esperado 4 unit + 3 unit + 4 integração ≈ 11; ajustar no
   relatório se divergir).
7. ✅ `crystalline-lint`: zero violations.
8. ✅ L1 de domínio intacto (se houve accessor novo, é puro
   e está justificado no relatório).
9. ✅ DEBT-52 gap 5 marcado `[x]` com referência ao Passo 140B.
10. ✅ ADR-0055 permanece `PROPOSTO`.

---

## Critério de conclusão

1. Pipeline passa a invocar `export_pdf_with_font` quando
   `#set text(font:)` resolve.
2. Comportamento fallback preservado para documentos sem font
   ou com font não resolvida.
3. DEBT-52 gap 5 fechado.
4. Limitação MVP (single-font, variant-default) documentada.
5. `cargo test --workspace` verde.
6. `crystalline-lint` zero violations.
7. Relatório 140B escrito.
8. Próximo passo (141) prepared com escopo claro.

---

## O que pode sair errado

- **`first_font_from_doc` exige accessor que não existe em L1**:
  adicionar accessor puro (sem lógica, só leitura). Se o
  accessor for não-trivial (ex: atravessar estruturas
  compostas), pausar e discutir — pode exigir ADR separada.

- **`FontBook::select` com variant default não resolve font
  que existe em peso regular mas nome ligeiramente diferente**
  (ex: "Inria Serif" vs "InriaSerif"): este é comportamento
  correcto de `FontBook`, não bug. Registar no relatório com
  exemplos para o utilizador.

- **`export_pdf_with_font` tem pré-condições não óbvias**
  (ex: font deve ter glyph para todos os chars do documento):
  se testes revelarem, documentar limitação. Solução real é
  multi-font (Passo 142).

- **PDF válido mas sem CIDFont quando esperado**: confirmar
  com `pdftotext` ou validador que font está realmente
  embutida. Assert por marker textual pode ser frágil —
  alternativa é parse estrutural do PDF.

- **Testes L3 exigem fixtures de font reais**: se fixture
  ainda não existe, criar subdirectório `tests/fixtures/fonts/`
  com fonts de licença permissiva + README de proveniência.
  Passo continua dentro do escopo.

- **Localização do wiring exige refactor maior**: se
  `pipeline.rs` não tem acesso a `font_book`+`world`,
  pode ser necessário propagar esses valores. Se a propagação
  é não-trivial, pausar e considerar se o wiring pertence a
  `04_wiring/` — onde a composição já os tem.

- **Iteração do `PagedDocument` cruza fronteira de abstracção
  inesperada** (ex: `FrameItem` é privado): exige accessor
  novo. Se accessor é não-trivial, pausar.

---

## Notas operacionais

- **Par 140B+141 é a unidade lógica**. Single-font é MVP;
  array fallback completa a "paridade básica" de ADR-0055.
  ADR-0055 transita a `IMPLEMENTADO` apenas após 141.

- **Limitação variant-aware é aceite**. Paridade total com
  vanilla requer selecção de font-file "Bold"/"Italic"
  dedicado; cristalino usa `FontVariant::default()` + faux-bold
  (Passo 139). ADR-0055bis futura pode refinar.

- **Silent drop de fonts subsequentes** é MVP. Warning/erro
  é decisão futura. Se utilizadores reportam confusão,
  abrir DEBT dedicado.

- **Fase C básica = 140B + 141**. Após 141, DEBT-1 pode
  fechar (gap 7 hyphenation e gap 8 font dict são opcionais
  segundo ADR-0054).

- **Zero crates novas** significa zero nova burocracia. Primeiro
  passo de materialização da Fase C sem autorização pendente.
