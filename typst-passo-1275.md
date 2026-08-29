# P1275 — verificar e certificar a promoção produtiva

**Estado:** PLANEADO E CONDICIONAL
**Predecessor:** implementação P1274
**Regime:** veredito segregado e reproduzível

## Objetivo

Provar que a promoção aprovada está ativa somente onde autorizada e que o
fragmento certificado não regrediu.

## Verificação

- conferir hashes do L0 aprovado, contrato selado, oráculos, implementação e
  baseline;
- executar as 96 fixtures pelo binário produtivo, não apenas pelo probe de
  laboratório;
- exigir servidor Linear/Radial nativo e ausência do fallback somente nos
  pares aprovados;
- exigir fallback explícito em todos os pares não aprovados;
- reexecutar budgets, grafo, raster, domínio, custo, determinismo, mutantes e
  controlos predecessores;
- executar build, testes, formatos, `git diff --check`, V1/V5/V15/V26 e lint
  completo;
- comprovar que entradas seladas não mudaram após o preseal.

## Certificado final

Publicar veredito por par, comandos, versões, hashes, working-tree snapshot e
limitação de escopo. Só declarar `PROMOTED-PRESERVED` quando todos os gates do
par passarem. Se a autoridade não estiver isolada, usar a linguagem
`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO` e nunca alegar segregação.
