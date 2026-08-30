# Passo 1280 — promover os seis pares polares Linear/Radial no SVG

## Estado do gate

`GATE ADR-0127 CONFIRMADO PELO DONO EM 2026-08-29`.

Este passo altera o comportamento por defeito do produto: seis combinações
que hoje emitem fallback sólido passarão a emitir servidor SVG adaptativo.
Por isso esta especificação e o Prompt L0 foram escritos antes do código. O
dono confirmou explicitamente a materialização em 2026-08-29.

## Objetivo

Promover exclusivamente Linear/Radial × Oklch/Hsl/Hsv, já classificados como
`Generalization-Preserved` pelo P1279, para a rota produtiva existente de
stops adaptativos do exportador SVG.

## Baseline e evidência congelados

- commit do certificado P1279: `ad93af9e80213fc410ffeda1f4916b031b088474`;
- baseline vanilla ratificado: upstream/main `a51e02804`;
- certificado P1279 SHA-256:
  `be1e16cfb002c70b8d7af83421e7027f2abdb32324a1e8ff94ace7152ee1e3ed`;
- resumo final P1279 SHA-256:
  `c0628bc0e2723e664c587a6b5077c8c9a59bea70d2ffe93909a44efdb5b5c80a`;
- manifesto de evidência P1279 SHA-256:
  `8f44e3ac8da2b9f75ee075f0656a7917f8039c39734e1cef12951ca21916c1a3`;
- pares finais P1279 SHA-256:
  `3dbd56eee4a5ac00bce53f7840f7d6bb2c7ad5858b39e7424e7d0c6486fc268a`.

P1279 fechou os seis pares em 144/144 fixtures gerais, 42/42 inválidos
polares rejeitados, custo/grafo/raster preservados e determinismo integral,
sem mudar budgets nem código produtivo.

## Mudança proposta

1. Em `paint_is_svg_native`, admitir Oklch, Hsl e Hsv tanto para Linear como
   para Radial.
2. Reusar sem alteração o servidor Linear/Radial e `svg_adaptive_stops` já
   existentes; não criar algoritmo, modo, flag ou API.
3. Preservar sRGB, Oklab e LinearRgb exatamente como estão.
4. Manter Luma e CMYK em fallback explícito `gradient-color-space`.
5. Não promover por arrasto Conic, Tiling ou qualquer espaço não certificado.

## Contrato RED→GREEN pós-confirmação

Antes de mudar o predicate produtivo, os testes devem exigir para os seis
pares:

- referência local resolvida e variante `<linearGradient>`/`<radialGradient>`
  correta;
- ausência de `data-crystalline-*-fallback="gradient-color-space"`;
- stops adaptativos, offsets originais, ordem, descontinuidades e alpha;
- cobertura separada de fill e stroke;
- Luma e CMYK ainda com fallback e sem servidor indevido.

Os testes devem falhar com o predicate atual, e só então o `match` pode ser
ampliado. O contrato pós-promoção exige 144/144 polares nativos, 48/48 Luma
mantidos como controle negativo, 56/56 inválidos rejeitados e 384/384 recibos
determinísticos na população completa.

## Ataques obrigatórios

- promover apenas Linear ou apenas Radial;
- omitir individualmente Oklch, Hsl ou Hsv;
- promover Luma ou CMYK por wildcard;
- remover o fallback sem produzir referência SVG resolvida;
- contornar `svg_adaptive_stops` ou reduzir o paint a uma cor sólida;
- aceitar somente cardinalidade, bytes ou IDs em vez dos observáveis de
  linguagem;
- alterar budget, limiar `0.001`, cap 64 ou política de `Unknown`.

Somente mutações semanticamente válidas executadas entram no score, que deve
ser `1.0`. A alteração da fronteira produtiva invalida o ataque P1279 que
tratava qualquer promoção como indevida; P1280 sela uma fronteira nova antes
da implementação.

## Validação final prevista

- testes RED→GREEN do owner SVG;
- matriz P1279 completa readjudicada com a nova fronteira produtiva;
- ataques P1280 e controles sRGB/Oklab/LinearRgb/Luma/CMYK;
- `cargo build --workspace` e `cargo fmt --all -- --check`;
- `crystalline-lint --checks v1,v5,v15,v26 .` e lint integral;
- manifests, hashes, comandos e estado exato da árvore registrados.

## Atestação

O L0 e o contrato são preparados no checkout compartilhado. Sem ambientes ou
autoridades de leitura fisicamente segregados, qualquer execução posterior
deve declarar `EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`.
