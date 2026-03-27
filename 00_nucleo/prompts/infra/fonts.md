# Prompt L0 — infra/fonts

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/fonts.rs`
**ADRs relevantes**: ADR-0019 (ttf-parser, rustybuzz → L3)

## Contexto

`FontSlot` encapsula o carregamento lazy de fontes OpenType/TrueType.
`discover_fonts` varre paths do filesystem e cria slots para cada
face de fonte encontrada.

`ttf-parser` e `rustybuzz` não saem da fronteira de L3 — L1 recebe
apenas `Font(Vec<u8>)` opaco.

## Interface pública

```rust
pub struct FontSlot {
    pub path:  PathBuf,
    pub index: u32,
    font:      OnceLock<Option<Font>>,
}

impl FontSlot {
    pub fn new(path: PathBuf, index: u32) -> Self
    pub fn get(&self) -> Option<Font>
}

pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot>
```

## Critérios de Verificação

```
Dado FontSlot com path inexistente
Quando get() for chamado
Então None

Dado discover_fonts com directório vazio
Quando chamado
Então Vec vazio

Dado discover_fonts com directório contendo .ttf inválido
Quando chamado
Então Vec não vazio (slots criados), mas get() retorna None
```
