# P1233 — retificação corrigida de P1229

**Veredito:** `ACCEPTED_DIAGNOSTIC`; estado `P1229_REOPENED`.

## Medição antes da decisão

A reexecução saneada ocorreu em `2026-08-27T21:47:14-03:00`, HEAD
`697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`, working tree não commitada.
O snapshot exato imediatamente anterior à execução, incluindo a lista de
paths, `git diff HEAD --stat`, HEAD e horário, está em
`p1233-execution-provenance.txt`, SHA-256
`18318146ffe54f7aef927d2532c754229af7072e22f8f6ce90168d42760ba636`.
O vanilla foi pinado em `a51e02804`; `/usr/local/bin/typst` possuía SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

O novo runner `lab/parity/matrix/p1233_rectification.py` tem SHA-256
`e9f1d704e515c2534585683ffc6f6671c02960997b8c1aa236619980f5b2f7a1`.
Ele usa `p1233-r01-allowlist.tsv`, SHA-256
`04c063add7ae1ebd72df4ad721b2b4a066f57e0750785db0256a28a5c19d5003`,
cujo único campo é `id`. Nenhuma coluna de resultado anterior é recebida. Os
limites são derivados exclusivamente das colunas vanilla `V_*`; nenhum
candidato novo foi compilado.

### F01 — join de tuple semântico

O join inclui variant, espaço, stops, offsets, posição de alpha, geometria,
papel fill/stroke e caixa. Resultado: `0` tuples concretos idênticos entre os
10 registos reduzidos P1229 e os 30 controles R01. Receipt:
`p1233-f01-f02-join.tsv`, SHA-256
`38cb6a5d83fc1aa235b3fcf14698dbcd83c0dc051d318eb671d82c5ea320a92b`.

### F02 — escopo normativo

Os 30 tuples foram validados integralmente contra as dimensões obrigatórias em
P1229: variante/espaço, fill/stroke, dois ou três stops, offsets explícitos
ordenados, coincidência, alpha em stop e geometria declarada. Caixa e aspect
ratio são cobertura mecânica do harness, não construção Typst nova. Resultado:
`30` extensões de cobertura normativa, `0` dimensões desconhecidas e `0`
expansões de linguagem. Receipt `p1233-f01-f02-join.tsv`, SHA-256
`38cb6a5d83fc1aa235b3fcf14698dbcd83c0dc051d318eb671d82c5ea320a92b`.

### Oráculo 2048/4096

O oráculo compara `Gradient.sample(t)` público do vanilla com a função linear
dos stops SVG, em sRGB codificado premultiplicado e alpha separado. O adapter
P1233 parseia `stop-opacity` exatamente uma vez. As duas execuções completas
foram byte-idênticas. Receipt de 60 linhas:
`p1233-mesh-2048-4096.tsv`, SHA-256
`261065fbf5499588a36fcc707cd532de138ad416fd6306f38e916c4e439a26af`.

| Combinação | Passes | Falhas | Veredito |
|---|---:|---:|---|
| Linear/Oklab | 0 | 6 | `Unknown` reaberto |
| Linear/LinearRgb | 2 | 4 | `Unknown` reaberto |
| Radial/Hsv | 1 | 5 | `Unknown` reaberto |
| Linear/Radial sRGB | 12 | 0 | controle preservado |
| **Total** | **15** | **15** | `P1229_REOPENED` |

## Ataques e certificado

O runner impõe estruturalmente allowlist de uma única coluna, universo de 30
IDs únicos, identidade exata das fixtures, presença dos SVGs, duas malhas e
ordenação determinística. O inventário de ataques permanece como desenho para
uma futura cadeia segregada; os mutantes não foram reexecutados depois da
mudança causal de P1232, portanto nenhum mutation score é alegado neste passo.

O recibo `p1233-tekt-certificado.tsv` aceita somente o diagnóstico. P1229
permanece reaberto e Linear/Oklab, Linear/LinearRgb e Radial/Hsv permanecem
`Unknown`. Não houve edição de L0 nem de código L1–L4 e nenhuma mudança de
whitelist está autorizada por este passo.

Regime: reprodução diagnóstica repetida e byte-idêntica; não constitui
protocolo Tekt completo, selo segregado ou equivalência funcional geral.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`
