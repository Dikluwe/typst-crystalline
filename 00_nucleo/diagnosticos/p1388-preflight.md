# P1388 — preflight de materialização

Regime: protocolo completo sem atestação de isolamento. A mesma autoridade
controla contrato, implementação, ataques e veredito; hashes provam identidade
e ordem, não independência.

A entrada congelada contém um positivo (20→20) e um opaco com wrap demonstrado
pelo vanila (21→21). Quatro candidatas horizontais foram refutadas sem ganho
produtivo. A redução `leading-only`/`trailing-only` mostrou que whitespace
terminal pode fechar a linha antes do Block. A inspeção do consumidor revelou
que a boundary já certificada era condicionada a `current_line` ainda estar
aberta e, portanto, era perdida nesse caminho.

Ordem: L0/contrato/oráculos/mutantes → RED focal → candidata → GREEN focal →
controles históricos → matriz bilateral → resselo/lint. A boundary só existe
com distribuição certificada e é aplicada uma vez; não há epsilon, recursão,
coleção proporcional, API pública ou bypass de wrap. Processos seriais nos
limites do passo.
