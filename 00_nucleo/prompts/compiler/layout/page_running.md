# Prompt L0 — `compiler/layout/page_running` — composição marginal

Hash do Código: 02034ce3

**Camada:** L1
**Ficheiro proprietário:** `01_core/src/compiler/layout/page_running.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/page-running.toml sha256:88345b9a32ff8678a2ff236d21cbd7c18913820ed151fc704c116ab2542288ed

## Contrato

Resolver offsets contra margens, posicionar numeração por alinhamento, compor
header/footer explícitos em sub-frames e materializar callback realizado sem
reduzi-lo a plain text. Conteúdo vazio não cria layer; o estilo/morfologia do
conteúdo marginal é preservado. Tipos e defaults pertencem ao owner da entidade.
