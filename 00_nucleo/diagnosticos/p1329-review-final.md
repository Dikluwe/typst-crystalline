# P1329 — revisão final

**Veredito: PASS_SCOPED.** Nenhum achado bloqueante no fragmento autorizado.
Aprovação limitada à operação dimensional de `calc.abs`, diagnóstico de Length
misto, preservação explicitada e evidência abaixo. Não é paridade geral de
abs/calc, prova de isolamento técnico ou refinement seal.

Revisor `/root/p1329_review`; leitura do L0, baseline, fontes, candidato,
testes congelados e recibos, sem edição de produto/oráculos. Escritas restritas
a `p1329-review-*`. Regime A/B executado sem atestação técnica de isolamento.

## Identidade e proveniência

Última conferência `2026-09-09T12:49:58.454Z`; HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Os recibos root auditados contêm diff/stat e inventários integrais antes/depois.
Seus inventários coincidem com o produto atual; o mapa produtivo fora de
calc L0/owner é exatamente o baseline, incluindo conjunto de caminhos.

- baseline: `d0e1787fac8b6264122ca6dcf5e29e4729552e8031e591ce6f4ee725e14cbd23`;
- manifesto R2: `c7f5963d2735226ae3ddf653deb13856e187f723312fcfd7521c42ae5011fbe1`;
- norma recalculada: `07f83fc24dc13837f54a25f0bec6be20ff495e1f679c4975bec3f1fb583ef5ae`;
- owner canônico recalculado: `483d55d0fd0da9545f76aa3d63dd0d6f6f8f32603593476cf27cf511a051b503`;
- relatório revisado: `aaab16679a4e2d37f8190fe08f5c465aa7a9b7201da280504280e9e644300b9b`;
- recibo A/B revisado: `d8854cd422f8a98f853652e63ef681b53b8f2ea0fa3d61ed17bfce0b71b5e794`;
- binário candidato `/tmp/p1329-target.bg3p5A/release/typst`:
  `9f347f742a5cdb5c4c36a4af985b4ff122ac1bb118a1e018b960e7f5d105c2ec`.

## Candidato, causalidade e preservação

O delta funcional inspecionado restringe-se aos braços dimensionais em
`01_core/src/compiler/stdlib/calc.rs:145-168` e aos imports Abs/Length/Ratio.
Remover esses braços, reverter o import e substituir os snippets congelados
pelo histórico reconstrói exatamente o baseline, descontado o prompt-hash.
Não há mudança funcional em conteúdo, helpers, outras funções ou dispatcher.

Length usa componente abs zero OU em zero; ambos não zero rejeitam, mesmo
com o mesmo sinal. Retorno conserva espécie/componentes; Angle não normaliza
voltas, Ratio não limita percentagens e Fraction conserva espécie. Erro misto
tem mensagem completa e `value_span` da primeira ocorrência posicional, com
detached sem fabricação de origem. Guards, saturação inteira e diagnóstico
Content/LocatedContent permanecem intactos. O código satisfaz a norma R2.

A cadeia pré-C está documentada pelos pareceres L0/R2, pre-C e RED. A
reabertura sobre Scalar ocorreu antes da integração/C e produziu sucessores
de manifesto/freeze. Testes r1 e valores esperados permaneceram congelados.
RED compilou e falhou pelas lacunas dimensionais previstas; GREEN executou os
mesmos bytes e percorreu as tabelas completas. Não houve remendo de expectativa
após candidato. Recalculei todos os hashes congelados da integração.

O sucessor P1328 modifica apenas os quatro controles dimensionais e imports
necessários. Reconfirmei 112 células históricas CLI: 96 integrais preservadas,
16 dimensionais migradas para vanilla. Também conferi por hash 847 arquivos
históricos em diagnosticos pinados na cadeia de fechamento P1328. Não acessei
conteúdo histórico de materialization/context para esta revisão; a preservação
global desses artefatos deve ser conferida pelo agregador autorizado do root.

## Gates auditados

| Gate | Evidência e resultado |
|---|---|
| RED | `p1329-unit-red.json`, SHA `a675be4c1a0811e80735cfa6554e5f94ea86a0bcf1dc62d353794e3620c632ad`: build/harness válidos, 8 verdes e 7 falhas previstas |
| GREEN | `p1329-unit-green.json`: 15 passaram, zero falhas/ignorados, mesmo filtro/testes |
| Workspace | `p1329-workspace-tests.json`, SHA `14ac6ecdf719629aeb49714eaddd3ca5efa3df34a8da67276bad76405a484219`: 6.701 passaram, zero falhas, 3 ignorados, soma independente de 17 resumos |
| Build/fmt/diff | `p1329-final-build.json`, `p1329-final-fmt.json`, `p1329-final-diff-check.json`: exit 0 |
| Lint | `p1329-final-lint.json`: zero errors, 240 warnings, 1.145 infos; mensagens dos warnings iguais às de P1328 |
| Linhagem | `p1329-final-lineage.json` e `p1329-final-lineage-lint.json`: hashes bidirecionais conferidos, V5/V15/V26 sem violations |
| CLI | Três recibos A/B abaixo: 252 chaves únicas por execução, zero diferenças literais de candidato contra expected R2 |

Hashes CLI recalculados:

- normal `a97b10d1689303fe65f214b979d75e2adb443e13c25cef3b92d2eda9dbcfe410`;
- repetida `4394d37c64fe4b878f90c3fcd152eeae55f28646f9d143d700b537371ed6fcc4`;
- invertida `54ee045431a1e09b06460278edc4f1262f43534b4ad4d1eb4069313fa90d20aa`.

Comparei diretamente exit/stdout/stderr com o expected externo, não apenas
booleanos ou exit zero do runner. Chaves, repetição e ordem invertida conferem;
hash do binário coincide com build e arquivo atual. A recontagem das saídas
confirma 128 mudanças para vanilla, 12 mudanças com dívida de nome de trace,
68 igualdades anteriores preservadas e 44 dívidas anteriores preservadas.
Isso sustenta o relatório sem inflar ganho a partir de labels históricos.

## Limites ratificados

O domínio de paridade exige entradas equivalentes já construídas. NaN
dimensional cristalino não tem correspondente vanilla, cujo Scalar normaliza
NaN na construção; a obrigação nativa é local. As testemunhas públicas de
angle/ratio mostram dívida alcançável antes de abs, não mera entrada artificial.
`p1329-domain-final.json`, SHA
`0b0d4d99479590f9a57f259f92821103b8de549081a9e6a13ad2a2e816565717`,
registra essas diferenças como report-only, sem creditá-las como equivalência.
Inf foi separado de NaN; repr de Length não foi usado para deduzir componentes.

Persistem as dívidas declaradas de construção, float.nan, Float×Fraction,
overflow Int, guards, outros diagnósticos, trace externo e rota math -2pt.
O relatório e o recibo A/B descrevem essas limitações corretamente. Este PASS
autoriza somente o fechamento agregado desta implementação e destes artefatos;
não aprova novas mudanças, commit ou alteração de escopo.
