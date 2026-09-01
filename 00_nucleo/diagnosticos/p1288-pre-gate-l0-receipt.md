# P1288 — receipt pré-gate de Núcleo + L0

**Instante:** 2026-08-31T10:40:40-03:00  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`  
**Árvore:** working tree não commitada e compartilhada. As escritas deste papel
estão enumeradas abaixo; nenhuma escrita ocorreu em L1–L4, lab, harness,
baseline, contrato, oráculos ou veredito.

## Regime, papel e capacidades

Protocolo completo da skill `tekt-materializacao-segregada`, fase causal
pré-implementação. Executor: agente `/root/p1288_l0`, autor/auditor de Núcleo +
L0. Entradas: Passo 1288 autorizado, ADR-0107/0108/0109/0127/0128/0129, L0s
enumerados, headers/trechos dos consumers e fonte vanilla pinada. Patch
candidato P1288 não foi lido. Escrita limitada a `_nuclei`, L0s e este receipt.
O checkout compartilhado não permite alegar isolamento ambiental forte.

## Medições anteriores às decisões

- Busca em `00_nucleo/prompts/_nuclei/**/*.toml`: não havia Núcleo de gates de
  feature; os hits existentes tratavam OpenType ou “feature” como unidade de
  atomização.
- `entities/html.rs:12-42`: `Feature/Features` pertencem hoje ao owner HTML e
  só representam `Html`.
- `cli.rs:77-91,137-141,310-322`: parser aceita somente `html` e não compõe
  lista por vírgula.
- `eval/mod.rs:117-120,911-921,2113`: contexto transporta features pelo path
  HTML, filtra apenas o binding `html` e instala o módulo PDF sem o trio.
- `pdf.rs:32-42`: namespace atual registra só attach/artifact.
- `table.rs:29-42` e `table_cell.rs:20-33`: não existem summary nem classe
  semântica de célula.
- `layout/table.rs:70-87`, `layout/table_cell.rs:14-20`: layout descarta a
  identidade semântica de Table/Cell depois de desenhar.
- `layout_types.rs:300-330,891-930`, `stream.rs:138-174,1196-1222` e
  `builder.rs:2522-2655`: carrier/tagging cobrem Formula/Artifact, não tabela.
- Ownership medido pelos headers: os consumers atuais continuam 1:1. Em
  particular, `rules.rs` pertence a `compiler/eval/rules.md` e
  `stdlib/structural/table_grid.rs` pertence a
  `compiler/stdlib/structural/table_grid.md`; nenhum foi apropriado por
  `compiler/eval/table.md`.

Fontes vanilla que precederam decisões: `typst-library/src/lib.rs:272-305`,
`pdf/mod.rs:12-23`, `pdf/accessibility.rs:137-143,203-220,264-310`,
`model/table.rs:271-277,772-784` e
`typst-pdf/src/tags/context/table.rs:63-135,197-239,289-353,374-440`.

## Decisão e artefatos

Criado `compiler-feature-gates.toml` com default vazio, ativação explícita,
ortogonalidade target/feature, ausência de binding desligado, ausência de
crédito de paridade e incapacidade ativa como Unknown/Violated.

O Núcleo tem SHA-256 raw
`9d7509d01a00589f48d7599d39921f017cc141cf45a8786de8fa73b65abe8de9` e
hash efetivo Tekt
`59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9`.
Os consumidores L0 pinam o hash efetivo completo: `compiler_features.md`,
`entities/html.md`, `shell/cli.md`, `wiring.md`, `infra/pipeline.md`,
`compiler/eval.md` e `compiler/stdlib/pdf.md`.

L0s atualizados: todos os acima, mais `entities/elements/table.md`,
`table_cell.md`, `compiler/eval/table.md`, `compiler/layout/table.md`,
`table_cell.md`, `entities/layout_types.md`, `infra/export/stream.md` e
`builder.md`.

Ownership futuro proposto:

- `entities/compiler_features.md` ↔ futuro
  `01_core/src/entities/compiler_features.rs`;
- `compiler/eval/table.md` ↔ futuro `01_core/src/compiler/eval/table.rs`;
- todos os demais prompts conservam seus consumers atuais.

Os dois owners futuros estão marcados `PROPOSTO`/não materializáveis até o
gate; nenhum consumer foi criado. `compiler/eval/table.md` não aponta mais
para dois consumers. Não foi necessária nem autorizada alteração de
`crystalline.toml`; V15 reconheceu o estado pré-gate e fechou limpo.

## Hashes SHA-256 dos artefatos L0 ao fim do papel

```text
compiler_features.md  5f12a9e8ca4b96fa3d9a06a54b5030f509ff2c5331374ff8fe5fb9b5ca567826
entities/html.md       71696c0e59cc8b66badfa6de3bd8b8622a6f60d994f3b589dc868f3df00bda50
shell/cli.md           de31ec1b7f6bc60faeb0942a25878922699f5ca8ac4810f4ffa7ea1908e9b880
wiring.md              960aa048d2075017116ec488280e2bf887066cd87edcff5daae1966b05344c32
infra/pipeline.md       250ec46bc143ea0f372799e7a24b5c7ac6b4573fec13e01c9085d0d5333d5378
compiler/eval.md        597324272eabfafe87f3588b8bd008d89caf760d40af1da55bd9b3956358db4d
compiler/stdlib/pdf.md  50a786cd8ee9d5c3709bae325bba24fe6def783f3b1b6da88e5612fc5f038530
entities/table.md       2fe30f7dca9306a66522b238d368505096aaa595ae3a4f3129789ba186ed3a63
entities/table_cell.md  f357690a18da076fc13c125000ea17ccfaa9f5ea871b46b8b6151185cae3baf6
compiler/eval/table.md  8984d3c54b00f039dcc6f759cf0c67a7eecbde28d27977199cf4ac6712d360d8
layout/table.md         637759bd4904c3d74ca8f9db3ed9437e41b6ece1d4bcf23cbe233fa02fe11d08
layout/table_cell.md    c27375012f64a3faae031bad5d257b7faa26e3814419748e130404c1c10f508e
entities/layout_types.md bc3b00aea1673ef898c79acd401211e8f3dac45ea023e53245db6933421f3436
export/stream.md        25c6f398f8df1703d73f12938032e412ece7ee68774ddcf32061e14e8c468a7b
export/builder.md       3e5c2fb0466745137d8600c3ad5ca73d53f1ae1e70d0448a4ca810d83f52dcd7
```

## Refinamento pelo baseline vanilla final congelado

Entradas lidas neste refinamento:

```text
p1288-vanilla-measurement-receipt.md 06a32ad3a77cc710a4ba1c54330325d0cc5a788ba2378cf0e6fda32269c5df80
p1288-vanilla-baseline.json          057bab7534c4855b0b47cc7cd05e243058d8b39407e30c2b3ee478f04d24c377
```

O baseline final refutou a hipótese provisória de que `summary: none` remove o
resumo: none explícito é erro (`expected string, found none`); somente omissão
produz ausência interna e omite `/Summary`. Os L0s agora fixam os diagnósticos
medidos, THead/TBody para header automático uniforme, TR direta para linha
explícita mista, Data em header preservada como TD, scopes e spans, e `level`
como determinante de relações `Headers` sem atributo numérico.

Para multipágina, os L0s especificam uma única árvore lógica, sem duplicar TH
de headers visuais repetidos; `StructParents` é por página, MCIDs reiniciam em
zero por página e `ParentTree /Nums` conserva a ordem local. `/MarkInfo` inclui
`/Marked true /Suspects false`. Estes resultados estruturais não alegam AT,
reflow nem PDF/UA, que permanecem Unknown.

## Gates executados

Binário `crystalline-lint` SHA-256
`80bb6b2aa23ce1b83f9a0a2540a4ff61a9a4993aca01b2e44f72bf16378e5cff`.

```text
python3 -c 'import tomllib; ...' compiler-feature-gates.toml -> TOML_OK
crystalline-lint --checks v26 --fail-on warning .            -> exit 0
crystalline-lint --checks v15 --fail-on warning .            -> exit 0
crystalline-lint --checks v15,v26 --fail-on warning .        -> exit 0 (pós-refinamento)
git diff --check -- 00_nucleo/prompts 00_nucleo/diagnosticos -> exit 0
```

Não foi executado `--fix-hashes`. Os `Hash do Código` permanecem deliberadamente
não ressellados porque o contrato público está no gate ADR-0127 e os dois
consumers novos são proibidos antes da confirmação.

## Paragem humana e pendências

PARAR. A proposta adiciona variante/capacidade pública de feature, nova flag,
carriers públicos de tabela/célula e extensões públicas de layout; ADR-0127
exige confirmação humana antes de testes RED ou qualquer código.

O baseline estrutural e a ortografia dos diagnósticos acima já estão
congelados e integrados. AT real, PDF/UA, reflow, certificação e acessibilidade
geral permanecem Unknown; não foram convertidos em sucesso por inferência.

**Linguagem proporcional:** Núcleo + L0 executados com segregação de papel e
ordem, sem atestação de isolamento ambiental forte; nenhum veredito funcional
foi emitido.
