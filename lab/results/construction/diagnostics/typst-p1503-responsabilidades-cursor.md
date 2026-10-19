# P1503 — responsabilidades do Cursor e baseline de separação

## Proveniência

Medição estrutural e bilateral em 2026-09-25T19:48:28Z–19:57:57Z,
HEAD `0de5edabed197d23b647cfc68ebefda2f5c2a8d5`. Nenhum arquivo
rastreado alterado (`git diff HEAD --stat` vazio); o passo
`00_nucleo/materialization/typst-passo-1503.md` estava não rastreado.
`cursor.rs` SHA-256 `f8e5694658685b08f9f4f84dd5b1aec0f73a20860dd0436eb66d714b1651462e`.
Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`;
cristalino release SHA-256
`c0663911190b0afcc9aad86ee78fb65abcd11687943611c6a47f27d3a0944d48`.

## Inventário funcional antes da decisão

| Região em `01_core/src/compiler/layout/cursor.rs` | Funções e responsabilidade | Estado/owners tocados |
|---|---|---|
| `:35-105` | `deferred_frame_origin_y` mede origem de float; `justify_frame_item` move itens da linha | `FontMetrics`; o primeiro pertence a floats, o segundo a fechamento |
| `:113-405` | `word_width`, `space_width`, `note_inline_extent`, `shift_current_line_y`, `baseline_y`, `ensure_initial_baseline`, `push_text`, `layout_word`, `layout_chunk`, `align_current_line_rtl` | cursor, estilo/métricas, `current_line`, página quando wrap exige avanço |
| `:406-569` | `expand_fr_spacings`, `justify_current_line` | `pending_fr`, `pending_repeats`, `pending_fr_natural_widths`, `justify_opportunities`, `current_line`, `cursor_x`; `repeat.rs` materializa cópias |
| `:570-1038` | `flush_line`, `arm_wrapped_inline_prefix_line`, `flush_wrapped_inline_prefix_line`, `flush_line_with_outer_singleton` | baseline, inline extents, `current_line/current_items`, numbering e decoração; chama hooks de cursor (`new_page`, `shift_current_line_y`, `align_current_line_rtl`) e observa transação de sufixo do owner `layout/mod.rs` |
| `:1039-1420` | `new_page`, `close_current_column`, `merge_column_items`, `finish_columns`, `start_next_column`, `start_column` | página, colunas, notas, fixups, `columns.rs`, `page_run.rs` |
| `:1421-1764` | `flush_fitting_pending_floats`, `layout_flow_suffix_transaction`, `finish_float_prefix_at_marker`, `flush_pending_floats`, `translate_deferred_float`, `current_page_number` | `place.rs`, `flush.rs`, checkpoint do `layout/mod.rs`; o helper `deferred_frame_origin_y` fica aqui |
| `:1765-1877` | `flush_pending_cell_tails` | caudas de Grid e item de página |
| `:1878-1985` | `line_segmenter`, `word_needs_line_segmentation`, `needs_cjk_quote_tailoring`, `is_cjk_context_char`, `filter_cjk_quote_breakpoints`, `layout_segmented_word` | segmentação ICU/CJK antes do fechamento |

`layout/mod.rs:797-874,1215-1417` possui/captura/restaura os campos da
transação de sufixo; `cursor.rs:721-765,1026-1036` apenas sinaliza rejeição
quando o fechamento não cabe. A fronteira candidata não migra esse estado.
`sequence.rs:902`, `linebreak.rs:41-43`, `columns.rs:150,495,669`, `repeat.rs`
via `pending_repeats` e `page_run.rs:31,92` são chamadores/colaboradores
relevantes. `place.rs` continua dono de fitting de float e `columns.rs` de
consolidação de numeração segmentada.

## Baseline bilateral e limites

Fontes: `tools/perf/corpus/p922923-canonical/02-lorem.typ` SHA-256
`515428fc6da638a5744ee0078d330f5fe849ff6c22c77285a3478439fe53049c`
e `lab/parity/corpus/p1407/repeat.typ` SHA-256
`4242ed2345e44d8babc48ac7f51b5dd174804944bec67ffa746fad6487b9e90a`.
Ambos os binários compilaram com exit 0; `pdftotext -layout` coincidiu
byte a byte para cada par de PDFs. PDF bruto não é gate. Saídas e medições
ficaram em `/tmp/p1503-baseline.wOkZLs/`.

`hyperfine --warmup 3 --runs 10` no mesmo host/cwd, saída PDF em `/tmp`:

| Entrada | Vanilla média ± desvio | Cristalino média ± desvio | Observação |
|---|---:|---:|---|
| lorem | 139,4 ± 4,0 ms | 336,6 ± 6,2 ms | `lorem-timings.json` SHA-256 `4211f6470cf3e55521ebf2946ac2e23a2ce1718f866806db367f58006bc961d5` |
| repeat | 134,4 ± 2,3 ms | 290,6 ± 2,0 ms | `repeat-timings.json` SHA-256 `ce3569b5019ae19d4cee04c60a7d5f2afd36e18c69aac9c7b61ddb11c23d039f` |

Cinco execuções independentes de `repeat` sob `/usr/bin/time -f %M`:
vanilla 30.432–30.880 KiB, cristalino 41.416–41.748 KiB de RSS máximo.
`--timings-json` do cristalino em uma execução de `repeat`: `layout_ms=0,705052`,
`total_ms=4,553927`; o campo total exclui criação do `SystemWorld`.
Logo, estas medições **não** atribuem a diferença externa ao fechamento.
`perf stat` foi recusado pelo host (`perf_event_paranoid=4`); não há perfil
de função/CPU neste snapshot. Qualquer afirmação de ganho por extração fica
pendente de medição posterior.

## Decisão restrita da fronteira

A unidade proposta é **fechamento de linha**, contendo somente
`justify_frame_item`, `expand_fr_spacings`, `justify_current_line` e os
quatro métodos `flush*`/arm listados acima. O fechamento **consulta** limites
de região e pede `new_page`, mas não move nem duplica a política de página,
float, colunas ou checkpoint. O estado permanece em `Layouter`, já possuído
por `compiler/layout.md`; não criar cópia ou interface pública para o mover.

Um módulo irmão `01_core/src/compiler/layout/line_commit.rs` conserva a
visibilidade `pub(super)` dos métodos para os mesmos descendants de layout
sem import reverso ou caminho `super::` adicional. O nome filho previsto
no passo era provisório; a escolha de irmão é mecânica de Rust, não mudança
de comportamento. O novo consumer exige L0 próprio
`00_nucleo/prompts/compiler/layout/line_commit.md`, atualização do L0 de
`cursor.rs` e do L0 de `layout/mod.rs` para registrar o módulo. Se o build ou
os testes exigirem alargar visibilidade/duplicar estado, rejeitar esta
fronteira e voltar ao diagnóstico.

## Execução P1503 e comparação pós-extração

Em 2026-09-25T20:23:53Z, mesmo HEAD `0de5edab...`, working tree
não commitada: `git diff HEAD --stat` mostrou
`layout.md` (16 linhas alteradas), `cursor.md` (26),
`cursor.rs` (698) e `layout/mod.rs` (8), total rastreado de
59 inserções/689 remoções. Não rastreados: este diagnóstico, o passo
`typst-passo-1503.md`, `compiler/layout/line_commit.md` e
`compiler/layout/line_commit.rs`. O release final manteve SHA-256
`aa1128d20fb3a8ef83bd8155560bbba6116414fd4a3bb39931b704c2c89b5de2`
após rebuild em 2026-09-25T20:24:55Z.

A extração moveu 639 linhas de métodos e 39 de helper sem alterar fórmulas.
`cursor.rs` caiu de 1.990 para 1.324 linhas; o owner novo possui 724
linhas incluindo seu teste. O teste arquitetural `p1503_*` foi RED
(exit 101, 0/1) às 20:03:59Z antes da transferência e GREEN (1/1)
às 20:06:11Z depois dela. Os logs estão em
`/tmp/p1503-baseline.wOkZLs/{red-test,green-test}.log`, SHA-256
`6b95523c5c8b907d3b97dae891b85d24f404d0cb4beaff2e3ee2bbbb4cfd05d7`
e `6e4cb5e5ff8df1638443ffcac11df6e99aac7acfd0a77398aea2b3b842ebc077`.
O teste unitário do helper de justificação passou junto do estrutural
(2/2). A suíte L1 final passou 6.196/6.196, com dois testes históricos
de recursão filtrados por stack overflow; log SHA-256
`5768ea2cfa39951ce769f1b5eab65c139d8d99a301d2598e66e6cf1de3dc85e2`.
P1407 L3 passou 5/5, P1493 L3 6/6 e P1502 L3 4/4.
O lint completo terminou exit 0 depois de corrigir V2 com teste no
novo módulo; V5/V15/V26 e dry-run de hashes ficaram verdes.

As duas fontes do baseline compilaram após a extração com exit 0.
`pdftotext -layout` permaneceu idêntico ao cristalino anterior e ao
vanilla. `pdftotext -bbox` do cristalino antes/depois coincidiu
após retirar somente `CreationDate` e `ModDate` do cabeçalho HTML;
páginas, texto e caixas das palavras não mudaram. Isto não afirma
paridade integral fora dos dois documentos.

`hyperfine --warmup 3 --runs 10`, mesmo host/cwd/fontes, controle
vanilla executado novamente:

| Entrada | Vanilla antes → depois | Cristalino antes → depois |
|---|---:|---:|
| lorem | 139,4 ± 4,0 → 137,2 ± 2,2 ms | 336,6 ± 6,2 → 333,1 ± 4,0 ms |
| repeat | 134,4 ± 2,3 → 134,2 ± 10,1 ms | 290,6 ± 2,0 → 291,6 ± 3,4 ms |

JSONs depois: `lorem-after-timings.json` SHA-256
`2b5ca8829872223e1d565412e1ae8a086a5782a6e3078793ca3fb8e93f697137`;
`repeat-after-timings.json` SHA-256
`f9b511a40ac8d8673491d387804d44f6cbaf4492ad05acda98af570388bab29a`.
O vanilla de repeat teve um outlier de 162,7 ms; não atribuir o
delta pequeno do cristalino ao refactor. Cinco RSS máximos de repeat
depois: vanilla 30.428–30.808 KiB, cristalino 41.468–42.052 KiB;
as faixas cristalinas antes/depois sobrepõem-se. **Não há ganho de
tempo ou RAM demonstrado**.

## Localização externa do gargalo de arranque

Harnesses `/tmp/p1503-baseline.wOkZLs/profile_startup.rs` e
`profile_vanilla_fonts.rs`, executados em 2026-09-25T20:21–20:23Z,
mediram 10 processos por rotina com `Instant` fora do produto.
Fontes release linkadas: `libtypst_infra-ed65bb36a0920044.rlib`
SHA-256 `07e424bb925c0433ae13fd8334e8805ff2d192b1c10cb0b4e47fde9f5d92c64b`
e `libtypst_kit-6459ab740790fb21.rlib` SHA-256
`7fc6cd459c59367f86c88a7ea5b4beba9c80ddcd4a8e83368a70382f75cc0ae2`.
Harnesses executáveis SHA-256 `1b34af1d18e20c8d87a5b81ba093b31cf35536a203c86dac477ca64a8da44178`
e `b0e989b48a5a4758150952183628e36ebc3bf388bb3b8c01abd174f510852e00`.
Cada catálogo resultou em 1.112 faces/slots no mesmo host.

| Rotina isolada | Cristalino média (10) | Vanilla média (10) |
|---|---:|---:|
| fontes do sistema | 270,203 ms | 123,214 ms |
| fontes embutidas | 9,051 ms | 0,751 ms |
| `SystemWorld::new` | 0,101 ms | não medido |
| `with_fonts_and_system` completo | 275,516 ms | não há chamada homóloga |

`03_infra/src/world.rs:337-355` invoca a rotina cristalina medida;
`lab/typst-original/crates/typst-cli/src/fonts.rs:36-51` invoca
`typst_kit::fonts::system`. A diferença isolada no scan de sistema
é ~147 ms, próxima da diferença de wall time de repeat, mas a
semelhança não prova que todo o delta CLI venha daí: os harnesses
não incluem a mesma composição de CLI/pipeline e usam processos
independentes. As séries brutas estão em
`/tmp/p1503-baseline.wOkZLs/{startup,vanilla}-*-ms.txt`, com SHA-256
registrados no terminal da execução. O cristalino também gasta ~9 ms
em fontes embutidas; qual função interna do scan explica o excedente
continua hipótese para passo próprio (por exemplo, parse/cobertura).

`perf stat` foi bloqueado por `perf_event_paranoid=4`, portanto não
há tempo exclusivo medido de `flush_line` nem de transição de região.
O `--timings-json` final de repeat mediu todo layout em 0,596387 ms
numa execução (arquivo SHA-256
`9cf98263a82abbf709be3d8b2b36671649ba776208ba87bfaababbef01be50`);
isto não resolve o microperfil interno, mas refuta o Cursor como
explicação da maior parte dos ~292 ms externos neste vetor. Não
adicionar relógio a L1 ou campo público de timings sem gate ADR-0127.

## Raio de mudança e limite da conclusão

Numa correção futura **somente da fórmula local de justificação**, o
arquivo/owner a consultar seria `line_commit.rs` +
`compiler/layout/line_commit.md`, contra `cursor.rs` +
`compiler/layout/cursor.md` antes: o número de arquivos não caiu
(2→2), mas a responsabilidade deixou de compartilhar o arquivo com
página, floats e segmentação. Correções que mudem estado transacional
continuam tocando `layout/mod.rs` e owners vizinhos; a atomização
não as tornou locais automaticamente. O ganho demonstrado é de
localidade arquitetural, não de performance.
