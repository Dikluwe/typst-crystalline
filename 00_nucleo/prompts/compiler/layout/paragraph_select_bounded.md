# L0 — paragraph_select_bounded
Hash do Código: adc9f605

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_select_bounded.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Medição anterior à decisão

`01_core/src/compiler/layout/paragraph_select.rs:101-176` usa DP exata sobre todos os finais de palavra, prefixos para medir intervalos em O(1), descarte seguro de predecessores overfull e desempate pelo predecessor posterior. `01_core/src/compiler/layout/paragraph_plan.rs` chama somente essa seleção. A fonte vanilla pinada em `lab/typst-original/crates/typst-layout/src/inline/linebreak.rs:227-381` combina limite aproximado e busca limitada. Esta é uma observação mecânica de código, sem medição de tempo ou declaração de equivalência com o vanilla.

## Decisão experimental

Protótipo privado compilado apenas em testes. Recebe a mesma entrada preparada, largura, tamanho, justificação e custos que o seletor de palavra exato. Não acrescenta API, modo, flag, default, fase de pipeline ou troca de produção. Não cobre candidatos de hifenização, UAX14 completo ou largura variável.

Calcular um limite superior válido somando os custos exatos das linhas da solução Simple existente, mas medir essas linhas pela mesma fórmula de prefixos do caminho Optimized. Como cada custo de linha é um quadrado e portanto não negativo, um prefixo acumulado estritamente maior que o limite não pode participar de solução melhor ou empatada; só esses prefixos podem ser podados. Preservar o descarte overfull e o desempate `<=` da DP exata. Se o limite ou resultado não for finito, delegar à seleção exata. Não aplicar margem aproximada nem mudar a função de custo.

Testes diferenciais devem comparar a sequência completa de ranges com a seleção exata em entradas variadas: vazio, overlong, justificado/não justificado, larguras/gaps, tracking e custo runt negativo. Expor contagens de candidatos avaliados e podados apenas ao teste; isso não prova ganho temporal. Build e crystalline-lint sem violações.

## Microbenchmark comparativo — 2026-09-30

Medição anterior à decisão: os testes diferenciais do protótipo demonstram equivalência das quebras e poda efetiva, mas não medem tempo; ambos os caminhos estão acessíveis neste consumer sob `cfg(test)`. Adicionar microbenchmark Criterion ignorado por padrão, em release, com as mesmas instâncias `Prepared` reutilizadas por ambas as variantes. Medir seleção completa, incluindo a construção do limite guloso no protótipo, com entradas determinísticas curta/estreita, média e larga. Consumir o resultado via black_box; verificar equivalência antes de cronometrar. Usar o mesmo custo, tamanho de fonte, justificação e largura por par. Os resultados são locais e devem registrar commit/árvore, comando, ambiente e incerteza; não extrapolar diretamente para o compilador completo.

## Escala de 1.000 palavras — 2026-10-01

Medição anterior à decisão: o relatório `00_nucleo/diagnosticos/microbench-paragraph-select-bounded-2026-09-30.md` contém somente 40 e 160 palavras; 160 palavras com 240pt e 1200pt favoreceram a poda, enquanto 40 palavras com 76pt não. Estender o mesmo benchmark aos três regimes de largura com 1.000 palavras, mantendo gerador, fonte, gaps, custos, verificação diferencial e medição da seleção completa. Registrar resultados e proveniência separados; não inferir escala pelo conjunto anterior.

## Comparação de podas por linha — 2026-10-02

Medição anterior à decisão: `lab/typst-original/crates/typst-layout/src/inline/linebreak.rs:264-344` estabelece o custo de uma linha subcheia como limite inferior para tentativas seguintes, desde que não haja largura negativa. O código de `kitlangton/justice` no commit `86dbc6da8cc2059cabb2a67c0aa42d773d5023e2`, `src/engine.ts:340-400`, percorre predecessores em ordem inversa e interrompe após transbordamento somente sob guarda de monotonicidade. `paragraph_select_bounded.rs:91-143` hoje possui apenas o limite superior global e visita os demais pares antes de podar. A descoberta externa é indício de mecânica, não contrato de linguagem nem prova de ganho local.

Adicionar duas variantes privadas sob `cfg(test)` no mesmo consumer, sem alterar API, modo, default ou fase. Ambas usam a mesma função `line_cost`, medidas prefixadas e desempate do seletor exato. A primeira mantém a solução Simple como limite superior, percorre inícios crescentes e usa o custo da primeira linha subcheia não final como limite inferior para os inícios posteriores no mesmo fim; só aplica quando todas as contribuições de largura e tracking relevantes são não negativas. A segunda percorre inícios decrescentes e interrompe após a primeira linha overfull se seu custo sozinho não pode melhorar nem empatar com vantagem o melhor custo já encontrado para aquele fim; só aplica quando a largura da linha não diminui ao estender o início para a esquerda, comprovada conservadoramente por palavras, gaps e tracking não negativos. Medir a segunda com e sem a construção da solução Simple para separar o custo da semente da eficácia da poda local. Em caso de dúvida ou valores não finitos, não podar. O prefixo tem custo não negativo porque a função de custo é quadrática, inclusive com runt negativo. Preservar a fronteira ativa e validar ranges iguais ao seletor exato, incluindo larguras negativas em que a poda de monotonicidade deve desligar. Medir separadamente visitas, podas e tempo com as mesmas entradas de 40, 160, 500, 1.000 e 2.000 palavras, também em largura larga. Resultados só valem para as entradas e o estado medidos.
