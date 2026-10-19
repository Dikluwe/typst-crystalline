from pathlib import Path
import datetime,gzip,json,hashlib,subprocess,collections,difflib
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));v=json.load(open(out/'validation-result.json'));assert v.get('complete') and v['sources_unchanged'];assert all(sha(p)==h for p,h in v['source_hashes'].items());assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head'];changed=[p for p,h in b['hashes'].items() if sha(p)!=h];assert set(changed)==set(scope['sources'])|set(scope['owners'].values())
source='01_core/src/compiler/eval/operators/equality.rs';assert '1 passed; 5 failed;' in (out/'red.log').read_text();assert '6 passed; 0 failed;' in (out/'green.log').read_text();suite=json.load(open(out/'workspace/workspace-result.json'));assert (suite['exit'],suite['passed'],suite['failed'],suite['ignored'])==(0,8816,0,7);obs=json.load(open(out/'observables-result.json'));assert obs['pairs']==24 and len(obs['repaired'])==17 and obs['no_measured_regressions'];replay=json.load(open(out/'ts1-replay.json'));assert replay['pairs']==105 and replay['unchanged_cases']==102 and len(replay['intended_boolean_repairs'])==3;assert json.load(open(out/'parbox-regressions.json'))['pairs']==50;assert json.load(open(out/'rg1-controls.json'))['pairs']==105;delta=json.load(open(out/'architecture-delta.json'));assert delta['after'].get('V16:error')==124 and delta['no_new_errors_or_warnings']
packages={'text-size-length-2026-10-17':'771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba','table-row-groups-2026-10-17':'a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9'}
for name,h in packages.items():
 p=Path('00_nucleo/materialization')/name;assert sha(p/'SHA256SUMS')==h
 for line in (p/'SHA256SUMS').read_text().splitlines():want,f=line.split('  ',1);assert sha(p/f)==want
prior=Path('00_nucleo/diagnosticos/text-size-length-2026-10-17');prior_state=json.load(gzip.open(prior/'final-state.json.gz'));assert all(v['source_hashes'][p]==h for p,h in prior_state['source_hashes'].items() if p not in changed)
resources={str(p.relative_to(out)):json.load(open(p)) for p in out.rglob('*resources.json')};peak=max(int(r['memory_after']['memory.peak']) for r in resources.values());events={n:sum(int(next(l.split()[1] for l in r['memory_after']['memory.events'].splitlines() if l.startswith(n+' '))) for r in resources.values()) for n in ['high','max','oom','oom_kill']}
for r in resources.values():
 m=r['memory_after'];assert (m['memory.max'],m['memory.high'],m['memory.swap.max'])==('12884901888','10737418240','0')
