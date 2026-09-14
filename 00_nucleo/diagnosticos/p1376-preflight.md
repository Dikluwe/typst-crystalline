# P1376 — preflight de materialização

Protocolo completo executado **sem atestação de isolamento**: uma autoridade
compartilha contrato, candidato, ataques e veredito. Hashes provam identidade,
não independência.

Baseline P1375: colunas contínuas ignoram fr; forma no topo e page-columns
ficam comprimidas, e a forma após `Intro` prova que a altura total seria uma
base incorreta. O marcador `page_columns: Some` já existe e distingue fluxo
contínuo de segmentos explícitos.

Ordem: L0 e contrato → oráculos/mutantes → preseal → RED focal → fórmula e
gate interno → GREEN focal → matrizes bilaterais → resselo/lint. Budget: uma
revisão de fórmula e nenhuma execução ampla; duas falhas da mesma causa
reabrem o desenho.
