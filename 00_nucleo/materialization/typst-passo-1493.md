# Passo 1493 — posições locais no contexto de `html.frame`

**Estado:** CONCLUÍDO em 2026-09-24; gate ADR-0127 confirmado pelo dono. Dependência: P1492.

## Medição anterior à decisão

O vanilla pinado devolve `none` para `page()` em HTML e, dentro de `html.frame`, `position()` com coordenadas locais `x/y` sem chave `page` (`lab/typst-original/crates/typst-html/src/introspect.rs:100-119,165-205` e `typst-library/src/introspection/position.rs:47-62`). A fixture `P1489-I-001` cobre as duas metades. No cristalino, `03_infra/src/pipeline/context_stabilization.rs:176-215,620-660` prepara o contexto HTML antes de `03_infra/src/pipeline.rs:318-344` fazer o layout da ilha; o L0 `00_nucleo/prompts/infra/pipeline/context_stabilization.md:273-280` manda falhar fechado para leitura de posições. `00_nucleo/prompts/compiler/layout/html_frame.md` já produz página geométrica **interna**, que não é uma página pública HTML. Proveniência do RED: relatório P1489; revalidar antes de decidir.

Inferência: responder `x/y` corretamente exige associar localização e geometria à ilha e estabilizar contexto no momento causal adequado; refutação: prova de que o pipeline atual já disponibiliza ambas sem reordenar fases nem contratos.

## Proposta L0 e parada

1. Medir probes dentro de frames simples, aninhados e com múltiplas localizações; controlar `page() = none`, `position() = (x, y)` sem `page`, identidade estável e ausência de paginação do documento HTML inteiro (ADR-0128).
2. Auditar/redigir L0 para cada consumer realmente alterado: `00_nucleo/prompts/compiler/eval.md`, `00_nucleo/prompts/compiler/eval/call_dispatch.md`, `00_nucleo/prompts/compiler/introspect.md`, `00_nucleo/prompts/compiler/layout.md`, `00_nucleo/prompts/compiler/layout/html_frame.md`, `00_nucleo/prompts/compiler/layout/helpers.md`, `00_nucleo/prompts/compiler/layout/link.md`, `00_nucleo/prompts/compiler/layout/metrics.md`, `00_nucleo/prompts/compiler/layout/stack.md`, `00_nucleo/prompts/compiler/layout/sub_frame.md`, `00_nucleo/prompts/compiler/layout/transform.md`, `00_nucleo/prompts/entities/position.md`, `00_nucleo/prompts/entities/introspector.md`, `00_nucleo/prompts/infra/measurements.md`, `00_nucleo/prompts/infra/pipeline.md` e `00_nucleo/prompts/infra/pipeline/context_stabilization.md`. Especificar fase, fluxo de dados e fail-closed; não criar owner 1:N.
3. **Parar e solicitar confirmação específica do dono** para mudança de fase/trait/entidade (ADR-0127). Aprovações de passos anteriores não cobrem automaticamente este desenho.
4. Somente depois: escrever testes RED por target, implementar dentro da ilha sem `PagedDocument` global, confirmar GREEN da fixture inteira `P1489-I-001` nos três perfis HTML, dos casos novos de frames aninhados/múltiplos e dos negativos (fora da ilha segue `none`; página interna não vaza); só então mudar `expected_state` para `MATCH` e revalidar paginado/bundle misto. Executar build seriado, lint, diff-check e relatório de proveniência.

Não reduzir a aceitação a `none` fora do frame, nem usar número lógico interno da `Page` como `location.page()` público.

## Gate redigido em 2026-09-24

A medição foi repetida no HEAD `b5abe9b288ce168c494a4c5172830e325b9aedae`.
A proposta separa `HtmlPosition` de `Position`, acrescenta consulta tracked
específica ao Introspector e insere fixpoint apenas para leituras efetivas de
posição dentro de ilhas. O documento HTML inteiro continua não paginado e a
página intrínseca nunca é projetada.

Medição, refutações, owners e pedido de confirmação:
`00_nucleo/diagnosticos/typst-p1493-gate-posicoes-html-frame.md`.


## Fechamento GREEN

O escopo ratificado foi materializado sem paginação global do target HTML:

- `HtmlPosition` e o store tracked do `Introspector` são distintos de `Position`/PageStore;
- a leitura efetiva de `location.position()` seleciona a fase localizada;
- `html.frame` devolve geometria detalhada por carrier privado de `ContextBlock`, inclusive através de wrappers, links e transformações, e remove o carrier antes da exportação;
- L3 associa `ContextBlock id` à `Location`, estabiliza somente as ilhas HTML e mantém stores paginados independentes no bundle;
- múltiplas Locations, frame aninhado, contexto externo negativo e bundle misto ficaram GREEN bilateralmente.

A tentativa adversarial de generalizar o store a qualquer Location consultada revelou reuso do ordinal raw entre gerações; essa associação insegura foi removida. O limite ficou explícito no L0 de `infra/pipeline.md` e exige passo/gate próprio de identidade estável.

Owners efetivamente ressellados: os dezesseis inicialmente ratificados, mais `compiler/columns.md`, `compiler/layout/boxed.md`, `compiler/layout/cursor.md`, `compiler/layout/equation.md`, `compiler/layout/page_run.md`, `compiler/layout/pdf_artifact.md`, `compiler/layout/place.md`, `compiler/layout/sequence.md` e `compiler/layout_outline.md`, necessários para neutralidade e transporte do carrier.

## Validação e proveniência final

Medição bilateral em `2026-09-24T18:00:54-03:00`, HEAD `b5abe9b288ce168c494a4c5172830e325b9aedae`, working tree P1493 não commitado. Binários: cristalino release SHA-256 `35a2fdea38c3e5a9b8b0c2a8a93c1cf9f4b6439965ed0103aa0e564c9ca3ca83`; vanilla pinado SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

- L1 P1493: 11/11 testes GREEN;
- L3 `--lib` P1493: 6/6 testes GREEN;
- runner: 56/56 testes GREEN;
- `P1489-I-001`: MATCH em `html`, `html+a11y` e `html+bundle`;
- `P1493-I-001`, `P1493-I-002` e `P1493-I-003`: MATCH, com ordem forward/reverse idêntica;
- controles `P1492-I-001` e `P1487-X-002`: MATCH;
- `cargo build`, `cargo build --release`, `crystalline-lint --fix-hashes .`, `crystalline-lint .` e `git diff --check`: sucesso; lint com zero violations.

A comparação de `P1493-I-003` foi restringida a `artifact_presence`: suas asserções de linguagem já provam `page() == none` e posição 30pt/20pt. A diferença morfológica preexistente do SVG vazio (0x0 cristalino versus 100x50 vanilla) é eixo separado e não foi usada para negar crédito à posição local.

A compilação da suíte de integração inteira de `typst-infra` continua bloqueada fora deste passo por dois construtores preexistentes de `Page` sem `logical_number` em `03_infra/tests/p1405_pdf_label.rs` e `03_infra/tests/p1250_svg_integrated.rs`; a biblioteca e os testes focais do P1493 compilam e passam.

Detalhes, digests e estado exato da árvore: `00_nucleo/diagnosticos/typst-p1493-gate-posicoes-html-frame.md`.
