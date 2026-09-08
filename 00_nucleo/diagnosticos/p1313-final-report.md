# P1313 — CSV/DataSource: proposta pronta, implementação não iniciada

## O que foi confirmado

CSV deve receber dados em memória, não apenas caminhos. O vanilla ratificado
`a51e02804` declara DataSource e aceita Bytes. O cristalino ainda rejeita Bytes
no helper de caminho antes de atingir o decoder puro já existente.

| Expressão | Vanilla ratificado | Cristalino P1312 |
|---|---|---|
| `csv(bytes("a,b\n1,2"))` | duas linhas de strings | rejeita bytes como caminho |
| `csv(bytes(""))` | array vazio | rejeita bytes |
| Bytes com delimiter `;` e row-type dictionary | dicionário com as colunas | rejeita bytes |
| `csv(42)` | `expected path, string, or bytes, found integer`, span em 42 | mensagem portuguesa, tipo int, detached |
| `read(bytes("a,b"))` | rejeita Bytes com origem correta | igual ao vanilla após P1312 |

Medição fresca de 14 expressões nos dois executáveis, em
`2026-09-08T11:26:34.704729–11:26:37.719210Z`, com fontes e saídas integrais:
`p1313-measurement.json`, SHA-256
`f07efa356c554a4b80a5702c516c4e7f7bee51f4b70fc8cc419ea6cf8754cb43`.
Também foram medidos With, Bytes malformados/UTF-8 inválido, opção inválida,
missing, named desconhecido, Symbol e csv.encode. Parsing/opções têm diferenças
diagnósticas adicionais; aceitar Bytes não as resolve automaticamente.

## Proposta escrita

Passo: `00_nucleo/materialization/typst-passo-1313.md`.
L0: seção **CSV/DataSource — P1313 (proposta; aprovação ADR-0127 pendente)**
em `00_nucleo/prompts/compiler/stdlib/loading.md`.

O recorte propõe aceitar Path/Str/Bytes em CSV, encaminhar Bytes ao decoder
existente sem I/O e corrigir tipo/origem do cast inválido. Read, demais loaders,
encoders, opções e sua ordem ficam preservados. Novas rotas de erro de Bytes
mantêm o comportamento legado do decoder/validador; não há promessa de paridade
dos diagnósticos de parsing. Symbol continua rejeitado com formatter explícito,
sem alegar paridade com a coerção vanilla.

## Por que não foi implementado ainda

P1312 só corrigiu texto e origem de uma rejeição. Aqui há **ampliação das
entradas aceitas por CSV**. Existe uma tensão normativa real: ADR-0127 permite
correções gerais de paridade em fluxo contínuo, mas também exige gate para
compatibilidade ao reverter; o L0 proprietário já tratou ampliação de casts
públicos como gate em P1141. Não basta a assinatura Rust continuar igual para
resolver inequivocamente esse precedente.

O parecer independente `p1313-review-gate.md` aplica a regra expressa de
paragem em caso de dúvida: **GATE_REQUIRED_BEFORE_CODE**. O pedido genérico
de escrever/implementar o próximo passo antecede esta proposta concreta.
Falta confirmação do dono para CSV passar de Path/Str a Path/Str/Bytes.
Não houve alteração de código nem de testes, build novo, RED/GREEN ou
validação de candidato; não se apresenta a proposta como implementação pronta.
Após a redação, `p1313-review-proposal.md` julgou a proposta concreta
**READY_FOR_OWNER_REVIEW**, sem achados impeditivos da apresentação. Isso
não substitui a aprovação humana nem autoriza materialização.

## Estado e linhagem

Baseline `p1313-baseline.json`, SHA-256
`0ad67b8f92c25160f044c352241572916716e2489ad105a6314608d51f59de92`,
HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, **working tree não commitado**
com P1310–P1312. Lista exata de arquivos e diff/stat estão no baseline e em
`p1313-proposal-validation.json`, SHA-256
`b1cfd0dcd111092e03638b5b51bcdce1aa81be662d020cff5f1ad82c989613df`.
Apenas loading.md mudou entre os arquivos inventariados; 203 artefatos históricos
foram preservados. Source loading.rs continua SHA-256
`acdb71c8775765a6f54bcbd9925ec76d7493a6aac2b1675dcd7f1fac9e1cd8be`.

L0 proposto SHA-256
`fe8e147fca053d9ebd9f9eeb4b6b4fa1bdf51346e00ea3f8576a6fdb38e9e4ed`,
hash efetivo `a338b735`. O header produtivo permanece `174f831b` e Hash do Código
permanece `33952fd4`: **há um warning V5 esperado pela proposta ainda não
materializada**. Não foi mascarado por resselo prematuro. V15/V26 e diff/check
passaram; dry-run mostrou somente esse resselo futuro, sem aplicá-lo.

Baseline binário `/dev/shm/p1312-target.B8uLa9/release/typst`, SHA-256
`446c3ccfa5ef7f5eaf6031543f4d28566dac6eb53dca842ec7eb16acb084fbf0`;
vanilla `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

A skill tekt-materializacao-segregada levou à revisão independente do gate
antes de qualquer candidato e à paragem explícita. Trata-se de préflight A/B
sem atestação técnica de isolamento, não de certificação de implementação.
Nenhum stage, commit, push ou limpeza histórica foi feito.
