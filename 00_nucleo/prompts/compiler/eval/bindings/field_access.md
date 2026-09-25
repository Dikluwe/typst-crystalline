# Prompt L0 — `compiler/eval/bindings/field_access` — acesso a campo sobre valores e `Content`
Hash do Código: 2b90802f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## P1484 — `std.asset` gated (CONFIRMADO; ADR-0127)

### Medição anterior à decisão

No HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`,
`01_core/src/compiler/eval/bindings/field_access.rs:516-529` trata
somente `format.html`; `:571-583` trata os três fields PDF a11y.
O vanilla pinado `586e1bd43` rejeita `type(std.asset)` sem feature
com ``cannot access field `asset` because the `bundle` feature is not
enabled`` no identificador `asset`, mais os hints de habilitação e ajuda;
com `bundle` devolve `function`. O cristalino pré-P1484 devolve função
nos dois perfis porque a stdlib contém o placeholder. Um campo homônimo
em Dict ou Module alheio não é o binding gated do std efetivo.

### Decisão e sucessão

Quando o target é o módulo `std` efetivo e `Bundle` está ausente,
aceder ao field `asset` emite exatamente o diagnóstico gated medido,
com span somente no field e os dois hints. O gate não intercepta
Dict, módulos arbitrários nem outro field; não executa o placeholder.
Com `Bundle`, P1484 ainda deixa `std.asset` ausente/RED, pois P1485
substitui a assinatura e o target bundle. O lookup normal de field ausente
aplica-se até essa substituição. Esta obrigação complementa o gate de
identificador global em `00_nucleo/prompts/compiler/eval.md` e sucede qualquer disponibilidade
incondicional herdada de P397; não declara paridade no perfil ativo.


## P1444 — superfície de campo de `MathPrimes` (MATERIALIZADO)

### Medição anterior à decisão

No vanilla, `math.primes(3).fields()` é `(count: 3)`, `has("count")` é
`true`, `at("count")` é `3`, e `body` permanece não declarado. O fallback
genérico cristalino procura apenas `body` ao enumerar campos.

### Decisão proposta

Para `Content::MathPrimes`, enumerar somente `count`, nessa ordem, delegando o
valor ao `get_field` do carrier. `has/at/fields` compartilham essa declaração;
campo diferente continua ausente e defaults de `at` permanecem gerais. Não
fabricar `body`, glyph ou campo derivado.

## P1339 — descoberta das nove rotas complementares

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto:
`field_access.rs:708-801` já delega Float à stdlib e famílias primitivas ao
dispatcher. As sondas congeladas `p1339-full-final-*` e
`p1339-full-boundaries-*` distinguem descoberta estática de extração de
método ligado, com comandos/UTC/spans/árvore próprios. A leitura de owners
e propostas de ligação estão na medição histórica.

### Decisão proprietária

O match de Value::Type descobre Angle deg/rad e Function with pelos wrappers
internos de call_dispatch, Float inf/nan/signum/from-bytes/to-bytes pelo
owner foundations/float, Version at por version_type_field na fachada stdlib.
Constantes Float permanecem valores, funções têm nome público curto.
Não escrever fórmula, cast, parser de Args ou construção de With neste owner.

Não expor deg/rad sobre valor Angle como função ligada: acesso sem chamada
conserva `cannot access fields on type angle` no field. Float signum/to-bytes
sem chamada continua com a rejeição correspondente do tipo float.
`calc.pow.with` sem chamada continua sendo field ausente de pow, não um
método capturado. `version(...).at` sem chamada continua componente
desconhecido, sem promover at a major/minor/patch. Valores homônimos em
Dict/Module e namespaces de funções permanecem campos ordinários.

Não instalar namespace reflexivo em instâncias nem ampliar membros futuros
implicitamente. Where continua regido por seu adendo específico. Esta
extensão sucede somente a ausência das nove rotas listadas; os demais
diagnósticos, estilos, features, campos e scope-outs mantêm seus owners.

## P1338 — field ausente em instância Array

### Medição anterior à decisão

A medição histórica
concluído em `2026-09-09T20:06:19.061817+00:00`, mede `(1,2).nope`,
`().absent` e alias Unicode multilinha: o cristalino publica
`array does not contain field "<field>"` sobre o acesso inteiro; vanilla
publica `cannot access fields on type array` somente sobre o identificador.
Baseline,
working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, diff/stat e inventário exatos.
Vanilla ratificado upstream a51e02804;
antecedente P1337.

`01_core/src/compiler/eval/bindings/field_access.rs:532–551` omite Array da
seleção field-only; `:670–679` possui ramo próprio com len/first/last e erro
genérico local. O pré-despacho `:468–481` pode retornar antes do lookup:
`().first` já erra por array vazio ali, enquanto o lookup puro retorna None.
`(1,2).len`/last divergem em disponibilidade, mas len()/first()/at(1) e
array.len((1,2)) coincidem. array.nope é Type, não instância; chamada com
panic expõe ordem anterior ao lookup. Comprimento mantém dívida própria.
Vanilla `lab/typst-original/crates/typst-eval/src/code.rs:347–366` fornece
field.span(); `lab/typst-original/crates/typst-library/src/foundations/value.rs:157–169`
delega Array a fields e `foundations/fields.rs:14–67` não lhe dá fields.
A checagem extra refuta tratar a correção diagnóstica como paridade total Array.

### Obrigação e sucessão restrita

Quando Value::Array chega ao lookup deste owner com field distinto de len,
first e last, emitir exatamente `cannot access fields on type array`,
independentemente de conteúdo/tamanho do array ou spelling do field. Não
interpolar nome arbitrário, consultar fixtures ou fabricar valor/método.
O acesso AST que chega a esse lookup fornece todos e somente os bytes de
access.field().span(), também com alias, parênteses, Unicode e multilinha.
O lookup puro conserva exatamente o span recebido, inclusive detached/vazio.
Manter erro único, severidade error, hints/laterais e traces causais existentes.

Esta obrigação sucede expressamente a preservação de erro/âncora Array em
P1337/P1336 e nas cláusulas gerais de preservação dos reparos anteriores,
somente nesse caminho. Migrar apenas a expectativa e o nome de
`p1337_preserve_array_ast_message_and_total_span_debt`, conservando seu caso
e comparadores. Todo outro teste anterior continua byte a byte.

Preservar lookup puro len (comprimento), first/last (valor clonado ou None
quando vazio). A AST mantém o pré-despacho existente, mesmo quando devolve
erro por array vazio ou difere do lookup puro: não removê-lo nem expandi-lo.
Disponibilidade de métodos como valor, chamadas ligadas/estáticas, consumo
Args, ordem/panic, Type::Array, Length e outras categorias permanecem intactos.
Bool/None/Auto e Int/Str mantêm a paridade já contratada. Dict, Content e
LocatedContent, Module, funções nativas/closures/With, Float, PDF/features,
text contextual e warnings conservam as obrigações vigentes.

Língua: mensagem e origem do diagnóstico, sob ADR-0107/0108. Enum, match e
forma de selecionar o span são mecânica, não igualdade de linguagem.
Não atribuir intenção histórica ao vanilla. É inferência que este owner basta:
outro owner causal, origem irrecuperável ou necessidade de mudar API/default/
fase refutam-na e exigem nova medição e classificação antes de ampliar código.
Correção interna ADR-0127 contínua, L0 primeiro + revalidação + RED→GREEN.

Aceitação: testes independentes puro/AST, arrays vazios/populados/heterogêneos,
aliases/Unicode/multilinha, diagnóstico completo e positivos/exclusões;
matriz bilateral pré-classificada em quatro perfis e três ordens, mutantes
realmente compilados e rejeitados por testemunhas. Unknown obrigatório bloqueia.
Este contrato é deliberadamente limitado ao erro ausente; não declara concluída
a paridade de fields Array nem a dívida de len/first/last e pré-despacho.

## P1337 — field ausente em Bool, None e Auto

### Medição anterior à decisão

A medição histórica
concluído em `2026-09-09T19:09:59.299611+00:00`, mede `true.nope` e
`false.ausência`: cristalino publica bool, vanilla boolean. `none.nope`,
`auto.nope` e aliases multilinha já têm o nome correto, mas, como Bool,
marcam o acesso inteiro em vez do field. Valores válidos coincidem.
Baseline,
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada
com diff/stat e inventário exatos. Vanilla ratificado upstream a51e02804;
antecedente cristalino.

`field_access.rs:532–547` omite Bool/None/Auto da seleção field-only;
`:843–846` usa type_name no fallback. `operators/error_formatting.rs:55–57`
já distingue none/auto/boolean, sem alterar nomes de type/repr. Vanilla
`typst-eval/src/code.rs:347–366` transmite field.span(). O teste legado
`field_access.rs:1795–1803` preserva precisamente os três diagnósticos antigos.
Array tem ramo próprio em `:667–677`; bool.nope usa Value::Type, não Bool.
Ambos divergem, mas não são a mesma obrigação. A medição de chamada com panic
mostra causa anterior ao lookup, também excluída.

### Obrigação e sucessão

Quando Bool, None ou Auto chegam ao lookup deste owner, o diagnóstico é
respectivamente `cannot access fields on type boolean`,
`cannot access fields on type none` e `cannot access fields on type auto`.
Qualquer valor Bool, alias ou nome de field obedece à regra. O acesso AST usa
todos e somente os bytes de access.field().span(), inclusive Unicode,
parênteses e multilinha; o lookup puro conserva exatamente o span recebido.
Manter erro único, severidade error, ausência de hints/laterais novos e traces
causais existentes. Não fabricar campo, valor ou método, não alterar type/repr.

Esta obrigação sucede explicitamente as preservações de Bool/None/Auto em
P1336 e nas cláusulas gerais P1301/P1303/P1306/P1311/P1324/P1325/P1326,
somente no acesso que chega ao lookup. Migrar somente as três expectativas
de `p1336_preserve_other_fallback_names_and_ast_span` para os nomes/spans
acima e renomear o teste como sucessor P1337; não apagar casos nem relaxar
comparadores. Demais testes e a evidência antecedente permanecem intactos.

Int/Str P1336 conservam sua obrigação; Array, valores-tipo (inclusive Bool),
pré-despacho, field_callee_error, ordem de argumentos/panic, valores válidos,
Dict, Content/LocatedContent, Module, nativas/closures/With, Float/is-nan,
PDF/features, text contextual e warnings não mudam. Não generalizar field-only
nem nomes longos a outras categorias; não mudar entidade, owner, API, default,
compatibilidade ou fase do pipeline.

Mensagem e origem são observáveis de linguagem ADR-0107/0108; enum e algoritmo
Rust são mecanismo. Não inferir intenção histórica do vanilla. É inferência
que o discriminante e o span existentes neste owner bastam; necessidade de
outro owner, origem irrecuperável ou alteração fora deste fragmento refutam
o recorte e exigem reabertura antes de ampliar o código. Correção interna de
paridade ADR-0127: L0 primeiro, revalidação, RED→GREEN e revalidação contínua.

Aceitação: testes independentes puro/AST e matriz bilateral congelados antes
de C, positivos, fronteiras e dívidas pré-classificadas, quatro perfis e
três ordens; mutantes aplicáveis compilados e rejeitados por testemunhas.
Unknown obrigatório bloqueia. Não declarar paridade geral nem somar
preservação de dívida como convergência.

## P1336 — field ausente em instância Int/Str

### Medição anterior à decisão

A medição histórica
concluído em `2026-09-09T17:56:26.736676+00:00`, preserva comandos, fontes,
UTC e canais integrais. Baseline P1336,
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada
com diff/stat e arquivos exatos. Vanilla ratificado upstream a51e02804;
cristalino antecedente.

`(1).nope`, `"abc".nope` e aliases multilinha/Unicode recebem nomes `int`/`str`
e âncora do acesso inteiro no cristalino; vanilla publica `integer`/`string`
e ancora só o field. `field_access.rs:533–542` exclui Int/Str de field-only;
`:834–837` usa type_name() no fallback. O helper vanilla_type_name já importado
em `:28` possui os nomes longos em `operators/error_formatting.rs:58,60`.
Vanilla `typst-eval/src/code.rs:347–366` fornece field.span(). O critério
canônico deste L0 para `(1).foo` já exige `integer`; não foi revogado pelas
preservações dos reparos estreitos anteriores. `true.nope` e `int.nope`
medem dívidas próprias, enquanto `"abc".len()` mantém o valor válido.

### Obrigação, sucessão e limites

Ao chegar ao fallback de lookup deste owner com `Value::Int` ou `Value::Str`,
o erro deve ser exatamente `cannot access fields on type integer` ou
`cannot access fields on type string`, respectivamente, qualquer que seja o
valor, nome do field ou alias. Não consultar ortografia da fixture nem mudar
os nomes de type()/repr(). O acesso AST fornece todos e somente os bytes de
access.field().span(), excluindo receiver, ponto, parênteses e whitespace;
o lookup puro conserva exatamente o span que recebeu, sem reconstruir origem.
Preservar erro único, severidade error, ausência de hints/laterais novos e
os traces causais existentes, sem criar membro, callable ou valor substituto.

Esta obrigação sucede expressamente a preservação de mensagem/span Int/Str
nas cláusulas gerais P1301/P1303/P1306/P1311/P1324/P1325/P1326, somente no
acesso que chega a este lookup. Pré-despacho de métodos, gates antecipados,
field_callee_error e ordem de avaliação permanecem intactos: uma falha anterior
ao lookup não é corrigida nem recebe crédito de paridade. Métodos Int/Str
existentes e os namespaces Type::Int/Type::Str não mudam. Preservar Bool e
outras variantes, Module, Dict, Content/LocatedContent, Float/is-nan, funções
nativas Some/None, closures/With, PDF/features, text contextual e warnings.
Não generalizar nomes longos ou field-only ao fallback de outros tipos.

Mensagem/origem são observáveis de linguagem (ADR-0107/0108); representação
Rust e algoritmo são mecanismo. A intenção normativa vem deste contrato de
lookup, não é inferida como intenção histórica do vanilla. É inferência que
AST e helper já disponíveis bastam neste owner; outro consumer, origem
irrecuperável, efeito em método válido ou nova API/default/fase refutam o
recorte e exigem reabrir o escopo antes de ampliá-lo.

Classificação ADR-0127: correção interna de paridade em fluxo contínuo,
L0-first + revalidação, RED→GREEN e revalidação. Aceitação exige testes locais e
A/B independentes congelados antes do candidato, nomes/valores variados,
aliases, parênteses, multilinha e Unicode, lookup puro, sucessos e exclusões;
comparação integral nos quatro perfis, repetição/inversão e mutantes aplicáveis
rejeitados. Unknown obrigatório bloqueia; preservação de uma dívida não
significa igualdade com vanilla nem fechamento geral de fields.

## P1326 — field ausente em closure definida pelo usuário

### Medição anterior à decisão

A medição histórica
concluído em `2026-09-09T01:23:32.345952+00:00`, preserva HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado,
lista exata/diff/stat dos dez arquivos tracked alterados e comandos/saídas.
Baseline P1325;
vanilla ratificado `a51e02804`.
Closure nomeada, anônima, alias, With aninhado, multilinha e callee benigno
emitem `cannot access fields on type function` com span total; vanilla emite
`cannot access fields on user-defined functions` somente sobre o field.
Chamada normal e With mantêm valor. `f.nope(panic("arg"))` expõe divergência
anterior de ordem: cristalino avalia o argumento primeiro; ela fica fora.

`01_core/src/compiler/eval/bindings/field_access.rs:502–510,652–664`
seleciona span e mensagem; `01_core/src/entities/func.rs` já conserva Closure
e With. Vanilla `lab/typst-original/crates/typst-library/src/foundations/func.rs:280–308`
nega fields quando a função não possui scope; sua AST fornece field.span().
As expectativas embutidas P1311/P1324 deste owner ainda preservam a mensagem
antiga para Closure agrupada com Element/Plugin; precisam de sucessão restrita.

### Obrigação e sucessão

Correção interna de paridade diagnóstica, fluxo contínuo ADR-0127.
Mensagem e origem são observáveis da linguagem (ADR-0107/0108), não igualdade
Rust. Para `FuncRepr::Closure`, inclusive através de qualquer cadeia With,
quando o acesso chega ao lookup de fields deste owner, emitir exatamente
`cannot access fields on user-defined functions`, independente de nome,
alias ou field. Acesso AST usa todos e somente os bytes de access.field();
lookup puro respeita o span recebido. Preservar erro único, severidade,
hints/laterais e traces existentes. Não fabricar namespace nem executar a
closure para determinar a categoria. Helper privado pode atravessar With
e consultar o discriminante existente sem mudar assinatura ou entidade.

Substituir expressamente as proteções P1311/P1324 de mensagem e span de
Closure/With neste owner, inclusive somente as expectativas correspondentes
dos testes embutidos. Separar Closure dos controles Element/Plugin sem
remover estes, relaxar comparadores ou alterar suas expectativas. Preservar
todas as demais obrigações, em particular nativas Some/None/With, Module,
Dict/Content raw/Float P1325, LocatedContent, Type, PDF, text contextual,
warnings, disponibilidade e valor/args/kind/chamada de funções existentes.

Não generalizar a Plugin ou Element; não alterar pré-despacho de métodos,
gates antecipados, field_callee_error, ordem de avaliação ou pipeline.
Uma chamada de field que alcance este lookup recebe a correção; falha
anterior no argumento continua baseline, explicitamente sem crédito de
paridade. Não corrigir essa ordem no owner call_dispatch neste recorte.
É inferência que categoria e AST locais bastam; necessidade de outro
carrier/owner funcional, API/default/fase ou divergência fora desse fragmento
refutaria a suficiência. Não atribuir intenção histórica ao vanilla.

Aceitação: testes A/B e sucessores legados derivados deste L0 antes de C,
RED real, positivos pareados, nomes/aliases/With/Unicode/multilinha,
fronteiras excluídas e comparação integral normal/repetida/inversa nos
quatro perfis. Classificar todo baseline antes de C; Unknown obrigatório
bloqueia. Preservação de residual não implica paridade geral da linguagem.

## P1325 — âncora do acesso direto em Dict, Content raw e Float

### Medição anterior à decisão

Em `2026-09-09T00:49:40.661695+00:00`, a medição histórica
preserva HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não
commitado, lista exata/diff/stat dos oito arquivos tracked alterados, comandos,
horários, saídas e hashes dos executáveis. Baseline P1324;
vanilla ratificado upstream `a51e02804`.

`01_core/src/compiler/eval/bindings/field_access.rs:502–510` seleciona span
total para Dict, Content raw e Float salvo is-nan; `:518–547` consome o span
recebido nos erros de lookup de Dict/Content. Os acessos `(x: 1).nope`,
`[x].nope`, `strong[x].absent`, `(1.0).nope`, alias e alias multilinha
produzem a mesma mensagem que o vanilla, mas sublinham o acesso inteiro,
não apenas o identificador. `(1.0).is-nan` já coincide. Lookup de chave
presente e `strong[x].body` coincide; `[x].text` mede uma ausência de campo
preexistente exclusiva do cristalino, que NÃO será corrigida por este nó.
A chamada `(x: 1).nope(2)` segue diagnóstico de método separado e coincide.

### Classificação, sucessão e obrigação

Classificação `ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`: âncora diagnóstica é
observável da linguagem, não igualdade mecânica do Rust (ADR-0107/0108).
Correção interna, sem API, entidade, default, feature, compatibilidade ou fase
nova. L0-first, revalidação, RED→GREEN e revalidação em fluxo contínuo.

Quando a avaliação direta de campo chega à delegação para
`eval_value_field_access` com `Value::Dict`, `Value::Content` raw ou
`Value::Float`, fornecer o span de todos e somente os bytes de
`access.field()`. Não incluir receiver, ponto, parênteses ou whitespace,
nem truncar campos longos/Unicode; aliases e deslocamentos de linha não
mudam a regra. Preservar mensagem, severidade, cardinalidade, hints, traces,
ordem de avaliação, lookup, valor, kind e morfologia. Para campos que ainda
divergem em disponibilidade, a única mudança autorizada é a mesma âncora:
isso não transforma erro em sucesso nem comprova paridade de lookup.

Esta obrigação substitui EXPRESSAMENTE a proteção histórica de span total
de Dict e a preservação indiscriminada de targets não Module em P1301,
P1303, P1306, P1311 e P1324, somente para os três variants raw nomeados,
na delegação de acesso direto. Float/is-nan continua field-only.
Não abrange `Value::LocatedContent`, os erros de `field_callee_error`,
pre-dispatch de métodos/chamadas, outros tipos ou gates antecipados.
As proteções anteriores de Module, nativas, PDF, warnings e features ficam.
Nenhum helper público novo, mudança de owner, carrier, entidade, wiring,
CLI, lab, render ou pipeline é autorizada.

É inferência que a seleção local de span basta; outro owner causal, mudança
de lookup/mensagem ou necessidade de provenance adicional a refutaria e
reabriria a classificação antes de ampliar código. Não atribuir intenção
ao vanilla. Aceitação limitada à âncora nas três categorias: testes A/B
independentes congelados antes de C, RED real, positivos e fronteiras,
comparação completa das saídas aplicáveis, repetição e ordem inversa.
`Unknown` obrigatório bloqueia; divergências residuais devem ser nomeadas
antes de C e não contadas como paridade. Não implica paridade geral.

## P1307-R5 — snapshot de conteúdo consultado (proposta; gate ADR-0127 pendente)

### Medição anterior à decisão

Baseline histórico R5:
HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não
commitado com diff/stat integral. A medição independente R5,
preserva fontes, horários e executáveis; referência upstream `a51e02804`.

`field_access.rs:589–657,731–768` consulta campos raw de Heading;
`:808–832` extrai receiver estático descartando tudo salvo Content/Location.
R5 access.labelled/unlabelled prova presença de label causal; default/unknown
não equivale a sintetizar um campo ausente.

### Decisão proprietária

Para LocatedContent Some, acesso direto, has, at e fields leem o mapa congelado
como fonte autoritativa. Fields devolve Dict na ordem preservada; has testa
presença; at usa o valor mesmo quando None/Auto, recorrendo ao default somente
se a chave não existe. Sem default, conservar mensagem, span e consumo Args
contratados. Não procurar um campo ausente no raw. Func deriva do elemento;
location usa a Location exata. None mantém a projeção preexistente, inclusive
Equation realizada. Raw Content conserva máscara de presença P829.

Acrescentar um helper interno de método recebendo `&IntrospectedContent`,
Location, método, Args e Span, com mesma visibilidade crate do helper vigente.
Ele reutiliza a validação comum e seleciona fields; os helpers existentes de
Content continuam chamáveis. O receiver estático privado conserva o Value
completo até a escolha do helper. Não aumentar a API pública externa nem
fazer I/O/contexto; não duplicar validação Args ou perder seus spans.

Formas de instância, estática e alias seguem a mesma seleção; content.func,
fields, has, at e location não fazem lowering de render. P1306 Module e os
diagnósticos anteriores permanecem intactos. Aceitação: labelled/unlabelled,
missing com/sem default, numbering None presente, clones e estático/instância.
Esta seção substitui a delegação incondicional ao raw de P1151.

---


**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/bindings/field_access.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/bindings.md`
**ADRs**: ADR-0107 (paridade língua), ADR-0026 (`Content` como enum fechado)

