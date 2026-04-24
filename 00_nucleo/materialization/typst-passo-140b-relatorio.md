# Passo 140B — Relatório (wiring single-font; DEBT-52 gap 5 fechado)

**Data**: 2026-04-24
**Precondição**: Passo 140A encerrado; 1100 total tests; zero
violations; 55 ADRs (ADR-0055 em `PROPOSTO`); 12 DEBTs abertos
(DEBT-52 com 4 gaps).
**Natureza**: passo **L3** com pequena edição em L0
(`prompts/infra/pipeline.md`). **Zero alteração em L1 de
domínio**. **Zero crates novas** (ADR-0055 decisão 1).
**ADRs aplicáveis**: ADR-0055 (decisão 3 materializada;
permanece `PROPOSTO` até Passo 141), ADR-0033, ADR-0053,
ADR-0054, ADR-0019.

---

## Sumário executivo

`compile_to_pdf_bytes` deixou de usar `export_pdf` cego
(Helvetica). Agora despacha para `export_pdf_with_font` quando o
`PagedDocument` contém `TextStyle.font` cuja primeira família
resolve em `world.book()` via `FontBook::select`. Caso contrário
mantém o fallback Helvetica (comportamento legado preservado).

**Gap 5 do DEBT-52 fechado** — primeiro consumer do `font` string
está em produção. Linha de DEBT.md actualizada com referência ao
passo. Início efectivo da **Fase C** do roadmap DEBT-1.

**ADR-0055 permanece `PROPOSTO`**. Transição a `IMPLEMENTADO`
fica para Passo 141 (array fallback chain — gap 6), conforme
spec do 140B (par 140B+141 é a unidade lógica).

**Tests**: 1100 → **1111** (+11: 4 unit `first_font_from_doc`
+ 3 unit `resolve_font` + 4 integração L3 `font_wiring_*`).
`cargo build` limpo. `crystalline-lint .` zero violations.

---

## 140B.1 — Localização do wiring escolhida

**Decisão**: `03_infra/src/pipeline.rs` — não foi necessário
escalar para `04_wiring/`.

**Razão**: a função `compile_to_pdf_bytes(world, source)` já
recebe `&dyn World`, que dá acesso a `world.book()` e
`world.font(index)`. Toda a informação necessária ao dispatch
está disponível **dentro** da função existente; não há
propagação adicional.

**Confirmações** (preconditions do 140A):
- `export_pdf_with_font(doc, font_data: &[u8]) -> Vec<u8>` em
  `03_infra/src/export.rs:423` — assinatura preservada.
- `FontBook::select(family, &FontVariant)` em
  `01_core/src/entities/font_book.rs:183` — case-insensitive
  por nome, escolhe variante mais próxima.
- `World::font(index) -> Option<Font>` em
  `01_core/src/contracts/world.rs` — entrega bytes via
  `Font::as_slice()`.

L1 **intacto** — nenhum accessor novo foi necessário. A iteração
do `PagedDocument` usa apenas APIs públicas pré-existentes
(`doc.pages`, `Page.items`, `FrameItem::Text { style, .. }`,
`TextStyle.font`, `FrameItem::Group { items, .. }`).

---

## 140B.2 — `first_font_from_doc` + testes

```rust
fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList>;
fn first_font_in_items(items: &[FrameItem]) -> Option<FontList>;
```

Iteração recursiva: percorre `doc.pages`; para cada página
percorre `page.items`. Em `FrameItem::Text`, devolve
`style.font.clone()` se for `Some`. Em `FrameItem::Group`,
desce recursivamente. `Line/Glyph/Image/Shape` são ignorados.

**Localização**: `03_infra/src/pipeline.rs` (privadas ao módulo).

**Testes unitários** (4):
- `first_font_from_doc_documento_vazio_devolve_none`
- `first_font_from_doc_sem_font_devolve_none`
- `first_font_from_doc_com_font_primeira_vence`
- `first_font_from_doc_font_em_pagina_segunda_encontrada`

---

## 140B.3 — `resolve_font` + testes

```rust
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>>;
```

Pega na primeira família de `font_list.as_slice()`. Consulta
`font_book.select(name, &FontVariant::default())`. Se match,
chama `world.font(index)` e devolve `font.as_slice().to_vec()`.

