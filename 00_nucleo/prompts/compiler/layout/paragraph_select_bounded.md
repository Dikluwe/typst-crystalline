# L0 — paragraph_select_bounded
Hash do Código: 032eb474

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_select_bounded.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Medição anterior à decisão

`01_core/src/compiler/layout/paragraph_select.rs:101-176` usa DP exata sobre todos os finais de palavra, prefixos para medir intervalos em O(1), descarte seguro de predecessores overfull e desempate pelo predecessor posterior. `01_core/src/compiler/layout/paragraph_plan.rs` chama somente essa seleção. A fonte vanilla pinada em `lab/typst-original/crates/typst-layout/src/inline/linebreak.rs:227-381` combina limite aproximado e busca limitada. Esta é uma observação mecânica de código, sem medição de tempo ou declaração de equivalência com o vanilla.

## Decisão experimental

Protótipo privado compilado apenas em testes. Recebe a mesma entrada preparada, largura, tamanho, justificação e custos que o seletor de palavra exato. Não acrescenta API, modo, flag, default, fase de pipeline ou troca de produção. Não cobre candidatos de hifenização, UAX14 completo ou largura variável.

Calcular um limite superior válido somando os custos exatos das linhas da solução Simple existente, mas medir essas linhas pela mesma fórmula de prefixos do caminho Optimized. Como cada custo de linha é um quadrado e portanto não negativo, um prefixo acumulado estritamente maior que o limite não pode participar de solução melhor ou empatada; só esses prefixos podem ser podados. Preservar o descarte overfull e o desempate `<=` da DP exata. Se o limite ou resultado não for finito, delegar à seleção exata. Não aplicar margem aproximada nem mudar a função de custo.

Testes diferenciais devem comparar a sequência completa de ranges com a seleção exata em entradas variadas: vazio, overlong, justificado/não justificado, larguras/gaps, tracking e custo runt negativo. Expor contagens de candidatos avaliados e podados apenas ao teste; isso não prova ganho temporal. Build e crystalline-lint sem violações.
