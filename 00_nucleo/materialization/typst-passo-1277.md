# Passo 1277 — materializar a fronteira SVG multi-space restante

## Objetivo

Executar o contrato congelado pelo P1276 para os oito pares
Linear/Radial × Oklch/Hsl/Hsv/Luma, sem promover nenhum deles no caminho
produtivo. CMYK permanece fora do escopo como `Unknown-ADR0097`.

## Classe ADR-0127

Este passo é diagnóstico e de paridade. Não altera contrato público,
comportamento por defeito, fase do pipeline nem compatibilidade. O L0 e o owner
produtivo SVG permanecem inalterados porque qualquer promoção está
explicitamente fora do escopo.

## Execução

1. Validar as 16 obrigações, seis políticas de Unknown e 18 ataques congelados
   pelo P1276.
2. Materializar, antes do candidato, os oráculos vanilla, budgets, máscaras e
   custos para 24 famílias gerais por par (192 grupos).
3. Medir o adaptador diagnóstico que usa o algoritmo real de
   `03_infra/src/export/gradients/adaptive.rs`, mantendo o fallback do produto.
4. Executar os 48 grupos focais congelados, sete posições por grupo (336
   observações), incluindo rota modular de hue e controles Luma/alpha.
5. Executar ataques semânticos, domínio inválido, ordem inversa e repetição.
6. Adjudicar cada par pela conjunção dos gates. Um subgate preservado não
   promove outro par nem compensa uma violação.

## Aceitação

- oráculo materializado antes do candidato;
- 192/192 grupos gerais e 336/336 observações focais executados;
- custos limitados por intervalo original, não por cap global;
- entradas inválidas rejeitadas;
- determinismo reproduzido em ordem inversa e repetição;
- 100% dos mutantes executados e rejeitados;
- fallback produtivo preservado nos oito pares;
- veredito restrito à população executada, sem alegação de equivalência SVG
  geral.

## Atestação

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Há segregação por artefatos entre
oráculo, candidato, ataques e adjudicação, mas não há isolamento forte de
leitura ou de autoria entre processos.
