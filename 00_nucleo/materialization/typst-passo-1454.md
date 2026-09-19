# Passo 1454 — preservar paint nomeado em `html.frame`

**Estado:** EXECUÇÃO AUTORIZADA — correção interna de paridade
**Data da medição:** 2026-09-18T21:30:54-03:00
**Commit medido:** `3267d9083afef6a5b2a493221e08714e402ac7b8`
**Árvore medida:** `797e2fc07e9a00c5f9d963683ecbaab194a463bf`, limpa
**Regime:** `executado sem atestação de isolamento`

## 1. Causa focal

P1453 isolou um witness em que `fill: red` chegava ao HTML cristalino como
`#ff0000`, contra `#ff4136` no vanilla. A auditoria P1454 mediu primeiro as 18
cores públicas por `to-hex()`: todas coincidem bilateralmente, inclusive
`red == #ff4136`.

A divergência nasce depois do layout. O único caminho produtivo que traduz
esse valor é `export_svg_html_frame_with_fonts`, que serializa a página pelos
writers normais e, em seguida, executa `.replace("#ff4136", "#ff0000")` em
toda a string. O SVG standalone não contém essa pós-transformação.

## 2. Obrigação

O caminho SVG embutido deve preservar paint como recebido:

- `#ff4136` permanece `#ff4136` em fill e stroke;
- `#ff0000` permanece `#ff0000`;
- nenhuma ocorrência textual fora de paint pode ser reescrita pelo exporter;
- geometria, transparência, raiz inline e writers existentes permanecem
  inalterados.

Não alterar a tabela de cores, aliases, parser, layout, API pública ou SVG
standalone. A implementação candidata é a remoção da transformação textual na
fronteira HTML-frame.

## 3. Prova RED → GREEN

Antes da implementação, adicionar teste focal no owner L3 construindo uma
`Page` com dois rects: um `Color::rgb(255, 65, 54)` e outro
`Color::rgb(255, 0, 0)`. O teste chama diretamente a entry point HTML-frame e
exige ambas as cores no SVG. No baseline, deve falhar porque as duas saem como
`#ff0000`; após a correção, deve passar.

Executar também os testes P1424 da fachada HTML para excluir regressão na ilha
e comparar por compilação pública um rect `fill: red` nos dois produtos.

## 4. Ataques e veredito segregado

Congelar antes do candidato:

- contrato e oráculos bilaterais;
- mutantes que removem paint, trocam `red` por vermelho puro, trocam vermelho
  puro por `red`, promovem igualdade unilateral, aceitam `Unknown` ou ignoram
  uma das duas cores;
- identidades dos dois binários e do baseline.

O gate exige score `1.0`, positivos preservados, opacos `Unknown`, três ordens
iguais e repetição independente. O certificado final fica em commit separado
do produto. Como sessão e filesystem são compartilhados, não há atestação de
isolamento.

## 5. Gate ADR-0127

É correção interna de serialização/paridade: não muda contrato público,
default, compatibilidade nem fase do pipeline. O L0 é atualizado primeiro e o
passo segue sem nova parada humana.

## 6. Critério de fecho

`cargo test -p typst-infra p1454`, testes P1424, campanha bilateral, ataques,
`cargo fmt --all -- --check`, `cargo build`, `crystalline-lint .` e
`git diff --check` passam. O veredito limita-se à preservação de paint na
variante SVG embutida de `html.frame`.
