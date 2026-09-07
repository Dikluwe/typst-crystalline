# P1307-R4 — inventário final de produtores, escritas e transporte de Args

## Resultado e limite do veredito

Auditoria documental, sem candidato e sem alteração de Rust. A cobertura dos
**18 owners pinados em R3 não era suficiente**: faltavam os transportes
produtivos de `compiler/eval/bindings/value_methods` e
`compiler/stdlib/counter`; no owner já incluído `field_access`, faltava
explicitar a remoção do receiver em `static_content_receiver`.
São extensões de cobertura do carrier aprovado, não necessidade demonstrada
de novos campos ou de novos comportamentos de linguagem.

Esta conclusão compara a fonte com os pins R3, não com as emendas que o
coordenador está redigindo concorrentemente em R4. As emendas precisam de
pins próprios e revisão antes de materializar os respectivos consumers.
Não declarar implementado o carrier nem certificar o candidato a partir
deste inventário.

Skill aplicada: `tekt-materializacao-segregada`, com leitura integral do
SKILL e referências obrigatórias. Papel: auditor de transporte; regime
**executado sem atestação de isolamento técnico**. Nenhum Rust, L0,
oráculo, ataque ou predecessor foi escrito por este auditor nesta rodada.

## Proveniência anterior à classificação

- HEAD: `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não commitado.
- Snapshot R4: `00_nucleo/diagnosticos/p1307-r4-baseline.json`,
  SHA-256 `52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
  capturado em `2026-09-07T17:30:53.345311+00:00`.
- Preflight: `00_nucleo/diagnosticos/p1307-r4-preflight.json`,
  SHA-256 `d18bfb5cf49a862d04ca7fa3c1676855dd6baed299d4531eaf9e9b8df75503cf`.
- Pins comparados: `00_nucleo/diagnosticos/p1307-r3-lineage.json`,
  SHA-256 `3b34aed0c540bac2dc0b96872596fa16859c10c6df6d69259a8fa744ddaa94eb`.
- O inventário de fontes e seus SHA-256 completos está em
  `source_inventory` do snapshot R4. Em
  `2026-09-07T17:36:31.413512+00:00`, os **473 arquivos Rust** ali
  registrados foram novamente hashados: **zero diferenças**. Este número
  verifica integridade do snapshot, não conta writers nem prova análise formal.
- Não houve execução de binário nesta auditoria. As referências de paridade
  continuam sendo as medições R2/R3; não se inferiu comportamento novo de
  versões nominais de Typst.

Diffstat exato do estado de código/documentação de origem:

```text
 00_nucleo/prompts/compiler/eval.md                 |  44 +++-
 .../prompts/compiler/eval/bindings/field_access.md | 123 ++++++++-
 .../compiler/eval/bindings/method_dispatch.md      |  31 +++
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |  76 ++++++
 00_nucleo/prompts/compiler/eval/closures.md        |  44 ++++
 00_nucleo/prompts/compiler/eval/math.md            |  33 +++
 00_nucleo/prompts/compiler/eval/operators/join.md  |  43 +++
 00_nucleo/prompts/compiler/eval/repr.md            |  68 ++++-
 00_nucleo/prompts/compiler/eval/tests.md           | 147 ++++++++++-
 00_nucleo/prompts/compiler/stdlib/_comum.md        |  41 ++-
 00_nucleo/prompts/compiler/stdlib/collections.md   |  61 +++++
 .../prompts/compiler/stdlib/foundations/float.md   |  27 ++
 .../prompts/compiler/stdlib/foundations/int.md     |  33 +++
 00_nucleo/prompts/compiler/stdlib/gradients.md     |  28 ++
 00_nucleo/prompts/compiler/stdlib/loading.md       | 111 ++++++++
 00_nucleo/prompts/compiler/stdlib/numbering.md     |  27 ++
 00_nucleo/prompts/entities/args.md                 | 290 ++++++++++-----------
 00_nucleo/prompts/entities/func.md                 |  75 ++++--
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 01_core/src/compiler/eval/tests.rs                 | 198 +++++++++++++-
 20 files changed, 1313 insertions(+), 191 deletions(-)
```

