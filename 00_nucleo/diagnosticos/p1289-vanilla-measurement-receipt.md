# P1289 — recibo de medição bilateral pré-candidata

**Estado:** `MEASURED_FORWARD_REVERSE`  
**Regime:** Tekt A/B, sem atestação de isolamento forte do checkout.  
**Janela:** `2026-08-31T10:35:47-03:00`–`2026-08-31T10:40:57-03:00`  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`  
**Árvore:** working tree não commitida e compartilhada.

## Identidades

| Entrada | SHA-256 |
|---|---|
| Passo 1289 | `156f223ce214eb13055dfd3ad0f7d9d45c25ac24e6c1f7379a9412c4077339fb` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| cristalino pré-candidato `/tmp/p1289-target-pre/debug/typst` | `e7c81e3a6c6f1933db95c2249286fca19f3b5991eb8bc392f49ea659cec0bd78` |
| fonte vanilla `foundations/float.rs` | `8e5c3d84b0263d6217e0f3d3d73dd114127f87f0d913a40eb57b5be6df74b8b2` |
| L0 lookup pré-P1289 | `97122927d00c44fac5039b1ae45fc1705e6f6a4743fa5d31e8a7eedbe7e2e649` |
| consumer lookup pré-P1289 | `f60bb28e09be3cd34ced92cc8e6939c4954635f002555e78554e0c4d8cb6b5b6` |

No snapshot final da medição, `git diff HEAD --stat` registrou `130 files
changed, 648091 insertions(+), 1812 deletions(-)`. O SHA-256 da saída foi
`9f20eec210f78e2213c9e15a52a46271346015921ebbaed40256a814c5bb116c` e
o de `git status --short` foi
`7e4416837195fb9f025d49fb54c23ac16580a6ea89aa735fe134d6837e524684`.

## Observáveis repetidos em ordem direta e inversa

| Caso | Vanilla | Cristalino pré-candidato |
|---|---|---|
| presença/repr estático | `(function, "is-infinite")` | field ausente |
| estático `0.0, 42.5, +inf, -inf, nan` | `false,false,true,true,false` | field ausente |
| ligado, mesmos valores | `false,false,true,true,false` | método ausente |
| estático com `0` inteiro | `false` | field ausente |
| sem `self` | `missing argument: self` | field ausente |
| positional extra | `unexpected argument` | field/método ausente |
| named `foo:` | `unexpected argument: foo` | field/método ausente |
| `"x"` como `self` | `expected float, found string` | field ausente |
| acesso ligado sem chamada | `cannot access fields on type float` | mesmo sentido, span distinto |

As duas ordens foram byte-idênticas por caso em cada lado; não houve parser
opaco nem identidade ambígua. A disponibilidade é bilateral e o estado inicial
é `Violated`, não `Unknown`.

## Separação linguagem/mecânica

A semântica observável é a função unária, sua superfície estática, a chamada
ligada e os diagnósticos. A macro `#[func]`, o scope gerado e a estrutura do
upstream são mecânica. A decisão cristalina usa owner numérico próprio e glues
fechados, sem copiar a macro.

## Auditoria de pré-condições e invalidação do selo v1

Depois do primeiro GREEN parcial, o runner revelou que os casos compostos com
`float.inf`/`float.nan` falhavam antes de chegar a `is-infinite`. A inspeção
pontual do inventário P1284 confirmou ambos como `MISSING_MEMBER`; portanto,
eles não podem servir de pré-condições para demonstrar este único ganho.

Medição bilateral adicional, repetível com `typst eval --format json`:

| Expressão | Vanilla | Cristalino pré-candidato |
|---|---|---|
| `float("1e999")` | valor infinito (serializa `null`) | idem |
| `float("NaN")` | valor NaN (serializa `null`) | idem |
| `float.is-infinite(float("1e999"))` | `true` | field sob teste ausente |
| `float.is-infinite(float("NaN"))` | `false` | field sob teste ausente |
| formas ligadas dos dois carriers | `true,false` | método sob teste ausente |

Decisão medida: o contrato v2 substitui somente os carriers compostos, sem
alterar resultados, aridade, diagnósticos ou mutações. O selo v1 e seus testes
ficam invalidados desde a primeira entrada afetada; `float.inf`/`float.nan`
permanecem fora de escopo para preservar o critério `+1 MATCH`.
