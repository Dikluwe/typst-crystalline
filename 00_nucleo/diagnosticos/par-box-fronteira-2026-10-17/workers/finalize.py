from pathlib import Path
import datetime,gzip,json,hashlib,subprocess,collections
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));v=json.load(open(out/'validation-result.json'));assert v.get('complete') and v['sources_unchanged'] and all(sha(p)==h for p,h in v['source_hashes'].items())
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head']
assert '17 passed; 0 failed' in (out/'accepted-green.log').read_text()
changed=[p for p,h in b['hashes'].items() if sha(p)!=h];assert set(changed)==set(scope['sources'])|set(scope['owners'].values())
rgs=Path('00_nucleo/materialization/table-row-groups-2026-10-17');assert sha(rgs/'SHA256SUMS')=='a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9'
for line in (rgs/'SHA256SUMS').read_text().splitlines():h,p=line.split('  ',1);assert sha(rgs/p)==h
rs=json.load(open(rgs/'scope.json'));protected=rs['semantic_sources']+rs['metadata_only_sources']+list(rs['owners']);assert all(sha(p)==b['hashes'][p] for p in protected)
resources={str(p.relative_to(out)):json.load(open(p)) for p in out.rglob('*resources.json')};events=lambda x:dict((n,int(v)) for n,v in (s.split() for s in x.splitlines()))
for r in resources.values():
 m=r['memory_after'];assert m['memory.max']=='12884901888' and m['memory.high']=='10737418240' and m['memory.swap.max']=='0';assert events(m['memory.events'])['oom']==events(m['memory.events'])['oom_kill']==0
