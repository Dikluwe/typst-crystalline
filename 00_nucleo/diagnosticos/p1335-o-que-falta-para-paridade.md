# P1335 — o que falta para a paridade após P1323–P1334

## 1. O que ainda falta

A sequência recente corrigiu comportamento real, sobretudo diagnósticos e
`calc.abs`, mas não completou a linguagem. A enumeração fresca continua com
**103 rotas públicas ausentes em algum perfil**, além de divergências de
valor/representação, resolução de chamadas e diagnósticos. Isso não significa
103 funcionalidades independentes: ancestors e seus membros estão separados.

As medições abaixo pertencem ao HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, **working tree não commitada**,
congelada em `2026-09-09T16:49:00.889589+00:00`. O estado integral, o diff/stat
dos arquivos efetivamente alterados e os binários estão em
[baseline](p1335-baseline.json) e [build](p1335-build.json). Cada recibo contém
seus horários UTC, comandos e saídas; não se usa apenas a string de versão.

| Falta observada | Testemunha atual e efeito para quem usa | Limite da conclusão |
| --- | --- | --- |
| Membros públicos indisponíveis | `selector.before/after`, `outline.entry` e seus membros, `color.spot/tint`, `float.from-bytes/to-bytes/signum`, `angle.deg/rad`, `version.at`, membros HTML e `math.abs/frac/mat` constam no vanilla e falham em lookup cristalino | Lookup ausente não prova inexistência de toda a operação: uma rota global/math diferente pode funcionar. Não juntar membros que exigem novos contratos com glue interno |
| Diagnóstico de instância Int/Str | `(1).nope` e `"abc".nope`: vanilla publica `integer`/`string` e marca só `nope`; cristalino publica `int`/`str` e marca o acesso inteiro | É mensagem/origem pública, não diferença aceitável do enum Rust; o L0 já exige `integer` no exemplo canônico |
| Resolução lexical em math | `{let abs(x)=[ok]; $abs(1)$}` e a variante `sqrt`: a função local não prevalece como no vanilla | A causa precede a fórmula de `calc.abs`. Não reúne conversão de retorno numérico, spread e serializer numa correção única |
| Representação de conteúdo e Symbol | `math.liminf/limsup` usam espaço comum em vez do thin space observado; `math.Dif/dif` e os repr de `sym.quote`/`math.quote` e variantes divergem | `repr` público é língua; não se está exigindo a estrutura Rust do vanilla |
| Carregamento, encoders e closures | Permanecem diagnósticos de fonte/argumentos de loaders, XML sem `namespace`, fallback CBOR de Symbol/Content e origem de arquivo capturada em closure | As causas são distintas; sucesso de `csv` ou existência de `cbor.encode` não quita todos os casts, erros e origens |
| CLI e ordem de diagnósticos | Query rejeita `--features` no cristalino, enquanto o vanilla ratificado aceita; warnings/saída de query e help diferem. No erro de export HTML, vanilla publica erro antes do warning, cristalino o contrário | A ordem HTML cristalina é dívida expressamente preservada pelo L0 de P1323, não regressão de P1327 nem contradição nova do contrato de eval |

Os [ledgers principais finais](p1335-classification-owner-ledger-r1.tsv) e
[funcionais](p1335-classification-supplemental-ledger.tsv) conservam a lista
literal, os perfis, testemunhas, owners, fontes `file:line` e condições de
refutação. Ausência bilateral, como `csv.encode`, não recebe rótulo de membro
faltante no cristalino. Os resultados de principal são presença/kind/repr,
não testes completos de cada função.

Há também fronteiras P1334 que continuam abertas sem refutar seu fechamento:
o literal mínimo inteiro falha antes de `abs`; Float × Fraction e `path()`
sem argumento falham na construção; `sqrt`, `gradient`, `show` e `where`
possuem causas próprias. A mera ocorrência da palavra `abs` não define owner.

## 2. O que foi confirmado desde P1322

As três rodadas funcionais frescas mantêm os fechamentos no recorte declarado:

