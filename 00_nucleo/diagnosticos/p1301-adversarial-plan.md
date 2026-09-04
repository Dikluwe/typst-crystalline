# P1301 — plano adversarial segregado

## Regime e autoridade

Regime completo da materialização segregada Tekt. Este artefato exerce apenas o
papel adversário: recebe intenção/L0, contrato, baseline e fonte pré-candidata
congelados; produz mutantes negativos; não escolhe nem corrige a solução e não
emite selo ou veredito final.

Linguagem de atestação: **executado sem atestação de isolamento técnico**. A
separação foi aplicada por allowlist de leitura/escrita e por proibição causal,
mas o ambiente compartilhado não prova isolamento técnico. O executor foi
`/root/p1301_adversary`; o contexto herdado incluiu a missão adversarial e as
entradas canônicas indicadas pelo dono. Não foram lidos testes candidatos,
implementação candidata, output privado do oracle nem artefatos P1300 fora das
referências já incorporadas nas entradas P1301.

## Entradas congeladas verificadas

| Entrada | SHA-256 verificado |
|---|---|
| `00_nucleo/diagnosticos/p1301-manifest.json` | `074fabeda17f741f064fe1d561ad6a78ad81e45654861825586664a9646f05a8` |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a` |
| `00_nucleo/diagnosticos/p1301-contract.json` | `2d8d7a141d63c13473681abdefa24ed9370e03925824a9cae74ca26305ea85f5` |
| `00_nucleo/diagnosticos/p1301-pre-gate-measurement.json` | `1202c06723e948f5586e10345c5f653f57b075e3ea41297f996fd5a27abb77cd` |
| `01_core/src/compiler/eval/bindings/field_access.rs` pré-candidata | `7873e47635ad2e99df9132bae9b13472581f5026a3f3a671f2f48de8a3b15497` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

A fonte pré-candidata só foi aberta depois de o hash integral coincidir com a
allowlist. O binário vanilla foi identificado pelo hash, mas não precisou ser
executado: os observáveis necessários já estão congelados no contrato e na
medição canônica. O conjunto materializado está em
`00_nucleo/diagnosticos/p1301-mutants.json`, SHA-256
`ee3c6c4d19e410850e56e26a984a8d53662056f8a4cba39da746fdb56c2ed18e`.

## Modelo binding-free

Cada mutante é um envelope sintético de observação pública, não um patch Rust.
O envelope-base é a expectativa do mesmo `case_id` no contrato congelado. Para
cada witness listado, o sealer substitui o envelope-base por
`mutated_envelope` e compara exatamente `exit_code`, bytes de `stdout`, classe,
mensagem, hints ordenados, `span_bytes`, bytes de `rendered_stderr` e
`public_kind` quando aplicável. Casos não substituídos mantêm o envelope
esperado. O primeiro observável divergente é a testemunha pública de
`Violated`.

Não há dependência de símbolo Rust, identidade de pointer, nome de storage,
controle de fluxo interno, binding do candidato, teste candidato ou blacklist
implementável. `Unknown` não mata mutante; em caso obrigatório, é sobrevivente
e falha do gate. Os dois opacos preservam somente os `Unknown` e reason codes já
declarados no contrato. `repr(std)` permanece completamente fora de execução,
comparação e score.

## Operadores e witnesses

| Mutante | Ataque nominal | Witness público decisivo | Primeiro observável |
|---|---|---|---|
| `M01-OLD-MESSAGE` | mensagem antiga | `N-STD-default-hsl`, `N-CALC-nope` | `message` |
| `M02-WHOLE-MODULE-SPAN` | span total do acesso Module | `N-COLOR-MAP-nope` | `span_bytes` |
| `M03-STD-NAME` | publica `std`, não `global` | `N-STD-a11y-hsv` | `message` |
| `M04-THREE-ALIAS-WHITELIST` | corrige só `hsl`/`hsv`/`linear_rgb` | `N-CALC-nope`, `N-SYM-nope`, `N-COLOR-MAP-nope` | `message` |
| `M05-BREAK-EXISTING-LOOKUP` | quebra lookup Module válido | `P-CALC-abs` | `exit_code` |
| `M06-GENERALIZE-NONMODULE-SPAN` | aplica span de field a dictionary | `S-NONMODULE-dictionary-whole-span` | `span_bytes` |
| `M07-ADD-HINT` | acrescenta hint | `N-SYM-nope` | `hints` |
| `M08-ZERO-EXIT-ON-ERROR` | erro termina com zero | `N-STD-default-hsl` | `exit_code` |
| `M09-POLLUTE-STDOUT` | erro escreve em stdout | `N-COLOR-MAP-nope` | `stdout` |
| `M10-DROP-HTML-WARNING` | apaga warning HTML | `N-STD-html-hsl`, `N-STD-html+a11y-linear_rgb` | `rendered_stderr` |
| `M11-DEFAULT-PROFILE-ONLY` | corrige somente default | um witness em cada perfil não-default | `message` |
| `M12-GLOBAL-FOR-EVERY-MODULE` | publica `global` para todo Module | `N-CALC-nope`, `N-SYM-nope`, `N-COLOR-MAP-nope` | `message` |

Os 12 operadores nominais bastam para as classes pedidas; não foram
acrescentados ataques, portanto o contrato não foi ampliado nem reinterpretado.
O score esperado por desenho é `12/12 = 1.0`, com zero sobreviventes, mas essa
é uma expectativa a ser materializada pelo sealer independente, não um
resultado já executado.

## Ordem normal e invertida

O sealer deve executar cada mutante nas duas ordens integrais e comparar mapas
por `case_id`, não por posição.

Ordem normal:

```text
N-STD-default-hsl, N-STD-default-hsv, N-STD-default-linear_rgb,
N-STD-html-hsl, N-STD-html-hsv, N-STD-html-linear_rgb,
N-STD-a11y-hsl, N-STD-a11y-hsv, N-STD-a11y-linear_rgb,
N-STD-html+a11y-hsl, N-STD-html+a11y-hsv, N-STD-html+a11y-linear_rgb,
N-CALC-nope, N-SYM-nope, N-COLOR-MAP-nope,
P-STD-rgb, P-CALC-abs, P-SYM-arrow, P-COLOR-MAP-turbo,
P-COLOR-hsl, P-COLOR-hsv, P-COLOR-linear-rgb,
S-NONMODULE-dictionary-whole-span
```

Ordem invertida:

```text
S-NONMODULE-dictionary-whole-span,
P-COLOR-linear-rgb, P-COLOR-hsv, P-COLOR-hsl,
P-COLOR-MAP-turbo, P-SYM-arrow, P-CALC-abs, P-STD-rgb,
N-COLOR-MAP-nope, N-SYM-nope, N-CALC-nope,
N-STD-html+a11y-linear_rgb, N-STD-html+a11y-hsv, N-STD-html+a11y-hsl,
N-STD-a11y-linear_rgb, N-STD-a11y-hsv, N-STD-a11y-hsl,
N-STD-html-linear_rgb, N-STD-html-hsv, N-STD-html-hsl,
N-STD-default-linear_rgb, N-STD-default-hsv, N-STD-default-hsl
```

Aceitação de ordem: vetor de vereditos e mapa de observáveis idênticos nas duas
ordens; cada mutante tem ao menos um `Violated` decidível em ambas; nenhum caso
obrigatório é `Unknown`.

## Anti-erasure

O gate completo mantém oito sentinelas em todos os mutantes, salvo quando o
próprio operador declara atacar exatamente uma delas:

- sucesso e kind: `P-STD-rgb`, `P-CALC-abs`, `P-SYM-arrow`,
  `P-COLOR-MAP-turbo`;
- rotas qualificadas de cor: `P-COLOR-hsl`, `P-COLOR-hsv`,
  `P-COLOR-linear-rgb`;
- comportamento não-Module: `S-NONMODULE-dictionary-whole-span`.

`M05` é morto por alterar deliberadamente um lookup válido, mas os outros
positivos continuam preservados; `M06` é morto pela sentinela de dictionary,
mas os casos Module continuam preservados. Os controles `calc`, `sym` e `map`
impedem uma whitelist dos três aliases. Os quatro perfis impedem reparo
default-only. A exigência de stdout vazio, exit 1, hints vazios e warning HTML
impede que uma mensagem aparentemente correta apague observáveis laterais.

## Procedimento para o sealer

1. Revalidar os hashes do manifesto, L0s, contrato, medição, baseline e suite
   de mutantes antes de qualquer gate.
2. Construir o mapa-base dos 23 casos obrigatórios a partir do contrato.
3. Para um mutante, substituir somente os witnesses explícitos pelos envelopes
   mutados e executar a função pública de veredito. Um único `Violated` mata;
   `Unknown` sobre mutante não mata.
4. Executar os 23 casos normais, os 23 invertidos e os dois controles opacos.
   Comparar os mapas por id e confirmar anti-erasure.
5. Registrar, por mutante, witness, primeiro observável diferente e reason code
   público. Calcular `mutation_score = mortos / válidos`.
6. Só selar com 12 válidos, 12 mortos, score `1.0`, zero sobreviventes, opacos
   exatamente `Unknown` e ausência completa de `repr(std)`.

## Custo, rendimento e limitações

Esta é a revisão adversarial 1. Foram materializados 12 mutantes nominais e
23 witnesses obrigatórios permanecem no corpus congelado, com duas ordens e
dois controles opacos. Custo de execução nesta autoria: zero execuções do
candidato, zero leituras de testes candidatos, zero leituras de output privado
do oracle, zero novas sondas vanilla e zero corridas do corpus completo. A base
foi o contrato e a medição canônica no HEAD registrado pelo manifesto,
`1f082370e59939de7b57992e137a9f74bfb6758f`, com working tree não commitida.

Limitação decisiva: estes envelopes provam apenas que o desenho do contrato
tem witnesses públicos para os 12 defeitos nominais. Não são mutantes-fonte
executáveis, não medem um candidato e não atestam equivalência funcional geral.
O sealer independente ainda precisa materializar o gate, registrar score,
ordem, custo e qualquer sobrevivente. Duas revisões consecutivas sem ganho no
vetor de classificação exigem revisão arquitetural; nunca conversão de
`Unknown` em sucesso.
