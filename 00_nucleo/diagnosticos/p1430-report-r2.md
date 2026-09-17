# P1430 — relatório de verificação final R2

## Veredito

**PASS** para a implementação candidata `d3b08225a`, preservada sem
alterações produtivas até o HEAD de refinamento
`7429ad5ef384b4437621f3223b1293e3f5ff1198`.

O protocolo completo da skill `tekt-materializacao-segregada` foi
**executado sem atestação de isolamento**. A autoridade verificadora não
editou código produtivo, testes Rust, L0s nem artefatos de contrato, oráculo
ou adversário.

## Integridade da cadeia

- 34/34 entradas protegidas da campanha R2 mantiveram os hashes congelados;
- manifesto R2: `b7119a8a29786c1a8da91cd08e705e3983514062bd935c639e3ff9a8f67e7adc`;
- contrato R2: `4fe68f16f4d9af0a9fb491963736c99241c63db201d6b284f098f2d31b73f309`;
- oráculos R2: `23747cac87e1682779fd3c8f36b5348a92eae9e506e3a49fce274878fe523728`;
- mutantes R3: `b8f02c9db02ab87b47c51f05bc71b956fb1878fbe69b205321e52798c8d82014`;
- recibo final: `p1430-verification-r2.json`, SHA-256
  `629564c6e88fe117158cf5a01ee647a6943bf6528d5bde29e019b416d72c49e5`;
- selo substituto: `p1430-seal-r2.json`, SHA-256
  `2a5fdf4c8612d6082e3c72354ecabb23d34d2eca9f35f9f7949ee49ccb8decd9`;
- veredito: `p1430-verdict-r2.json`, SHA-256
  `d9ca45bdc4f2b094592ba35f82c575e483df796f07e1cd59946d821ceb15bac6`.

O selo R1 permanece invalidado e não foi usado como crédito final.

## Ordens determinísticas

| Ordem | Sequência | Resultado |
|---|---|---|
| A — forward | stdlib → eval → entities → layout → V01–V05 | 23/23 + V01–V05 PASS |
| A — repeat | stdlib → eval → entities → layout → V01–V05 | 23/23 + V01–V05 PASS |
| B — reverse | V01–V05 → layout → entities → eval → stdlib | 23/23 + V01–V05 PASS |

Os três vetores foram idênticos. U01–U06 permaneceram `Unknown` opaco,
com crédito de aceitação e mutação igual a zero. Nenhum caso obrigatório
terminou `Unknown`.

## Campanha física

O recibo R3 contém 21 IDs únicos, 21 patches com SHA-256 correspondente e
21 falhas atribuíveis dos killers congelados:

- mortos: 21/21;
- sobreviventes: 0;
- `Unknown`: 0;
- crédito de `Unknown`: 0;
- mutation score: **21/21 = 1.0**.

O M21 foi auditado e reproduzido numa cópia temporária. O patch altera o
ramo executado `classification == "Unknown"` de `evaluate_mutation`, atribui
indevidamente `mutation_credit` e `score_numerator_delta`, e faz o oráculo
falhar de forma atribuível em V03 e V04 com exit code 1.

## Regressões e gates finais

- P1047: 7/7 PASS;
- P945: 11/11 PASS;
- `math_cases`: 6/6 PASS;
- `cargo fmt --all -- --check`: PASS;
- `cargo build`: PASS;
- `crystalline-lint .`: PASS, exit 0;
- `git diff --check`: PASS.

## Limite da alegação

O certificado cobre somente os observáveis congelados de `math.cases` no
P1430 e a política executável de falha fechada contra os 21 mutantes R3. Não
prova paridade matemática geral, os outros dezessete caminhos residuais,
igualdade de bytes de render, adequação exaustiva de mutações ou isolamento
formal do workspace.
