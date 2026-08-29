# P1235 — corrigir o helper adaptativo e recuperar os controles

**Estado:** EXECUTADO — ROLLBACK SEGURO  
**Predecessor:** P1234  
**Saída:** sRGB e as promoções P1229 passam o contrato retificado, ou rollback.

## Objetivo

Aplicar apenas a menor correção causal indicada por P1234 no owner correto.
Atualizar primeiro o Prompt L0 vigente e ressellar hashes. Preservar cap 64,
stops coincidentes, ordem, alpha, variantes e a whitelist fechada; não promover
nenhuma das nove candidatas P1231.

## RED→GREEN

Congelar RED nos piores pontos e em todos os 30 controles. A implementação não
pode afrouxar budgets, rasterizar, introduzir ICC/dependências ou mudar PDF/L1
por arrasto. Exigir testes P1229/P1230/P1231, build, V5/V15/V26, linter e
mutantes discriminativos.

## Gate

Se os controles não fecharem, restaurar a política segura: reclassificar as
três promoções P1229 como `Unknown` com fallback explícito. O passo termina com
uma dessas duas saídas, nunca com uma whitelist sem evidência.

## Resultado

P1234 foi saneado como `ACCEPTED_DIAGNOSTIC`: nenhuma divergência pública foi
reproduzida e os 30 owners causais permaneceram `Unknown`. Como os 30 controles
P1233 não fecharam e nenhuma causa corrigível foi demonstrada, mantém-se a
saída segura confirmada pelo dono no gate ADR-0127: Linear/Oklab,
Linear/LinearRgb e Radial/Hsv regressaram a `Unknown` com fallback explícito
`gradient-color-space`. Nenhuma candidata P1231 foi promovida.

Somente o L0 proprietário `infra/export/svg` contém a decisão de whitelist. O
L0 `infra/export/gradients/adaptive` descreve apenas o helper que possui e não
duplica a política do consumer SVG. A implementação de rollback está confinada
a `paint_is_svg_native`; cap 64, stops coincidentes, ordem, alpha, variantes,
helper adaptativo, PDF e L1 não são alterados por esta decisão.

GREEN reproduzido: os três testes focais passam após o rollback. O RED
histórico e os três mutantes não foram preservados em recibos reproduzíveis;
portanto não são usados como prova e nenhum mutation score é alegado. O gate
atual V15/V26 não apresenta violações no recorte; V5 ainda reporta duas derivas
preexistentes fora dos owners P1235, que não são atribuídas nem corrigidas por
este passo.

Proveniência: commit base
`697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`; working tree não commitada, com
alterações preexistentes preservadas e snapshot exato registrado no relatório
de saneamento. Execução sem atestação de isolamento: o host compartilha
filesystem e não há alegação de materialização Tekt segregada.

**Recibo saneado:** `00_nucleo/diagnosticos/typst-p1235-rollback-saneamento.md`
e `00_nucleo/diagnosticos/p1235-gates.tsv`.
