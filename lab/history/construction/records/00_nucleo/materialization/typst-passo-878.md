# Prompt — typst-passo-878: confirmar se o filtro de fallback do P875 está de fato evitando os `.ttc` CJK no cenário de matemática, antes de investigar o algoritmo de layout

**Origem**: P877 concluiu, via `--timings-json` (`layout_ms = 5709ms` de um total de `5756ms`), que a causa do tempo em matemática é o algoritmo de layout, não fonte/fallback — descartando coverage e subsetting como irrelevantes. Isso pode estar em contradição com P873, que já tinha localizado 79% do tempo (5.04s de 6.40s) em `system time`, majoritariamente em `read()` de arquivos `.ttc` CJK grandes, dentro do caminho de fallback do shaper — que roda **por dentro** da chamada de layout. Um bucket chamado `layout_ms` numa instrumentação nova pode estar simplesmente incluindo esse custo de I/O sem separá-lo, não refutando a causa de P873.
**Estado**: aguardando execução — **isto é a medição que falta antes de decidir se P878c (investigar algoritmo de layout matemático) é o caminho certo, ou se a causa continua sendo a mesma do P873 e o filtro do P875 não está funcionando como deveria.**

---

## O teste direto, sem ambiguidade de rótulo de instrumentação

Não confiar no bucket `layout_ms` da instrumentação nova sem saber o que ele inclui por dentro. Em vez disso, repetir exatamente a metodologia que já funcionou em P873, no estado de código atual (pós P874-877):

## Passo 1 — Repetir a medição de P873 tal qual, no código atual

1. `strace -f -c` no cenário `04-math.typ` (mesmo arquivo do benchmark, `/tmp/p872-bench/`) com o binário cristalino atual — confirmar se `read()` ainda domina o `system time`, na mesma proporção medida por P873 (92.88% do tempo de syscall), ou se caiu.
2. `strace -c` filtrado por arquivo, contando quantas vezes cada `.ttc` CJK grande (`NotoSansCJK-Regular.ttc`, etc.) é aberto/lido — comparar contra a contagem de P873 (21 `openat` em math vs 11 de baseline, delta de 10 batendo com o número de faces). Se o filtro do P875 estiver funcionando, essa contagem deve ter caído para perto do baseline (11), não continuar em 21.
3. `/usr/bin/time -v` — comparar `system time` e `Maximum RSS` contra os números de P873 (5.04s de system time, ~9.4GB de RSS). Se caíram bastante, o filtro funcionou e a causa real é outra. Se continuam parecidos, o filtro não está evitando as leituras que deveria.

## Passo 2 — Se o filtro não estiver funcionando, descobrir por quê

Se o Passo 1 mostrar que os `.ttc` CJK continuam sendo lidos por inteiro no caminho de matemática, apesar do filtro de `candidates_for_char` (P875) existir:

1. Confirmar se os símbolos matemáticos/gregos usados no documento de teste têm um bloco Unicode que, pela aproximação de 256 codepoints por bit do `Coverage` (decisão registrada em P875), acaba caindo no mesmo bloco que outros codepoints cobertos por fontes CJK — o que faria o filtro achar que a fonte CJK é candidata, mesmo sem ela realmente ter esses glifos específicos. Essa é a limitação mais provável da aproximação por bloco: um falso positivo de cobertura por causa da granularidade grosseira, não ausência de filtro.
2. Se for isso, o filtro está tecnicamente funcionando (reduzindo candidatos), mas não o suficiente para os blocos específicos usados em matemática — a correção seria refinar a granularidade do bitmap (ou usar a checagem real de `glyph_index` mais cedo, antes de abrir o arquivo inteiro, se isso for possível de forma barata) nesses blocos.
3. Se não for isso (o filtro simplesmente não está sendo chamado no caminho que P873 tinha identificado, por algum motivo de integração), localizar onde a chamada deveria estar e não está.

## Passo 3 — Decidir o próximo passo com base no resultado

1. **Se o filtro não estava funcionando e o Passo 2 achou a causa**: o próximo passo é corrigir o filtro (refinar granularidade de bloco, ou consertar a integração) — não investigar o algoritmo de layout matemático, que provavelmente não é o problema.
2. **Se o filtro estava de fato funcionando** (leituras de `.ttc` caíram para perto do baseline) **e mesmo assim o tempo continua alto**: aí sim a causa é outra, e o P878c (investigar o algoritmo de layout matemático) faz sentido como próximo passo — mas com a instrumentação `--timings-json` decomposta mais fundo (não só `layout_ms` como bloco único, mas o que dentro dele consome o tempo: shaping, resolução de estilo, posicionamento de attach, etc.), para não repetir o mesmo problema de rótulo genérico escondendo a causa real.

## Relatório

`00_nucleo/diagnosticos/typst-passo-878-relatorio.md` com: a repetição da medição de P873 no estado atual (Passo 1), confirmando ou refutando se o filtro de P875 está de fato reduzindo as leituras de `.ttc` CJK; se não estiver, a causa exata (Passo 2); e a recomendação clara de qual passo vem a seguir — corrigir o filtro, ou investigar o layout matemático com instrumentação mais fina. Nenhuma correção de código neste passo.