---

## Contexto

Este nó resolve `a.b` como **leitura**: campo de dicionário, campo sintético de
um tipo primitivo (`.days` de uma duração, `.major` de uma versão, `.em` de um
comprimento), campo de `Content`, e os métodos de `Content`. É o caminho de
r-value, simétrico ao l-value de `compiler/eval/bindings/access.md`.

No vanilla isto está distribuído por `foundations::value::field()` e
`Content::field()`; não há um ficheiro correspondente em `typst-eval`.

`eval_field_access` é a função de maior churn de todo o `bindings.rs`: 20
commits com mudança de corpo (critério 3), contra 8 do segundo colocado. É o
ponto onde cada tipo novo da língua ganha os seus campos.

## Restrições Estruturais

- L1 puro. `eval_value_field_access`, `eval_content_method`,
  `field_callee_error` e os helpers de `Content` **não recebem contexto** —
  operam sobre `Value`/`Content` já avaliados. Só `eval_field_access` recebe
  `Scopes`/`EvalContext`/`Engine`, para avaliar o alvo.
- `Content` é enum fechado (ADR-0026): o acesso a campo é `match` exaustivo,
  nunca reflexão.

## Instrução

### `eval_field_access(access, scopes, ctx, engine)`

Avalia o alvo e delega em `eval_value_field_access`. Antes de falhar, consulta
`field_callee_error` para produzir a mensagem certa quando o alvo é algo que
deveria ter sido chamado.

