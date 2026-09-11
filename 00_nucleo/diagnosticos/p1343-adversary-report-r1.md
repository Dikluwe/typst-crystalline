# P1343 — relatório adversarial R1

## Veredito

**ADVERSARIAL_SURVIVORS_BLOCK_SEAL.** O contrato/oráculo R1 não pode ser
selado. Dez mutações negativas novas foram aceites como `Preserved`, e o
controle positivo sintético aceito pelo oráculo não é uma árvore Rust válida.

Regime: **executado sem atestacao de isolamento**. O workspace é compartilhado;
a independência usada aqui é de papel, entradas pinadas, ordem e allowlist de
escrita. O adversário não editou contrato, baseline, source verifier, checker,
corpus nem candidato produtivo.

## Proveniência reproduzível

Execução focal em `2026-09-10T21:40:24-03:00`:

- HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree não commitada; SHA-256 de
  `git status --porcelain=v1 -z --untracked-files=all`:
  `915f0afc71fcaceb32412a93484b955d3684d1b18a5841ee57080eb1cce7b721`;
- SHA-256 de `git diff --binary HEAD`:
  `f73da54bdbeb1bd086caeff259b9517bdbf0f1648efcb8119d36699f5bcf0be5`;
- stat rastreado: 72 arquivos, 10889 inserções e 830 remoções;
- temporários exclusivamente em `/dev/shm`;
- comando: `python3 -B 00_nucleo/diagnosticos/p1343-adversary-runner-r1.py`;
- execução do corpus completo: **zero**;
- runner SHA-256:
  `6064ab8d20199cd5fc0d938c97bbda4e0be1d3882a1d784b537565714595a899`;
- relatório JSON SHA-256:
  `2ceaf77378591bb16458d3e594d1c9f4ff459f6bcdfffc2f3665df8894e9ff8d`.

Todos os pins recebidos coincidiram antes do ataque. O JSON contém os paths e
hashes completos das entradas protegidas.

## Resultado focal

Foram executadas 11 provas: 10 negativos válidos e um controle de validade do
positivo. Os 10 negativos sobreviveram, portanto o score adversarial novo é
`0/10 = 0.0`. Sete dos dez arquivos do positivo sintético produziram erro de
parse no `rustfmt --edition 2021 --config skip_children=true --emit stdout`.

| Ataque | Esperado | Real | Witness mínimo |
|---|---|---|---|
| ADV01 hook H01 com payload `NOT-H01` | Violated | Preserved | source verifier emitiu PASS |
| ADV02 hook H01 só dentro de função local nunca chamada | Violated | Preserved | source verifier emitiu PASS |
| ADV03 writer faz `.push` em `scratch`, não no ledger | Violated | Preserved | source verifier emitiu PASS |
| ADV04 segundo writer usa `ledger.extend` | Violated | Preserved | `alternate_writers` continuou vazio |
| ADV05 projeção escreve com `ledger.extend` | Violated | Preserved | `projection_writes` continuou vazio |
| ADV06 constructor `Event::forged_after_execution` pós-hoc | Violated | Preserved | `posthoc_constructors` continuou vazio |
| ADV07 `raw_freeze` nominal, sem snapshot/clone | Violated | Preserved | token nominal bastou |
| ADV08 fachada vazia + refs runtime duplicadas | Violated | Preserved | 19 rows autodeclarados aceitaram a fachada vazia |
| ADV09 combinação dead/wrong hook + fake freeze + runtime único | Violated | Preserved | source verifier emitiu PASS |
| ADV10 corpus atacante de um único positivo, sem hash externo | Violated | Preserved | checker saiu 0, `agreement=true`, zero negativos |
| ADV11 positivo sintético autoral | Preserved e parseável | InvalidPositive | sete arquivos falharam parse Rust |

O JSON preserva finalidade, classificação e witness de cada prova. ADV09
demonstra que a composição de falhas não restaura a discriminação.

## Causas medidas

1. **Owner/row não são resolvidos.** Em
   `p1343-source-verifier-r1.py:775-790`, `symbol_sha256` é apenas o hash de
   `anchor_before || capsule || anchor_after`; nenhum símbolo Rust é resolvido.
   Em `:821-841`, o path/symbol do row é copiado do manifesto e a presença do
   hook não é derivada do corpo.

