# P1493 — gate ADR-0127 para posições locais em `html.frame`

## Estado

CONCLUÍDO em 2026-09-24. O gate ADR-0127 foi confirmado pelo dono, os testes foram escritos RED antes do código e a implementação foi levada a GREEN; hashes e matriz foram ressellados somente após a medição bilateral.

## Proveniência da medição

Medição em `2026-09-24T15:24:12-03:00`, HEAD limpo
`b5abe9b288ce168c494a4c5172830e325b9aedae`.

| Produto | SHA-256 |
|---|---|
| cristalino release | `6e3962091d94148095ca36416f407c79c9b698475a1e990dccb7964def9ce0fd` |
| vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |

A fixture completa `P1489-I-001` permaneceu `VIOLATED` somente na
asserção interna `position() == (x: 30pt, y: 20pt)`. Em todos os perfis,
vanilla saiu 0, cristalino saiu 1 e a ordem bilateral foi idêntica:

| Perfil | digest forward/reverse |
|---|---|
| `html` | `6f87f6ce2ed27abc7c7ae1eedc3c2b53c16b2e374558addf725bc5e69046eb1a` |
| `html+a11y` | `cd1d72933b507f193e1ed81afa891fa45b4dfcdfd53889bf7f648c02fa9ad051` |
| `html+bundle` | `7f1232375b21b24ab1268e131d92c6c2efa93bf897b03f4bf3bb1b42e714d5c6` |

## Sondas adversariais

Duas sondas temporárias adicionais foram compiladas bilateralmente:

- duas Locations na mesma ilha, em 10pt/5pt e 40pt/25pt, com identidade
  repetida estável e `page() == none`: vanilla passou; cristalino devolveu
  `none` nas duas posições;
- frame aninhado em 10pt/5pt com posição interna em 7pt/3pt: vanilla confirmou
  a coordenada relativa à ilha externa `(x: 17pt, y: 8pt)`; cristalino
  devolveu `none`.

Essas medições refutam tanto uma única posição por frame quanto coordenadas
relativas ao frame aninhado. A página intrínseca 1 não aparece na linguagem.

### Refinamento adversarial de normalização (2026-09-24T16:41:36-03:00)

No vanilla pinado, o contexto em `place(dx: 30pt, dy: 20pt)` continuou a
devolver exatamente `(x: 30pt, y: 20pt)` mesmo quando emitiu
`[html-control]`; um contexto visível direto devolveu `(x: 0pt, y: 0pt)`.
Logo o recorte visual final da Page não integra a coordenada da linguagem.
A sonda usou HEAD `b5abe9b288ce168c494a4c5172830e325b9aedae`, working tree
não commitado com 34 caminhos em `git diff HEAD --stat`; untracked: este
diagnóstico e as três fixtures P1493 de múltiplas, aninhada e bundle.
A sonda adicional `move(11pt,5pt)` sob `place(30pt,20pt)` confirmou
41pt/25pt no vanilla; o RED L1 permaneceu em 30pt/20pt porque a fórmula
centrada descartava `matrix.tx/ty`.
O estado integral dos caminhos será atualizado no fechamento GREEN.

## Causa medida

- `prepare_html` executa produtores antes do layout intrínseco e remove seus
  markers;
- o log de eval registra `LocationPosition`, mas não expõe essa demanda como
  causa de seleção;
- `layout_intrinsic` registra posições dos markers no runtime e depois as
  descarta;
- o único carrier atual, `Position`, exige página não nula;
- P1492 retorna `none` para toda posição no target HTML porque não existe
  store local discriminado.

## Desenho submetido ao gate

1. Introduzir `HtmlPosition { point }`, sem página, separado de `Position`.
2. Adicionar ao `Introspector` uma consulta/store HTML separados e injeção
   substitutiva por geração.
3. Fazer eval expor somente a presença real de leitura
   `LocationPosition`.
4. Fazer o layout detalhado da ilha devolver Page mais
   `ContextBlock id -> HtmlPosition`, após transforms e antes do recorte visual privado da Page.
5. Fazer L3 associar ids às Locations da topologia candidata e executar um
   fixpoint de até cinco tentativas apenas para a demanda HTML de posição.
6. Manter PageStore e positions paginadas fora do target HTML; em bundle misto,
   cada target conserva seu store.
7. Projetar `(x, y)` sem `page` no dispatch HTML; fora de frame continua
   `none` e `page()` permanece sempre `none`.

