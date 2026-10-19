# P1504 — separação remanescente do Cursor: execução e limites

## Proveniência e objeto

Medição estrutural final em 2026-09-25T22:38:07Z, HEAD
`0de5edabed197d23b647cfc68ebefda2f5c2a8d5`, **working tree não
commitada** contendo P1503 e P1504. `git diff HEAD --stat` naquele instante
registrou 624 inserções/1.525 remoções nestes oito arquivos rastreados:

- `00_nucleo/prompts/compiler/columns.md` (15 linhas alteradas);
- `00_nucleo/prompts/compiler/layout.md` (42);
- `00_nucleo/prompts/compiler/layout/cursor.md` (79);
- `00_nucleo/prompts/compiler/layout/place.md` (15);
- `01_core/src/compiler/layout/columns.rs` (139);
- `01_core/src/compiler/layout/cursor.rs` (1.582);
- `01_core/src/compiler/layout/mod.rs` (12);
- `01_core/src/compiler/layout/place.rs` (265).

Não rastreados nesse instante: `00_nucleo/diagnosticos/typst-p1503-responsabilidades-cursor.md`,
`00_nucleo/materialization/typst-passo-1503.md`,
`00_nucleo/materialization/typst-passo-1504.md`,
`00_nucleo/prompts/compiler/layout/line_commit.md`,
`01_core/src/compiler/layout/line_commit.rs`,
`00_nucleo/prompts/compiler/layout/page_commit.md`,
`00_nucleo/prompts/compiler/layout/cell_tail_commit.md`,
`00_nucleo/prompts/compiler/layout/word_segment.md`,
`00_nucleo/prompts/compiler/layout/flow_suffix.md`,
`01_core/src/compiler/layout/page_commit.rs`,
`01_core/src/compiler/layout/cell_tail_commit.rs`,
`01_core/src/compiler/layout/word_segment.rs` e
`01_core/src/compiler/layout/flow_suffix.rs`.
O diff do HEAD **não** representa sozinho a fonte do binário; os novos
arquivos integram a working tree. O release cristalino final tem SHA-256
`9b2886b813798764d414682ee934c18638178319d09579262bc48435b940871a`.
O vanilla continua pinado no commit
`9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

O baseline pós-P1503 está em
`00_nucleo/diagnosticos/typst-p1503-responsabilidades-cursor.md`:
`cursor.rs` tinha 1.324 linhas. Após P1504, tem 562 linhas, SHA-256
`030948b7287f41d4b0ff157aeedbacfd6eb8ca6405ab6ac51c05b6c3c9a1d35b`.
A contagem é métrica estrutural, não prova de velocidade. As fontes
`tools/perf/corpus/p922923-canonical/02-lorem.typ` e
`lab/parity/corpus/p1407/repeat.typ` têm SHA-256, respectivamente,
`515428fc6da638a5744ee0078d330f5fe849ff6c22c77285a3478439fe53049c`
e `4242ed2345e44d8babc48ac7f51b5dd174804944bec67ffa746fad6487b9e90a`.

## Decisão por responsabilidade

| Origem no Cursor P1503 | Destino P1504 | Limite que permaneceu |
|---|---|---|
| `new_page:424-572` | `page_commit.rs`, owner `compiler/layout/page_commit.md` | `new_page` decide a transição, drena inserções/notas e prepara a próxima região; a página terminal de `finish()` permanece em `compiler/layout.md` |
| `close_current_column`/`merge_column_items`/`finish_columns`/`start_next_column`/`start_column` | `columns.rs`, owner já existente `compiler/columns.md` | o ponto causal da transição continua em `new_page` |
| `flush_pending_cell_tails` | `cell_tail_commit.rs`, owner `compiler/layout/cell_tail_commit.md` | a fila/checkpoint continuam em `compiler/layout.md` e o hook no topo é chamado pelo Cursor |
| helpers ICU/CJK e `layout_segmented_word` | `word_segment.rs`, owner `compiler/layout/word_segment.md` | `layout_word`, `layout_chunk`, medição e wrap continuam no Cursor |
| `deferred_frame_origin_y` e `translate_deferred_float` | `place.rs`, owner já existente `compiler/layout/place.md` | fitting já pertencia a Place; progresso regional continua no Cursor |
| aceite de transição ativa e `layout_flow_suffix_transaction` | `flow_suffix.rs`, owner `compiler/layout/flow_suffix.md` | campos/captura/restauração do checkpoint continuam em `compiler/layout.md` |

`finish_float_prefix_at_marker`, `flush_pending_floats` e o hook de
admissão ficaram deliberadamente no Cursor: ligam a fila de inserções ao
avanço de região, não à geometria do float nem ao replay da unidade.
`new_page` e `layout_word` também permanecem coordenadores. Assim, nem toda
função candidata saiu por meta de linhas. A mudança do owner da página
terminal de `finish()` foi explicitamente excluída no L0 novo; sua eventual
unificação precisa de outro passo e medição.

Quatro novos prompts L0 possuem cada um um consumer; `columns` e `place`
continuaram com seus owners 1:1. `compiler/layout/cursor.md` e
`compiler/layout.md` foram atualizados antes do código de cada fatia.
Os testes estruturais por fatia foram RED antes da transferência e GREEN
depois. Na primeira execução conjunta, uma asserção autorreferente de
`Page` deu falso positivo; foi corrigida sem mudar produto. O conjunto
P1504 final passou 11/11. V15/V26 passaram antes do resselo; o reparador
tocou apenas os oito pares previstos e o dry-run final devolveu
`Nothing to fix`. O lint completo terminou exit 0, com avisos legados
não bloqueantes.

## Controles funcionais e bilateral

`cargo test --offline -p typst-core --lib` com somente os dois testes
históricos de recursão excluídos passou 6.207/6.207 (log
`/tmp/p1504-core-suite.log`, SHA-256
`0f53119ec87532945b245435311d48305b323e71c8322798c7578fad26ffdb18`).
Entre os controles executados estão P756 CJK/Thai, P251 cell tails,
P537b/P538c colunas e footnotes, P1140.24 header, P245 top/bottom float
e P1292 checkpoint/clearance. L3 passou P1407 5/5, P1493 6/6 e P1502
4/4. Builds debug e release offline passaram; `crystalline-lint .`,
V5/V6/V15/V26 estritos e `git diff --check` passaram.

O release P1504 compilou as duas fontes acima com exit 0 em
`/tmp/p1504-verify.8neoNC/`. `pdftotext -layout` coincidiu byte a byte
com os PDFs do cristalino pós-P1503. `pdftotext -bbox` também coincidiu
após excluir apenas `CreationDate` e `ModDate` do cabeçalho HTML; páginas,
texto e caixas das palavras não mudaram nesses controles. O texto desses
dois documentos já coincidia com o vanilla pinado no P1503; isto **não**
declara paridade integral nos demais recursos da linguagem.

## Tempo, CPU, RAM e interpretação

`hyperfine --warmup 3 --runs 10`, mesmo host e fontes, saída PDF em
`/tmp/p1504-verify.8neoNC/`. Os JSONs foram gravados às
2026-09-25T22:34:15Z e 22:34:21Z; hashes SHA-256
`53d3df59f3b7b7cd05b919e239e4b4b7ccfd517a22ccbc66730005775718d851`
e `a614961e0c4b311930b535467889a481956fd8fa52a0a55994322f1ef97be46a`.
Medições anteriores vêm do relatório P1503, mesma working tree base mais
este refactor; o último ajuste L0 mudou apenas comentário/hash do owner
`page_commit`, e o release reconstruído conservou o SHA-256 medido.

| Fonte | Vanilla P1503 → P1504 | Cristalino P1503 → P1504 |
|---|---:|---:|
| lorem | 137,2 ± 2,2 → 142,0 ± 4,9 ms | 333,1 ± 4,0 → 332,6 ± 2,4 ms |
| repeat | 134,2 ± 10,1 → 131,3 ± 0,8 ms | 291,6 ± 3,4 → 292,7 ± 8,8 ms |

No P1504, `hyperfine` registrou para o cristalino user/system
249,7/81,7 ms em lorem e 209,0/80,1 ms em repeat; são médias de CPU
do processo completo, não tempos exclusivos do Cursor. Cinco execuções
de repeat via `/usr/bin/time` deram RSS máximo 41.060–41.748 KiB no
cristalino e 30.224–30.976 KiB no vanilla, com user/system cristalino
0,20–0,21/0,07–0,08 s. Séries em
`/tmp/p1504-verify.8neoNC/repeat-{crystal,vanilla}-resources.txt`, hashes
SHA-256 `56d9ed3ce85b79158e7171f69f2aa867cbb08a6ccbbbbfd2ba907c9407a2669a`
e `977639b0d23baeec02891bfd51324e60365c606e822c183a4330c2f2275e5e68`.
As faixas de RSS P1503/P1504 se sobrepõem. **Não há ganho de tempo ou RAM
demonstrado**; a carga de fontes continua sendo um candidato externo mais
promissor, não otimizado neste passo.

## Raio de mudança e conclusão

Uma correção local de composição de página intermediária agora começa em
`page_commit.rs` + seu L0; uma de coluna física, em `columns.rs` + L0;
uma de emissão de tail, em `cell_tail_commit.rs` + L0; uma de quebra CJK,
em `word_segment.rs` + L0; uma de geometria de float, em `place.rs` + L0;
uma de replay, em `flow_suffix.rs` + L0. Alterações de estado/checkpoint,
`finish()` terminal ou política de avanço ainda atravessam owners. O
número mínimo de arquivos por mudança focal não caiu necessariamente;
o ganho verificado é a localização da responsabilidade, sem paridade
regredida nos controles executados.
