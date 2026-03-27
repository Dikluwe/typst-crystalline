# ⚖️ ADR-0020: `fontdb` → L3 (adiada)

**Status**: `PROPOSTO — adiada`
**Data**: 2026-03-23

---

## Contexto

`fontdb` é uma crate de descoberta de fontes do sistema — percorre
directórios standard (`/usr/share/fonts`, `~/.fonts` em Linux;
`/Library/Fonts` em macOS; `C:\Windows\Fonts` em Windows) e
constrói um índice de fontes disponíveis.

O Typst original usa `fontdb` no `SystemWorld` do CLI para encontrar
fontes do sistema automaticamente sem que o utilizador precise de
especificar paths explicitamente.

---

## Estado actual

O Passo 8 implementa `SystemWorld::new(root, main, font_paths)`
com `font_paths: Vec<PathBuf>` explícitos. Isto é suficiente para
compilar documentos quando os paths de fontes são conhecidos.

`fontdb` não é necessária para o pipeline básico de compilação —
é uma conveniência de UX (descoberta automática) não um requisito
de correctitude.

---

## Decisão

**Adiada.** `fontdb` não entra em `03_infra` no Passo 8.

Condição de activação: quando o CLI cristalino (Passo 11+) precisar
de descoberta automática de fontes do sistema sem que o utilizador
especifique `--font-path`.

Quando activada, `fontdb` vai para `03_infra/Cargo.toml`:

```toml
[dependencies]
fontdb = "0.x"   # ADR-0020 — descoberta de fontes do sistema
```

E `SystemWorld` ganha um método de construção alternativo:

```rust
impl SystemWorld {
    /// Constrói com fontes do sistema descobertas automaticamente.
    pub fn with_system_fonts(root: PathBuf, main: PathBuf)
        -> Result<Self, WorldError>
    {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        let font_paths: Vec<PathBuf> = db.faces()
            .filter_map(|f| f.source.path().map(PathBuf::from))
            .collect();
        Self::new(root, main, font_paths)
    }
}
```

---

## Análise de pureza (para registo)

`fontdb` faz I/O de sistema de ficheiros e lê variáveis de ambiente
para localizar directórios de fontes — é infraestrutura pura, L3
sem questão.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/infra/system-world.md` | Actualizar quando activada — método `with_system_fonts` |

---

## Referências

- ADR-0019 — `ttf-parser` e `rustybuzz` em L3
- `fontdb`: https://github.com/RazrFalcon/fontdb
