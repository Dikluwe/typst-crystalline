# P1322 — o que falta para a paridade

**Estado:** auditoria concluída com recomendação única, validada pela revisão
separada, e limitações de processo explicitadas abaixo. Não é declaração de
paridade completa. Próximo lote: **completar os hints do warning HTML**.

## O que ainda falta

O cristalino ainda não tem paridade completa com o vanilla ratificado
upstream/main **a51e02804**. Há rotas públicas ausentes, valores/reflexão
divergentes e chamadas que retornam resultado ou diagnóstico diferente.
Os avanços recentes de CSV são reais, mas não fecham esses outros eixos.

Exemplos concretos da nova medição:

- `json.nope`: o vanilla identifica a função `json` e destaca apenas `nope`;
  o cristalino omite o nome da função e destaca o acesso inteiro. O mesmo
  limite aparece em funções nativas com namespace, inclusive via `with()`.
- `repr(cbor(cbor.encode(sym.alpha)))`: o vanilla devolve a string `α`;
  o cristalino devolve a string `symbol("α")`. A diferença de bytes do
  encoder acompanha uma diferença do valor da linguagem, não apenas do formato.
- `csv(bytes("a,b\n1,2"), delimiter: sym.alpha)`: o vanilla informa que o
  delimitador precisa ser ASCII; o cristalino rejeita o tipo Symbol antes
  dessa validação. Igualdade dos outros casts não fecha esse caso.
- Uma string de caminho transportada por closure ainda pode ser resolvida
  em relação à origem errada. O documento com valor capturado passa nas
  asserções; o documento que constrói o caminho na closure falha. Isso não é
  a mesma causa que `missing` de uma função nativa CSV.
- `typst query ... --features html` funciona no vanilla e é rejeitado pelo
  parser de argumentos cristalino. A nova medição com flags efetivas também
  observa a rejeição com `a11y-extras` e com ambas as features. O diagnóstico
  CLI completo confirma a incompatibilidade; não se alega que o resultado
  JSON de query tenha sido executado ou validado nesses perfis cristalinos.
- Mesmo quando o JSON de query coincide no perfil default, seu hint de
  depreciação é genérico no cristalino, em vez de indicar a consulta e o
  arquivo reais. A exportação HTML também omite os três hints que acompanham
  o warning vanilla. Coincidência de JSON/DOM não fecha esses diagnósticos.

As testemunhas completas, incluindo fontes, argv e stderr sem supressão,
estão nas [sentinelas](p1322-sentinels-normal.json) e no
[transversal corrigido](p1322-transversal-r2.json). São diferenças observadas,
não autorização para mudar produto ou L0 neste passo.

No inventário, **103 paths** são aceitos pelo vanilla e ausentes no cristalino
em pelo menos um perfil. Entre eles estão membros HTML, rotas `math.*`,
`float.from-bytes`, `float.to-bytes`, `function.with`, `function.where` e
submembros de elementos. Uma rota ausente não determina sozinha o custo de
implementá-la: ownership completo, contrato e comportamento de chamada
precisam ser especificados. A lista integral fica no
[ledger causal](p1322-classification-owner-ledger.tsv), com o
[ledger funcional](p1322-classification-supplemental-ledger.tsv) separado.

Também permanecem diferenças de `repr`/identidade, warnings e intenção ainda
não fundamentada para `calc.deg`, `calc.rad` e `calc.log10`. A omissão de
warnings de certas variantes Symbol está explicitamente documentada no L0:
isso explica a intenção, mas **não transforma a diferença em paridade** nem
recebe crédito de extensão no cálculo ajustado.

## O que os passos recentes realmente fecharam

O [suplemento reconciliado](p1322-classification-functional-reconciliation.json)
reexecuta os fechamentos P1310–P1321 sem usar presença de binding
como substituto de comportamento. Os recortes verificados incluem casts dos
decoders/read, identificação de funções nativas sem namespace, CSV Bytes,
validação de opções e duplicatas, ordinal/posição/cause de parsing, origem de
arquivo binário inválido e precedência dos argumentos e `missing` nativo CSV.

