# Relatório de Paridade de Produção — Passo 517

**Data:** 2026-06-30  
**Tema:** Fontes do sistema por defeito na CLI e marcação de subset no PDF  
**Alcance:** Fecho da Trilha 5 de produção: system fonts + subset marking.

---

## 1. Resumo Executivo

O Passo 517 implementou as duas últimas melhorias de produção pendentes da Trilha 5:

1. **Fontes do sistema activas por defeito na CLI** — L4 (`04_wiring/src/main.rs`) passou a usar `SystemWorld::with_fonts_and_system(&font_paths)`, combinando `--font-path` com fontes do sistema via `fontdb`.
2. **Marcação de subset no PDF** — quando uma fonte é subsetada, o nome `/BaseFont` no PDF recebe o prefixo `AAAAAA+`, conforme convenção dos produtores PDF e requisito para que ferramentas como `pdffonts` reportem `sub = yes`.

| Métrica | Valor medido |
|---------|--------------|
| `cargo test -p typst-core` | 3550 passed; 0 failed |
| `cargo test -p typst-wiring` | 23 passed; 0 failed |
| `cargo test -p typst-infra` | 555 passed; 0 failed |
| `cargo build --release` | ✅ OK |
| `crystalline-lint .` | 0 violations |
| Bateria P490 + P500 | 37/37 OK |
| Fixture 09-cidfont | 30 KB, `AAAAAA+CrystallineFont` |

---

## 2. Auditoria L0 e ADRs

Os Prompts L0 afectados já existiam graças ao Passo 515/516:

- `00_nucleo/prompts/infra/fontdb.md` — descoberta de system fonts.
- `00_nucleo/prompts/infra/export/font_subset.md` — subsetting e remapeamento.
- `00_nucleo/prompts/wiring.md` — orquestração L4.

Foram actualizados:

- `wiring.md`: pipeline agora usa `with_fonts_and_system`; removido o scope-out de system fonts.
- `font_subset.md`: adicionado critério de prefixo `AAAAAA+` no nome subsetado.

O linter (`crystalline-lint --fix-hashes .`) sincronizou o hash de `04_wiring/src/main.rs`, `04_wiring/tests/cli.rs` e `04_wiring/tests/crystalline_lint.rs` para `ae486d4c`.

---

## 3. Implementação — System Fonts por Defeito

### Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `04_wiring/src/main.rs` | `SystemWorld::new(...).with_fonts_and_system(&font_paths)` |
| `00_nucleo/prompts/wiring.md` | Pipeline actualizado |

### Comportamento

- Sem `--font-path`: o compilador carrega apenas as fontes do sistema.
- Com `--font-path DIR`: o compilador carrega fontes do sistema **e** as fontes do directório indicado, com as do projecto em primeiro lugar (ordem preservada por `with_fonts_and_system`).
- O método anterior `with_fonts(...)` permanece disponível em L3 para testes e casos controlados.

### Decisões

- A decisão de usar system fonts por defeito ficou em L4 (wiring), não em L2 (CLI). L2 continua apenas a transportar `font_paths` no `RunIntent`.
- Não foi adicionada flag `--ignore-system-fonts`; se necessário, entra como scope-out futuro em L2.

---

## 4. Implementação — Marcação de Subset

### Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `03_infra/src/export/builder.rs` | `subset_font_name(base_name, ...)` com prefixo `AAAAAA+`; aplicado em `build_cidfont` e `build_multifont` |
| `03_infra/src/integration_tests.rs` | Aceita `+CrystallineFont` no assert do nome de fonte |
| `00_nucleo/prompts/infra/export/font_subset.md` | Critério de prefixo documentado |

### Comportamento

```rust
fn subset_font_name(base_name: &str, _subset_data: &[u8]) -> String {
    format!("AAAAAA+{}", base_name)
}
```

- O prefixo é aplicado **apenas** quando `glyph_mapping` não está vazio, isto é, quando o subsetting foi efectivamente realizado.
- Se a fonte cair em fallback (fonte completa), o nome permanece sem prefixo.
- O prefixo é fixo (`AAAAAA+`) para evitar instabilidade nos snapshots entre builds debug/release.

