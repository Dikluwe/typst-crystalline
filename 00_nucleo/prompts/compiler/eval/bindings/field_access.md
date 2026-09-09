# Prompt L0 — `compiler/eval/bindings/field_access` — acesso a campo sobre valores e `Content`
Hash do Código: a4267e57

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## P1338 — field ausente em instância Array

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1338-measurement.json`, SHA-256
`6bd65bffd487cf7103bbaa54e3655ff95467df557c852f6e5c7b44c96ea165e1`,
concluído em `2026-09-09T20:06:19.061817+00:00`, mede `(1,2).nope`,
`().absent` e alias Unicode multilinha: o cristalino publica
`array does not contain field "<field>"` sobre o acesso inteiro; vanilla
publica `cannot access fields on type array` somente sobre o identificador.
Baseline SHA-256 `6d85aece9e323950a8f722b11e87eb125b4a342a7404f969f60ea69595790b3c`,
working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, diff/stat e inventário exatos.
Vanilla ratificado upstream a51e02804 SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
antecedente P1337 SHA-256
`55b5dc263bba15b050e9caab755c17deb22f617eec35b1e9259a20a57f9b350b`.

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
Correção interna ADR-0127 contínua, L0 primeiro + resselo + RED→GREEN.

Aceitação: testes independentes puro/AST, arrays vazios/populados/heterogêneos,
aliases/Unicode/multilinha, diagnóstico completo e positivos/exclusões;
matriz bilateral pré-classificada em quatro perfis e três ordens, mutantes
realmente compilados e rejeitados por testemunhas. Unknown obrigatório bloqueia.
Este contrato é deliberadamente limitado ao erro ausente; não declara concluída
a paridade de fields Array nem a dívida de len/first/last e pré-despacho.

## P1337 — field ausente em Bool, None e Auto

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1337-measurement.json`, SHA-256
`f318ee743fa9adfc372f1b44946897438ffdbd5eba0ba329f57ddee10bb8eab2`,
concluído em `2026-09-09T19:09:59.299611+00:00`, mede `true.nope` e
`false.ausência`: cristalino publica bool, vanilla boolean. `none.nope`,
`auto.nope` e aliases multilinha já têm o nome correto, mas, como Bool,
marcam o acesso inteiro em vez do field. Valores válidos coincidem.
Baseline SHA-256 `d84ebca44348e4d12d3a7d243a35751edda1b898599a03a898634302cd0ec672`,
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada
com diff/stat e inventário exatos. Vanilla ratificado upstream a51e02804 SHA
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
antecedente cristalino SHA
`646a8d97400c0abe262a65c9b9559b47a3ecf7d10eafd82fa79a64ade0504497`.

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
paridade ADR-0127: L0 primeiro, resselo, RED→GREEN e revalidação contínua.

Aceitação: testes independentes puro/AST e matriz bilateral congelados antes
de C, positivos, fronteiras e dívidas pré-classificadas, quatro perfis e
três ordens; mutantes aplicáveis compilados e rejeitados por testemunhas.
Unknown obrigatório bloqueia. Não declarar paridade geral nem somar
preservação de dívida como convergência.

## P1336 — field ausente em instância Int/Str

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1336-measurement.json`, SHA-256
`0865c045fde6b0d3689a77d83a9d255ea927e76185ad7cfb0dcb6dfb8ca3e375`,
concluído em `2026-09-09T17:56:26.736676+00:00`, preserva comandos, fontes,
UTC e canais integrais. Baseline P1336 SHA-256
`e81e034167a994ab3dd1c318a7dd7384cd18ef75e0d2b3c1c666eb26724e441f`,
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada
com diff/stat e arquivos exatos. Vanilla ratificado upstream a51e02804 SHA
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
cristalino antecedente SHA
`11e3164fa509030cc78dc048d5bb4f2426348e32a24edc7cd2f320c704e6ef61`.

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
L0-first + resselo, RED→GREEN e revalidação. Aceitação exige testes locais e
A/B independentes congelados antes do candidato, nomes/valores variados,
aliases, parênteses, multilinha e Unicode, lookup puro, sucessos e exclusões;
comparação integral nos quatro perfis, repetição/inversão e mutantes aplicáveis
rejeitados. Unknown obrigatório bloqueia; preservação de uma dívida não
significa igualdade com vanilla nem fechamento geral de fields.

## P1326 — field ausente em closure definida pelo usuário

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1326-baseline.json`, SHA-256
`3198d97b15c02085af158e88e44c4025e4f76b29654e7309c2ca21fdb2a41f79`,
concluído em `2026-09-09T01:23:32.345952+00:00`, preserva HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado,
lista exata/diff/stat dos dez arquivos tracked alterados e comandos/saídas.
Baseline P1325 SHA `3511b08aa89d088e908dd239d8942f1eeea1978d31140ef9510e81de06023dab`;
vanilla ratificado `a51e02804`, SHA
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
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

