# Passo 1287 — Fechar exportadores e emitir o veredito global de paridade

## Natureza

Documento tático de execução. Não é Prompt L0 e não legitima código.

## Objetivo

Adjudicar as diferenças residuais de PNG/PDF/SVG, executar um corpus bilateral amplo e
emitir um veredito sustentado sobre paridade com o vanilla ratificado.

## Pré-condições

- Passos 1281–1286 fechados.
- Suíte integral executável.
- Inventário bilateral atual sem lacuna nominal ou semântica não classificada.

## Escopo de exportação

1. PNG: localizar a origem dos 11 pixels divergentes e decidir se representam língua,
   render observável permitido ou bug; registrar tolerância somente se justificada.
2. PDF: adjudicar a identidade nominal de fonte mantendo texto, páginas, boxes e
   acessibilidade como observáveis separados.
3. SVG:
   - Linear/Radial em Luma e CMYK;
   - produto Conic residual por espaço de cor;
   - Tiling com conteúdo, imagem ou gradiente;
   - links entre páginas/bundle;
   - formatos de imagem opacos, dependências externas e SVGZ;
   - conflitos de glyph, fontes ausentes e wrappers sem fonte;
   - máscaras alfa e clipping even-odd.

## Corpus amplo obrigatório

Cobrir, no mínimo, texto e Unicode, bidi, estilos morfológicos, matemática, tabelas,
grids, colunas, floats, notas, contadores, referências, links, imagens, gradientes,
paginação, query, eval e todos os exportadores suportados.

## Disciplina

- Atualizar L0 antes de qualquer correção de código e aplicar o gate ADR-0127.
- Comparar semântica, sintaxe e morfologia; bytes e passos internos só são gate quando
  forem o observável público relevante.
- Cada número do relatório registra commit, árvore, hora, comando e hashes dos binários.
- Não transformar contagem de testes, paths ou pixels numa percentagem global de
  linguagem sem modelo de cobertura explicitamente aprovado.

## Validação final

- `cargo test --workspace`;
- testes do runner;
- inventário bilateral padrão e HTML;
- matriz focal;
- corpus bilateral amplo;
- `cargo build --workspace --bin typst`;
- `crystalline-lint .` com zero violações;
- `git diff --check`.

## Veredito permitido

O passo termina com exatamente uma classificação:

- **PARIDADE DEMONSTRADA** — nenhuma divergência de linguagem confirmada e corpus/
  inventário suficientes, com limitações explicitadas;
- **PARIDADE PARCIAL** — diferenças conhecidas, listadas e reproduzíveis;
- **INCONCLUSIVO** — harness, cobertura ou proveniência ainda impedem a decisão.

Não declarar paridade apenas porque a matriz focal está verde.
