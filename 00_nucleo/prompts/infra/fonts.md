# Prompt L0 — `infra/fonts` — Gestão e Carregamento de Fontes

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/fonts.rs`
**Criado em**: 2026-03-26 (Passo 11)
**Atualizado em**: 2026-04-12 (restauro — expandido com font_info_from_bytes, build_font_book, .ttc multi-face, ADR-0022)
**ADRs relevantes**: ADR-0019 (`ttf-parser` → L3 exclusivo), ADR-0022 (`FontInfo` — L1 recebe apenas campos primitivos)

---

## Contexto e Objetivo

O sistema precisa mapear pedidos de fonte ("Arial, Bold") para bytess binários
no disco ou memória. Este módulo gere o "livro de fontes" (*Font Book*) e é o
**sistema central de resolução tipográfica** — todo o I/O de ficheiros de fonte
está confinado aqui.

### Fronteira de Arquitectura

- `ttf_parser` **não escapa** a fronteira de L3: L1 recebe apenas
  `Font(Vec<u8>)` opaco (tipo de L1 em `world_types.rs`) e `FontInfo`
  (struct de L1 com campos primitivos — `String`, `FontVariant`, `FontFlags`).
- `OnceLock<Option<Font>>` garante carregamento **lazy** — os bytes da fonte
  são lidos do disco apenas na primeira chamada a `FontSlot::get()`.
- Validação de bytes acontece em `get()` (via `ttf_parser::Face::parse`),
  **não** na descoberta. Um slot pode ser criado para um ficheiro inválido;
  `get()` retornará `None`.

---

## Interface Pública

### `FontSlot` — carregamento lazy de uma face de fonte

```rust
pub struct FontSlot {
    pub path:  PathBuf,  // caminho do ficheiro no disco
    pub index: u32,      // índice da face num TrueType Collection (.ttc); 0 para fontes simples
    font:      OnceLock<Option<Font>>,  // interior mutável — thread-safe
}

impl FontSlot {
    pub fn new(path: PathBuf, index: u32) -> Self

    /// Carrega e valida lazy. None se: ficheiro não existe, não legível,
    /// bytes inválidos, ou índice fora dos limites da colecção.
    /// OnceLock garante idempotência — resultado sempre igual para mesma instância.
    pub fn get(&self) -> Option<Font>
}
```

### `discover_fonts` — varrimento de paths

```rust
/// Descobre fontes nos paths fornecidos.
/// Cada path pode ser: ficheiro de fonte directamente, ou directório (varrido recursivamente).
/// Extensões aceites: .ttf, .otf, .ttc, .otc
/// Colecções .ttc → múltiplos slots (um por face via face_count / ttf_parser::fonts_in_collection)
pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot>
```

### `font_info_from_bytes` — extrai metadados para o FontBook

```rust
/// Extrai FontInfo de bytes de fonte OpenType/TrueType (ADR-0022).
/// ttf_parser fica em L3 — L1 recebe apenas FontInfo com campos primitivos.
/// Retorna None se bytes inválidos ou índice inexistente.
///
/// Nota: serif não detectado pelo ttf_parser → flags.serif = false sempre
/// (heurísticas por nome de família são trabalho futuro).
pub fn font_info_from_bytes(data: &[u8], index: u32) -> Option<FontInfo>
```

Campos extraídos:
- **`family`**: nome tipográfico (TYPOGRAPHIC_FAMILY ou FAMILY em inglês; fallback para qualquer idioma)
- **`variant.style`**: `Italic` se `face.is_italic()`, `Oblique` se `face.is_oblique()`, senão `Normal`
- **`variant.weight`**: `FontWeight(face.weight().to_number())` — escala OpenType 100–900
- **`variant.stretch`**: `FontStretch::from_number(face.width().to_number())`
- **`flags.monospace`**: `face.is_monospaced()`
- **`flags.serif`**: `false` (trabalho futuro)

### `build_font_book` — popula o FontBook a partir de slots

```rust
/// Lê os bytes de cada slot e extrai FontInfo via font_info_from_bytes.
/// Slots inválidos (bytes ilegíveis ou fonte inválida) são silenciosamente ignorados.
/// NOTA: duplica o I/O com FontSlot::get() — optimização futura (Passo 11).
pub fn build_font_book(slots: &[FontSlot]) -> FontBook
```

---

## Funções Internas

| Função | Responsabilidade |
|--------|-----------------|
| `is_font_file(path)` | Verifica extensão: `.ttf`, `.otf`, `.ttc`, `.otc` |
| `face_count(path)` | Lê `ttf_parser::fonts_in_collection(data)` → n faces; fallback: 1 |
| `push_slots(path, slots)` | Cria `face_count` slots para o ficheiro (suporte a `.ttc`) |
| `discover_in_dir(dir, slots)` | Varredura recursiva de directório |

---

## Critérios de Verificação

```
// FontSlot
FontSlot::new("/nao/existe.ttf", 0).get() = None
FontSlot::new(path_bytes_invalidos, 0).get() = None

// Idempotência do OnceLock
slot.get() == slot.get()   // sempre igual para a mesma instância

// discover_fonts
discover_fonts(&[directorio_vazio])                   = []
discover_fonts(&[dir_com_readme_txt_e_data_bin])      = []  // extensões não-fonte ignoradas
discover_fonts(&[dir_com_fake_dot_ttf])               = [slot]  // slot criado
// slot de bytes inválidos → get() = None:
discover_fonts(&[dir_com_fake_dot_ttf])[0].get() = None

// font_info_from_bytes
font_info_from_bytes(b"not a font", 0) = None

// build_font_book
build_font_book(&[slot_invalido]).is_empty() = true
// com fonte real (.ttf fixture):
// FontInfo.family não vazio
// FontInfo.variant.weight ∈ [100, 900]
// FontInfo.flags.monospace: correcto para fonte monospace
```

---

## Relação com Outros Módulos

| Módulo | Como consome `fonts.rs` |
|--------|------------------------|
| `FontBookMetrics` (este crate, L3) | Consome `Font(Vec<u8>)` de `FontSlot::get()` para construir `Face` |
| `SystemWorld` (L3 — `world.rs`) | Chama `discover_fonts` na inicialização; chama `build_font_book` para o `FontBook` |
| `MathLayouter` e `Layouter` (L1) | Recebem `&dyn FontMetrics` — nunca tocam em `FontSlot` |
| `FontBook` (L1) | Recebe `FontInfo` (primitivos) — nunca recebe `ttf_parser::Face` |

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-03-26 | Criação — Passo 11: `FontSlot`, `discover_fonts` (lazy I/O) | `fonts.rs` |
| 2026-04-12 | Restauro — expandido: `font_info_from_bytes` (ADR-0022), `build_font_book`, suporte `.ttc`, relação com SystemWorld | `fonts.md` |
