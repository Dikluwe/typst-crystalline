# P1392 — preflight de materialização

Regime: protocolo completo sem atestação de isolamento. Uma autoridade possui
contrato, código, ataques e veredito; a separação é causal e por artefatos,
não independência atestada.

Entrada congelada: quatro positivos bilaterais, um opaco com delta composto e
controles P1391/P1390/P1389/P1388. A hipótese é que a reserva linha-a-linha e
o arm overlong já são suficientes; falta certificar a célula cruzada e fazer o
fechamento singleton usar o tamanho textual real para detectar mistura abaixo
do fallback exterior. A matriz bilateral refina a hipótese: a linha menor não
é mista para baseline/ascent; apenas o leading vem do exterior. Na ordem
maior→menor, a baseline já exterior é antecipada pelo delta de top-edge, que a
reserva própria da linha menor já representa.

Ordem: passo → L0 → contrato/oráculos/mutantes → RED focal → implementação
→ GREEN focal → controles → matriz bilateral → resselo/lint. Execução serial
e limitada; duas tentativas sem ganho reabrem o desenho.
