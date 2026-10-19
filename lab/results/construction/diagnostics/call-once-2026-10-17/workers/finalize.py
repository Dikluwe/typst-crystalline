from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,collections,difflib
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));v=json.load(open(out/'validation-result.json'));assert v['complete'];hashes={p:sha(p) for p in list(b['hashes'])+scope['nuclei']};assert hashes==v['source_hashes'];w=json.load(open(out/'workspace/workspace-result.json'));assert w['exit']==w['failed']==0 and w['passed']==8913 and w['ignored']==7;obs=json.load(open(out/'observables-result.json'));arch=json.load(open(out/'architecture-delta.json'));assert json.load(open(out/'public-contract-before.json'))==json.load(open(out/'public-contract-after.json'));assert json.load(open(out/'red-result.json'))['functional_red'];resources=[];seen=set();events=collections.Counter()
for p in sorted(out.rglob('*-resources.json')):
 r=json.load(open(p));key=json.dumps(r,sort_keys=True)
 if key in seen:continue
 seen.add(key);resources.append(dict(path=str(p),record=r));assert r['memory_before']['memory.max']==str(12*1024**3) and r['memory_before']['memory.high']==str(10*1024**3) and r['memory_before']['memory.swap.max']=='0';a=dict(x.split() for x in r['memory_before']['memory.events'].splitlines());z=dict(x.split() for x in r['memory_after']['memory.events'].splitlines());events.update({k:int(z[k])-int(a[k]) for k in z})
peak=max(int(r['record']['memory_after']['memory.peak']) for r in resources);assert events['max']==events['oom']==events['oom_kill']==0
# The catalogue tail and primitive algorithms must not change with the dispatch adapter.
for p,anchor in [('01_core/src/compiler/stdlib/collections.rs','pub(crate) fn collection_type_field'),('01_core/src/compiler/stdlib/collections/bytes.rs','pub(super) fn unexpected_named'),('01_core/src/compiler/eval/bindings/method_dispatch.rs','fn call_method_mut(')]:
 old=b['files'][p];new=Path(p).read_text();assert old[old.index(anchor):]==new[new.index(anchor):],p
measured=json.load(gzip.open(out/'after-probe.json.gz'))['binaries']
final_binaries={}
for name,info in measured.items():
 with open(info['path'],'rb') as stream:digest=hashlib.file_digest(stream,'sha256').hexdigest()
 assert digest==info['sha256'],(name,'binary changed after measurement')
 final_binaries[name]=dict(path=info['path'],sha256=digest)