Continuam separados: Symbol como fonte/delimitador, arquivos UTF-8 válidos
com erro e trecho externo, envelope de I/O, origem de string em closure,
import em modo eval, fronteira de namespace math e validações de outros
loaders. Em particular, **P1321 não quitou I/O** ao corrigir argumentos.

`csv.encode`, `read.encode` e `xml.encode` não são três encoders faltantes:
eles estão ausentes dos dois lados. A nova comparação confirma também a
igualdade dos diagnósticos de acesso a esses campos.

## Regressões e novas descobertas

Nas três ordens, os **2.182 probes históricos** foram preservados literalmente.
Nenhuma célula historicamente MATCH se tornou divergente. Há **12 células**
que passaram de diagnóstico diferente para igual: os três campos `encode`
acima nos quatro perfis. As [matrizes agregadas](p1322-aggregate.json) confirmam
estabilidade por chave e pelos canais completos nas três ordens, tanto no
corpus principal quanto nas sentinelas. As
[transições sucessoras](p1322-classification-transitions.tsv) não reescrevem
o ledger histórico.

Os **2.536 probes novos** são modificadores Symbol enumerados; não são
2.536 funcionalidades implementadas. Eles ampliam testemunhas de causas já
conhecidas, como escape de aspas em `repr` e warning do ancestor `math.join`.
Descobrir uma diferença num input novo não demonstra regressão temporal.

O contraste de localização isolou três diagnósticos que diferem quando o
arquivo está fora da raiz do repositório: caminho relativo no vanilla e
absoluto no cristalino. As cópias internas coincidem nos quatro perfis e três
ordens. Mantêm-se os transcripts externos; não se remove o caminho para
fabricar igualdade. A mudança de localização impede atribuir esse delta a
uma regressão desde a amostra P1320.

## Cobertura, números e seus limites

