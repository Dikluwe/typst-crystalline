# Passo 1492 — localização HTML fora de `html.frame`

**Estado:** PLANEADO. Dependência: P1489; primeiro recorte da sonda `P1489-I-001`.

## Medição anterior à decisão

No target HTML do vanilla pinado, `location.page()` e `location.position()` fora de uma ilha paginada devolvem `none` (`lab/typst-original/crates/typst-library/src/introspection/location.rs:87-132`). No cristalino, `01_core/src/compiler/eval/call_dispatch.rs:2125-2165` fabrica página 1 e coordenada 0,0 quando não encontra posição. A fixture `lab/parity/matrix/fixtures/location-html.typ` mede também um caso **dentro** de `html.frame`; portanto passar só a metade externa não fecha `P1489-I-001`. Baseline, binários e worktree: relatório P1489; repetir a sonda com proveniência atual.

Inferência: a ausência de posição no target HTML externo deve permanecer `none`, sem transformar a `Page` intrínseca de `html.frame` em página pública. Refutação: sonda vanilla em contexto externo que reporte `page` ou `position`.

## Contrato e sequência

1. Medir `page()`/`position()` fora da ilha em `html`, `html+a11y` e `html+bundle`; usar documento paginado e bundle misto como controles. Registrar o alvo semântico e a feature separadamente (ADR-0128).
2. Atualizar primeiro o L0 proprietário `00_nucleo/prompts/compiler/eval/call_dispatch.md` e auditar `00_nucleo/prompts/compiler/introspect.md` somente se seu consumer mudar; auditar também `00_nucleo/prompts/compiler/eval.md` se o replay de leituras em `eval/mod.rs` mudar. Não reutilizar `Position { page, point }` paginada para expressar `none`.
3. Criar testes RED e uma sonda independente na matriz **apenas** para o contexto externo, corrigir o dispatch, confirmar GREEN desse recorte e revalidar os controles paginados. Manter a fixture completa `P1489-I-001` RED até P1493, se a parte interna ainda falhar.
4. Rodar testes focais, build com um job, `crystalline-lint .` e `git diff --check`; registrar HEAD/worktree e hashes.

**Gate ADR-0127:** correção interna de paridade pode fluir com L0 primeiro e RED→GREEN. Se exigir mudança de trait, entidade, assinatura pública ou ordem do pipeline, parar e remeter o desenho ao P1493.
