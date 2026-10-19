# L0 — compiler/eval/repr/content
Hash do Código: 7636ef68

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3


## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/repr/content.rs`. No HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` e árvore limpa, `01_core/src/compiler/eval/repr.rs:673-1373` possui o comportamento separado. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-repr-2026-09-30-before.json.gz`. O objetivo é pesquisa algorítmica por comportamento, sem ganho de desempenho presumido.

## Contrato

Manter o dispatcher exaustivo e estático de Content e sua representação por construtor. Preservar texto recebido, campos explícitos, ordem, Empty, sequências, morfologia de matemática e ausência/none distintos. Delegar helpers matemáticos e de formato estaticamente.

O contrato detalhado e as evidências históricas de paridade continuam em `00_nucleo/prompts/compiler/eval/repr.md`; a extração não muda saída, assinatura pública ou fase. Não há I/O nem estado global mutável.

## Aceitação

Preservar testes de morfologia de linguagem, compilar e passar `crystalline-lint .` com owner e hash válidos.


## Realização com avaliação única

repr de carriers internos projeta body/visual e conserva representação dos elementos da linguagem.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Repr de Source delega ao body e não publica carrier ou span.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Transformações tipadas — contrato aprovado

### Medição anterior à decisão

O corpus transformacoes-2026-10-14/results.json.gz e controles suplementares
registra identidade/presença perdida no carrier matrix/body. Fonte e owner
pré-migração estão em lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14/base-files.json.gz
com hashes e linhas em consumers.json. A aprovação do manifesto
960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d autoriza
a nova representação e resolução no layout. Semântica/morfologia são linguagem;
formato Rust e mecanismo de reconstrução são mecânica.

### Responsabilidade exclusiva

Repr nativo usa nome da operação e campos explicitamente presentes na ordem do contrato, com body por último e formatação de valores vigente. Matrix preserva repr genérico.

Testar RED→GREEN e revalidar os caminhos dependentes; o build não fecha X1–X5.
