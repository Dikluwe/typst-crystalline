# P1342 — verificação independente de pré-selo R1

Regime: **executado sem atestacao de isolamento**. Papel: verificador
independente de pré-selo. Veredito: **REJECTED_NOT_SEALED**.

## Decisão

A substância do gate focal passou: os hashes protegidos pós-freeze estão
intactos, os dez owners L0 e dez consumers coincidem com o freeze, V5/V15/V26
estão verdes, as 19 linhas do manifesto R2 resolvem nos símbolos/branches
candidate-free reais sem `cfg(test)` decoy, a fixture e todos os ranges do
parser foram reproduzidos, e o checker R3 classificou os 45 casos corretamente
em `normal`, `repeat` e `reverse`: 43 negativos `Violated`, P01 `Preserved`, P02
`Unknown`, score `1.0`, agreement verdadeiro e zero sobreviventes.

O selo, contudo, é rejeitado por um blocker processual objetivo. O budget
congelado permite **uma** execução completa do corpus antes do selo. Este
verificador executou o corpus completo duas vezes: a primeira terminou com exit
0, mas o relatório redirecionado para `/dev/shm` não persistiu entre chamadas;
a segunda terminou com exit 0 e produziu o resumo verificável acima. A perda do
artefato temporário não apaga a primeira execução. Resultado: 2 executadas para
1 autorizada.

Portanto, o resultado substantivo não pode ser promovido a
`SEALED_FOR_IMPLEMENTATION`. É necessária autorização humana explícita para
aceitar o overrun e reabrir ou substituir o gate. Não executar uma terceira
passagem sob o budget atual.

## Entradas, hashes e cadeia protegida

Pins decisivos confirmados:

| entrada | SHA-256 |
|---|---|
| `typst-passo-1342.md` | `5b7322e7223b2320d961c164e0b603c10d5f390030cd671d962592af2ff92d48` |
| `p1342-authority-manifest.json` | `d247e6714faf3249109beb3cfb2ef718a4d2049bb56c95aaabd270eb6eed87a4` |
| `p1342-l0-freeze-r1.json` | `2fb962c3edd8cdd83848d2cd7c9158c39a5d0218f510e81bb5e8011a540e8c0e` |
| `p1342-contract-fixture-r1.typ` | `98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714` |
| `p1342-contract-spec-r2.json` | `62d656241ecd96cd3d984025e59990d607a0efd75c3c326503c7ec07a3486c4d` |
| `p1342-contract-binding-r2.json` | `3b522cf1b0f288fe1e472f21f775089541835ede754250a14a3c2d17c17e9d48` |
| `p1342-oracle-checker-r3.py` | `047dd04c811fd7672143bdc253b759450d2f61d548cc3b49dd3fbcbce71791b7` |
| `p1342-oracle-corpus-r3.json` | `872b906a0665b933d1ca30337a44849e579257f3eb331f33e034a8a2c5ed6160` |

O inventário recursivo conferiu 217 ocorrências de pins em 46 paths. Houve 215
matches atuais. As duas diferenças são fotografias históricas pré-Fase-B do
manifesto de autoridade para `context_stabilization.rs` (`1e42852f...`) e
`eval/mod.rs` (`6f885d48...`); ambas foram sucedidas antes do freeze pelo resselo
L0 documentado, respectivamente `2f03c9e5...` e `8706947c...`. Não são drift
pós-freeze: os 10 hashes L0 e os 10 hashes de consumers do freeze coincidem
integralmente agora, e o checker R3 confirmou a cadeia transitive de inputs
protegidos.

Estado da medição em `2026-09-10T22:08:46Z`: HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitada;
SHA-256 de `git status --porcelain=v1 -z` antes destes outputs
`c51953ca4887da406c4929af7dc49e70b201b23066eca5291f605dd1cc471be6`;
SHA-256 de `git diff --binary HEAD`
`ff4dfff0c1401e48855b4116c5e11ce895e4c21fafc9977d8af5c218f6338a86`;
stat: 72 ficheiros, 10881 inserções e 830 remoções.

