# P1303 — recibo dos testes RED independentes

**Veredito:** `P1303_RED_VALID`

O gate P5 foi materializado no papel **TESTADOR A/B**, em regime de materialização
segregada **executado sem atestação de isolamento técnico**. O filesystem e o
contexto de coordenação eram compartilhados; hashes, allowlist e ordem causal
identificam a execução, mas não provam isolamento técnico.

## Identidade, capacidades e entradas congeladas

- executor: sessão Codex no papel exclusivo `TESTADOR A/B`;
- ambiente: workspace compartilhado
  `/repos/Antigravity/typst-crystalline`;
- HEAD: `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`;
- instante da captura final: `2026-09-04T01:00:15,703792499-03:00`;
- leitura autorizada exercida: `AGENTS.md`, skill
  `tekt-materializacao-segregada` e suas duas referências, passo P1303,
  contrato, oracle/recibo, plano adversarial, owner L0 de testes e
  `01_core/src/compiler/eval/tests.rs`;
- escrita exercida: somente
  `01_core/src/compiler/eval/tests.rs` e este recibo;
- explicitamente não lido nem editado:
  `01_core/src/compiler/eval/bindings/field_access.rs` e qualquer corpo de
  implementação candidata;
- não executados: `crystalline-lint --fix-hashes`, staging ou commit.

Inputs congelados conferidos imediatamente antes da escrita dos testes:

| Input | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1303-contract.md` | `837cb4bd1f38428d93cdadf9216c0182c69e3d8bfa8eb489ed52a710e2bbdf8a` |
| `00_nucleo/diagnosticos/p1303-pre-measurement.json` | `ef3a4eb0b6fcb3fb0e9b1d8ec54bfbfa8f1a4f7b58dd7c675cbf677bada573d4` |
| `00_nucleo/diagnosticos/p1303-oracle-receipt.md` | `f23960b7c5a59f0586fda5378588d52de7546ff8551a5c6787c322af6f652289` |
| `00_nucleo/diagnosticos/p1303-adversarial-plan.md` | `ebaaad08d475a53fc2dc6fe41ea69c901069f302633b7e41ba0f5293031057b8` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `f50afc2f609a738f3fff5e7c511cdc26d8e6c8c3878107862ef0881c0a45a479` |
| `01_core/src/compiler/eval/tests.rs` pré-RED | `6fd1e0087ef29f8b2bb52082d08a61f87a1deabeb6689016721c046ee9c90c07` |

Artefato produzido:

- `01_core/src/compiler/eval/tests.rs` pós-RED:
  `ab57d8c30dd54e1f8d638fcad4ab52e3c2c4bf9c4b3f674816806fe5ba8824b0`;
- diff de `tests.rs`, serializado por
  `git diff -- 01_core/src/compiler/eval/tests.rs`:
  `4fffe9e364130e5c798549619cd352d456b3152227b00909f8d68640bc894893`;
- `cargo fmt --all -- --check`: exit `0`;
- `git diff --check -- 01_core/src/compiler/eval/tests.rs`: exit `0`.

## Assassinos materializados

Foram adicionados exatamente os cinco nomes do plano adversarial:

1. `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y`;
2. `p1303_positivos_pdf_exatos_nos_perfis_com_a11y`;
3. `p1303_sentinelas_de_span_module_e_nao_module`;
4. `p1303_sentinelas_pdf_ungated_e_html_disabled`;
5. `p1303_ordem_repeticao_e_estado_completo`.

O negativo cobre os três fields em `default` e `html`, acumulando por caso
mensagem exata, dois hints exatos e ordenados, cardinalidades primária/lateral
e range half-open exato. Os positivos cobrem `a11y` e `html+a11y`, exigem
kind `function`, nome/`repr` público e chamadas representativas com conteúdo
cru, `table.cell` e `table-summary`. As sentinelas cobrem `Module`,
dicionário, float, lookups existentes, `pdf.attach`, `pdf.artifact` e o gate
`html`. O meta-teste compara o mesmo mapa observável na ordem normal, na
repetição e na ordem invertida.

## Calibração test-only

Uma primeira execução compilável expôs três erros de fixture além do RED:
prefixo incompatível com os ranges congelados, expectativa de range do float
deslocada em um byte e comparações de `repr` de carriers fora do fragmento
P1303. Somente o teste foi corrigido. Produto, L0, contrato e oracle não foram
alterados. Uma segunda execução obteve o vetor RED correto; depois o bloco foi
somente formatado conforme `cargo fmt --check` e o comando exigido foi
repetido. A execução abaixo é a execução final aceita: os quatro controles
passam e resta uma única falha agregada, composta exatamente pelas seis
divergências de span previstas.

## Comando e resultado aceito

Comando integral:

```bash
cargo test -p typst-core p1303 -- --test-threads=1
```

- exit: `101` (RED esperado);
- output combinado stdout+stderr: `4150` linhas, `220980` bytes;
- SHA-256 do output integral:
  `04bfe8688765a02a3795631abdd483969aaa5bbee4475408300ed31350c03572`;
- resumo: `1 failed; 4 passed; 0 ignored; 0 measured; 5431 filtered out`;
- duração da suite filtrada reportada pelo harness: `0.59s`;
- warnings preexistentes não constituem falha do gate; a compilação e execução
  chegaram ao harness dos cinco testes.

Falha única:

`compiler::eval::tests::tests::p1303_negativos_pdf_exatos_nos_perfis_sem_a11y`

Mismatches exatos:

| Perfil/field | Esperado | Observado | Única classe |
|---|---:|---:|---|
| `default/data-cell` | `14..23` | `10..23` | span |
| `default/header-cell` | `14..25` | `10..25` | span |
| `default/table-summary` | `14..27` | `10..27` | span |
| `html/data-cell` | `14..23` | `10..23` | span |
| `html/header-cell` | `14..25` | `10..25` | span |
| `html/table-summary` | `14..27` | `10..27` | span |

A função de comparação registra cada diferença de mensagem, hints,
cardinalidade primária, cardinalidade lateral e span separadamente. O output
aceito contém somente as seis linhas de mismatch de span acima; portanto
mensagem, os dois hints em ordem, `1` erro primário e `0` diagnósticos
laterais coincidiram em todos os seis casos. Os quatro testes de controle
passaram, incluindo os perfis positivos, todas as sentinelas e
ordem/repetição. Não houve `Unknown` nem `EXECUTION_UNKNOWN`.

## Estado exato da árvore na captura

```text
 M 00_nucleo/prompts/compiler/eval/bindings/field_access.md
 M 00_nucleo/prompts/compiler/eval/tests.md
 M 01_core/src/compiler/eval/bindings/field_access.rs
 M 01_core/src/compiler/eval/tests.rs
?? 00_nucleo/diagnosticos/p1303-adversarial-plan.md
?? 00_nucleo/diagnosticos/p1303-baseline-status.txt
?? 00_nucleo/diagnosticos/p1303-contract.md
?? 00_nucleo/diagnosticos/p1303-l0-gate-receipt.md
?? 00_nucleo/diagnosticos/p1303-oracle-receipt.md
?? 00_nucleo/diagnosticos/p1303-pre-measurement.json
?? 00_nucleo/diagnosticos/p1303-red-tests-receipt.md
?? 00_nucleo/materialization/typst-passo-1303.md
```

`git diff HEAD --stat` no mesmo instante:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  80 +++++
 00_nucleo/prompts/compiler/eval/tests.md           | 100 +++++-
 01_core/src/compiler/eval/bindings/field_access.rs |   2 +-
 01_core/src/compiler/eval/tests.rs                 | 382 ++++++++++++++++++++-
 4 files changed, 561 insertions(+), 3 deletions(-)
```

`git diff --cached --stat` estava vazio. As alterações fora dos dois caminhos
graváveis deste papel já estavam presentes e não foram inspecionadas nem
modificadas pelo TESTADOR A/B.

## Output integral aceito

```text
   Compiling typst-core v0.1.0 (/repos/Antigravity/typst-crystalline/01_core)
warning: unused imports: `Pt` and `TextStyle`
    --> 01_core/src/compiler/eval/tests.rs:1999:41
     |
1999 |     use crate::entities::layout_types::{Pt, TextStyle};
     |                                         ^^  ^^^^^^^^^
     |
     = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `crate::entities::value::Value`
    --> 01_core/src/compiler/eval/tests.rs:6244:13
     |
6244 |         use crate::entities::value::Value;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::label::Label`
    --> 01_core/src/compiler/eval/tests.rs:6657:13
     |
6657 |         use crate::entities::label::Label;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::label::Label`
    --> 01_core/src/compiler/eval/tests.rs:6682:13
     |
6682 |         use crate::entities::label::Label;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::contracts::world::World as _`
     --> 01_core/src/compiler/eval/tests.rs:16433:13
      |
16433 |         use crate::contracts::world::World as _;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::label::Label`
    --> 01_core/src/compiler/introspect.rs:3714:13
     |
3714 |         use crate::entities::label::Label;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::layout_types::Length`
    --> 01_core/src/compiler/layout/grid.rs:1511:9
     |
1511 |     use crate::entities::layout_types::Length;
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::content::Content`
  --> 01_core/src/compiler/layout/title.rs:10:5
   |
10 | use crate::entities::content::Content;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::layout_types::Length`
    --> 01_core/src/compiler/layout/tests.rs:1632:9
     |
1632 |     use crate::entities::layout_types::Length;
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::layout_types::Length`
    --> 01_core/src/compiler/layout/tests.rs:1789:9
     |
1789 |     use crate::entities::layout_types::Length;
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::label::Label`
    --> 01_core/src/compiler/layout/tests.rs:2898:9
     |
2898 |     use crate::entities::label::Label;
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::label::Label`
    --> 01_core/src/compiler/layout/tests.rs:2933:9
     |
2933 |     use crate::entities::label::Label;
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::compiler::introspect::introspect_with_introspector`
    --> 01_core/src/compiler/layout/tests.rs:2972:9
     |
