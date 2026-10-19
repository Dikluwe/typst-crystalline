# Passo 1042 — Corrigir padding duplicado e `&` em `cases`/`matrix`/`vec`

**Tipo**: Investigar (já feito, ver achado) → gate (ADR-0127, categoria 2 — muda
posicionamento visual) → corrigir.
**Base**: `00_nucleo/diagnosticos/typst-achado-math-cases-gutter-width.md`, investigação
completa e fechada — dois mecanismos isolados com prova numérica, âmbito confirmado por
varredura exaustiva (11 construtos, só `cases`/`matrix`/`vec` afectados).
**Aplicar também**: emenda a `auditar-fatiamento.md` sobre proveniência de constantes
geométricas — os L0s corrigidos por este passo têm de citar o campo real da fonte
(`delim_gap` ou equivalente), não outro número solto.
**Pré-condição**: `git status` limpo.

---

## Mecanismo A — padding duplicado

**Causa**: o glifo do delimitador esticado (`layout_stretchy_delimiter`) já inclui a
margem lateral nas suas próprias métricas de avanço horizontal (OpenType MATH). A soma
manual de `padding = style.size * 0.1` depois do delimitador duplica essa margem.

**Correcção**:
- `cases.rs` — remover `padding = style.size * 0.1` após `left_box.width`.
- `matrix.rs` — remover `padding = style.size * 0.1` dos dois lados (afecta `mat` e,
  por herança, `vec`).

**Antes de remover**: confirmar por leitura do vanilla (`resolve.rs`/`glyph.rs`, mesmos
ficheiros já usados no P1026) que o avanço do glifo esticado é mesmo suficiente sozinho,
sem margem adicional nenhuma — não presumir que "remover a soma" é a correcção completa
sem essa confirmação directa. Citar `file:line` no L0.

## Mecanismo B — `&` em `cases` vira coluna de grelha em vez de ponto de alinhamento

**Causa**: `cases.rs` delega directamente para `layout_grid_rows` sem passar por
`align_boundaries`/`split_cell_on_align_point` (que `matrix.rs` já usa correctamente).
Cada `&` extra em `cases` é tratado como coluna nova de grelha (`col_gap` completo,
5.50pt), quando devia ser um ponto de alinhamento dentro da mesma célula lógica
(espaçamento de símbolo natural, ~2.2pt no vanilla).

**Correcção**: `cases.rs` passa a usar o mesmo mecanismo `align_boundaries`/
`split_cell_on_align_point` que `matrix.rs` já tem — reaproveitar, não duplicar
implementação.

## `mat(delim: #none)`

Suprimir delimitadores e padding quando `delim: #none` — confirmar que a correcção do
Mecanismo A não quebra este caso (sem delimitador, não deve haver padding nenhum a
remover, mas confirmar por teste, não por inspecção).

---

## Fase A — L0 primeiro, com proveniência (emenda aplicada)

Reescrever `matrix.md` e `cases.md`:
- Remover a fórmula `padding = style.size * 0.1` apresentada como regra.
- Citar o campo real da fonte de onde o avanço do delimitador vem (`file:line` do
  vanilla), ou marcar explicitamente como aproximação a confirmar se não for possível
  obter o campo exacto agora.
- Documentar o mecanismo `align_boundaries` partilhado entre `cases`/`matrix`.

## Fase B — Critérios de verificação

```
Dado cases(1, 2) — 1 coluna, sem &
Quando renderizado
Então largura bate com vanilla, delta 0.00pt (repetir para 1/2/3/4 linhas, guarda directa
  dos números já medidos no achado)

Dado cases(1 & "if" & x > 0) — com &
Quando renderizado
Então & funciona como ponto de alinhamento (espaçamento de símbolo natural), não como
  nova coluna de grelha — largura bate com vanilla

Dado mat(1, 2; 3, 4)
Quando renderizado
Então largura bate com vanilla, delta 0.00pt

Dado vec(1, 2)
Quando renderizado
Então largura bate com vanilla, delta 0.00pt (herda a correcção de matrix.rs)

Dado mat(delim: #none)
Quando renderizado
Então sem delimitador nem padding, comportamento correcto preservado
```

Não-regressão: os 8 construtos já confirmados a 0.00pt no achado (`lr`, `delimited`,
`frac`, `root`, `underover`, `accent`, `cancel`) — confirmar que continuam inalterados,
já que não deviam ser tocados por este passo.

## Fase C — Implementar e validar

```
crystalline-lint .
cargo test --workspace
```
Decalque contra `00_nucleo/corpus-docs/math/` (`cases.typ`, `mat.typ`, `vec.typ`) e o
corpus canónico.

---

## Resultado esperado

`cases`/`matrix`/`vec` com largura idêntica ao vanilla, `&` em `cases` tratado como ponto
de alinhamento (mecanismo partilhado com `matrix`, não duplicado). L0s corrigidos com
proveniência real das constantes, não números soltos. Os 8 construtos já correctos
permanecem inalterados.
