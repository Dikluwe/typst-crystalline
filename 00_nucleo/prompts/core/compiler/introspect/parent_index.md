# L0 — introspect/parent_index
Hash do Código: 1f01298e

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/parent_index.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:1693-1746` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

Construir relações parentais empilhando Starts reais e desempilhando Ends; registros derivados não inventam ancestrais nem sobrescrevem relações das ocorrências.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/core/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.