Os dois Rust já modificados neste diff são herança P1306 do baseline, não
implementação P1307. O JSON preserva o diff integral e a lista completa do
estado, inclusive arquivos não commitados.

## Método e critério de cobertura

A descoberta percorreu Rust em `01_core/src`, `02_shell/src`,
`03_infra/src` e `04_wiring/src`: ocorrências do tipo Args, construtores
`Self`/`Args`, `Value::Args`, aliases, assinaturas mutáveis, clones,
reconstruções, concatenações, remoções, atribuições e mutações de
`items`/`named`. Em seguida foram lidos os corpos e o fluxo de dados
das funções candidatas, os builders da AST e os limites de módulos
`#[cfg(test)]`. Nomes locais como `rest`, `synth` e
`layout_args` foram seguidos; portanto o resultado não se reduz à
busca por literais `Args {`.

Foi feita classificação manual de corpo/AST e origem; não se executou
analisador Rust tipado ou expansão formal de macros. Bibliotecas externas e
fontes futuras não estão implicitamente certificadas. A busca textual é
instrumento de descoberta, não evidência de exaustividade por contagem.
Os L0 dos owners de transporte afetados foram lidos integralmente antes
da classificação, incluindo os dois owners adicionais e field_access.

Critérios usados:

- **Causal**: Args vindo do usuário/AST ou de outro Args com origem conhecida.
  Deve manter `Some(occurrences)`, valores, ordem, nomes e os dois spans.
- **Sintético**: argumentos novos derivados de estado, geometria, conteúdo
  ou callbacks sem correspondência lexical direta. `None` é honesto;
  agregar um span de chamada não fabrica origem individual.
- **Terminal**: projeção/consumo em valores que não reenvia Args.
  Não exige reconstruir carrier só para descartá-lo.
- **Teste sintético**: fixture `None`; adaptar um literal para compilar
  não cria automaticamente nova obrigação semântica nem novo owner.

Os caminhos de Rust abreviados nas tabelas seguintes têm prefixo
`01_core/src/`, salvo quando explicitado. As linhas são do snapshot R4.

## Inventário produtivo causal e transformações