### `eval_value_field_access(target, field, span)`

| Tipo do alvo | Campos |
|---|---|
| `Dict` | a chave; erro `dictionary does not contain key "…"` se ausente |
| `Module` | binding exportado do módulo |
| `Content` | ver `content_field` |
| `Symbol`, `Func`, `Type` | campos/variantes do próprio tipo |
| `Duration` | `days`, `hours`, `minutes`, `seconds` |
| `Version` | `major`, `minor`, `patch` |
| `Length` | `abs`, `em` |
| `Relative` | `length`, `ratio` |
| `Args` | `positional`, `named` |
| `Stroke` | `paint`, `thickness` |
| `Point`/dimensões | `x`, `y` |

### `content_field(content, field) -> ContentField`

`match` exaustivo sobre o `Content` (ADR-0026) que devolve o campo pedido —
`body`, `text`, `level`, `depth`, `lang`, `outlined`, `bookmarked`, `delta`,
`first`/`last`, `len`, … — ou a indicação de campo inexistente. O enum
`ContentField` distingue "campo presente com valor" de "campo não existe neste
elemento", para a mensagem de erro ser exacta.

`content_set_fields` cobre os campos definidos por set-rule;
`content_elem_func` devolve a função do elemento (`.func()`);
`content_func_not_callable` produz o erro de callee.

### `eval_content_method(content, method, args, span)`

`func`, `has`, `at`, `fields`, `location` — os métodos de leitura de `Content`.
Usa `expect_positional`/`finish_args` de `super::method_dispatch`.

### `element_or_type_with_name` e `field_callee_error`

Produzem as mensagens que nomeiam correctamente elemento vs tipo, incluindo o
caso em que o alvo é `Symbol`/`Func`/`Type`/`Module` (nesses, `None` — não é
erro de campo).

## Critérios de Verificação

### P1311 — field ausente em nativa sem namespace

#### Medição anterior à decisão

Baseline P1310 não commitado sobre HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, congelado na medição histórica.
Medição fresca em diagnósticos,
registra argv, UTC, diff/stat, fontes literais e saídas dos binários pinados.
`csv.encode` e `csv.encode(1)` produzem no vanilla ratificado `a51e02804`
``function `csv` does not contain field `encode` `` com span `4..10`;
o cristalino emite `cannot access fields on type function` no acesso inteiro.
Alias/With não mudam o nome público; `calc.abs.nope` nomeia `abs`.

Vanilla `foundations/func.rs:280-308` distingue scope nativo vazio de ausência
de scope de função definida pelo usuário. `typst-eval/src/code.rs:347-367`
passa o span do identificador do field. No cristalino,
`field_access.rs:103-112,254-268` usa span total e trata namespace None como
proibição genérica. `entities/func.rs:24-48,291-300,355-365` já distingue as
categorias e With; `compiler/eval/repr.rs:76-90` já projeta o nome público
nativo a partir do último segmento do nome interno qualificado.

#### Obrigação e fronteira

Para função nativa Native ou NativeWithEngine cujo namespace seja None,
incluindo With que envolva essa mesma categoria, field ausente produz
exatamente ``function `<nome-público>` does not contain field `<field>` ``.
Nome público vem da função subjacente, último segmento de nome qualificado,
nunca do alias lexical, identidade de ponteiro, string da fixture ou blacklist.
É erro único, sem hints/laterais novos; manter os traces causais existentes.
O acesso usa `access.field().span()`, cobrindo somente o identificador ausente,
também quando a expressão serve como callee e quando há parênteses/multilinha.
O helper de lookup puro utiliza o span recebido; não reconstrói AST/range.

Um helper privado deste owner pode distinguir a categoria já existente de
Func, atravessando With sem alterar argumentos ou entidades. Nome presente
não basta para classificar: closures e plugins nomeados não são nativas.
Não confundir Native sem namespace com namespace Some vazio.
Não fabricar namespace, membro, encoder, callable ou valor de fallback.

Namespace Some (lookup presente ou ausente), Closure, Plugin e Element de
usuário preservam mensagens e spans vigentes. Module/PDF, Dict, Type, Content,
float/is-nan, contexto text e warnings não mudam. Esta exceção restrita substitui
a proteção genérica de targets não Module de P1301/P1306 somente para a
categoria nativa sem namespace aqui especificada; não generaliza field-only
ao restante. Sem alterar assinatura, estrutura de Func, namespace ou dispatch.

Texto e origem diagnóstica são observáveis de linguagem (ADR-0108); a categoria
Rust é mecanismo, não critério de paridade. O cast de identidade nativa e o
contrato de lookup fundamentam a intenção; texto/range são medidos. É inferência
que os carriers atuais bastam: origem irrecuperável, nome público incompatível
ou necessidade de outro owner refutam a suficiência e exigem reabrir o escopo.
Correção interna de paridade ADR-0127, L0-first + RED→GREEN e revalidação.
Testes independentes devem distinguir leitura/chamada ausente, alias/With,
nome qualificado, span individual/total e as categorias excluídas, com controles
de sucesso. Não alegar paridade de funções em geral ou quitação dos namespaces
ausentes e demais dívidas.

```
#let d = (a: 1); d.a                        → 1
#let d = (a: 1); d.b                        → Err "dictionary does not contain key \"b\""
#(1pt).em                                   → 0em
#duration(days: 2).days                     → 2
#version(1, 2, 3).major                     → 1
#[= T].level                                → 1
#[*x*].body                                 → conteúdo de x
#[= T].func()                               → heading
#[= T].has("level")                         → true
#[= T].fields()                             → dicionário com level e body
#(1).foo                                    → Err (nomeia "integer")
```

## P1324 — field ausente em nativa com namespace

### Medição anterior à decisão

A medição histórica
registra a medição fresca sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada,
diff/stat, UTC, fontes e binários. Vanilla ratificado upstream `a51e02804`:
`json.nope` publica ``function `json` does not contain field `nope` `` e
marca somente `nope`; cristalino publica `function does not contain field
"nope"` e marca o acesso inteiro. yaml/toml/cbor/assert/table, alias, With
aninhado e callee confirmam a mesma classe. csv sem namespace e closure
nomeada foram medidos separadamente, sem fundir suas categorias.