Em `2026-09-09T00:49:40.661695+00:00`, o recibo
`00_nucleo/diagnosticos/p1325-baseline.json`, SHA-256
`e2ed7ae9f8290ee9557b38d151ffffebae760b79fa1abb64979d6c1eb9d7998b`,
preserva HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não
commitado, lista exata/diff/stat dos oito arquivos tracked alterados, comandos,
horários, saídas e hashes dos executáveis. Baseline P1324 SHA-256
`c5c9aa39c8b7053a19f53bf9f37c8d3731a4081683a51cf8ca5e9e9c0197d2f7`;
vanilla ratificado upstream `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

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
nova. L0-first, resselo, RED→GREEN e revalidação em fluxo contínuo.

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

Baseline R5 `00_nucleo/diagnosticos/p1307-r5-baseline.json`, SHA-256
`32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`:
HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não
commitado com diff/stat integral. A medição independente R5, SHA-256
`82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`,
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
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, congelado em
`00_nucleo/diagnosticos/p1311-baseline.json`, SHA-256
`2338de6be4567c9ea03be335c5935832241d8010272b10eb29c1ca858c9af753`.
Medição fresca `p1311-measurement.json` em diagnósticos, SHA-256
`f444074ac6c1e8be4509b08eed480ceb1e0188f7e8afec892bdaa31938a3d6c3`,
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

`00_nucleo/diagnosticos/p1324-baseline.json`, SHA-256
`8684dabd1370997232a61bc58a48a99528718e49ec0e81962eb14c70b3769b4d`,
registra a medição fresca sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada,
diff/stat, UTC, fontes e binários. Vanilla ratificado upstream `a51e02804`,
SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`:
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
Correção interna ADR-0127: L0 primeiro, resselo, RED→GREEN e revalidação.

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
callback selada de `math.cancel(angle:)` executa com o snapshot correto,
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

## P1293.reopen-A — span do campo ligado `is-nan` sem chamada

### Medição anterior à decisão

Em `2026-09-01T15:13:15-03:00`, sobre HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não commitida, o
recibo segregado `p1293-implementation-receipt-a.md` de SHA-256
`14ca51a7b440ce65e46eda9dadd7b47a21e15dd7a991b193e5d103beac42ced1`
mediu `float("NaN").is-nan` com a mensagem pública correta, mas range
cristalino `0..19` contra `13..19` no vanilla ratificado e no contrato.

No consumer vigente e ainda sem patch desta reabertura,
`field_access.rs:106` entrega `access.span()` ao lookup comum e
`field_access.rs:441-446` reutiliza esse span total no diagnóstico. A própria
AST já expõe `access.field().span()` — precedente local em
`field_access.rs:99-101` —, que corresponde exatamente ao identificador
`is-nan` medido. Esse erro não atravessa a nativa nem `Args`; portanto este nó
é o owner causal da quinta âncora.

### Classificação e decisão

O span publicado pelo diagnóstico é linguagem sob ADR-0107. A intenção P1293
já exige o range exato; trocar somente a âncora interna é correção de paridade
em fluxo contínuo ADR-0127, sem contrato Rust público, default ou mudança de
fase.