| Fonte e linhas | Corpo/origem verificado | Cobertura L0 R3 e obrigação |
|---|---|---|
| `entities/args.rs:18,35` | Campos públicos; construtor `Self` de posicionais. | `entities/args.md`: carrier opcional, projeções, helpers, clone/igualdade e criação sintética. |
| `entities/func.rs:39,237` | `With(Arc<Func>, Args)` e construção de partial. Transporta Args inteiro. | `entities/func.md`: não requer novo campo de Func; clone mantém carrier. |
| `entities/value.rs:143` | Variante Args, clone do valor inteiro; acessos de tipo em 343/397. | Transporte sem reconstrução ou mutação de Args; não demanda reabrir owner Value. |
| `compiler/eval/call_dispatch.rs:389-430` | `eval_args`: positional 403, named 405-408, spread 410-419; Args spread 416 desmonta projeções. | `call_dispatch.md`: builder causal com spans AST; Args spread não pode perder ocorrências/cross-source. Array/Dict spread não inventa origem lexical de elementos. |
| `compiler/eval/call_dispatch.rs:434-468,1016-1023` | Dispatch de closure/native; With concatena posicionais e atualmente faz map.extend. | `call_dispatch.md`: concatenação causal de With, sem colapsar duplicatas e sem duplicar validação de encoders. |
| `compiler/eval/call_dispatch.rs:606-628` | `p1284_static_call`: literal 619 recompõe resto após receiver. | Mesmo owner: remover receiver pelo helper, preservar o resto causal. |
| `compiler/eval/call_dispatch.rs:1225,1578-1582` | Ajustes do span agregado. | Mesmo owner: não sobrescrever spans individuais; não invalidar origem por alteração apenas agregada. |
| `compiler/eval/call_dispatch.rs:1325-1330,1632` | .with armazena Args; Type::Arguments devolve Value::Args. | Mesmo owner: transportar carrier inteiro, inclusive ao escapar em alias/closure/valor. |
| `compiler/eval/closures.rs:77,83-86,117-131` | Consumo de named/positional; sink literal 122 recompõe o restante. | `closures.md`: remover ocorrências consumidas sem reintroduzi-las no sink; defaults/erros existentes não ganham novas políticas. |
| `compiler/eval/math.rs:322-341,570-596,1330-1351` | Três builders próprios da AST, não apenas eval_args. | `math.md`: preservar origem dos positional/named já aceitos. Spread ignorado nesses caminhos é dívida anterior explícita, não implementação autorizada neste transporte. |
| `compiler/eval/operators/join.rs:73-76` | Args + Args altera items/named. | `operators/join.md`: operação distinta de With; remover todas as named da esquerda sombreadas pela direita, concatenar restante + direita, agregado detached. |
| `compiler/stdlib/collections.rs:511-563` | Static collection receiver; literal 545 recompõe resto. | `collections.md`: remoção coerente do receiver, preservando causalidade do restante. |
| `compiler/stdlib/collections.rs:2402-2452` | arguments.filter reconstrói Args; loops atuais por projeção. | Mesmo owner: filtrar sequência causal preserva ocorrência inteira; agregado detached. |
| `compiler/stdlib/collections.rs:2455-2487` | arguments.map reconstrói Args e substitui valores. | Mesmo owner: ordem causal, span da ocorrência preservado, value_span detached e agregado detached; regenerar projeções. |
| `compiler/stdlib/gradients.rs:97` | Prepend do receiver gradient em clone de Args. | `gradients.md`: receiver sintético detached + sequência causal original. |
| `compiler/stdlib/foundations/float.rs:120,124` | Prepend de float em Args recebido. | `foundations/float.md`: mesma regra, não invalidar o restante. |
| `compiler/stdlib/foundations/int.rs:56,149-153,264-268` | Prepend de int; shift/to-bytes reconstruções que removem named. | `foundations/int.md`: preservar ocorrências posicionais, remover named de modo coerente; não injetar receiver onde a fonte só elimina named. |
| `compiler/eval/bindings/method_dispatch.rs:36-46,336,388` | expect_positional remove items[0]; remoções de default. | `bindings/method_dispatch.md`: helpers aprovados, removendo todas as ocorrências do nome quando a projeção é consumida, sem nova política de cast. |
| `compiler/eval/bindings/field_access.rs:750-768` | Consumo delegado de positional e remoção de default 767. | `bindings/field_access.md`: já coberto no texto R3. |
| `compiler/eval/bindings/field_access.rs:804-831` | static_content_receiver clona 828 e remove items[0] em 829. | **Lacuna no texto R3 do mesmo owner**: precisa preservar origens do Args restante; não requer novo owner. |
| `compiler/eval/bindings/value_methods.rs:190-244` | eval_color_method chama eval_args 201; synth.items.insert 202. | **Owner fora dos 18**: prepend coerente, receptor detached e origens recebidas preservadas. |
| `compiler/eval/bindings/value_methods.rs:248-314` | parse_counter_display_args cria Args 256; aceita AST named at/both 268/272 e positional 284. | **Mesmo owner faltante**: builder causal dos argumentos já aceitos, sem nova aceitação de spread/names/casts. |
| `compiler/eval/bindings/value_methods.rs:457-570` | eval_counter_static_method_value: ramo display 524-548 cria Args depois de consumir receiver AST. | **Mesmo owner faltante**: preservar spans das AST aceitas e causalidade do restante. Não classificar como sintético só por usar Args::positional vazio. |
| `compiler/stdlib/counter.rs:151-161` | static_counter_receiver clona em 158, remove positional 159, retorna resto. | **Owner fora dos 18**: usar remoção coerente de receiver. |
| `compiler/stdlib/counter.rs:207-236` | native_counter_step_static remove named level 216 ou positional 220, depois verifica resto. | **Mesmo owner faltante**: helpers coerentes; manter escolha/validação existentes, sem impor política nova de cast de todas as ocorrências. |