`01_core/src/compiler/eval/bindings/field_access.rs:181–195,271–277,420–435`
restringe a projeção nominal/field-only às nativas None e usa mensagem
legada no lookup Some ausente. `entities/func.rs:291–302,355–365` já conserva
nome e namespace através de With. Vanilla
`lab/typst-original/crates/typst-library/src/foundations/func.rs:291–308`
constrói erro a partir do nome da função, enquanto
`lab/typst-original/crates/typst-eval/src/code.rs:347–366` passa field.span().

### Obrigação e sucessão delimitada

Para Native ou NativeWithEngine com namespace Some, vazio ou populado, campo
ausente produz exatamente ``function `<nome-público>` does not contain field
`<field>` ``. With, inclusive aninhado, conserva a categoria e o nome da
função subjacente; o nome público é o último segmento do nome qualificado,
nunca o alias lexical, um nome de fixture ou uma lista fechada de funções.
O acesso AST usa somente access.field().span(), também com parênteses,
multilinha e como callee. O lookup puro conserva integralmente o span recebido.
Erro único, sem novos hints/laterais; manter traces causais e ordem de avaliação.

Esta obrigação substitui expressamente a proteção P1311 de mensagem e span
vigentes para namespace Some AUSENTE, inclusive a expectativa correspondente
do teste anterior. Seus controles de lookup presente permanecem; Some vazio
continua diferente de None como dado, sem fabricar um namespace. A promessa
P1311 para Native/NativeWithEngine None permanece idêntica. Um helper privado
pode projetar o nome nativo para ambos os casos atravessando With, sem mudar
Func, Scope, argumentos ou assinatura/visibilidade pública.

Preservar binding/valor/kind/chamada de fields presentes. Closure, Plugin e
Element de usuário não se tornam nativas por terem nome; mensagens e spans
dessas categorias permanecem. Module/PDF features e hints, Dict, Type,
Content/LocatedContent snapshot, float/is-nan, text contextual e warnings
mantêm as obrigações vigentes. Não generalizar field-only a outras categorias,
não criar membro, encoder, fallback ou formatter, não mudar fases nem defaults.

Texto e origem diagnóstica são língua sob ADR-0107/0108; enum/algoritmo Rust
são mecanismo. A intenção é completar o contrato nominal de lookup nativo
deste owner, não atribuir intenção histórica ao comportamento observado.
É inferência que nome/categoria/AST existentes bastam; nome incompatível,
origem irrecuperável, novo owner/API/default/fase refutam-na e reabrem o escopo.
Correção interna ADR-0127: L0 primeiro, revalidação, RED→GREEN e revalidação.

Aceitação exige testes independentes e processos bilaterais: namespaces
vazios/populados, ambas as categorias nativas, nomes qualificados, alias e
With, leitura/chamada ausente, identificadores e deslocamentos distintos,
sucessos pareados e negativos das categorias excluídas. Perfis default/html/
a11y/html+a11y, repetição e inversão conservam o vetor. Unknown obrigatório
nunca é sucesso; não declarar paridade geral de funções ou fields.

## P1291.cancel-angle-runtime — `text.size` contextual

### Medição anterior à decisão

`eval_field_access` já reconhece `Value::Func("text")` como fronteira de
propriedades contextuais e lê `text.lang` da `StyleChain` ativa. Quando a
callback validada de `math.cancel(angle:)` executa com o snapshot correto,
`context text.size` chega ao mesmo braço, mas cai no acesso comum de `Func` e
falha com `cannot access fields on type function`. A falha está no catálogo
fechado deste consumer, não no transporte de estilo nem no layouter.

### Decisão

No braço existente `text.<campo>`, `size` devolve `Value::Length` a partir do
accessor tipado `engine.styles.size()`. Esse accessor observa primeiro o slot
`StyleDelta.size` do topo — inclusive o `TextStyle.size` math derivado que o
transcript empurra —, depois o custom lexical compatível e, na ausência de
ambos, o default vigente `11pt`. Ler somente `custom("text.size")` é proibido,
pois perde redução de script/cramped. `lang` permanece inalterado e qualquer
outro campo continua no despacho fechado normal, sem namespace fabricado ou
fallback reflexivo.

Este consumer apenas lê o contexto quando a expressão contextual é executada.
Não clona nem altera `Func`, não executa callback, não conhece requests math e
não move avaliação para layout. Aceitação: `context text.size` observa um
`#set text(size: 19pt)` capturado; dentro de script observa o tamanho math
reduzido do snapshot (por exemplo `20pt × 0,7 = 14pt`); ausência observa
`11pt`; campo desconhecido mantém o erro vigente.

## Resultado Esperado

- `eval_field_access`, `eval_value_field_access`, `eval_content_method`,
  `field_callee_error` visíveis em `eval`; `ContentField` e os helpers de
  `Content` privados ao nó.

## P1146 — fields do valor-tipo `datetime`

Medição nos dois binários do vanilla ratificado: `datetime.day(d)` funciona e
`type(datetime.day) == function`, enquanto `d.day` falha com
`cannot access fields on type datetime`. Portanto, o braço fechado
`Value::Type(Type::Datetime)` delega os fields ao owner
`foundations::datetime_type_field`; não se cria field access nem intercepção de
método para `Value::Datetime`. Field desconhecido mantém o erro do valor-tipo.


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — preservação de namespaces/fields

Converter os sete bindings para `Value::Type` não autoriza perder fields
públicos. Todo field já observado em algum binding anterior deve ser resolvido
por braço explícito `(Type::<Kind>, field)` e devolver a mesma função/constante
que o namespace anterior. Field inexistente mantém o erro vigente. Não se usa
mapa reflexivo nem fallback genérico; a cobertura é estática e exaustivamente
testada por kind.

## P1140.4-C — campos mínimos da equação passada a `supplement`

### Medição antes da decisão

No vanilla pinado, a callback `supplement: it =>
[#repr(type(it))|#repr(it.block)|#it.body]` produziu
`content|true|x + y`. O teste E2E cristalino chegou à callback, mas falhou em
`field_access.rs:105` com `equation does not have field "block"`;
`content_field` (`field_access.rs:430-470`) não tem braço Equation.

### Decisão

`content_field` ganha braço explícito para `Content::Equation`: `block` devolve
`Value::Bool` e `body` devolve `Value::Content`. `content_set_fields` enumera
`block`, `body` nessa ordem. Não se cria fallback reflexivo nem se expõem os
campos ainda não materializados por valores inventados.

## P1140.5-A — acesso ao campo `alt` em equações styled

### Medição antes da decisão

Vanilla `fields()` medido inclui `alt: "description"`, `alt: none` e
`alt: ""` quando explícitos; o campo aparece antes de `body`. O cristalino
transportará `alt` na style chain, enquanto `EquationElem` permanece mínimo.

### Decisão

O acesso a `Content::Styled` reconhece descendente Equation e resolve o delta
top-wins `equation.alt`: `Str`/`None` tornam-se campo set; ausência mantém o
estado não explícito/default. `block` e `body` continuam delegados ao elemento.
`content_set_fields` preserva ordem pública e não assa `alt` em `plain_text`.

## P1147 — fields do valor-tipo `int`

O braço `Type::Int` preserva `min`/`max` e delega os outros nove fields ao
owner `foundations::int_type_field`. A fonte e as sondas ratificadas medem
formas estática e de instância para funções com `self`; `from-bytes` permanece
somente estática. Field desconhecido mantém o erro do valor-tipo.

## P1148 — fields do valor-tipo `counter`

`Value::Type(Type::Counter)` delega os seis fields públicos ao owner
`counter_type_field`. O field access só descobre a função; contexto,
introspecção e semântica permanecem no owner/dispatch existente.

## P1150 — fields do valor-tipo `content`

Sondas coincidentes nos dois binários ratificados confirmam cinco functions:
`func`, `has`, `at`, `fields`, `location`. `content_type_field` expõe wrappers
não ligados que recebem `Value::Content` primeiro e delegam ao mesmo
`eval_content_method` usado pelos métodos de instância.

Medição com `strong[Hi]` confirma igualdade de `fields`, `has`, `at`,
`default:`, `func` e `location`; `content.func(strong[Hi]) == strong` é true e
location inline é `none`. Match fechado, sem duplicar `content_field` ou
`content_set_fields`. É glue interno de paridade, fluxo contínuo ADR-0127.

## P1151 — `content.location()` para conteúdo introspectado (GATE ADR-0127)

### Medição antes da decisão

Os dois binários vanilla ratificados devolvem Location presente para conteúdo
vindo de `query`, `none` para conteúdo inline e Locations distintas para duas
headings de morfologia idêntica. Igualdade, `repr` e `fields` não incorporam a
Location. No cristalino P1150, o método só recebe `&Content` e retorna sempre
`none`; a Location já foi descartada por `native_query`.

### Decisão condicionada ao gate

O despacho de métodos deve aceitar tanto conteúdo declarativo sem metadado
quanto o valor locatável de `entities/value.md` P1151. `func`, `has`, `at` e
`fields` delegam ao mesmo `Content`; `location` devolve `Value::Location(loc)`
somente no segundo caso e `Value::None` no primeiro. As formas estática e de
instância permanecem equivalentes. Não consultar o introspector por igualdade,
não mover a decisão para layout e não adicionar Location a cada elemento.

A mudança depende da nova representação pública em `Value`; parar antes do
código conforme ADR-0127.

## P1161 — modifiers sobre valor multi-codepoint (GATE ADR-0127)

Medição vanilla: `emoji.heart.arrow` → `💘`, `emoji.heart.excl` → `❣️` e
`emoji.heart.nope` falha com `unknown symbol modifier` no span de `nope`.
`eval_value_field_access` continua a delegar em `Symbol::modified`; o valor
selecionado passa a ser o `EcoString` integral da variant, sem alterar a
mensagem ou o span. Não duplicar seleção de variants neste nó.

## P1284 — catálogo fechado de valores-tipo, constantes e wrappers

### Medição antes da decisão

