# Relatório P1453 — projeção pública de `html.frame`

## Veredito

**PASS no fragmento P1453 e 66/66 `MATCH` por composição P1452+P1453.**

P1453 substituiu somente para `html.frame` a projeção genérica de árvore SVG
P1420 por uma lente derivada do L0 P1424. Produto, L0, fixtures históricas e os
65 resultados `MATCH` P1452 permaneceram byte a byte.

## Cadeia e identidades

- baseline causal: commit `1a9998a5a3aa8801856be969b84f046bcecfce9f`,
  tree `1f45c3af8381ddc563ad35930a5e3550e67f5f10`;
- vanilla ratificado: SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino: SHA-256
  `4c55affaa6486683165d91af507b03f99e0041f18ce5a4fc062cbc6e2b2d05de`;
- P1452 protegido: SHA-256
  `769cd0feba8293a7bc030c1b511e48b9338aef30b80bafd1c0b42ffd1040b6f4`;
- resultado P1453: SHA-256
  `8a23e1b2fe77176b4f2aabfd220ae2d0a41f58e0ca792c3b4290ff0578f0ea00`;
- repetição independente: SHA-256
  `746d4eb1c44d98c83899fc9eaa277a1826d507294c583aecfb24341db1868690`.

## Calibração R0 → R1

R0 falhou somente em `rect_expected`. A causa era a fixture `fill: red`:
vanilla materializa `red` como `#ff4136`, enquanto o cristalino usa `#ff0000`.
Essa é uma divergência pública da paleta nomeada, observável antes do frame, e
não decide se `html.frame` preserva paint.

O selo R0 foi invalidado. R1 alterou apenas o estímulo para
`fill: rgb("#ff0000")`, preservando a exigência de paint exato e removendo a
dependência externa. O recorte rect passou nas três ordens antes da campanha
completa. Nenhum resultado R0 recebeu crédito. A diferença da constante `red`
fica aberta para P1454.

## Evidência funcional

Os cinco casos e seis probes verificaram:

- binding `frame`, carrier `frame(body: [x])` e diagnóstico de body ausente;
- uma ilha SVG simples, sem tag literal `frame`, com dimensões positivas,
  `viewBox` coerente, `display:block`, `overflow:visible`, transparência e
  payload visual;
- nesting geometricamente idêntico à ilha simples;
- rect 10pt×5pt com `rgb("#ff0000")` preservado bilateralmente;
- `html.elem("frame")` ainda como tag literal, sem SVG.

Paths de glyph, `defs`, IDs, `<use>` versus `<text>`, namespaces auxiliares,
ordem de atributos e bytes foram excluídos como mecânica conforme o L0.

Cada execução completa produziu 36 recibos. Não houve timeout, truncamento ou
swap; o pico observado foi 39.392 KiB de RSS. Forward, forward-repeat e reverse
produziram o mesmo vetor, repetido independentemente.

## Poder discriminatório

O gate rejeitou 18/18 mutações válidas, preservou 6/6 representações privadas
equivalentes e classificou 6/6 casos opacos como `Unknown`. Mutation score
`1.0`; `Unknown` não recebeu crédito.

## Composição

P1452 contém exatamente 65 rotas `MATCH` e uma única `DIFFERENCE`,
`html.frame`, sob os mesmos dois binários. P1453 classifica essa rota como
`Preserved` na lente pública P1424. Como nenhuma camada de produto mudou, a
composição fecha o denominador P1420 em 66/66 sob as duas projeções registradas.

## Limite

Execução sem atestação de isolamento. O resultado não afirma paridade geral de
HTML, SVG, cores, math ou Typst. Em particular, não concede crédito à constante
nomeada `red` divergente.
