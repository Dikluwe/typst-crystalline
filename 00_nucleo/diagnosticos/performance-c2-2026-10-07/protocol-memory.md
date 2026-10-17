# Memória — população separada

A primeira população wall usa wait4 após fork/preexec Python. Seu ru_maxrss inclui o RSS do launcher antes do exec e não mede exclusivamente o Typst: números originais serão preservados, mas excluídos de conclusões sobre memória.

Esta população adicional, preparada antes de executar, mede RSS por GNU time após o helper já executar. /usr/bin/time cria taskset/Typst a partir de seu próprio processo pequeno; %M representa o máximo do processo compilador, incluindo seu lifecycle CLI. O pico inclui taskset antes do exec (pequeno), sem herdar o footprint Python. Wall/CPU desse wrapper não serão misturados à população de tempos principal.

Mesmos três executáveis/fontes/fontconfig/opções, CPU4, sete fixtures; três rodadas, seis trios por fixture com todas as permutações de ordem, dois warmups por engine/bloco. Quiet gate30s, monitor, fingerprints e hashes antes/depois. Mediana/p05/p95 em MiB por engine, sem inferir alocações exclusivas ou bytes retidos. Cache de filesystem aquecido, processos novos; não representa memória de watch ou recompilação no mesmo processo.
