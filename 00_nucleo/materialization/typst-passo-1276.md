# P1276 — congelar a próxima fronteira SVG multi-space

**Estado:** EXECUTADO — CONTRATO CANDIDATO CONGELADO; PRODUTO INALTERADO
**Predecessor:** P1275 `PROMOTED-PRESERVED`
**Regime:** protocolo Tekt completo, fase pré-candidato; sem atestação de isolamento

## Objetivo

Preparar a generalização dos oito pares Linear/Radial ainda sem orçamento SVG
selado e que não dependem de perfil ICC:

- Linear/Radial × Oklch;
- Linear/Radial × Hsl;
- Linear/Radial × Hsv;
- Linear/Radial × Luma.

CMYK permanece fora da população por ADR-0097. O passo não altera L0, código
produtivo, testes do owner nem o fallback `gradient-color-space`.

## Medição antes da decisão

O P1275 certificou quatro pares promovidos e preservou fallback nos dez pares
não aprovados. O mapa P1241 registra que os oito pares acima possuem evidência
L1, mas não orçamento SVG selado; os dois pares CMYK continuam
`Unknown-ADR0097`. No owner produtivo, `paint_is_svg_native` admite apenas
sRGB, Oklab e LinearRgb para Linear/Radial, enquanto os oito pares desta
população continuam no marcador explícito.

## População congelada

Cada par recebe as 24 famílias gerais P1266. Os seis pares polares recebem
também os seis cenários P1238 (`seam-forward`, `seam-reverse`, `no-wrap`,
`low-chroma`, `alpha-first`, `alpha-last`) em sete posições. Os dois pares
Luma recebem seis controles próprios em sete posições, incluindo alpha e
extremos de luminância.

O conjunto contém 240 grupos de fixture e 336 observações focais adicionais:
192 grupos gerais, 36 grupos polares e 12 grupos Luma. A cardinalidade não é
prova de paridade; apenas define a população que P1277 deve materializar.

## Contrato candidato

- oráculos, budgets, máscaras e limites de custo são derivados do vanilla
  ratificado antes da primeira execução candidata;
- cada geometria/espaço fecha isoladamente;
- hue é observado pela rota curta e distância angular modular;
- cor premultiplicada e alpha são observados separadamente antes de `u8`;
- stops originais, ordem, coincidência, right-continuity, grafo, papel,
  transformações e domínio permanecem obrigatórios;
- custo é limitado por intervalo, nunca por cap global inventado;
- stroke degenerado preserva a região pintada; fill sem área não pinta área
  positiva;
- `Unknown` nunca conta como sucesso;
- o fallback produtivo permanece obrigatório em P1276/P1277.

## Ataques e limite do selo

O runner P1276 rejeita mutações da definição do contrato e da população. Esse
score prova somente que o pacote congelado não aceita omissões estruturais
óbvias; não substitui mutantes semânticos executados sobre oráculos e candidato.
O selo semântico pertence a P1277.

## Resultado

Artefatos canônicos:

- `00_nucleo/diagnosticos/p1276-pairs.tsv`;
- `00_nucleo/diagnosticos/p1276-population.tsv`;
- `00_nucleo/diagnosticos/p1276-contract.tsv`;
- `00_nucleo/diagnosticos/p1276-unknown-policy.tsv`;
- `00_nucleo/diagnosticos/p1276-attacks.tsv`;
- `00_nucleo/diagnosticos/p1276-role-capabilities.tsv`;
- `00_nucleo/diagnosticos/p1276-input-manifest.tsv`;
- `00_nucleo/diagnosticos/p1276-preseal-receipt.tsv`;
- `00_nucleo/diagnosticos/p1276-summary.json`;
- `00_nucleo/diagnosticos/typst-p1276-preseal.md`.

Próxima rota: P1277 materializa oráculos vanilla-first, ataques semânticos e
adjudicação por par. Qualquer promoção posterior volta ao gate ADR-0127.

**Atestação:** EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
