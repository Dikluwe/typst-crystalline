# P1274 — promover somente os pares aprovados pelo dono

**Estado:** EXECUTADO — RECIBO DE IMPLEMENTAÇÃO PUBLICADO
**Predecessor:** gate ADR-0127 P1273 confirmado
**Regime:** implementação produtiva L0-first já aprovada

## Objetivo

Aplicar a promoção exatamente autorizada, sem expandir por similaridade para
outros espaços ou geometrias.

## Execução

1. Confirmar que o L0 aprovado e todos os artefatos P1272 mantêm os hashes do
   gate.
2. Escrever primeiro testes públicos RED para o caminho produtivo de cada par:
   servidor nativo resolvido, marcador de fallback ausente e observáveis do
   envelope preservados.
3. Alterar somente o predicado produtivo proprietário da promoção e os testes
   de fallback diretamente afetados.
4. Manter todos os pares fora da aprovação com o marcador explícito vigente.
5. Atualizar `@prompt-hash`/`Hash do Código` pelo fluxo Tekt válido.
6. Executar envelope, controles, workspace e lint; nenhuma falha pode ser
   convertida em mudança de contrato nesta fase.

## Limite

Este passo implementa a decisão; não emite o veredito independente final.

## Resultado de 2026-08-29

- baseline commitado: `b989c95a5ceaf7300904594b690f70f9d986f4e0`;
- L0 P1273 e hashes P1272 confirmados antes da alteração;
- RED focal: 4 falhas positivas esperadas e 1 gate negativo preservado;
- GREEN focal: 5/5;
- mutantes focais: 3/3 rejeitados;
- envelope: 96/96 fixtures, 384/384 métricas, 1.224/1.224 intervalos,
  28/28 inválidos rejeitados, 192/192 determinísticos e 24/24 ataques P1268;
- fronteira produtiva: 96/96 nativos e zero fallback nos quatro pares;
- workspace, build, formato e lint: PASS.

Recibo principal:
`00_nucleo/diagnosticos/typst-p1274-promocao-materializada.md`. Atestação de
autoria única: sem isolamento e sem veredito independente final.
