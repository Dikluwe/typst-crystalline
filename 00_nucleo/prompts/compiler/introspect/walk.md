# L0 — introspect/walk
Hash do Código: 15c33ffc

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/walk.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:1897-2485` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `00_nucleo/diagnosticos/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

Percorrer Content por match exaustivo e estático. Alocar Location, capturar payload e snapshot antes de Start, descer uma vez segundo cada variante e emitir End no mesmo Location após a recursão. Delegar Heading e Label a seus módulos existentes; não executar callbacks nem I/O.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.
