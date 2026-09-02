# P1293/C — reautoria C-P08 por neutralidade semântica

Data: `2026-09-02T15:06:04-03:00`

HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`

Estado: **FALSO RED MECÂNICO REMOVIDO; RED PRODUTIVO C PRESERVADO; NÃO SELADO**.

## Papel e entradas

- Papel: `testador_c_serializacao_p1293`, autor independente do oráculo
  protegido.
- Regime: protocolo Tekt completo, segregado por capacidades e artefatos, sem
  isolamento técnico de leitura no filesystem compartilhado.
- Entrada causal do coordenador: a primeira execução C passou `3/5` grupos;
  C-P08 divergiu somente em metadata temporal e IDs internos de dois PDFs
  consecutivos. Esses bytes não são observável de linguagem pelo ADR-0107.
- O patch e o recibo do implementador não foram lidos. Produto, L0, manifesto,
  selo, mutantes e veredito não foram alterados.
- Escrita limitada ao oráculo, a este recibo e logs efêmeros em `/tmp`.

## Classificação e correção estreita

A igualdade byte a byte de PDF era um falso gate: o L0 exige que a opção de
serialização HTML seja aceita e neutra fora de HTML, não que timestamps,
DocumentID/InstanceID ou mecânica do writer PDF coincidam entre compilações.

C-P08 agora compara somente observáveis semânticos por formato:

- PDF: texto extraído por `pdftotext -layout` e estrutura estável reportada por
  `pdfinfo` (`Pages`, `Page size`, `Page rot`, `Tagged`, `Encrypted` e
  `PDF version`); também exige que `neutral` permaneça extraível;
- PNG: dimensões, channels, colorspace e assinatura dos pixels decodificados
  via `identify`, excluindo chunks de metadata;
- SVG: `viewBox` e contagens estruturais já usadas pelo oráculo, sem igualdade
  de bytes integrais.

O mesmo input e o mesmo output temporários são reutilizados entre
`crystalline` e `vanilla`, eliminando path como variável. A prova HTML continua
forte e inalterada: default byte-idêntico ao `crystalline` explícito, modos com
escaping contextual distinto e DOM decodificado equivalente.

C-P03..C-P05, seus transcripts e o diagnóstico com o polo de linguagem `none`
não foram tocados nem relaxados. A/B/D foram preservados. O registro de
mutantes permanece integralmente igual: `31` IDs, incluindo MC10/MC11; não foi
necessário criar mutante novo para substituir uma comparação mecânica por uma
semântica.

## Artefatos pinados

| Artefato | SHA-256 |
|---|---|
| binário candidato executado `target/debug/typst` | `910d0622ca79e4617a30bb46034404b51cfd45367c984ccecb80a2a6a95b028d` |
| oráculo protegido reautorado | `ed3e0b57c703a1c4dbdbeda187fcf74dcf46476f60a2c850e48a9ef8f0b2df1e` |
| corpo canônico, removendo somente `@prompt-hash` | `b07eb8d6eb6bfa0a892c06bb1f5599d8e2e188963dcca821d83f4a34badfb13a` |
| registro de mutantes inalterado | `417d82202439b39388d223fe46c9dc193e41b705a951dfeb19c80eddfd75bd8b` |
| execução C final | `f869e7fd7dec9b91206ea37515d3051f59428c12bb4ccabfbdb18f4b7e615182` |
| `pdftotext` | `0fb98ea179e19154a90202608c164f2a319b79f16576fa6534b2d601033565e7` |
| `pdfinfo` efetivo no PATH | `fee70ade670fb025343aca2b5c3a2aacacb8ed9edce1b716b233b5de924b6bf5` |
| `identify` | `9c47b9e24bfcf60e05ef327ea432742623ceb08d7a34e7356f38522447731e02` |

O oráculo tem `49656` bytes; o log C final tem `40563` bytes.

## Execuções e causalidade

Teste estreito da correção:

```text
$ cargo test -p typst-wiring --test p1293_contract p1293_c_serialization_modes_default_scope_and_non_html_neutrality -- --exact --test-threads=1
exit 0
test result: ok. 1 passed; 0 failed; 10 filtered out
```

Execução dos cinco grupos C sobre o oráculo final:

```text
$ script -q -e -c 'cargo test -p typst-wiring --test p1293_contract p1293_c_ -- --test-threads=1' /tmp/p1293-c-after-p08-semantic-fix-v2.log
exit 101
test result: FAILED. 4 passed; 1 failed; 6 filtered out
```

Verdes: DOM/escaping, fachada pública direta, eixos feature/target e C-P08.
O único grupo RED é
`p1293_c_surface_all_specific_casts_and_closed_errors`, pelas divergências
produtivas já comunicadas pelo coordenador: morfologia multiline de attrs em
C-P03..C-P05 e diagnóstico fechado que omite o valor de linguagem `none`.
Não houve timeout, crash, erro dos parsers semânticos ou `Unknown`.

Gates locais:

```text
$ rustfmt --edition 2021 --check 04_wiring/tests/p1293_contract.rs
exit 0
$ crystalline-lint --checks v5,v15,v26 --fail-on warning .
exit 0; No violations found
$ git diff --check
exit 0
```

## Proveniência da working tree

A árvore estava não commitada. No instante da medição:

```text
git diff HEAD --binary | sha256sum
110a4890379074b876e45fb0a546561c09aa39f856c9b8756867527e285ce200  -

git diff HEAD --name-only | sha256sum
6d6fb9913dafa3be6587076a92f5a19265bc45547c0541d4d9a9d52a48c9fa36  -
63 tracked paths

git ls-files --others --exclude-standard | sha256sum
6143f4e8f69b889cd1a2d223d5b18aac77fd8d4b33a44d128f8b29130ee8bb5c  -
45 untracked paths
```

A lista exata dos 63 paths tracked é byte-idêntica à registrada em
`p1293-red-tests-receipt-c-export-facade.md`, SHA-256
`a536c667cffcdc5c8f3756fe40198ffae12e11347b030497499d08c093c2e245`;
o hash `git diff HEAD --name-only` acima confirma a identidade da lista. O hash
binário difere porque o produto e o oráculo avançaram, sem atribuição dessas
alterações compartilhadas a este papel.

## Próximo estado

O falso negativo C-P08 está encerrado. O lote continua corretamente RED pelo
grupo de superfície produtivo. Após a correção por autoridade separada, o
verificador deve repetir os cinco grupos C e o gate discriminatório completo
`31/31`; este recibo não reivindica selo, score ou veredito final.
