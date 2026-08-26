# Prompt L0 — `infra/runtime_info` — snapshot do runtime

Hash do Código: 16249cb6

**Camada:** L3
**Ficheiro proprietário:** `03_infra/src/runtime_info.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shell/info-projection.toml sha256:f7f63d22ab1b490c71f5e93f5fdf22da8e068dc15dfe1f9aa9cdb5fd7f95ab4d

## Contrato

Capturar paths de packages por HOME/XDG, `TYPST_FONT_PATHS`, allowlist de env e
somente a presença de `TYPST_CERT`. Não tocar rede nem varrer fontes. Valores
de certificado/proxy nunca entram no snapshot.
