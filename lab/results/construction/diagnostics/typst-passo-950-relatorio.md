# Passo 950 — Relatório (`BaseFont`/`FontName` com nome real da fonte)

**Data**: 2026-08-01
**Estado da árvore**: commit base `932f11690` (P949); alterações deste passo por cima.

---

## 1. Fase A — o que estava genérico, e porquê

Dois caminhos de emissão de fonte em `03_infra/src/export/builder.rs`:

- **Cidfont** (single-font, linha ~957): `base_font_name = "CrystallineFont"` (ou
  `AAAAAA+CrystallineFont` com subset).
- **Multifont** (linha ~1439): `base_name = "CrystallineFont{N}"` (idem com prefixo).

Os três sítios que consomem o nome — `/BaseFont` do Type0, `/BaseFont` do CIDFont e
`/FontName` do FontDescriptor — usam a mesma variável, logo uma correcção só cobre tudo.
O **nome do recurso interno** (`/F1` no dicionário `/Resources` e no content stream) é
independente e **não** foi tocado (decisão do passo: não é o problema).

O nome real está disponível nos bytes da fonte embutida (tabela `name` — PostScript,
name_id 6), dispensando canal novo de dados do `FontBook`.

## 2. Fase B — implementação (TDD directo)

- Helper novo `real_base_name(font_data, fallback)` em `builder.rs`: lê o nome real da
  tabela `name` (prioridade PostScript 6 → full name 4 → família 1), remove espaços
  (convenção BaseFont), cai no `fallback` (o genérico anterior) se a leitura falhar.
  **Prefixo determinístico `AAAAAA+` de P517 mantido** (reprodutibilidade de build —
  o prefixo aleatório do vanilla foi deliberadamente não adoptado).
- Teste novo `p950_basefont_usa_nome_real_das_fontes` (red → green): compila documento
  com corpo + math, extrai todos os `/BaseFont` do PDF e asserta: nenhum genérico, ≥2
  nomes reais distintos, família `NewCM` presente.
- Canários actualizados (o marcador CIDFont deixou de ser o literal `CrystallineFont`):
  `font_wiring_set_text_font_existente_embute_cidfont`,
  `font_wiring_array_fallback_primeira_falha_segunda_vence`, o canário de ligadura
  `fi` (agora asserta `LibertinusSerif`) e o canário de nome canónico single-font
  (agora asserta a ausência do genérico + presença de `/BaseFont`).
- Snapshot `p307b_09_cidfont` regenerado (mudança intencional de output;
  `UPDATE_P307B_SNAPSHOTS=1`).

**Prova directa** (`pikepdf` sobre o documento de 30 secções recompilado):

```
/F1 /AAAAAA+NewCM10-Bold
/F2 /AAAAAA+NewCMMath-Book
/F3 /AAAAAA+LibertinusSerif-Regular
/F4 /AAAAAA+NewCM10-Regular
```

Mesmos nomes reais que o vanilla embute (com o nosso prefixo determinístico).

## 3. Validação

- `cargo test --workspace`: **todas as 9 suites verdes** (4821 + 750 + 41 + 2 + 37 + 2,
  incluindo o novo teste P950 e os canários actualizados).
- `crystalline-lint .`: zero violations (`--fix-hashes` resselou `builder.rs` +
  `bitmap_glyphs.rs`; resta só o V7 pré-existente alheio).
- Grep por `CrystallineFont` no código/testes: só restam o fallback interno do helper
  (por desígnio), comentários e as asserções negativas (que continuam verdadeiras).
- L0: `00_nucleo/prompts/infra/export/builder.md` §P950.

## 4. Benchmark

(hyperfine, warmup 1, min 10 runs; "antes" = binário de `932f11690` compilado em
worktree; "depois" = working tree P950; corpus canónico; JSONs em
`tools/perf/results/p950-*.json`)

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 87.14 | 87.94 | 1.009 |
| 02-lorem | 106.65 | 107.25 | 1.006 |
| 03-images | 96.98 | 95.58 | 0.986 |
| 04-math | 118.63 | 119.58 | 1.008 |
| 05-tables | 91.22 | 92.47 | 1.014 |
| 06-long | 285.39 | 288.82 | 1.012 |
| 07-context | 127.91 | 127.74 | 0.999 |

Ratio médio **1.005** — zero regressão (mudança de nome, sem custo de performance).