state=dict(utc=now(),head=b['head'],binaries=final_binaries,source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),scope=scope,workspace=w,observables=obs,architecture=arch,memory=dict(peak_bytes=peak,peak_gib=peak/1024**3,events=dict(events),executions=resources),objective='active')
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'changes.diff').write_text(''.join(''.join(difflib.unified_diff(b['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in b['files']))
(out/'follow-ups.md').write_text('''# Frentes posteriores

- Corrigir o panic 101 ao importar fonte sintaticamente inválida, reproduzido com palavra reservada em invalid-fixture-attempt. A fixture válida das sondas de chamadas não apaga esse defeito; não classificar a tentativa inválida como prova de efeitos.
- Medir/generalizar outras ausências de campos Type e sua grafia de diagnóstico; este lote corrigiu Type::Array medido, sem normalizar silenciosamente os erros restantes.
- As sondas negativas matemáticas divergentes são preservadas em observables-result.json; não equivalem a cobertura de construção de frac/vec ou coerção de ints.
- O inventário global de V16/V19/V20 e as frentes RG1, FG1, RM1, SY1, igualdade std entre arquivos e demais itens anteriores continuam abertos. RG1 exige aprovação própria; candidatos futuros devem ser rebaseados sobre o estado atual.
- A classificação interna de Func/Type versus outras variantes testa o contrato já existente de chamada; ela não prova toda capacidade de cast upstream, nem fecha hints de shadowing ou identidade/morfologia de Symbol.
- Não houve commit, push, merge, release ou supressão de lint. O objetivo global permanece ativo.
''')
(out/'relatorio.md').write_text(f'''# Relatório — avaliação única de chamadas

Etapa concluída. Receivers, callee e argumentos já resolvidos são conservados entre interceptores e fallback. As sondas agora mostram uma avaliação por efeito; métodos ausentes e callee não chamável falham antes de executar argumentos comuns. Métodos mutantes conservam sua ordem especial de argumentos antes do local.

## Proveniência

HEAD `{b['head']}`, working tree não commitado. Baseline `{b['utc']}`, fechamento `{state['utc']}`. before.json.gz, red-state.json.gz e final-state.json.gz conservam fontes/L0, hashes, UTC, status e diff HEAD --stat. Os 2124 hashes da etapa anterior foram conferidos antes do baseline. Artefatos de sondas guardam argv, fixtures, saídas e binários. Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; fontes call.rs/methods.rs pinadas em pinned-source.json.gz.

## Correção e L0

Dezesseis pares consumer/L0 e núcleo `eval/call-once.toml` pinado. L0 antes dos testes/código, fluxo contínuo ADR-0127; sem campo, trait, assinatura ou reexport Rust público novo, default ou mudança de fase. public-contract-before/after confere interfaces; integrity-result confere fontes fora do escopo. Não houve commit/publicação/release.

A chamada guarda receiver/callee/Args no próprio stack e compartilha valores por empréstimo/ownership entre interceptores. Lookup completo usa o receiver já resolvido e preserva AST/spans, features, a11y, estilos e deprecações. Seletores/contexto/conteúdo não reavaliam a expressão quando recusam interceptar.

CollectionDispatch distingue Handled/Unhandled e devolve o receiver original no fallback. FnOnce adia Args até reconhecer o método na tabela estática única; o adapter antigo continua aceitando Args avaliados. Não há enum novo de callable, vtable, registry de métodos ou tabela de nomes duplicada. Bytes len/at/slice usam os mesmos primitives. O catálogo Type, wrappers estáticos, algoritmos bytes e writeback mutante conservam os corpos anteriores, conferidos no fechamento. As extensões Tekt declaradas permanecem.

MutatingCall transporta receiver e Args depois de argumentos→local; a invocação final reutiliza esses Args. O callee final é classificado antes de preparar argumentos. O complemento explícito de Value rejeita não chamáveis com diagnóstico causal, sem executar panic de argumento. Type::Array ausente agora usa backticks medidos do pin e o field-only span, como confirma legacy-array-probe.json com stderr idêntico nos dois binários. As compensações por nome mat, cópia isolada de scopes e cortes por prefix/array_lengths foram removidas; não se pode apagar efeitos legítimos para compensar execução duplicada.

## Evidência

- RED funcional: cinco falhas reais e um controle aprovado antes da implementação.
- GREEN: oito testes novos, incluindo duas matrizes de 39 discriminantes distintos de Value. Elas verificam Unhandled sem callback/sem trocar payload ou ponteiro do Vec e a classificação dos 37 não chamáveis antes de args, além dos dois caminhos Func/Type.
- Corpus principal: 36 casos bilaterais. Corpus matemático: sete casos. Resultado agregado: {obs['counts']}; {len(obs['changes'])} resultados corrigidos. Mensagens/hints/âncoras/traces são comparados integralmente, descontando somente o aviso Tekt já conhecido de arquivo vazio por literal exato; dados brutos permanecem. Negativos ainda divergentes não recebem crédito de paridade.
- calc.abs/array.len/str.from-unicode: receiver/args 3/2→1/1. Interceptores não aplicáveis podem medir 4/2→1/1; fallback mutante args 3→1. math.mat com array prévio (1,) produz (1,1), preservando o efeito legítimo; não (1,1,1) nem corte para (1,).
- Suíte workspace: {w['passed']} aprovados, {w['failed']} falhas e {w['ignored']} ignorados. Build, formato, Clippy all-targets estrito, Rustdoc estrito/quatro índices, V1/V5/V15/V26, reverse lineage e diff --check passaram.
- Diagnóstico Tekt: {arch['after']}; uma nota V19 nova do complemento de 37 não chamáveis está ligada às matrizes de 39 variantes. V20 não aumentou. Consultar architecture-delta.json para a justificação. Não se presume redução numérica de V16 por endurecer o match final: o baseline não reportava V16 em invoke.rs. Configuração e severidades não foram relaxadas.
- Recursos sequenciais: MemoryMax12GiB/High10GiB/swap0/Cargo jobs1. Pico {peak/1024**3:.6f} GiB ({peak} bytes); eventos {dict(events)}.

## Tentativas e pendências

A fixture inicial de import usava a palavra reservada or como nome de função; foi substituída para o corpus válido e preservada com o panic do Tekt para correção própria. O primeiro GREEN teve cinco controles passando e falhou na grafia antiga de Type::Array; o esperado do pin foi mantido e o literal produtivo corrigido. Uma tentativa da matriz usava path() num World em memória sem resolução filesystem; a fixture passou a RootedPath puro, sem implementar I/O em L1. As tentativas e recursos ficam arquivados. Três bindings de Args ignorados nas extensões de string foram explicitados com prefixo _, conservando sua avaliação e sem supressões. Dois padrões com captura @ foram achatados para conservar V20 sem aumento. A suíte completa encontrou um único esperado antigo de #array.nope; legacy-array-probe.json confirmou o diagnóstico e a âncora no pin, e somente esse esperado foi atualizado, sem retirar assertivas.

O objetivo global continua ativo; follow-ups.md registra parsing/import inválido, erros de campos/negativos matemáticos e os contratos anteriores que permanecem abertos. Esta etapa não afirma paridade universal de chamadas, todos os casts, schemas, hints ou conteúdo renderizado.
''');print('Closed',w['passed'],obs['counts'],len(obs['changes']),peak)
