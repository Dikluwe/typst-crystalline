# P1269 — reexecução contra o contrato P1268-v3

**Estado:** EXECUTADO  
**HEAD medido:** `7df0e3174d0d651189c9281c81c4508e3c691478`  
**Árvore:** não commitada; snapshot anterior à primeira execução candidata em `p1269-working-tree-snapshot.txt`  
**Baseline:** upstream/main `a51e02804`, binário ratificado `/usr/local/bin/typst`  
**Atestação:** segregação causal/processual; **sem atestação técnica de isolamento de leitura**

## Veredito

`OWNER-RETURN`.

P1269 não pode seguir globalmente ao P1270 nem emitir
`Generalization-Preserved`: G04A falhou em duas fixtures lineares de S14. Os
dois pares radiais ficaram apenas com falhas numéricas congeladas, mas a quebra
estrutural dos pares lineares tem precedência sobre o passo numérico.

| Par | Conjunção | G05 numérico | Falha não numérica | Classificação |
|---|---:|---:|---|---|
| linear/oklab | 18/24 | 19/24 | G04A em S14 | Owner-Return |
| radial/oklab | 17/24 | 17/24 | nenhuma | Numeric-Failures-Frozen |
| linear/linear-rgb | 17/24 | 18/24 | G04A em S14 | Owner-Return |
| radial/linear-rgb | 18/24 | 18/24 | nenhuma | Numeric-Failures-Frozen |

Não houve promoção produtiva, alteração de fallback, alegação de equivalência
SVG geral ou novo selo P1268.

## Medição antes da decisão

Os hashes do preseal P1268, contrato, observáveis, política de Unknown,
budgets, custos, landmarks, coincidências, degenerados, máscaras e ataques
foram verificados e congelados antes da primeira execução candidata. O binário
candidato permaneceu byte-idêntico (`408f1240...cf21f68`) antes e depois dos
controlos. O corpus produzido na raiz temporária teve hash
`54b5f0fd...c9522894`, idêntico ao corpus congelado; o corpus protegido não foi
escrito.

A execução cobriu 96 fixtures válidas, 28 probes inválidos e 192 reexecuções
semânticas nas ordens inversa e repetida. Os totais atuais foram `M=1320`,
`R=848`, `E=11188` e `sum(a_i)=9868`. Todos os 1.224 intervalos, dos quais 848
elegíveis, reproduziram o traço congelado e respeitaram
`s_i<=64`, `a_i<=63`, `d_i<=127`, `b_i<=126`, `q_i<=253` e
`E=M+sum(a_i)<=M+63R`.

## Falha estrutural G04A

As fixtures `S14-linear-oklab` e `S14-linear-linear-rgb` preservam a sequência
pública, a multiplicidade e os offsets, mas violam a adjudicação no ponto exato
das coincidências internas em `1/3` e `2/3`.

O probe usa o ratio público do próprio candidato, sem reserialização decimal:

```typst
g.sample(g.stops().at(i).at(1))
```

Nesses quatro pontos, o candidato seleciona o último stop coincidente. O
vanilla ratificado e o contrato P1268-v3 exigem o primeiro stop na coincidência
interna; epsilon à direita continua selecionando corretamente o último. As
variantes radiais de S14 preservaram ambas as regras. Portanto a divergência é
G04A, não diferença mecânica de carrier e não dívida numérica absorvível.

## Dívida numérica separada

G05 preservou 72/96 fixtures. Restaram 28 métricas excedidas em 24 fixtures; o
maior excesso foi `9.931466019146669e-05` em
`S06-linear-oklab/color_p95`. As identidades, hashes de fonte e métricas exatas
estão congeladas em `p1269-numeric-failure-freeze.tsv` e `p1269-numeric.tsv`.
Elas permanecem preparadas para P1270, mas não autorizam essa rota enquanto
G04A estiver aberto.

## Gates preservados

- G01, G02, G03, G04B, G06, G07A, G07B, G08, G09, G10, G11 e G13: 96/96.
- G04A: 94/96; as duas violações são S14 linear.
- G05: 72/96.
- G12, a conjunção derivada: 70/96.
- Domínio inválido: 28/28 rejeitado.
- Direto/inverso/repetido: 192/192 byte-idêntico no digest semântico.
- Grafo e geometria: 96/96.
- Produto/fallback: 96/96 sem promoção.
- Raster corrigido: 96/96. Os quatro `Violated` do campo histórico em
  `p1269-raster.tsv` são exclusivamente o falso gate P1266 que exigia área zero
  para S20; G06/G07A usam o suporte positivo congelado de 1.920 pixels.
- Ataques: 24/24 negativos válidos mortos; controle positivo preservado.
- Opacidade: 10/10 continuaram `Unknown`, excluídos do numerador e de sucesso.

Os controlos `cargo build --workspace`, `cargo test --workspace`, formatos
root/probe, sintaxe Python, `git diff --check`, V1/V5/V15/V26 e lint completo
passaram. O lint completo manteve apenas avisos informativos V16–V20
preexistentes. Os hashes protegidos de SVG, adaptive e P1264 permaneceram
idênticos.

## Limite da segregação

O mesmo executor `/root` realizou freeze, execução, adjudicação e
materialização em fases causais distintas, com allowlists e hashes. Todos os
papéis, porém, compartilharam o mesmo workspace e mantiveram capacidade técnica
de leitura. Pedir que um papel não leia reduz o contexto usado, mas não revoga
a capacidade do filesystem. Consequentemente este resultado é reproduzível e
processualmente segregado, porém não constitui atestação de isolamento técnico
ou de autoridade independente.