2972 |     use crate::compiler::introspect::introspect_with_introspector;
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::label::Label`
    --> 01_core/src/compiler/layout/tests.rs:2973:9
     |
2973 |     use crate::entities::label::Label;
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `introspect`
    --> 01_core/src/compiler/layout/tests.rs:3000:22
     |
3000 |         introspect::{introspect, introspect_with_introspector},
     |                      ^^^^^^^^^^

warning: unused import: `crate::entities::geometry::ShapeKind`
    --> 01_core/src/compiler/layout/tests.rs:4318:9
     |
4318 |     use crate::entities::geometry::ShapeKind;
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::layout_types::Length`
    --> 01_core/src/compiler/layout/tests.rs:7546:13
     |
7546 |         use crate::entities::layout_types::Length;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::layout_types::Length`
     --> 01_core/src/compiler/layout/tests.rs:10994:13
      |
10994 |         use crate::entities::layout_types::Length;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::layout_types::Length`
     --> 01_core/src/compiler/layout/tests.rs:11056:13
      |
11056 |         use crate::entities::layout_types::Length;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `introspect`
     --> 01_core/src/compiler/layout/tests.rs:15302:39
      |
15302 |     use crate::compiler::introspect::{introspect, introspect_with_introspector};
      |                                       ^^^^^^^^^^

warning: unused import: `crate::entities::introspector::Introspector`
     --> 01_core/src/compiler/layout/tests.rs:15303:9
      |
15303 |     use crate::entities::introspector::Introspector;
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::element_kind::ElementKind`
     --> 01_core/src/compiler/layout/tests.rs:15585:9
      |
15585 |     use crate::entities::element_kind::ElementKind;
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::introspector::Introspector`
     --> 01_core/src/compiler/layout/tests.rs:15586:9
      |
15586 |     use crate::entities::introspector::Introspector;
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::state_update::StateUpdate`
     --> 01_core/src/compiler/layout/tests.rs:15745:9
      |
15745 |     use crate::entities::state_update::StateUpdate;
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::value::Value`
     --> 01_core/src/compiler/layout/tests.rs:15746:9
      |
15746 |     use crate::entities::value::Value;
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `TagIntrospector`
     --> 01_core/src/compiler/layout/tests.rs:15869:55
      |
15869 |     use crate::entities::introspector::{Introspector, TagIntrospector};
      |                                                       ^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::state_update::StateUpdate`
     --> 01_core/src/compiler/layout/tests.rs:15979:9
      |
15979 |     use crate::entities::state_update::StateUpdate;
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::value::Value`
     --> 01_core/src/compiler/layout/tests.rs:15980:9
      |
15980 |     use crate::entities::value::Value;
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `Pt` and `TextStyle`
     --> 01_core/src/compiler/layout/tests.rs:18475:52
      |
18475 |     use crate::entities::layout_types::{FrameItem, Pt, TextStyle};
      |                                                    ^^  ^^^^^^^^^

warning: duplicated attribute
     --> 01_core/src/compiler/layout/tests.rs:19009:5
      |
19009 |     #[test]
      |     ^^^^^^^
      |
      = note: `#[warn(duplicate_macro_attributes)]` on by default

warning: duplicated attribute
     --> 01_core/src/compiler/layout/tests.rs:19010:5
      |
19010 |     #[test]
      |     ^^^^^^^

warning: unused import: `crate::entities::label::Label`
     --> 01_core/src/compiler/layout/tests.rs:18987:9
      |
18987 |     use crate::entities::label::Label;
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ecow::EcoString`
 --> 01_core/src/compiler/math/layout/matrix.rs:9:5
  |
9 | use ecow::EcoString;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `crate::compiler::layout::vanilla_defaults`
  --> 01_core/src/compiler/math/layout/matrix.rs:11:5
   |
11 | use crate::compiler::layout::vanilla_defaults;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::compiler::math::layout::symbols`
  --> 01_core/src/compiler/math/layout/matrix.rs:12:5
   |
12 | use crate::compiler::math::layout::symbols;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `Color`
  --> 01_core/src/compiler/math/layout/matrix.rs:15:37
   |
15 | use crate::entities::layout_types::{Color, FrameItem, Length, Point, Pt, TextStyle};
   |                                     ^^^^^

warning: unused import: `TrackedMut`
   --> 01_core/src/compiler/stdlib/mod.rs:276:25
    |
276 |     use comemo::{Track, TrackedMut};
    |                         ^^^^^^^^^^

warning: unused import: `ShapeKind`
    --> 01_core/src/compiler/stdlib/mod.rs:4885:51
     |
4885 |         use crate::entities::geometry::{PathItem, ShapeKind};
     |                                                   ^^^^^^^^^

warning: unused import: `ShapeKind`
    --> 01_core/src/compiler/stdlib/mod.rs:4915:51
     |
4915 |         use crate::entities::geometry::{PathItem, ShapeKind};
     |                                                   ^^^^^^^^^

warning: unused import: `ShapeKind`
    --> 01_core/src/compiler/stdlib/mod.rs:5151:51
     |
5151 |         use crate::entities::geometry::{PathItem, ShapeKind};
     |                                                   ^^^^^^^^^

warning: unused import: `ShapeKind`
    --> 01_core/src/compiler/stdlib/mod.rs:5185:51
     |
5185 |         use crate::entities::geometry::{PathItem, ShapeKind};
     |                                                   ^^^^^^^^^

warning: unused import: `ShapeKind`
    --> 01_core/src/compiler/stdlib/mod.rs:5221:51
     |
5221 |         use crate::entities::geometry::{PathItem, ShapeKind};
     |                                                   ^^^^^^^^^

warning: unused import: `ShapeKind`
    --> 01_core/src/compiler/stdlib/mod.rs:5294:51
     |
5294 |         use crate::entities::geometry::{PathItem, ShapeKind};
     |                                                   ^^^^^^^^^

warning: unused import: `ShapeKind`
    --> 01_core/src/compiler/stdlib/mod.rs:5331:51
     |
5331 |         use crate::entities::geometry::{PathItem, ShapeKind};
     |                                                   ^^^^^^^^^

warning: unused import: `ShapeKind`
    --> 01_core/src/compiler/stdlib/mod.rs:5371:51
     |
5371 |         use crate::entities::geometry::{PathItem, ShapeKind};
     |                                                   ^^^^^^^^^

warning: unused import: `ShapeKind`
    --> 01_core/src/compiler/stdlib/mod.rs:5410:51
     |
5410 |         use crate::entities::geometry::{PathItem, ShapeKind};
     |                                                   ^^^^^^^^^

warning: unused import: `crate::entities::span::Span`
  --> 01_core/src/entities/ast/expr.rs:16:5
   |
16 | use crate::entities::span::Span;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::bib_store::BibStore`
    --> 01_core/src/entities/introspector.rs:1573:13
     |
1573 |         use crate::entities::bib_store::BibStore;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::lang::Lang`
   --> 01_core/src/entities/style_chain.rs:861:13
    |
861 |         use crate::entities::lang::Lang;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::entities::lang::Lang`
    --> 01_core/src/entities/style_chain.rs:1028:13
     |
1028 |         use crate::entities::lang::Lang;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/eval/tests.rs:6299:70
     |
6299 |                     if let crate::entities::layout_types::FrameItem::Text {
     |                                                                      ^^^^
     |
     = note: `#[warn(deprecated)]` on by default

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/mod.rs:1320:35
     |
