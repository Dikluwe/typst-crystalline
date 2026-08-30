# P1265 — decidir a promoção produtiva dos quatro pares SVG certificados

**Estado:** EXECUTADO — OPÇÃO B CONFIRMADA, FALLBACK MANTIDO  
**Predecessor:** P1264  
**Regime:** preparação de decisão Tekt; nenhuma alteração produtiva autorizada

## Objetivo

Decidir, sem extrapolação silenciosa, se os quatro pares certificados pelo
P1264 devem entrar no caminho SVG nativo por defeito:

- Linear/Oklab;
- Radial/Oklab;
- Linear/LinearRgb;
- Radial/LinearRgb.

## Medição antes da decisão

Estado medido em `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`, working tree não
commitado, em 2026-08-28T19:43:24-03:00. A lista exata de alterações que
formou os números está congelada em
`00_nucleo/diagnosticos/p1264-working-tree-snapshot.txt`.

O P1264 reproduziu 24/24 fixtures preservadas no grafo e numericamente, com
6/6 para cada par, 23/23 mutantes focais e 7/7 mutantes de integração
rejeitados. Também declarou expressamente:

- classificação `Preserved-fragment`;
- nenhuma equivalência SVG geral;
- nenhuma promoção produtiva;
- `paint_is_svg_native` inalterado;
- mudança do fallback por defeito subordinada ao ADR-0127.

O owner produtivo vigente ainda aceita somente sRGB para Linear/Radial. A
promoção proposta removeria o fallback diagnóstico `gradient-color-space` e
passaria a emitir servidores nativos para toda entrada desses quatro pares,
um universo maior que as 24 fixtures certificadas.

## Decisão necessária

Escolher exatamente uma opção:

### A — promover agora os quatro pares

Aceitar o fragmento P1264 como evidência suficiente para o comportamento por
defeito. Depois da confirmação, atualizar primeiro o L0
`00_nucleo/prompts/infra/export/svg.md`, ressellar a linhagem e só então:

1. ampliar `paint_is_svg_native` apenas para Linear/Radial ×
   Oklab/LinearRgb;
2. substituir os testes de fallback desses quatro pares por testes nativos;
3. preservar Hsv, Oklch, Hsl, Luma, CMYK e combinações não seladas como
   `Unknown` com fallback explícito;
4. reexecutar P1234, P1236, P1237, P1264, workspace e lint Tekt;
5. publicar certificado limitado, sem alegar equivalência SVG geral.

### B — ampliar o envelope antes da promoção — recomendada

Manter o fallback produtivo e executar um corpus de generalização que cubra
as dimensões públicas ainda não legitimadas para extrapolação: cardinalidade
e distribuição arbitrária de stops, offsets fora de ordem/limite quando
admitidos pelo domínio, alpha e gamut extremos, `anti_alias`, ângulo/foco/raio,
fill/stroke, transformações e caixas degeneradas. A promoção volta ao gate
ADR-0127 somente depois de cada par passar independentemente.

### C — conservar o fallback

Registrar que `Preserved-fragment` serve apenas como diagnóstico e manter a
política produtiva atual sem novo corpus.

## Trava

Este passo não autoriza edição de L0 nem código produtivo enquanto o dono não
escolher A, B ou C. Uma resposta genérica para “continuar” não é interpretada
como aprovação de mudança do comportamento por defeito.

## Fechamento esperado

- opção e data da confirmação registradas;
- se A: L0 confirmado antes do código, testes RED→GREEN e certificado;
- se B: passo sucessor de generalização com contrato, ataques e política
  `Unknown` segregados;
- se C: certificado de não promoção e fila encerrada sem alteração produtiva.

## Decisão do dono

Em `2026-08-28T19:52:26-03:00`, o dono confirmou explicitamente a opção
**B — ampliar o envelope antes da promoção**.

Consequências materializadas:

- `paint_is_svg_native`, o L0 SVG e o fallback `gradient-color-space`
  permanecem inalterados;
- nenhuma promoção produtiva foi aplicada;
- o sucessor `typst-passo-1266.md` congela um corpus de generalização de 96
  fixtures válidas, probes inválidos, contrato, ataques e política `Unknown`;
- qualquer promoção futura volta obrigatoriamente a um novo gate ADR-0127.

Decisão e certificado:
`00_nucleo/diagnosticos/typst-p1265-owner-decision.md`.

