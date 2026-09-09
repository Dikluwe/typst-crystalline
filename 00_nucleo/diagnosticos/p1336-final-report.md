# P1336 — campo ausente em instâncias integer/string

## O que mudou

`(1).nope` agora informa `cannot access fields on type integer`;
`"abc".nope` informa `cannot access fields on type string`. Nos dois casos,
o diagnóstico aponta somente para `nope`, e não para o acesso inteiro.
Aliases, parênteses, identificadores Unicode e acessos multilinha obedecem
à mesma regra. É uma correção de linguagem no diagnóstico, não de bytes de
renderização nem de representação interna dos valores.

A alteração produtiva tem dois pontos em
`01_core/src/compiler/eval/bindings/field_access.rs`: incluir Int/Str na escolha
do span do identificador e usar o nome longo no fallback dessas instâncias.
O lookup puro continua respeitando o span fornecido. Nenhum outro ramo
produtivo foi alterado, segundo a revisão independente do diff.

Resultado técnico: recorte corrigido e gates aprovados, sem fechamento de paridade
geral. O veredito e o fechamento auditável são registrados separadamente em
`p1336-review-final.json` e `p1336-closure.json`.

## Evidência do resultado

Testes e expectativas foram escritos por outro agente antes da implementação.
O RED compilou e falhou por assertions do comportamento contratado, com controles
de preservação aprovados. Após a correção, os cinco testes focais passaram.
As expectations não foram afrouxadas para acomodar o candidato.

O corpus independente contém 45 casos, quatro perfis (`default`, `html`, `a11y`,
`html+a11y`) e três ordens (`normal`, repetida e invertida). As 540 comparações
candidatas passaram por igualdade integral de exit/stdout/stderr:

- 156 confirmam convergência com o vanilla no recorte corrigido;
- 264 preservam comportamentos já coincidentes;
- 120 preservam dívidas identificadas antes da correção, **sem crédito de paridade**.

Não houve Unknown obrigatório nem instabilidade por ordem. Os controles opacos
do auditor são testes de transporte, não sucessos do produto. Recibos:
`p1336-tests-receipt.md`, `p1336-tests-comparison.json` e revisão independente
`p1336-review-channels.json`.

O workspace passou com **6.739 testes aprovados, nenhuma falha e três ignorados**.
Os controles legados de LocatedContent e de warning de `sym.join` passaram também
em execuções focais. Build release, formatação e `git diff --check` passaram.
V5/V15/V26 estritos: nenhum apontamento. O lint geral terminou sem erros, mas
não está sem avisos: os 233 warnings herdados foram preservados; surgiram dois
informes V19/V20 sobre o novo braço fechado Int/Str. Ver `p1336-review-gates.json`.

Os seis mutantes produtivos planejados atacam separadamente nomes Int/Str,
spans Int/Str, extrapolação para Bool e confusão com namespaces de tipos.
A execução ocorreu somente em workspace/target temporários exclusivos:
**seis mutantes válidos foram rejeitados**, todos com recompilação comprovada
e falha específica da propriedade atacada; o controle C passou. Nomes incorretos
falharam nos testes puro/AST correspondentes; os mutantes de span falharam somente
no AST respectivo; a extrapolação Bool falhou somente no controle de outros tipos;
o ataque aos namespaces falhou somente no controle de métodos/namespaces.
Recibo efetivo `p1336-attacks-final.json`, SHA-256
`05073308aed67b16600a1090e9480fede71d7bd77ce20855166c05d165d698f3`;
fontes, logs e executáveis individuais retidos, com UTC e identidade por rodada.

Na primeira rodada, somente M1 recompilou; M2–M6 reutilizaram o binário de M1.
Essas cinco observações foram invalidadas como **Unknown instrumental**, e não
contam como rejeições. Os recibos originais foram mantidos. A repetição focal R2
usou timestamp fresco da fonte exclusiva, recompilação comprovada e identidade
do executável; substituiu somente as cinco observações inválidas. O conjunto
efetivo tem zero Unknown obrigatório. Esse incidente é distinto do A/B CLI, que passou com binário
candidato pinado; não houve alteração no produto principal para contorná-lo.