assert events['oom']==events['oom_kill']==0
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],working_tree='uncommitted',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],changed_since_TS1=changed,tests={k:suite[k] for k in ['passed','failed','ignored']},observables=obs,ts1_replay=dict(pairs=105,unchanged_cases=102,intended_boolean_repairs=replay['intended_boolean_repairs']),architecture=delta,peak_bytes=peak,memory_events=events,resources=resources,public_api_changes=0,global_goal='active: known V16 and independent parity debts remain',proposal_manifests_preserved=packages)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'patch-since-TS1.diff').write_text(''.join(''.join(difflib.unified_diff(b['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in changed))
rows=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];remaining=[r for r in rows if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');groups=collections.defaultdict(list)
for r in remaining:
 loc=r['locations'][0]['physicalLocation'];groups[loc['artifactLocation']['uri']].append(loc['region']['startLine'])
(out/'inventario-v16-restante.md').write_text('# V16 após TS1 e correção de igualdade\n\n124 erros em '+str(len(groups))+' consumers. Proveniência: final-state.json.gz e SARIF integral.\n\n| Consumer | Casos | Linhas |\n|---|---:|---|\n'+''.join(f'| `{p}` | {len(ns)} | {", ".join(map(str,ns))} |\n' for p,ns in sorted(groups.items())))
combined_peak=max(peak,prior_state['peak_bytes']);edges=json.load(open(out/'edge-measurement.json'));edge_summary={name:{r['binary']:r['exit'] for r in edges['rows'] if r['name']==name} for name in dict.fromkeys(r['name'] for r in edges['rows'])}
(out/'pendencias.json').write_text(json.dumps(dict(V16=124,RG1='Pending human approval; do not overwrite TS1 when rebasing candidates',still_open=['Text Paint Gradient/Tiling and relative:self diagnostic','show text selector','Builtin Heading defaults when a show function replaces the node','Script size/offset parity; relative and absolute controls agree within each renderer','Custom-only constructor presence outside this operator projection is not universally certified','Fractional Box after 1fr, multi-page SVG and integer literal minimum construction'],next_v16='Eight cases in four modules were triaged but not changed in this TS1 approval turn; resume from tekt-v16-abs-2026-10-17/next-batch-triage.json'),indent=2)+'\n')
(out/'relatorio.md').write_text(f'''# Relatório final da etapa — TS1 e igualdade Styled

**Etapa encerrada com as regressões observadas de TS1 corrigidas.** O projeto inteiro continua aberto: há124 V16 e frentes de paridade independentes no inventário. Não houve commit/push do Typst, merge ou release.

## O que foi entregue

[Checkpoint TS1](../text-size-length-2026-10-17/relatorio.md): Style::Size(Pt)→Size(Length) e StyleDelta.size:Option<f64>→Option<Length>, conforme aprovação humana de manifesto771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba. TextStyle.size permanece Pt resolvido. Constructor/set conservam abs+em, busca posicional por tipo, preferência nomeada, corpo None/Str/Content, diagnóstico e ocorrência causal. Captura/collapse fazem composição afim; Raw/math usam comprimentos absolutos já resolvidos. Heading afim e filhos relativos foram corrigidos em layout, sem mover fase. Foram20 consumers, incluindo o adapter de Heading exigido pelos testes, e20 L0s mais o núcleo compartilhado.

A validação TS1 expôs duas regressões de igualdade (relative-vs-zero e affine-vs-absolute-part, true→false). Este lote separado as fecha: somente `01_core/src/compiler/eval/operators/equality.rs` e seu L0 mudaram desde o checkpoint. Após morph_canon existente, o operador compara uma projeção temporária que conserva Styled/child, coalesce Styled adjacente e ignora seu mapa; todos os outros nós permanecem completos. A projeção não entra no render, em Value/Engine, Hash, ObservationEq ou terminação. **PartialEq Rust e morph_canon não mudaram.** Wrapper versus plain, Strong/Emph e corpos distintos continuam distintos. Eq/Neq, arrays/dicts e pertença usam a mesma relação.

Foi correção interna de paridade em fluxo contínuo ADR-0127, com L0 antes de teste/código; nenhuma nova API pública, default ou fase. A referência pinada foundations/content/mod.rs:376–385,779–782 mede coalescência e igualdade pelo child. Isso não é igualdade dos valores de tamanho declarados nem apagamento indiscriminado de wrappers.

## Testes e resultados

TS1 teve seis RED iniciais e quatro adicionais de Heading,14 testes novos GREEN e8810 testes aprovados no checkpoint. Igualdade teve cinco RED funcionais e um grupo negativo já aprovado; depois os **seis testes novos passaram**. A suíte final completa tem **8816 aprovados, zero falhas e sete ignorados**.

**24 pares bilaterais de igualdade coincidem agora:17 divergências reparadas e sete controles preservados.** Incluem as duas regressões TS1, tamanho relativo/resolvido, cores/lang, wrappers adjacentes, filhos nominais, containers, in/not-in, != e NaN de estilo, sem tornar Float NaN reflexivo. O primeiro corpus tinha chamadas aninhadas como texto literal e calc.nan ausente; foi corrigido antes de especificar/implementar. Não conta como RED funcional. Um import de teste introduzido mecanicamente foi removido antes dos gates finais.

O replay dos **105 pares textuais TS1** preserva102 casos na geometria/saídas medidas e muda somente três resultados booleanos false→true, coincidentes com o pin. Os **50 pares Par/Box** mantêm a geometria anterior dentro0.002pt. Os **105 pares RG1** preservam diagnósticos/efeitos, estrutura HTML e linhas coloridas SVG. Oito pares de cor sólida já foram validados no checkpoint em vetores de PDF, comparando a paleta real (#ff4136), sem presumir vermelho RGB puro.

Build workspace, fmt, Clippy workspace/all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices HTML, V1/V5/V15/V26 estrito, reverse lineage e diff check passaram. SARIF final: **{delta['after']}**. Nenhum erro/warning novo; diagnósticos fora do consumer de igualdade são idênticos ao checkpoint e V16 por consumer permanece. O gate global continua exit1 pelos124 V16 conhecidos; não foi relaxado, e não houve supressão genérica de V19.

## RAM, API e proveniência

Teto rígido **12GiB**, MemoryHigh10GiB, swap zero, Cargo jobs1 e comandos pesados sequenciais. Pico deste lote **{peak} bytes ({peak/1024**3:.3f} GiB)**; maior pico dos dois lotes **{combined_peak} bytes ({combined_peak/1024**3:.3f} GiB)**. Eventos deste lote: {events}. Sem OOM/OOM-kill. Memória de cgroup inclui cache, não é RSS nem prova de otimização.

StyleDelta/Styles medidos336bytes, antes328; Style permanece32. O custo aprovado é+8bytes por campo de tamanho. Chamadores Rust devem migrar Size(Pt(v)) para Size(Length::pt(v)) e size:Some(v) para Some(Length::pt(v)). A sonda externa da API do checkpoint passou. Este lote de igualdade não mudou layout de tipos.

Os probes não finitos iniciais de TS1 usavam namespaces ausentes e não provam admissão desses valores. O lote final usa float("nan")/float("inf"); códigos medidos: `{edge_summary}`. A certificação é de admissão/repr, não de render NaN/infinito. Zero/negativos finitos também têm coletas de render no checkpoint; não foi inventada uma restrição de positividade.

HEAD **{b['head']}**, working tree não commitado. before/final-state.json.gz, RED/GREEN, validation-result.json, recursos e corpora registram UTC, status/diffstat, hashes de todas as fontes/L0/configuração e de binários/artefatos, comandos e saídas. Vanilla **9f2b6e8715237cb086899a42873660fe744622e8**, SHA-2567fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter **12797073972e375d9ab078cf6750c8b30b71e2b9**, SHA-256 {b['linter']['sha256']}. A branch autorizada do linter já estava publicada e seu pin CI foi preservado.

Os manifestos públicos TS1/RG1 foram conferidos integralmente e preservados. RG1 continua aguardando confirmação; ao materializar seus candidatos, rebasear apply/tests sobre TS1 e esta correção, sem sobrescrever a árvore. Nenhum contrato RG1 foi implementado aqui.

## Ordem seguinte e limites

[Inventário atualizado](inventario-v16-restante.md):124 V16 em {len(groups)} consumers. Retomar primeiro os oito casos já triados de type_fields/collections/figure_image/casting, medindo antes e sem alterar API sem lista/revisão. Figure/kind e casts relativos têm limitações próprias; não trocar por grupos silenciosos.

Permanecem Paint Gradient/Tiling textual, seletor show text, defaults de Heading substituído por função, scripts, presença de constructores custom-only fora da projeção aqui medida, Box1fr, SVG multipágina e literal inteiro mínimo. O lote não certifica todo Content nem toda a paridade. As diferenças de scripts são reproduzidas no controle absoluto e relativo de cada backend. [Pendências](pendencias.json) conserva a lista; as duas regressões de igualdade que impediam a entrega desta etapa estão resolvidas.

[Patch desde TS1](patch-since-TS1.diff) · [Checkpoint anterior](../text-size-length-2026-10-17/final-state.json.gz).
''')
p=out/'relatorio.md';t=p.read_text()
for a,z in [('há124','há 124'),('manifesto771','manifesto 771'),('Foram20','Foram 20'),('e20','e 20'),('Heading,14','Heading, 14'),('e8810','e 8810'),('coincidem agora:17','coincidem agora: 17'),('preserva102','preserva 102'),('dentro0.002','dentro de 0.002'),('exit1','exit 1'),('pelos124','pelos 124'),('12GiB','12 GiB'),('MemoryHigh10GiB','MemoryHigh 10 GiB'),('Cargo jobs1','Cargo jobs=1'),('medidos336','medidos 336'),('antes328','antes 328'),('permanece32','permanece 32'),('é+8','é +8'),('SHA-2567fa','SHA-256 7fa'),('md):124','md): 124')]:t=t.replace(a,z)
p.write_text(t)
manifest=''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='SHA256SUMS');(out/'SHA256SUMS').write_text(manifest);print(json.dumps(dict(tests=suite['passed'],v16=124,peak_bytes=combined_peak,manifest_sha256=sha(out/'SHA256SUMS'))))
