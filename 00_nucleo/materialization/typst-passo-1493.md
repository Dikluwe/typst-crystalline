# Passo 1493 — posições locais no contexto de `html.frame`

**Estado:** PLANEADO; requer gate humano ADR-0127 antes de código. Dependência: P1492.

## Medição anterior à decisão

O vanilla pinado devolve `none` para `page()` em HTML e, dentro de `html.frame`, `position()` com coordenadas locais `x/y` sem chave `page` (`lab/typst-original/crates/typst-html/src/introspect.rs:100-119,165-205` e `typst-library/src/introspection/position.rs:47-62`). A fixture `P1489-I-001` cobre as duas metades. No cristalino, `03_infra/src/pipeline/context_stabilization.rs:176-215,620-660` prepara o contexto HTML antes de `03_infra/src/pipeline.rs:318-344` fazer o layout da ilha; o L0 `00_nucleo/prompts/infra/pipeline/context_stabilization.md:273-280` manda falhar fechado para leitura de posições. `00_nucleo/prompts/compiler/layout/html_frame.md` já produz página geométrica **interna**, que não é uma página pública HTML. Proveniência do RED: relatório P1489; revalidar antes de decidir.

Inferência: responder `x/y` corretamente exige associar localização e geometria à ilha e estabilizar contexto no momento causal adequado; refutação: prova de que o pipeline atual já disponibiliza ambas sem reordenar fases nem contratos.

## Proposta L0 e parada

1. Medir probes dentro de frames simples, aninhados e com múltiplas localizações; controlar `page() = none`, `position() = (x, y)` sem `page`, identidade estável e ausência de paginação do documento HTML inteiro (ADR-0128).
2. Auditar/redigir L0 para cada consumer realmente alterado: `00_nucleo/prompts/compiler/eval/call_dispatch.md`, `00_nucleo/prompts/compiler/layout/html_frame.md`, `00_nucleo/prompts/infra/pipeline/context_stabilization.md` e `00_nucleo/prompts/infra/pipeline.md`; se tocar `Position`/`Introspector`, também `00_nucleo/prompts/entities/position.md`/`00_nucleo/prompts/entities/introspector.md`. Especificar fase, fluxo de dados e fail-closed; não criar owner 1:N.
3. **Parar e solicitar confirmação específica do dono** para mudança de fase/trait/entidade (ADR-0127). Aprovações de passos anteriores não cobrem automaticamente este desenho.
4. Somente depois: escrever testes RED por target, implementar dentro da ilha sem `PagedDocument` global, confirmar GREEN da fixture inteira `P1489-I-001` nos três perfis HTML, dos casos novos de frames aninhados/múltiplos e dos negativos (fora da ilha segue `none`; página interna não vaza); só então mudar `expected_state` para `MATCH` e revalidar paginado/bundle misto. Executar build seriado, lint, diff-check e relatório de proveniência.

Não reduzir a aceitação a `none` fora do frame, nem usar número lógico interno da `Page` como `location.page()` público.
