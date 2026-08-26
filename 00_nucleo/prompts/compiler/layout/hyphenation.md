# Prompt L0 — hyphenation pura
Hash do Código: b24b6f9c

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/hyphenation.rs`

## Contrato e aceitação

Mapear Lang para padrões TeX embutidos e devolver pontos de quebra válidos em
chars. Idioma ausente/não suportado e palavra indivisível retornam vazio, sem
I/O, panic ou estado global.
