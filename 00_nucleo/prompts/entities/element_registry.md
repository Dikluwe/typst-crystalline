# Prompt L0 — registro injetado de elementos
Hash do Código: 0b151b49

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:cafcd80a58c3e84a9b44a49eb92a93cd2422ddbcf530d295c3645ae54bf1e41b

**Camada:** L1
**Ficheiro alvo:** `01_core/src/entities/element_registry.rs`
**ADRs:** ADR-0029, ADR-0105, ADR-0106, ADR-0108, ADR-0129.

## Medição anterior à decisão

O consumer mapeia nome para `ElementCtor`, é injetado e não possui estado
global. Construtores são `Arc<dyn Fn + Send + Sync>`, recebem valores e devolvem
`SourceResult<Content>`. A suíte varre cada registro e prova construção e
dispatch da porta dinâmica.

## Contrato

- Registrar/substituir construtor por nome, consultar nomes e clonar ctor O(1).
- Construir elemento conhecido; nome desconhecido produz diagnóstico, não panic.
- Permanecer puro em L1, sem `static`, `OnceLock` ou I/O.
- A trava percorre todos os nomes e exercita o contrato dinâmico e `get_field`.

## Aceitação

Registro vazio, lookup, construção, erro desconhecido e varredura passam sem
alterar a superfície vigente.