Todos os números atuais usam o mesmo estado: HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093` **com working tree não commitado**,
identificado integralmente no [baseline](p1322-baseline.json). Os quatro
arquivos de produto/L0 alterados antes da auditoria estão relacionados na
seção de proveniência abaixo; os recibos registram UTC e hashes.

| Observação principal, por ordem | Células |
| --- | ---: |
| MATCH_VALUE | 17.960 |
| MATCH_DIAGNOSTIC | 260 |
| VANILLA_ONLY | 324 |
| CRYSTALLINE_ONLY | 168 |
| DIFFERENT_VALUE | 40 |
| DIFFERENT_DIAGNOSTIC | 120 |
| EXECUTION_UNKNOWN | 0 |
| Total: 4.718 probes × 4 perfis | 18.872 |

Igualdade bruta: **18.220 / 18.872 = 96,5451%** das células do inventário.
São **4.533 / 4.718 paths** com igualdade bruta nos quatro perfis. O catálogo
reúne 4.708 rotas enumeradas e dez controles históricos não enumerados.
Esses números medem os probes de lookup/kind/repr/diagnóstico congelados,
**não a percentagem de toda a linguagem implementada**. Repetições não
aumentam o denominador.

Igualdade ajustada: **18.220 / (18.872 − 39 × 4) = 97,3499%**.
O [resumo causal](p1322-classification-summary.json) lista as 39 extensões
documentadas retiradas do denominador. As três extensões `calc` sem intenção
normativa fundamentada continuam contando como diferença, assim como os
warnings Symbol intencionalmente divergentes. Unknown continua explícito,
sem exclusão silenciosa.

O suplemento possui **816 casos / 3.186 células por ordem**, fora desse
denominador: 1.646 MATCH_VALUE, 1.052 MATCH_DIAGNOSTIC, 424 diferenças de
diagnóstico, 40 de valor, 20 VANILLA_ONLY e quatro CRYSTALLINE_ONLY.
Preservação do contrato histórico e igualdade com vanilla são projeções
distintas; diferenças brutas de transporte em casos contextuais não são
automaticamente novas dívidas semânticas.

O transversal é apenas uma amostra de 20 casos por perfil, com feature gates
explícitos. **MATCH nessa tabela é da projeção escolhida no manifesto**
(por exemplo JSON, DOM ou versão sem hash), não de todos os canais.
Na revisão R2, por ordem:

| Perfil | MATCH da projeção | DIFFERENCE da projeção | Não executado pelo gate |
| --- | ---: | ---: | ---: |
| default | 12 | 5 | 3 |
| html | 13 | 6 | 1 |
| a11y | 12 | 6 | 2 |
| html+a11y | 14 | 6 | 0 |

Essas DIFFERENCE não representam todas a mesma dívida de linguagem: incluem
os caminhos externos, a rejeição CLI e observáveis mecânicos de exportação.
Além delas, a [conferência dos canais completos](p1322-transversal-channels.json)
identifica **17 células por ordem** rotuladas MATCH pela projeção, mas com
stdout/stderr diferente. Elas incluem hints de query/HTML, texto de help,
o hash de build em `--version` e um controle que compara apenas a mensagem
primária, sem o caminho externo. Todos esses deltas são estáveis e permanecem
registrados; o hash de build não é dívida de língua, enquanto hints não podem
ser descartados para alegar paridade diagnóstica. Esse complemento não altera
a matriz principal nem seu denominador.

No raster medido, há 11 pixels diferentes entre 2.005.644, com delta máximo
de um canal; no PDF, ambos têm uma página, mesmo tamanho e texto, mas o nome
interno da fonte difere. Isso não prova falha de semântica/morfologia. A
[inspeção visual](p1322-visual-qa.json) examinou a página simples dos dois PDFs,
sem clipping ou deslocamento visível; **não certifica layout/export global**.

As oito sondas adicionais de fronteira CSV/JSON coincidem nos quatro perfis
e três ordens. As duas compilações de closure observam asserções, não
igualdade global do documento renderizado. Nenhuma dessas amostras infla o
catálogo principal.

## Próximo lote

A [recomendação sucessora](p1322-classification-selection-r2.json) foi
**reaberta pela revisão de substância**, conforme a
[retificação causal R2](p1322-classification-query-hint-addendum-r2.md). A
[seleção inicial](p1322-classification-selection.json) escolhia diagnósticos
de seis funções com namespace; ela não é a decisão final.

A causa prioritária encontrada nos canais completos é o **warning experimental
de HTML sem seus três hints**. `04_wiring/src/main.rs:388–392` imprime apenas
a headline. O vanilla, em `lab/typst-original/crates/typst/src/lib.rs:249–256`,
inclui os três hints. O L0 `00_nucleo/prompts/wiring.md:171–173` já exige o
warning experimental medido no vanilla, e `:85` exige diagnóstico humano
vanilla-espelhado. A ausência contraria essa promessa específica: **prioridade
2**, antes de qualquer desempate por quantidade de paths. Exigir que o L0
repetisse literalmente cada hint para reconhecer a contradição teria
enfraquecido indevidamente o contrato.

O recorte proposto é a emissão fixa do warning HTML no owner
`04_wiring/src/main.rs`, com L0 proprietário `00_nucleo/prompts/wiring.md`:
preservar feature gate, target, conteúdo/serialização HTML, demais formatos e
pipeline. Grafia legada e `compile` explícito são testemunhas da mesma causa,
não dois paths causais. O futuro passo deve delimitar a obrigação no L0 antes
de código e confirmar RED→GREEN, incluindo controles de ausência do warning.
Necessidade de formatter/API adicional refuta a suficiência do owner único
e exige reabrir o escopo/gate; este diagnóstico não autoriza implementação.

Comparação das primeiras alternativas elegíveis:

| Coorte | Prioridade | Owners completos | Paths com a mesma causa |
| --- | ---: | ---: | ---: |
| Hints do warning experimental HTML | 2 | 1 | 1 |
| Campo ausente em nativa com namespace | 3 | 1 | 6 |
| Warning do ancestor `math.join` | 3 | 1 | 4 |
| Span de field em targets não Module | 3 | 1 | 3 |
| Campo ausente em closure | 3 | 1 | 1 |
| Escape de aspas em repr Symbol | 4 | 1 | 6 |

A [tabela inicial](p1322-classification-cohorts.md) conserva as alternativas
anteriores e seus limites; a seleção sucessora incorpora a causa HTML e
compara **128 coortes, 17 elegíveis** sob os critérios declarados.
Os seis paths `assert`, `cbor`, `json`, `table`, `toml` e `yaml` continuam como
próxima alternativa de prioridade 3. Seu L0 field_access preserva namespace
Some na fronteira P1311: também exigiria alteração explícita antes de código.

A validação de CBOR não foi tratada como um reparo barato de um
owner: chamadas indiretas também expõem a identidade qualificada em traces.
I/O, origem externa/closure e novos contratos CLI permanecem com escopo ou
risco não demonstrado; Unknown de custo não foi convertido em baixo risco.

A regra explícita é: regressão; contradição L0 em rota canônica;
diagnóstico isolado; valor/kind/identidade/repr; membros ausentes com carriers;
membros que exigem contrato/entidade/fase nova; restante.

Dentro da prioridade, o desempate usa menos owners necessários ao observável
completo, mais paths com causa demonstrada, menor superfície de regressão
demonstrada e ID lexical. Risco desconhecido não é baixo. Nenhum P1323 foi
escrito ou implementado por esta auditoria.

## Proveniência, verificação e incidentes

O [manifesto](p1322-manifest.json) congela papéis separados de inventário,
operação, classificação e revisão. A skill de materialização segregada
orientou essa separação e os ataques em cópias dos dados do auditor.
**Não há atestação técnica de isolamento nem selo implícito de independência.**
Nenhum mutante foi executado sobre o produto.

O estado protegido inicial, preservado pelos recibos de execução, contém
estas alterações anteriores: 756 linhas adicionadas e 53 removidas no total.

- `00_nucleo/prompts/compiler/stdlib/loading.md`
- `01_core/src/compiler/stdlib/loading.rs`
- `00_nucleo/prompts/compiler/eval/call_dispatch.md`
- `01_core/src/compiler/eval/call_dispatch.rs`

O build novo usou `cargo build --workspace --release --locked` no target
exclusivo `/tmp/p1322-target.Ir19xI`, a partir de cópia de cache sem hardlinks.
O espaço livre de `/dev/shm` era menor que o cache; por isso foi usado `/tmp`.
O [recibo de build](p1322-build.json) registra o executável cristalino SHA-256
`756b1c85a5879ea0afb79fc35184525aebfafa6ccb926ae86679a868691f99fa`.
Vanilla: `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Strings de versão não foram usadas como identidade.

