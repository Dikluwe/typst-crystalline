from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,collections,difflib
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));v=json.load(open(out/'validation-result.json'));assert v['complete'];hashes={p:sha(p) for p in list(b['hashes'])+scope['nuclei']};assert hashes==v['source_hashes'];w=json.load(open(out/'workspace/workspace-result.json'));assert w['exit']==w['failed']==0;assert w['passed']==8905 and w['ignored']==7;probe=json.load(open(out/'probe-verification.json'));paths=json.load(gzip.open(out/'paths-verification.json.gz'));arch=json.load(open(out/'architecture-delta.json'));causal=json.load(open(out/'causal-verification.json'));assert arch['exact_mapped_diagnostics_preserved'];assert json.load(open(out/'red-result.json'))['functional_red'];assert json.load(open(out/'public-contract-before.json'))==json.load(open(out/'public-contract-after.json'))
resources=[];seen=set();events=collections.Counter()
for p in sorted(out.rglob('*-resources.json')):
 r=json.load(open(p));key=json.dumps(r,sort_keys=True)
 if key in seen:continue
 seen.add(key);resources.append(dict(path=str(p),record=r));assert r['memory_before']['memory.max']==str(12*1024**3);assert r['memory_before']['memory.high']==str(10*1024**3);assert r['memory_before']['memory.swap.max']=='0';a=dict(x.split() for x in r['memory_before']['memory.events'].splitlines());z=dict(x.split() for x in r['memory_after']['memory.events'].splitlines());events.update({k:int(z[k])-int(a[k]) for k in z})
peak=max(int(r['record']['memory_after']['memory.peak']) for r in resources);assert events['max']==events['oom']==events['oom_kill']==0
layout={}
for phase,file in [('before','red-test-output.log'),('after','validation-green.log')]:
 import re
 s=(out/file).read_text();m=re.findall(r'MODULE_LAYOUT (\d+) (\d+)',s);assert len(m)==1;layout[phase]=dict(Module=int(m[0][0]),ModuleInner=int(m[0][1]))
