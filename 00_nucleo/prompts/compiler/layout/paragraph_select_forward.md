# L0 — paragraph_select_forward
Hash do Código: 37a64a32

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_select_forward.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Medição anterior à decisão

`01_core/src/compiler/layout/paragraph_select.rs:151-180` fixa o fim da linha e percorre seus inícios ativos, guardando custo total e predecessor. As medidas do intervalo são O(1) via prefixos. `01_core/src/compiler/layout/paragraph_select_bounded.rs:91-143` usa a solução Simple como limite superior, mas ainda mede/visita cada par antes de podar. O microbenchmark de 2026-10-01 mostra 1.000 palavras a 240pt mais lentas com essa poda e 1.200pt mais rápidas; isso não prova que inverter os laços será melhor.

## Protótipo experimental

Consumer privado só sob `cfg(test)`, sem API, modo, default, fase ou mudança no caminho produtivo. Recebe `Prepared`, largura, font-size, justificação e custos do seletor de palavras; retorna os mesmos ranges que Optimized no subconjunto preparado. Não representa a hifenização nem todo UAX14.

Calcular as mesmas medidas prefixadas e o mesmo custo-base. Construir duas partições iniciais válidas: (a) a linha mais cheia que ainda respeita o limite de ratio, preservando progresso de palavra indivisível; (b) linha cujo fim válido é mais próximo da largura-alvo estimada pelo material restante dividido pela estimativa de linhas restantes. Pontuar ambas com a função exata e conservar o melhor custo como limite superior; a solução Simple existente pode ser usada como controle. A estimativa de linhas orienta somente o candidato, nunca descarta soluções.

Inverter o percurso da DP: para cada início com custo conhecido, percorrer finais ainda elegíveis e relaxar o custo do prefixo terminado neles. Pré-computar para cada fim o primeiro início ativo exatamente conforme a regra do seletor produtivo; essa fronteira não decresce. Parar o vetor de finais de um início quando ele fica abaixo da fronteira ativa. Pular um início inteiro quando seu custo parcial já excede o melhor limite; custos futuros são quadrados não negativos. Preservar o desempate `<=` para início posterior. Limite não finito ou saída incompleta delega à seleção exata.

Testar diferencialmente vazio, overlong, larguras/gaps, justificação, tracking e runt negativo. Microbenchmark em release mede a seleção completa de ambos os caminhos sobre as mesmas entradas com verificação prévia de ranges. Registrar provenance do estado e não alegar ganho geral a partir de casos sintéticos. Build e crystalline-lint sem violações.

## Ablação do limite inicial — 2026-10-02

Medição anterior à decisão: `00_nucleo/diagnosticos/microbench-paragraph-select-forward-2026-10-02.md` registrou que Simple forneceu o menor limite em todos os quatro cenários, enquanto as duas sementes adicionais consumiram tempo. Materializar variante experimental no mesmo consumer que compila só com o limite Simple, sem construir as sementes packed/balanced nem seu vetor de larguras acumuladas. Comparar na mesma rodada Criterion baseline, variante completa e variante Simple-only, exigindo ranges idênticos antes de medir. O resultado separa custo das sementes do custo de inverter a DP; não promover a produção a partir de matriz sintética.

## Ablação da medição do limite Simple — 2026-10-02

Medição anterior à decisão: `01_core/src/compiler/layout/paragraph_select.rs:110-122` recompõe a medida de todo o intervalo para cada palavra candidata da linha Simple; `paragraph_select_forward.rs:49-101` já calcula medidas prefixadas para a DP. A variante Simple-only ainda perde para a seleção exata no cenário de 1.000 palavras a 240pt. Comparar uma terceira variante experimental que constrói a mesma partição gulosa com a medida prefixada O(1) já disponível, mantendo a regra de avanço e de palavra indivisível. A medida prefixada pode ter arredondamento distinto; exigir igualdade das quebras finais com a seleção exata na matriz diferencial e nos cenários cronometrados. Medir as três variantes forward e o baseline na mesma rodada, sem alterar o caminho produtivo.

## Escolha experimental do caminho — 2026-10-02

Medição anterior à decisão: `00_nucleo/diagnosticos/microbench-paragraph-select-forward-ablation-2026-10-02.md` mostra que a variante Simple prefixada vence o seletor exato com 160 palavras/240pt e 1.000 palavras/1.200pt justificadas, mas perde com 40 palavras/76pt e 1.000 palavras/240pt. Não há vencedor universal nessa matriz. Acrescentar ao consumer `cfg(test)` uma escolha determinística de baixo custo entre o seletor exato e a variante forward prefixada, baseada apenas em tamanho do parágrafo, largura relativa ao avanço médio e justificação; não executar ambos para decidir. Expor o caminho escolhido ao teste. Ajustar a regra com cenários adicionais, incluindo largura larga sem justificação e parágrafo médio estreito. O teste diferencial compara o resultado escolhido com o exato. O microbenchmark mede a seleção adaptativa inteira, inclusive o custo de escolha, contra os caminhos isolados sobre as mesmas entradas. Não alterar default de produção nem alegar ganho geral sem medir corpus real.

Primeira medição da escolha (`2026-10-02 18:50:57 UTC`, working tree sobre `39a74d477`, dois arquivos alterados): com 160 palavras/76pt, selecionar forward pela contagem isolada custou 10,744 µs contra 8,338 µs do exato. A largura relativa ao avanço médio deve participar também nesse intervalo; repetir a rodada após ajustar a regra. Essa calibração sintética não constitui limiar de produção.

## Reparação local da semente — 2026-10-02

Medição anterior à decisão: `00_nucleo/diagnosticos/microbench-paragraph-select-local-seed-2026-10-02.md` registra que, em 500 palavras/240pt, o forward prefixado avaliou 2.932 candidatos em 43,289 µs, enquanto a variante com reparação local de uma passagem avaliou 2.305 em 39,529 µs. Em 1.000 palavras/240pt, foram 10.300 candidatos/134,19 µs contra 7.895/114,82 µs. No cenário largo, a reparação não reduziu candidatos nem tempo. A busca de todos os cortes por par e a segunda passagem foram medidas e rejeitadas no diagnóstico.

Partir da partição Simple prefixada. Percorrer pares de linhas vizinhas uma vez, da esquerda para a direita. Para cada par, testar apenas mover o corte uma palavra para a esquerda ou para a direita, conservando ambas as linhas não vazias. Usar a soma dos custos exatos das duas linhas e aceitar somente melhoria estrita e finita. A partição reparada fornece apenas um limite superior; nenhum candidato da DP é excluído por esta heurística. Preservar medida prefixada, fronteira ativa e desempate da busca. Testar diferencialmente as quebras contra o seletor exato.

A escolha experimental `cfg(test)` seleciona exato para menos de 64 palavras; entre 64 e 256 seleciona forward prefixado quando cabem pelo menos sete avanços médios por linha; acima disso seleciona forward prefixado quando cabem pelo menos 24 avanços médios com justificação ou 32 sem ela. Nos demais casos de 257 a 1.200 palavras com pelo menos sete avanços médios, seleciona forward com reparação local; fora dessas faixas, seleciona exato. O avanço médio soma largura de palavra e gaps anteriores. Esses limiares não alteram o default de produção e exigem validação em corpus real antes de qualquer promoção.
