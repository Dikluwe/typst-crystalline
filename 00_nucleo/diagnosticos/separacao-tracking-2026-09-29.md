# Separação e organização de tracking — 2026-09-29

A correção foi atomizada por comportamento. Fronteiras textuais e alinhamento da baseline têm módulos e prompts proprietários; medição não examina frames e emissão não contém a fórmula da fronteira nem o alinhamento de carriers. As chamadas continuam estáticas, por descendência de módulo (ADR-0109).

## Evidência e proveniência

Antes das alterações, `text_measure.rs:85` possuía boundary, `word_layout.rs:89` possuía admit_baseline e `space.rs:162` possuía seu teste de integração. A observação motivou a divisão de responsabilidades; não demonstra intenção histórica. L0 foi escrito antes da movimentação do código. Não houve alteração de contrato público, comportamento por defeito ou fase: fluxo contínuo ADR-0127.

Estado: HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitada. `separacao-tracking-2026-09-29/initial.json` registra UTC, lista completa de alterações (`git status`), `git diff HEAD --stat` e hashes anteriores. `before/` conserva código e prompts integrais anteriores; o histórico removido dos prompts continua acessível nesses snapshots. `scope.json`, `review.diff.txt` e `scoped-seals.json` delimitam exclusivamente este lote; nenhum resselo global foi usado.

## Responsabilidades atuais

| Unidade | Responsabilidade | L0 proprietário |
|---|---|---|
| `01_core/src/compiler/layout/text_measure.rs` | Largura nominal/default e overrides de métricas | `00_nucleo/prompts/compiler/layout/text_measure.md` |
| `01_core/src/compiler/layout/text_tracking.rs` | Elegibilidade e valor de um intervalo entre unidades textuais | `00_nucleo/prompts/compiler/layout/text_tracking.md` |
| `01_core/src/compiler/layout/text_baseline.rs` | Admissão da baseline e alinhamento dos separadores iniciais | `00_nucleo/prompts/compiler/layout/text_baseline.md` |
| `01_core/src/compiler/layout/space.rs` | Produção, avanço e identidade dos separadores | `00_nucleo/prompts/compiler/layout/space.md` |
| `01_core/src/compiler/layout/word_layout.rs` | Aplicação da seleção e emissão de texto/highlight/substituição | `00_nucleo/prompts/compiler/layout/word_layout.md` |
| `03_infra/src/shaper/tracking.rs` | Resíduo fracionário e largura dos glyphs shaped | `00_nucleo/prompts/infra/shaper/tracking.md` |
| `01_core/src/compiler/layout/mod.rs` | Estado, composição e registro dos descendentes | `00_nucleo/prompts/compiler/layout.md` |

Os testes de fronteira acompanharam text_tracking; o teste integrado de espaços iniciais acompanhou text_baseline e continua usando produtores Space/Word reais. Os testes de seleção/emissão permanecem em word_layout. Não foram criados testes que apenas espelham a movimentação.

Os cinco prompts especializados existentes foram consolidados em contratos vigentes, eliminando a sequência de cláusulas históricas supersedidas. Cada módulo novo possui um único owner. O Núcleo `00_nucleo/prompts/_nuclei/layout/text-tracking.toml` permanece inalterado: text_measure deixa de consumir claims de fronteira que não lhe pertencem; text_tracking passa a consumi-las. Baseline usa os Núcleos de coordenadas e separadores, sem novo Núcleo redundante.

## Linha do histórico organizada

1. [Tracking até o PDF — 2026-09-28](tracking-pdf-2026-09-28.md): diagnóstico e correção da soma adicional no exportador.
2. [Resíduo e fronteiras — 2026-09-29](tracking-residual-2026-09-29.md): investigação causal da quantização e dos intervalos entre unidades.
3. [Correção e controles — 2026-09-29](correcao-tracking-2026-09-29.md): correção do resíduo, das fronteiras e dos espaços literais iniciais; resultados e limites bilaterais.
4. Este documento: separação dos owners e consolidação das especificações, conservando os observáveis da correção.

Relatórios e suas evidências permanecem em diagnosticos; contratos vigentes em prompts; claims compartilhadas em prompts/_nuclei. Os caminhos dos relatórios anteriores foram preservados.

## Verificação desta separação

- Core: 6.369 testes unitários, 8 de integração e 3 doctests passaram — 6.380 no total, zero falhas; 2 testes existentes ignorados. Evidência em `core-tests.log`.
- `cargo build --offline` e build release de typst-wiring concluíram: `build.log`, `release-build.log`.
- `crystalline-lint . --format sarif`: zero erros, 503 warnings e 1557 notes. Evidência em `lint-final.sarif`; não se declara zero avisos.
- 78 fontes dos lotes after/extra-after foram recompiladas com o binário anterior e o novo, nos modos padrão e compacto: 312 PDFs e 156 comparações. Texto extraído e traços de desenho coincidem integralmente, incluindo caixas de página, transformações, glyphs, posições, estilos e objetos. Apenas o nome do arquivo é removido da raiz do trace para comparar. Isto verifica preservação, não uma nova prova de paridade global com vanilla.
- Binário anterior SHA-256 `f6cd7011742a146b37775d93de73539208ff90de438e619e03763fc50b0faaf7`; novo `b8be5c19ae68d7c134d715895160826cf600a8a45df66b0f1593cbe9317f7b89`. Entradas, fontes, comandos, horários, estado e hashes completos em `verification-provenance.json`, `controls.json` e `verification-summary.json`. `final-receipt.json` confirma que as fontes permaneceram intactas durante a verificação.

Não se compara igualdade de bytes PDF como critério de língua; a comparação usa texto e operações de desenho observáveis. Não foram regenerados goldens ou introduzidos fatores numéricos para ocultar diferenças. O resíduo L3 conserva sua aritmética existente.

## Frentes que permanecem abertas

| Frente | Local da próxima investigação | Limite medido anteriormente |
|---|---|---|
| Tracking entre estilos diferentes | text_tracking e transições de estilo | Controle misto ainda difere do vanilla |
| RTL e limites entre unidades | text_tracking, segmentação e shaping | Direção RTL explicitamente fora da elegibilidade atual |
| Seleção global com tracking | paragraph_prepare/select/pagination | Preparação global recusa tracking; caso paginado continua 3+1 contra 2+2 |
| Espaços isolados e clusters | geometria auto, text_measure e medição shaped | Página auto só com espaços e fallback por codepoints não resolvidos |

Esses limites são os do relatório da correção; a separação não os fecha. Não há alegação nova de desempenho, superioridade ou estado da arte.

## Seguimento posterior

[Correção de escopos lexicais](correcao-tracking-escopos-2026-09-29.md) fecha a diferença medida de tracking entre escopos explícitos, inclusive repetidos, e confirma caixas/conteúdo oculto como barreiras. O quadro acima registra o estado na separação; as frentes de RTL, dimensão auto e seleção/paginação global continuam abertas.
