# Prompt L0 — `wiring/tests/p1291_callback_runtime` — contrato black-box de callbacks math
Hash do Código: 7b63f9c7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4bf17f1455eef032ab3e30ea038edabed721e8378b913aaecf2b544bf288a917

**Camada:** L4 — teste de integração
**Ficheiro alvo:** `04_wiring/tests/p1291_callback_runtime.rs`
**Origem:** `P1291.cancel-angle-runtime`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

O teste independente P1291 executa o binário cristalino como caixa-preta para
observar a linguagem final, mas um consumer Rust sem `@prompt` viola V1. Ele
não pode partilhar o owner produtivo de callbacks: ADR-0129 exige Prompt L0
próprio 1:1 para este consumer de teste.

## Contrato

O consumer compila fixtures temporárias e compara somente o resultado final ou
o diagnóstico:

- callback `angle` altera a geometria e coincide com o ângulo explícito de
  controlo;
- snapshot contextual observa o tamanho lexical e o tamanho math derivado em
  script;
- `cross` prevalece sobre `inverted` na morfologia das duas linhas;
- retorno inválido preserva path/linha da chamada, falha sem panic e não cria
  output.

Arquivos temporários são locais ao teste e removidos ao final. O teste não lê
implementação, não aceita documento provisório, não converte `Unknown` em
sucesso e não cobre `vec`/`underline`.

## Verificação

`cargo test -p typst-wiring --test p1291_callback_runtime`, V1/V5/V15/V26,
`cargo fmt --all -- --check` e `git diff --check`. Este prompt possui
exatamente o consumer acima e não legitima código produtivo.
