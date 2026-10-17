from pathlib import Path
import gzip,json,hashlib,re,subprocess,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));src='01_core/src/compiler/eval/call_dispatch.rs';owner=re.search(r'^//! @prompt (.+)$',Path(src).read_text(),re.M)[1];assert hashlib.sha256(Path(src).read_bytes()).hexdigest()==b['hashes'][src];scope['sources'].append(src);scope['owners'][src]=owner;b['files'][src]=Path(src).read_text();b['files'][owner]=Path(owner).read_text()
with gzip.open(out/'before.json.gz','wt') as f:json.dump(b,f)
pin='9f2b6e8715237cb086899a42873660fe744622e8';up={}
for p in ['crates/typst-eval/src/call.rs','crates/typst-eval/src/methods.rs']:
 s=subprocess.check_output(['git','show',pin+':'+p],text=True);up[p]=dict(text=s,sha256=hashlib.sha256(s.encode()).hexdigest())
with gzip.open(out/'pinned-source.json.gz','wt') as f:json.dump(dict(pin=pin,files=up),f)
nucleus='00_nucleo/prompts/_nuclei/eval/call-once.toml';s='''tekt = 1
kind = "nucleus"
id = "eval-call-once"
title = "Resolução e efeitos uma vez por chamada"

[[claims]]
id = "receiver-ownership"
level = "must"
statement = "Uma chamada conserva seu receiver efetivamente resolvido ao atravessar interceptores e fallback. Cache é local à chamada, sem memoização global/por texto; clones de coleções não substituem transferência de ownership e empréstimo."

[[claims]]
id = "lazy-arguments"
level = "must"
statement = "Resolver método/callee e seus gates antes de avaliar argumentos de chamada comum; método desconhecido não executa argumentos. Coleções avaliam argumentos só depois de reconhecer a operação na tabela estática única, uma vez e com spans/occurrences intactos."

[[claims]]
id = "mutating-order"
level = "must"
statement = "Métodos mutantes preservam a ordem especial argumentos, local mutável, operação. Se o local permite fallback para campo chamável, transportar o receiver resolvido e os Args já avaliados; nunca reavaliar o alvo/argumentos nem mutar uma cópia temporária."

[[claims]]
id = "callee-fallback"
level = "must"
statement = "Seletores, conteúdo e interceptores contextuais conservam receivers/callee quando não interceptam; despacho final recebe o valor e Args resolvidos, mantendo políticas de field access, span, trace, contexto, aliases e prioridades."

[[claims]]
id = "no-compensating-effects"
level = "must-not"
statement = "Identificar uma função por seu nome para isolar scopes, truncar arrays, apagar efeitos ou compensar contagens de avaliação. Argumentos matemáticos seguem uma avaliação causal normal; a preparação/inserção matemática mantém sua fase e carriers próprios."

[[claims]]
id = "static-surface"
level = "must"
statement = "Preservar APIs Rust externas, catálogo estático de métodos e suas extensões Tekt, callbacks, esquema de argumentos, defaults e pureza L1. Nenhuma vtable, registry dinâmico, nova feature ou mudança de fase é autorizada."
''';Path(nucleus).write_text(s);effective=hashlib.sha256(s.encode()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest();scope['nuclei']=[nucleus];(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
responsibilities={
'call.rs':'''Manter caches locais Option<Value> para receiver e callee e Option<Args> para argumentos do fallback mutante. Helpers internos resolve_receiver(access, cache, scopes, ctx, engine) e resolve_callee(call, cache_receiver, cache_callee, scopes, ctx, engine) devolvem referências aos valores guardados. Reusar eval_field_access_with_target para lookup completo, incluindo features/spans/estilos/deprecações. Preservar prioridade where/with, seletores, mutação, instância, coleção, contexto, conteúdo, erro de callee e invoke. Transportar cache por todos os fallbacks, sem duplicar lógica de lookup; erro de método ocorre antes de argumentos comuns. Where/with continua no ramo inicial único. No final nomear field_callee_error antes de resolver campo; invoke recebe callee e Args já obtidos. Testes co-localizados usam World em memória e APIs vigentes antes do código.''',
'instance_methods.rs':'''Resolver receiver pelo helper com cache compartilhado e emprestá-lo ao match atual. Conservar handlers, spans, args, retornos e prioridades; não apagar o valor no fallthrough e não clonar coleções para observar tipo.''',
'collection_methods.rs':'''Substituir rota eager por try_collection_method: retirar uma vez o receiver do cache, verificar gate, construir spans e passar avaliação adiada de args via closure estática FnOnce ao dispatcher stdlib. Handled conserva trace Args map/filter; Unhandled devolve o Value por ownership ao cache. Remover o ramo With tardio redundante: call.rs já trata exatamente essa forma primeiro. Não duplicar catálogo nem decidir por nome público da função.''',
'selector_intercepts.rs':'''Receber cache de receiver; reconhecer os mesmos nomes e obter o valor uma vez pelo helper. Passar referência do valor aos adapters de selector. Se não reconhecer selector, conservar o valor no cache, sem executar argumentos.''',
'contextual_calls.rs':'''Receber os caches de receiver e callee; nas mesmas verificações sintáticas de measure/layout, usar resolve_callee uma vez, conservando mesmo o não-nativo para fallback. Preservar comparação tipada de fn pointer, gate de contexto, dimensões e handlers.''',
'content_methods.rs':'''Receber cache e usar referência ao receiver resolvido em Location, Content/LocatedContent e OutlineEntry, sem reavaliar AST em cada grupo. Preservar helpers de transparência, snapshot/Location, args, traces e schemas; ajustar somente empréstimos/derefs necessários.''',
'invoke.rs':'''Acrescentar apenas parâmetro interno resolved_args: Option<Args>. Usar os Args transportados pelo fallback mutante ou avaliar args normalmente uma vez. Remover is_mat_call por nome, isolate_mat_args, cópias isoladas de Scopes e reparos por array_lengths/prefix/truncate: efeitos legítimos do usuário não são deduplicáveis por conteúdo. Preservar todos os metadados/spans de chamadas, match fechado, callbacks, With, traces, Content/ShowRecipe e contratos matemáticos de inserção já existentes.''',
'method_dispatch.rs':'''Adicionar enum restrito a eval MutatingCall { Handled(Value), Fallback { target: Value, args: Args } }. try_eval_mutating_method passa a SourceResult<MutatingCall> mantendo os sete parâmetros. Preservar args antes de access e writeback real; sucesso mutante mapeia Handled; fallback de Module/Func/Type/Symbol/Content clona o valor do local uma única vez e transporta os Args. Não reexecutar access ou args. Isto supersede a aceitação histórica de dupla avaliação no comentário P717; a nova medição prova efeito observável e exige correção.''',
'selector.rs':'''Os três adapters eval_selector_or_and/within/before_after recebem target: &Value em vez de Expr; retirar somente eval_expr(target_expr). Preservar conversões/ordem/validação e avaliar args apenas após reconhecer selector. Essas assinaturas são internas a eval, não Rust externo.''',
'fields.rs':'''Extrair eval_field_access_with_target(access, target: Value, scopes, ctx, engine), interno a eval: o corpo do lookup atual é preservado, recebendo o target já resolvido. eval_field_access só avalia receiver e delega. Não substituir por eval_value_field_access puro: manter features, estilo/contexto, a11y, spans e avisos de módulos.''',
'field_access.rs':'''Reexportar somente eval_field_access_with_target com visibilidade interna a eval; não absorver lógica. APIs puras existentes permanecem.''',
'collections.rs':'''Manter try_dispatch_collection_method com assinatura vigente como adapter para args já avaliados. Adicionar CollectionDispatch { Handled(SourceResult<Value>), Unhandled(Value) } restrito ao crate e try_dispatch_collection_method_with com o mesmo catálogo estático, recebendo FnOnce(&mut Scopes, &mut EvalContext, &mut Engine) -> SourceResult<Args> em vez de Args. Uma macro local bind_args(args, expr) resolve args somente no braço reconhecido e retorna Handled; falha de args propaga SourceResult. Nome desconhecido devolve receiver original (reconstruído por move na família) em Unhandled; sem clones/probes de execução/segunda tabela. O complemento nominal Value mantém enum exaustivo. Integrar somente os três nomes bytes len/at/slice na tabela principal, delegando aos mesmos primitives; não mover algoritmos. Adapter eager converte Handled a Some e Unhandled a None. Catálogo Type/static wrappers e extensões Tekt preservados.''',
'bytes.rs':'''Manter try_dispatch_bytes_method apenas cfg(test) para seus controles existentes; o despacho produtivo único len/at/slice fica no pai, conforme responsabilidade de catálogo já vigente. Primitives bytes_len/bytes_at/bytes_slice e seus corpos permanecem intactos. Não duplicar validação/algoritmos.'''}
for src,owner in scope['owners'].items():
 p=Path(owner);s=p.read_text();line=f'- {nucleus} sha256:{effective}'
 if 'Núcleos Tekt:\n' in s:
  start=s.index('Núcleos Tekt:\n')+len('Núcleos Tekt:\n');end=s.index('\n\n',start);s=s[:start]+'\n'.join(sorted(s[start:end].splitlines()+[line]))+s[end:]
 else:s=s.replace('\n\n','\n\nNúcleos Tekt:\n'+line+'\n\n',1)
 name=Path(src).name;responsibility=responsibilities.get(name,'Composição interna somente: atualizar imports/reexports dos helpers/enum de transporte; preservar definições e assinaturas Rust públicas e não mover lógica para a fachada.')
 # distinguish the two mod.rs hubs
 if src.endswith('bindings/mod.rs'):responsibility='Reexportar eval_field_access_with_target e MutatingCall somente para eval; manter demais rotas internas, sem lógica.'
 if src.endswith('stdlib/mod.rs'):responsibility='Reexportar CollectionDispatch e try_dispatch_collection_method_with somente no crate e preservar os exports existentes, sem wrapper ou lógica.'
 if src.endswith('call_dispatch.rs'):responsibility='Atualizar somente o import interno do dispatcher diferido de coleções; manter exports públicos, tipos de spans, módulos e testes. Não criar novo callee ou AST global.'
 s+='''\n\n## Avaliação única em chamadas — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/call-once-2026-10-17/before.json.gz registra UTC,
status, diff HEAD --stat, fontes/L0/config e hashes. Os 2124 hashes da etapa
anterior foram verificados exatamente antes do baseline. before-probe.json.gz
e before-math-probe.json.gz conservam binários/argv/fixtures/saídas bilaterais.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 do binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
calc.abs/array.len/str.from-unicode medem receiver=3/args=2 contra 1/1;
module.measure/layout/within/before/after/func/has/body chegam a receiver=4;
fallback mutante mede args=3 contra 1. Métodos ausentes executam panic de
argumento antes do erro causal. math.mat soma duas vezes; array prévio (1,)
fica (1,1,1) contra (1,1). Array vazio mascara o defeito com truncate.
call.rs:70–118 perde receiver; collection_methods.rs:43 avalia args antes de
reconhecer operação; contextual/content/selector reavaliam AST no fallthrough;
method_dispatch.rs:247 descarta args; invoke.rs:66–120 compensa por nome.
O L0 call_dispatch.md já exige receiver/args uma vez. pinned-source.json.gz
fixa call.rs/methods.rs upstream: interesse é ordem/efeito/diagnóstico da
linguagem, não copiar Vm, storage ou igualdade Rust.

A primeira fixture declarava palavra reservada or como função: é inválida e
está arquivada em invalid-fixture-attempt, sem crédito para efeitos de chamada.
Ela revelou panic 101 em import inválido; manter aberto para correção causal
própria no owner de parsing/import. O baseline válido substitui só a fixture.

### Decisão e responsabilidade exclusiva

'''+responsibility+'''

Consumir o núcleo eval/call-once.toml pinado. Mudança mecânica interna e
correção de paridade ADR-0127 em fluxo contínuo. Assinaturas pub fn/pub use e
structs públicos não mudam; os parâmetros novos/retornos/helpers acima têm
visibilidade crate/eval, sem novo trait público, campo público, default ou
fase. Esta cláusula supersede somente restrições históricas conflitantes de
assinatura interna e compensação por nome; manter extensões Tekt existentes.

### Aceitação e limites

Testes funcionais co-localizados antes do código: efeitos de receiver/args,
fallback mutante, erro antes de args, where/with/selector/coleções preservados
e matriz sem truncar efeito legítimo. Confirmar RED funcional e GREEN.
Comparar casos bilaterais válidos e controles; hits e misses de todos os
métodos de coleção, erros, spans/occurrences e complementos de Value. Não
mascarar dívida residual de diagnóstico como igualdade. Testar dispatch
Unhandled sem perder receiver ou executar callback de args; contar avaliações
pelo efeito observado, não por bytes de PDF, Rust Eq ou microbenchmark.

Ownership/V26 antes de resselo só dos owners do lote. Build, suíte workspace,
Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
fmt, V1/V5/V15/V26 e reverse, SARIF diferencial sem erro novo. V19 uniforme
continua informativo, com matriz de alternativas e relação de comportamento;
sem supressão nem mudança de configuração. Comandos pesados sequenciais com
12 GiB/high10GiB/swap0/jobs1. Não publicar/commit/merge/release nem declarar o
objetivo global fechado por esta etapa.
''';p.write_text(s)
(out/'l0-record.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),effective=effective,owners={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['owners'].values()}),indent=2)+'\n');print('Prescribed',len(scope['sources']),effective)