**MVP single-font / single-family**: apenas a primeira família
é tentada. Array fallback chain é Passo 141 (gap 6).

**Variant**: `FontVariant::default()` (regular, normal, normal
stretch). `weight`/`style` continuam a ser renderizados via
faux-bold do Passo 139 — selecção variant-aware (font-file
"Bold" dedicado) é candidato ADR-0055bis e está registada como
**limitação conhecida** (ver secção dedicada abaixo).

**Testes unitários** (3):
- `resolve_font_match_primeiro_devolve_bytes`
- `resolve_font_nao_match_devolve_none`
- `resolve_font_font_book_vazio_devolve_none`

Mock `FontMockWorld` ad-hoc nos tests injecta `Vec<Option<Font>>`
indexado, evitando dependência do filesystem.

---

## 140B.4 — Dispatch no pipeline

Substituição em `compile_to_pdf_bytes`:

```diff
- let pdf = export_pdf(&doc);
+ let pdf = match first_font_from_doc(&doc)
+     .and_then(|fl| resolve_font(&fl, world.book(), world))
+ {
+     Some(bytes) => export_pdf_with_font(&doc, &bytes),
+     None        => export_pdf(&doc),
+ };
```

Comportamento legado preservado: documentos sem `#set
text(font:)` ou com família não resolvida caem no ramo `None →
export_pdf` (Helvetica). 27 testes pré-existentes de
`integration::pipeline_*` continuam verdes sem alteração — o ramo
`Some` só é atingido por testes que carregam fonts.

---

## 140B.5 — Testes L3 de integração

Ficheiro: `03_infra/src/integration_tests.rs` (mod `integration`,
final do módulo).

| Teste | Cobre |
|-------|-------|
| `font_wiring_set_text_font_existente_embute_cidfont` | Documento com `#set text(font: <família-real>)` produz PDF com marker `CrystallineFont` (Type0). |
| `font_wiring_set_text_font_inexistente_fallback_helvetica` | `#set text(font: "FontQueNaoExiste")` cai no fallback; sem `CrystallineFont`, com `Helvetica`. |
| `font_wiring_sem_set_text_font_usa_helvetica` | Documento sem `#set text(font:)` mantém comportamento legado (Helvetica fallback). |
| `font_wiring_segunda_font_diferente_primeira_vence` | Dois `#set text(font:)` consecutivos com famílias distintas produzem **exactamente 1** `/Subtype /Type0` no PDF (single-font per document MVP). |

**Helper `discover_any_system_fonts()`**: probe de directórios
canónicos (`/usr/share/fonts/truetype/{dejavu,liberation}`,
`/Library/Fonts`, `/System/Library/Fonts`, etc). Devolve `None`
se nenhum candidato existe. Tests 1 e 4 fazem early-return com
`eprintln!("[skip] ...")` quando ausente — testes passam mas
sem assertions reais.

**Decisão sobre fixture dedicado**: o spec autoriza
`tests/fixtures/fonts/` mas não obriga. Optou-se por **probe
do sistema**, com graceful degradation, em vez de comprometer o
repo a binários TTF. Decisão revisitar quando 141/142 abrirem
discussão de testing reproduzível em CI sem fonts do sistema —
nesse momento abrir passo dedicado para fixture com licença
permissiva (DejaVu) e README de proveniência.

**Ambiente Linux deste passo** tinha DejaVu + Liberation +
Open Sans + Noto + outras instaladas: tests 1 e 4 executaram
assertions reais (sem skip).

---

## Edição L0 (`prompts/infra/pipeline.md`)

L0 estendido com:
- Spec do dispatch font-aware (descrição completa do match
  arm: `Some → export_pdf_with_font`, `None → export_pdf`).
- Secção "Helpers privados de dispatch (Passo 140B)"
  documentando `first_font_from_doc` e `resolve_font` com
  contratos e regras de iteração.
- Notas explícitas sobre MVP single-font (decisão 3),
  selecção variant-default e diferimento de array fallback
  chain (decisão 4 → Passo 141).

