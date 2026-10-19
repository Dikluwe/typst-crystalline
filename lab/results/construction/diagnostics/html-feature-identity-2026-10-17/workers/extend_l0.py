from pathlib import Path
import gzip,json,hashlib,re,datetime,shutil
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();extra=['01_core/src/compiler/eval/bindings/mod.rs','01_core/src/compiler/eval/bindings/field_access.rs','01_core/src/compiler/eval/call_dispatch/collection_methods.rs','01_core/src/compiler/eval/rules/set.rs']
for src in extra:
 owner=re.search(r'^//! @prompt (.+)$',Path(src).read_text(),re.M)[1];assert sha(src)==b['hashes'][src] and sha(owner)==b['hashes'][owner];b['files'][src]=Path(src).read_text();b['files'][owner]=Path(owner).read_text();scope['sources'].append(src);scope['owners'][src]=owner
with gzip.open(out/'before.json.gz','wt') as f:json.dump(b,f)
(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n');archive=out/'candidate-1';archive.mkdir()
for p in list(out.glob('validation-*'))+list(out.glob('*-resources.json')):
 if p.name in ['before-probe-resources.json','red-resources.json','before-set-probe-resources.json']:continue
 shutil.copy(p,archive/p.name)
# source snapshots of candidate-1 ensure ownership/differential can be re-audited
with gzip.open(archive/'sources.json.gz','wt') as f:json.dump({p:Path(p).read_text() for p in scope['sources']+list(scope['owners'].values())},f)
nucleus=scope['nuclei'][0];s=Path(nucleus).read_text();effective=hashlib.sha256(s.encode()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
responsibilities={extra[0]:'Somente reexportar check_module_field_feature com visibilidade interna a eval, sem absorver lógica.',extra[1]:'Somente reexportar check_module_field_feature do owner fields com visibilidade interna a eval.',extra[2]:'Invocar check_module_field_feature no receiver já resolvido/cacheado, antes de coletar/avaliar argumentos. Não reavaliar o receiver, mudar dispatch de coleção nem alterar os demais fallbacks.',extra[3]:'''Após condition false/true/tipo já vigente, propagar o erro causal de resolução do target pure_path em vez de apagá-lo com ok(). Essa mudança inclui feature gate, variável ausente e field ausente; probes mostram aceitação indevida no Tekt e erro causal no pin. Não generalizar a resolução de targets não pure_path. Conservar target_function disponível por as_ref quando extraindo native_target. O braço existente de set HTML usa a identidade real FuncRepr::Format(FormatElementKind::Html, _) do target_function e não a grafia format.html; mover somente esse braço para antes do bloco restante de FieldAccess. Remover gate literal duplicado dentro desse braço: a resolução normal de target já autentica e aplica a feature. Assim aliases/std/raiz nativa usam o validador pretty existente e homônimos comuns não entram no braço por grafia. Preservar corpo de validação pretty, spans, condición falsa, demais formatos/targets e prioridades. Não introduzir API pública ou nova fase.''' }
for src,owner in scope['owners'].items():
 p=Path(owner);s=p.read_text();line=f'- {nucleus} sha256:{effective}'
 if src in extra:
  if 'Núcleos Tekt:\n' in s:s=s.replace('Núcleos Tekt:\n','Núcleos Tekt:\n'+line+'\n',1)
  else:s=s.replace('\n\n','\n\nNúcleos Tekt:\n'+line+'\n\n',1)
 s+='''\n\n## Continuação causal do gate — chamadas e set rules

Medição anterior à decisão: candidate-1/validation-green.log registra quatro
controles aprovados e falha real do teste de argumentos: std.html(panic(...))
executa panic antes do gate. before-set-probe.json.gz registra UTC, hashes,
fontes e saídas bilaterais no mesmo HEAD/árvore não commitada: std.html,
format.html, std.asset e format.bundle chamados avaliam argumentos antes do
gate; set std.html/std.format.html/alias apaga o erro do target e ignora os
argumentos. Variável/field ausente em set também é aceito indevidamente;
condition false conserva atalhamento. Html ativo em set por alias ignora panic
no Tekt e o executa no pin. São ordem causal, diagnóstico e efeito da linguagem.
collection_methods.rs:36 avalia argumentos antes de autenticar módulo; set.rs:
44 usa ok() na resolução e :326 autentica HTML pela sintaxe literal.

'''+responsibilities.get(src,'Conservar a responsabilidade anterior; fields extrai os três gates para check_module_field_feature(&Value, &str, Span, Features) -> SourceResult<()>, interno a eval, e o chama antes do lookup. Os demais owners não duplicam os diagnósticos nem regras.')+'''

L0 precede esta continuação; correção interna/paridade em fluxo ADR-0127.
O núcleo builtin-module-identity continua pinado e sem alteração. O contrato
público e os campos externos permanecem iguais; nenhum fallback novo ou
supressão. Os controles e a matriz dos oito perfis permanecem; acrescentar
verificação bilateral dos efeitos e de set com/sem feature. A suite completa
regressa após o source freeze novo, sem reaproveitar gates de fonte alterada.
''';p.write_text(s)
# update API snapshot only with the original declarations of the extra files
contract=Path(out/'workers/verify.py').read_text().split("values={p:")[0];ns={};exec(contract,ns);values={p:ns['contract'](b['files'][p]) for p in scope['sources']};(out/'public-contract-before.json').write_text(json.dumps(values,indent=2)+'\n')
(out/'extended-l0-record.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources=scope['sources'],owners={p:sha(p) for p in scope['owners'].values()}),indent=2)+'\n');print('L0 expanded to',len(scope['sources']))
