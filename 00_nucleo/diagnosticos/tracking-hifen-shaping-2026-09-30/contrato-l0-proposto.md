# Proposta L0 pendente: segurança de corte com tracking

Este documento é um diagnóstico para revisão, não um Prompt L0 vigente. Não autoriza código L1/L3.

## Medição anterior à decisão

[Proveniência](proveniencia.json), [teste causal](causal-probe.rs.txt), [log](causal-test.log), [sonda bilateral](bilateral/results.json) e [triângulo de caixas](triangulo-causal.json) fixam HEAD, working tree, fonte, binários, rustybuzz e comandos. No mesmo Noto Sans, o prefixo isolado `de` mede 1179 unidades. O corte após `de` em `development` é inseguro e o prefixo do run completo mede 1159; em `declaration`, é seguro e mede 1179. No PDF, `representa-` (corte seguro) precisa de 0,2 pt terminal; `de-` (inseguro) ganha 0,2 pt indevidamente se essa fórmula for aplicada a todos. Texto, quebra e caixa são observáveis de linguagem. A flag de shaping é dado mecânico, não alvo de paridade.

Fonte vanilla: `lab/typst-original/crates/typst-layout/src/inline/shaping.rs:546-571,642-710,1096-1101` reutiliza glyphs somente em limites seguros e remodela os demais. `inline/line.rs:173-195` insere hífen separado. Fonte Tekt: `01_core/src/compiler/layout/metrics.rs:240-257` expõe largura escalar; `03_infra/src/shaper/shaping.rs:96-101` descarta flags na medição; `paragraph_prepare.rs:245-252` recusa tracking não nulo com hifenização. Inferência refutável: um resultado de corte dependente do run completo é necessário para o subconjunto latino uniforme; uma função só das larguras isoladas que reproduzisse os dois cortes e PDFs refutaria a necessidade.

## Contrato público proposto — owner metrics.md

Após aprovação ADR-0127, adicionar a `FontMetrics`, com default neutro e forwarding em `&dyn FontMetrics`:

```rust
fn word_break_safety(
    &self, word: &str, break_byte: usize, size: Pt, style: &TextStyle
) -> Option<bool> { None }
```

`Some(true)`: offset UTF-8 em fronteira de cluster cujo slice do run completo pode ser reutilizado. `Some(false)`: fronteira válida que exige remodelagem. `None`: sem garantia, inclusive offset inválido, cluster indivisível, múltiplas faces/runs ou implementação sem suporte. Usar a mesma face, variações, idioma, direção e recursos OpenType da medição/emissão. O default `None` mantém implementações existentes e força fallback integral. Esta é uma **adição a trait pública**, portanto o gate ADR-0127 é obrigatório apesar do default.

## Cláusulas propostas por owner

- `infra/shaper/shaping.md`: consultar cluster e `unsafe_to_break` no run completo moldado com o setup existente; retornar `None` sem identidade única de face/run. Cache por palavra/estilo, se necessário; nenhuma lista de palavras.
- `infra/font_metrics/fallback.md`: delegar ao shaper sob tamanho e fonte efetivos, preservando fallback e variações; `FontBookMetrics`/`FixedMetrics` ficam no default até prova própria.
- `compiler/layout/paragraph_prepare.md`: admitir tracking não nulo com hifenização somente se todos os candidatos tiverem `Some`; qualquer `None` recusa o parágrafo inteiro. Escopo misto permanece fora.
- `compiler/layout/paragraph_hyphen.md`: range com ambos os limites seguros reutiliza o slice e conserva tracking terminal quando segue outro cluster; qualquer limite inseguro remodela o substring. Medir hífen separado; Simple/Optimized compartilham os mesmos candidatos.
- `compiler/layout/paragraph_select.md` e `paragraph_emit.md`: transportar em dados privados a largura/avanço decidido por corte; custo e emissão devem coincidir, sem duplicar tracking ou mudar SHY.
- `compiler/layout/paragraph_plan.md`: selecionar a peça global estática somente após certificação integral; manter `match` exaustivo e fallback.

Teste RED pós-gate: fake `FontMetrics` devolve `Some(true)`, `Some(false)` e `None` para candidatos; verificar largura, avanço, recusa integral e ausência de exceções por palavra. Teste L3 real reproduz `development` inseguro, `declaration` e `representation` seguros. Aceitar só após comparação bilateral de texto, linhas, páginas e caixas para tracking 0, ±0,2 e 1 pt em Simple/Optimized e controles de regressão. O teste causal atual é pré-gate e não altera código produtivo.

Tracking misto exige investigação própria da identidade lexical dos runs: `paragraph_prepare.rs:184-190` recusa `Styled` com tracking e `style_scope.rs:25-34` preserva essa identidade no percurso incremental. Este contrato não o promove, não move fases e não muda defaults.
