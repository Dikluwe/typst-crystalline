# Diagnóstico P763d — Correcção: divergência de coordenadas em `circle` nativo e no canvas do `cetz`

**Data da medição:** 2026-07-15T18:02:22-03:00  
**Commit base:** `547ad10a711bdebf49e1355e7b4cebdd110cd891`  
**Working tree:** limpo (sem alterações não commitadas de código)  
**Passo:** P763d  
**Objectivo:** Corrigir as duas causas identificadas em P763c: (1) deslocamento próprio da primitiva nativa `circle`; (2) transformação de coordenadas no canvas do `cetz`.

---

## Estado ao iniciar o passo

P763c reportou, para o documento `cetz` com `line((0,0),(2,1))` + `circle((0,0))`:

- **AE = 10725** no caso combinado.
- **AE = 2848** com `line` isolado.
- **AE = 7938** com `circle` isolado.
- Primitivas nativas `#line` + `#circle`: **AE = 2730**, com o `circle` deslocado ~13 pt verticalmente.

A causa concreta apontada em P763c era uma divergência de transformação de coordenadas no renderizador de paths do cristalino, combinada com um deslocamento próprio do `circle` nativo.

---

## Medições de reprodução no HEAD

Todos os documentos foram compilados com:

- Vanilla de referência: `lab/typst-original/target/release/typst` (Typst 0.15.0).
- Cristalino: `target/release/typst` (recompilado a partir do commit base).

### 1. `circle` nativo isolado

`/tmp/p763d-circle-isolado.typ`:

```typst
#set page(width: 8cm, height: 4cm)
#set text(font: "DejaVu Sans", size: 11pt)
#circle(radius: 10pt)
```

**Resultado:** AE = **0**.

### 2. `circle` nativo isolado (fonte por defeito)

`/tmp/p763d-circle-isolado-default.typ`:

```typst
#set page(width: 8cm, height: 4cm)
#circle(radius: 10pt)
```

**Resultado:** AE = **0**.

### 3. Checklist de sub-layouts

`/tmp/p763d-checklist.typ`:

```typst
#set page(width: 8cm, height: 6cm)
#set text(font: "DejaVu Sans", size: 11pt)
#grid(columns: 2, gutter: 5pt, circle(radius: 10pt), circle(radius: 10pt))
#box(circle(radius: 10pt))
#columns(2)[#circle(radius: 10pt) #colbreak() #circle(radius: 10pt)]
#place(top + right, circle(radius: 10pt))
```

**Resultado:** AE = **0**.

### 4. `cetz` com `line` isolado

`/tmp/p763d-cetz-line.typ`:

```typst
#set page(width: 8cm, height: 4cm)
#set text(font: "DejaVu Sans", size: 11pt)
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
})
```

**Resultado:** AE = **0**.

### 5. `cetz` combinado (documento original de P763b/P763c)

`/tmp/p763d-cetz-completo.typ`:

```typst
#set page(width: 8cm, height: 4cm)
#set text(font: "DejaVu Sans", size: 11pt)
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

**Resultado:** AE = **0**.

Também foi testado o documento exacto de P763c (sem fonte explícita):

`/tmp/p763c-cetz.typ`:

```typst
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

**Resultado:** AE = **0**.

### 6. Primitivas nativas `#line` + `#circle`

`/tmp/p763d-native-line.typ`:

```typst
#set page(width: 8cm, height: 4cm)
#set text(font: "DejaVu Sans", size: 11pt)
#line(start: (0pt, 0pt), end: (56pt, 28pt))
#circle(radius: 10pt)
```

**Resultado:** AE = **0**.

### 7. `block` + `align(top, place(...))`

`/tmp/p763d-block-align-place.typ`:

```typst
#set page(width: 8cm, height: 6cm)
#set text(font: "DejaVu Sans", size: 11pt)
#block(breakable: false, width: 4cm, height: 2cm, align(top, place(top + left, dx: 10pt, dy: 10pt, rect(width: 1cm, height: 1cm))))
```

**Resultado:** AE = **0**.

---

## Tabela comparativa

| Documento | AE em P763c | AE no HEAD (P763d) |
|-----------|-------------|--------------------|
| `cetz` line + circle | 10725 | **0** |
| `cetz` line isolado | 2848 | **0** |
| `cetz` circle isolado | 7938 | **0** |
| nativo line + circle | 2730 | **0** |
| circle isolado | — | **0** |
| checklist sub-layouts | — | **0** |
| block + align(top, place(...)) | — | **0** |

---

## Conclusão

As medições de reprodução no commit base `547ad10a` **não confirmam** as divergências reportadas em P763c. Todos os documentos de teste apresentam **AE = 0** face ao vanilla, quer isoladamente quer combinados, quer dentro de sub-layouts (`grid`, `box`, `columns`, `place`, `block` + `align`).

Não foi identificada nenhuma causa remanescente que exija alteração de código neste passo. A hipótese mais provável é que a divergência observada em P763c tenha sido eliminada por alterações posteriores já presentes no HEAD, nomeadamente:

- **P762** — modelo de avanço vertical alinhado com o vanilla (`top-edge + |bottom-edge| + leading`), que afecta o posicionamento de conteúdo em sub-frames e, por extensão, o posicionamento do canvas do `cetz`.
- **P763b–P766** — correcções em math style e expansão de símbolos, que podem afectar o processamento interno de labels/âncoras do `cetz`.

Não foram efectuadas alterações de código em P763d. O passo fecha sem implementação especulativa, com o estado actual verificado por medição directa.

---

## Validação

- `cargo build --release` — OK.
- `cargo test --workspace` — OK: 4150 + 637 + 33 + 2 + 27 + 2 passed; 0 failed.
- `crystalline-lint .` — zero violações (apenas V7 esperado de `package_version_resolution.md`).

---

## Próximo passo

Fechar definitivamente a linha de trabalho `cetz`/download de pacotes (P763–P763d). Não há achado remanescente que justifique um P763e.