### Validação externa

```bash
pdffonts /tmp/09-cidfont-new.pdf
```

```
name                                 type              encoding         emb sub uni object ID
------------------------------------ ----------------- ---------------- --- --- --- ---------
AAAAAA+CrystallineFont               CID TrueType      Identity-H       yes yes yes      5  0
```

- `sub = yes` confirma que o PDF identifica a fonte como subsetada.

---

## 5. Validação

### Comandos executados

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline
cargo test -p typst-core
cargo test -p typst-wiring
cargo test -p typst-infra
cargo build --release
crystalline-lint .

# Fixture 09-cidfont
target/release/typst --font-path lab/krilla-reference/assets/fonts \
  03_infra/fixtures/p307b/sources/09-cidfont.typ \
  /tmp/09-cidfont-new.pdf
pdffonts /tmp/09-cidfont-new.pdf

# Bateria P490/P500
mkdir -p /tmp/p517
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" "/tmp/p517/$(basename "$f" .typ).pdf" >/dev/null 2>&1 \
    && echo "OK: $(basename "$f")" \
    || echo "FAIL: $(basename "$f")"
done
```

### Resultados

- **typst-core**: 3550 passed; 0 failed
- **typst-wiring**: 23 passed; 0 failed
- **typst-infra**: 555 passed; 0 failed
- **cargo build --release**: sucesso
- **crystalline-lint**: 0 violations
- **Bateria P490/P500**: 37/37 OK
- **Fixture 09-cidfont**: PDF válido, 30 206 B, texto extraível, `AAAAAA+CrystallineFont` com `sub = yes`

---

## 6. Snapshot 09-cidfont

O snapshot `03_infra/fixtures/p307b/reference/09-cidfont.pdf` foi actualizado para 30 206 bytes.

Nota técnica: `oxifont-subset` produz bytes ligeiramente diferentes entre perfis debug e release. Para estabilizar o snapshot:

1. O nome da fonte foi fixado em `AAAAAA+CrystallineFont` (prefixo constante).
2. O snapshot foi regenerado a partir do binary de teste (debug) usado pela suite `p307b_snapshot`, garantindo reprodutibilidade no CI.

---

## 7. Brechas Remanescentes (Pós-P517)

| Brecha | Tamanho | Estado |
|--------|---------|--------|
| CFF subsetting | XL | `oxifont-subset` suporta CFF, mas fallback para fonte completa permanece activo |
| Variation fonts (VF) | XL | Fora de scope — subsetting de eixos não implementado |
| Flag `--ignore-system-fonts` | S | Não solicitado; adicionar em L2 se necessário |
| Kerning no subset | M | Tabelas GPOS/GSUB/kern são removidas pelo subset; posicionamento já aplicado em `x_offset`/`x_advance` |

---

## 8. Próximos Passos (P518+)

| Passo | Foco | Prioridade |
|-------|------|------------|
| **P518** | DEBT-42 Benchmark revalidado (shaping + subsetting + system fonts) | 🟡 Média |
| **P519** | Lookahead Layout Engine (inovação arquitetural) | 🟢 Futuro |
| **P520** | Outros exports (PNG/SVG/HTML) | 🟢 Futuro |

---

## 9. Ficheiros Alterados

| Camada | Ficheiro | Alteração |
|--------|----------|-----------|
| L0 | `00_nucleo/prompts/wiring.md` | Actualizado (system fonts por defeito) |
| L0 | `00_nucleo/prompts/infra/export/font_subset.md` | Actualizado (prefixo `AAAAAA+`) |
| L3 | `03_infra/src/export/builder.rs` | Marcação de subset no nome da fonte |
| L3 | `03_infra/src/integration_tests.rs` | Assert flexível para nome com prefixo |
| L4 | `04_wiring/src/main.rs` | `with_fonts_and_system` por defeito |
| Lab | `03_infra/fixtures/p307b/reference/09-cidfont.pdf` | Snapshot actualizado (30 206 B) |