## Owners L0 ratificados

- `compiler/eval.md`;
- `compiler/eval/call_dispatch.md`;
- `compiler/introspect.md`;
- `compiler/layout.md`;
- `compiler/layout/html_frame.md`;
- `compiler/layout/helpers.md`;
- `compiler/layout/link.md`;
- `compiler/layout/metrics.md`;
- `compiler/layout/stack.md`;
- `compiler/layout/transform.md`;
- `compiler/layout/sub_frame.md`;
- `entities/position.md`;
- `entities/introspector.md`;
- `infra/measurements.md`;
- `infra/pipeline.md`;
- `infra/pipeline/context_stabilization.md`.

O store paginado `entities/sealed-positions` não muda. A proposta não cria
`PagedDocument` global, não usa feature como target, não executa callback no
layouter e não expõe o número lógico da Page intrínseca.

## Confirmação recebida

O dono confirmou em 2026-09-24 a adição pública de `HtmlPosition` e do
método tracked no `Introspector`, além da nova fase localizada
eval/introspecção → layout de ilhas → replay contextual. A materialização
prossegue com testes RED antes do código; os vinte e cinco owners foram ressellados
após o GREEN e a validação das fixtures simples, múltiplas, aninhadas, externa
negativa, paginada e bundle misto.


## Resultado materializado

O contrato confirmado foi implementado com `HtmlPosition` separado do store paginado, consulta tracked no `Introspector`, seleção por demanda efetiva, fachada detalhada de `html.frame`, carrier privado de `ContextBlock` através dos wrappers geométricos e fixpoint HTML localizado em L3. A página intrínseca da ilha não vaza para `location.page()` e o documento HTML não ganha paginação global.

Além dos dezesseis owners inicialmente listados, a implementação mediu a necessidade de neutralidade/transporte do carrier e atualizou, antes do código correspondente, `compiler/columns.md`, `compiler/layout/boxed.md`, `compiler/layout/cursor.md`, `compiler/layout/equation.md`, `compiler/layout/page_run.md`, `compiler/layout/pdf_artifact.md`, `compiler/layout/place.md`, `compiler/layout/sequence.md` e `compiler/layout_outline.md`. Total: 25 owners L0, todos ressellados.

A generalização experimental para qualquer Location consultada foi descartada: uma regressão de geração estável para alterada demonstrou que o ordinal raw é reutilizado e poderia transferir posição a outro elemento. O L0 de `infra/pipeline.md` delimita P1493 a `ContextBlock`/`here()` e reserva identidade estável de Locations arbitrárias para passo e gate próprios.

## Medição GREEN e matriz

Medição em `2026-09-24T18:00:54-03:00`, HEAD `b5abe9b288ce168c494a4c5172830e325b9aedae`, working tree P1493 não commitado. SHA-256 do release cristalino: `35a2fdea38c3e5a9b8b0c2a8a93c1cf9f4b6439965ed0103aa0e564c9ca3ca83`; SHA-256 do vanilla pinado: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

| Caso/perfil | Estado | digest forward/reverse |
|---|---|---|
| `P1489-I-001` / `html` | MATCH | `dd61dd283417ce092979c6a50dc26776b15f31ddfa2d28a8ced923e83773157d` |
| `P1489-I-001` / `html+a11y` | MATCH | `731a45a62b5f0a58df27836fd4cc0880e7c8ccb8507c9281e50cf5c210ded0e8` |
| `P1489-I-001` / `html+bundle` | MATCH | `2a6228dfc25edea6c4d85df05da6ea34af2a033b1ffb5c2a9e1f0756a43c6983` |
| `P1493-I-001` / `html` | MATCH | `341c1f00c839fbbb829f3e4a92d5572011ab77578f02b5286a30358cd49a02dc` |
| `P1493-I-002` / `html` | MATCH | `8dde80800b81aa125b4e81a77103a30aa60c7087060e769e6fa8b361c674fcb5` |
| `P1493-I-003` / `html+bundle` | MATCH | `65127b9ee9e3f992dbba527dee5f6d5545460a18ef5db226415493c46d4eb2f2` |
| controle `P1492-I-001` / `html` | MATCH | `5a808d28b0190618cfb2336fb24454d6f4067c6f55d90a285582fab2e625473b` |
| controle `P1487-X-002` / `bundle` | MATCH | `c64a19364c30f48d101105dd9b27cd91878dcf3afa241d829cd349dd764a1f3e` |

