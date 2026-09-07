# P1308-R2 — migração autorizada e entrega acumulada

O conflito de transcripts que bloqueava a entrega foi resolvido pela autorização
do dono: “Faça o commite do que falta e autorizo”. Os L0s foram atualizados antes
dos testes. Não houve nova alteração de comportamento produtivo nesta revisão:
o patch integrado modifica somente seis expectativas de stderr de P1293;
o oráculo sucessor modifica somente os 76 transcripts já identificados.

O relatório anterior `p1308-final-report.md` permanece histórico, com seus
resultados e bloqueio daquele momento intactos. Esta revisão o sucede, não
reescreve falhas passadas como sucessos.

A entrega acumulada reúne a correção do diagnóstico de módulos P1306,
encoders JSON/TOML/YAML, Args com ordem causal e snapshot de introspecção P1307,
as correções de origem/trace/repr P1308 e esta migração de expectativas.
Inclui os L0s, o núcleo compartilhado de introspecção, passos explicitamente
autorizados e evidências em diagnosticos, inclusive a nota pendente do commit
P1305. Não altera o alvo vanilla, não muda de branch e não faz push.

## Resultado e proveniência

Medições no HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, com working
tree não commitado. Cada recibo de comando registra UTC inicial/final, argv,
saídas integrais, exit, HEAD e `git diff HEAD --stat`/diff/status antes e depois.
O commit de entrega será posterior às medições; não é alegado como estado limpo
usado para produzir os resultados.

| Gate fresco | Resultado | Recibo em diagnosticos |
|---|---|---|
| Workspace release completo | 6620 passam, 0 falham, 3 ignorados | `p1308-r2-workspace-tests.json` |
| Matriz pública sucessora | 1982 Preserved, 0 Violated, 0 Unknown | `p1308-r2-public-matrix.json` |
| Matriz pública em ordem inversa | 1982 Preserved, 0 Violated, 0 Unknown; observáveis idênticos | `p1308-r2-public-reverse.json` |
| Controles públicos focais | 128 Preserved, 0 Violated, 0 Unknown | `p1308-r2-public-focal.json` |
| P1293 focal | 1 teste passa, sem falha | `p1308-r2-focal-p1293.json` |
| Build release | exit 0 | `p1308-r2-build.json` |
| Formatação | exit 0 | `p1308-r2-fmt.json` |
| Linter e linhagem | exit 0; warnings/info permanecem | `p1308-r2-lint.json`, `p1308-r2-lineage.json` |

Pins dos gates principais:

- Workspace: `207dbd534febcdf207c914a5bd208e20416c9d9e1b5f35e7c561bb2fbb533c42`.
- Matriz: `89db8675ebe23d3cd8a9ddba4347445a8354f018378b70209d8ff45e0e901882`.
- Matriz inversa: `5a15e9b089257eff26222b04d4f9361262048e4c6527fc6abc4829e99aaaeb19`.
- Build: `64a85701ad5dc76260336788a11853f4ccebfc868cef88383c3dde13b477e041`.
- Formatação: `c25bbfc184e2789d95f282fe072f646ca5325e0b27df9ef40d7415a5e27981b4`.
- Linter: `fef8a930422f78b3e8ec726bcae5f2c76de3064f2158b1750fe7bae933ee4cfb`.

Binário medido: `/dev/shm/p1307-r4-target.8W1BEA/release/typst`, SHA-256
`09725fff8b4c472ee6b50a9ca6105d90268b2a0da8a22f1d8abd0f45e20c4c50`.
A referência continua upstream vanilla ratificado `a51e02804`; não se usa
a string de versão do cristalino para identificar o baseline.

## O que a autorização mudou

A medição independente `p1308-r2-measure.json` executou os 19 casos históricos,
os seis erros grid/table e um controle positivo em quatro perfis e duas ordens:
208 Preserved, sem Violated ou Unknown. SHA-256
`b507c4a71345012a2e6b2ccd7ad3c7c89f130f1d66c32c853e5d0d28d4637079`.