1320 |             items.push(FrameItem::Text {
     |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/mod.rs:2245:56
     |
2245 |                         marginal_items.push(FrameItem::Text {
     |                                                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/mod.rs:2334:53
     |
2334 |                     page.foreground.push(FrameItem::Text {
     |                                                     ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/metrics.rs:275:32
    |
275 |                     FrameItem::Text { pos, .. }
    |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/metrics.rs:289:32
    |
289 |                     FrameItem::Text { text, style, .. } => {
    |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/grid.rs:855:40
    |
855 | ...                   FrameItem::Text { .. } | FrameItem::TextShaped { .. }
    |                                  ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/sub_frame.rs:365:28
    |
365 |                 FrameItem::Text { .. } | FrameItem::TextShaped { .. } => {
    |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/boxed.rs:316:24
    |
316 |             FrameItem::Text { pos, .. }
    |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
  --> 01_core/src/compiler/layout/stack.rs:35:24
   |
35 |             FrameItem::Text { pos, text, style } => {
   |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/stack.rs:237:32
    |
237 |                     FrameItem::Text { pos, .. }
    |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/transform.rs:162:28
    |
162 |                 FrameItem::Text { pos, .. }
    |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/transform.rs:175:28
    |
175 |                 FrameItem::Text { pos, .. }
    |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/transform.rs:188:28
    |
188 |                 FrameItem::Text { pos, .. }
    |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/transform.rs:230:28
    |
230 |                 FrameItem::Text { style, .. } | FrameItem::TextShaped { style, .. } => {
    |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
  --> 01_core/src/compiler/layout/linebreak.rs:33:36
   |
33 |             items: vec![FrameItem::Text {
   |                                    ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
  --> 01_core/src/compiler/layout/parbreak.rs:32:32
   |
32 |         items: vec![FrameItem::Text {
   |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
  --> 01_core/src/compiler/layout/tests.rs:52:24
   |
52 |             FrameItem::Text { pos, .. }
   |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:422:24
    |
422 |             FrameItem::Text { pos, text, style }
    |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1003:24
     |
1003 |             FrameItem::Text { text, .. } => text.contains(needle),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1031:28
     |
1031 |                 FrameItem::Text { text, .. } => out.push_str(text),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1064:31
     |
1064 |             if let FrameItem::Text { pos, .. } = item {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1090:31
     |
1090 |             if let FrameItem::Text { pos, .. } = i {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1151:41
     |
1151 |         .any(|i| matches!(i, FrameItem::Text { style, .. } if style.bold));
     |                                         ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1164:41
     |
1164 |         .any(|i| matches!(i, FrameItem::Text { style, .. } if style.italic));
     |                                         ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1180:31
     |
1180 |             if let FrameItem::Text { style, .. } = i {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1199:28
     |
1199 |     if let Some(FrameItem::Text { style, text, .. }) = items.last() {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1216:31
     |
1216 |             if let FrameItem::Text { text, style, .. } = i {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1240:31
     |
1240 |             if let FrameItem::Text { text, style, .. } = i {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1263:28
     |
1263 |     if let Some(FrameItem::Text { style, text, .. }) = items.last() {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1287:31
     |
1287 |             if let FrameItem::Text { text, pos, style } = i {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1316:31
     |
1316 |             if let FrameItem::Text { text, pos, style } = i {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1347:24
     |
1347 |             FrameItem::Text { text, .. },
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1513:41
     |
1513 |         .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "•"));
     |                                         ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1533:44
     |
1533 |         .filter(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "Um"))
     |                                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1537:20
     |
1537 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                    ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1560:44
     |
1560 |         .filter(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "Um"))
     |                                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1564:20
     |
1564 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                    ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1597:42
     |
1597 |         .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "→"));
     |                                          ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1600:20
     |
1600 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                    ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1616:44
     |
1616 |         .filter(|i| matches!(i, FrameItem::Text { text, .. } if !text.as_str().trim().is_empty() && text.as_str() != "→"))
     |                                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1621:20
     |
1621 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                    ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1645:24
     |
1645 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1680:24
     |
1680 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1755:42
     |
1755 |         .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "1."));
     |                                          ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1758:20
     |
1758 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                    ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1773:44
     |
1773 |         .filter(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str().contains("Primeiro")))
     |                                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1778:20
     |
1778 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                    ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1816:24
     |
1816 |             FrameItem::Text { text, pos, .. } if matches!(text.as_str(), "1." | "2.") => {
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1850:24
     |
1850 |             FrameItem::Text { text, pos, .. } if matches!(text.as_str(), "1." | "2.") => {
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1882:24
     |
1882 |             FrameItem::Text { style, .. } => Some(style.size.val().to_bits()),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1908:24
     |
1908 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1939:24
     |
1939 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1977:24
     |
1977 |             FrameItem::Text { text, pos, .. } if text.as_str().ends_with('.') => {
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2019:24
     |
2019 |             FrameItem::Text { text, .. } if text.as_str().ends_with('.') => {
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2044:24
     |
2044 |             FrameItem::Text { text, .. } if text.as_str().ends_with('.') => {
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2066:24
     |
2066 |             FrameItem::Text { text, pos, .. }
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2099:24
     |
2099 |             FrameItem::Text { text, pos, .. }
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2133:24
     |
2133 |             FrameItem::Text { text, pos, .. }
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:3118:28
     |
3118 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "(1)" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:3149:28
     |
3149 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "(1)" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:3179:28
     |
3179 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "(1)" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4219:42
     |
4219 |         .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "Normal"))
     |                                          ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4221:23
     |
4221 |     if let FrameItem::Text { style, .. } = item_norm {
     |                       ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4239:42
     |
4239 |         .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "Italic"))
     |                                          ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4241:23
     |
4241 |     if let FrameItem::Text { style, .. } = item_styled {
     |                       ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4687:24
     |
4687 |             FrameItem::Text { pos, .. } => Some(pos.x.val()),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4725:28
     |
4725 |                 FrameItem::Text { pos, .. } => Some((pos.x.val(), pos.y.val())),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4785:24
     |
4785 |             FrameItem::Text { pos, .. } => Some(pos.x.val()),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4875:24
     |
4875 |             FrameItem::Text { pos, .. } => Some(pos.x.val()),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5120:24
     |
5120 |             FrameItem::Text { pos, .. } => Some(pos.x.val()),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19363:24
      |
19363 |             FrameItem::Text { pos, .. } => Some(pos.y.val().to_bits()),
      |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19376:31
      |
19376 |             if let FrameItem::Text { pos, text, .. } = item {
      |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19412:24
      |
19412 |             FrameItem::Text { pos, .. } => Some(pos.y.val().to_bits()),
      |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19425:31
      |
19425 |             if let FrameItem::Text { pos, text, .. } = item {
      |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:20182:24
      |
20182 |             FrameItem::Text { text, style, .. } if text.as_str() == "placed" => {
      |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:219:28
    |
219 |                 FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
    |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:243:28
    |
243 |                 FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
    |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:270:28
    |
270 |                 FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
    |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2276:28
     |
2276 |                 FrameItem::Text { pos, text, .. } if text.contains(needle) => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2298:28
     |
2298 |                 FrameItem::Text { pos, text, .. } if text.contains(needle) => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2382:28
     |
2382 |                 FrameItem::Text { pos, .. } => Some(pos.y.val()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2427:28
     |
2427 |                 FrameItem::Text { pos, .. } => Some(pos.y.val()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2519:59
     |
2519 |                 crate::entities::layout_types::FrameItem::Text { pos, .. } => {
     |                                                           ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5251:54
     |
5251 |             .filter(|item| matches!(item, FrameItem::Text { .. }))
     |                                                      ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5272:31
     |
5272 |             if let FrameItem::Text { style, .. } = item {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5297:31
     |
5297 |             if let FrameItem::Text { style, .. } = item {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5324:40
     |
5324 |             |i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "STYLED"),
     |                                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5327:40
     |
5327 |             |i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "plain"),
     |                                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5333:32
     |
5333 |         if let Some(FrameItem::Text { style, .. }) = styled_item {
     |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5336:32
     |
5336 |         if let Some(FrameItem::Text { style, .. }) = plain_item {
     |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5424:28
     |
5424 |                 FrameItem::Text { text, style, .. } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5570:28
     |
5570 |                 FrameItem::Text { text, style, pos } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5691:28
     |
5691 |                 FrameItem::Text { text, style, pos } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5829:28
     |
5829 |                 FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5860:28
     |
5860 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5870:28
     |
5870 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5887:31
     |
5887 |             if let FrameItem::Text { text, pos, .. } = it {
     |                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5921:28
     |
5921 |                 FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5947:24
     |
5947 |             FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5970:28
     |
5970 |                 FrameItem::Text { pos, .. } => Some(pos.x.0),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5995:24
     |
5995 |             FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6002:24
     |
6002 |             FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6046:47
     |
6046 |                     matches!(item, FrameItem::Text { text, style, .. }
     |                                               ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6074:36
     |
6074 |                         FrameItem::Text { text, .. } => Some(text.as_str()),
     |                                    ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6102:28
     |
6102 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6130:28
     |
6130 |                 FrameItem::Text { pos, .. } => Some(pos.x.0),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6153:28
     |
6153 |                 FrameItem::Text { pos, .. } => Some(pos.x.0),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6175:32
     |
6175 |                     FrameItem::Text { text, style, .. } if text.as_str() == needle => {
     |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6290:28
     |
6290 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6344:28
     |
6344 |                 FrameItem::Text { text, style, .. } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6483:28
     |
6483 |                 FrameItem::Text { pos, .. } => Some(pos.y.val()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6500:28
     |
6500 |                 FrameItem::Text { pos, .. } => Some(pos.y.val()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6524:54
     |
6524 |             .filter(|item| matches!(item, FrameItem::Text { .. }))
     |                                                      ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6550:28
     |
6550 |                 FrameItem::Text { pos, text, .. } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6583:28
     |
6583 |                 FrameItem::Text { pos, text, .. } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6610:28
     |
6610 |                 FrameItem::Text { pos, text, .. } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6724:35
     |
6724 |                 if let FrameItem::Text { text, .. } = item {
     |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6843:28
     |
6843 |                 FrameItem::Text { pos, text, .. } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6892:28
     |
6892 |                 FrameItem::Text { pos, text, .. } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6933:28
     |
6933 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "M" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6957:28
     |
6957 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "M" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6986:28
     |
6986 |                 FrameItem::Text { pos, text, .. } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7013:28
     |
7013 |                 FrameItem::Text { pos, text, .. } => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7060:28
     |
7060 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7083:28
     |
7083 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7128:28
     |
7128 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7157:28
     |
7157 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7187:54
     |
7187 |             .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "X"))
     |                                                      ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7218:28
     |
7218 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7247:28
     |
7247 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7271:28
     |
7271 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7288:28
     |
7288 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7307:28
     |
7307 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7325:28
     |
7325 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7343:28
     |
7343 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7366:28
     |
7366 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7395:28
     |
7395 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7417:28
     |
7417 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7471:28
     |
7471 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "p220before" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7480:28
     |
7480 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "p220after" => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7520:28
     |
7520 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => Some(pos),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7569:28
     |
7569 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => Some(pos),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7616:28
     |
7616 |                 FrameItem::Text { text, pos, .. } if pos.y.0 > 400.0 => {
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7643:24
     |
7643 |             FrameItem::Text { text, .. } => text.as_str() == "Nota",
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7719:28
     |
7719 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7750:28
     |
7750 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7789:28
     |
7789 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7824:28
     |
7824 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:8403:28
     |
8403 |                 FrameItem::Text { text, .. } if text.as_str().contains("ZorderTest")
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:8977:32
     |
8977 |                     FrameItem::Text { text, .. } => out.push_str(text.as_str()),
     |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9020:28
     |
9020 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9180:35
     |
9180 |                 if let FrameItem::Text { text, .. } = item {
     |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9219:35
     |
9219 |                 if let FrameItem::Text { text, .. } = item {
     |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9254:35
     |
9254 |                 if let FrameItem::Text { text, .. } = item {
     |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9403:35
     |
9403 |                 if let FrameItem::Text { text, .. } = item {
     |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9715:35
     |
9715 |                 if let FrameItem::Text { text, .. } = item {
     |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10364:35
      |
10364 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10725:35
      |
10725 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10802:35
      |
10802 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10831:35
      |
10831 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10859:35
      |
10859 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10903:35
      |
10903 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10942:35
      |
10942 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10978:35
      |
10978 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11062:35
      |
11062 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11092:35
      |
11092 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11133:35
      |
11133 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11186:35
      |
11186 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11226:35
      |
11226 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11272:32
      |
11272 |                     FrameItem::Text { text, .. } => out.push_str(text.as_str()),
      |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11443:35
      |
11443 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11497:35
      |
11497 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11515:35
      |
11515 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11761:35
      |
11761 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12087:32
      |
12087 |                     FrameItem::Text { pos, text, .. }
      |                                ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12354:28
      |
12354 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12402:28
      |
12402 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12452:28
      |
12452 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12500:28
      |
12500 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12548:28
      |
12548 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13542:58
      |
13542 |                 .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == label))
      |                                                          ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13586:28
      |
13586 |                 FrameItem::Text { text, pos, .. } => {
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13601:28
      |
13601 |                 FrameItem::Text { text, pos, .. } => {
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13631:54
      |
13631 |             .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "X"))
      |                                                      ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13657:58
      |
13657 |                 .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == label))
      |                                                          ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13677:54
      |
13677 |             .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "HDR"))
      |                                                      ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13689:54
      |
13689 |             .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "FTR"))
      |                                                      ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13715:58
      |
13715 |                 .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == label))
      |                                                          ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:14351:28
      |
14351 |                 FrameItem::Text { pos, .. } => Some(pos.y.0.round() as i64),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:14456:28
      |
14456 |                 FrameItem::Text { pos, .. } | FrameItem::TextShaped { pos, .. } => {
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:14476:28
      |
14476 |                 FrameItem::Text { pos, .. } | FrameItem::TextShaped { pos, .. } => {
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:17234:35
      |
17234 |                 if let FrameItem::Text { text, .. } = item {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:17411:35
      |
17411 |                 if let FrameItem::Text { text, .. } = i {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:17975:28
      |
17975 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:17996:24
      |
17996 |             FrameItem::Text { pos, text, .. } if text.contains("RODAPE") => Some(pos.y.0),
      |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18020:24
      |
18020 |             FrameItem::Text { pos, text, .. } if text == "1" => Some(pos.y.0),
      |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18024:24
      |
18024 |             FrameItem::Text { pos, text, .. } if text.contains("RODAPEB") => {
      |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18052:24
      |
18052 |             FrameItem::Text { pos, text, .. } if text.contains("AAAA") => Some(pos.y.0),
      |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18056:24
      |
18056 |             FrameItem::Text { pos, text, .. } if text.contains("BBBB") => Some(pos.y.0),
      |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18060:24
      |
18060 |             FrameItem::Text { pos, text, .. } if text.contains("CCCC") => Some(pos.y.0),
      |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18095:28
      |
18095 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18123:28
      |
18123 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18157:28
      |
18157 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18188:28
      |
18188 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18213:28
      |
18213 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18233:28
      |
18233 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18257:28
      |
18257 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18283:28
      |
18283 |                 FrameItem::Text { pos, text, .. } if text.contains("wordSentinel") => {
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18361:28
      |
18361 |                 FrameItem::Text { pos, text, .. } if text.contains("nota de rodapé") => {
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18422:28
      |
18422 |                 FrameItem::Text { pos, .. } => Some(pos.x.0),
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18486:45
      |
18486 |             .any(|i| matches!(i, FrameItem::Text { style, .. } if style.bold))
      |                                             ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18493:45
      |
18493 |             .any(|i| matches!(i, FrameItem::Text { style, .. } if style.italic))
      |                                             ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18897:35
      |
18897 |                 if let FrameItem::Text { style, text, .. } = i {
      |                                   ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18958:49
      |
18958 |                 .any(|i| matches!(i, FrameItem::Text { text: t, .. } if t == "Marcado"))
      |                                                 ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19503:28
      |
19503 |                 FrameItem::Text { pos, text, .. }
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19519:28
      |
19519 |                 FrameItem::Text { pos, text, .. }
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21103:28
      |
21103 |                 FrameItem::Text { pos, text, style, .. }
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21197:28
      |
21197 |                 FrameItem::Text { text, style, .. }
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21385:28
      |
21385 |                 FrameItem::Text { pos, text, style, .. } if text.as_str() == needle => {
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21423:28
      |
21423 |                 FrameItem::Text { pos, text, .. } if text.as_str().contains("dado") => {
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:22028:28
      |
22028 |                 FrameItem::Text { pos, text, .. }
      |                            ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/entities/layout_types.rs:1877:24
     |
1877 |             FrameItem::Text { .. } => self.visit_text(item),
     |                        ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/entities/layout_types.rs:1376:27
     |
1376 |         f.push(FrameItem::Text {
     |                           ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/entities/layout_types.rs:1381:27
     |
1381 |         f.push(FrameItem::Text {
     |                           ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/entities/layout_types.rs:1401:36
     |
1401 |             items: vec![FrameItem::Text {
     |                                    ^^^^

warning: use of deprecated variant `entities::layout_types::FrameItem::Text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/entities/layout_types.rs:1416:36
     |
1416 |             items: vec![FrameItem::Text {
     |                                    ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/eval/tests.rs:6300:25
     |
6300 |                         text, ..
     |                         ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/metrics.rs:275:39
    |
275 |                     FrameItem::Text { pos, .. }
    |                                       ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/metrics.rs:289:39
    |
289 |                     FrameItem::Text { text, style, .. } => {
    |                                       ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/metrics.rs:289:45
    |
289 |                     FrameItem::Text { text, style, .. } => {
    |                                             ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/boxed.rs:316:31
    |
316 |             FrameItem::Text { pos, .. }
    |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
  --> 01_core/src/compiler/layout/stack.rs:35:31
   |
35 |             FrameItem::Text { pos, text, style } => {
   |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
  --> 01_core/src/compiler/layout/stack.rs:35:36
   |
35 |             FrameItem::Text { pos, text, style } => {
   |                                    ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
  --> 01_core/src/compiler/layout/stack.rs:35:42
   |
35 |             FrameItem::Text { pos, text, style } => {
   |                                          ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/stack.rs:237:39
    |
237 |                     FrameItem::Text { pos, .. }
    |                                       ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/transform.rs:162:35
    |
162 |                 FrameItem::Text { pos, .. }
    |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/transform.rs:175:35
    |
175 |                 FrameItem::Text { pos, .. }
    |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/transform.rs:188:35
    |
188 |                 FrameItem::Text { pos, .. }
    |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/transform.rs:230:35
    |
230 |                 FrameItem::Text { style, .. } | FrameItem::TextShaped { style, .. } => {
    |                                   ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
  --> 01_core/src/compiler/layout/tests.rs:52:31
   |
52 |             FrameItem::Text { pos, .. }
   |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:219:35
    |
219 |                 FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
    |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:243:35
    |
243 |                 FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
    |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:270:35
    |
270 |                 FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
    |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:422:31
    |
422 |             FrameItem::Text { pos, text, style }
    |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:422:36
    |
422 |             FrameItem::Text { pos, text, style }
    |                                    ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
   --> 01_core/src/compiler/layout/tests.rs:422:42
    |
422 |             FrameItem::Text { pos, text, style }
    |                                          ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1003:31
     |
1003 |             FrameItem::Text { text, .. } => text.contains(needle),
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1031:35
     |
1031 |                 FrameItem::Text { text, .. } => out.push_str(text),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1064:38
     |
1064 |             if let FrameItem::Text { pos, .. } = item {
     |                                      ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1090:38
     |
1090 |             if let FrameItem::Text { pos, .. } = i {
     |                                      ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1151:48
     |
1151 |         .any(|i| matches!(i, FrameItem::Text { style, .. } if style.bold));
     |                                                ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1164:48
     |
1164 |         .any(|i| matches!(i, FrameItem::Text { style, .. } if style.italic));
     |                                                ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1180:38
     |
1180 |             if let FrameItem::Text { style, .. } = i {
     |                                      ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1199:35
     |
1199 |     if let Some(FrameItem::Text { style, text, .. }) = items.last() {
     |                                   ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1199:42
     |
1199 |     if let Some(FrameItem::Text { style, text, .. }) = items.last() {
     |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1216:38
     |
1216 |             if let FrameItem::Text { text, style, .. } = i {
     |                                      ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1216:44
     |
1216 |             if let FrameItem::Text { text, style, .. } = i {
     |                                            ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1240:38
     |
1240 |             if let FrameItem::Text { text, style, .. } = i {
     |                                      ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1240:44
     |
1240 |             if let FrameItem::Text { text, style, .. } = i {
     |                                            ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1263:35
     |
1263 |     if let Some(FrameItem::Text { style, text, .. }) = items.last() {
     |                                   ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1263:42
     |
1263 |     if let Some(FrameItem::Text { style, text, .. }) = items.last() {
     |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1287:38
     |
1287 |             if let FrameItem::Text { text, pos, style } = i {
     |                                      ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1287:44
     |
1287 |             if let FrameItem::Text { text, pos, style } = i {
     |                                            ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1287:49
     |
1287 |             if let FrameItem::Text { text, pos, style } = i {
     |                                                 ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1316:38
     |
1316 |             if let FrameItem::Text { text, pos, style } = i {
     |                                      ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1316:44
     |
1316 |             if let FrameItem::Text { text, pos, style } = i {
     |                                            ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1316:49
     |
1316 |             if let FrameItem::Text { text, pos, style } = i {
     |                                                 ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1347:31
     |
1347 |             FrameItem::Text { text, .. },
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1513:48
     |
1513 |         .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "•"));
     |                                                ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1533:51
     |
1533 |         .filter(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "Um"))
     |                                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1537:27
     |
1537 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                           ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1560:51
     |
1560 |         .filter(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "Um"))
     |                                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1564:27
     |
1564 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                           ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1597:49
     |
1597 |         .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "→"));
     |                                                 ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1600:27
     |
1600 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                           ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1616:51
     |
1616 |         .filter(|i| matches!(i, FrameItem::Text { text, .. } if !text.as_str().trim().is_empty() && text.as_str() != "→"))
     |                                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1621:27
     |
1621 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                           ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1645:31
     |
1645 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1645:37
     |
1645 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1680:31
     |
1680 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1680:37
     |
1680 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1755:49
     |
1755 |         .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "1."));
     |                                                 ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1758:27
     |
1758 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                           ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1773:51
     |
1773 |         .filter(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str().contains("Primeiro")))
     |                                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1778:27
     |
1778 |         FrameItem::Text { pos, .. } => pos.x.val(),
     |                           ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1816:31
     |
1816 |             FrameItem::Text { text, pos, .. } if matches!(text.as_str(), "1." | "2.") => {
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1816:37
     |
1816 |             FrameItem::Text { text, pos, .. } if matches!(text.as_str(), "1." | "2.") => {
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1850:31
     |
1850 |             FrameItem::Text { text, pos, .. } if matches!(text.as_str(), "1." | "2.") => {
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1850:37
     |
1850 |             FrameItem::Text { text, pos, .. } if matches!(text.as_str(), "1." | "2.") => {
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1882:31
     |
1882 |             FrameItem::Text { style, .. } => Some(style.size.val().to_bits()),
     |                               ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1908:31
     |
1908 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1908:37
     |
1908 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1939:31
     |
1939 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1939:37
     |
1939 |             FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1977:31
     |
1977 |             FrameItem::Text { text, pos, .. } if text.as_str().ends_with('.') => {
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:1977:37
     |
1977 |             FrameItem::Text { text, pos, .. } if text.as_str().ends_with('.') => {
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2019:31
     |
2019 |             FrameItem::Text { text, .. } if text.as_str().ends_with('.') => {
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2044:31
     |
2044 |             FrameItem::Text { text, .. } if text.as_str().ends_with('.') => {
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2066:31
     |
2066 |             FrameItem::Text { text, pos, .. }
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2066:37
     |
2066 |             FrameItem::Text { text, pos, .. }
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2099:31
     |
2099 |             FrameItem::Text { text, pos, .. }
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2099:37
     |
2099 |             FrameItem::Text { text, pos, .. }
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2133:31
     |
2133 |             FrameItem::Text { text, pos, .. }
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2133:37
     |
2133 |             FrameItem::Text { text, pos, .. }
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2276:35
     |
2276 |                 FrameItem::Text { pos, text, .. } if text.contains(needle) => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2276:40
     |
2276 |                 FrameItem::Text { pos, text, .. } if text.contains(needle) => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2298:35
     |
2298 |                 FrameItem::Text { pos, text, .. } if text.contains(needle) => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2298:40
     |
2298 |                 FrameItem::Text { pos, text, .. } if text.contains(needle) => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2382:35
     |
2382 |                 FrameItem::Text { pos, .. } => Some(pos.y.val()),
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2427:35
     |
2427 |                 FrameItem::Text { pos, .. } => Some(pos.y.val()),
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:2519:66
     |
2519 |                 crate::entities::layout_types::FrameItem::Text { pos, .. } => {
     |                                                                  ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:3118:35
     |
3118 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "(1)" => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:3118:40
     |
3118 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "(1)" => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:3149:35
     |
3149 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "(1)" => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:3149:40
     |
3149 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "(1)" => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:3179:35
     |
3179 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "(1)" => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:3179:40
     |
3179 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "(1)" => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4219:49
     |
4219 |         .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "Normal"))
     |                                                 ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4221:30
     |
4221 |     if let FrameItem::Text { style, .. } = item_norm {
     |                              ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4239:49
     |
4239 |         .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "Italic"))
     |                                                 ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4241:30
     |
4241 |     if let FrameItem::Text { style, .. } = item_styled {
     |                              ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4687:31
     |
4687 |             FrameItem::Text { pos, .. } => Some(pos.x.val()),
     |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4725:35
     |
4725 |                 FrameItem::Text { pos, .. } => Some((pos.x.val(), pos.y.val())),
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4785:31
     |
4785 |             FrameItem::Text { pos, .. } => Some(pos.x.val()),
     |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:4875:31
     |
4875 |             FrameItem::Text { pos, .. } => Some(pos.x.val()),
     |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5120:31
     |
5120 |             FrameItem::Text { pos, .. } => Some(pos.x.val()),
     |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5272:38
     |
5272 |             if let FrameItem::Text { style, .. } = item {
     |                                      ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5297:38
     |
5297 |             if let FrameItem::Text { style, .. } = item {
     |                                      ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5324:47
     |
5324 |             |i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "STYLED"),
     |                                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5327:47
     |
5327 |             |i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "plain"),
     |                                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5333:39
     |
5333 |         if let Some(FrameItem::Text { style, .. }) = styled_item {
     |                                       ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5336:39
     |
5336 |         if let Some(FrameItem::Text { style, .. }) = plain_item {
     |                                       ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5424:35
     |
5424 |                 FrameItem::Text { text, style, .. } => {
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5424:41
     |
5424 |                 FrameItem::Text { text, style, .. } => {
     |                                         ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5570:35
     |
5570 |                 FrameItem::Text { text, style, pos } => {
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5570:41
     |
5570 |                 FrameItem::Text { text, style, pos } => {
     |                                         ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5570:48
     |
5570 |                 FrameItem::Text { text, style, pos } => {
     |                                                ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5691:35
     |
5691 |                 FrameItem::Text { text, style, pos } => {
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5691:41
     |
5691 |                 FrameItem::Text { text, style, pos } => {
     |                                         ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5691:48
     |
5691 |                 FrameItem::Text { text, style, pos } => {
     |                                                ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5829:35
     |
5829 |                 FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5829:41
     |
5829 |                 FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5860:35
     |
5860 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => {
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5860:41
     |
5860 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => {
     |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5870:35
     |
5870 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => {
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5870:41
     |
5870 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => {
     |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5887:38
     |
5887 |             if let FrameItem::Text { text, pos, .. } = it {
     |                                      ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5887:44
     |
5887 |             if let FrameItem::Text { text, pos, .. } = it {
     |                                            ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5921:35
     |
5921 |                 FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5921:41
     |
5921 |                 FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5947:31
     |
5947 |             FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5947:37
     |
5947 |             FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5970:35
     |
5970 |                 FrameItem::Text { pos, .. } => Some(pos.x.0),
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5995:31
     |
5995 |             FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:5995:37
     |
5995 |             FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6002:31
     |
6002 |             FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6002:37
     |
6002 |             FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
     |                                     ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6046:54
     |
6046 |                     matches!(item, FrameItem::Text { text, style, .. }
     |                                                      ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6046:60
     |
6046 |                     matches!(item, FrameItem::Text { text, style, .. }
     |                                                            ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6074:43
     |
6074 |                         FrameItem::Text { text, .. } => Some(text.as_str()),
     |                                           ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6102:35
     |
6102 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6130:35
     |
6130 |                 FrameItem::Text { pos, .. } => Some(pos.x.0),
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6153:35
     |
6153 |                 FrameItem::Text { pos, .. } => Some(pos.x.0),
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6175:39
     |
6175 |                     FrameItem::Text { text, style, .. } if text.as_str() == needle => {
     |                                       ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6175:45
     |
6175 |                     FrameItem::Text { text, style, .. } if text.as_str() == needle => {
     |                                             ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6290:35
     |
6290 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6344:35
     |
6344 |                 FrameItem::Text { text, style, .. } => {
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6344:41
     |
6344 |                 FrameItem::Text { text, style, .. } => {
     |                                         ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6483:35
     |
6483 |                 FrameItem::Text { pos, .. } => Some(pos.y.val()),
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6500:35
     |
6500 |                 FrameItem::Text { pos, .. } => Some(pos.y.val()),
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6550:35
     |
6550 |                 FrameItem::Text { pos, text, .. } => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6550:40
     |
6550 |                 FrameItem::Text { pos, text, .. } => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6583:35
     |
6583 |                 FrameItem::Text { pos, text, .. } => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6583:40
     |
6583 |                 FrameItem::Text { pos, text, .. } => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6610:35
     |
6610 |                 FrameItem::Text { pos, text, .. } => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6610:40
     |
6610 |                 FrameItem::Text { pos, text, .. } => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6724:42
     |
6724 |                 if let FrameItem::Text { text, .. } = item {
     |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6843:35
     |
6843 |                 FrameItem::Text { pos, text, .. } => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6843:40
     |
6843 |                 FrameItem::Text { pos, text, .. } => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6892:35
     |
6892 |                 FrameItem::Text { pos, text, .. } => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6892:40
     |
6892 |                 FrameItem::Text { pos, text, .. } => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6933:35
     |
6933 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "M" => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6933:40
     |
6933 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "M" => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6957:35
     |
6957 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "M" => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6957:40
     |
6957 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "M" => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6986:35
     |
6986 |                 FrameItem::Text { pos, text, .. } => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:6986:40
     |
6986 |                 FrameItem::Text { pos, text, .. } => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7013:35
     |
7013 |                 FrameItem::Text { pos, text, .. } => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7013:40
     |
7013 |                 FrameItem::Text { pos, text, .. } => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7060:35
     |
7060 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7060:40
     |
7060 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7083:35
     |
7083 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7083:40
     |
7083 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7128:35
     |
7128 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7128:40
     |
7128 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7157:35
     |
7157 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7157:40
     |
7157 |                 FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
     |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7187:61
     |
7187 |             .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "X"))
     |                                                             ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7218:35
     |
7218 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7247:35
     |
7247 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7271:35
     |
7271 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7288:35
     |
7288 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7307:35
     |
7307 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7325:35
     |
7325 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7343:35
     |
7343 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7366:35
     |
7366 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7395:35
     |
7395 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7417:35
     |
7417 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7471:35
     |
7471 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "p220before" => {
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7471:41
     |
7471 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "p220before" => {
     |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7480:35
     |
7480 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "p220after" => {
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7480:41
     |
7480 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "p220after" => {
     |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7520:35
     |
7520 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => Some(pos),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7520:41
     |
7520 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => Some(pos),
     |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7569:35
     |
7569 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => Some(pos),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7569:41
     |
7569 |                 FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => Some(pos),
     |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7616:35
     |
7616 |                 FrameItem::Text { text, pos, .. } if pos.y.0 > 400.0 => {
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7616:41
     |
7616 |                 FrameItem::Text { text, pos, .. } if pos.y.0 > 400.0 => {
     |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7643:31
     |
7643 |             FrameItem::Text { text, .. } => text.as_str() == "Nota",
     |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7719:35
     |
7719 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7750:35
     |
7750 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7789:35
     |
7789 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:7824:35
     |
7824 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:8403:35
     |
8403 |                 FrameItem::Text { text, .. } if text.as_str().contains("ZorderTest")
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:8977:39
     |
8977 |                     FrameItem::Text { text, .. } => out.push_str(text.as_str()),
     |                                       ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9020:35
     |
9020 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
     |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9180:42
     |
9180 |                 if let FrameItem::Text { text, .. } = item {
     |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9219:42
     |
9219 |                 if let FrameItem::Text { text, .. } = item {
     |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9254:42
     |
9254 |                 if let FrameItem::Text { text, .. } = item {
     |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9403:42
     |
9403 |                 if let FrameItem::Text { text, .. } = item {
     |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
    --> 01_core/src/compiler/layout/tests.rs:9715:42
     |
9715 |                 if let FrameItem::Text { text, .. } = item {
     |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10364:42
      |
10364 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10725:42
      |
10725 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10802:42
      |
10802 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10831:42
      |
10831 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10859:42
      |
10859 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10859:48
      |
10859 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                                ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10903:42
      |
10903 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10903:48
      |
10903 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                                ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10942:42
      |
10942 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10942:48
      |
10942 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                                ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10978:42
      |
10978 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:10978:48
      |
10978 |                 if let FrameItem::Text { text, pos, .. } = item {
      |                                                ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11062:42
      |
11062 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11092:42
      |
11092 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11133:42
      |
11133 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11186:42
      |
11186 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11226:42
      |
11226 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11272:39
      |
11272 |                     FrameItem::Text { text, .. } => out.push_str(text.as_str()),
      |                                       ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11443:42
      |
11443 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11497:42
      |
11497 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11515:42
      |
11515 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:11761:42
      |
11761 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12087:39
      |
12087 |                     FrameItem::Text { pos, text, .. }
      |                                       ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12087:44
      |
12087 |                     FrameItem::Text { pos, text, .. }
      |                                            ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12354:35
      |
12354 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12402:35
      |
12402 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12452:35
      |
12452 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12500:35
      |
12500 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:12548:35
      |
12548 |                 FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13542:65
      |
13542 |                 .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == label))
      |                                                                 ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13586:35
      |
13586 |                 FrameItem::Text { text, pos, .. } => {
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13586:41
      |
13586 |                 FrameItem::Text { text, pos, .. } => {
      |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13601:35
      |
13601 |                 FrameItem::Text { text, pos, .. } => {
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13601:41
      |
13601 |                 FrameItem::Text { text, pos, .. } => {
      |                                         ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13631:61
      |
13631 |             .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "X"))
      |                                                             ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13657:65
      |
13657 |                 .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == label))
      |                                                                 ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13677:61
      |
13677 |             .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "HDR"))
      |                                                             ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13689:61
      |
13689 |             .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "FTR"))
      |                                                             ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:13715:65
      |
13715 |                 .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == label))
      |                                                                 ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:14351:35
      |
14351 |                 FrameItem::Text { pos, .. } => Some(pos.y.0.round() as i64),
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:14456:35
      |
14456 |                 FrameItem::Text { pos, .. } | FrameItem::TextShaped { pos, .. } => {
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:14476:35
      |
14476 |                 FrameItem::Text { pos, .. } | FrameItem::TextShaped { pos, .. } => {
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:17234:42
      |
17234 |                 if let FrameItem::Text { text, .. } = item {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:17411:42
      |
17411 |                 if let FrameItem::Text { text, .. } = i {
      |                                          ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:17975:35
      |
17975 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:17996:31
      |
17996 |             FrameItem::Text { pos, text, .. } if text.contains("RODAPE") => Some(pos.y.0),
      |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:17996:36
      |
17996 |             FrameItem::Text { pos, text, .. } if text.contains("RODAPE") => Some(pos.y.0),
      |                                    ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18020:31
      |
18020 |             FrameItem::Text { pos, text, .. } if text == "1" => Some(pos.y.0),
      |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18020:36
      |
18020 |             FrameItem::Text { pos, text, .. } if text == "1" => Some(pos.y.0),
      |                                    ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18024:31
      |
18024 |             FrameItem::Text { pos, text, .. } if text.contains("RODAPEB") => {
      |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18024:36
      |
18024 |             FrameItem::Text { pos, text, .. } if text.contains("RODAPEB") => {
      |                                    ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18052:31
      |
18052 |             FrameItem::Text { pos, text, .. } if text.contains("AAAA") => Some(pos.y.0),
      |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18052:36
      |
18052 |             FrameItem::Text { pos, text, .. } if text.contains("AAAA") => Some(pos.y.0),
      |                                    ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18056:31
      |
18056 |             FrameItem::Text { pos, text, .. } if text.contains("BBBB") => Some(pos.y.0),
      |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18056:36
      |
18056 |             FrameItem::Text { pos, text, .. } if text.contains("BBBB") => Some(pos.y.0),
      |                                    ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18060:31
      |
18060 |             FrameItem::Text { pos, text, .. } if text.contains("CCCC") => Some(pos.y.0),
      |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18060:36
      |
18060 |             FrameItem::Text { pos, text, .. } if text.contains("CCCC") => Some(pos.y.0),
      |                                    ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18095:35
      |
18095 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18123:35
      |
18123 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18157:35
      |
18157 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18188:35
      |
18188 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18213:35
      |
18213 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18233:35
      |
18233 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18257:35
      |
18257 |                 FrameItem::Text { text, .. } => Some(text.to_string()),
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18283:35
      |
18283 |                 FrameItem::Text { pos, text, .. } if text.contains("wordSentinel") => {
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18283:40
      |
18283 |                 FrameItem::Text { pos, text, .. } if text.contains("wordSentinel") => {
      |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18361:35
      |
18361 |                 FrameItem::Text { pos, text, .. } if text.contains("nota de rodapé") => {
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18361:40
      |
18361 |                 FrameItem::Text { pos, text, .. } if text.contains("nota de rodapé") => {
      |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18422:35
      |
18422 |                 FrameItem::Text { pos, .. } => Some(pos.x.0),
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18486:52
      |
18486 |             .any(|i| matches!(i, FrameItem::Text { style, .. } if style.bold))
      |                                                    ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18493:52
      |
18493 |             .any(|i| matches!(i, FrameItem::Text { style, .. } if style.italic))
      |                                                    ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18897:42
      |
18897 |                 if let FrameItem::Text { style, text, .. } = i {
      |                                          ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18897:49
      |
18897 |                 if let FrameItem::Text { style, text, .. } = i {
      |                                                 ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:18958:56
      |
18958 |                 .any(|i| matches!(i, FrameItem::Text { text: t, .. } if t == "Marcado"))
      |                                                        ^^^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19363:31
      |
19363 |             FrameItem::Text { pos, .. } => Some(pos.y.val().to_bits()),
      |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19376:38
      |
19376 |             if let FrameItem::Text { pos, text, .. } = item {
      |                                      ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19376:43
      |
19376 |             if let FrameItem::Text { pos, text, .. } = item {
      |                                           ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19412:31
      |
19412 |             FrameItem::Text { pos, .. } => Some(pos.y.val().to_bits()),
      |                               ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19425:38
      |
19425 |             if let FrameItem::Text { pos, text, .. } = item {
      |                                      ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19425:43
      |
19425 |             if let FrameItem::Text { pos, text, .. } = item {
      |                                           ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19503:35
      |
19503 |                 FrameItem::Text { pos, text, .. }
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19503:40
      |
19503 |                 FrameItem::Text { pos, text, .. }
      |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19519:35
      |
19519 |                 FrameItem::Text { pos, text, .. }
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:19519:40
      |
19519 |                 FrameItem::Text { pos, text, .. }
      |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:20182:31
      |
20182 |             FrameItem::Text { text, style, .. } if text.as_str() == "placed" => {
      |                               ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:20182:37
      |
20182 |             FrameItem::Text { text, style, .. } if text.as_str() == "placed" => {
      |                                     ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21103:35
      |
21103 |                 FrameItem::Text { pos, text, style, .. }
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21103:40
      |
21103 |                 FrameItem::Text { pos, text, style, .. }
      |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21103:46
      |
21103 |                 FrameItem::Text { pos, text, style, .. }
      |                                              ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21197:35
      |
21197 |                 FrameItem::Text { text, style, .. }
      |                                   ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21197:41
      |
21197 |                 FrameItem::Text { text, style, .. }
      |                                         ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21385:35
      |
21385 |                 FrameItem::Text { pos, text, style, .. } if text.as_str() == needle => {
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21385:40
      |
21385 |                 FrameItem::Text { pos, text, style, .. } if text.as_str() == needle => {
      |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::style`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21385:46
      |
21385 |                 FrameItem::Text { pos, text, style, .. } if text.as_str() == needle => {
      |                                              ^^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21423:35
      |
21423 |                 FrameItem::Text { pos, text, .. } if text.as_str().contains("dado") => {
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:21423:40
      |
21423 |                 FrameItem::Text { pos, text, .. } if text.as_str().contains("dado") => {
      |                                        ^^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::pos`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:22028:35
      |
22028 |                 FrameItem::Text { pos, text, .. }
      |                                   ^^^

warning: use of deprecated field `entities::layout_types::FrameItem::Text::text`: Use FrameItem::TextShaped. Preserved as fallback for fonts not loaded or Type1.
     --> 01_core/src/compiler/layout/tests.rs:22028:40
      |
22028 |                 FrameItem::Text { pos, text, .. }
      |                                        ^^^^

warning: unused import: `Track`
  --> 01_core/src/compiler/eval/modules.rs:15:14
   |
15 | use comemo::{Track, TrackedMut};
   |              ^^^^^

warning: unused import: `crate::contracts::world::World`
    --> 01_core/src/compiler/introspect.rs:3382:9
     |
3382 |     use crate::contracts::world::World as _;
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `super`
     --> 01_core/src/compiler/layout/tests.rs:17444:9
      |
17444 |     use super::*;
      |         ^^^^^

warning: unused import: `super`
     --> 01_core/src/compiler/layout/tests.rs:17558:9
      |
17558 |     use super::*;
      |         ^^^^^

warning: unused import: `super`
     --> 01_core/src/compiler/layout/tests.rs:17689:9
      |
17689 |     use super::*;
      |         ^^^^^

warning: unused import: `std::hash::Hash`
  --> 01_core/src/entities/elements/page_run.rs:43:13
   |
43 |         use std::hash::Hash;
   |             ^^^^^^^^^^^^^^^

warning: variable does not need to be mutable
   --> 01_core/src/compiler/eval/mod.rs:406:5
    |
406 |     mut sink: TrackedMut<Sink>,
    |     ----^^^^
    |     |
    |     help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
   --> 01_core/src/compiler/eval/mod.rs:433:5
    |
433 |     mut sink: TrackedMut<Sink>,
    |     ----^^^^
    |     |
    |     help: remove this `mut`

warning: variable does not need to be mutable
   --> 01_core/src/compiler/eval/mod.rs:526:9
    |
526 |     let mut run_pass = |apply_show_rules: bool,
    |         ----^^^^^^^^
    |         |
    |         help: remove this `mut`

warning: unused variable: `e`
    --> 01_core/src/compiler/eval/tests.rs:6787:47
     |
6787 |             matches!(content, Content::Figure(e)),
     |                                               ^ help: if this is intentional, prefix it with an underscore: `_e`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `e`
    --> 01_core/src/compiler/eval/tests.rs:6801:51
     |
6801 |         assert!(matches!(content, Content::Figure(e)));
     |                                                   ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/eval/tests.rs:6830:47
     |
6830 |             matches!(content, Content::Figure(e)),
     |                                               ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unreachable pattern
   --> 01_core/src/compiler/layout/equation.rs:436:17
    |
358 |                 FrameItem::Glyph { pos, glyph_id, x_advance, size, style, base_char } => {
    |                 --------------------------------------------------------------------- matches all the relevant values
...
436 |                 FrameItem::Glyph { pos, glyph_id, x_advance, size, style, base_char } => {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no value can reach this
    |
    = note: `#[warn(unreachable_patterns)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `rowspan`
   --> 01_core/src/compiler/layout/grid.rs:245:21
    |
245 |                 let rowspan = rowspan.unwrap_or(1).max(1);
    |                     ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_rowspan`

warning: unused variable: `line_leading_pt`
   --> 01_core/src/compiler/layout/sub_frame.rs:216:13
    |
216 |         let line_leading_pt = self
    |             ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_line_leading_pt`

warning: variable does not need to be mutable
   --> 01_core/src/compiler/layout/boxed.rs:262:13
    |
262 |         let mut outer_w = outer_w;
    |             ----^^^^^^^
    |             |
    |             help: remove this `mut`

warning: variable does not need to be mutable
   --> 01_core/src/compiler/layout/boxed.rs:267:13
    |
267 |         let mut outer_h = inner_h + outset_top + outset_bottom;
    |             ----^^^^^^^
    |             |
    |             help: remove this `mut`

warning: variable does not need to be mutable
   --> 01_core/src/compiler/layout/boxed.rs:268:13
    |
268 |         let mut pos = crate::entities::layout_types::Point {
    |             ----^^^
    |             |
    |             help: remove this `mut`

warning: unused variable: `body_items_before`
   --> 01_core/src/compiler/layout/boxed.rs:108:9
    |
108 |     let body_items_before = layouter.regions.current.current_items.len();
    |         ^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_body_items_before`

warning: unused variable: `heading_descent`
   --> 01_core/src/compiler/layout/heading.rs:114:9
    |
114 |     let heading_descent = bottom.0.abs();
    |         ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_heading_descent`

warning: unused variable: `font_size`
   --> 01_core/src/compiler/layout/mod.rs:862:9
    |
862 |         font_size: f64,
    |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_font_size`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:1494:9
     |
1494 |     let state = introspect(content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:2216:9
     |
2216 |     let state = introspect(content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:3728:9
     |
3728 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:3778:9
     |
3778 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:3800:9
     |
3800 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:3920:9
     |
3920 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:3939:9
     |
3939 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:4059:9
     |
4059 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:4085:9
     |
4085 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:4100:9
     |
4100 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:4135:9
     |
4135 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:4173:9
     |
4173 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:4198:9
     |
4198 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:4260:9
     |
4260 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:4284:9
     |
4284 |     let state = introspect(&content);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:5182:9
     |
5182 |     let state = introspect(&grid);
     |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:5413:13
     |
5413 |         let state = introspect(content);
     |             ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
    --> 01_core/src/compiler/layout/tests.rs:6281:13
     |
6281 |         let state = introspect(content);
     |             ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `b1`
     --> 01_core/src/compiler/layout/tests.rs:11013:13
      |
11013 |         let b1 = p250_mk_block_with(
      |             ^^ help: if this is intentional, prefix it with an underscore: `_b1`

warning: unused variable: `state`
     --> 01_core/src/compiler/layout/tests.rs:13808:13
      |
13808 |         let state = introspect(c);
      |             ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state`
     --> 01_core/src/compiler/layout/tests.rs:14742:13
      |
14742 |         let state = introspect(&content);
      |             ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state_legacy`
     --> 01_core/src/compiler/layout/tests.rs:14757:13
      |
14757 |         let state_legacy = introspect(&content);
      |             ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_state_legacy`

warning: unused variable: `state_legacy`
     --> 01_core/src/compiler/layout/tests.rs:14872:17
      |
14872 |             let state_legacy = crate::compiler::introspect::introspect(&content);
      |                 ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_state_legacy`

warning: unused variable: `state`
     --> 01_core/src/compiler/layout/tests.rs:14942:13
      |
14942 |         let state = crate::compiler::introspect::introspect(&content);
      |             ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `state_legacy`
     --> 01_core/src/compiler/layout/tests.rs:15479:13
      |
15479 |         let state_legacy = introspect(&content);
      |             ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_state_legacy`

warning: unused variable: `state_com`
     --> 01_core/src/compiler/layout/tests.rs:16071:13
      |
16071 |         let state_com = introspect(&doc_com_outline);
      |             ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_state_com`

warning: unused variable: `state_sem`
     --> 01_core/src/compiler/layout/tests.rs:16080:13
      |
16080 |         let state_sem = introspect(&doc_sem_outline);
      |             ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_state_sem`

warning: unused variable: `state`
     --> 01_core/src/compiler/layout/tests.rs:16239:13
      |
16239 |         let state = introspect(&content);
      |             ^^^^^ help: if this is intentional, prefix it with an underscore: `_state`

warning: unused variable: `leading`
     --> 01_core/src/compiler/layout/tests.rs:19920:9
      |
19920 |     let leading = style
      |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_leading`

warning: unused variable: `leading`
     --> 01_core/src/compiler/layout/tests.rs:20060:9
      |
20060 |     let leading = short_style
      |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_leading`

warning: unused variable: `leading`
     --> 01_core/src/compiler/layout/tests.rs:20190:9
      |
20190 |     let leading = placed_style
      |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_leading`

warning: unused variable: `leading`
     --> 01_core/src/compiler/layout/tests.rs:20608:9
      |
20608 |     let leading = style
      |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_leading`

warning: unused variable: `leading`
     --> 01_core/src/compiler/layout/tests.rs:20725:9
      |
20725 |     let leading = style
      |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_leading`

warning: unused variable: `leading`
     --> 01_core/src/compiler/layout/tests.rs:20860:9
      |
20860 |     let leading = style
      |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_leading`

warning: unused variable: `dx`
  --> 01_core/src/compiler/math/layout/accent.rs:79:13
   |
79 |         let dx = base_attach - accent_attach;
   |             ^^ help: if this is intentional, prefix it with an underscore: `_dx`

warning: value assigned to `ratio` is never read
  --> 01_core/src/compiler/math/layout/assembly.rs:36:21
   |
36 |     let mut ratio = 0.0_f64;
   |                     ^^^^^^^
   |
   = help: maybe it is overwritten before being read?
   = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `sub_top`
   --> 01_core/src/compiler/math/layout/attach.rs:592:21
    |
592 |                 let sub_top = sup_b.descent - shift_down; // em coordenadas onde y cresce para baixo, sub_top é -shift_down + sub_b.ascent
    |                     ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_sub_top`

warning: unreachable pattern
   --> 01_core/src/compiler/math/layout/spacing.rs:104:11
    |
 86 |         Content::Equation(e) => base_math_class(&e.body),
    |         -------------------- matches all the relevant values
...
104 |         | Content::Equation(_)
    |           ^^^^^^^^^^^^^^^^^^^^ no value can reach this

warning: unreachable pattern
   --> 01_core/src/compiler/math/layout/spacing.rs:135:11
    |
 85 |         Content::Styled(inner, _) => base_math_class(inner),
    |         ------------------------- matches all the relevant values
...
135 |         | Content::Styled(_, _)
    |           ^^^^^^^^^^^^^^^^^^^^^ no value can reach this

warning: unreachable pattern
   --> 01_core/src/compiler/math/layout/spacing.rs:295:9
    |
278 |         (Opening, _) | (_, Closing) => Some(0.0),
    |         --------------------------- matches all the relevant values
...
295 |         (Opening, Fence) | (Fence, Closing) => Some(0.0),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no value can reach this

warning: unused variable: `items`
    --> 01_core/src/compiler/math/layout/tests.rs:1800:10
     |
1800 |     let (items, ext) = ml.layout_equation_measured(&attach, &default_style());
     |          ^^^^^ help: if this is intentional, prefix it with an underscore: `_items`

warning: variable does not need to be mutable
   --> 01_core/src/compiler/parse/parser.rs:741:13
    |
741 |         let mut p = Parser::new("", 0, SyntaxMode::Markup);
    |             ----^
    |             |
    |             help: remove this `mut`

warning: variable does not need to be mutable
   --> 01_core/src/compiler/stdlib/foundations/str.rs:130:28
    |
130 | pub(crate) fn format_radix(mut n: i64, base: u32) -> String {
    |                            ----^
    |                            |
    |                            help: remove this `mut`

warning: unused variable: `idx`
   --> 01_core/src/compiler/stdlib/numbering.rs:168:10
    |
168 |     for (idx, (prefix, symbol)) in pieces.iter().enumerate() {
    |          ^^^ help: if this is intentional, prefix it with an underscore: `_idx`

warning: variable does not need to be mutable
    --> 01_core/src/compiler/stdlib/mod.rs:367:17
     |
 367 |             let mut $ctx = EvalContext::new();
     |                 ----^^^^
     |                 |
     |                 help: remove this `mut`
...
1130 |         null_ctx!(ctx);
     |         -------------- in this macro invocation
     |
     = note: this warning originates in the macro `null_ctx` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: variable does not need to be mutable
    --> 01_core/src/compiler/stdlib/mod.rs:367:17
     |
 367 |             let mut $ctx = EvalContext::new();
     |                 ----^^^^
     |                 |
     |                 help: remove this `mut`
...
1141 |         null_ctx!(ctx);
     |         -------------- in this macro invocation
     |
     = note: this warning originates in the macro `null_ctx` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: variable does not need to be mutable
    --> 01_core/src/compiler/stdlib/mod.rs:367:17
     |
 367 |             let mut $ctx = EvalContext::new();
     |                 ----^^^^
     |                 |
     |                 help: remove this `mut`
...
1324 |         null_ctx!(ctx);
     |         -------------- in this macro invocation
     |
     = note: this warning originates in the macro `null_ctx` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7182:53
     |
7182 |             matches!(r, Value::Content(Content::Pad(e))),
     |                                                     ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7542:55
     |
7542 |             matches!(r, Value::Content(Content::Block(e))),
     |                                                       ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7550:53
     |
7550 |             matches!(r, Value::Content(Content::Pad(e))),
     |                                                     ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7578:59
     |
7578 |         assert!(matches!(r, Value::Content(Content::Block(e))));
     |                                                           ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7583:59
     |
7583 |         assert!(matches!(r, Value::Content(Content::Boxed(e))));
     |                                                           ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7588:57
     |
7588 |         assert!(matches!(r, Value::Content(Content::Pad(e))));
     |                                                         ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7747:59
     |
7747 |         assert!(matches!(r, Value::Content(Content::Stack(e))));
     |                                                           ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7752:59
     |
7752 |         assert!(matches!(r, Value::Content(Content::Block(e))));
     |                                                           ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7757:59
     |
7757 |         assert!(matches!(r, Value::Content(Content::Boxed(e))));
     |                                                           ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: unused variable: `e`
    --> 01_core/src/compiler/stdlib/mod.rs:7762:57
     |
7762 |         assert!(matches!(r, Value::Content(Content::Pad(e))));
     |                                                         ^ help: if this is intentional, prefix it with an underscore: `_e`

warning: function `ck` is never used
   --> 01_core/src/compiler/introspect/fixpoint.rs:170:8
    |
170 |     fn ck(s: &str) -> CounterKey {
    |        ^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `format_bib_entry` is never used
    --> 01_core/src/compiler/layout/mod.rs:2701:4
     |
2701 | fn format_bib_entry(e: &crate::entities::bib_entry::BibEntry) -> String {
     |    ^^^^^^^^^^^^^^^^

warning: function `line_content_bottom` is never used
   --> 01_core/src/compiler/layout/helpers.rs:100:15
    |
100 | pub(super) fn line_content_bottom<'a>(
    |               ^^^^^^^^^^^^^^^^^^^

warning: function `ck` is never used
   --> 01_core/src/compiler/layout/tests.rs:325:4
    |
325 | fn ck(s: &str) -> CounterKey {
    |    ^^

warning: function `ck` is never used
     --> 01_core/src/compiler/layout/tests.rs:15749:8
      |
15749 |     fn ck(s: &str) -> CounterKey {
      |        ^^

warning: function `ck` is never used
     --> 01_core/src/compiler/layout/tests.rs:15872:8
      |
15872 |     fn ck(s: &str) -> CounterKey {
      |        ^^

warning: function `heading_with_text` is never used
     --> 01_core/src/compiler/layout/tests.rs:15876:8
      |
15876 |     fn heading_with_text(level: u8, text: &str) -> Content {
      |        ^^^^^^^^^^^^^^^^^

warning: function `ck` is never used
     --> 01_core/src/compiler/layout/tests.rs:15983:8
      |
15983 |     fn ck(s: &str) -> CounterKey {
      |        ^^

warning: function `doc_3_equations` is never used
     --> 01_core/src/compiler/layout/tests.rs:15992:8
      |
15992 |     fn doc_3_equations() -> Content {
      |        ^^^^^^^^^^^^^^^

warning: function `lbl` is never used
     --> 01_core/src/compiler/layout/tests.rs:19719:8
      |
19719 |     fn lbl(s: &str) -> Label {
      |        ^^^

warning: function `native_array_all_static` is never used
   --> 01_core/src/compiler/stdlib/collections.rs:657:4
    |
657 | fn native_array_all_static(
    |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `native_str_clusters_static` is never used
   --> 01_core/src/compiler/stdlib/collections.rs:691:4
    |
691 | fn native_str_clusters_static(
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `str_trim` is never used
    --> 01_core/src/compiler/stdlib/collections.rs:2150:4
     |
2150 | fn str_trim(s: EcoString) -> Value {
     |    ^^^^^^^^

warning: function `expect_one_array` is never used
    --> 01_core/src/compiler/stdlib/collections.rs:2561:4
     |
2561 | fn expect_one_array(args: Args, context: &str) -> SourceResult<Vec<Value>> {
     |    ^^^^^^^^^^^^^^^^

warning: struct `DummyDownloader` is never constructed
  --> 01_core/src/contracts/package_downloader.rs:93:12
   |
93 |     struct DummyDownloader;
   |            ^^^^^^^^^^^^^^^

warning: function `p311b5_cal_L_emite_script_L` should have a snake case name
    --> 01_core/src/compiler/math/layout/tests.rs:1597:4
     |
1597 | fn p311b5_cal_L_emite_script_L() {
     |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `p311b5_cal_l_emite_script_l`
     |
     = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: variable `cal_L` should have a snake case name
    --> 01_core/src/compiler/math/layout/tests.rs:1598:9
     |
1598 |     let cal_L = Content::math_styled(
     |         ^^^^^ help: convert the identifier to snake case: `cal_l`

warning: `typst-core` (lib test) generated 675 warnings (run `cargo fix --lib -p typst-core --tests` to apply 115 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.35s
     Running unittests src/lib.rs (target/debug/deps/typst_core-d51858d7cd5964e0)

running 5 tests
test compiler::eval::tests::tests::p1303_negativos_pdf_exatos_nos_perfis_sem_a11y ... FAILED
test compiler::eval::tests::tests::p1303_ordem_repeticao_e_estado_completo ... ok
test compiler::eval::tests::tests::p1303_positivos_pdf_exatos_nos_perfis_com_a11y ... ok
test compiler::eval::tests::tests::p1303_sentinelas_de_span_module_e_nao_module ... ok
test compiler::eval::tests::tests::p1303_sentinelas_pdf_ungated_e_html_disabled ... ok

failures:

---- compiler::eval::tests::tests::p1303_negativos_pdf_exatos_nos_perfis_sem_a11y stdout ----

thread 'compiler::eval::tests::tests::p1303_negativos_pdf_exatos_nos_perfis_sem_a11y' (298) panicked at 01_core/src/compiler/eval/tests.rs:17941:9:
P1303-N/default/data-cell: span esperado 14..23, obtido Some(10..23)
P1303-N/default/header-cell: span esperado 14..25, obtido Some(10..25)
P1303-N/default/table-summary: span esperado 14..27, obtido Some(10..27)
P1303-N/html/data-cell: span esperado 14..23, obtido Some(10..23)
P1303-N/html/header-cell: span esperado 14..25, obtido Some(10..25)
P1303-N/html/table-summary: span esperado 14..27, obtido Some(10..27)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    compiler::eval::tests::tests::p1303_negativos_pdf_exatos_nos_perfis_sem_a11y

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 5431 filtered out; finished in 0.59s

error: test failed, to rerun pass `-p typst-core --lib`
```

O hash acima é do conteúdo literal dentro deste bloco, incluindo a newline
final emitida pelo comando capturado. O veredito limita-se ao P5 RED e não
aprova implementação, mutantes, GREEN ou equivalência funcional geral.

