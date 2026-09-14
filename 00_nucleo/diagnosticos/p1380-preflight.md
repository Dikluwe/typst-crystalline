# P1380 — preflight de materialização

Regime: protocolo completo **sem atestação de isolamento**. A mesma autoridade
possui contrato, testes, candidato, ataques e veredito; hashes comprovam ordem
e identidade, não independência.

A medição bilateral precedeu a decisão e separou três formas positivas. A
fonte vanila coleta o prefixo inline como parágrafo antes de coletar o Block;
o cristalino P1379 mede nós individualmente e reconhece a fronteira apenas
quando o predecessor imediato é `Text` direto.

Hipótese congelada: varrer a fatia anterior ao primeiro `fr` sem recursão,
aceitar somente fragmentos textuais de uma linha e acumular largura, primeiro
top-edge e uma única altura de fechamento. Wrappers ou estilos que poderiam
alterar a geometria de modo não coberto ficam `Unknown`; nenhuma simulação de
layout, checkpoint, Vec ou árvore auxiliar é autorizada.

Ordem: L0/contrato/oráculos/mutantes → RED focal → candidato → GREEN focal →
matriz bilateral → resselo/lint. Budget: duas revisões focais; duas falhas da
mesma causa reabrem o desenho antes de nova candidata. Execução sempre serial
com os limites registrados no passo.

Calibração v2: o primeiro teste Strong inferiu uma diferença de baseline a
partir de `yMin` do PDF. O RED/GREEN focal mostrou `One=38,6` e `Two=17,7`
internamente, a mesma diferença `20,9pt` dos demais casos; os `0,143pt` do
oráculo externo pertencem à caixa de tinta do glifo bold (`Lead=7,261`) e não
ao cursor da linha seguinte. O contrato e o oráculo PDF não mudam. O teste
interno passa a verificar `20,9pt`; o pré-selo v1 é invalidado antes da nova
execução focal.
