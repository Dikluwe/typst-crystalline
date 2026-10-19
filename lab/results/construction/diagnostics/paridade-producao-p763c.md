# Diagnóstico P763c — Investigação directa: AE=10725 em `cetz` (line + circle)

**Data da medição:** 2026-07-15T17:19:00-03:00  
**Commit base:** `dee903868c9c7d6b63e8fbc47257e5082f7638f0`  
**Working tree:** limpo (sem alterações não commitadas)  
**Passo:** P763c  
**Objectivo:** Não aceitar a conclusão de P763b ("diferença mecânica de renderização") sem medição directa; investigar a causa concreta de AE=10725 no documento `cetz` com `line` + `circle`.

---

## Documento de teste

`/tmp/p763c-cetz.typ`:

```typst
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Vanilla de referência: `lab/typst-original/target/release/typst` (Typst 0.15.0).  
Cristalino: `target/release/typst`.

---

## Passo 0 — Reconciliação com P762

O relatório P762 inclui um teste de regressão com `cetz` cujo documento é **idêntico** ao de P763b/P763c (`line((0,0),(2,1))` + `circle((0,0))`). No entanto, P762 **não** reporta uma medição AE para esse documento — apenas regista que compila sem erro (2387 bytes). O handoff de P762 afirma genericamente que `cetz` "acabou por renderizar com paridade de pixels exacta (AE=0) no caso de teste final", mas o relatório P762 não identifica qual caso concreto obteve AE=0, nem fornece medição para o documento `line`+`circle`.

**Conclusão:** P763b introduziu uma medição nova para este documento específico. Não é possível classificar AE=10725 como regressão de um baseline AE=0 documentado; é uma divergência recém-medida.

---

## Passo 1 — Mapa de diferenças

```bash
mutool draw -o /tmp/p763c-vanilla.png -r 300 /tmp/p763c-cetz-vanilla.pdf
mutool draw -o /tmp/p763c-cristalino.png -r 300 /tmp/p763c-cetz-cristalino.pdf
compare -metric AE /tmp/p763c-vanilla.png /tmp/p763c-cristalino.png -highlight-color red -lowlight-color none /tmp/p763c-diffmap-highlight.png
```

**Resultado:** AE = **10725** (reproduzido; idêntico a P763b).

A inspecção visual de `/tmp/p763c-diffmap-highlight.png` mostra os dois objetos (`line` + `circle`) deslocados verticalmente em bloco, mantendo a forma e o traço. Não é diferença difusa de anti-aliasing nem diferença de espessura: é um **deslocamento vertical concentrado e uniforme** do grupo desenhado pelo canvas.

---

## Passo 2 — Coordenadas exactas (PDF operators)

```bash
mutool trace /tmp/p763c-cetz-vanilla.pdf > /tmp/p763c-trace-vanilla.txt
mutool trace /tmp/p763c-cetz-cristalino.pdf > /tmp/p763c-trace-cristalino.txt
```

### Vanilla

```text
stroke_path transform="1 0 0 1 99.2126 70.86615"
  moveto x="0" y="28.346457"
  lineto x="56.692914" y="0"

stroke_path transform="1 0 0 1 70.86614 70.86615"
  moveto x="28.346457" y="0"
  ... (circle radius 28.346457)
```

Coordenadas absolutas estimadas (sistema do vanilla, Y cresce para baixo):
- Linha: `(99.21, 99.21)` → `(155.91, 70.87)`
- Círculo centro: `(99.21, 99.21)`

### Cristalino

```text
stroke_path transform="1 0 0 -1 0 841.89"
  moveto x="99.21" y="681.64"
  lineto x="155.9" y="709.99"

stroke_path transform="1 0 0 -1 0 841.89"
  moveto x="99.21" y="709.99"
  ... (circle radius ~28.35)
