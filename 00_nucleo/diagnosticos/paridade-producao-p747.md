# P747 — Investigar o deslocamento vertical de ~11pt entre cristalino e vanilla

**Data:** 2026-07-14 (09:15 -03:00)
**Commit base:** `19579db56` ("P746: preenche hash do commit no relatorio")
**Commit da sonda:** `PREENCHER_APOS_COMMIT`
**Estado no momento das medições:** working tree limpa; nenhum ficheiro de código alterado.

Vanilla de referência: `lab/typst-original/target/release/typst`
(typst 0.15.0, 969087ec).

---

## Contexto

P746 encontrou um deslocamento vertical absoluto de ~11pt nas coordenadas da
linha entre cristalino e vanilla, e classificou-o rapidamente como "escolha de
layout". P747 investiga se esse deslocamento é real, visualmente detectável, e
qual a sua causa concreta.

Documento de teste (`/tmp/p747-nativo.typ`):

```typst
#line(start: (0pt, 0pt), end: (100pt, 50pt))
#circle(radius: 30pt)
```

---

## Sonda

### 1. Confirmação visual

Imagens rasterizadas a 150 dpi:

| | Cristalino | Vanilla |
|---|---|---|
| Ficheiro | `/tmp/p747-cristalino.png` | `/tmp/p747-vanilla.png` |
| Pixels não-brancos | 1903 | 1897 |

Inspecção visual: as duas imagens parecem quase idênticas à primeira vista, mas
um mapa de diferenças (`/tmp/p747-diffmap.png`) revela claramente duas linhas
e dois círculos ligeiramente deslocados. O deslocamento é real, mas pequeno
(~23 px a 150 dpi).

### 2. Diff de pixels deste documento específico

```text
A: 1241x1754 ch=3  B: 1241x1754 ch=3
não-brancos A=1897  B=1903
diff (>8): 10683 / 6530142 bytes (0.1636%)
```

O diff percentual (0.1636%) é da mesma ordem do documento `cetz` de P745
(~0,14–0,15%), indicando que não há um deslocamento massivo nem erro de
renderização — o padrão continua a ser de diferenças finas nas fronteiras.

### 3. MediaBox e tamanho de página

| | Cristalino | Vanilla |
|---|---|---|
| MediaBox | `[0 0 595.28 841.89]` | `[0 0 595.2756 841.8898]` |

As páginas são praticamente idênticas em tamanho; a diferença de ~0.004 pt na
largura e ~0.0002 pt na altura não explica o deslocamento de ~11 pt.

### 4. Medições de posicionamento no content stream

Documento combinado:

| Elemento | Cristalino (PDF y) | Vanilla (PDF y) | Diferença |
|---|---|---|---|
| Topo da linha | 759.990 | 771.0236 | −11.0336 pt (cristalino abaixo) |
| Centro do círculo | 679.990 | 677.8236 | +2.1664 pt (cristalino abaixo) |

Bounding boxes das imagens rasterizadas:

| | Cristalino | Vanilla | Diferença |
|---|---|---|---|
| BBOX | (146, 169, 356, 400) | (146, 146, 356, 405) | dy = +23 px = 11.04 pt |

A medição visual confirma o deslocamento de ~11 pt para o conjunto dos
elementos.

### 5. Testes com elementos isolados

Para isolar a causa, geraram-se PDFs separados com apenas a linha e apenas o
círculo.

**Só a linha:**

| | Cristalino | Vanilla |
|---|---|---|
| Topo da linha | 759.990 | 771.0236 |
| Diferença | −11.0336 pt | |

**Só o círculo:**

| | Cristalino | Vanilla |
|---|---|---|
| Centro do círculo | 729.990 | 771.0236 − 30 = 741.0236? |

Vanilla círculo isolado: transformação `q 1 0 0 -1 70.86614 771.0236 cm`, círculo
centrado em (30,30) local → centro PDF = (100.86614, 741.0236).

| | Cristalino | Vanilla | Diferença |
|---|---|---|---|
| Centro do círculo isolado | 729.990 | 741.0236 | −11.0336 pt |

Quando isolados, **tanto a linha como o círculo têm exactamente o mesmo
deslocamento vertical de −11.0336 pt no cristalino**. A diferença aparente no
documento combinado (onde o círculo só difere 2.17 pt) é resultado do
**espaçamento vertical entre elementos**.

### 6. Espaçamento entre elementos

Documento combinado:

| | Cristalino | Vanilla |
|---|---|---|
| Posição y do primeiro elemento | 759.990 (linha) | 771.0236 (linha) |
| Posição y do segundo elemento | 679.990 (círculo) | 677.8236 (círculo) |
| Distância entre topo da linha e topo do círculo | 50.000 pt | 63.200 pt |

O cristalino coloca o segundo elemento exactamente 50 pt abaixo do primeiro
(altura da linha, sem espaçamento extra). O vanilla coloca o segundo elemento
63.2 pt abaixo do primeiro, adicionando ~13.2 pt de espaçamento vertical entre
elementos.

---

## Decisão

O deslocamento de ~11pt é **real**, mas **não é um bug de precisão
geométrica**. A causa concreta é uma diferença no **algoritmo de layout
vertical**:

1. **Margem/posição inicial do primeiro elemento:** o vanilla começa a colocar
   conteúdo 11.0336 pt mais acima na página do que o cristalino. No vanilla, a
   posição y=771.0236 corresponde a uma distância do topo de ~70.87 pt (= 25
   mm), consistente com a margem padrão do Typst. O cristalino posiciona o
   primeiro elemento em y=759.990, correspondendo a ~81.90 pt do topo — uma
   diferença de ~11 pt.

2. **Espaçamento entre elementos:** o vanilla adiciona ~13.2 pt de espaçamento
   extra entre a linha e o círculo, enquanto o cristalino os coloca em
   contacto direto (distância = altura da linha = 50 pt). Estas duas diferenças
   quase compensam-se para o círculo no documento combinado, mas não para a
   linha.

As formas geométricas em si (comprimento/inclinação da linha, raio e
aproximação do círculo) são correctas nos dois lados. A divergência é de
**layout/posicionamento**, ou seja, de mecânica (ADR-0107), não de linguagem.

Não foi identificado nenhum consumidor em `cetz` nem teste de paridade que
dependa deste posicionamento absoluto na página; a cadeia `cetz` continuou sem
regressão mensurável. Portanto, este deslocamento não é tratado como bug a
corriger neste momento, mas sim registado como divergência mecânica conhecida.

---

## Validação global

- Nenhum ficheiro de código alterado.
- `crystalline-lint .`: 0 violations.
- Testes anteriores continuam válidos.

---

## Proveniência

- Commit base: `19579db56`
- Hora das medições: 2026-07-14T09:15-03:00
- Binário vanilla: `lab/typst-original/target/release/typst`
- Binário cristalino: `./target/release/typst`
- Ferramenta de inspecção: `mutool show`, `mutool draw`
- Script de comparação: `/tmp/pngdiff.py`
- Script de mapa visual: `/tmp/pngdiff_map.py`
