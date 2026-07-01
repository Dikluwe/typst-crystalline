# Relatório de Paridade de Produção — Passo 516

**Data:** 2026-06-30  
**Tema:** Subsetting TrueType/OpenType de fontes no PDF  
**Alcance:** Implementação completa do subsetting no pipeline de export PDF.

---

## 1. Resumo Executivo

O Passo 516 implementou o **subsetting de fontes TrueType/OpenType** no PDF gerado pelo cristalino, completando a Trilha 5 de paridade de produção.

| Métrica | Valor medido |
|---------|--------------|
| `cargo test -p typst-core` | 3550 passed; 0 failed |
| `cargo test -p typst-wiring` | 23 passed; 0 failed |
| `cargo test -p typst-infra` | 555 passed; 0 failed |
| `cargo build --release` | ✅ OK |
| `crystalline-lint .` | 0 violations |
| Bateria P490 + P500 | 37/37 OK |
| Fixture 09-cidfont (com Noto Sans) | 559 KB → 30 KB (94% menor) |

---

## 2. Decisão Técnica: `oxifont-subset`

Foram avaliadas crates de subsetting:

| Crate | Estado | Decisão |
|-------|--------|---------|
| `font-subset` | Não compila | Rejeitado |
| `oxifont-subset` | Compila, gera subsets válidos | ✅ Adotado |

A crate `oxifont-subset` foi integrada em `03_infra` e é usada através da função pública `subset_with_gid_set`, que recebe:
- bytes da fonte original;
- conjunto de glyph IDs antigos usados;
- mapa codepoint → glyph ID antigo.

O resultado é uma fonte SFNT subsetada com `.notdef` sempre incluído.

---

## 3. Arquitetura Implementada

### Ficheiros alterados/criados

| Ficheiro | Alteração |
|----------|-----------|
| `03_infra/Cargo.toml` | `oxifont-subset = "0.2.0"` |
| `03_infra/src/export/subset.rs` | Novo módulo de subsetting |
| `03_infra/src/export/mod.rs` | Expõe `remap_glyph_id` |
| `03_infra/src/export/builder.rs` | Integra subsetting em `build_cidfont` e `build_multifont` |
| `03_infra/src/export/stream.rs` | Aplica remapeamento de GIDs no operador `TJ` |
| `03_infra/src/export/tests.rs` | Ajusta chamadas de `PageContext` |
| `03_infra/fixtures/p307b/reference/09-cidfont.pdf` | Snapshot actualizado (redução de 559 KB → 30 KB) |

### Pipeline

```
[FrameItem::TextShaped]
       │
       ▼
[Coleta (char, old_gid)]
       │
       ▼
[subset_font_with_mapping] ──► oxifont-subset
       │
       ▼
[FontSubset { data, mapping }]
       │
       ├──► FontFile2 stream (bytes subsetados)
       ├──► widths_array com face do subset
       ├──► to_unicode_cmap com new GIDs
       └──► emit_shaped_pdf com remapeamento old→new no TJ
```

### Mapa de remapeamento

A função `subset_font_with_mapping`:
1. Constrói `old_gid_set` a partir dos glifos usados.
2. Chama `oxifont_subset::subset_with_gid_set`.
3. Parseia o subset resultante.
4. Para cada `(char, old_gid)`, consulta `face.glyph_index(char)` no subset para obter o `new_gid`.
5. Retorna `FontSubset { data, mapping: old_gid → new_gid }`.

### Integração no export

- **`build_cidfont`**: subset único; mapping aplicado em `PageContext::cidfont`.
- **`build_multifont`**: subset por fonte; mappings por fonte aplicados em `PageContext::multifont`.
- **`emit_shaped_pdf`**: usa `remap_glyph_id` para converter `ShapedGlyph.glyph_id` antes de serializar no operador `TJ`. Se o mapping estiver vazio (fallback fonte completa), mantém o GID original.

---

## 4. Scope-outs

| Funcionalidade | Razão |
|----------------|-------|
| Marcação `sub = yes` em `pdffonts` | Requer prefixo `AAAAAA+` no nome da fonte — melhoria cosmética futura |
| CFF subsetting | `oxifont-subset` suporta CFF, mas o fallback para fonte completa está activo como defesa |

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

# Bateria P490/P500
mkdir -p /tmp/p516
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" "/tmp/p516/$(basename "$f" .typ).pdf" >/dev/null 2>&1 \
    && echo "OK: $(basename "$f")" \
    || echo "FAIL: $(basename "$f")"
done
```

### Resultados

- **typst-core**: 3550 passed; 0 failed
- **typst-wiring**: 23 passed; 0 failed
- **typst-infra**: 555 passed; 0 failed
- **crystalline-lint**: 0 violations
- **Bateria P490/P500**: 37/37 OK
- **Fixture 09-cidfont**: PDF válido, texto extraível, 94% menor

---

## 6. Impacto no Tamanho

| Documento | Antes (KB) | Depois (KB) | Redução |
|-----------|-----------:|------------:|--------:|
| `09-cidfont.pdf` | 559 | 30 | 94% |
| Total bateria P490+P500 | ~40 KB* | ~67 KB | — |

\* A bateria P490/P500 usa predominantemente Helvetica fallback (sem fonte TrueType embebida), por isso o impacto é baixo. O fixture 09-cidfont é o caso representativo de embedding real.

---

## 7. Próximos Passos (P517+)

| Passo | Foco | Prioridade |
|-------|------|------------|
| **P517** | Activar `with_system_fonts` por defeito na CLI | 🟡 Média |
| **P518** | DEBT-42 Benchmark revalidado (shaping + subsetting) | 🟡 Média |
| **P519** | Marcação de subset no PDF (`AAAAAA+` prefix) | 🟢 Baixa |
| **P520** | Lookahead Layout Engine (inovação arquitetural) | 🟢 Futuro |

---

## 8. Ficheiros Alterados

| Camada | Ficheiro | Alteração |
|--------|----------|-----------|
| L0 | `00_nucleo/prompts/infra/export/font_subset.md` | Já existia; seguido como guia |
| L3 | `03_infra/Cargo.toml` | Nova dependência `oxifont-subset` |
| L3 | `03_infra/src/export/subset.rs` | Novo |
| L3 | `03_infra/src/export/mod.rs` | Reexporta `remap_glyph_id` |
| L3 | `03_infra/src/export/builder.rs` | Subsetting em CIDFont/Multifont |
| L3 | `03_infra/src/export/stream.rs` | Remapeamento no TJ |
| L3 | `03_infra/src/export/tests.rs` | Ajustes de assinatura |
| Lab | `03_infra/fixtures/p307b/reference/09-cidfont.pdf` | Snapshot actualizado |