```

Convertendo o eixo Y do cristalino (`y_user = 841.89 - y_pdf`):
- Linha: `(99.21, 160.25)` → `(155.90, 131.90)`
- Círculo centro: `(99.21, 131.90)`

### Diferença observada

- A linha cristalina está deslocada **~61 pt para baixo** em relação ao vanilla.
- O círculo cristalino está deslocado **~32,7 pt para baixo**.
- No vanilla, o círculo coincide com o **ponto inicial** da linha; no cristalino, o círculo coincide com o **ponto final** da linha. Isto indica que o eixo Y dentro do canvas do `cetz` está a ser invertido no cristalino.
- Adicionalmente, o cristalino **não aplica a translação do canvas** observada no vanilla (`transform="1 0 0 1 70.86614 70.86615"` para `line` e `70.86614 70.86615` para `circle`). Em vez disso, usa apenas um flip Y global (`1 0 0 -1 0 841.89`) e coordenadas absolutas.

---

## Passo 3 — Isolamento da variável

### (a) `cetz` com `line` isolado

AE = **2848**. A linha sozinha já apresenta deslocamento vertical semelhante.

### (b) `cetz` com `circle` isolado

AE = **7938**. O círculo sozinho também apresenta deslocamento.

### (c) Primitivas nativas `#line` + `#circle`

Documento `/tmp/p763c-native-line.typ`:

```typst
#set page(width: 8cm, height: 4cm)
#line(start: (0pt, 0pt), end: (56pt, 28pt))
#circle(radius: 10pt)
```

AE = **2730**. O mapa de diferenças mostra a **linha praticamente coincidente** (cinza), mas o **círculo nativo deslocado verticalmente**.

Trace:
- Vanilla `circle`: centro absoluto `(23.50, 64.70)`.
- Cristalino `circle`: centro absoluto convertido `(23.50, 51.49)`.
- **ΔY ≈ 13,2 pt** no círculo nativo.

### Interpretação do isolamento

- A primitiva nativa `line` do cristalino posiciona-se correctamente quando convertida para o mesmo sistema de coordenadas.
- A primitiva nativa `circle` do cristalino **já está deslocada verticalmente** (~13 pt no teste nativo).
- O caso `cetz` amplifica esse desvio porque o canvas do `cetz` depende de transformações de coordenadas que o cristalino aplica de forma diferente do vanilla (flip Y global + ausência da translação do canvas + possível inversão interna do eixo Y).

---

## Causa concreta identificada

A diferença de AE=10725 **não é anti-aliasing nem ruído de rasterização**. É uma **divergência de transformação de coordenadas no renderizador de paths** do cristalino:

1. O cristalino usa um sistema de coordenadas PDF com **flip Y global** (`1 0 0 -1 0 H`), enquanto o vanilla usa transformações locais com Y crescente para baixo (`1 0 0 1 tx ty`).
2. O cristalino **não aplica a translação do canvas do `cetz`** que o vanilla aplica (`tx = ty ≈ 70.87 pt`).
3. A primitiva nativa `circle` do cristalino tem um **deslocamento vertical próprio** (~13 pt no teste isolado), independente do `cetz`.
4. No canvas do `cetz`, o eixo Y interno parece estar **invertido** no cristalino: o círculo desenhado em `(0,0)` aparece no ponto final da linha em vez do ponto inicial.

---

## Conclusão

- A explicação de P763b ("diferença mecânica de renderização") foi **investigada e refutada como mero anti-aliasing**. A causa é real e localiza-se no renderizador de paths do cristalino.
- O problema afeta tanto primitivas nativas (`circle`) como pacotes de desenho (`cetz`), sendo amplificado no segundo caso pela transformação do canvas.
- **Próximo passo recomendado:** abrir P763d para correcção específica da transformação de coordenadas no renderizador de paths, com validação AE antes/depois nos mesmos documentos (`cetz` line+circle, `circle` nativo isolado).
- A linha de trabalho de download de pacotes (P763–P763b) continua fechada: a aquisição automática de `cetz` e `oxifmt` funciona; a divergência visual é preexistente e causada pelo renderizador, não pelo download.

---

## Validação

- `cargo test --workspace` — OK.
- `crystalline-lint .` — zero violações (exceto V7 esperado de `package_version_resolution.md`).
- Medições reproduzíveis em cache preenchida.
