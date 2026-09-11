# P1340 — contrato observável R4

Componente congelado da proposta R4, sujeito a revisão independente. Autor
independente
`/root/p1340_contract_correction`; executado sem atestação de isolamento.
Autoridade: `p1340-authority-successor-r4.json`. Nenhum candidato, relatório
de implementação, materialization ou context é entrada deste autor.

## Medição normativa anterior à classificação

`compiler/eval.md:490-498` exige preservar os recursos efetivos da tentativa
original; `infra/pipeline/context_stabilization.md:101-110` proíbe corrigir
propagação legada de target/features fora do recorte. O predicado v2-r3
exige o perfil solicitado para todo BodyStarted, incluindo descoberta.
`p1340-verifier-gate-pause-r1.json` registra a incompatibilidade e congela a
falta de observabilidade de Func e Span que impede o gate seguinte.

`compiler/eval.md:475-480,527-550` distingue replay de fold de callback
consumidora e proíbe invocação para comparação. O witness histórico
SW-LC-invocation-origin-completeness exige todos os callbacks reais
alcançáveis por comparação, validação, diagnóstico e fold. Sua prova não
se satisfaz por lista vazia. O caso selected-stable-error exige ausência
de callback, enquanto a entrada do próprio ContextBlock é uma operação
distinta dessa obrigação. `compiler/eval/call_dispatch.md:194` descreve
dispatch fechado por FuncRepr e merge recursivo de With.

`entities/counter_registry.md:189-197` especifica Location, origem e ação
reais, sem exigir Span lexical no carrier produtivo. O DTO histórico exige
Span por CounterEvent. A ausência desse campo no carrier não legitima
inventar root span, span de leitura ou offset esperado. O inventário
allowlisted deve identificar onde a origem ainda existe antes dessa perda.

## Recursos reais

O envelope profile continua a identificar configuração solicitada/compilada.
Resources.features representa somente a lista efetivamente usada por aquele
EvalContext. Toda retenção, requisição e replay continua a preservar a tupla
causal completa do R3; seleção não muda retroativamente recursos ordinários.

A obrigação corrigida distingue a origem do contexto, não o valor final do
bit selected: descoberta ordinária e contribuições legadas preservam a
construção ordinária; tentativas selecionadas usam sua construção autorizada;
novos descendentes seguem o transporte real dos recursos da rota que os cria.
Não se permite trocar a origem reportada para satisfazer o perfil. A ligação
exata das rotas é auditada contra os pontos do inventário antes do selo.

O inventário `p1340-interface-inventory-r4.md`, SHA-256
`323e48e162053065c18b335a4864111e0f1a8f5f58b3df3bd6461ff49e0a1a68`,
fixa `EvalContext::new` e Features::default vazios, atribuições explícitas dos
entrypoints e transporte do observer ao contexto do fold. Não inspecionou L3
e não certifica a rota de cada nested. O recibo JSON, SHA-256
`3985c96185f302cf6fd6563bcb094637ec8087127c6b9cb0eff77d8f5a2eb816`,
conserva UTC, HEAD, diff stat e pins. O autor do inventário declara conhecimento
histórico como implementador; este autor recebe somente a interface factual.

`BodyStarted.feature_origin` acrescenta `mode` e `source_edge_ref`:
ordinary_default exige features []; selected_profile exige perfil compilado
e tentativa > 0; inherited exige from_execution_id anterior, geração pai
correta e exatamente suas features reais. Descoberta de attempt 0 conserva
features vazias. A escolha de mode vem da rota real auditada, nunca do bit
selected ou de uma condição para fazer o predicado passar. Cada RequestRecorded
conserva as features do seu BodyStarted; replay/retenção seguem a igualdade
integral histórica. A auditoria deve rejeitar selected_profile reportado como
ordinary_default para ocultar perda de features, ou uma origem inherited
inventada. Faltando a testemunha de rota, a exigência continua aberta.

## Função invocada e causa

Cada FunctionInvoked exigido no recorte é ligado à Func real recebida no
dispatch, à origem causal real e à fase body/validation/diagnostics. Categoria
de callback sem identidade não basta. A identidade é válida somente com
lifetime/generation auditados; ponteiro reutilizado ou nome não prova Func.
With e sua aplicação interna devem conservar os eventos de despacho reais,
sem contar como invocação a mera construção/retenção de uma closure.

O vínculo passivo distingue entrada do próprio ContextBlock de callback de
leitura. Essa exclusão é sustentada por callsite/causa auditados, nunca por
nome, FuncRepr, resultado, ou pelo desejo de satisfazer o teste sem callback.
Um callback que tenta ler outro counter mantém sua identidade, origem de fold
e erro contextual. Toda chamada efetuada para categoria/comparação permanece
visível e é refutação. Nested calls dentro de callback mantêm a cadeia causal.
Não são permitidos hooks que escolham execução ou modifiquem callback/contexto.

SW-LC-invocation-origin-completeness continua obrigatório: enumerar as arestas
de invocação alcançáveis, ligar cada uma à porta observadora e conferir a
bijeção com os eventos reais. Lista vazia só é aceitável com essa auditoria.
Observações do dispatcher ordinário podem ser conservadas em ledger próprio;
o transcript semântico não pode apagá-las sem uma exclusão causal auditada.