`C-P1284-v2` mede 180 paths residuais e exige que membros de instância sejam
também projetados como funções não ligadas no valor-tipo. A representação
cristalina pré-candidata já possui `Value`/`Type`/entidades suficientes para o
lote contínuo, exceto `selector.before/after`, `color.spot/tint` e
`outline.entry/*`, bloqueados por ADR-0127.

### Decisão

`eval_value_field_access(Value::Type(t), field, span)` mantém um match fechado
e delega a descoberta ao owner semântico. Este nó possui somente o glue de
lookup e a classificação do valor devolvido; não duplica callbacks, unidades,
introspecção, fórmulas de cor nem mutação.

| Valor-tipo | Fields autorizados em P1284 | Owner semântico / kind |
|---|---|---|
| `array` | 32 métodos de instância + `range` (33 fields) de `stdlib/collections` | `function` |
| `dictionary` | os 9 métodos de `stdlib/collections` | `function` |
| `str` | 20 métodos de instância, inclusive `to-unicode`, + `from-unicode` (21 fields) | `function`; conversões no owner `foundations/str` |
| `bytes` | `at`, `len`, `slice` | `function`, owner `stdlib/collections` |
| `arguments` | `at`, `filter`, `len`, `map`, `named`, `pos` | `function`, owner `stdlib/collections` |
| `color` | catálogo vigente + `map` | `map` é `module`, delegado a `stdlib/color` |
| `direction` | `axis`, `end`, `inv`, `sign`, `start`, `from`, `to`; `btt/ltr/rtl/ttb` | funções; quatro constantes `direction` |
| `alignment` | `axis`, `inv`; `bottom/center/end/horizon/left/right/start/top` | funções; oito constantes `alignment` |
| `duration` | `days`, `hours`, `minutes`, `seconds`, `weeks` | `function` |
| `length` | `cm`, `inches`, `mm`, `pt`, `to-absolute` | `function` |
| `selector` | `and`, `or`, `within` | `function`, owner de orchestration `value_methods` |
| `state` | `at`, `final`, `get`, `update` | `function`, owner `stdlib/state` |
| `location` | `page`, `page-numbering`, `position` | `function`, owner `call_dispatch` |

As constantes qualificadas reutilizam o mesmo `Value` do binding global:
`direction.rtl == rtl`, `alignment.left == left`, etc. `axis` devolve
`"horizontal"`/`"vertical"` ou `none` para alinhamento 2D; `inv`,
`start/end`, `sign`, `from/to` seguem as tabelas fechadas das entidades
existentes. Nenhuma constante é embrulhada como função.

Cada wrapper de instância recebe `self` como primeiro positional e encaminha
à mesma função que a forma ligada. Nome, `repr`, ordem, required/named,
variadic, settable e default são os do owner. Field desconhecido mantém a
mensagem `type <nome> does not contain field <field>`; não há fallback
reflexivo.

### Gates negativos

- `selector.before` e `selector.after` permanecem ausentes:
  `BLOCKED_ADR0127_PUBLIC_CONTRACT`; não mapear para `Within`/`And`/`Or`.
- `color.spot`, `color.spot.tint` e `outline.entry/*` permanecem bloqueados;
  não fabricar `dict`, `none`, stub ou módulo aproximado.
- Este L0 não autoriza campo, variante, trait, assinatura Rust pública, default
  de produto ou mudança de fase. Se um wrapper não couber no glue interno,
  parar no ADR-0127.

### Verificação

Para todo field: existência, kind, `repr`, metadata, forma ligada/não ligada,
chamada real, defaults e erros. Sentinelas: `array.len((1,2,3)) == 3`,
`arguments.len(arguments(1,x:2)) == 2`, `direction.rtl == rtl`,
`alignment.left == left`, `type(color.map) == module`. A mera resolução do
nome sem chamada real é insuficiente.

## P1289 — descoberta fechada de `float.is-infinite`

### Medição anterior à decisão

O vanilla pinado devolve `(function, "is-infinite")` para o field do valor-tipo;
o cristalino pré-candidato falha com
`type float does not contain field "is-infinite"`. Chamadas ligadas existem,
mas obter `(1.0).is-infinite` como valor continua a falhar no vanilla com
`cannot access fields on type float`.

### Decisão

O braço `Value::Type(Type::Float)` delega a descoberta ao owner
`foundations::float_type_field`. Este nó não contém a fórmula, não cria
reflexão para `Value::Float` e não intercepta a chamada ligada. Field
desconhecido mantém `type float does not contain field "<field>"`.

## Span do acesso ligado `is-nan`

Quando `eval_field_access` recebe `Value::Float` e o campo `is-nan` sem
chamada, o erro `cannot access fields on type float` usa
`access.field().span()`. Mensagem, severidade, hints e ausência do valor
ligado permanecem iguais.

Todos os outros targets, fields e erros conservam sua âncora. Não generalizar
a regra, criar reflexão ou fabricar o método como valor. Fórmula e chamada
pertencem ao owner de float; spans de chamadas pertencem a `call_dispatch`.

## P1301 — diagnóstico de field ausente em `Module`

### Medição anterior à decisão

Em `2026-09-03T20:57:32.863761582-03:00`, no HEAD
`1f082370e59939de7b57992e137a9f74bfb6758f` e working tree não commitida,
A medição histórica registrou o vanilla
ratificado (`7b4f40c5…`) contra o cristalino pré-candidato (`799546de…`). A
matriz P1300 já continha `12/12` divergências para `std.hsl`, `std.hsv` e
`std.linear_rgb` nos quatro perfis. A nova sonda confirmou a mesma classe em
`calc.nope`, `sym.nope` e `color.map.nope`, sem `Unknown`:

- o vanilla recebe `field.span()` em
  `lab/typst-original/crates/typst-eval/src/code.rs:347-366` e ancora somente o
  identificador à direita do ponto;
- `Module::field` forma ``module `<nome>` does not contain `<field>` `` em
  `lab/typst-original/crates/typst-library/src/foundations/module.rs:139-150`;
- o módulo que alimenta o binding `std` é construído como `global` em
  `lab/typst-original/crates/typst-library/src/lib.rs:221-225,374`;
- o cristalino entrega `access.span()` em
  `01_core/src/compiler/eval/bindings/field_access.rs:106-111` e forma
  `module '<nome>' does not contain field "<field>"` em
  `01_core/src/compiler/eval/bindings/field_access.rs:373-379`.

Mensagem e âncora são observáveis públicos da linguagem sob ADR-0107; o nome
interno do módulo e o modo de obtê-los são mecânica. É inferência que uma regra
por categoria `Module`, com projeção semântica do binding padrão `std` para o
nome público `global`, corrige a classe inteira sem blacklist por field. Um
módulo ausente cujo vanilla use outra forma/âncora, ou a necessidade de mudar
API pública ou fase, refutaria essa inferência.

`repr(std)` também foi medido como diferente (`<module global>` contra
`module(std)`), mas pertence à representação/construção do módulo e fica
explicitamente fora deste consumer e deste passo. Não usar a correção de
diagnóstico para mascarar essa divergência separada.

### Decisão e aceitação

Quando `eval_field_access` avalia um `Value::Module`, deve passar
`access.field().span()` ao lookup de field; sucesso continua a devolver o
binding sem alterar valor, kind ou avaliação. Se o field não existir,
`eval_value_field_access` emite exatamente:

```text
module `<nome-público>` does not contain `<field>`
```

O diagnóstico é erro, não possui hints e ancora somente o identificador do
field. Para módulos nomeados, `<nome-público>` é o nome guardado no módulo.
A premissa P1301 de que o global cristalino guardava `std` foi superada pela
construção P1305; a revisão medida P1306 abaixo substitui expressamente essa
projeção nominal. O módulo padrão alcançado pelo binding `std` guarda e
publica `global`, enquanto um arquivo ordinário `std.typ` publica `std`.
A identidade independe do nome lexical do alias ou do conteúdo do scope.
É proibido listar `hsl`,
`hsv`, `linear_rgb` ou qualquer outro field na implementação.

A regra vale para qualquer field ausente de `Module` e deve ser verificada ao
menos em `std`, `calc`, `sym` e `color.map`, além de um field existente de
controle. Os quatro perfis `default`, `html`, `a11y` e `html+a11y` devem manter
o mesmo resultado para o fragmento P1300. Ordem normal e invertida devem gerar
o mesmo vetor; `Unknown` não satisfaz aceitação.

A exceção P1293 para `Value::Float` + `is-nan` permanece. Targets que não são
`Module` preservam a âncora vigente, salvo decisões próprias já escritas neste
L0. Não alterar `Module`, `Scope`, assinatura pública, entidade, default,
ordem de avaliação ou fase; não criar wrapper, fallback reflexivo, blacklist
por field nem correção de `repr(std)` neste passo.

Classificação: correção de paridade em fluxo contínuo ADR-0127. O gate
funcional é RED→GREEN contra o vanilla ratificado, seguido de revalidação da
linhagem.

## P1303 — span dos fields `pdf.*` bloqueados por `a11y-extras`

### Medição fresca anterior à decisão

Em `2026-09-04T00:38:07.847942-03:00`–`00:38:10.279557-03:00`, no HEAD
`5b4a0d0438a535c54fdb5e74b28903c1313f5bc2` e working tree não commitada
sem diff tracked ou staged, a medição histórica
executou `48` runs e `24` comparações, em ordem normal e invertida, contra o
vanilla ratificado de e um
binário cristalino release fresco de.

Nos perfis `default` e `html`, os três acessos `pdf.data-cell`,
`pdf.header-cell` e `pdf.table-summary` produziram em ambos os produtos exit
`1`, stdout vazio, exatamente um erro primário, zero diagnósticos laterais, a
mesma mensagem e os mesmos dois hints na mesma ordem. A única diferença foi a
âncora: vanilla `14..23`, `14..25` e `14..27`, cobrindo respectivamente
`data-cell`, `header-cell` e `table-summary`; cristalino `10..23`, `10..25` e
`10..27`, cobrindo `pdf.<field>`. Nos perfis `a11y` e `html+a11y`, os três
fields coincidiram como `(function, "data-cell")`, `(function,
"header-cell")` e `(function, "table-summary")`. A ordem normal mediu `3`
`DIFFERENT_DIAGNOSTIC` em cada perfil negativo e `3` `MATCH_VALUE` em cada
perfil positivo; somadas as duas ordens, foram `12`
`DIFFERENT_DIAGNOSTIC`, `12` `MATCH_VALUE`, `0` Unknown e `0` divergências de
repetição.

