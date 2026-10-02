# L0 — paragraph_select_forward
Hash do Código: 0f1346e7

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