R4 torna esse ledger obrigatório: `r4_provenance.function_dispatches` preserva
cada entrada de despacho no escopo observado da compilação, inclusive entrada
de ContextBlock e chamadas ordinárias, com dispatch_id/dispatch_sequence,
func_id, scope, phase, dispatch_role, parent_dispatch_id, dispatch_edge_ref e
causal_edge_ref. Os scopes ordinary/context_body_entry ficam somente no ledger;
counter_fold/display_callback/projection/comparator_category_test/other_callback
geram uma bijeção exata com FunctionInvoked, que acrescenta dispatch_id e phase.
`other_callback` usa o spelling histórico other_actual_origin no evento.
With inclui wrapper e target reais, com parent_dispatch_id e escopo/fase
preservados. Chamadas internas a um callback herdam sua causa até a saída;
não se tornam ordinary por atravessar um dispatch genérico. A auditoria de
escopo inclui todas as arestas alcançáveis das fixtures congeladas, inclusive
collections/math/rules se alcançadas. Não afirma inventário universal de toda
invocação Rust de Func. Scope e fase comparison refutam execução de Func mesmo
que ela não apareça no transcript semântico.

## Proveniência do CounterEvent

Toda ocorrência guarda origem capturada no ponto real que a conhece e uma
cadeia de identidade até o CounterEvent da árvore atual. A cadeia deve ligar
a construção do Content/elemento, a ocorrência/Location e a inserção real da
ação no snapshot. Clone e retenção conservam a origem; nova construção ganha
identidade de ocorrência própria. Dois updates de mesma chave/ação/valor
não são identificados por igualdade estrutural.

Para os eventos lexicais das fixtures atuais, Span integral resolve a Source
real da construção/ocorrência, com FileId, hash, bytes e raw_span. Root span,
span de demanda do contador, span de consumidor ou coordenada da fixture não
substituem essa origem. Set, Step automático e Func têm ação e origem próprias.
Uma origem realmente sintética/detached é explicitada como tal e não ganha
bytes inventados; não pode satisfazer uma obrigação lexical destas fixtures.
Origem perdida/ambígua no recorte obrigatório é Unknown que bloqueia selo.

A proveniência é observação cfg-only, sem campo/assinatura/entidade pública
nova ou mudança dos defaults. Antes da escrita do hook, a autoridade precisa
conter seus owners exatos e a revisão deve confirmar que ele apenas observa.

O envelope r4_provenance acrescenta counter_origins, counter_occurrences e
content_derivations. Cada CounterEvent ganha occurrence_id. CounterOrigin
liga origin_id, kind manual/automatic, source_edge_ref, span_status lexical,
span_role call/arguments/markup/element_source, span integral e
content_resource_id. A fonte de arguments é o Args/AST real da construção,
nunca o Args da consulta que consome o evento. CounterOccurrence liga
occurrence_id, snapshot_id, event_index, location, producer_generation_id,
origin_id, content_resource_id, source_edge_ref e derivation_ids ordenados.
Cada ContentDerivation liga derivation_id, from_content_resource_id,
to_content_resource_id e source_edge_ref. Exige-se caminho contínuo do Content
de origem ao Content real caminhado e bijeção de ocorrências com eventos atuais.
Um Arc conservado permite caminho vazio; reallocação sem aresta não permite.

`r4_provenance.audited_edges` mapeia IDs para kind/path/sha256/line e scope
quando aplicável. Kinds são features:<mode>, func_dispatch, invocation_scope,
counter_origin:manual/automatic, counter_occurrence e content_derivation.
Esses pontos são parte da auditoria independente do binding, não autodeclarações
do adaptador. A sintaxe/igualdade do DTO sozinha não prova que os pontos foram
observados ou que o ledger é completo. A omissão simultânea de ledger/evento,
ou a troca conjunta de IDs, exige rejeição pelo witness de origem/binding.

## Dict e herança

O componente `p1340-contract-dict-r4.json` demonstra a colisão do Dict sem tag
com carriers Func/Length. Corrigir somente essa representação por wrapper
Dict recursivo com pares ordenados. Não alterar a igualdade ou a serialização
do produto. Todo resultado, erro, argumento, chain e request do R3 permanece.

Todas as fontes, casos públicos/lifecycle, perfis, ordens, política terminal
R2, T01–T06, folds integrais, prefixo negativo testemunhado, completude positiva,
capturas, retenção, invalidação, orçamento e sinks continuam obrigatórios.
Os originais são imutáveis. Esta revisão não fecha F, P1340 ou P1339.

A herança mantém obrigações e arquivos R2 sem atestar que seus inputs sejam
válidos. Uma causa terminal independente recém-identificada não é corrigida
nem usada para redesenhar oráculos nesta autorização de três causas.

## Calibração delimitada

Uma revisão contratual adicional, no máximo duas tentativas focais por causa,
zero corpus completo antes de sucesso focal e teto de 60 minutos de revisão.
Primeiro discriminar recursos ordinários versus selecionados; identidade e
origem de callback; Span lexical de evento versus substituto incorreto; Dict
versus carriers colidentes. Repetição da mesma causa/vetor ou necessidade de
outra revisão interrompe a cadeia. O verificador registra positivos preservados,
negativos rejeitados, opacos Unknown, regressões, causa dominante e custo.
Exemplos do predicado não são observações produtivas nem mutation kills.