## O que continua faltando

Este passo **não fecha a paridade geral** e não recalcula a porcentagem global
do P1335. Continuam abertos no corpus: diagnósticos de `int.nope`/`str.nope`
(valores-tipo, não instâncias), fronteiras Bool/Array/None/Auto, Content.text,
o pré-despacho de `"abc".len` sem chamada e a ordem de `panic` nos argumentos
de métodos ausentes. Métodos válidos, `type` e `repr` foram preservados.

LocatedContent e warnings têm testes legados, não cobertura bilateral adicional
nesta matriz. O ramo de leitura contextual de `text` foi preservado por inspeção
do diff; não foi criado um experimento A/B específico dessa superfície. Além da
suíte existente, não houve nova campanha dedicada de layout/PDF nem alegação de
resolver os demais eixos do P1335.

## Proveniência e reprodução

Todas as contagens acima vêm da working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, com inventários e diff/stat exatos
before/after nos recibos. O baseline P1336 conserva as alterações herdadas;
somente o par L0/source de `field_access` muda em relação a ele.

- Baseline: `p1336-baseline.json`, SHA-256
  `e81e034167a994ab3dd1c318a7dd7384cd18ef75e0d2b3c1c666eb26724e441f`.
- Vanilla ratificado upstream/main `a51e02804`: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Antecedente: `/tmp/p1335-target.EkAvyv/release/typst`, SHA-256
  `11e3164fa509030cc78dc048d5bb4f2426348e32a24edc7cd2f320c704e6ef61`.
- Candidato: `/tmp/p1336-target.QiOMGq/release/typst`, SHA-256
  `646a8d97400c0abe262a65c9b9559b47a3ecf7d10eafd82fa79a64ade0504497`.
- A/B candidato: `2026-09-09T18:20:13.266956+00:00` a
  `2026-09-09T18:20:33.092501+00:00`; fontes, argv e canais integrais retidos.
- Workspace: `2026-09-09T18:19:57.285746+00:00` a
  `2026-09-09T18:23:03.610124+00:00`, recibo `p1336-workspace-tests.json`.

Comandos dos gates, com `CARGO_TARGET_DIR=/tmp/p1336-target.QiOMGq`:

```text
cargo test -p typst-core --release --locked --lib p1336_tests
cargo build --workspace --release --locked
cargo test --workspace --release --locked
cargo fmt --all -- --check
crystalline-lint --checks v5,v15,v26 --fail-on warning .
crystalline-lint .
git diff --check
```

O runner A/B e suas entradas estão pinados em `p1336-tests-freeze.json`.
Os recibos são imutáveis: repetir a execução exige nomes de saída novos,
sem sobrescrever as observações originais.

L0 atualizado antes do código, em fluxo contínuo ADR-0127. Corpo normativo
congelado SHA-256 `46e50b3c2dc18448eab41cc55489795c56a90004b2d9ba65667915d3be9db02b`;
linhagem final A `65cbfb3c`, B `4e4810f3`. O linter não reparou B sozinho quando
A já estava correto; o metadado recíproco foi recalculado, ressellado e conferido
independentemente em `p1336-lineage-reciprocal.json`, sem mudar a norma congelada.

A skill `tekt-materializacao-segregada` determinou a separação entre autoria
da implementação, testes, ataques e veredito. Regime A/B **sem atestação técnica
de isolamento**, sem selo de refinamento. As limitações e ocorrências de processo
ficam nos recibos de calibração/revisão, não substituem a evidência funcional.
`/dev/shm` estava montado somente leitura; temporários ficaram em `/tmp`.
Nenhum staging, commit ou limpeza destrutiva foi realizado.

A revalidação de preservação conferiu 5.022 artefatos históricos e 441 exports
ativos/sucessores do P1335, sem diferenças (`p1336-preservation-before-closure.json`,
estado completo e UTC incluídos). Isso não reivindica recuperar os bytes R0 que
o P1335 já registrava como perdidos. O fechamento repete essa conferência e
registra os hashes do relatório, gates, mutantes e veredito.