O span, a mensagem, os hints e a cardinalidade do diagnóstico são observáveis
da linguagem sob ADR-0107; a função/helper Rust e a forma de selecionar o span
são mecânica interna. A medição não atribui intenção a todo comportamento do
vanilla: ela confirma somente o contrato público desses três erros e dos três
sucessos feature-gated.

É inferência que a âncora total usada pelo ramo especial de feature ausente é
causa suficiente das seis divergências, porque todos os demais observáveis
negativos coincidem e os seis casos positivos já coincidem. Refutam essa
inferência: qualquer diferença fresca de mensagem, hints, severidade ou
cardinalidade; divergência com `a11y-extras` ligada; impossibilidade de resolver
o span do identificador; outro owner causal; ou necessidade de alterar API
pública, default, feature, compatibilidade ou fase do pipeline. Nenhuma dessas
condições foi medida.

### Classificação e decisão

Classificação: `ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`. Trata-se de correção
interna e localizada de paridade diagnóstica, sem campo de entidade, método de
trait, assinatura pública, comportamento por defeito, mudança de fase ou quebra
de compatibilidade. O fluxo é L0-first + revalidação e segue sem nova paragem humana
para o gate RED→GREEN e a revalidação.

Quando `eval_field_access` intercepta a ausência de `a11y-extras` para
exatamente `pdf.data-cell`, `pdf.header-cell` ou `pdf.table-summary`, o erro
deve usar somente o span de `access.field()`: todos e somente os bytes do
identificador à direita do ponto. Não deve incluir `pdf`, o ponto, bytes à
esquerda ou à direita, nem truncar o field. Mensagem, severidade, os dois hints
e sua ordem, cardinalidade e gate de disponibilidade permanecem inalterados.
Com `a11y-extras` ligada, valor, kind `function`, nome/`repr` e morfologia das
chamadas permanecem os vigentes.

A regra é exclusiva do ramo especial desses três fields. É proibido
generalizá-la ao erro de dicionário ou a targets não cobertos; alterar a regra
geral de mensagem e span de `Module` validada por P1301r2; ou regredir
a exceção field-only de `float("NaN").is-nan` validada por P1293.
A proteção histórica da projeção nominal `std` → `global` é substituída
exclusivamente pela obrigação de identidade nomeada P1306 abaixo, preservando
o nome público `global` do módulo padrão real. As demais proteções P1303 ficam.

Também é proibido mover o gate para `stdlib/pdf`, duplicar o catálogo, criar
blacklist/fallback reflexivo, expor o trio sem `a11y-extras`, ocultá-lo com a
feature ligada, alterar `pdf.attach`/`pdf.artifact`, `color.map`, o gate `html`,
extensões cristalinas, `Module`, `Scope`, `Features`, entidades, traits,
assinaturas públicas, defaults, CLI, exportadores, wiring, lab ou a fase
`eval`/`layout`. Se a correção field-only não satisfizer o contrato, voltar à
medição e bloquear/ampliar somente mediante novo L0 e classificação aplicável.

Aceitação limita-se aos três spans feature-gated nos quatro perfis, com ordem
normal/invertida, repetição determinística e `0` Unknown. Não prova equivalência
funcional geral de `pdf`, do compilador, do PDF exportado ou da acessibilidade,
nem autoriza resolver membros ausentes, `repr(std)` ou o inventário residual
P1299.

## P1306 — identidade no erro de campo ausente de módulo nomeado

### Medição independente anterior à decisão

Em `2026-09-07T15:07:28.245613+00:00`–`15:07:53.110165+00:00`, sobre HEAD
`b303f1f15b610e09872b567027e0d806387fde8c`, sem diff tracked/staged,
A medição histórica
registrou fixtures, hashes, comandos, cwd, horários, saídas e status completos
antes/depois. A árvore continha somente untracked enumerados no registro. O
baseline validado P1305 tinha; o vanilla
ratificado upstream/main `a51e02804`.

Os 19 probes nos quatro perfis, dois produtos e ordens normal/inversa
produziram 304 execuções válidas da CLI. As cinco rotas ordinárias de `std.typ`
(direta, alias com field `absent`, sombra lexical, reexport integral e nesting)
divergiram somente no nome: cristalino `global`, vanilla `std`. Global real e
alias publicaram `global`; arquivos `global.typ`, `map.typ`, `forest.typ` e
módulos calc/sym/color.map mantiveram seus nomes próprios. Os três positivos
pareados de repr/lookup/chamada coincidiram, inclusive
`["<module std>",7]` e `["<module std>",11,3]`; os vetores foram estáveis por
ordem. O controle de dicionário confirmou separadamente sua âncora cristalina
total preexistente, divergente da vanilla, cuja preservação é obrigatória.
A tentativa anterior com opção CLI `--color` não aceita pelo vanilla fica
integralmente no registro como falha de instrumentação, sem valor de RED.

A leitura adicional da fonte confirma: em
`01_core/src/compiler/eval/bindings/field_access.rs:376-382` o diagnóstico
rebatiza nominalmente `std`, enquanto `:106-112` já seleciona field-only;
`01_core/src/compiler/eval/mod.rs:324,562` e
`01_core/src/compiler/eval/modules.rs:66` já constroem o global com `global`.
No vanilla,
`lab/typst-original/crates/typst-library/src/foundations/module.rs:139-150`
publica o nome guardado.

### Classificação, inferência e obrigação perene

Mensagem, severidade, hints, cardinalidade e âncora diagnóstica são observáveis
da linguagem (ADR-0107/0108); identidade de ponteiro, forma de armazenamento e
algoritmo não são critérios. Não se atribui intenção histórica ao vanilla.
É inferência que o nome já guardado basta para este fragmento: alias e reexport
refutam identidade por variável ou conteúdo. Um módulo nomeado legítimo com
nome guardado insuficiente/incorreto, ou necessidade de outro owner/API,
refutaria essa inferência e reabriria o escopo.

Classificação: `ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`. Após L0-first e
revalidação, a correção interna segue por RED→GREEN e revalidação, sem alteração
de API, entidade, trait, assinatura, default, compatibilidade ou fase.

Para qualquer módulo nomeado e field ausente, o nome em
``module `<nome>` does not contain `<field>` `` deve ser exatamente o nome
guardado no módulo. O global real continua `global`, inclusive por aliases;
o módulo ordinário `std.typ` continua `std`, inclusive por alias, sombra
lexical, nesting e reexport integral. Não inferir identidade do spelling da
variável, do nome reservado `std`, dos fields exportados ou de uma whitelist.
O field ausente pode ser qualquer identificador; nenhum perfil recebe exceção.

Preservar erro único de severidade error, zero hints/laterais novos, stdout
vazio, exit de erro e span resolvível cobrindo somente o field. Lookups presentes
preservam valor, kind, chamada e repr. Os nomes e os imports validados P1305
são entradas protegidas; este nó não altera suas construções ou representação.
P1303 PDF (spans, hints ordenados e features), o span total do dict, a exceção
field-only float/is-nan e todos os targets não Module permanecem vigentes.
Os reconhecimentos nominais pdf/sym, warnings e gates não entram nesta revisão.

Aceitação focal cobre os quatro perfis com positivos pareados, campos distintos
e deslocamentos de linha/coluna; exige testes independentes, matriz bilateral
estável por repetição/inversão e mutantes aplicáveis rejeitados. `Unknown`,
erro de import inválido, falha de execução ou de adaptação não satisfazem RED
nem GREEN. Opacidade só é admissível quando realmente faltar informação pública,
sem transformar Unknown obrigatório em sucesso. Esta obrigação não resolve
anonimato de plugin, encoders ou disponibilidade residual, nem implica paridade
geral do compilador.

## P1307-R3 — consumo de default nos métodos de Content

### Medição anterior à decisão

`01_core/src/compiler/eval/bindings/field_access.rs:750-768` extrai o field
pelo helper de method_dispatch, remove default diretamente de named e chama
finish_args. A operação não modifica o Content nem a política de fields.
Snapshot adicional R3
conserva este L0 após P1306 e antes deste amendment, HEAD
`b303f1f15b610e09872b567027e0d806387fde8c` e diff/stat integral. O consumer
Rust P1306 permanece byte-idêntico durante a redação.

### Migração, pendente do gate Args

Quando method é at, retirar default por `Args::remove_named("default")`,
preservando o último valor retornado e eliminando as ocorrências consumidas
do carrier Some. A extração do positional continua no helper proprietário
de method_dispatch, também migrado. Não invalidar Some nem copiar o helper.
None mantém a view legada; o span agregado e as âncoras dos erros não mudam.

P1306 Module, P1303 PDF, dict/float, lookup e todas as demais obrigações deste
owner permanecem intactos. Não introduzir validação de named duplicados ou
namespace de encoder em field_access. Aceitação cobre Content/LocatedContent
has/at/default, consumidos ausentes de ambas as views/carrier e sentinelas
anteriores. É inferência de suficiência refutada por outra escrita causal.
Este amendment só adapta consumo à definição do owner Args, que aguarda
aprovação humana ADR-0127; não autoriza este consumer a redefinir Args.

### P1307-R4 — wrapper estático após aprovação do carrier

Medição adicional, anterior à decisão: no baseline R4,
`01_core/src/compiler/eval/bindings/field_access.rs:828-830` clona Args e
remove o receiver apenas de items. O L0 R3 cobriu o consumo de default, mas
não este writer. Com o contrato Args aprovado, o wrapper deve consumir por
`remove_positional(0)`, preservando o clone original, span agregado e todas
as ocorrências restantes. Não invalidar Some nem mudar os erros vigentes.
É extensão interna da mesma migração, não nova API ou fase; verificar a
equivalência estática/ligada de content.func/has/at/fields/location. A fonte
e a lista de writers refutam a suficiência da enumeração R3, não a API aprovada.

## P1339 — descoberta de `function.where`

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto:
`field_access.rs:689-832` despacha fields de Type sem cobrir Type::Function;
`call_dispatch.rs:1117-1129` intercepta somente a chamada ligada de where.
Os registros `p1339-full-final-{vanilla,crystalline-before}-runs.json` fixam
binários, UTC e working tree e medem `(function, "where")` no vanilla contra
ausência no cristalino. A fonte ratificada `foundations/func.rs:412-450`
define a função pública, enquanto `:280-308` distingue fields próprios de
uma instância Func do namespace do seu tipo.

