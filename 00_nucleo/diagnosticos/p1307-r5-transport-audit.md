# P1307-R5 — auditoria independente do transporte de campos realizados

Estado: proposta documental; **não autoriza materialização**. Papel: auditor de
fonte/transporte, distinto do autor da implementação e do oráculo. Regime:
executado sem atestação de isolamento técnico. Não houve execução de candidato,
alteração de Rust, L0, testes ou oráculos nesta auditoria.

## Proveniência e leituras

Baseline R5: `p1307-r5-baseline.json`, SHA-256
`32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`,
capturado em `2026-09-07T18:13:33.573995+00:00`; HEAD
`b303f1f15b610e09872b567027e0d806387fde8c`, working tree não commitado.
O campo `state.diff_stat` desse baseline registra exatamente 23 arquivos,
1533 inserções e 191 remoções; `state.diff` e `source_inventory` preservam
o estado completo. Os únicos consumers Rust modificados nesse estado são
`compiler/eval/bindings/field_access.rs` e `compiler/eval/tests.rs`, de P1306.
Nenhum deles foi alterado aqui. Os pins abaixo referem-se a esses mesmos
bytes, não ao HEAD isolado. L0s podem avançar em paralelo sob autoria do root;
as observações deste relatório são sobre o baseline pinado.

