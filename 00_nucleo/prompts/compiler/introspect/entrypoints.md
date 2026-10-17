# L0 — introspect/entrypoints
Hash do Código: 598bead3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/entrypoints.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:750-921` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `00_nucleo/diagnosticos/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

Executar a passagem pura e as variantes de bundle/runtime com um Locator causal compartilhado, registrar sementes antes dos bodies, preparar outline e pais uma vez e coordenar os pós-processadores sem reavaliar o documento.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.


## Realização com avaliação única

Entry points reservam Locations explícitas de toda a entrada antes de novos elementos ordinários/contextuais, conservando disjunção e ordem de tags.

### Pertencimento de ocorrências em bundles

Os dois entrypoints de bundle registram privadamente no CounterRegistry as Locations dos Tag::Start produzidos por cada body. Checkpoints continuam servindo ao layout; não são limites exclusivos da identidade semântica. Bodies vazios registram um escopo vazio.
