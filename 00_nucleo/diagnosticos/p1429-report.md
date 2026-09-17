# P1429 — relatório de verificação final R4

## Veredito

**PASS** para a revisão
`925f63ba06d85050fc00d9926b63a370d1e501b5`, árvore
`66afa18b8f36cbcfb9649311656b219dee138ad2`.

O selo R4 substitui integralmente o R3 e fecha o bloqueio anterior com doze
mutantes físicos versionados. O protocolo completo do skill
`tekt-materializacao-segregada` foi executado sem atestação formal de
isolamento. Cada mutante usou um clone local separado; somente o cache Cargo
foi compartilhado.

## Integridade

- selo substituto: `p1429-seal-r4.json`, SHA-256 `f9a7096d1056e7b0...`;
- 28/28 entradas protegidas R4 idênticas: 16 artefatos/testes + 12 patches;
- o selo R3 invalidado não foi usado como evidência final;
- nenhum código produtivo, L0, teste, artefato adversarial ou selo foi editado
  pelo verificador final.

## Ordens A/B/C

| ordem | processo | resultado |
|---|---|---|
| A — forward | suite focal completa | 10/10 PASS, exit 0, 0,24 s |
| B — forward repeat | novo processo, suite focal completa | 10/10 PASS, exit 0, 0,24 s |
| C — reverse suite order | layout → eval → entities → stdlib, quatro processos | 3+1+2+4 = 10/10 PASS |

Nenhum caso obrigatório terminou `Unknown`. U01–U06 permaneceram `Unknown`
com `acceptance=false`, `mutation_credit=false` e crédito zero. Não houve
falha de baseline não atribuível.

## Campanha física independente

Comando:

```text
python3 00_nucleo/diagnosticos/p1429-mutation-runner.py \
  --repo . --output /tmp/p1429-r4-final-verifier.json
```

Resultado bruto SHA-256:
`0e1e4dc24e9c9627ae0d8fc87c1ffc6abe7c3357db778f3626014eb07cd07b8f`.

- clones frescos: 12;
- patches aplicados: 12/12;
- killers únicos executados: 12/12;
- `Violated`: 12;
- `Preserved`: 0;
- `Unknown`: 0;
- sobreviventes: 0;
- crédito de `Unknown`: 0;
- mutation score: **12/12 = 1.0**.

Cada processo exibiu exatamente `running 1 test` e `test result: FAILED`, com
exit code 101 e testemunha atribuível ao mutante correspondente.

## Gates finais

- `cargo fmt --all -- --check`: PASS;
- testes focais A/B/C: PASS;
- `cargo build`: PASS, apenas avisos não bloqueantes;
- `crystalline-lint .`: PASS, exit 0; V5/V15/V26 limpos;
- `git diff --check`: PASS;
- JSON da cadeia R4: sintaxe válida.

## Limite da alegação

O certificado cobre somente os observáveis congelados de `math.accent` no
P1429 e os doze mutantes concretos R4. Não prova paridade matemática geral,
adequação exaustiva de mutações, igualdade de bytes de render ou isolamento
formal do workspace.