Todos os digests tiveram forward/reverse idênticos. `P1493-I-003` usa `artifact_presence`, pois a própria fixture aborta se `page()` ou 30pt/20pt divergirem; a diferença de morfologia do SVG vazio (0x0 contra 100x50) é preexistente e constitui eixo separado.

## Validação

- `RUSTFLAGS=-Awarnings cargo test -p typst-core p1493 -- --nocapture`: 11/11;
- `RUSTFLAGS=-Awarnings cargo test -p typst-infra --lib p1493 -- --nocapture`: 6/6;
- `python3 -m unittest -q lab/parity/matrix/test_runner.py`: 56/56;
- `cargo build` e `cargo build --release`: sucesso;
- `crystalline-lint --fix-hashes .`: 25 consumers ressellados, zero drift;
- `crystalline-lint .`: exit 0, zero violations;
- `git diff --check`: limpo.

A suíte de integração inteira `cargo test -p typst-infra p1493` não chega aos testes do passo porque dois testes preexistentes instanciam `Page` sem `logical_number`: `03_infra/tests/p1405_pdf_label.rs:17` e `03_infra/tests/p1250_svg_integrated.rs:30`. Esse bloqueio externo não afeta a biblioteca nem as seis regressões L3 do P1493.

## Estado exato da árvore na medição

`git diff HEAD --stat` registrou 52 caminhos tracked, 2.284 inserções e 354 remoções. Lista exata:

```text
00_nucleo/materialization/typst-passo-1493.md
00_nucleo/prompts/compiler/columns.md
00_nucleo/prompts/compiler/eval.md
00_nucleo/prompts/compiler/eval/call_dispatch.md
00_nucleo/prompts/compiler/introspect.md
00_nucleo/prompts/compiler/layout.md
00_nucleo/prompts/compiler/layout/boxed.md
00_nucleo/prompts/compiler/layout/cursor.md
00_nucleo/prompts/compiler/layout/equation.md
00_nucleo/prompts/compiler/layout/helpers.md
00_nucleo/prompts/compiler/layout/html_frame.md
00_nucleo/prompts/compiler/layout/link.md
00_nucleo/prompts/compiler/layout/metrics.md
00_nucleo/prompts/compiler/layout/page_run.md
00_nucleo/prompts/compiler/layout/pdf_artifact.md
00_nucleo/prompts/compiler/layout/place.md
00_nucleo/prompts/compiler/layout/sequence.md
00_nucleo/prompts/compiler/layout/stack.md
00_nucleo/prompts/compiler/layout/sub_frame.md
00_nucleo/prompts/compiler/layout/transform.md
00_nucleo/prompts/compiler/layout_outline.md
00_nucleo/prompts/entities/introspector.md
00_nucleo/prompts/entities/position.md
00_nucleo/prompts/infra/measurements.md
00_nucleo/prompts/infra/pipeline.md
00_nucleo/prompts/infra/pipeline/context_stabilization.md
01_core/src/compiler/eval/call_dispatch.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/introspect.rs
01_core/src/compiler/layout/boxed.rs
01_core/src/compiler/layout/columns.rs
01_core/src/compiler/layout/cursor.rs
01_core/src/compiler/layout/equation.rs
01_core/src/compiler/layout/helpers.rs
01_core/src/compiler/layout/html_frame.rs
01_core/src/compiler/layout/link.rs
01_core/src/compiler/layout/metrics.rs
01_core/src/compiler/layout/mod.rs
01_core/src/compiler/layout/outline.rs
01_core/src/compiler/layout/page_run.rs
01_core/src/compiler/layout/pdf_artifact.rs
01_core/src/compiler/layout/place.rs
01_core/src/compiler/layout/sequence.rs
01_core/src/compiler/layout/stack.rs
01_core/src/compiler/layout/sub_frame.rs
01_core/src/compiler/layout/transform.rs
01_core/src/entities/introspector.rs
01_core/src/entities/position.rs
03_infra/src/measurements.rs
03_infra/src/pipeline.rs
03_infra/src/pipeline/context_stabilization.rs
lab/parity/matrix/manifest.yaml
```

Untracked no mesmo instante:

```text
00_nucleo/diagnosticos/typst-p1493-gate-posicoes-html-frame.md
lab/parity/matrix/fixtures/location-html-frame-bundle.typ
lab/parity/matrix/fixtures/location-html-frame-multiple.typ
lab/parity/matrix/fixtures/location-html-frame-nested.typ
```
