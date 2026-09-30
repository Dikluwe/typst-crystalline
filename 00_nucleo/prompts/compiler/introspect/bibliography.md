# L0 — introspect/bibliography
Hash do Código: c77fe24a

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/bibliography.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:67-286` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `00_nucleo/diagnosticos/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

Preservar a varredura recursiva de chaves bibliográficas e converter apenas referências cuja chave aparece nas entradas. A transformação mantém a ordem e a estrutura dos demais conteúdos; o walk recebe as citações resultantes.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.
