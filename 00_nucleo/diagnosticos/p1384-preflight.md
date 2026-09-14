# P1384 — preflight de materialização

Regime: protocolo completo **sem atestação de isolamento**. A mesma autoridade
possui contrato, testes, candidato, ataques e veredito; hashes comprovam ordem
e identidade, não independência.

A medição bilateral precedeu a decisão. P1382 já iguala a baseline e a tinta
da ordem decrescente. A diferença restante está no fail-closed do preflight,
logo P1384 concede somente a forma estrutural simétrica agora observável.

Ordem: L0/contrato/oráculos/mutantes → RED focal → candidato → GREEN focal →
matriz bilateral → resselo/lint. Budget: duas revisões focais; duas falhas da
mesma causa reabrem o desenho. Processos sempre seriais nos limites do passo.

Não são autorizados recursão, `Vec` auxiliar, reconstrução de `Content`, API
pública, fragmento único maior, dois wrappers, terceiro fragmento ou delta
além de size.

A cadeia inicial foi invalidada porque a fixture histórica supostamente
exterior era 18pt→8pt. A candidata e os testes foram retirados; a medição
bilateral foi repetida com 18pt→11pt antes desta revisão normativa.