**Hash actualizado**: `169fbacd → 367f8790`. Header de
`03_infra/src/pipeline.rs` actualizado para `@prompt-hash
367f8790` + `@updated 2026-04-24`. `crystalline-lint --fix-hashes`
não foi necessário — actualização manual confirmada pelo lint
sem drift.

---

## Edição em `03_infra/Cargo.toml`

Adicionada secção `[dev-dependencies]` com `ecow = { workspace
= true }`. Razão: `FontList::single` exige `EcoString`; tests do
módulo `pipeline::tests` constroem `FontList` directamente.
`ecow` já era dep transitiva via `typst-core`; promovida a
dev-dep explícita para satisfazer o resolver do cargo. Zero
impacto em runtime / produção.

---

## DEBT-52 — actualização

Diff aplicado em `00_nucleo/DEBT.md` (secção DEBT-52, "Âmbito"):

```diff
- - [ ] Consumer `font` string (nome via `FontBook::select`).
+ - [x] Consumer `font` string (nome via `FontBook::select`).
+       **Resolvido no Passo 140B** — ...
+       **Início da Fase C.**
```

Gaps fechados: 4 → **5**. Restantes: array fallback (gap 6 →
Passo 141), lang hyphenation (gap 7 → opcional), font dict
(gap 8 → opcional, ADR-0054bis se).

---

## Limitações registadas

1. **Selecção variant-aware**: usar `FontVariant::default()`
   significa que `#set text(font: "Inria Serif", weight: 700)`
   não selecciona a face "Inria Serif Bold" do disco — a
   regular é embutida e o weight é simulado pelo faux-bold do
   Passo 139. Paridade total com vanilla exige ADR-0055bis
   futura.
2. **Multi-font per document**: dois `#set text(font:)` com
   famílias distintas → primeira vence; segunda silenciosamente
   ignorada. Multi-font é Passo 142 (opcional).
3. **Array fallback chain**: `#set text(font: ("A", "B"))`
   tenta apenas "A" actualmente. Iteração sequencial da lista é
   Passo 141 (próximo).
4. **Subsetting**: a font é embutida inteira; sem subsetting.
   Out-of-scope DEBT-1; candidato ADR-0056 futura.
5. **Shaping**: continuamos sem `rustybuzz`. Glyph mapping é
   trivial (CMAP directo). Out-of-scope (DEBT-53 candidato XL).
6. **Lang/hyphenation**: gap 7 inalterado.
7. **Reprodutibilidade de testes em CI**: tests `font_wiring_*`
   1 e 4 dependem de fonts no sistema (probe de
   `/usr/share/fonts` + macOS paths). Em ambientes sem TTFs,
   degradam para early-return. CI definitivo exigirá fixture
   dedicado — passo futuro.

---

## Próximo passo: 141 (array fallback chain — XS, ~45min)

Spec esperada para 141:
- `resolve_font` itera **todas** as famílias de
  `font_list.as_slice()` em ordem; primeira que resolve vence.
- Tests adicionais para `font_list("A","B","C")` com cenários
  de match no índice 0/1/2 e nenhum match.
- ADR-0055 transita de `PROPOSTO → IMPLEMENTADO`.
- Gap 6 do DEBT-52 marcado `[x]`.

Após 141: DEBT-1 pode fechar (gap 7 hyphenation e gap 8 font
dict são opcionais segundo ADR-0054).

---

## Verificação final

| Item | Estado |
|------|--------|
| `first_font_from_doc` + 4 unit tests | ✅ |
| `resolve_font` + 3 unit tests | ✅ |
| Dispatch em `compile_to_pdf_bytes` | ✅ |
| 4 testes L3 `font_wiring_*` | ✅ |
| 27 testes pré-existentes do `pipeline_*` | ✅ verdes |
| `cargo test --workspace --lib` | 1090 passed (excl. 6 ignored pré-existentes); +11 novos |
| `crystalline-lint .` | ✅ zero violations |
| Hash L0/L3 sincronizado | ✅ `367f8790` |
| L1 de domínio intacto | ✅ |
| DEBT-52 gap 5 marcado `[x]` | ✅ |
| ADR-0055 permanece `PROPOSTO` | ✅ (transita em 141) |
| Limitações registadas | ✅ (7 itens) |
