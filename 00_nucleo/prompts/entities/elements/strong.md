# Prompt L0 — elemento `strong`
Hash do Código: 8b160586

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:cafcd80a58c3e84a9b44a49eb92a93cd2422ddbcf530d295c3645ae54bf1e41b

**Camada:** L1
**Ficheiro alvo:** `01_core/src/entities/elements/strong.rs`
**ADRs:** ADR-0026, ADR-0105, ADR-0107, ADR-0109, ADR-0129.

## Medição anterior à decisão

O consumer representa `*bold*` como variante semântica própria com um único
`body`. É transparente para plain text/vazio, recursa em maps e expõe somente o
campo `body`. O bold de render pertence ao layout.

## Contrato

- `StrongElem::new(body)` preserva o conteúdo.
- `plain_text` e `is_empty` delegam ao body.
- `map_content`/`map_text` reconstroem `Content::strong`.
- `get_field("body")` devolve o Content; demais campos retornam `None`.

## Aceitação

Preservar distinção morfológica de strong, transparência textual e recursão.