Os 515 casos do oráculo permanecem; somente 76 campos `future_expected.stderr`
recebem o trace literal autorizado. Mensagens primárias, stdout, exit, fixtures,
comparador, observador e classificação Unknown não foram afrouxados.
As seis strings P1293 também preservam mensagens e controles de sucesso.
O oráculo R6 original e seus resultados Violated permanecem imutáveis.

Autoria e pins em `p1308-r2-contract-note.md`. Sucessor independente em
`p1308-manifest-successor-2.json`, SHA-256
`e93ca4ece40955aa34c86803613c8b024e71fc55dfb4fdf479e74801249100ec`.
O runner de recibos antigo continua registrando manifesto original e sucessor 1;
a vinculação R2 é explícita pelos pins deste relatório e da verificação final,
sem alegar que o arquivo do sucessor 2 precedeu todos os comandos.

## Alcance e limitações

A skill `tekt-materializacao-segregada` determinou autoria independente das
expectativas e verificação segregada. Regime: **executado sem atestação de
isolamento técnico**. A limitação cronológica do selo original P1308 continua
descrita no relatório anterior; esta revisão não a apaga.

Preservar erros primários portugueses/detached com traces resolvíveis não os
torna paridade vanilla. As dívidas de panic com named inválido, CBOR e decoders
continuam fora deste reparo. As 37 famílias históricas de mutação P1307 continuam
pendentes; não se emite certificado geral de P1307 ou do compilador.

Na revisão de whitespace, código e L0s passaram. Foram preservadas duas linhas
vazias finais em artefatos históricos pinados: `p1308-tests-note.md:132` e
`p1308-tests.patch:193`. Não se altera evidência congelada para silenciar esses
avisos documentais; eles não são falhas de teste ou de linhagem.
O check do índice também sinalizou os dois espaços de quebra de linha Markdown
em `typst-passo-1307.md:3–4`; foram igualmente preservados. No índice,
`git diff --cached --check` de código e L0s continuou limpo.

## Verificação adversarial focal

Os seis mutantes P1308 foram executados em cópia descartável em RAM, compilados
no mesmo perfil dos controles e rejeitados por diferenças causais: identidade
Args/none; span completo de panic; nome longo do tipo no cast bool de filter;
origem do callback; Source resolvível no CLI; e forma multilinha de Args longo.
Resultado focal: **6/6 rejeitados, zero Unknown e zero falhas de compilação**.
Não se contam falhas de instrumentação como morte de mutante.

Os controles sem mutação passaram: 18 testes Rust e 24 células CLI. M05 foi
verificado pelo binário CLI real, não apenas pelo MockWorld dos testes. Seus
20 transcripts perderam traces; os quatro controles de panic direto foram
preservados. Os recibos `p1308-mutation-M01.json` a `p1308-mutation-M06.json`
guardam comandos, perfil, patches, hashes de fontes/binários, UTC e saídas.
O campo bruto de revisão pendente desses recibos é resolvido pelo ledger
independente final, sem reescrever os registros de execução.

Ledger independente: `p1308-mutation-ledger.json`, SHA-256
`c636972025a884bf33af0dedb316b77c82ef44535ca02ca78057d804d32e2837`.
Verificação final: `p1308-verification-r2-final.json`, SHA-256
`6f6d9cfd4d3cb8233915c0cf59f9734a1b021139486e09fb743f2646fa5c37c0`.
Esta confirma integração canônica, L0 sucessor, predecessores, HEAD/index,
os gates frescos e igualdade exata dos observáveis nas duas ordens da matriz.

A restauração byte a byte da cópia e a integridade do candidato original
foram verificadas ao fim. Não houve aplicação de mutantes na working tree
produtiva. A entrega permanece limitada ao contrato observado e ao regime
sem atestação de isolamento técnico descrito acima.

Este relatório integra a entrega autorizada na branch `Tekt`. Seu commit
é identificável pelo histórico deste arquivo; as medições acima continuam
explicitamente atribuídas à working tree anterior. Nenhum push integra a entrega.