Referência vanilla: upstream ratificado `a51e02804`; binário
`/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Evidência pública anterior, preservada sem edição:

- `p1307-r2-measurement.json`, SHA-256
  `847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`;
- `p1307-r4-content-observability.json`, SHA-256
  `26a27995e98dcde01a7dbb47838a40ad8394fc39ac3af8e2e7863cc6639e6634`.

Foram lidos integralmente os L0s vigentes de Content, Value, HeadingElem,
Element comum, Introspector, introspect, from_tags, fixpoint, introspect/heading,
pipeline, query L1, heading constructor, eval/rules, CLI e query-helpers L3.
As leituras de field_access e dos transportes de Value efetuadas nas auditorias
R3/R4 complementam o inventário; nenhuma recomendação dispensa o autor de
amendar e pin-ar cada owner antes da futura alteração correspondente.
Aplicam-se a skill `tekt-materializacao-segregada`, seus documentos de papéis
e gates, e ADR-0107/0108/0109/0127/0129.

## Medição de fonte anterior à recomendação

### O que falta, e onde se perde

O vanilla serializa Content como `func` seguido de `fields()`
(`lab/typst-original/crates/typst-library/src/foundations/content/mod.rs:709`).
`fields()` enumera os campos presentes do elemento e só depois a label
(`:583`). Não serializa o payload de tags nem a representação de debug.
Heading sintetiza level e supplement; `auto` usa o nome localizado da chain,
`none` vira Content vazio e a forma funcional executa resolução própria
(`lab/typst-original/crates/typst-library/src/model/heading.rs:249`).

No cristalino, `HeadingElem` guarda level, body, outlined, bookmarked e uma
máscara de presença (`entities/elements/heading.rs:24`). A máscara distingue
raw constructor/markup e **não** significa que todos os defaults estejam
assentes (`heading.md`, §P829). `get_field` devolve uma projeção pequena
(`heading.rs:156`); `content_field` e `content_set_fields` possuem outra
projeção, baseada em presença (`eval/bindings/field_access.rs:589,634`).
Logo não é correto declarar que get_field, fields, repr e payload já são uma
mesma fonte completa de campos realizados.

`HeadingElem::to_payload` (`:174`) contém depth/body_hash/counter_update/gate,
não os valores públicos completos. O `walk` recebe Content, chain lexical e
label causal (`compiler/introspect.rs:1259`), extrai apenas o gate de numbering
do Heading (`:1325`) e grava **content.clone() sem a chain nem label** em
`intr.elements` (`:1350`). Esse é o ponto preciso em que ainda há dados para
produzir um snapshot sem procurar o contexto posteriormente.

As wrappers Styled empurram a chain apenas na descida da subárvore (`:1887`).
Label propaga `Some(&label)` ao elemento alvo (`:1596`); o campo `auto` dessa
wrapper também representa a sintaxe escrita `<label>`, não autoriza excluir
essa label. Labels artificiais de TOC são produzidas mais tarde no braço
Heading (`:1500`) e não devem contaminar o campo público `label`.

`query` L1 clona `element_at` para `Value::LocatedContent` (`foundations/query.rs:59`);
não realiza campos. O valor precisa levar consigo o resultado da realização:
procurar de novo por location, igualdade de Content ou valor da label quando
um encoder for chamado não preserva escape entre contextos/snapshots.

### O primeiro snapshot também é observável

`introspect_with_introspector` é puro e devolve o resultado do walk sem os
postprocessadores runtime (`introspect.rs:360`). A versão runtime só depois
executa callbacks/counters/equations (`:391–420`). O precedente Equation
reconstrói exclusivamente as entradas de Equation com Styles realizados
(`introspect/from_tags.rs:422–472`); não é um mecanismo genérico de Heading.

No pipeline, `:644` constrói **o primeiro** snapshot pela versão pura; `:651`
entrega-o a `expand_context_blocks_and_reintrospect`. A expansão copia esse
snapshot para `EvalContext` (`pipeline.rs:248`) e só posteriormente chama a
versão runtime (`:666`). Um `context` que consulta e faz panic nesse primeiro
passo nunca chega a observar uma correção aplicada somente em `:666`.
Nas iterações seguintes (`:702–744`) há novo walk; cada novo snapshot precisa
ser construído coesamente. A assinatura/fase atual pode permanecer intacta
se a captura de Heading for **pura no walk**, antes da primeira expansão.

O fixpoint L1 separado também chama o mesmo walk (`fixpoint.rs:100`), calcula
hash de tags (`:111`) e postprocessa Equation (`:122–126`). Não se propõe
substituir o pipeline por esse fixpoint, nem antecipar callbacks existentes.
Snapshots puros determinísticos derivados do mesmo Content/chain não exigem
um novo passo de execução; qualquer futuro callback contextual de Heading
teria de reabrir a análise de convergência e ordem, não apenas ganhar um match.

### Limites de entrada anteriores, distintos do transporte

`native_heading` aceita numbering Str/None (`structural/heading.rs:134`),
outlined Bool/None (`:152`) e bookmarked Bool/None (`:164`). Numbering None e
omitido são colapsados; supplement/depth/offset/hanging-indent não são
armazenados por esse constructor. `eval/rules.rs:973–1004` trata apenas
numbering no set-heading e ignora os demais named nesse braço. Isso contradiz
a regra geral do L0 rules de não ignorar named desconhecido: é dívida anterior,
não prova de suporte funcional já existente.

`Content::heading_numbered_native` transporta gate e padrão na chain
(`content.rs:1539`); essa informação pode ser copiada causalmente durante walk.
A cadeia permite chaves adicionais sem mudar StyleChain (`style_chain.rs:337`),
mas **criar/validar essas entradas no constructor/set rule é mudança semântica
separada**, não migração mecânica do carrier. Um snapshot não recupera argumento
já descartado pelo produtor. Callback Heading não é requisito congelado R4 e
não está aceito pelo domínio atual do constructor: mantê-lo como dívida evita
introduzir uma fase runtime por acidente. Novas sondas R5 desse domínio devem
continuar visíveis como classificação de limite, não testes apagados.

O nome localizado deve ser resolvido em L1 a partir da chain alvo e da tabela
ratificada de Heading (`model/heading.rs:356`), não por `"Section"` constante
no encoder. O helper existente `introspect/labelled.rs:31` só distingue pt de
outros idiomas; ele não constitui uma tabela completa que legitime esse atalho.

## Comparação e recomendação refutável

| Opção | Benefício | Custo/limite concreto |
|---|---|---|
| Campo `HeadingElem.realized_fields` | Mantém forma de Value e API `Vec<Content>` da query CLI | Introduz Value no IR de render; deriva Hash deixa de servir, Debug afeta content_hash, PartialEq/morph_canon precisam decidir metadados; with_body deve sincronizar campo body duplicado. |
| Carrier `IntrospectedContent` em Value e no store existente | Separa snapshot observável da árvore de render; clones escapam causalmente; sem novo estado em HeadingElem | Migra todos os bindings LocatedContent e o transporte público L3→L2; não basta adaptar somente query L1. |
| Apenas mapa auxiliar por location | Conveniente para construir o snapshot | Não resolve escape: query/encoder dependeriam de contexto atual; aceitável só como construção temporária seguida de cópia ao carrier. |
| Styled custom como snapshot universal | Precedente Equation já existe | Muda a morfologia para Styled e mistura styles de render/fields; get_field e morph_canon não são universalmente transparentes. Exige mais invariantes, não menos. |

**Recomendação:** carrier em Value com store único. É a alternativa mínima
em invariantes de domínio, embora não em quantidade de matches Rust alterados.
Não se alega impossibilidade matemática de um campo em Heading; rejeita-se o
custo evitável de obrigar cada transformação de render a manter um snapshot
que não usa. Refutariam esta preferência: um requisito congelado que exija que
o snapshot sobreviva após lowering explícito para Content puro, ou uma rota
produtiva de introspecção que só consiga retornar Content e não possa ter sua
API reaberta. A rota CLI abaixo **pode**, mas requer gate público explícito.

### API de dados recomendada para o próximo L0, não código autorizado

Owner `entities/value`:

```rust
pub struct IntrospectedContent {
    content: Content,
    fields: Option<Arc<Vec<(EcoString, Value)>>>,
}
// acesso imutável: new(content, fields), content(), fields(), into_content()
// Value mantém a localização fora da morfologia:
Value::LocatedContent(IntrospectedContent, Location)
```

Os campos privados com acesso imutável tornam explícito que não há edição
independente de body ou Vec após congelamento. `None` significa ausência de
snapshot contratado; `Some` é sequência autoritativa completa de campos para
a família realizada, com ordem canônica, nomes únicos e sem `func` (projetado
pelo consumer antes dos campos). Label, se causalmente presente, é a última
entrada. `Some` não é um cache parcial com fallback por campo para o raw.
A construção validada deve recusar duplicatas/incoerência em testes de domínio;
não inferir validade comparando dois Values iguais. Clones compartilham dados
imutáveis. O `body` exposto no snapshot corresponde à mesma versão congelada
do Content; não se oferecem mutadores nem transformações in-place do carrier.
O lowering `into_content` perde explicitamente o snapshot quando a finalidade
é render; introspecção posterior produz outro snapshot, nunca reutiliza o antigo.

Owner `entities/introspector`: substituir somente o tipo dos valores do store
`elements` por `IntrospectedContent`; **não criar segundo store de campos**.
`Introspector::element_at(Location) -> Option<&Content>` conserva assinatura e
extrai `entry.content()`. Query L1 já tem `ctx.introspector` concreto e pode
clonar a entrada diretamente: não é necessário acrescentar método ao trait.
Uma função inerente é opcional, não necessidade de contrato. O wrapper
MeasurementIntrospector (`03_infra/src/measurements.rs:270`) continua delegando
o mesmo método, sem migração semântica ou novo owner por essa razão.

Owner introspect, preferencialmente helper privado da feature
`compiler/introspect/heading`: produzir fields a partir do Heading raw,
chain e label presentes em `walk:1345–1350`. Nenhum formatter, serde, engine,
consulta ambiental ou endereço global entra nesse helper. Os valores default
da família são regras semânticas ratificadas, não heurísticas baseadas no texto
do corpo; dados de estilo aceitos são lidos da chain causal. A lista exata de
defaults, presença, idiomas e entradas adicionais autorizadas pertence ao
contrato/oráculo independente R5. Não se promete corrigir todo o constructor.

### Transporte CLI: assinaturas que precisam de gate

O caminho atual perde a informação duas vezes: query-helpers usa element_at
(`:511`) e reconstrói Content::Label por busca reversa (`:514–523`); depois
L2 hardcoda Heading offset/numbering/supplement (`cli.rs:758`). Manter essas
rotas como estão tornaria o carrier insuficiente para a própria CLI.

Mudanças propostas, todos os parâmetros restantes intactos:

- `query_elements(&dyn World, &Source, &str)` em
  `03_infra/src/query_helpers.rs:459`: retorno atual
  `(Result<Vec<Content>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>)`;
  retorno proposto substitui `Vec<Content>` por `Vec<Value>`, com entradas
  `LocatedContent(snapshot.clone(), location)`. Warnings, ordem, cardinalidade,
  selectors e escolha pre-show/realized do helper não mudam nesta migração.
- `serialize_query` (`02_shell/src/cli.rs:664`) e
  `serialize_query_with_format` (`:674`): parâmetro `elements: &[Content]`
  passa a `elements: &[Value]`; os retornos continuam `Result<Vec<u8>, String>`.
  Projeção usa snapshot Some antes do caminho raw; `--field`, `--one`, formato,
  pretty e erros existentes mantêm a ordem atual. Não converter de volta para
  Content antes de selecionar campos.
- `04_wiring/src/main.rs:557–572`: encaminha o vetor recebido sem reconstrução
  de campos. Nenhuma assinatura pública nova em wiring é necessária; a
  obrigação do owner é preservar o tipo transportado.

Para famílias sem snapshot, preservar o comportamento legado explicitamente,
inclusive label existente onde contratado. Isso não autoriza acrescentar
defaults falsos a raw Heading. Para Heading Some, não fazer busca reversa no
registry nem usar a label do selector para substituir a label congelada.

## Inventário de owners e operações afetadas

Os paths abaixo são consumers produtivos; seus owners são os paths `@prompt`
correspondentes, não novos prompts agregados. A migração semântica exige leitura
e amendment do owner; ajustes de teste sintético não criam ownership adicional.

| Consumer/ponto | Natureza | Obrigação |
|---|---|---|
| `entities/value.rs:64,323,378` | API pública de carrier | Novo tipo/forma da variante; tipo público continua content, clone e PartialEq explicitados. |
| `entities/introspector.rs:372,727` | API de store/projeção | Uma entrada canônica; element_at conserva projeção Content. |
| `compiler/introspect.rs:1350` | Escrita semântica | Capturar Heading no momento causal, demais Content em entrada sem snapshot. |
| `compiler/introspect/heading.rs` | Helper puro proposto | Resolver defaults/estilos/label congelados, sem execução runtime. |
| `compiler/introspect/from_tags.rs:432,464` | Escrita existente de Equation | Desembrulhar/reembrulhar Content sem perder snapshot incompatível silenciosamente; Equation conserva sua realização atual. |
| `compiler/introspect/from_tags.rs:190` | Lowering de callback | Extrair Content de LocatedContent quando a operação já demanda Content, não serializar. |
| `compiler/stdlib/foundations/query.rs:59` | Leitura/retorno semântico | Clonar entrada completa e location; fallback sintético vigente preservado. |
| `compiler/eval/bindings/field_access.rs:130,404,820` | Observáveis | Snapshot manda em campos/has/at/fields; raw mantém presença; métodos não descartam carrier. |
| `compiler/eval/call_dispatch.rs:1510` | Transporte de métodos | Passar snapshot para método de Content, incluindo método alias existente. |
| `compiler/eval/repr.rs:35` | Observável | Repr realizado usa snapshot conforme medição própria, sem afirmar igualdade com serialização. |
| `compiler/stdlib/loading.rs` | Novo consumer já contratado | Mapear func+fields recursivamente para encoders; CBOR fallback repr registra delta de Heading realizado. |
| `compiler/eval/operators/equality.rs:120` | Semântica preservada | Igualdade linguística continua projeção morph_canon de Content; location/snapshot não entram incidentalmente pelo derive. |
| `compiler/stdlib/state.rs:187` | Lowering já existente | Extrair Content do carrier para render; sem alterar os Values armazenados no state. |
| `compiler/stdlib/structural/math.rs:602,626,641` | Lowering já existente | Extrair Content nos braços que já o exigem; nenhuma realização nova de Heading. |
| `03_infra/src/query_helpers.rs:459,511` | API/transporte público | Retorno Vec<Value>, snapshot preservado, não reconstrução por label. |
| `02_shell/src/cli.rs:664,674,1000` | API/projeção pública | Receber Values e não apagar os campos com defaults hardcoded. |
| `04_wiring/src/main.rs:557` | Transporte mecânico | Encaminhar valores; sem lógica de campos em L4. |

Ocorrências nominais que só classificam o tipo e mantêm dois slots da variante
não exigem alteração de comportamento: `value.rs:323,378`,
`stdlib/transforms.rs:113`, `eval/call_dispatch.rs:195`,
`bindings/method_dispatch.rs:78`, `operators/error_formatting.rs:37,66`.
Não são produtores de snapshot. `stdlib/mod.rs:828,829,852` e
`introspect/fixpoint.rs:758,764,765` são testes, não novos owners de domínio.

`infra/pipeline.md` deve explicitar que o snapshot puro já está completo para
o domínio contratado antes do primeiro context; a proposta não precisa alterar
a ordem das chamadas Rust. `compiler/introspect/fixpoint.md` registra a mesma
obrigação de captura pelo walk compartilhado, sem novo postprocessador. Os
L0s de HeadingElem/Content podem documentar a não-contaminação e presença raw,
mas esta opção não exige novo campo nesses consumers. Se o autor ampliar as
entradas Heading aceitas, os owners `stdlib/structural/heading` e `eval/rules`
ganham obrigações **semânticas**, separadas e medidas, antes de código.

## Pins de fonte auditada

Todos SHA-256 completos; prefixos de diretório são literais.

```text
01_core/src/entities/value.rs 0fdd94c7fff7953f4902f43dc2ede76812d24c5783d489f0470b36a98919542b
01_core/src/entities/elements/heading.rs d82758d34763ecaea1253531ca92c3d1a282dc0665c874274a0a9d6ba413051a
01_core/src/entities/content.rs 34838354261fa472c5efbc73c39ee6587b8b77be4d4606ca7587537818ed4a98
01_core/src/entities/introspector.rs 3df7cb46cf5c01465756a3018f4b14c69188f0513cb941ec253159c50859fdd8
01_core/src/compiler/introspect.rs 6e50fed8b66e17855c25fc81f945e8e88c888a0e8afa2c814331ad64ab3fb6ad
01_core/src/compiler/introspect/from_tags.rs dd731eabc274395ca22ea8a916f1d0a4eb4fad68d65a26f1c35a7cf4b22cbdd7
01_core/src/compiler/introspect/fixpoint.rs 52383410ca023b303e28b4df718e82f882d7d5b44afce226bb5035367c1c0fe6
01_core/src/compiler/stdlib/foundations/query.rs f62870a4f9a13cf31f36122300a93d5258497f323c2e1ddbf41d677936911310
01_core/src/compiler/stdlib/structural/heading.rs 30312d058d9d8170a14b4b71bef464ffdb9f51d794a729b8d75d1bbdcc807bd7
01_core/src/compiler/eval/rules.rs d1b2a493682ba0989d397360b5c3781b15a1e130d460f6e6854980bc001b267c
03_infra/src/pipeline.rs 72f9c080b55ca295e5b51ae45acf527c76921cd06211a64c2c43fdd010af6088
03_infra/src/query_helpers.rs 6b95408d0ce85f82c97cc54d58d35aaad8dfe1e2ea19d26546ff80c769252df3
02_shell/src/cli.rs d3e9fecdfce6c1aad0f9badce9000f0b4de41888e299cad63522d25223b42989
```

## Gate e refutação

O desenho é realizável sem alteração da árvore de render nem antecipação de
callbacks. **Não é uma correção apenas mecânica:** muda a forma pública de
Value, o store público e as assinaturas L3/L2 listadas; exige L0 novo pinado e
confirmação ADR-0127 antes de RED/implementação. A autorização documental R5
não equivale a esse gate. Não há veredito de paridade neste relatório.

A aceitação futura precisa confrontar raw versus query, label causal versus
TOC, idioma lexical versus idioma do context consumidor, escape em array/dict/
closure, igualdade morfológica preservada, query CLI e encoders, com o oráculo
independente. É refutação do transporte qualquer campo Some perdido numa
dessas rotas ou reaparecendo depois de transformação com body antigo. É
refutação da limitação de escopo um callback Heading já aceito em fonte ou
dependência num caso obrigatório anterior; não foi encontrado esse caso.
Render/layout de Content original, ordem/fase do pipeline, Equation e raw
Content permanecem controles distintos. A mudança intencional de repr de
Heading realizado propaga ao fallback CBOR: não alegar preservação byte-a-byte
global ao mesmo tempo que se contrata esse delta.