O percurso por clones confirma que `None` não pode ser a solução universal
para writers produtivos: uma vez que eval_args produz `Some`, os
receivers e sinks acima podem transportar origem de outra fonte.
Também não é aceitável reconstruir a origem comparando valores iguais.
Para uma mutação direta de fixture já `None`, nenhuma invalidação
redundante é necessária. Para user-origin `Some`, invalidar e descartar
origem não satisfaz o contrato aprovado.

## Produtores sintéticos e consumo terminal

| Fonte e linhas | Classificação e motivo |
|---|---|
| `compiler/eval/call_dispatch.rs:1468-1472` | Args de callback de layout com size_dict calculado de geometria. `from_parts(..., call.span)` pode continuar sem origem individual. |
| `compiler/stdlib/numbering.rs:36-40,103-107` | Numeração deriva Int de usize/u32. Preservar span agregado não legitima atribuir o span lexical do número original a um valor transformado. Owner R3 já cobre migração sintética. |
| `compiler/stdlib/collections.rs:687,1030,1048,1102,1137,1155,1189,1220,1260,1381,1552,1576,2304,2323,2415,2434,2467,2480` | Args::positional de callbacks: cast, sort, filter/map/predicados, dedup, fold/reduce e callbacks de dictionary/arguments. Valores individuais são entradas novas para callback, não Args originais encaminhados. Podem permanecer None. |
| `compiler/stdlib/collections.rs:164-171,2351-2399` | Despacho arguments e len/pos/named/at leem projeções ou devolvem valores. Apenas map/filter recriam Args. Consumo de items em 1527 para fold não reenvia o Args consumido. |
| `compiler/eval/bindings/field_access.rs:47` | Chamada implícita sem argumentos: Args vazio sintético. |
| `compiler/introspect/from_tags.rs:65,105,181,286,347,405` | Callbacks derivados de estado/counter/content/equação/supplement. Posicionais calculados, sem Args lexical a preservar. |
| `compiler/eval/rules.rs:537,659,745,905` | Show/regex/text/label passam Content ao callback. Nova lista sintética; nenhuma reconstrução de Args do usuário. |
| `compiler/stdlib/state.rs:115` | Callback display recebe valor de estado resolvido; sintético. |
| `compiler/stdlib/counter.rs:480` | Callback display recebe valores atuais do counter transformados em Int; sintético, embora outros corpos desse arquivo exijam a emenda acima. |
| `03_infra/src/pipeline.rs:273,881,898` | Callbacks de context vazios e math.angle com Angle calculado. Args::positional continua None; sem nova obrigação de transporte lexical no owner infra. |

`repr.rs:108-120` e loading são leitores do valor e encoders/diagnósticos,
não novos writers de Args. Sua observabilidade causal e seleção dos
argumentos já são obrigações dos respectivos owners R3. `eval/mod.rs`
registra nativas/valores, sem reconstrução adicional do carrier detectada.

## Testes, homônimos e migração mecânica