## Manifesto R2 e topologia candidate-free

O manifesto é closed-world e contém exatamente 19 rows. H00D e H00S são
mutuamente exclusivos, logo exatamente 18 rows aplicam-se por célula de origem.
Para todas as 19 rows foram confirmados hash do L0, hash do consumer, linha
baseline, símbolo, branch e needle. Nenhuma âncora está dentro de módulo
`#[cfg(test)]`; nenhum decoy de teste foi aceito. H01 e H06 têm texto repetido
no ficheiro, mas resolvem uma vez no símbolo declarado; H16 possui duas
ocorrências file-wide e uma única em `Session::stabilize`.

Os dez consumers congelados contêm zero marcadores P1342 e permanecem
candidate-free. As cláusulas L0 vigentes autorizam somente observação privada
sob cfg, preservam API, defaults, compatibilidade, resultados e fase do
pipeline. Não há mudança ADR-0127 nesta fase de pré-selo.

## Fixture, parser, ranges e Dict

O parser candidate-free foi executado diretamente com
`typst_core::entities::source::Source` e `LinkedNode` sobre os 178 bytes exatos,
somente LF e com LF final. A raiz foi `Markup 0..178`, sem nó `Error`. Foram
reproduzidos: Closure `47..121`, CodeBlock `64..121`, Dict witness `82..97`,
Contextual `123..177`, ContentBlock `131..177`, chamada update `136..164`, Args
`144..164`, chamada With `145..163` e Dict prebound `155..162`.

Os valores são produzíveis pela linguagem e preservam tipo/ordem: prebound
`[(a, Int(1))]`; witness `[(outer, Dict([(a, Int(1))]))]`. A compilação integral
da fixture candidate-free terminou com exit 0.

## Gates executados

```text
cargo build -p typst-core
=> exit 0 (warnings existentes; build concluído)

<probe Rust via stdin> | rustc ... && /dev/shm/p1342_parser
=> exit 0; root/ranges acima; zero Error

target/release/typst compile p1342-contract-fixture-r1.typ /dev/shm/p1342-contract-fixture-r1.pdf
=> exit 0

crystalline-lint . --checks v5,v15,v26
=> exit 0; No violations found

git diff --check
=> exit 0
```

Execução completa 1 do checker R3:

```text
python3 -B p1342-oracle-checker-r3.py --contract p1342-contract-spec-r2.json \
  --manifest p1342-contract-binding-r2.json --corpus p1342-oracle-corpus-r3.json \
  > /dev/shm/p1342-preseal-r3-report.json
=> exit 0; output temporário não retido
```

Execução completa 2, por pipe para extrator read-only:

```text
mesmo checker/contrato/manifesto/corpus | extrator de resumo
=> exit 0
=> normal=45, repeat=45, reverse=45
=> negativos=43/43 Violated; P01=Preserved; P02=Unknown
=> mutation_score=1.0; agreement=true; survivors=[]
```

Assim, A01–A21 fecham todos os blockers R1 e X01–X22 fecham todos os blockers
R2. O único blocker restante é o próprio excesso de execução do gate.

## Limites e próximo poder de decisão

Nenhum candidato futuro P1342 nem artefato de teste futuro foi lido. Nenhum
input julgado foi modificado; não foram escritos código, testes, L0, contrato,
oráculo ou adversário. Esta verificação não fecha lifecycle/profile P1340,
NT01–NT06, retenção/descarte/invalidação, política terminal, aceitação de
implementação ou equivalência funcional geral.

Blocker: `BUDGET_PRESEAL_FULL_RUNS_EXCEEDED`. Resolução necessária: autorização
humana explícita para aceitar o overrun e reabrir/substituir o gate. Até essa
decisão, o veredito permanece **REJECTED_NOT_SEALED**.