| Passo | Efeito confirmado agora | O que continua fora |
| --- | --- | --- |
| P1323 | Warning HTML com texto, três hints e parágrafo; controles de feature/target e ausência preservados | Ordem HTML com erro; caminhos CLI exclusivos não dão crédito de paridade bilateral |
| P1324 | Field ausente em nativa com namespace nomeia a função e marca o identificador; alias/With e sucessos preservados | Ordem de avaliação de `json.nope(panic(...))` |
| P1325 | Acessos diretos Dict, Content raw e Float usam a origem específica prometida | `[x].text`, Int/Str, métodos/callee e LocatedContent fora do recorte |
| P1326 | Closure/With recebe erro específico e field-only | Ordem de avaliação do argumento em callee; Plugin/Element não certificados por CLI |
| P1327 | Warning de bare import e ordem erro→warning em eval | Rename redundante, trace de import, raw serializer e export HTML |
| P1328 | Rejeição de Content por abs com união de tipos e origem | Representação sintética de LocatedContent não vira prova geral de produção |
| P1329 | Grandezas dimensionais válidas e rejeição do comprimento misto | Erros ao construir a grandeza antes da chamada |
| P1330 | Overflow do mínimo i64 construído aritmeticamente é erro, não saturação | Literal mínimo do parser |
| P1331 | Tipos rejeitados por abs recebem nomes públicos e origem | Falha do constructor de entrada não é teste do guard de abs |
| P1332 | Identidade/nome intrínseco de abs e traces nas rotas medidas | Diagnósticos das operações externas gradient/show/where |
| P1333 | Primeiro valor inválido de abs prevalece sobre sobras | Panic durante a avaliação anterior dos argumentos |
| P1334 | Missing, named `value` com hint e primeira sobra na ordem conjunta; alias/import/With/Args/spread e origem entre arquivos | Resolução de callee math e carriers sintéticos incoerentes |

Base: [reconciliação das expectativas finais](p1335-classification-preservation-reconciliation.json),
[revisão das três ordens](p1335-review-supplement-final-r3.json) e
[controle no caminho original](p1335-location-control.json).
Das **1376 expectativas** protegidas, **1332** coincidem literalmente,
**32** são alterações autorizadas por correções posteriores P1325/P1326 e
**12** coincidem literalmente no controle da localização original. O controle
destas últimas executou 36 células ao incluir repetição/inversão; não são
36 expectativas adicionais. Não houve normalização de caminhos para obter sucesso.

O classificador inicial marcou **76 células** como `CLOSED_NOW`, mas a revisão
separou **64** com fixture comparável de **12** que agora têm igualdade integral
em localização diferente. Estas últimas são confirmação atual, **não avanço
temporal causal irrestrito**. As transições originais e sua
[retificação](p1335-classification-supplemental-transitions-final-r1.tsv) ficam
preservadas. Nenhuma dessas contagens equivale a funções novas. Outros casos
mudaram de localização/rota ou foram adicionados como suplemento: não recebem
crédito temporal sem controle equivalente.

## 3. Regressões

Não foi reproduzida regressão nas observações e expectativas verificadas.
O inventário principal conserva os canais completos de todas as **18872 células**
de P1322; suas três ordens atuais também são estáveis. A sequência recente
melhorou chamadas e diagnósticos que esse inventário de nomes não exercita.

Nos suplementos, diferença em relação a uma expectativa histórica intermediária
não foi automaticamente chamada de regressão: a comparação usa seu sucessor
autorizado e, quando necessário, o caminho original. Dívidas preservadas não
viraram paridade por terem sido preservadas. A ausência de regressão neste
corpus não certifica todas as entradas possíveis da linguagem.

## 4. Cobertura, números e limites

O catálogo foi reenumerado: **4708 rotas da união estrutural pública** mais
**10 controles históricos**, formando **4718 probes**. Todos os IDs P1322 foram
reconciliados literalmente; nenhum foi removido, renomeado ou acrescentado ao
principal. A expansão finita de Symbol foi calibrada antes da execução.

Uma rodada principal = `4718 × 4 perfis = 18872 células bilaterais`.
Normal, repetição e inversão foram executadas, mas **não triplicam o denominador**.

| Classe bruta | Células por rodada |
| --- | ---: |
| MATCH_VALUE | 17960 |
| MATCH_DIAGNOSTIC | 260 |
| VANILLA_ONLY | 324 |
| CRYSTALLINE_ONLY | 168 |
| DIFFERENT_VALUE | 40 |
| DIFFERENT_DIAGNOSTIC | 120 |
| EXECUTION_UNKNOWN | 0 |
| Total | 18872 |

