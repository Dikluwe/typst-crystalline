# Relatório de Paridade — Passo 622

**Data:** 2026-07-09
**Commit base dos testes:** `b87201b9b39bc151315eec84cc40ba4fd428efee`
**Hash do L0 `entities/content.md`:** `eaca2719`
**Hash do L0 `rules/eval.md`:** `d1f52ef5`
**Hash do L0 `rules/layout.md`:** `6c4613b5`

## Parte 1 — Quebras de parágrafo RTL

### Sintoma
Dois parágrafos RTL separados por uma linha em branco eram renderizados
numa única linha visual. O mesmo sintoma afectava texto latino, indicando
que não era específico de RTL.

### Causa
`01_core/src/rules/eval/mod.rs:494` mapeava tanto `SyntaxKind::Space` como
`SyntaxKind::Parbreak` para `Content::Space`, perdendo a semântica de quebra
de parágrafo.

### Correcção
Adicionada a variante `Content::Parbreak` ao enum `Content`
(`01_core/src/entities/content.rs`):

- `is_empty()` → `false` (marker estrutural).
- `plain_text()` → `"\n"`.
- `map_content` / `map_text` → terminal (clone directo).
- `fmt_content` → `"parbreak"`.
- `PartialEq` → `(Parbreak, Parbreak) => true`.

O eval de markup passou a distinguir:

- `SyntaxKind::Space` → `Content::Space`
- `SyntaxKind::Parbreak` → `Content::Parbreak`

O layout passou a tratar `Content::Parbreak` com `flush_line()` no ponto onde
ocorre, avançando o cursor verticalmente por `line_height + leading` e
separando os parágrafos visualmente.

### Sítios actualizados

| Ficheiro | Alteração |
|---|---|
| `01_core/src/entities/content.rs` | Nova variante `Parbreak`; métodos do hub; teste unitário. |
| `01_core/src/rules/eval/mod.rs` | `SyntaxKind::Parbreak` → `Content::Parbreak`. |
| `01_core/src/rules/layout/mod.rs` | Braço `Content::Parbreak` → `flush_line()`. |
| `01_core/src/rules/eval/repr.rs` | `repr_content` → `"parbreak"`. |
| `01_core/src/rules/introspect.rs` | Terminais em `materialize_time` e `walk`. |
| `01_core/src/rules/introspect/locatable.rs` | Não-locatable. |
| `01_core/src/rules/introspect/extract_payload.rs` | Teste: `extract_payload(&Parbreak) == None`. |
| `01_core/src/rules/layout/tests.rs` | Teste `parbreak_separa_dois_paragrafos_em_linhas_distintas`. |
| `03_infra/src/query_helpers.rs` | Matches `has_any_text` e `count_variant` tratam `Parbreak`. |
| `03_infra/fixtures/p307b/reference/03-text-styling.pdf` | Snapshot actualizado (o fixture tem dois parágrafos). |

### Medição visual (release build)

Input árabe (`/tmp/p622-dois-paragrafos.typ`):

```text
#set text(lang: "ar", font: "DejaVu Sans", size: 24pt)
مرحبا بالعالم

هذا فقرة ثانية منفصلة
```

Cristalino após a correcção:

| linha | top (pt) | conteúdo |
|---|---|---|
| 1 | 62.70 | ملاعلاب ابحرم |
| 2 | 97.88 | ةلصفنم ةيناث ةرقف |

Antes da correcção: apenas uma linha em `top=62.70`.

Input latino (`/tmp/p622-latim-dois-paragrafos.typ`):

```text
Primeiro parágrafo latino aqui.

Segundo parágrafo latino, separado por linha em branco.
```

Cristalino após a correcção:

| linha | top (pt) | conteúdo |
|---|---|---|
| 1 | 73.10 | Primeiro parágrafo latino aqui. |
| 2 | 89.23 | Segundo ... |

### Nota sobre parágrafos vazios
Múltiplos `Parbreak` consecutivos: o primeiro drena a linha actual;
subsequentes encontram `current_line` vazia e `flush_line()` é no-op em
termos de avanço vertical. Parágrafos vazios adicionais não são criados
nesta versão — comportamento registado como limitação conhecida.

## Parte 2 — `advance_shaped` em devanágari

### Observação
`needs_shaped_width` (`01_core/src/rules/layout/metrics.rs:150`) lista
`Arabic`, `Syriac`, `Mongolian`, `Nko` e `Mandaic`. **Devanagari não está
incluído.**

### Teste
Input longo em devanagari (`Noto Sans Devanagari`, 40pt):

```text
नमस्ते संसार यह एक लंबा वाक्य है जो पंक्ति के अंत तक पहुंच सकता है
```

| Implementação | Páginas |
|---|---|
| Cristalino | 1 |
| Vanilla | 1 |

Output visual comparado com `mutool draw` foi **idêntico** entre as duas
implementações. Não se observou a quebra prematura que o árabe apresentava
antes de P590/P591.

### Decisão
**Não adicionar devanagari a `needs_shaped_width` neste passo.** A ausência
de divergência visual indica que a largura monoespaçada já é suficiente para
o devanagari neste input, provavelmente porque as ligaduras/contextual forms
do devanagari não reduzem a largura total da mesma forma que as ligaduras
árabes. Fica registado para reavaliação se surgir caso de regressão.

## Validação

```bash
cargo test --workspace   # ok — 4250+ tests passados
crystalline-lint .       # ✓ No violations found
```

## Regressões e notas

- Snapshot `03-text-styling.pdf` actualizado porque o fixture contém dois
  parágrafos separados por linha em branco; a mudança de output é a própria
  correcção.
- Testes de P621 (tracking) não foram explicitamente reexecutados como suite
  isolada, mas `cargo test --workspace` cobre todos os testes do workspace,
  incluindo os de tracking.
