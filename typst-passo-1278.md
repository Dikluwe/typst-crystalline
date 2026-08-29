# Passo 1278 — refinar a aproximação SVG dos espaços polares

## Objetivo

Refinar, sem promoção produtiva, o adaptador SVG diagnóstico dos seis pares
Linear/Radial × Oklch/Hsl/Hsv que P1277 deixou como
`Unknown-generalization`. Luma permanece controle negativo separado por causa
da divergência conhecida de alpha; CMYK continua fora do escopo por ADR-0097.

## Classe ADR-0127

Correção interna de fórmula e paridade: não altera contrato público,
comportamento por defeito, fase do pipeline, compatibilidade nem a fronteira
`paint_is_svg_native`. O fluxo é contínuo: L0 primeiro, teste RED→GREEN,
resselo e revalidação.

## Implementação autorizada pelo L0

1. Amostrar Oklch/Hsl/Hsv com posição e offsets em `f64`, convertendo apenas
   os pesos finais para `f32`, como o vanilla ratificado.
2. Preservar o caminho curto de hue nos três espaços polares.
3. Medir o erro Oklch no sRGB recortado e premultiplicado.
4. Serializar todos os offsets adaptativos polares por `Ratio::repr`.
5. Não incluir Luma/CMYK e não promover nenhum par no produto.

## Aceitação

- testemunhos P1278 falham antes e passam depois da implementação;
- matriz P1277 completa reexecutada sem budgets novos;
- nenhuma regressão nos seis pares polares e melhora material reproduzível;
- focal público, domínio inválido, custo, raster, grafo e determinismo
  preservados;
- qualquer fixture geral remanescente continua `Unknown`, sem arredondamento
  de veredito;
- fallback produtivo preservado e zero promoções.

## Atestação

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Os papéis têm artefatos e ordens
separados, porém compartilham o mesmo checkout e capacidade de leitura.