Igualdade bruta: `(17960 + 260) / 18872 = 96,5451%`.
São **652 células sem igualdade bruta**, não 652 causas.
Em 4533 dos 4718 probes, todos os perfis têm igualdade bruta; essa
contagem inclui controles negativos. Fórmulas, resultados por perfil,
hashes das três matrizes e estabilidade estão em [agregação](p1335-aggregate-r1.json).

O ajuste exclui somente **39 extensões fundamentadas × 4 perfis = 156 células**:
`18220 / (18872 − 156) = 18220 / 18716 = 97,3499%`.
O numerador não recebe crédito novo. Ele não altera os canais brutos nem
autoriza chamar extensão de paridade.
Lista e fórmula completas: [classificação final](p1335-classification-summary-r1.json).
`calc.deg`, `calc.rad` e `calc.log10` não possuem autorização explícita de
binding nos owners examinados e não podem desaparecer do débito por conveniência.

O suplemento funcional tem **1125 casos**, **4422 células por rodada**,
37 relações de alias preservadas e três ordens estáveis, sem Unknown.
Seus 3930 matches brutos e 492 diferenças ficam **fora** do denominador principal.
Igualdade da projeção funcional, igualdade integral dos canais e preservação
de expectativa são campos diferentes no ledger.

A amostra transversal contém **80 células por rodada**: 51 matches da projeção
escolhida, 23 diferenças e 6 gates de perfil sem execução. Nos canais integrais,
são 46 iguais, 28 diferentes e os mesmos 6 gates. Isso demonstra por que
comparar só JSON, DOM ou geometria perderia diferenças de stderr. Ver
[métricas transversais](p1335-transversal-metrics.json).

Exemplos que **não bastam** para dívida de linguagem: a amostra raster difere em
11 de 2005644 pixels, com diferença máxima de um nível por canal; o PDF tem o
mesmo texto e caixa, mas nome interno de fonte distinto. Esses números descrevem
projeções mecânicas, não fechamento nem defeito semântico. A inspeção visual de
uma página plain bilateral não viu corte, sobreposição ou glifo ausente; os dois
renders Poppler desta página coincidem. [Recibo de QA](p1335-visual-qa.json).
Não há certificação global de layout, PDF, HTML ou acessibilidade.

O suplemento de warnings possui 15 casos por ordem com o contrato cristalino
preservado. Entre eles, dois são controles exclusivamente cristalinos, dois
produzem rejeição pública de CLI vanilla e onze chegam à comparação bilateral
dos canais: sete iguais e quatro diferentes. A rejeição não prova export nem
paridade de warnings. O [overlay estrito](p1335-cli-capability-overlay.json)
reconhece somente os transcripts completos pinados de `html-legacy` e
`--document-id`; crash, truncamento ou erro genérico continuam Unknown.

Limites adicionais: a enumeração estrutural é um adapter externo validado por
CLI, não reflexão completa da linguagem. O scanner cristalino filtra candidatos
cujo eval falha; a fronteira hipotética de profundidade permanece limitação do
método, apesar dos controles finitos passarem. Suites históricas também possuem
lacunas de conteúdo, contexto, argumentos e export. Nenhum percentual deste
relatório significa “percentual de toda a linguagem Typst”.

## 5. Próximo lote e por quê

Recomendação única: **diagnóstico de field ausente em instância Int/Str**,
coorte `primitive-instance-field-diagnostic`, para eventual P1336.
Não é um novo acesso a campo nem mudança no valor de `type()`/`repr()`.

A testemunha canônica é `(1).nope`: o critério vigente de
`00_nucleo/prompts/compiler/eval/bindings/field_access.md:320` exige o nome
`integer`. Em `01_core/src/compiler/eval/bindings/field_access.rs:533`, Int/Str
não recebem field-only; em `:836`, o mesmo fallback usa `type_name()` curto.
O helper já importado `vanilla_type_name` fornece `integer`/`string` em
`01_core/src/compiler/eval/operators/error_formatting.rs:58,60`.
A hipótese de correção completa é local ao owner `field_access`, com o L0
respectivo lido integralmente e a linhagem atual conferida. O erro de string
compartilha a causa; não é uma segunda prioridade fundida por conveniência.

