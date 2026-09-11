# P1340 R4 — inventário factual de interfaces L1

## Natureza e fronteira

Inventário somente leitura, por `/root/p1312_review`, para o autor independente R4. Não é contrato, proposta de algoritmo, grant, implementação ou veredito. Autoridade: `p1340-authority-successor-r4.json`; pins e estado exato no recibo homônimo JSON. HEAD observado: `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado. Primeiro snapshot UTC desta investigação: `2026-09-10T14:38:34Z`; snapshot pinado: `14:39:49Z`. A árvore é compartilhada e o diff/stat mudou fora deste inventário entre esses dois instantes.

A skill `tekt-materializacao-segregada` e suas duas referências foram lidas integralmente. Segregação é procedimental; não há atestação de isolamento técnico. O agente tem conhecimento histórico como implementador, mas este artefato usa somente fatos das interfaces L1 indicadas abaixo. Não foi aberto o arquivo `context_stabilization.rs`, nem usado seu conteúdo para formular este inventário. Não houve leitura de corpus/expectativas para escolher eventos ou reconstruir origem. Não houve escrita em fonte, L0 ou teste, nem execução de testes. L0s extensos foram consultados nas seções pertinentes; não se afirma revisão integral de todos os owners ou conformidade arquitetural global.

## 1. Invocação de Func: identidade não é origem da chamada

`entities/func.rs:23` guarda `Arc<FuncRepr>` e Span privado; `:53-82` guarda body SyntaxNode, captura `Arc<Scope>` e Capturer. O clone mantém esses recursos. `:253-267` cria With com a função original e Args, mantendo o span diagnóstico. Esse span é origem da definição, não span universal da chamada. `compiler/eval/closures.rs:192-196` cria snapshot eager, e o final de `eval_closure_expr` atribui o span dos parâmetros; `eval/mod.rs:7564-7583` cria a closure de Contextual com body span, Capturer::Context e novo id de bloco.

`compiler/eval/call_dispatch.rs:516-562` recebe Func, Args, Scopes, EvalContext e Engine reais. O match aplica Closure, Element, Plugin, With, Native ou NativeWithEngine. Não recebe um discriminante de origem da invocação. A mesma Func pode ser invocada de posições distintas. Nome, body span, igualdade de resultado ou Capturer não identificam o callsite dinâmico.

With é importante: `:544-546` entra recursivamente em apply_func para a função interna. `:1155-1161` concatena ocorrências pre/new e preserva o agregado new.span. Observar cada entrada sem distinguir wrapper/target descreve mais de uma entrada de dispatcher para uma aplicação externa. Construir With (`entities/func.rs:253-255`, `call_dispatch.rs:1496-1501`) não executa seu target.

### Callsites causais examinados

| Origem real | Fonte | Dados disponíveis imediatamente antes da aplicação |
|---|---|---|
| Chamada AST ordinária | `compiler/eval/call_dispatch.rs:1751-1764` | Func resolvida, call.span, Args avaliados, ctx/Engine/scopes |
| Delegação interna de With | mesmo owner `:544-546` | wrapper, target, preargs/newargs; nenhum AST novo |
| Callback de layout | mesmo owner `:1639-1644` | Func extraída do argumento, Args sintéticos com call.span, ctx/Engine/scopes |
| Callback de state.display no corpo | `compiler/stdlib/state.rs:104-123` | callback real, valor lido, span recebido, mesmo ctx/Engine/scopes; Args::positional é detached |
| Callback de counter.display | `compiler/stdlib/counter.rs:562-569` | callback real, componentes do counter, span do helper e mesmo ctx/Engine/scopes; Args::positional detached |
| CounterUpdate::Func do fold demandado | `compiler/introspect/from_tags.rs:37-94` | evento real com Location/key/action, Func dentro da ação, span da demanda, observer proprietário, ctx novo e Engine; Args::from_parts usa span da demanda |
| StateUpdate::Func pós-walk | mesmo owner `:115-140` | Tag/Location, Func real e valor anterior; ctx recebido; Args::positional detached |
| CounterUpdate::Func legado pós-walk | mesmo owner `:149-174` | Tag/Location/key/Func reais; chaves contendo Element são excluídas dessa execução; Args::positional detached |
| StateDisplay diferido | mesmo owner `:233-255` | Tag/Location/key/callback real; Args sintéticos detached |
| CounterDisplay diferido | mesmo owner `:340-361` | Tag/Location/key/callback real; Args sintéticos detached |
| Numbering de Equation | mesmo owner `:401-418` | Location real e callback obtida do store; Args sintéticos detached |
| Supplement de Equation | mesmo owner `:460-479` | Location real, callback e conteúdo base real; Args sintéticos detached |
| Numbering tipado / numbering global | `compiler/stdlib/numbering.rs:22-47,101-114` | Numbering::Func/Value::Func real, números, span recebido, ctx/Engine/scopes |

O inventário textual adicional em `01_core/src/compiler` localizou chamadas em collections (`:1026,1044,1098,1135,1151,1187,1218,1258,1377,1548,1572,2302,2319,2414,2445`), math (`:368,588,1326`) e rules (`:539,663,749,907`). São fronteiras adicionais reais; esta enumeração textual não atesta cobertura semântica integral desses owners. Um contrato de toda invocação Rust de Func exigiria incluí-las, além dos callers externos à L1 não inspecionados aqui.

Entrada do próprio corpo ContextBlock é distinta de callback de introspecção. A criação de Contextual apenas produz Content; `ContextBlockElem` conserva id/closure e é terminal. `apply_closure` executa body via eval_expr (`closures.rs:150-171`) com o mesmo EvalContext, sem distinguir por si só quem solicitou a invocação. Portanto ausência de callback de fold/display não implica ausência de invocação do corpo. A classificação deve vir da chamada real, não do nome da Func ou do fato de estar dentro de um contexto.

O hook cfg existente é estreito: `eval/mod.rs:5271-5285` declara StateDisplay/PageNumbering/CounterFold e Body/Validation/Diagnostics; `:5869-5873` armazena somente `(phase, kind)`, não Func nem span. `from_tags.rs:75-81` observa no contexto proprietário, porém executa no contexto novo. Esses dois contextos não são intercambiáveis como identidade.

## 2. Recursos de EvalContext e contextos criados posteriormente

Fatos de interface, sem inferência sobre um orquestrador não lido:

- `EvalContext::new` (`eval/mod.rs:6361-6378`) inicia target Paged, Features::default, introspector vazio, Location None e in_context false. `entities/compiler_features.rs:14-23` define default/empty como ambos bits false.
- Entrypoints explícitos substituem esses defaults: `eval_expression_with_features`, `eval/mod.rs:6547-6548`; eval de módulo `:6745-6747` atribui target/features recebidos.
- `Expr::Contextual` (`:7564-7583`) guarda closure/captura/id; não guarda Features nem Engine no ContextBlockElem. Criar um bloco não prova recursos de uma futura execução.
- `apply_closure` usa o ctx recebido e Engine local com styles clonados (`closures.rs:150-171`). Não há novo EvalContext nessa aplicação.
- Fold demandado cria contexto sem introspecção, mas copia target/features do observer (`from_tags.rs:70-74`); isso não é um contexto de corpo ordinário.
- L0 `compiler/eval.md:379-401,435-440,489-498` separa seleção por demanda Element de registro de recursos e exige mesmos recursos causais para reuso. O L0 não autoriza deduzir Features de um bit de seleção. Defaults, perfil explicitamente recebido e recursos efetivamente usados são fatos diferentes.

Sem inspecionar L3, este inventário não certifica recursos da descoberta, da execução ordinária, nem de um nested criado durante uma tentativa. A interface L1 permite observar o ctx/Engine efetivo e o momento de criação; não fornece um perfil futuro implícito no Func.

## 3. Origem dos updates manuais: rotas distintas

| Rota | Fonte | Origem ainda disponível / limite |
|---|---|---|
| Método ligado `c.update/step` | `call_dispatch.rs:1302-1310` → `bindings/value_methods.rs:345-384` | caller tem call.span; helper recebe somente Args AST e usa args.span, além de ctx/Engine/scopes. update/step chamam helpers sem Span/ctx. A rota não passa por apply_func para o método ligado |
| Método estático descoberto | `bindings/field_access.rs:726` → `stdlib/counter.rs:168-179` | descoberta constrói Func::native_with_engine; não produz update ou executa callback |
| Método estático executado | `stdlib/counter.rs:212-252` | Args original, ctx/Engine/scopes recebidos; remove receiver no clone coerente. Antes do retorno existe Content real e origem dos Args; helper final não recebe span |
| Alias/With do método estático | `call_dispatch.rs:1760-1764,544-546,1155-1161` | identidade real da nativa/With, ocorrências pre/new, agregado da chamada final. Não inferir chamada inteira de args.span |
| Global legado `counter_step` | `stdlib/counter.rs:758-771` | ctx e Args recebidos; construção direta de Content::counter_update; não usa counter_step helper moderno |
| Chamador Rust sintético | `stdlib/counter.rs:121,152`; `entities/content.rs:2278-2282` | API recebe key/value ou key/action, sem AST/span/ctx. Pode existir ausência legítima de origem lexical |

`counter_update` converte Int/Array/Func em Set/Func e cria Content (`counter.rs:121-148`); `counter_step` cria Step (`:152-153`). Nenhum dos dois aplica a callback Func. O tipo público `CounterUpdateElem` tem somente key/action (`entities/elements/counter_update.rs:23-26`), sem span e sem geração.

`eval_args` (`call_dispatch.rs:399-452`) guarda spans de argumento/valor, expande Array/Dict no span do spread e transporta ocorrências originais de Args. Agregado começa como span da lista. O transporte de chamada inteira é seletivo (`:455-513`), não política geral para counter. Com With, origem de preargumento e origem da invocação final coexistem e não se substituem.

Ausência legítima em uma chamada Rust sintética é diferente de origem lexical disponível no caller que deixou de ser transportada. Não classificar o segundo caso como detached legítimo para preencher observabilidade. Também não usar o span diagnóstico da callback como span da construção do update: são dados distintos.

## 4. Def-use do conteúdo ao CounterActionEvent

1. `Content::counter_update` cria `Arc<CounterUpdateElem>` (`entities/content.rs:2278-2282`). Reuso por clone conserva essa alocação.
2. O hub `Content::map_content` processa bottom-up (`:3580-3596`); CounterUpdate está no braço terminal `self.clone()` (`:3701-3742`). Depois chama transform; Some substitui, None mantém (`:3801-3806`). Portanto essa rota conserva o Arc do update se não houver substituição. Não é uma garantia sobre qualquer transform.
3. A chamada direta ao trait `CounterUpdateElem::map_content` cria `Arc::new(self.clone())` (`elements/counter_update.rs:33-38`); map_text faz o mesmo (`:41-45`). Ela não é a rota do braço terminal do hub. Uma ligação somente pelo Arc inicial não cobre essas recriações.
4. `extract_payload` (`compiler/introspect/extract_payload.rs:82`) delega a `to_payload`; este clona key/action e não o Arc do elemento (`elements/counter_update.rs:52-57`). `ElementInfo` só tem payload/label (`entities/element_info.rs:18-21`).
5. O walk tem simultaneamente `&Content`, `&mut TagIntrospector`, Locator e chain (`compiler/introspect.rs:1629-1649`). Obtém payload e Location real, cria ElementInfo e chama populate (`:1715-1716`), guarda `content.clone()` em `intr.elements[loc]` (`:1743-1745`) e emite Tag::Start (`:1747`). Esse par canônico conteúdo/Location é evidência real do nó caminhado, não dedução por key/action.
6. Populate registra ação manual com Location/key/action (`:1157-1158`). `CounterActionEvent` (`entities/counter_registry.rs:21-24`) guarda só esses campos, e actions é Vec privado (`:43-57`). Não há span, produtor, geração ou identidade do Arc no evento. O store canônico de conteúdo por Location é distinto desse log.

Consequências: conteúdo clonado pode aparecer em várias Locations; a identidade do recurso não é identidade de ocorrência. Chaves e ações iguais podem ter produtores diferentes. O mesmo callback pode estar em vários updates. Nenhuma dessas igualdades permite inventar uma aresta de proveniência. O walk é puro, não recebe EvalContext/Engine; seu estado explícito não inclui uma geração de avaliação. Não foi encontrado identificador `ContentMarker` na busca textual restrita a `01_core/src`; não se importa um conceito externo como se fosse campo L1.

## 5. Ações automáticas, especialmente Heading

`populate_intr_from_tag_start`, `introspect.rs:1144-1155`, distingue ações automáticas pelos payloads e gates reais: NativeElement/Footnote/Citation, Heading com numbering ativo (nível efetivo), Figure/Table contados e Equation block+numbering ativo. Registra key None; as escritas auxiliares figure:kind não representam uma nova ação automática. Key None não é falta de chave a ser preenchida por heurística.

Heading tem level/body/outlined/bookmarked/set_fields, **sem Span** (`entities/elements/heading.rs:23-40`, L0 correspondente §Struct). A Location e o conteúdo real existem no walk; o gate vem da chain (`introspect.rs:1698-1700`), não de uma chamada counter.step sintética.

A origem lexical antecede o elemento: markup `eval_heading` recebe AST/ctx/Engine e cria Content antes de interceptar show (`compiler/eval/markup.rs:90-113`); a nativa recebe Args/ctx e cria heading nativo/numbered (`stdlib/structural/heading.rs:34-39,181-193`). Essas duas rotas não transportam um span para HeadingElem. Seu map_content recria HeadingElem (L0 `entities/elements/heading.md:57`), ao contrário do terminal CounterUpdate no hub. O inventário não prova uma ligação lexical universal de Heading após transformações. Ausência de span no store final não prova que o documento não tinha AST de origem.

## 6. Owners que contêm os fatos, não autorização de escrita

Núcleo de interfaces para uma eventual concessão cfg-only estreita:

- `compiler/eval/mod.rs`: EvalContext local, criação Contextual, identidades/recursos e registro cfg existente.
- `compiler/eval/call_dispatch.rs`: aplicação real, With, chamada AST completa e rota ligada antes de perder call.span.
- `compiler/eval/bindings/value_methods.rs`: retorno real dos métodos ligados de counter, com ctx e Args AST.
- `compiler/stdlib/counter.rs`: wrappers estáticos/legado, construção real e callback display.
- `compiler/stdlib/state.rs`: callback state.display real, no contexto proprietário.
- `compiler/introspect/from_tags.rs`: fold demandado e callbacks pós-walk, incluindo separação entre observer e contexto de execução.

Pontos adicionais condicionais: `compiler/introspect.rs` é dono do encontro conteúdo/Location/ação; `stdlib/numbering.rs` contém aplicações reais de Numbering; `eval/markup.rs` e `stdlib/structural/heading.rs` contêm origens de Heading. `entities/content.rs`, `entities/elements/counter_update.rs` e `entities/elements/heading.rs` definem limites de conservação do recurso durante transformações. `entities/func.rs`, Args, Span, ElementInfo e CounterRegistry fornecem dados já existentes, não um pedido de novos campos normais.

Esse conjunto localiza a informação; **não demonstra que seis owners bastem para todos os eventos de um contrato ainda não redigido**. Cobertura universal de callbacks acrescentaria collections/math/rules e callers de outras camadas; conservação universal através de transformações exige demonstrar cada aresta. Não há nesta investigação fundamento para alterar API normal, entidade pública, igualdade, seleção, recursos por default ou semântica. Onde a proveniência não está disponível, o limite deve permanecer explícito até decisão independente, sem mock, join por valor, índice de fixture ou reconstrução de evento.

## Resultado da investigação

Há pontos L1 concretos com Func real e origem causal da aplicação, e há uma ligação real entre o conteúdo caminhado e sua Location. O log atual de callbacks não conserva a Func; o log de ações não conserva span/geração. A preservação de Arc de CounterUpdate no hub é útil e delimitada; a ausência de span lexical no Heading final e a recriação de elementos impedem alegação de proveniência universal baseada apenas em ponteiro. Nenhum gate de execução foi exercitado ou encerrado por este inventário.