Gates já executados:

- [Testes workspace release](p1322-workspace-tests.json): **6.666 passaram,
  zero falharam, três ignorados**.
- [Lint](p1322-lint.json): zero erros, **240 warnings e 1.138 infos**, mantidos
  explicitamente; não se alega ausência de todos os avisos.
- [V5/V15/V26](p1322-lineage.json): zero violações com fail-on warning.
- [Fmt](p1322-fmt.json): passou sem modificar arquivos.
- [Git diff --check](p1322-diff-check.json): passou; estado protegido idêntico
  ao baseline não commitado.

A revisão separada confirmou [matriz principal](p1322-review-runtime-final.json),
[sentinelas](p1322-review-supplement.json),
[projeções transversais R2](p1322-review-transversal-r2.json),
[canais completos complementares](p1322-review-channels.json),
[classificação congelada](p1322-review-semantic-final.json) e
[gates](p1322-review-gates.json). Foram **13/13 ataques planejados válidos
rejeitados**, em cópias do auditor: [dois de catálogo](p1322-review-attacks-catalog.json),
[cinco de runtime](p1322-review-attacks-runtime.json) e
[seis de semântica/seleção](p1322-review-attacks-semantic-r1.json).
Repetições e controles adicionais não aumentam esse score; não é mutation
score do produto. A rejeição CLI exit 2 recebeu
[controles específicos](p1322-review-query-exit2-controls.json): não houve
aceitação genérica de crash, saída truncada ou exit desconhecido.
A [revisão da seleção R2](p1322-review-selection-r2.json) repetiu D12 com
HTML como controle e a seleção Some anterior como mutante de prioridade:
rejeitado, sem aumentar o score. O
[veredito final](p1322-review-final.json) e seu
[resumo](p1322-review-final.md) julgam a recomendação HTML, não a seleção
supersedida. A [preservação final](p1322-review-preservation-final.json)
confirma os 3.912 arquivos protegidos, 126 evidências iniciais, seis inputs
históricos, HEAD/diff/staged e o passo congelado.

