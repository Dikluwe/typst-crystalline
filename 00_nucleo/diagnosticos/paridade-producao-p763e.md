# Diagnóstico P763e — Reprodução metodológica do AE=10725 em `cetz`

**Data da medição:** 2026-07-15T18:39:20-03:00  
**Commit base:** `6541ae1d6bd9b2b561adb1d63bbd95b275e0f41a`  
**Working tree:** limpo (sem alterações não commitadas de código)  
**Passo:** P763e  
**Objectivo:** Verificar se a conclusão de P763d (AE=0, bug resolvido) é válida, ou se foi produzida por um erro metodológico de medição.

---

## Passo 0 — Sanity-check da ferramenta `compare`

Para descartar que a ferramenta de medição esteja a devolver zeros espúrios:

```bash
convert -size 200x200 xc:white /tmp/p763e-branco.png
convert -size 200x200 xc:black /tmp/p763e-preto.png
```

| Comparação | AE esperado | AE obtido |
|------------|-------------|-----------|
| branco vs preto | > 0 | **40000** |
| branco vs branco | 0 | **0** |

A ferramenta funciona como esperado.

---

## Passo 1 — Comparação directa de PDF vs PNG rasterizado

P763d comparou os PDFs directamente com `compare -metric AE`. P763c usou `mutool draw -r 300` para rasterizar antes de comparar. Testámos ambos os métodos sobre os mesmos PDFs gerados por P763d:

```bash
# PDFs gerados por P763d para o documento cetz line+circle
/tmp/p763d-completo-vanilla.pdf
/tmp/p763d-completo-cristalino.pdf
```

| Método | AE |
|--------|-----|
| `compare` directo sobre PDF | **0** |
| `mutool draw -r 300` → `compare` sobre PNG | **10261** |

A comparação directa de PDF dá AE=0; a rasterização a 300 ppp seguida de `compare` dá AE≈10 k, no mesmo patamar de P763c (10725). **O método de medição explica a discrepância.** O método correcto é o de P763c (rasterizar primeiro), não o de P763d (comparar PDF directamente).

---

## Passo 2 — Git diff entre P763c e P763d

Intervalo considerado: `dee903868c9c7d6b63e8fbc47257e5082f7638f0` (P763c/P763b-P766) até `547ad10a711bdebf49e1355e7b4cebdd110cd891` (P763c).

```bash
git log --oneline dee903868..547ad10a
```

Resultado: apenas o próprio commit de P763c.

```bash
git diff --stat dee903868..547ad10a
```

Resultado: apenas ficheiros de documentação (`paridade-producao-p763c.md`, `typst-passo-763c.md`).

**Não houve alterações de código entre a medição original de P763c e o ponto em que P763d mediu.** A explicação "P762 corrigiu isto" é cronologicamente impossível para a regressão observada, porque P762 já estava commitado quando P763c mediu o bug.

---

## Passo 3 — Reprodução exacta do documento de P763c

Documento byte-a-byte idêntico ao de P763c:

`/tmp/p763e-cetz-original.typ`:

```typst
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Pipeline exacto de P763c:

```bash
lab/typst-original/target/release/typst compile /tmp/p763e-cetz-original.typ /tmp/p763e-vanilla.pdf
./target/release/typst /tmp/p763e-cetz-original.typ /tmp/p763e-cristalino.pdf
mutool draw -o /tmp/p763e-vanilla.png -r 300 /tmp/p763e-vanilla.pdf
mutool draw -o /tmp/p763e-cristalino.png -r 300 /tmp/p763e-cristalino.pdf
compare -metric AE /tmp/p763e-vanilla.png /tmp/p763e-cristalino.png -highlight-color red -lowlight-color none /tmp/p763e-diffmap.png
```

**Resultado: AE = 10725** — reproduz exactamente o valor de P763c.

---

## Passo 4 — Coordenadas exactas via `mutool trace`

### Vanilla

```text
stroke_path transform="1 0 0 1 99.2126 70.86615"
  moveto x="0" y="28.346457"
  lineto x="56.692914" y="0"

stroke_path transform="1 0 0 1 70.86614 70.86615"
  moveto x="28.346457" y="0"
  ... (círculo, raio 28.346457)
```

Coordenadas absolutas (Y cresce para baixo):
- Linha: `(99.21, 99.21)` → `(155.91, 70.87)`
- Círculo centro: `(99.21, 99.21)` — coincide com o ponto inicial da linha.

### Cristalino

```text
stroke_path transform="1 0 0 -1 0 841.89"
  moveto x="99.21" y="681.64"
  lineto x="155.9" y="709.99"

stroke_path transform="1 0 0 -1 0 841.89"
  moveto x="99.21" y="709.99"
  ... (círculo, raio ~28.35)
```

Convertendo o eixo Y do cristalino (`y_user = 841.89 - y_pdf`):
- Linha: `(99.21, 160.25)` → `(155.90, 131.90)`
- Círculo centro: `(99.21, 131.90)` — coincide com o ponto final da linha.

### Diferença confirmada

- A linha cristalina está deslocada **~61 pt para baixo** em relação ao vanilla.
- O círculo cristalino está deslocado **~32,7 pt para baixo**.
- No vanilla, o círculo coincide com o **ponto inicial** da linha; no cristalino, o círculo coincide com o **ponto final** da linha. O eixo Y dentro do canvas está invertido.
- O cristalino não aplica a translação local do canvas que o vanilla usa (`transform="1 0 0 1 tx ty"`), usando apenas um flip Y global.

Estas diferenças são idênticas às reportadas em P763c, confirmando que o bug é real e persistente.

---

## Conclusão

A conclusão de P763d ("bug corrigido, AE=0") é **falsa**, produzida por um erro metodológico: comparar PDFs directamente com `compare -metric AE` em vez de rasterizar primeiro com `mutool draw -r 300`. Quando se aplica o método correcto, o bug reproduz-se com **AE = 10725**, exactamente como em P763c.

Adicionalmente, não houve alterações de código entre P763c e o ponto de medição de P763d que pudessem ter corrigido o problema. A explicação "provavelmente P762" não é suportada pela cronologia nem pelo diff.

**O bug real permanece:** divergência de transformação de coordenadas no renderizador de paths do cristalino, com flip Y global, ausência de translação local do canvas e inversão do eixo Y dentro do canvas do `cetz`.

---

## Validação

- `cargo test --workspace` — verde (4150 + 637 + 33 + 2 + 27 + 2 passed; 0 failed).
- `crystalline-lint .` — zero violações (apenas V7 esperado de `package_version_resolution.md`).

---

## Próximo passo

Reabrir a correcção como **P763f**, agora com a certeza de que o bug é real e que P763d mediu de forma inconsistente. Recomenda-se também rever se outros relatórios recentes usaram comparação directa de PDF em vez de rasterização, antes de aceitar as suas métricas de AE sem verificação adicional.
