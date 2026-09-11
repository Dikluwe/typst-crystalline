# P1342 — verificação independente de pré-selo R2

Regime: **executado sem atestacao de isolamento**. Papel: verificador
independente de pré-selo. Veredito: **SEALED_FOR_IMPLEMENTATION**.

## Decisão substituta

Este parecer preserva integralmente o parecer R1 e todos os inputs julgados. Não
houve nova execução de checker, corpus, build ou lint. A decisão usa
exclusivamente as evidências já medidas e registradas no R1.

O R1 concluiu que a substância do gate focal passou sem blocker técnico: cadeia
protegida pós-freeze intacta; dez owners L0 e dez consumers coincidentes com o
freeze; V5/V15/V26 verdes; manifesto R2 fechado com 19 rows, 18 aplicáveis por
célula, âncoras reais e nenhum `cfg(test)` decoy; fixture, parser, ranges e Dict
reproduzidos; e checker R3 com 45/45 casos corretos em `normal`, `repeat` e
`reverse`, incluindo 43/43 negativos `Violated`, P01 `Preserved`, P02 `Unknown`,
score `1.0`, agreement verdadeiro e zero sobreviventes. A01–A21 e X01–X22
ficaram fechados.

O único blocker do R1 era processual: duas execuções completas, ambas exit 0,
para budget congelado de uma execução. O humano autorizou explicitamente nesta
mensagem aceitar esse overrun de 2 para 1 e prosseguir sem nova execução do
corpus. A autorização constitui waiver limitado a
`BUDGET_PRESEAL_FULL_RUNS_EXCEEDED`; não altera os inputs, não converte falha
técnica em aprovação, não autoriza terceira passagem e não muda budgets futuros.

Como não existe outro blocker registrado no R1, o gate focal fica
**SEALED_FOR_IMPLEMENTATION**.

## Evidência herdada e pins completos

| entrada/evidência | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1342.md` | `5b7322e7223b2320d961c164e0b603c10d5f390030cd671d962592af2ff92d48` |
| `00_nucleo/diagnosticos/p1342-l0-freeze-r1.json` | `2fb962c3edd8cdd83848d2cd7c9158c39a5d0218f510e81bb5e8011a540e8c0e` |
| `00_nucleo/diagnosticos/p1342-contract-spec-r2.json` | `62d656241ecd96cd3d984025e59990d607a0efd75c3c326503c7ec07a3486c4d` |
| `00_nucleo/diagnosticos/p1342-contract-binding-r2.json` | `3b522cf1b0f288fe1e472f21f775089541835ede754250a14a3c2d17c17e9d48` |
| `00_nucleo/diagnosticos/p1342-oracle-checker-r3.py` | `047dd04c811fd7672143bdc253b759450d2f61d548cc3b49dd3fbcbce71791b7` |
| `00_nucleo/diagnosticos/p1342-oracle-corpus-r3.json` | `872b906a0665b933d1ca30337a44849e579257f3eb331f33e034a8a2c5ed6160` |
| `00_nucleo/diagnosticos/p1342-contract-fixture-r1.typ` | `98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714` |
| `00_nucleo/diagnosticos/p1342-verifier-preseal-r1.md` | `8373ebdec40c09b005f78266d0f0b133462b29c45d5551501c84fc02c8a1a53b` |
| `00_nucleo/diagnosticos/p1342-verifier-preseal-r1.json` | `700546a17638e7b5326559b49a03422e1a4db4f83ef72d94f01b722e04d0c6da` |

O freeze pinado contém os dez pares owner L0/consumer cuja correspondência foi
medida no R1 como 10/10 hashes L0 e 10/10 hashes de consumers, com zero marcador
P1342 nos consumers candidate-free. Este R2 não remede esses ficheiros; herda a
medição imutavelmente identificada pelos hashes do R1 acima.

## Resultados R1 incorporados sem reexecução

```text
crystalline-lint . --checks v5,v15,v26
=> exit 0; No violations found

cargo build -p typst-core
=> exit 0

probe parser candidate-free + compilação integral da fixture
=> exit 0; 178 bytes; LF-only; LF final; Markup 0..178; zero Error

checker R3, execução completa 1
=> exit 0; relatório temporário não retido

checker R3, execução completa 2
=> exit 0
=> normal=45, repeat=45, reverse=45
=> negativos=43/43 Violated; P01=Preserved; P02=Unknown
=> mutation_score=1.0; agreement=true; survivors=[]
```

O estado de proveniência dessas medições permanece o registrado no R1:
`2026-09-10T22:08:46Z`, HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitada,
SHA-256 de `git status --porcelain=v1 -z`
`c51953ca4887da406c4929af7dc49e70b201b23066eca5291f605dd1cc471be6` e
SHA-256 de `git diff --binary HEAD`
`ff4dfff0c1401e48855b4116c5e11ce895e4c21fafc9977d8af5c218f6338a86`.

## Limites do waiver e do selo

O waiver cobre somente o excesso histórico de uma execução completa no pré-selo
R1. Não autoriza nova execução sob este gate e não modifica o protocolo
congelado. Nenhum candidato futuro P1342 nem artefato de teste futuro foi lido.
Nenhum input foi modificado; não foram escritos código, testes, L0, contrato,
oráculo ou adversário.

O selo é apenas para implementação do contrato focal pinado. Não fecha
lifecycle/profile P1340, NT01–NT06, retenção/descarte/invalidação, política
terminal, aceitação posterior da implementação ou equivalência funcional geral.
Não há mudança de API pública, comportamento por defeito, fase do pipeline ou
compatibilidade no escopo autorizado pelo pré-selo.