Incidentes preservados, sem sobrescrever medições:

1. A primeira expansão cristalina do inventário combinou indevidamente
   modificadores já expandidos. Foi invalidada antes da observação CLI e do
   catálogo final. O inventário R2 corrigiu a causa e possui controles focais;
   as combinações artificiais não contam como superfície pública.
2. O executor transversal herdado só acrescentava features em alguns
   comandos. A revisão detectou rótulos de perfil sem flags reais. Os
   [recibos R1](p1322-transversal.json) foram preservados; a
   [revisão R2](p1322-transversal-r2.json) injeta as flags reais, preserva os
   gates e reexecuta as três ordens, incluindo o contraste de localização.
   Os extras/closures R1 já tinham flags efetivas e não foram invalidados.
3. Uma hipótese de escape incorreto nas sondas adicionais foi refutada por
   igualdade textual exata. A repetição já iniciada foi preservada como
   redundante, sem crédito adicional; o motivo incorreto de invalidação foi
   [formalmente retratado](p1322-boundary-hypothesis-retraction.json).
4. A primeira versão do resumo causal contou as 49 linhas suplementares do
   ledger histórico como probes principais: 2.231 anteriores / 2.487 novos,
   em vez de 2.182 / 2.536. O classificador corrigiu seus nove artefatos antes
   do veredito, mas substituiu a versão inicial em vez de preservá-la em
   arquivos sucessores. Essa versão inicial chegou a ser lida pelo operador;
   não se alega sua preservação integral. A revisão usa o conjunto corrigido
   e [congelado](p1322-classification-freeze.md), com
   [retificação explícita](p1322-classification-correction.md). Catálogo,
   matrizes, fontes e evidências históricas não mudaram.
5. O leitor TSV da revisão interpretou aspas de expressões como delimitadores
   de campos e comparou quebras de linha sem considerar a representação do
   ledger. A [calibração focal](p1322-review-semantic-calibration.md) corrigiu
   somente o leitor, com comparação da fonte literal. Não foram alteradas
   expressões de teste, saídas dos compiladores ou expectativas do produto.
6. A conferência final explicitou o alcance estreito dos MATCH transversais:
   JSON/DOM/mensagem primária iguais não significam stdout/stderr íntegros
   iguais. O complemento de canais preserva as 17 diferenças por ordem e
   acrescenta sua análise, sem reexecutar ou sobrescrever a medição.
7. A revisão rejeitou a prioridade 3 inicialmente atribuída aos hints HTML.
   O L0 já promete o warning vanilla medido: a classificação como contradição
   L0, prioridade 2, muda o vencedor. A seleção inicial e o primeiro anexo
   foram preservados; não se manteve o lote de seis funções por conveniência.

A dívida histórica de certificação/adversarial permanece separada da dívida
de língua no [ledger próprio](p1322-classification-certification-debt.json).
As **37 famílias históricas** sem quitação continuam pendentes. Os ataques
atuais ao auditor não quitam famílias de mutantes do produto; os mutantes
M01–M06 de P1308 pertencem a outro recorte.

**Limite desta entrega:** diagnóstico e seleção de trabalho futuro, sem
alterar produto/L0, sem stage/commit/push e sem apagar temporários ou
evidências anteriores. O passo de execução congelado permanece como plano;
este relatório e o veredito sucessor registram sua execução.