peak=max(int(r['memory_after']['memory.peak']) for r in resources.values());high=sum(events(r['memory_after']['memory.events'])['high'] for r in resources.values());cap=sum(events(r['memory_after']['memory.events'])['max'] for r in resources.values());suite=json.load(open(out/'workspace/workspace-result.json'));assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged'];obs=json.load(open(out/'observables-result.json'));delta=json.load(open(out/'architecture-delta.json'));assert delta['all_diagnostics_preserved'];assert sha(b['linter']['path'])==b['linter']['sha256'];assert json.load(open(out/'rustdoc-result.json'))['exit']==json.load(open(out/'strict-details.json'))['exit']==0
rows=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];remaining=[r for r in rows if r['ruleId']=='V16'];assert len(remaining)==125;(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');groups=collections.defaultdict(list)
for r in remaining:
 loc=r['locations'][0]['physicalLocation'];groups[loc['artifactLocation']['uri']].append(loc['region']['startLine'])
(out/'inventario-v16-restante.md').write_text('# V16 após a fronteira Par/Box\n\n125 erros em '+str(len(groups))+' consumers. Inventário integral atualizado, sem supressões; final-state.json.gz guarda proveniência.\n\n| Consumer | Casos | Linhas |\n|---|---:|---|\n'+''.join(f'| `{p}` | {len(ns)} | {", ".join(map(str,ns))} |\n' for p,ns in sorted(groups.items())))
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],working_tree='uncommitted',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],changed_since_checkpoint=changed,tests={k:suite[k] for k in ['passed','failed','ignored']},observables=obs,architecture=delta,resources=resources,peak_bytes=peak,memory_high_events=high,memory_max_events=cap,RG1_preserved=True,remaining_v16=125,public_api_changes=0)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'pendencias.json').write_text(json.dumps(dict(global_v16=125,RG1='Pending ADR-0127 approval; package and protected sources unchanged',fractional_box='Composition stage B still incomplete; unchanged empty-fr-box witness: painted backing width is zero; logical allocation/suffix x already advances',pagination_limits=['Columns, reserves/floats and suffix transactions retain existing protocols','External decoration collector excluded from new ordinary root admission'],inherited=['NativeText positional/relative constructor audit','Block negative margins rejected by constructor','Existing PDF xMax residue classification','Native SVG CLI treats {p} literally and emits only the first page in measured multi-page controls','Other open findings in prior reports']),indent=2)+'\n')
report=f'''# Relatório — fronteira Par/Box e paginação finita

Etapa concluída. Corrigida a transição de parágrafos/blocos para caixas inline não fracionárias e a admissão da linha inteira em páginas finitas ordinárias. **Nenhuma API pública mudou.** O gate global continua aberto: **125 V16**, **1065 notas V19** e **867 notas V20**, integralmente preservados no diferencial. Esta etapa trata falhas funcionais encontradas na validação do lote anterior; não contabiliza redução fictícia de V16. Não houve commit/push do Typst, merge ou release.

## Correções

Três consumers e seus L0 proprietários mudaram: boxed.rs, paragraph.rs e line_commit.rs. L0 primeiro, correção interna/de paridade ADR-0127, com resselo restrito dos três pares; dependências/núcleos e contratos públicos foram preservados.

- Box com dimensão não fracionária usa a fronteira causal e colapsa a margem de parágrafo com a margem pendente. Margens fortes de Block permanecem fortes. A própria caixa fornece seu ascent; fontes não inventam um strut para sua geometria.
- O helper privado de espaçamento recebe o tamanho ativo; o helper lexical anterior delega a ele e conserva seus consumidores. Valores relativos e restauração de escopos permanecem.
- O fechamento ordinário compara o fundo real da linha e o ajuste tardio de ascent antes de comitar. A linha inteira, seus wrappers e posições internas migram juntos para a página correta; avanço horizontal e indentação acompanham a margem da nova página. A baseline usa o ascent real uma vez, inclusive quando texto maior vem depois da caixa.
- Primeiro frame maior que a região é comitado uma vez; não cria página terminal vazia. A testemunha de unidade anterior pertence à região vigente: altura herdada de uma página anterior não provoca migração adicional depois de pagebreak.

A primeira caixa não fracionária de uma nova raiz finita ordinária vazia recupera o topo geométrico, em vez de herdar strut/margem da página anterior. As rotas de regiões reservadas, colunas, floats/transações e collector externo de decoração conservam o protocolo anterior. Elas não receberam certificação universal neste lote. Box com largura `1fr` conserva a composição B explicitamente incompleta e sua testemunha divergente.

## Validação

**17 testes novos passaram.** O primeiro RED teve 14 falhas e um controle aprovado. A contagem bruta de carriers de um teste incluía Text vazio: o teste foi refinado para conteúdo visível e geometria, sem atribuir esse erro de fixture a uma falha do produto. O candidato inicial passou 13/15; um dos problemas restantes era essa fixture e o outro a inclusão da própria linha no critério de conteúdo prévio. Um refutador adicional após pagebreak confirmou o risco de altura herdada e precedeu o ajuste da testemunha regional. Outro RED comprovou o topo indevido da primeira caixa pequena após pagebreak; sua admissão geométrica também foi corrigida. Logs e candidatos intermediários foram conservados, sem contar erro mecânico de teste como RED funcional.

A repetição contra o vanilla pinado mediu **{obs['pairs']} pares de execução, somados entre os corpora**, incluindo entradas repetidas entre famílias: **{obs['certified']} certificados**, **{obs['repaired']} observações divergentes corrigidas** e **{obs['preserved']} controles preservados**; **{len(obs['retained_open'])} caso fracionário permanece aberto e inalterado**. Não são {obs['pairs']} defeitos independentes. Páginas e palavras são exatas; dimensões de página, xMin/yMin/yMax e bounds vermelhos do corpus SVG principal usam tolerância fixa de 0.002 pt. A cor foi verificada como #ff4136. Cinco pares de pagebreak usam vetores de todas as páginas PDF convertidos por pdftocairo, com comandos/hashes por página. O binário anterior dessas cinco sondas é comprovadamente idêntico ao checkpoint e registra separadamente o build_source_hashes e a árvore observada. xMax tem resíduo de até {obs['max_xmax_residue']:.6f} pt, registrado separadamente e sem ampliar a tolerância geométrica. Primeiro frame alto pode naturalmente exceder a página; a aceitação verifica progresso/páginas, sem prometer que tinta fora da região passe a ser visível.

Suíte completa: **{suite['passed']} passaram, {suite['failed']} falhas e {suite['ignored']} ignorados**. Build workspace, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, V1/V5/V15/V26 estrito, linhagem reversa e diff check passaram. O erro de sintaxe inicial do harness regional foi corrigido antes de medir e não conta como RED funcional. SVG nativo com template permanece uma pendência CLI: os arquivos medidos gravam literalmente {{p}} e só a primeira página; a geometria regional foi verificada pelo PDF.

SARIF global continua com exit 1 pelos 125 V16 conhecidos; o diferencial preserva todos os diagnósticos, com linhas atualizadas e nenhuma supressão.

## RAM e proveniência

Comandos pesados sequenciais, cada gate em cgroup próprio: **MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero, Cargo jobs=1**. Pico **{peak} bytes ({peak/1024**3:.3f} GiB)**; eventos memory.max **{cap}**, memory.high **{high}**, sem OOM/OOM-kill. É memória do grupo, incluindo cache, não RSS isolado nem prova de melhoria de desempenho por comparação entre políticas de contabilização.

HEAD **{b['head']}**, working tree não commitado. before.json.gz, extended-before.json.gz, red-state.json.gz, medições bilaterais, validation-result.json e final-state.json.gz guardam UTC, status/diffstat exatos, hashes de fontes/L0/configuração/binários, argv e fontes tipográficas. Oráculo 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 {b['linter']['sha256']}.

[Inventário V16 restante](inventario-v16-restante.md), [pendências](pendencias.json) e [próximas auditorias](next-audit.md). O pacote RG1, manifesto a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9, continua aguardando aprovação e foi preservado, assim como seus consumers/owners. O objetivo geral permanece ativo.
'''
(out/'relatorio.md').write_text(report)
for p in (out/'workers').rglob('*.pyc'):p.unlink()
cache=out/'workers/__pycache__'
if cache.exists() and not any(cache.iterdir()):cache.rmdir()
manifest=''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='SHA256SUMS');(out/'SHA256SUMS').write_text(manifest)
print(json.dumps(dict(tests=suite['passed'],remaining_v16=125,peak_bytes=peak,observables={k:obs[k] for k in ['pairs','certified','repaired','preserved']},manifest_sha256=sha(out/'SHA256SUMS'))))