2. **Reachability e identidade do hook viraram busca textual.** Em
   `p1343-source-verifier-r1.py:843-877`, uma regex exige apenas uma chamada ao
   nome do writer em cada conjunto de cápsulas. Ela não inspeciona o argumento,
   a posição herdada, a branch nem se a função que contém a chamada é
   executada. Isso explica ADV01/ADV02.

3. **Writer/projeção são subaproximados por duas formas sintáticas.** O writer
   é inferido pela única ocorrência `.push(` (`:857-860`); `extend`, indexação,
   substituição e objeto receptor não são provados. A projeção só reconhece
   nomes que contenham `project|dto` e só marca `.push(` ou chamada ao writer
   (`:881-886`). As listas `alternate_writers` e `posthoc_constructors` são
   emitidas vazias sem enumeração (`:891-899`). Isso explica ADV03–ADV06.

4. **H16 é nominal.** A prova de freeze é somente a regex por uma palavra
   `freeze|raw_snapshot|raw_freeze` (`:887-890`), sem ordem causal, clone,
   imutabilidade ou ligação à projeção. Isso explica ADV07.

5. **Runtime é um envelope de strings.** Em
   `p1343-source-verifier-r1.py:687-711`, qualquer ref não vazia é aceita; não
   se exige unicidade, desafio, conteúdo, hash, frescor, evento raw ou ligação
   à fachada. Isso explica ADV08/ADV09.

6. **O corpus não é input externamente pinado.** O parser do checker declara
   `--corpus PATH`, mas não `--corpus-sha256` (`p1343-oracle-checker-r1.py:393-401`).
   Depois confia nos próprios pins embutidos no corpus recebido (`:411-434`).
   Um corpus fechado de um caso positivo preserva todos esses pins e passa.

7. **O positivo não satisfaz a própria validade exigida.** O gerador insere
   blocos genéricos em posições de campo, initializer e item e constrói ramos
   replace sem wrapper sintático adequado. O source verifier focal aceita, mas
   o parser Rust rejeita 7/10 arquivos. Logo o controle positivo não prova a
   classe `required_discrimination.positive` do contrato.

## Classes examinadas e limite de parada

A inspeção confirmou que o R1 contém controles fail-closed explícitos para
UTF-8/LF, pares/IDs, âncoras, reposição integral, cfg inicial, symlink de arquivo
allowlisted e decoys de marcador. Esses casos já estão no corpus autoral
C01–C22. O adversário não os reexecutou depois do primeiro survivor: pelo
protocolo, um survivor relevante já bloqueia o selo e a instrução de execução
proibia ampliar para uma busca aberta ou executar o corpus completo.

Isto não declara essas classes completas. Em particular, a normalização prova
corretamente o limite de bytes, mas ADV01–ADV09 mostram que ela não prova a
semântica dentro da cápsula. O scanner focal também não substitui os gates Rust,
como ADV11 torna observável.

## Hipótese adversarial e refutação

Hipótese: o R1 modelou bem a transformação inversa de bytes, porém tratou
`owner`, hook, writer, projeção, freeze e runtime como metadados ou padrões
textuais. Por isso o oracle discrimina adulteração do envelope, mas não
discrimina implementações causalmente falsas que conservam a forma lexical.

Esta hipótese seria refutada por uma revisão independente que, no mesmo
conjunto focal, classificasse ADV01–ADV10 como `Violated`, fornecesse um positivo
Rust realmente parseável como `Preserved` e mantivesse as entradas protegidas
sem converter falha em `Unknown`. Até lá, `SEALED_FOR_IMPLEMENTATION` é proibido.

## Limitações

- O resultado cobre apenas o binding focal P1343/P1342; não julga P1340,
  NT01–NT06, retenção ou política terminal.
- Não houve isolamento técnico do workspace.
- Não foi executado corpus completo, race, build do candidato ou teste A/B.
- O adversário não propõe nem escreve a correção; contrato, oráculo e solução
  permanecem autoridades separadas.