Ordem de decisão: regressão; contradição canônica L0; diagnóstico isolado;
valor/repr; membros ausentes; demais causas. Dentro da mesma prioridade:
menos owners **completos**, mais rotas distintas da mesma causa, menor superfície
de regressão demonstrada e ID lexical. Risco desconhecido não foi chamado de baixo.

| Coorte elegível | Prioridade | Owners completos | Rotas da mesma causa | Superfície de regressão |
| --- | ---: | ---: | ---: | --- |
| Int/Str: mensagem e field-only | 2 | 1, field_access | 2 | 1: erros locais de acesso direto |
| Callee lexical math abs/sqrt | 2 | 1, eval/math | 2 | 2: despacho e preservação de chamadas nativas |
| Trace de erro de import | 3 | 1, eval/modules | 1 | 1: trace do erro de avaliação importada |
| Warning de rename redundante | 3 | 1, eval/modules | 1 | 1: guarda localizada de rename |
| Ordem warning/erro de export HTML | 3 | 1, wiring | 1 | 2: ordem dos canais de compilação |
| Fonte ausente em cinco decoders | 3 | 2, loading + call_dispatch | 5 | 2: erro e transporte de origem/With |
| Fonte ausente em read | 3 | 2, loading + call_dispatch | 1 | 2: mensagem PathOrStr e origem |
| Escape de aspas em Symbol repr | 4 | 1, repr | 6 | 1: escape público da representação |
| Texto de liminf/limsup | 4 | 1, math stdlib | 2 | 1: texto dos operadores |
| Repr de context block | 4 | 1, repr | 1 | 2: repr e fallbacks textuais herdados |
| Namespace de nó XML | 4 | 1, loading | 1 | 2: resultado de todos os nós/nesting |

Os números de superfície são ranks ordinais, **não probabilidades de falha**.
Int/Str e math empatam em prioridade, owners e duas rotas; Int/Str vence pelo
recorte de alteração demonstrado menor. Não houve regressão reproduzida com
prioridade superior. Coortes de membros ausentes e outras pendências sem prova
de escopo/owners completos ficam inelegíveis, não descartadas do inventário.
O `cbor.encode()` sem argumento, por exemplo, não recebe automaticamente um recorte barato de
dois owners: a identidade/trace da rota indireta permanece por demonstrar.
Tabela integral, testemunhas, gates e refutações:
[seleção final](p1335-classification-selection-r1.json) e
[revisão independente](p1335-review-selection-r1.json).

Esta auditoria **não autoriza a implementação**. A materialização futura deve
medir novamente, atualizar primeiro o L0 e resselar, preservar métodos válidos,
Type::Int/Type::Str e outras variantes, testar deslocamentos e nomes de campo
novos, depois RED→GREEN e revisão segregada. A classificação provável é correção
interna de paridade ADR-0127; necessidade de outro owner/API/default/fase ou
alteração de um método válido refuta o recorte e exige reabertura.
P1335 não escreve nem implementa P1336.

## 6. Proveniência, integridade e ressalvas do auditor

O vanilla é upstream/main ratificado **a51e02804**, `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
O cristalino foi rebuildado da árvore efetiva em
`/tmp/p1335-target.EkAvyv/release/typst`, SHA-256
`11e3164fa509030cc78dc048d5bb4f2426348e32a24edc7cd2f320c704e6ef61`.
O cache medido excedia o espaço livre de `/dev/shm`; utilizou-se target exclusivo
em `/tmp`, copiado sem hardlinks mutáveis. Não houve resync do vanilla.

O baseline conserva o diff integral e este `git diff HEAD --stat` de origem:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  187 +-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |   88 +-
 00_nucleo/prompts/compiler/eval/modules.md         |   47 +-
 00_nucleo/prompts/compiler/eval/tests.md           |   78 +-
 00_nucleo/prompts/compiler/stdlib/_comum.md         |   25 +-
 00_nucleo/prompts/compiler/stdlib/calc.md           |  542 +++-
 00_nucleo/prompts/compiler/stdlib/loading.md       |  198 +-
 00_nucleo/prompts/wiring.md                        |  104 +-
 01_core/src/compiler/eval/bindings/field_access.rs |  731 ++++-
 01_core/src/compiler/eval/call_dispatch.rs         |  205 +-
 01_core/src/compiler/eval/modules.rs               |    5 +-
 01_core/src/compiler/eval/tests.rs                 |  236 +-
 01_core/src/compiler/stdlib/calc.rs                | 3067 +++++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs             |  466 ++-
 01_core/src/compiler/stdlib/mod.rs                 |    3 +-
 04_wiring/src/main.rs                              |   50 +-
 16 files changed, 5914 insertions(+), 118 deletions(-)
```

