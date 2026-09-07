# P1307-R4 — bloqueio real na informação de Content

## Resultado

A aprovação do dono para Args, encoders e suas alterações de representação
foi recebida e continua válida. A implementação **não começou**: a auditoria
prévia encontrou uma obrigação que o contrato aprovado não consegue cumprir
com os dados que chegam ao serializer. Não houve RED semântico, GREEN, build
candidato, mutação Rust, selo final, stage, commit, push ou remoção de evidência.

Não é uma nova espera burocrática pela mesma aprovação. O L0 que preparamos
tratava como suficiente o transporte de Args e a projeção de Content; a
auditoria refutou essa suficiência para conteúdo realizado de introspecção.
Essa lacuna deveria ter sido identificada antes de apresentar o contrato R3.

## Evidência anterior à decisão

Estado: HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree
não commitado. O snapshot integral, com diff/stat, hashes e horário, está em
`00_nucleo/diagnosticos/p1307-r4-baseline.json`, SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`.
A conferência posterior está em `p1307-r4-lineage.json`, SHA-256
`7d1b1130de0461f0c9f8a030f9d7b476fee3eb6efbf4bdf4c9c443c3b334f3fb`:
nove L0 mudaram desde esse snapshot; todos os arquivos produtivos e de testes
permanecem idênticos, inclusive as alterações P1306 preexistentes. Os
artefatos predecessores protegidos também permanecem idênticos.

No vanilla ratificado `a51e02804`,
`lab/typst-original/crates/typst-library/src/foundations/content/mod.rs:709-718`
serializa `func` seguido dos campos do próprio conteúdo. O corpus independente
R2, preservado em `p1307-r2-measurement.json`, exige que uma heading devolvida
por `query()` conserve campos realizados e label, também nos encoders.

No cristalino atual:

- `01_core/src/entities/elements/heading.rs:24-42` conserva nível, corpo,
  outlined, bookmarked e máscara de presença, não o conjunto realizado;
- `01_core/src/compiler/eval/bindings/field_access.rs:589-657` projeta os
  campos explicitamente presentes; não realiza os campos de uma heading
  consultada;
- `01_core/src/compiler/introspect.rs:1345-1350` separa a label no payload
  da tag e armazena o Content nu para consultas;
- `01_core/src/compiler/stdlib/foundations/query.rs:59-61` clona esse
  conteúdo, acrescentando a Location, mas não os campos ausentes;
- `01_core/src/compiler/eval/rules.rs:970-1003` aceita o padrão string de
  numeração e guarda-o na chain, enquanto
  `01_core/src/entities/introspector.rs:388` conserva apenas a ativação
  booleana por heading. Outros argumentos de set heading ainda são
  explicitamente ignorados como dívida anterior.

Uma Location não contém os campos perdidos. Existe consulta indireta de
labels no introspector, mas isso não equivale a transportar os campos do
valor e não resolve padrões, suplemento ou valores que escapem do contexto.
Portanto não se alega impossibilidade de qualquer desenho sem nova API;
alega-se insuficiência do desenho puro de Value atualmente congelado.

Preencher `numbering: none`, `supplement: [Section]` e outros defaults no
encoder faria a testemunha simples parecer correta, mas inventaria dados
para documentos com outros estilos. Não é uma implementação aceitável.

### Contraprova bilateral focal

`p1307-r4-content-observability.json`, SHA-256
`26a27995e98dcde01a7dbb47838a40ad8394fc39ac3af8e2e7863cc6639e6634`,
preserva fontes, binários, comandos, saídas e diff/stat. Foram 40 execuções
no perfil default em `2026-09-07T17:57:32.589178Z`–
`2026-09-07T17:57:34.687368Z`, sem Unknown do adaptador. O baseline
executado é o binário congelado no snapshot acima, SHA-256
`945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3`;
não é um build deste L0 ainda não materializado.

Com `heading(numbering: "1")` e `heading(numbering: "I")` em set rules,
o vanilla conserva os respectivos padrões nos campos do conteúdo consultado
e nas strings JSON. No cristalino, ambos continuam de tipo content, mas
`fields()` devolve a mesma string `(depth: 1, body: [Probe])` e repr também
é idêntica. A auditoria da chain e do armazenamento localiza a perda;
igualdade de repr sozinha não seria prova suficiente.

O par de suplementos `[Alpha]`/`[Beta]` também é distinguido pelo vanilla e
não pelo baseline. Sua causa inclui a dívida anterior de set heading que
ignora esse argumento, portanto não foi apresentada como o mesmo defeito
de transporte do padrão de numeração. A ausência de json.encode no baseline
foi registrada, mas **não** usada como prova de perda de dados de Content.

## Trabalho preservado

A auditoria de writers está em `p1307-r4-writers.md`. Os refinamentos de L0
anteriores ao código corrigem transporte por métodos/counters, ordem do sink,
delegação de Args+Args pelo operador público e agregado da chamada para
diagnósticos de encoders. Seus snapshots anteriores estão nos recibos
`p1307-r4-transport-amendment.json`, `p1307-r4-contract-refinement.json` e
`p1307-r4-call-span-refinement.json`.

O corpus principal independente foi congelado em `p1307-r4-oracle.json`:
455 casos nos quatro perfis, reunindo obrigações anteriores e os deltas de
Args/repr/CBOR. O suplemento `p1307-r4-math-oracle.json` acrescenta 11 casos.
São expectativas e controles anteriores ao candidato, **não testes verdes
da implementação**. A nota `p1307-r4-oracle-note.md` delimita a reutilização
dos predecessores: o primeiro corpus P1307 só executou a ordem normal;
não se reivindicam repetições binárias que não ocorreram.

`crystalline-lint .` foi executado em leitura, com comando, horário,
stdout/stderr e estado integral em `p1307-r4-commands.json`. Não houve erro
estrutural; há 21 avisos V5 esperados pelos L0 ainda não materializados, além
dos informes existentes. Isso não satisfaz o gate final de linhagem.

O cache de compilação candidato foi preparado separadamente na RAM em
`/dev/shm/p1307-r4-target.8W1BEA`; seu binário ainda é uma cópia do baseline,
não um candidato compilado. Nenhum temporário foi apagado.

## Decisão e próximo limite de autorização

P1307 permanece incompleto. A aprovação existente não é revogada, mas não
autoriza editar silenciosamente novos owners de Content, query, realização
de estilos ou introspecção. O passo exige parar e reabrir o escopo quando
essa necessidade surge.

O próximo trabalho necessário é especificar como os campos públicos
realizados acompanham o conteúdo consultado, antes da serialização, sem
hardcodes nem recuperação de origem por igualdade. É ampliação de L0 além
da migração de Args; o contrato concreto e os owners afetados precisam ser
definidos e auditados antes de qualquer implementação dessa ampliação.

Regime: **executado sem atestação de isolamento técnico**. Coordenador e
auditor não emitem aprovação da própria solução; o verificador independente
não editou os materiais julgados. Não há certificado de paridade do produto.
