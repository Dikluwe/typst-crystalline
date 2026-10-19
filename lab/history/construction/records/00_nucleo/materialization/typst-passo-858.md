# Prompt — typst-passo-858: implementar a Opção 1 (injeção de `FontMetrics` no `Engine`) para fechar `measure()` (achado #34, DEBT-69)

**Origem**: decisão do dono, tomada sobre o levantamento de P857 — Opção 1 reformulada (inversão de dependência via trait, o mesmo padrão já usado por `Layouter`), não a abordagem 4.2 (fase de realização). A razão explícita da escolha: manter a arquitetura própria do cristalino, sem importar a estrutura de fases do vanilla que o projeto já diverge dela de propósito em outros pontos.
**Estado**: aguardando execução — implementação real, seguindo o roteiro que o próprio P857 já deixou pronto (§6 do relatório).

---

## O que já está confirmado, não precisa resondar

- O trait `FontMetrics` já existe em L1 (`01_core/src/engine/layout/metrics.rs:22`), com `FixedMetrics` (L1, heurística) e `FallbackFontMetrics` (L3, real) como implementações.
- `Engine` (`01_core/src/entities/engine.rs:40`) já tem o precedente de capacidade externa injetada como trait object (`world: &'a dyn World`) — adicionar `font_metrics` segue a mesma forma.
- ~13 sites de construção literal de `Engine { ... }` em L1, listados no relatório de P857 (`eval/mod.rs`, `eval/markup.rs`, `eval/modules.rs`, `eval/closures.rs`, `stdlib/eval.rs`, `stdlib/mod.rs`, `eval/tests.rs`, e as macros `with_engine!` em `introspect.rs`, `introspect/fixpoint.rs`, `introspect/from_tags.rs`).
- Ponto de injeção da métrica real: `03_infra/src/pipeline.rs:107-160` (`expand_context_blocks`), onde `world` já está disponível para construir `FallbackFontMetrics::new(world)`.
- `FallbackFontMetrics` tem caches internas (`Arc<Mutex<...>>`) — instanciar uma métrica só, fora do loop de `ContextBlock`, e reutilizá-la entre todos eles (evitar recriar por bloco).

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core`/`typst-infra`/`typst-shell`/`typst-wiring` a bater com os testes novos, **discriminada por crate** (mesma lição de P851-856 — não reportar um número único sem dizer de que crate é).

---

## Passo 1 — L0 antes do código

Atualizar `00_nucleo/prompts/entities/engine.md` para incluir o campo `font_metrics` — protocolo de nucleação do projeto (L0 antes do código), como o próprio P857 já registrou como primeiro item do roteiro.

## Passo 2 — Adicionar o campo a `Engine`

`font_metrics: &'a dyn FontMetrics` em `01_core/src/entities/engine.rs`. Decidir (e documentar a decisão) se o campo é obrigatório ou `Option<&'a dyn FontMetrics>` — considerar que nem todo `Engine` construído em L1 precisa de métricas reais (ex.: eval inicial, antes de qualquer `#context`); um `Option` com fallback para `FixedMetrics` pode ser mais seguro que forçar todos os ~13 sites a fornecer algo logo de cara. Registrar a escolha e o motivo no relatório.

## Passo 3 — Propagar pelos ~13 sites

Para cada ponto de construção de `Engine` listado por P857: se o site já tem acesso a métricas reais (ou pode ter, sem mudar o que ele constrói), propagar; caso contrário, usar `&FixedMetrics` (o comportamento atual, sem regressão). Não é preciso que todos os 13 ganhem métricas reais — só o(s) que efetivamente levam a `measure()` sendo chamado precisam.

## Passo 4 — Injetar a métrica real no ponto certo

Em `expand_context_blocks` (`03_infra/src/pipeline.rs`), construir `FallbackFontMetrics::new(world)` uma vez, reutilizar entre os `ContextBlock`s do documento, e injetar no `Engine` usado por `apply_func` nesse ponto.

## Passo 5 — Ajustar `measure_content_real`

`01_core/src/engine/layout/mod.rs:1790` — trocar o `FixedMetrics` hard-coded por `engine.font_metrics` (a métrica agora disponível via injeção), mantendo `FixedMetrics` como fallback nos contextos onde métricas reais não estão disponíveis (ou não fazem sentido — confirmar quais).

## Passo 6 — Validação

1. Recompilar. Repetir os casos já medidos em P842/P849 (`measure([hello])`, `measure([x])`, `measure([abcd])`, `measure([a b])`, `measure([])`, `measure([x])` com tamanho de fonte customizado) — confirmar que agora batem com o vanilla.
2. Confirmar que documentos sem nenhum `#context`/`measure()` continuam idênticos ao comportamento anterior (o objetivo explícito de P849 §6.d — "impacto em documentos sem measure() deve ser zero").
3. Testar um documento com múltiplos `#context`/`measure()` para confirmar que a reutilização da métrica (cache) não introduz nenhum comportamento estranho (resultado errado por cache compartilhado entre contextos diferentes, por exemplo).
4. Suíte completa por crate (`typst-core`, `typst-infra`, `typst-shell`, `typst-wiring`), comando real `cargo test --workspace` com números discriminados.
5. `crystalline-lint .` — zero violações novas.

## Passo 7 — Fechar DEBT-69

Atualizar `00_nucleo/diagnosticos/debt/DEBT.md`: DEBT-69 passa de ABERTA para FECHADA, com referência a este passo e à decisão do dono (Opção 1 reformulada, não a 4.2), incluindo o motivo registrado (manter a arquitetura própria do cristalino).

## Relatório

`00_nucleo/diagnosticos/typst-passo-858-relatorio.md` com: a decisão do Passo 2 (campo obrigatório ou `Option`) e o motivo, o diff completo, a medição antes/depois dos casos de `measure()`, a confirmação de zero impacto em documentos sem contexto, e as contagens de teste discriminadas por crate.
