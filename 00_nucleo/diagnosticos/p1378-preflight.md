# P1378 — preflight de materialização

Protocolo completo executado **sem atestação de isolamento**: uma autoridade
compartilha contrato, testes, candidato, ataques e veredito. Hashes provam
identidade, não independência.

A fonte vanila realiza os segmentos como regiões irmãs com a mesma origem.
O cristalino pré-soma ascender apesar de manter `initial_baseline_pending`, e
não restaura esse estado entre irmãos. A coluna desigual prova separadamente
que o cursor máximo não basta: a baseline usada pela margem de saída também
deve vir do segmento mais profundo.

Ordem congelada: L0/contrato/oráculos/mutantes → RED focal → snapshots
escalares → GREEN focal → matriz bilateral → resselo/lint. Budget: uma revisão
do estado de entrada e uma da saída; duas falhas da mesma causa reabrem o
desenho.

Calibração v2: o primeiro teste de segmento vazio comparava duas origens
igualmente deslocadas e não discriminava M4. Antes de qualquer implementação,
o controle passou a ser texto direto no topo da página; o pré-selo v1 foi
invalidado.
