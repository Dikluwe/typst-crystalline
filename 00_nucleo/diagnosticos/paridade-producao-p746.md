# P746 — As coordenadas do PDF são idênticas, ou há diferença de precisão por trás do "anti-aliasing"?

**Data:** 2026-07-14 (09:05 -03:00)
**Commit base:** `83ca2f4a4` ("P745: preenche hash do commit no relatorio")
**Commit da sonda:** `PREENCHER_APOS_COMMIT`
**Estado no momento das medições:** working tree limpa; nenhum ficheiro de código alterado.

Vanilla de referência: `lab/typst-original/target/release/typst`
(typst 0.15.0, 969087ec).

---

## Contexto

P745 confirmou que o padrão do diff residual (~0,14%) é consistente com
anti-aliasing. Este passo vai mais fundo: compara directamente as coordenadas
dos content streams dos PDFs para saber se a causa é só o rasterizador a
variar, ou se os dois PDFs têm geometria vectorial diferente.

Documento de teste (`/tmp/p746-nativo.typ`):

```typst
#line(start: (0pt, 0pt), end: (100pt, 50pt))
#circle(radius: 30pt)
```

---

## Sonda

### Content streams extraídos

Cristalino (objecto 4):

```text
q
0.000 0.000 0.000 RG
1.00 w
70.867 759.990 m
170.867 709.990 l
S
Q
q
0.000 0.000 0.000 RG
1.00 w
100.867 709.990 m
117.435 709.990 130.867 696.559 130.867 679.990 c
130.867 663.422 117.435 649.990 100.867 649.990 c
84.298 649.990 70.867 663.422 70.867 679.990 c
70.867 696.559 84.298 709.990 100.867 709.990 c
S
Q
```

Vanilla (objecto 7):

```text
/Artifact<</Type/Layout>>BDC
q 1 0 0 -1 70.86614 771.0236 cm/c0 CS 0 SCN 4 M 0 0 m 100 50 l
S
Q
EMC/Artifact<</Type/Layout>>BDC
q 1 0 0 -1 70.86614 707.8236 cm/c0 CS 0 SCN 4 M 0 30 m 0 13.44648 13.44648 0 30
 0 c 46.55352 0 60 13.44648 60 30 c 60 46.55352 46.55352 60 30 60 c 13.44648 60
 0 46.55352 0 30 c
S
Q
EMC
```

### Linha

| Propriedade | Cristalino | Vanilla (após transformação) |
|---|---|---|
| Coordenadas PDF | (70.867, 759.990) → (170.867, 709.990) | (70.86614, 771.0236) → (170.86614, 721.0236) |
| Δx / Δy | 100.000 / −50.000 | 100.000 / −50.000 |
| Comprimento | 111.803399 pt | 111.803399 pt |
| Inclinação | −26.5651° | −26.5651° |

A **geometria da linha é idêntica**; só difere a posição vertical na página
(~11 pt mais alta no vanilla). Esta diferença é de layout/posicionamento, não
de precisão geométrica.

### Círculo

| Propriedade | Cristalino | Vanilla (após transformação) |
|---|---|---|
| Centro PDF | (100.867, 679.990) | (100.86614, 677.8236) |
| Raio | 30 pt | 30 pt |
| Segmentos de Bézier | 4 | 4 |

Ambos usam 4 curvas cúbicas de Bézier para aproximar o círculo, mas a
**constante de aproximação difere**:

| | Valor de k | Erro vs. k teórico `(4/3)(√2−1)` |
|---|---|---|
| Teórico | 0.5522847498 | — |
| Cristalino | 0.5522666667 | 0.000018 |
| Vanilla | 0.5517840000 | 0.000501 |

O cristalino está mais próximo da constante teórica para aproximação de
círculo por Bézier cúbica; o vanilla usa uma aproximação ligeiramente mais
mais grosseira (diferença de ~0.0005 no valor de k, que se traduz num
desvio máximo estimado de ~0.015 pt no contorno do círculo).

---

## Decisão

As coordenadas do PDF **não são idênticas**, mas as diferenças são de
dois tipos distintos:

1. **Posicionamento na página** — a linha e o círculo estão colocados em
   posições y ligeiramente diferentes (~11 pt para a linha, ~2.2 pt para o
   círculo). Isto é uma escolha de layout/meio de renderização, não uma
   questão de correcção matemática.

2. **Aproximação do círculo por Bézier** — ambos usam 4 segmentos, mas com
   constantes de controlo diferentes. O cristalino está numericamente mais
   próximo do círculo matematicamente exacto. No entanto, ambos são
   aproximações aceitáveis; a diferença (~0.015 pt no contorno) explica o
   diff residual de anti-aliasing observado em P745.

A resposta directa à pergunta "vanilla é mais correcto?" é: **não
universalmente**. Para a linha, ambos são igualmente correctos. Para a
aproximação do círculo, o cristalino está ligeiramente mais próximo do
valor teórico. Para o posicionamento na página, a pergunta não se aplica
sem um critério externo de layout.

Trata-se portanto de uma **divergência de mecânica** (estrutura interna do
PDF, escolhas de aproximação numérica e posicionamento), não de uma
divergência de linguagem (semântica/sintaxe/morfologia) — conforme
ADR-0107.

---

## Validação global

- Nenhum ficheiro de código alterado.
- `crystalline-lint .`: 0 violations.
- Testes anteriores continuam válidos.

---

## Proveniência

- Commit base: `83ca2f4a4`
- Hora das medições: 2026-07-14T09:05-03:00
- Binário vanilla: `lab/typst-original/target/release/typst`
- Binário cristalino: `./target/release/typst`
- Ferramenta de inspecção: `mutool show`
