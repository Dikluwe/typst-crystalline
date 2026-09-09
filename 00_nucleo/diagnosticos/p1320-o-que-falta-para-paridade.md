# P1320 — o que realmente falta para paridade

**Auditoria executada; paridade continua aberta.** O problema restante não é
simplesmente acrescentar mais posições aos erros de CSV. Há chamadas válidas
rejeitadas, chamadas inválidas aceitas e resolução de arquivo no contexto errado.
Também foi confirmada uma ausência fora de CSV: o construtor `array(bytes(...))`.

Este documento é um diagnóstico, não um L0 nem autorização para implementar
todo o backlog. A implementação de P1320 é o verificador e sua execução.
O produto e o L0 P1319 foram preservados, sem commit neste passo.

## Evidência atual antes da classificação

Medição sobre HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093` **mais working
tree não commitado P1319**. O commit sozinho não contém o candidato medido:

```text
00_nucleo/prompts/compiler/stdlib/loading.md |  92 +++++++++-
01_core/src/compiler/stdlib/loading.rs       | 247 +++++++++++++++++++++++----
2 files changed, 306 insertions(+), 33 deletions(-)
```

Binário cristalino `/tmp/p1319-target.VqXtmj/release/typst`, SHA-256
`37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd`.
Vanilla `/usr/local/bin/typst`, upstream ratificado **`a51e02804`**, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Não se usou string de versão para identificar revisão.

Os recibos abaixo contêm UTC por comando, argv/cwd, saída integral, diff/stat,
estado Git, hashes antes/depois do produto, fixtures, ferramentas e evidências
P1319. Os artefatos temporários e seus hashes foram preservados em `/tmp`.

| Recibo | Janela UTC em 2026-09-08 | Uso |
|---|---|---|
| `p1320-measurement.json` | 20:00:32.597364–20:02:21.562293 | Matriz transversal válida; lote focal inicial inválido, excluído |
| `p1320-focal-r1.json` | 20:03:30.208932–20:03:41.221145 | CSV e sondas transversais corrigidas |
| `p1320-supplement.json` | 20:04:54.476403–20:04:57.074575 | Controles isolados de Path/closure e JSON |
| `p1320-reconciliation.json` | UTC próprio no recibo | Integridade, classificação consolidada e gates |

SHA-256 da reconciliação:
`95118b2e9f82cb391a486bf33f60a70f1f95b6643de74d0112e36adeed7296ce`.
Ela fixa os hashes de todos os recibos acima e distingue correção de sonda de
correção do produto. Nenhuma evidência antiga foi sobrescrita.

## Lacunas confirmadas e ordem sugerida

Os IDs referem-se às expressões e saídas integrais nos recibos focais. A ordem
é uma priorização inferida por impacto e dependência, não estimativa de esforço.

| Ordem | Falta confirmada | Medição e consequência | Critério para fechar em passo futuro |
|---|---|---|---|
| 1 | Validação de argumentos de CSV | `csv-extra-valid`: vanilla rejeita o excedente, cristalino devolve dados. `csv-extra-bad`: muda o erro vencedor. `csv-unknown-before-source` e `csv-unknown-before-missing`: precedência errada. | Rejeição, mensagem, span e erro vencedor corretos; controles de leitura e chamadas indiretas, sem regressão dos casos válidos. |
| 2 | Origem de resolução de strings | `closure-only`: vanilla lê `sub/data.csv`; cristalino procura `data.csv` na raiz do chamador. `csv-detached-good`: cristalino lê onde vanilla rejeita falta de origem. | Resolver pela origem legitimada da chamada/argumento; manter Path capturado, chamadas diretas e pacote; não contornar sandbox. |
| 3 | Import no modo `eval` | `csv-captured-eval` e `csv-closure-eval`: vanilla retorna dados; cristalino falha com `current file is outside its sandbox root` antes do CSV. | Import local legítimo funcionar em eval, com testes negativos de escape. Separar esta falha da resolução da closure em compile. |
| 4 | Coerção de `Symbol` em CSV | `csv-symbol-source` lê o arquivo chamado `,` no vanilla e é rejeitado aqui; `csv-symbol-delimiter` também é aceito só pelo vanilla. | Cobrir conversões legitimadas pelos casts vanilla, inclusive símbolos inválidos, sem converter qualquer valor arbitrariamente. |
| 5 | Diagnóstico externo de CSV UTF-8 válido | `csv-unequal-{array,dictionary}-{str,path}`: causa coincide, mas faltam arquivo, linha e marcação externa. | Transportar a identidade diagnóstica necessária sem usar inclusão de Source como atalho; preservar ramo binário P1319 e parser único. |
| 6 | Diagnósticos de I/O e argumentos ausentes/desconhecidos | `csv-missing-file`, `csv-missing-file-path`, `csv-missing-source` e `csv-unknown-option`: mensagem/origem diferentes. | Diagnósticos públicos corretos, sem wrapper português indevido, com causa e origem preservadas. |
| 7 | Construtor `array` para bytes | `bytes-roundtrip`: vanilla retorna `[0,127,255]`; cristalino diz que array não tem construtor. | Conversão de coleções conforme contrato vanilla, com erro de entrada inválida e limites explícitos do construtor. |

Checagem da fonte que sustenta a classificação:

- `01_core/src/compiler/stdlib/loading.rs:1381` verifica named desconhecido antes
  da fonte; `:1100` usa o primeiro argumento e `:1391` segue ao parser sem rejeitar
  excedentes. Vanilla `foundations/args.rs:259` descreve rejeição dos remanescentes.
- `01_core/src/compiler/stdlib/foundations/path.rs:15` resolve Str pelo
  `current_file`; `compiler/eval/call_dispatch.rs:512` encaminha o contexto do
  engine à função nativa. Vanilla `loading/mod.rs:87` resolve com `self.span.id()`.
  O controle `captured-only` passa nos dois binários; isso refuta a hipótese ampla
  de que todo Path capturado esteja quebrado. A causa completa do import em eval
  ainda precisa ser rastreada; não foi inferida a partir da mensagem de sandbox.
- `loading.rs:1102` aceita Str/Path/Bytes; `:1329` limita delimiter a Str.
  Vanilla `loading/mod.rs:55` declara DataSource via PathOrStr/Bytes e
  `loading/csv.rs:103` declara o cast de delimiter via caractere. As sondas
  confirmam a diferença de coerção; não se prescreve uma conversão global nova.
- `01_core/src/contracts/world.rs:59` devolve bytes, sem identidade externa;
  vanilla `diag.rs:858` usa FileId para o range em texto válido. O contexto File
  em `loading.rs:990` só recebe apresentação especial se o buffer é inválido.
- `compiler/eval/call_dispatch.rs:1659` não despacha Type::Array e cai na rejeição
  de `:1691`. Vanilla `foundations/array.rs:164` documenta o construtor e `:1178`
  converte Bytes em inteiros. Aqui há intenção documentada, além da observação.

Essas diferenças são de semântica ou diagnóstico público, não da estrutura Rust.
Uma implementação futura que faça os casos falhantes coincidirem e preserve
os controles refutará a classificação de dívida aberta; mera presença de função
ou lint verde não a refuta. Cada correção começa pela auditoria do L0 proprietário.
Se exigir contrato público/default/fase/compatibilidade, aplica-se o gate ADR-0127.

## O que não deve continuar listado como recurso faltante

**Retificação explícita de P1319: `csv.encode` não é dívida de paridade.**
`csv-encode-member` é rejeitado com o mesmo diagnóstico nos dois binários, em
todos os perfis focais. A fonte ratificada `loading/csv.rs:26` declara apenas a
função de leitura, sem escopo de encoder; `loading/mod.rs:37` registra essa função.
Implementar um encoder seria extensão do produto, não fechamento de uma ausência
do vanilla. O relatório histórico P1319 foi preservado, mas sua classificação
como dívida separada fica corrigida por esta medição.

`sys.version` já coincide: `version(0, 15, 1)`. HTML básico também já funciona
com a feature apropriada. Não reutilizar como estado atual as ausências do
rebaseline P1210 ou os percentuais do inventário antigo de cobertura.

No CSV, coincidiram os valores amostrados por Bytes/Path/Str, row-types, arquivo
vazio, cabeçalhos duplicados, campo citado multilinha, spread/with, e diagnósticos
binários P1319. Isso confirma controles específicos, não paridade completa CSV.

## Matriz transversal: o que seus resultados realmente dizem

Matriz vigente `lab/parity/matrix/manifest.yaml`: 20 casos, repetidos em ordem
direta/inversa, com binários explicitamente fixados. Fonte/manifest/fixtures e
saídas da ferramenta são identificados no recibo, sem alteração do harness antigo.

| Perfil | MATCH | DIFFERENCE | Desabilitado pelo perfil | Unknown |
|---|---:|---:|---:|---:|
| default | 15 | 2 | 3 | 0 |
| html | 17 | 2 | 1 | 0 |
| a11y-extras | 16 | 2 | 2 | 0 |

**O exit 0 do runner não significa zero diferenças:** os dois DIFF já são
expectativas históricas do manifesto. A reconciliação os mantém abertos.
Perfis da matriz não injetam features em comandos não declarados; logo os
resultados repetidos não provam cada subsistema sob todas as combinações.

- **PNG:** na fixture simples, dimensões iguais; 11 pixels divergentes em
  2.005.644, delta máximo de canal 1. É diferença raster medida, não prova de
  erro de língua. Falta localizar os pixels e separar geometria/antialiasing/
  arredondamento antes de decidir se há correção funcional. Não elevar tolerância
  para converter o resultado em sucesso.
- **PDF:** texto `Hello, parity.`, uma página e caixa `595.276 × 841.89 pt`
  coincidem. O comparador difere apenas em `LibertinusSerif-Regular` versus
  `LibertinusSerif-Regular-Identity-H`. Hipótese: nome/encoding mecânico, não
  família diferente; `03_infra/src/export/builder.rs:295` constrói o nome subset
  e `:302` lê o nome PostScript. Falta verificar a fonte efetivamente embutida e
  seus observáveis antes de classificar como bug de produto ou normalização
  insuficiente. Igualdade textual do nome não é critério de língua.
- **SVG/HTML, query, geometria e demais MATCH:** limitados às fixtures e aos
  comparadores declarados. Smoke de produção não prova renderização completa.

## O que falta medir para responder sobre o produto inteiro

Não existe neste passo um denominador completo da linguagem. O corpus focal
contém 51 expressões em quatro perfis: após correção explícita da sonda JSON,
84 observações de valor coincidem, 44 diagnósticos coincidem, 72 observações
divergem e quatro são desabilitadas por feature. As divergências se repetem
entre perfis; **72 não significa 72 bugs**. Há ainda os controles de compilação
isolados. Nenhum desses denominadores é percentual global.

Ficam **não verificados amplamente neste passo**:

- inventário fresco e completo de funções, membros, casts, defaults e erros;
- parser/eval/morfologia do corpus vanilla completo, além dos exemplos da matriz;
- layout complexo, bidi, fontes, matemática, floats, footnotes e introspecção;
- recursos externos, pacotes, bibliografia, imagens, plugins e sandbox além das sondas;
- exportação paginada, acessibilidade e HTML em documentos variados;
- cobertura nominal dos scope-outs vigentes e comparação com as respectivas ADRs.

São lacunas desta auditoria, não declaração de que todos esses recursos estejam
ausentes, nem de que não existam testes em outros lugares do repositório.
O próximo ciclo amplo precisa enumerar superfícies e corpus atuais, definir
observadores por eixo e publicar casos sem cobertura como Unknown. Não basta
somar testes unitários, buscar TODOs ou reutilizar métricas históricas.

## Ferramenta entregue, limites e reprodução

`p1320-audit.py` implementa execução bilateral, comparação de valores tipados,
diagnósticos integrais, Unknown para falhas de harness, quatro perfis focais,
proveniência antes/depois e recusa de sobrescrita. `p1320-supplement.py` isola
controles; `p1320-close.py` verifica a cadeia e consolida estados.

Falhas de medição preservadas: a opção global `--color` estava inicialmente
depois de eval; esse lote focal é inválido. A sonda `json(json.encode(...))`
passava texto como nome de arquivo; foi corrigida para `json(bytes(...))` em
lote focal separado. Nenhum desses erros conta como defeito de paridade.
Os quatro Unknown de features desabilitadas foram classificados separadamente
somente após observar rejeição idêntica e sucesso no perfil ativo.

Testes do harness: sete testes do classificador e 41 do runner existente passam,
conforme `p1320-harness-tests-r1.json`. Os gates finais de lint e diff-check têm
saídas completas na reconciliação. Não houve build/teste Rust novo: nenhum código
produtivo mudou; o binário P1319 já validado foi reutilizado por SHA. O problema
de hash reverso do linter relatado em P1319 permanece dívida da ferramenta,
separada da paridade da linguagem; não houve resselo nem correção do linter.

Para repetir, usar nomes novos de recibo (os existentes são imutáveis), na raiz:

```bash
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1320-audit.py --self-test --output /tmp/p1320-new-tests.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1320-audit.py --output /tmp/p1320-new-audit.json
```

Os caminhos e SHAs dos binários são intencionais; reprodução exige os mesmos
executáveis e a fixture binária P1319 preservada, cujos bytes constam do recibo.
O runner principal preserva a sonda JSON original como Unknown; sua correção
está no suplemento, não foi aplicada retroativamente ao corpus medido.
Regime: diagnóstico de autor único, sem selo ou alegação de independência.