state=dict(utc=now(),head=b['head'],source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),scope=scope,workspace=w,architecture=arch,probe=probe,causal=causal,paths_summary={k:paths[k] for k in ['disabled_rejections','active_controls']},memory=dict(peak_bytes=peak,peak_gib=peak/1024**3,events=dict(events),executions=resources),layout=layout,objective='active')
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'changes.diff').write_text(''.join(''.join(difflib.unified_diff(b['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in b['files']))
(out/'relatorio.md').write_text(f'''# Relatório — gate HTML e identidade dos módulos

Etapa concluída. `std.html` exige agora a feature `html`, inclusive em aliases,
closures e módulos importados. Módulos comuns chamados `global` ou `format`
conservam seus próprios campos sem gates indevidos. O objetivo global segue ativo.

## Proveniência e limites

HEAD `{b['head']}`, working tree não commitado. Baseline `{b['utc']}`,
fechamento `{state['utc']}`. before.json.gz, red-state.json.gz e final-state.json.gz
registram status, diff HEAD --stat, fontes/L0 e hashes exatos. Baseline conferido
contra os 2123 hashes da etapa anterior. As medições conservam UTC, argv,
fixtures, saídas e SHA-256 dos binários. Vanilla: pin
`9f2b6e8715237cb086899a42873660fe744622e8`, binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Nenhum commit, push, merge ou release.

## Correção

Dez pares consumer/L0 e um Núcleo Tekt compartilhado e pinado. A especificação
foi atualizada antes dos testes/código, em fluxo contínuo ADR-0127. `ModuleInner`
recebe identidade privada Ordinary/Std/Format. Somente as factories internas de
composição concedem Std/Format; `Module::new` sempre cria módulo comum. As três
construções reais de std e a factory de format usam essas factories. A identidade
acompanha o Arc, sem depender do nome local ou de bindings semelhantes.

O gate HTML reconhece std e format; asset/bundle conservam suas features.
O helper único é compartilhado entre acesso a campo e chamadas, antes dos
argumentos. Set preserva erros de resolução de pure_path e identifica o alvo
HTML por FuncRepr::Format(Html, _) em vez da grafia do alvo, conservando o
validador pretty existente para aliases. Condition false continua atalhando. Não
há mudança de campos, métodos ou assinaturas Rust públicos, Eq de Module,
Debug, content, metadata, setters, fases ou defaults. public-contract-before/after
confere as declarações; integrity-result.json confere que fontes fora do escopo
permanecem iguais. Tamanho observado (bytes): Module {layout['before']['Module']}→{layout['after']['Module']},
ModuleInner privado {layout['before']['ModuleInner']}→{layout['after']['ModuleInner']}.
Não se alega ganho de desempenho ou memória de documentos.

## Validação

- RED funcional: três falhas antes do código e dois controles aprovados; GREEN:
  cinco testes novos. Gates nos oito perfis, aliases/closures com shadowing,
  cópia de bindings nativos em módulos comuns, igualdade/debug/estado vazio e
  precedência de diagnóstico antes dos argumentos de chamada/set nos três targets.
- {probe['cases']} sondas bilaterais: {len(probe['changed'])} resultados corrigidos;
  categorias em probe-verification.json. Rejeições com feature desligada são
  registradas separadamente e não recebem crédito de paridade de feature ativa.
  Mensagem, hints e âncoras coincidem com o pin; somente o aviso já existente de
  arquivo vazio do Tekt é descontado por literal exato, preservado na coleta.
- {causal['cases']} sondas adicionais: 26 casos válidos de diagnóstico, hints, âncoras e
  efeitos de chamada/set, aliases HTML ativos, alvo ausente e condição falsa;
  dois controles de rejeição sintática, sem crédito de execução de set.
  Houve 16 resultados causais corrigidos, além dos 28 da matriz de identidade.
- {paths['disabled_rejections']} rejeições corretas em 115 caminhos std.html nos quatro perfis
  com Html desligada; {paths['active_controls']} controles desses caminhos nos quatro perfis
  ativos conservaram type/where/kind. Outputs vanilla reutilizados somente após
  conferir binário pinado, expressão, flags e arquivo de entrada vazio.
- Suíte workspace: {w['passed']} aprovados, {w['failed']} falhas, {w['ignored']} ignorados.
  Build, Clippy all-targets estrito, Rustdoc estrito, fmt, V1/V5/V15/V26,
  reverse lineage e git diff --check passaram.
- Diagnóstico global preservado: {arch['counts']}. Nenhuma supressão ou configuração
  de lint alterada; diferencial exato após mapear deslocamentos de linhas.
- Recursos sequenciais: MemoryMax 12 GiB, High 10 GiB, swap zero e Cargo jobs=1.
  Pico de cgroup {peak/1024**3:.6f} GiB ({peak} bytes); eventos {dict(events)}.

## Pendências e tentativas

A igualdade entre módulos std de arquivos diferentes permanece divergente
(Tekt false / pin true), medida em oito controles; não foi alterada por esta
correção de gate. Permanecem os 117 V16 anteriores e as frentes RG1, FG1, RM1,
SY1 e demais contratos específicos já inventariados. RG1 continua dependente de
aprovação; candidatos futuros exigem rebase sobre o estado vigente.

A primeira sonda usava incorretamente let m = import e está arquivada, sem crédito;
as sondas válidas usam import as. Uma compilação inicial dos testes encontrou
import Rust incorreto de SysInputs; foi corrigido antes do RED funcional, com
saída e recursos arquivados. Essas tentativas não foram tratadas como prova de
falha do produto. O RED funcional foi confirmado antes da implementação. O primeiro GREEN revelou a execução prematura de
argumentos em chamadas; as sondas causais também mediram erros apagados em
set. A continuação foi prescrita nos L0 antes de corrigir estes caminhos, e
a versão candidata anterior permanece arquivada em candidate-1.
''')
print('Final state and report',w['passed'],peak,layout)