| Fonte | Classificação pelo corpo e escopo |
|---|---|
| `compiler/eval/call_dispatch.rs:1710-1711,1853` | Literal dentro de cfg(test)/mod tests; migração de fixture None, owner já incluído. |
| `compiler/stdlib/collections.rs:2677-2678,2681-2688` | make_args e named.insert são fixture no módulo de testes. |
| `compiler/stdlib/mod.rs:207-208,281,290,2899` | Helpers e literal de testes; várias mutações subsequentes usam Args sintético. O L0 _comum já prevê migração, sem invalidações redundantes de None. |
| `compiler/stdlib/shapes.rs:1152-1153,1213,1224,1233` | Literais exclusivamente em testes; owner existente `00_nucleo/prompts/compiler/stdlib/shapes.md`. |
| `compiler/stdlib/plugin.rs:163-164,558,568` | Literais exclusivamente em testes, um com span agregado; conservar esse span em from_parts, origem None. Owner existente `00_nucleo/prompts/compiler/stdlib/plugin.md`. |
| `compiler/stdlib/structural/mod.rs:65-66,140,557,574,586,628,652,664,677,696,708,720,735,754,766,928` | Helper e literais de testes, não writers produtivos. Owner existente `00_nucleo/prompts/compiler/stdlib/structural.md`. |
| `compiler/stdlib/context.rs:46-47,100` | Args em teste, não o callback produtivo de context. |
| `compiler/stdlib/primitives_constructors.rs:17-18,33,37,39` | Helpers de test_support sob cfg(test). |
| `compiler/stdlib/pdf.rs:342-343,394`; `foundations/float.rs:131-132,279`; `compiler/stdlib/html.rs:1648-1650` | Helpers/mutações de módulos de teste, não nova rota causal produtiva. |
| `02_shell/src/cli.rs:129` | Args da CLI/clap, tipo homônimo distinto da entidade. |
| `compiler/eval/math.rs:25`; AST/parser | Alias Args as AstArgs e nós Args sintáticos não são novos layouts Rust do carrier. Seus builders produtivos foram inspecionados acima. |
| `items` de Frame/layout e outras coleções | Falsos positivos da busca de campo; não são Args. |

Um literal de fixture que exige adaptação devido ao novo campo público não
autoriza trocar o comportamento do teste, a expectativa ou seu oráculo.
Onde `Args::positional` já compila com assinatura preservada, não há
razão automática para alteração. Onde houver escrita mecânica necessária
fora da allowlist produtiva, identificá-la e preservar o owner 1:1 existente;
não confundir esse inventário mecânico com reabertura semântica obrigatória
de cada owner de teste.

## Testemunhas das lacunas

SHA-256 de fonte e L0 **antes** das emendas R4:

| Caminho | SHA-256 |
|---|---|
| `01_core/src/compiler/eval/bindings/value_methods.rs` | `139873711c2d6fba0216152753e602a90083c98db5850e5495a13504b48bb522` |
| `00_nucleo/prompts/compiler/eval/bindings/value_methods.md` | `85d31916ee6091f9853e2605760c3ecc41d4be5c5a702f1fa7e4b3bacc1b6091` |
| `01_core/src/compiler/stdlib/counter.rs` | `873aa133447ba6402cf7949b900d562dc513f32397a84ca7ee0784dd6c1dd4e7` |
| `00_nucleo/prompts/compiler/stdlib/counter.md` | `a4802f75d7437152b3f738c90552db359e7a27efb71187636821eed8d407d27f` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `bcec86b9e0853d2c1c5e5e159d06908bde9e2ef8203daa82b8d2fa73cfc65557` |

Pins L0 R3 comparados com sucesso antes das emendas:

| Prompt | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `645dcc4e7c16d48b1b36b9a0d12dc385f13d6a6f940fb5fab8e8381e24d20dda` |
| `00_nucleo/prompts/compiler/eval/bindings/method_dispatch.md` | `7360a5335f0adcb1ce45deea487c93347bbf9cefcfb1e8cbfceb9164d336e521` |
| `00_nucleo/prompts/compiler/eval/call_dispatch.md` | `6f1e02969301600a8b78782133736f956f63c3f91e5273867c116bf17d730588` |
| `00_nucleo/prompts/compiler/eval/closures.md` | `dbf7bccc186ad25035db5b7ee1d581d001d805dfc42dc1b5f8944fc1264a9309` |
| `00_nucleo/prompts/compiler/eval/math.md` | `409c81da6419b359e46dd970811100367edfdadcc7e59a2cd14799d1ef12335c` |
| `00_nucleo/prompts/compiler/eval.md` | `05b4eac0f2e814c67c75ae1cdcaa08083665da9878a9f67eda0d773e73eefdbb` |
| `00_nucleo/prompts/compiler/eval/operators/join.md` | `155d40ba7397b5ae28a23456f57509b4f5f0ae822d56c3e28901637d6521cf66` |
| `00_nucleo/prompts/compiler/eval/repr.md` | `855c657c79141e608b47f1bf942e16e1a579a292c92f13337d449df058b8cd93` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `606e41df1d5e55ad9fa51662695ed9faf937b35d1dea443067488e3c14dfc3b6` |
| `00_nucleo/prompts/compiler/stdlib/collections.md` | `f5eadc7eb3ec75c73c1ced46d87670f47705c956e1f41acb4a5310bc1ad6b781` |
| `00_nucleo/prompts/compiler/stdlib/foundations/float.md` | `68e877cfdaf7feaf7fee344c268244d04df13707f3aa2beeba83bf8513f49794` |
| `00_nucleo/prompts/compiler/stdlib/foundations/int.md` | `2037bf60d00edd067d84f7814bbb3229f26e61b2f6e3412c458fda316bb76e3c` |
| `00_nucleo/prompts/compiler/stdlib/gradients.md` | `25ab680bf8fb4f6f6efef8a2d3f66c9cf5e05589977484848a843e6b744807e8` |
| `00_nucleo/prompts/compiler/stdlib/loading.md` | `783fa3d7c087767b13f4bd2cc3e5a3aa6a1e34632bcf2a6bd3c39c766cc61976` |
| `00_nucleo/prompts/compiler/stdlib/_comum.md` | `87c18ca349ee8669dd242d85be09f95ab792336e6e35941f7663d36f23932211` |
| `00_nucleo/prompts/compiler/stdlib/numbering.md` | `3ef25a8ed94b9d41b088e6c515859d5e011f321d21b9d4cf22b992991bb5ebfe` |
| `00_nucleo/prompts/entities/args.md` | `75de6ac49d69331fc604984c92874a706442f823cc4706bd480b434c55c83d94` |
| `00_nucleo/prompts/entities/func.md` | `523f25d1e5926d52c1402c911c66444df7c61d6bf21e720ee692e77b68b1c0db` |

## Fechamento refutável e gates

A evidência de origem/uso, e não a semelhança do nome da função, sustenta
a classificação. O inventário seria refutado por um writer produtivo
não listado que receba `Some` e mutile projeções, ou por um produtor aqui
classificado como sintético que realmente reconstrua Args lexical recebido.
Tal descoberta exige incorporar o corpo e seu owner antes de código.

As três lacunas documentais identificadas são cobertas pelo desenho público
já aprovado de Args: `from_occurrences`, sequência de ocorrências,
`remove_positional`/`remove_named` e criação sintética `from_parts`.
Não foi encontrada necessidade de nova API pública além desse contrato.
Isso não autoriza corrigir dívidas históricas de spread de math/counter,
mudar casts de counter/int, nem generalizar a validação dos encoders no
dispatch. Tais mudanças refutariam a alegação de mera extensão de transporte
e exigiriam nova decisão L0/gate apropriado.

Antes da implementação: incorporar e pinar os dois owners adicionais e a
emenda de field_access, completar revisão segregada e congelar as obrigações
independentes. Depois: RED antes de candidato, testes de coerência após
cada transformação, alias/closure/cross-source e pares de valores iguais
com origens distintas; validar separado o caso sintético None. A comparação
de hashes acima apenas preserva baseline; não substitui RED/GREEN,
ataques, revisão independente ou lint/linhagem final.
