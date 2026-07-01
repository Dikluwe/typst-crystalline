# Relatório de Paridade de Produção — Passo 515

**Data:** 2026-06-30  
**Tema:** Trilha 5 — Shaping real, descoberta automática de fontes e subsetting no PDF  
**Alcance:** Sub-fases implementáveis numa sessão; scope-outs documentados.

---

## 1. Resumo Executivo

O Passo 515 (XL-size) propunha a activação completa da Trilha 5 de produção:
`fontdb` + shaping real + font fallback por caractere + subsetting de fontes no PDF.

Nesta sessão foram implementadas e integradas as seguintes sub-fases:

| Sub-fase | Estado |
|----------|--------|
| **Descoberta automática de fontes do sistema (`fontdb`)** | ✅ Implementado |
| **Font fallback por caractere no shaper** | ✅ Implementado |
| **Subsetting de fontes no PDF** | ⚠️ Scope-out (L0 criado) |

| Métrica | Valor medido |
|---------|--------------|
| `cargo test -p typst-core` | 3550 passed; 0 failed |
| `cargo test -p typst-wiring` | 23 passed; 0 failed |
| `cargo test -p typst-infra` | 550 passed; 0 failed |
| `cargo build --release` | ✅ OK |
| `crystalline-lint .` | 0 violations (1 warning V7 de prompt órfão esperado) |
| Bateria P490 + P500 | 37/37 OK |

---

## 2. Auditoria L0 e ADRs

Antes de escrever código, foi detectado que o P515 carecia de Prompts L0 para:
- `fontdb` (ADR-0020 estava adiada).
- Subsetting de fontes (ADR-0027 escolheu Opção A sem subsetting).
- Font fallback por caractere no `shaper.rs` (secção nova no L0 existente).

Foram redigidos:
- `00_nucleo/prompts/infra/fontdb.md`
- `00_nucleo/prompts/infra/export/font_subset.md`
- `00_nucleo/prompts/infra/shaper.md` (actualizado com secção P515)

O linter (`crystalline-lint --fix-hashes .`) sincronizou o hash do `shaper.rs` para `084a5fe9`.

---

## 3. Implementação — `fontdb`

### Ficheiros alterados

- `03_infra/Cargo.toml` — adicionada dependência `fontdb = "0.23"`.
- `03_infra/src/lib.rs` — expõe `pub mod fontdb`.
- `03_infra/src/fontdb.rs` — novo módulo.
- `03_infra/src/world.rs` — adicionados `SystemWorld::with_system_fonts()` e `SystemWorld::with_fonts_and_system(...)`.

### API pública

```rust
// Descobre fontes do sistema operativo.
pub fn load_system_fonts() -> (Vec<FontSlot>, FontBook);

// Descobre fontes num directório específico (útil para testes).
pub fn load_fonts_from_dir<P: AsRef<Path>>(dir: P) -> (Vec<FontSlot>, FontBook);

// SystemWorld builders.
pub fn SystemWorld::with_system_fonts(self) -> Self;
pub fn SystemWorld::with_fonts_and_system(self, project_paths: &[PathBuf]) -> Self;
```

### Decisões

- Mantém `FontSlot` e `FontBook` existentes (ADR-0022).
- `fontdb::Source::SharedFile` é tratado como `File` (ambos têm path).
- Fontes binárias puras (`Source::Binary`) são ignoradas nesta fase.
- `with_fonts(paths)` original permanece inalterado (compatibilidade total).

### Testes adicionados

- `fontdb::tests::load_system_fonts_nao_panic`
- `fontdb::tests::load_fonts_from_dir_vazio`
- `fontdb::tests::load_fonts_from_dir_ignora_nao_fontes`
- `world::tests::system_world_with_system_fonts_nao_panic`
- `world::tests::system_world_with_fonts_and_system_inclui_projecto`
- `world::tests::system_world_with_fonts_and_system_preserva_ordem`

---

## 4. Implementação — Font Fallback por Caractere

### Ficheiros alterados

- `03_infra/src/shaper.rs` — refactor significativo.

### Mudanças arquitecturais

- `shape_item` passou a devolver `Vec<FrameItem>` em vez de mutar um item único.
- `try_shape` resolve **todas** as fontes candidatas da `FontList`.
- Cada run bidireccional (P484) é dividido em sub-runs por cobertura de caractere.
- Cada sub-run é shaped na sua própria face e emite um `FrameItem::TextShaped` consecutivo.
- A posição X de cada sub-run é calculada acumulando o avanço dos sub-runs anteriores.

