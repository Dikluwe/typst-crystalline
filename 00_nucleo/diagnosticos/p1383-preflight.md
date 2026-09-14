# P1383 — preflight de materialização

Regime: protocolo completo **sem atestação de isolamento**. A mesma autoridade
possui contrato, teste, candidato, ataques e veredito; hashes comprovam ordem
e identidade, não independência.

A medição bilateral precedeu a decisão. P1382 eliminou a única causa pela qual
a candidata v2 do P1381 falhava no caso crescente: baseline composta 4,606pt
alta. O corpo e a cauda dessa candidata já coincidiam. P1383 não cria fórmula;
reabre somente a forma estrutural exata agora decidível.

Ordem: L0/contrato/oráculos/mutantes → RED focal → candidato → GREEN focal →
matriz bilateral → resselo/lint. Budget: duas revisões focais; duas falhas da
mesma causa reabrem o desenho. Processos sempre seriais nos limites do passo.

Não são autorizados recursão, Vec auxiliar, reconstrução de Content, mudança
de contrato público, ordem inversa, terceiro fragmento ou delta além de size.

Revisão v2: a candidata estrutural deixou `After` 4,55pt antes de `Tail`.
Preflight e runtime reservaram juntos `par.spacing`, mas dividiram a soma com
leadings diferentes. P1382 exige o estilo ativo exterior; v2 usa esse estilo
para leading e boundary somente no caso P1383.