Quando `eval_field_access` recebe exatamente `Value::Float` e o campo
`is-nan` como acesso ligado sem chamada, o erro vigente
`cannot access fields on type float` deve usar `access.field().span()`, nunca
o span total do acesso. Mensagem, severidade, hints e ausência do valor ligado
permanecem idênticos. Todos os outros targets, fields e erros conservam a
âncora atual; não generalizar esta regra, não criar reflexão e não fabricar um
método como valor.

É proibido alterar `entities::Args`, API pública, entidade, default, ordem de
avaliação ou fase. Este Prompt continua proprietário 1:1 apenas de
`01_core/src/compiler/eval/bindings/field_access.rs`; `call_dispatch` possui as
quatro âncoras de chamadas e `stdlib/foundations/float` possui a função.

## P1301 — diagnóstico de field ausente em `Module`

### Medição anterior à decisão

Em `2026-09-03T20:57:32.863761582-03:00`, no HEAD
`1f082370e59939de7b57992e137a9f74bfb6758f` e working tree não commitida,
`00_nucleo/diagnosticos/p1301-pre-gate-measurement.json` mediu o vanilla
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

Classificação: correção de paridade em fluxo contínuo ADR-0127. O dono ainda
autorizou explicitamente a reabertura dedicada em `2026-09-03`, após o
veredito `P1300_BLOCKED_IMPLEMENTATION`. O gate funcional é RED→GREEN contra o
oracle vanilla, seguido de nova cadeia segregada e resselo; a cadeia P1300 não
é reutilizada.

## P1303 — span dos fields `pdf.*` bloqueados por `a11y-extras`

### Medição fresca anterior à decisão

Em `2026-09-04T00:38:07.847942-03:00`–`00:38:10.279557-03:00`, no HEAD
`5b4a0d0438a535c54fdb5e74b28903c1313f5bc2` e working tree não commitada
sem diff tracked ou staged, o oracle fresco
`00_nucleo/diagnosticos/p1303-pre-measurement.json` (SHA-256
`ef3a4eb0b6fcb3fb0e9b1d8ec54bfbfa8f1a4f7b58dd7c675cbf677bada573d4`)
executou `48` runs e `24` comparações, em ordem normal e invertida, contra o
vanilla ratificado de SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` e um
binário cristalino release fresco de SHA-256
`28667d00fd9959344981b23060594d112ada0ccab2ae1bd2e39955b851b4de8a`.

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
de compatibilidade. O fluxo é L0-first + resselo e segue sem nova paragem humana
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
geral de mensagem e span de `Module` selada por P1301r2; ou regredir
a exceção field-only de `float("NaN").is-nan` selada por P1293.
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
`00_nucleo/diagnosticos/p1306-contract-measurement.json` (SHA-256
`d3066b1bf00ff3120190e329752df7547758f5b03f367a22c6cded83f31310fb`)
registrou fixtures, hashes, comandos, cwd, horários, saídas e status completos
antes/depois. A árvore continha somente untracked enumerados no recibo. O
baseline certificado P1305 tinha SHA-256
`be51045f1df75ac42ee081801ddf7f738f709029e3694b9205473ba5fa8b31d4`; o vanilla
ratificado upstream/main `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

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
integralmente no recibo como falha de instrumentação, sem valor de RED.

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
resselo, a correção interna segue por RED→GREEN e revalidação, sem alteração
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
preservam valor, kind, chamada e repr. Os nomes e os imports certificados P1305
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
Snapshot adicional R3 SHA-256
`592497d1e786241b9ba121479c0c55dbad370a70732e130b4f7ac3bd709d4405`
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

Medição adicional, anterior à decisão: no baseline R4 SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
`01_core/src/compiler/eval/bindings/field_access.rs:828-830` clona Args e
remove o receiver apenas de items. O L0 R3 cobriu o consumo de default, mas
não este writer. Com o contrato Args aprovado, o wrapper deve consumir por
`remove_positional(0)`, preservando o clone original, span agregado e todas
as ocorrências restantes. Não invalidar Some nem mudar os erros vigentes.
É extensão interna da mesma migração, não nova API ou fase; verificar a
equivalência estática/ligada de content.func/has/at/fields/location. A fonte
e a lista de writers refutam a suficiência da enumeração R3, não a API aprovada.