### Decisão de integração

O par fechado `(Type::Function, "where")` descobre a nativa do owner
`compiler/eval/bindings/value_methods.md`. Este owner só encaminha lookup;
não contém validação de receiver, tabela de campos, normalização de filtros
ou construção de Selector. O resultado é Value::Func com nome público curto
`where`, chamável diretamente, por alias e por aplicação de função existente.
Type::Function continua não construtor; não fabricar namespace nas instâncias.

Extrair `f.where` sem chamada mantém os erros/âncoras vigentes das categorias
Native/NativeWithEngine/Closure/With. Não transformar método ligado em field
de instância, não reconhecer identidade por nome e não executar o receiver.
As preservações históricas de disponibilidade de Type são sucedidas somente
para esse membro. Outros fields ausentes, categorias, PDF/features, snapshots,
text contextual e diagnósticos P1301–P1338 conservam suas obrigações.

Helpers de projeção já proprietários de Content podem ganhar apenas
visibilidade interna para reutilização estática pelos novos filtros, sem
alterar a política pública de acesso raw/LocatedContent nem inventar defaults.
Não ampliar lookup raw Text/Raw por efeito lateral dessa reutilização: o
owner de matching decide sua projeção, com snapshot Some autoritativo onde
receba conteúdo consultado. Não criar API pública nova.

Aceitação: descoberta/type/repr/chamada real, alias, rejeição de extração
ligada e preservação dos demais lookups. Refutam a suficiência outra
identidade que ganhe campo ou necessidade de novo carrier público.
Correção de namespace em fluxo contínuo ADR-0127. Este recorte não completa
os contratos das demais rotas.

## P1401 — descoberta de `selector.before/after` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline P1401, o acesso de campo de `Value::Selector` reconhece os métodos
vigentes, mas não `before` ou `after`; por isso a chamada falha antes do
dispatcher, enquanto o vanila pinado expõe ambos.

### Decisão proposta

Instâncias de selector expõem campos chamáveis `before` e `after`; a descoberta
por tipo também os reconhece como métodos não estáticos. Este owner apenas
resolve o nome e vincula o receptor, sem construir variants nem consultar o
documento. Outros nomes e mensagens de campo ausente permanecem iguais. A nova
superfície pública aguarda confirmação ADR-0127.

## P1402 — método de `outline.entry`

Quando o receptor é `Content::OutlineEntry` (direto ou localizado), o campo
`body` vincula o receptor ao interceptor dedicado. Os campos materiais
`level`, `element` e `fill` continuam a ser lidos pelo caminho comum de
`Content`; nenhum outro conteúdo ganha esse método.
## P1446 — campos de `math.stretch` (PROPOSTA; gate ADR-0127)

`body` está sempre disponível. `size` existe em acesso, `fields`, `has` e
`at` somente quando o argumento foi explicitamente fornecido; a cascata e o
default geométrico não fabricam presença. `func()` devolve a função nativa
`stretch`. Campo desconhecido conserva o diagnóstico global.

## P1447 — campos do carrier `MathLr`

Para `Content::MathLr`, `fields()` enumera `size` antes de `body` quando o
tamanho foi explicitamente fornecido e somente `body` quando foi omitido.
`body`, `size`, `has` e `at` usam a projeção do owner
`entities/elements/math_delimited.md`; defaults e cascata não fabricam
presença. `func()` continua resolvendo a identidade pública `lr`, inclusive
quando o carrier foi construído por `math.abs`.

## P1448 — reflexão causal de `MathUnderbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

No vanilla, `body` está sempre assente; `annotation` omitida não aparece em
`fields()` e faz `has("annotation")` devolver falso, enquanto `annotation:
none` aparece como campo assente com valor `none`. `func()` conserva a função
pública `math.underbrace`.

### Decisão

Delegar `Content::MathUnderbrace` ao `get_field` do elemento. Expor `body`
sempre e `annotation` somente quando `annotation_explicit` for verdadeiro,
preservando `Value::None` como valor assente. `content_set_fields` mantém a
ordem `body`, `annotation`; não inferir presença pelo `Option` convertido nem
pela geometria `MathUnderover`.

## P1449 — reflexão causal de `MathUnderbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

No vanilla, `body` está sempre assente; `annotation` omitida não aparece em
`fields()` e `annotation: none` aparece como campo assente. `func()` conserva
a função pública `underbracket`.

### Decisão proposta

Delegar `Content::MathUnderbracket` ao owner: expor `body` sempre e
`annotation` somente quando `annotation_explicit`, preservando `Value::None`.
Manter a ordem `body`, `annotation`; não inferir presença pelo valor convertido
nem pela redução geométrica.

## P1450 — reflexão causal de `MathUnderparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

No vanilla, `body` está sempre assente; annotation omitida não aparece em
`fields()`, `annotation: none` aparece e `func()` devolve `underparen`.

### Decisão proposta

Expor `body` sempre e `annotation` somente quando `annotation_explicit`, na
ordem declarada, preservando `Value::None` e a função pública.

## P1451 — reflexão causal de `MathUndershell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

No vanilla, `body` está sempre assente; annotation omitida não aparece em
`fields()`, `annotation: none` aparece e `func()` devolve `undershell`.

### Decisão proposta

Expor `body` sempre e `annotation` somente quando `annotation_explicit`, na
ordem declarada, preservando `Value::None` e a função pública.

## P1464 — diagnóstico gated sobre `format.pdf` (MATERIALIZADO)

### Medição anterior à decisão

No baseline `a1286f958f15a539dca17a3696ef83ef9e5b1b78`, o ramo especial de
`a11y-extras` reconhece somente `Value::Module` cujo nome público é `pdf`.
P1464 introduz `format.pdf` como função nativa com namespace; sem ampliação
deste owner, o acesso ao trio gated cairia no erro genérico de função em vez do
diagnóstico público de feature desativada.

O vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d` produz, para
`table-summary`, `header-cell` e `data-cell`, exatamente um erro ancorado no
identificador do field:

```text
cannot access field `<field>` because the `a11y-extras` feature is not enabled
hint: try enabling the `a11y-extras` feature
hint: see https://typst.app/help/compiler-features for more details
```

Com a feature ativa, o lookup devolve a função projetada. Mensagem, hints,
ordem e span são linguagem pública; o predicate Rust usado para reconhecer a
função e atravessar `Func::With` é mecânica.

### Decisão e fronteira

O intercept existente aceita dois alvos legítimos para o mesmo gate:

1. o módulo global nomeado `pdf`;
2. a função nativa `format.pdf`, inclusive envolvida por uma ou mais camadas
   de `.with`, desde que conserve o namespace de formato contratado.

O segundo caso não pode ser reconhecido somente por alias lexical ou por uma
string de nome: deve ser uma função nativa cuja identidade subjacente seja
`pdf` e cujo namespace contenha os bindings permanentes `attach` e `artifact`
e os defaults `pages`, `standard`, `tagged` e `pretty`. Closures de utilizador,
plugins, elementos homônimos e funções nativas sem esse namespace continuam
nos seus diagnósticos vigentes.

Sem `A11yExtras`, exatamente os três fields gated recebem a mensagem e os dois
hints acima, nessa ordem, com `access.field().span()`. Com a feature, o
intercept não executa e o lookup ordinário devolve os bindings presentes.
Fields diferentes, `format.svg`, `format.png`, `format.html`, módulos
homônimos, o módulo global `pdf` e todas as obrigações P1303 permanecem
inalterados. Não fabricar valor, mutar namespace, duplicar catálogo nem ativar
feature por target.

Aceitação cobre os três fields, acesso direto/alias/`std`/`.with`, os quatro
perfis, mensagem integral, dois hints, span do field e controles homônimos.
`Unknown` bloqueia. A confirmação humana posterior ao commit causal P1464 foi
recebida em 2026-09-20; o diagnóstico foi materializado e coberto pelo ciclo
RED→GREEN independente.

## P1465 — diagnóstico gated sobre `format.html` (MATERIALIZADO)

### Medição anterior à decisão

No baseline `8e74134a721310cc4fb2a596a195398e16f1d5db`, o gate `html`
existe somente no lookup de identificador bare em `compiler/eval/mod.rs`.
P1465 introduz `html` como field do módulo `format`; sem intercept neste owner,
o binding seria devolvido mesmo com a feature desligada, violando o pin
`compiler-feature-gates`.

O vanilla ratificado produz em `format.html` e `std.format.html`, sem a
feature, exatamente um erro ancorado no identificador `html`:

```text
cannot access field `html` because the `html` feature is not enabled
hint: try enabling the `html` feature
hint: see https://typst.app/help/compiler-features for more details
```

`a11y-extras`, target ou exporter não alteram o resultado. Com `html` ativa,
o lookup devolve a função projetada. Mensagem, hints, ordem e span são língua;
o predicate Rust usado para reconhecer o módulo é mecânica.

### Decisão e fronteira

Antes do lookup ordinário, quando o field é exatamente `html`, reconhecer o
módulo `format` efetivo pela combinação de nome público e superfície estrutural
contratada (`svg`, `png`, `pdf` e `html` como funções de formato). Alias lexical
e o caminho por `std` conservam esse carrier e recebem o mesmo gate. Não
interceptar módulo importado apenas porque um alias local se chama `format`,
nem funções, dicts ou módulos homônimos sem a superfície completa.

Sem `Feature::Html`, devolver a mensagem e os dois hints acima, nessa ordem,
com `access.field().span()`. Com a feature, cair no lookup normal. Fields
diferentes, o identificador global `html`, `std.html`, os gates PDF, funções
com `.with`, closures e todos os diagnósticos anteriores permanecem fora deste
ramo.

Aceitação cobre acesso direto, alias e `std`, perfis default/a11y/html/
html+a11y, mensagem integral, hints, span e controles homônimos. `Unknown`
bloqueia. A confirmação humana posterior ao commit causal
`e300314e2f87e90e543c4923a8cae97f9ad978a8` foi recebida em 2026-09-20; o
gate foi materializado e coberto pelo RED→GREEN funcional.

## P1471 — retificação: o primeiro alvo de `is_a11y_pdf_target` fica morto (MATERIALIZADO)

### Medição anterior à decisão

`compiler/stdlib/pdf.md` P1471 mede (2026-09-21) que o vanilla ratificado
expõe `pdf` como `function`, não `module`. O predicate atual
`is_a11y_pdf_target` (`01_core/src/compiler/eval/bindings/field_access.rs:490-496`)
aceita **dois** alvos, exatamente como a seção P1464 acima já documenta
em §"Decisão e fronteira" item 1-2:

```rust
fn is_a11y_pdf_target(target: &Value) -> bool {
    match target {
        Value::Module(module) => module.name() == "pdf",       // alvo 1
        Value::Func(func) => is_format_pdf_function(func),      // alvo 2
    }
}
```

O alvo 1 (`Value::Module` nomeado `pdf`) reconhecia a raiz global `pdf`
**antes** de P1471 nos owners upstream. O alvo 2 (`Value::Func` nativa
`pdf` cujo namespace contém os bindings/defaults contratados, ver
`is_format_pdf_namespace` nas mesmas linhas) já reconhecia `format.pdf`. A
seção P1464 acima descreve isto como intencional — dois alvos legítimos
para o mesmo gate, um para cada forma existente na época.

### Decisão

Com `pdf` retificado para `Value::Func` (mesma forma de `format.pdf`,
mesmo `is_format_pdf_namespace`), o alvo 1 (`Value::Module(module) =>
module.name() == "pdf"`) deixa de ser alcançável — `pdf` nunca mais é
`Value::Module`. `is_a11y_pdf_target` deve ser simplificado para o único
caso remanescente:

```rust
fn is_a11y_pdf_target(target: &Value) -> bool {
    match target {
        Value::Func(func) => is_format_pdf_function(func),
        _ => false,
    }
}
```

`is_format_pdf_function`/`is_format_pdf_namespace` **não mudam** — já
testam exatamente a forma que a raiz `pdf` passa a ter. Este owner não
ganha nem perde diagnóstico, mensagem, hint ou span: o comportamento
observável de `format.pdf.table-summary`/`header-cell`/`data-cell` sem
`a11y-extras` é idêntico antes/depois; muda somente qual branch do `match`
Rust deixa de ser alcançável (limpeza de código morto, não mudança de
contrato de linguagem). A retificação equivalente para `html` (P1465
acima, "reconhecer o módulo `format` efetivo pela ... superfície estrutural
contratada") não usa `Value::Module` para `html`/`pdf` diretamente — já
reconhece `format` pela superfície de `svg`/`png`/`pdf`/`html` como
funções — e não precisa de retificação própria.

### Correção descoberta na integração — `is_format_pdf_namespace` exigia os defaults exclusivos da projeção

A afirmação acima ("`is_format_pdf_function`/`is_format_pdf_namespace` **não
mudam**") estava **errada** — refutada pela suíte RED→GREEN durante a
integração (papel C, 2026-09-21). `is_format_pdf_namespace` exigia
`pages`/`standard`/`tagged`/`pretty` além de `attach`/`artifact`; esses
quatro primeiros são defaults exclusivos da **projeção** `format.pdf`
(`compiler/stdlib/format.md`), nunca existiram no namespace da **raiz**
`pdf` (só `attach`/`artifact`[/trio a11y]). Com o ramo `Value::Module`
removido, `pdf.table-summary`/`header-cell`/`data-cell` acedidos na raiz
(sem passar por `format.pdf.*`) deixavam de bater com nenhum dos dois
ramos e caíam no erro genérico de field-access-em-função
(`"function `pdf` does not contain field `data-cell`"`), regredindo os
oráculos já materializados em P1303 (`cannot access field ... because the
`a11y-extras` feature is not enabled`, com os dois hints).

`is_format_pdf_namespace` foi corrigida para exigir somente `attach`/
`artifact` como `Value::Func` — suficiente, combinado com o nome nativo
`"pdf"` já verificado pelo chamador, para identificar as duas formas
(raiz e projeção) sem depender dos defaults exclusivos da projeção. Os
cinco testes P1303 (negativos/positivos/sentinelas) voltaram a GREEN sem
mudança de mensagem, hint ou span.

### Gate ADR-0127

Consequência mecânica da quebra de contrato decidida em
`compiler/stdlib/pdf.md`/`compiler/stdlib/html.md` P1471 — mesmo gate,
mesma confirmação humana explícita posterior exigida antes de tocar este
código.

Confirmação recebida em 2026-09-21, posterior ao commit causal
`2ae054ab5`. Agente A (`e118c2053`) simplificou `is_a11y_pdf_target`.
Agente B (`b546f709b`) escreveu 2 testes RED
(`p1471_format_pdf_a11y_gate_message_and_hints_stable_across_profiles`,
`p1471_arbitrary_module_named_pdf_is_not_special_cased_by_pure_lookup`)
sem ver a implementação. Integração (papel C) descobriu, via a suíte
completa, que a simplificação sozinha regredia 3 oráculos P1303 já
materializados (`pdf.data-cell`/`header-cell`/`table-summary` acedidos na
**raiz** sem `a11y-extras` deixavam de produzir o diagnóstico gated) —
corrigido relaxando `is_format_pdf_namespace` (ver "Correção descoberta na
integração" acima); os 2 testes novos de B não cobriam esse caso porque o
próprio L0, nesta secção, também não o antecipava antes da integração.
Suíte: 35/35 neste ficheiro (2 novos + 33 irmãos, incluindo os 5 P1303
reconfirmados GREEN). Zero regressão em typst-core (6059/6059).
## P1482 — campo estático `color.spot.tint` (CONFIRMADO em 2026-09-23; ADR-0127)

### Medição anterior à decisão

`compiler/eval/bindings/field_access.rs:774-919` tem lookup de
`Type::Color` mas nenhum ramo de `Type::SpotColorant`.
Vanilla: `type(color.spot.tint) == function`,
`repr(color.spot.tint) == "tint"`,
`color.spot.tint(s, 50%)` funciona e `s.tint` isolado erra
`cannot access fields on type spot colorant` (sondas P1482).
O constructor P1473 continua em `color.spot(...)`.

### Decisão proprietária

`Type::SpotColorant` expõe somente o campo estático `tint` com
`Func` nativa do owner `compiler/stdlib/color.md`; campos
desconhecidos conservam o erro de tipo. `Value::SpotColorant` não
adquire field access de instância: a chamada sintática ligada é
interceptada exclusivamente em `compiler/eval/call_dispatch.md`.
Não alterar lookup de `Type::Color` nem introduzir função ligada
first-class ausente do vanilla.

## P1485 — acesso gated a `format.bundle` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, P1484
cobre `std.asset` e `format.html`, mas
`01_core/src/compiler/eval/bindings/field_access.rs:497-535`
não conhece `format.bundle`. O vanilla pinado sem feature
diagnostica ``cannot access field `bundle` because the `bundle`
feature is not enabled`` no field de `format.bundle` com os
dois hints do gate; com feature devolve função nominal `bundle`.
Um Dict ou módulo alheio com field homônimo não é esse binding.

### Decisão proprietária

No módulo `format` efetivo, o field `bundle` sem
`Feature::Bundle` devolve exatamente o diagnóstico/hints acima,
antes do lookup normal; alias de `format` e `std.format` conservam
o gate. Com feature, o lookup devolve a função de
`compiler/stdlib/format.md`. O controle de módulo efetivo não
intercepta Dict/Module arbitrário ou outro field. O gate P1484
de `std.asset` permanece; não converter ausência em sucesso quando
o constructor/target não estiverem completos.

## P1495 — depreciação dos bindings de símbolos (P1496)

### Medição anterior à decisão

No vanilla pinado `9f2b6e871`, `typst-library/src/symbols.rs:12-35`
inclui a metadata de `codex::SYM` tanto em `sym` quanto em `math`.
Sondas P1495: `sym.spacebar` e `math.spacebar` resolvem ␣ e
avisam `` `spacebar` is deprecated, use `keyboard.space` instead ``;
`join` está ausente em ambos. No cristalino P1494,
`field_access.rs:618-629` emite warning só para módulo chamado
`sym`, e o faz antes de verificar se o field existe; por isso
`math.spacebar` fica silencioso e uma remoção mecânica de `join`
avisaria antes de falhar. Proveniência no diagnóstico P1495.

### Decisão

Depois de resolver com sucesso um binding de símbolo de topo do
módulo efetivo `sym` ou `math`, emitir sua depreciação pinada com
span no field. `spacebar` avisa nos dois namespaces. `join`,
preservado como extensão por `compiler/stdlib/sym.md`, conserva
o aviso histórico apenas em `sym`; `math.join` fica silencioso.
Não criar warning novo para esta extensão sem gate ADR-0127.
A mesma regra pós-lookup vale para a rota qualificada em modo math
(`std.sym.spacebar`, `std.math.spacebar`), que deve chamar o hook
deste owner em vez de omiti-lo no lookup puro. Acesso a campo
inexistente não avisa antes de errar. Dict, módulos homônimos alheios,
modifiers de `Symbol` e outros campos não adquirem warnings
por nome parecido. Warnings de variantes seguem a dívida
separada P1283; não ampliar o carrier público neste passo.
Todos os diagnósticos não relacionados mantêm a precedência atual.

### P1496 — acesso seletivo aos aliases `tri`

A medição de `repr(sym.gt)`/`repr(sym.lt)` no HEAD P1495 e no
vanilla pinado (registrada em `compiler/stdlib/sym.md`) mostra que
injetar `tri*` nos parents novos adulteraria sua morfologia. Depois
de tentar `Symbol::modified(field)`, apenas para `field == "tri"`
e um `Symbol` do parent canônico `gt`/`lt`, ainda sem modifiers
ou com somente `eq`/`not` aplicados, resolver o mesmo conjunto mais
`tri` pelo `sym_lookup` seletivo. Confirmar igualdade estrutural
com o parent codex submetido aos modifiers já aplicados; um
`Symbol` arbitrário de nome semelhante não ganha a extensão.
Os modifiers subsequentes `.eq`, `.eq.not`, `.not` usam o
`Symbol` legado resultante; permutações antigas como
`gt.eq.tri.not` continuam válidas. Não enriquecer o parent, ampliar esse
fallback a outros modifiers/symbols ou transformar erro em warning.
Isto conserva os oito paths públicos e mantém `repr(gt/lt)` do
catálogo pinado sem alterar o carrier nem o contrato de
`entities/symbol.md`.