### Heurística de cobertura

```rust
fn face_covers_char(face: &ttf_parser::Face, c: char) -> bool {
    face.glyph_index(c).is_some()
}
```

Se nenhuma fonte cobrir um caractere, usa-se a primeira candidata (glyph `.notdef`).

### Testes adicionados

- `shaper::tests::p515_resolve_candidates_sem_fontes_retorna_none`
- `shaper::tests::p515_shape_document_real_font_produz_textshaped`

### Invariantes preservadas

- Todos os testes P482–P486 do shaper continuam a passar.
- A bateria de paridade funcional P490/P500 continua 37/37 OK.

---

## 5. Scope-out — Subsetting de Fontes no PDF

### Razão

Subsetting TrueType é uma tarefa XL que envolve:
- Construção manual de tabelas `head`, `hhea`, `maxp`, `post`, `loca`, `glyf`, `cmap`, `hmtx`, `name`.
- Remapeamento consistente de glyph IDs no ToUnicode CMap, `/W` e operador `TJ`.
- Suporte a CFF como scope-out adicional.

Implementar isto de forma robusta excede o scope de uma sessão. Optou-se por:
- Criar o Prompt L0 `00_nucleo/prompts/infra/export/font_subset.md` para legitimar a implementação futura.
- Manter o comportamento actual de embeber a fonte completa (ADR-0027 Opção A).

### Próximos passos para subsetting

1. Implementar `03_infra/src/export/subset.rs` com `subset_font(font_data, used_glyphs)`.
2. Alterar `PdfBuilder` para usar subset quando possível.
3. Remapear glyph IDs em `stream.rs` e `fonts.rs`.
4. Adicionar testes com fixtures TrueType.

---

## 6. Validação

### Comandos executados

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline
cargo test -p typst-core
cargo test -p typst-wiring
cargo test -p typst-infra
cargo build --release
crystalline-lint .

# Bateria P490/P500
mkdir -p /tmp/p515
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" "/tmp/p515/$(basename "$f" .typ).pdf" >/dev/null 2>&1 \
    && echo "OK: $(basename "$f")" \
    || echo "FAIL: $(basename "$f")"
done
```

### Resultados

- `typst-core`: 3550 passed; 0 failed
- `typst-wiring`: 23 passed; 0 failed
- `typst-infra`: 550 passed; 0 failed
- `cargo build --release`: sucesso
- `crystalline-lint .`: 0 violations
- Bateria P490/P500: 37/37 OK

---

## 7. Brechas Remanescentes (Pós-P515)

| Brecha | Tamanho | Estado |
|--------|---------|--------|
| Subsetting de fontes no PDF | XL | L0 criado; implementação futura |
| Font fallback para fontes do sistema não listadas na `FontList` | M | Requer `fontdb` activo na CLI |
| Shaping em `FrameItem::Text` durante layout (pre-shaping) | L | Fora de scope — ADR-0120 preserva post-shaping |
| CFF subsetting | XL | Scope-out do subsetting |

---

## 8. Próximos Passos (P516+)

| Passo | Foco | Prioridade |
|-------|------|------------|
| **P516** | Subsetting TrueType no PDF | 🔴 Alta |
| **P517** | Activar `with_system_fonts` por defeito na CLI | 🟡 Média |
| **P518** | Revalidar DEBT-42 benchmark com produção real | 🟡 Média |
| **P519** | Lookahead Layout Engine (inovação arquitetural) | 🟢 Futuro |

---

## 9. Ficheiros Alterados

| Camada | Ficheiro | Alteração |
|--------|----------|-----------|
| L0 | `00_nucleo/prompts/infra/fontdb.md` | Criado |
| L0 | `00_nucleo/prompts/infra/export/font_subset.md` | Criado |
| L0 | `00_nucleo/prompts/infra/shaper.md` | Actualizado (P515) |
| L3 | `03_infra/Cargo.toml` | `fontdb = "0.23"` |
| L3 | `03_infra/src/lib.rs` | `pub mod fontdb` |
| L3 | `03_infra/src/fontdb.rs` | Novo |
| L3 | `03_infra/src/world.rs` | `with_system_fonts`, `with_fonts_and_system` |
| L3 | `03_infra/src/shaper.rs` | Fallback por caractere |