Build/testes workspace release `--locked`, `fmt --check`, linter e
`git diff --check` passaram. Foram **6734 testes aprovados, zero falhas e
três doctests ignorados**: layout_with_introspector, inject_pages e
inject_positions. O linter geral tem **zero erros, 240 warnings e 1146 infos**;
não foi alegado zero warnings. V5/V15/V26 estrito passou sem violações.
[Gates reavaliados](p1335-review-gates.json), [testes](p1335-workspace-tests.json).

A skill Tekt exigiu autores distintos para inventário, operação, classificação
e revisão, com capacidades congeladas. O regime foi **executado sem atestação
técnica de isolamento**; não é selo de refinamento. A
[dívida certificatória](p1335-classification-certification-debt.json) revalida
37 famílias históricas e mantém separadas seis famílias P1308: não houve
mutantes produtivos nem quitação dessa dívida por ataques ao auditor.

As falhas do próprio método permanecem registradas, sem apagar seus antecedentes:

- O setup de fixtures inicialmente interpretou um descritor como string; o
  sucessor corrigiu o formato. Revisões focais posteriores corrigiram hash do
  arquivo realmente compilado, expectativas de aliases e flags raw. A matriz
  funcional completa só foi executada pelo R3 validado.
- Uma repetição focal transversal R1 reutilizou os destinos R0 e **sobrescreveu
  oito PDFs temporários**. Os bytes originais não foram recuperados; os JSONs
  e canais CLI originais permanecem. Isso invalida a retenção daqueles PDFs,
  não autoriza dizer que foram preservados. O
  [incidente](p1335-transversal-retention-incident.json) lista paths/hashes.
  Após revisão do método, R3 usou destinos exclusivos que recusam reuso; seus
  artefatos completos são as evidências válidas, preservadas.
- O leitor transversal R3 deixou seis observações como Unknown por rejeições
  CLI e atribuiu indevidamente ausência de warning vanilla a erro do contrato.
  A política sucessora restrita foi congelada e atacada em cópias dos dados;
  não houve repetição da matriz nem conversão genérica de exit 2 em sucesso.
  O relatório original de Unknown continua disponível ao lado da
  [revisão sucessora](p1335-review-transversal-final-r4.json).
- O controle `$f(1)$` com nome de uma letra foi refutado como controle de chamada:
  o parser o trata como texto matemático. O sucessor com `custom` retorna corpo
  `[ok]` bilateralmente e isola resolução lexical, mantendo a diferença geral
  do serializer explicitamente aberta. [Controle sucessor](p1335-math-controls-r1.json).
- A classificação inicial tratava o erro Int como dívida sem contradição L0.
  A revisão encontrou o critério `integer` vigente; a
  [retificação causal](p1335-classification-priority-refinement.md) preserva
  a hipótese anterior e fundamenta a prioridade nova.

Produto, L0, Núcleos e evidências anteriores ao P1335 são protegidos pelo
manifesto. Nada foi implementado, staged ou commitado por esta execução.
Os **14 ataques distintos congelados ao auditor foram executados e rejeitados**,
incluindo troca de prioridade e omissão de owner sem alterar artificialmente o
rank. Eles verificam o auditor, não são mutation score do produto.

Conclusão: **auditoria válida, com recomendação única e limites explícitos**.
As observações obrigatórias finais não têm Unknown ou instabilidade; as versões
anteriores inconclusivas e o incidente dos PDFs não foram apagados. A seleção
não fecha as demais dívidas nem certifica paridade total. O
[veredito independente final](p1335-review-final.json) e o
[fechamento verificável](p1335-closure.json) pinam este relatório, os artefatos
ativos e a preservação do estado de origem. A execução só fica encerrada com
ambos presentes e válidos.
