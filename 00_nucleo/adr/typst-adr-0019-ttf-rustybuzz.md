# ⚖️ ADR-0019: `ttf-parser` e `rustybuzz` → L3

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-23

---

## Contexto

O Passo 8 requer carregamento e parsing de fontes OpenType/TrueType
em `03_infra`. Duas crates são necessárias:

**`ttf-parser`**: parsing de ficheiros de fonte binários (tabelas
OpenType, metadata de família, extracção de glifos). Usado em
`FontBook` (metadata) e `Font` (acesso às tabelas).

**`rustybuzz`**: shaping de texto — processo de converter sequências
de codepoints Unicode em sequências de glifos com posicionamento
correcto para cada script. Usa `ttf-parser` internamente para
aceder às tabelas de fonte.

Ambas as crates lêem dados binários em memória (`&[u8]` ou `Bytes`)
— não fazem I/O directamente. O I/O (ler o ficheiro `.ttf` do
disco) é feito por `SystemWorld::file()` antes de passar os bytes
para estas crates.

---

## Análise de pureza (para L3)

| Propriedade | ttf-parser | rustybuzz |
|-------------|-----------|-----------|
| I/O directo | ✗ — opera sobre `&[u8]` | ✗ — opera sobre dados já carregados |
| Estado global mutável | ✗ | ✗ |
| Código `unsafe` | ✓ — necessário para parsing de binários | ✓ — interop com HarfBuzz |
| Adequado para L1 | ✗ — parsing binário não é domínio | ✗ — shaping não é domínio |
| Adequado para L3 | ✓ | ✓ |

`unsafe` em L3 é aceitável quando confinado a parsing de formatos
binários externos. L1 nunca recebe `ttf_parser::Face` nem
`rustybuzz::Face` — a fronteira L3→L1 é `Font(Vec<u8>)` (bytes
opacos do stub).

---

## Decisão

`ttf-parser` e `rustybuzz` são adicionados a `03_infra/Cargo.toml`:

```toml
[dependencies]
ttf-parser  = "0.x"   # ADR-0019 — parsing de tabelas OpenType/TrueType
rustybuzz   = "0.x"   # ADR-0019 — shaping de texto
```

**Não entram em `[l1_allowed_external]`** — são dependências de L3
exclusivamente. V14 dispara se algum tipo destas crates aparecer
em assinaturas de L1.

### Fronteira L3→L1

```rust
// L3 — 03_infra/src/fonts.rs
use ttf_parser::Face;

pub struct FontSlot {
    pub path:  PathBuf,
    pub index: u32,
    font:      OnceLock<Option<typst_core::entities::world_types::Font>>,
}

impl FontSlot {
    pub fn get(&self) -> Option<typst_core::entities::world_types::Font> {
        self.font.get_or_init(|| {
            let data = std::fs::read(&self.path).ok()?;
            // Validar com ttf_parser — se inválido, retornar None
            Face::parse(&data, self.index).ok()?;
            // Passar bytes opacos para L1 — ttf_parser não escapa
            Some(typst_core::entities::world_types::Font(data))
        }).clone()
    }
}
```

`ttf_parser::Face` nunca aparece em assinaturas de L1. `Font(Vec<u8>)`
em L1 é um contentor opaco de bytes — quando `Font` real migrar para
L1, a fronteira muda sem alterar `World`.

---

## ADR-0020 (fontdb) — adiada

`fontdb` para descoberta automática de fontes do sistema
(`/usr/share/fonts`, `~/.fonts`, etc.) é opcional para o Passo 8.
`SystemWorld::new()` aceita `font_paths: Vec<PathBuf>` explícitos.
Se no futuro for necessária descoberta automática, ADR-0020 trata
`fontdb`.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/infra/fonts.md` | Criar — FontSlot, carregamento lazy |
| `00_nucleo/prompts/infra/system-world.md` | Actualizar — font_paths, FontSlot |

---

## Consequências

**Positivas**: `SystemWorld::font()` retorna `Some(Font)` para fontes
reais; pipeline avança para documentos com fontes.

**Negativas**: `unsafe` em L3 — confinado a parsing de binários,
auditável e isolado da lógica de domínio.

**Neutras**: A fronteira `Font(Vec<u8>)` é temporária — quando
`Font` real migrar para L1, os bytes são substituídos por uma struct
com campos tipados sem alterar a interface de `World`.

---

## Referências

- Análise foundations/ — FontBook e Font classificados como L3
- ADR-0005 — World trait e stubs opacos
- ADR-0020 — fontdb (descoberta automática, adiada)
- `ttf-parser`: https://github.com/RazrFalcon/ttf-parser
- `rustybuzz`: https://github.com/nickel-lang/rustybuzz
