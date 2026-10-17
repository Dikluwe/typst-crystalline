from pathlib import Path
import collections,datetime,difflib,gzip,hashlib,json,subprocess,re
out=Path(__file__).resolve().parents[1]
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for block in iter(lambda:f.read(1048576),b''):h.update(block)
 return h.hexdigest()
b=json.load(gzip.open(out/'before.json.gz'));v=json.load(open(out/'validation-result.json'));scope=json.load(open(out/'scope.json'));assert v.get('complete') and v['sources_unchanged'];assert all(sha(p)==h for p,h in v['source_hashes'].items());assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head'];changed=[p for p,h in b['hashes'].items() if sha(p)!=h];assert set(changed)==set(scope['sources'])|set(scope['owners'].values())
prior=Path('00_nucleo/diagnosticos/text-styled-equality-2026-10-17');prior_state=json.load(gzip.open(prior/'final-state.json.gz'));assert all(v['source_hashes'][p]==h for p,h in prior_state['source_hashes'].items() if p not in changed)
suite=json.load(open(out/'workspace/workspace-result.json'));assert (suite['exit'],suite['passed'],suite['failed'],suite['ignored'])==(0,8820,0,7)
obs=json.load(open(out/'observables-result.json'));assert obs['pairs']==159 and obs['all_within_backend_stdout_stderr_exit_preserved'];delta=json.load(open(out/'architecture-delta.json'));assert delta['after']['V16:error']==120 and delta['no_new_errors_or_warnings']
assert '4 passed; 0 failed;' in (out/'controls-before-corrected.log').read_text();assert '4 passed; 0 failed;' in (out/'validation-controls-after.log').read_text();assert len(json.load(open(out/'red-probe-result.json'))['results'])==8;assert len(json.load(open(out/'green-probe-result.json'))['results'])==6
integrity=json.load(open(out/'catalogue-integrity-result.json'));assert [r['preserved_arms'] for r in integrity['results']]==[47,71,71]
packages={'text-size-length-2026-10-17':'771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba','table-row-groups-2026-10-17':'a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9'}
for name,h in packages.items():
 p=Path('00_nucleo/materialization')/name;assert sha(p/'SHA256SUMS')==h
 for line in (p/'SHA256SUMS').read_text().splitlines():want,f=line.split('  ',1);assert sha(p/f)==want
resources={str(p.relative_to(out)):json.load(open(p)) for p in out.rglob('*resources.json')};peak=max(int(r['memory_after']['memory.peak']) for r in resources.values());events={n:sum(int(next(l.split()[1] for l in r['memory_after']['memory.events'].splitlines() if l.startswith(n+' '))) for r in resources.values()) for n in ['high','max','oom','oom_kill']}
for r in resources.values():
 m=r['memory_after'];assert (m['memory.max'],m['memory.high'],m['memory.swap.max'])==('12884901888','10737418240','0')
assert events['oom']==events['oom_kill']==0
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],working_tree='uncommitted',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],changed_since_previous=changed,tests={k:suite[k] for k in ['passed','failed','ignored']},observables=obs,architecture=delta,peak_bytes=peak,memory_events=events,resources=resources,public_api_changes=0,global_goal='active: known V16 and independent parity debts remain',proposal_manifests_preserved=packages)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'patch.diff').write_text(''.join(''.join(difflib.unified_diff(b['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in changed))
rows=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];remaining=[r for r in rows if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');groups=collections.defaultdict(list)
for r in remaining:
 loc=r['locations'][0]['physicalLocation'];groups[loc['artifactLocation']['uri']].append(loc['region']['startLine'])
(out/'inventario-v16-restante.md').write_text('# V16 após o lote de catálogos\n\n120 erros em '+str(len(groups))+' consumers. Proveniência: final-state.json.gz e SARIF integral.\n\n| Consumer | Casos | Linhas |\n|---|---:|---|\n'+''.join(f'| `{p}` | {len(ns)} | {", ".join(map(str,ns))} |\n' for p,ns in sorted(groups.items())))
triage=[]
for source in ['01_core/src/compiler/stdlib/figure_image.rs','01_core/src/compiler/stdlib/layout/casting.rs']:
 owner=re.search(r'^//! @prompt (.+)$',Path(source).read_text(),re.M).group(1)
 selected=[r for r in remaining if r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')==source]
 assert len(selected)==2
 triage.append(dict(source=source,owner=owner,source_sha256=sha(source),owner_sha256=sha(owner),diagnostics=selected))
(out/'next-batch-triage.json').write_text(json.dumps(dict(utc=state['utc'],head=b['head'],working_tree='uncommitted',snapshot='final-state.json.gz',status='Triage only; read owner and measure before design/code. No API authorization inferred.',selected=triage,total_v16=4),indent=2)+'\n')
(out/'pendencias.json').write_text(json.dumps(dict(V16=120,RG1='Pending human approval; rebase candidates preserving TS1/equality',diagnostic_gaps=obs['preexisting_diagnostic_gaps'],other_open=prior_state['global_goal'],next='Inspect remaining figure_image/casting V16 with their owners and measured semantics; do not silence fallback or alter public API without review.',preserved_text_debts=['Text Paint Gradient/Tiling and relative:self diagnostic','show text selector','Heading defaults when replaced by show function','Script size/offset parity','Custom-only constructor presence outside equality projection','Fractional Box1fr, multi-page SVG, integer literal minimum']),indent=2)+'\n')
(out/'relatorio.md').write_text(f'''# Relatório — V16 dos catálogos

Etapa concluída: **quatro V16 corrigidos, de 124 para 120**, sem mudança de API, default, fase ou comportamento pretendido. Foram alterados somente dois consumers e seus dois L0 desde o checkpoint de igualdade Styled. Não houve commit/push do Typst, merge ou release.

## Alteração e equilíbrio com V19

`type_fields.rs`: constantes e funções passam por uma única classificação exaustiva de Type. Nove famílias mantêm seus catálogos; as outras 29 ficam explicitamente fora deste catálogo, permitindo a continuação no caller. Um nome de campo continua sendo uma string aberta: desconhecido retorna None.

`collections.rs`: classificação exaustiva de Value precede o lookup de método. Array/Dict/Str/Bytes/Args preservam seus caminhos e as outras 34 variantes continuam para o caller. Bytes conserva integralmente a delegação e seus metadados de span. A tabela estática classifica as cinco famílias Type de coleção; as outras 33 têm ausência local de catálogo. Some(Err) de método reconhecido não vira None.

Os três grupos negativos são coesos por responsabilidade local e estão prescritos nos L0. Não significam ausência universal de membros na linguagem. Não houve supressão de V16/V19 ou mudança de política do linter. Os nomes e corpos de 47 entradas do primeiro owner, 71 da tabela estática e 71 do dispatcher de instância foram conferidos por tokens, preservando strings, callbacks e ABI. Essa prova estrutural complementa os observáveis; não substitui paridade de linguagem. As alternativas negativas não têm todas sondas funcionais isoladas; a coesão foi auditada e a exaustividade foi verificada pelo compilador. As notas V19 permanecem visíveis.

## Evidência e validação

O inventário mede 118 entradas: 115 exercitadas no CLI e três Location no módulo. **159 pares bilaterais foram repetidos após a mudança**, preservando exatamente código de saída, stdout e stderr completos dentro de cada backend. Entre backends, 141 pares coincidem no critério código/stdout/erro principal+hints. Permanecem **18 divergências anteriores de diagnóstico**, detalhadas em observables-result.json e pendencias.json: aspas/nomes de tipos, mensagem de cast e bounds e hint de self. Não foram escondidas nem declaradas corrigidas.

O RED foi estrutural: os quatro padrões antigos compilavam mesmo com variante futura. Depois, os três padrões produtivos novos compilam para os enums atuais e rejeitam a variante futura com E0004. As sondas extraem os padrões reais, apagam somente RHS e usam payloads stub; não modificam o workspace/target Cargo. Os quatro controles funcionais passam antes e depois. A primeira versão de um controle tinha uma premissa inválida: Func::PartialEq compara nomes, não callbacks; o teste foi corrigido para repr/ABI/fn_addr_eq antes da implementação. Essa falha não é alegada como RED do produto. O verificador auxiliar também foi corrigido para ler padrões que começam por string e tolerar vírgulas finais introduzidas pelo rustfmt; não houve mudança produtiva por causa disso. As tentativas estão em tooling-corrections.json.

**Suíte completa: {suite['passed']} aprovados, zero falhas e {suite['ignored']} ignorados.** Build workspace, fmt, Clippy all-targets com -D warnings e -D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices HTML, V1/V5/V15/V26 estrito, reverse lineage e diff check passaram. Linhagem foi resselada somente nos dois owners após validar V15/V26.

SARIF: **{delta['after']}**. Nenhum novo erro/warning; todos os diagnósticos fora dos dois consumers são idênticos ao checkpoint anterior. O gate global continua exit 1 pelos 120 V16 restantes. [Inventário atualizado](inventario-v16-restante.md).

## RAM e proveniência

Todos os comandos pesados foram sequenciais sob **MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero e Cargo jobs=1**. Pico: **{peak} bytes ({peak/1024**3:.3f} GiB)**; eventos agregados: {events}. Sem OOM/OOM-kill. O valor é memória do cgroup, incluindo cache; não é RSS nem prova de otimização.

HEAD **{b['head']}**, working tree não commitado. before/final-state.json.gz registram UTC, status/diffstat e hashes de fontes/L0/config. Logs de comandos, recursos, corpus antes/depois e [patch restrito](patch.diff) permitem reprodução. As alterações anteriores, inclusive TS1 e igualdade Styled, foram preservadas por hash. Manifestos TS1 e RG1 conferidos integralmente; RG1 permanece pendente e não foi materializado.

Oracle: pin **9f2b6e8715237cb086899a42873660fe744622e8**, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter pin **12797073972e375d9ab078cf6750c8b30b71e2b9**, binário SHA-256 {b['linter']['sha256']}. A classe foi correção estrutural interna em fluxo contínuo ADR-0127: L0, testes, implementação e selos.

## Continuidade

O objetivo global continua ativo. [Próximo lote triado](next-batch-triage.json): quatro casos remanescentes de figure_image/casting, medindo fallbacks e contratos antes de decidir. As 18 divergências de diagnóstico ficam em lote próprio. Permanecem também RG1 pendente, Paint textual, seletor show text, defaults de Heading substituído por função, scripts, presença custom-only fora da projeção de igualdade, Box1fr, SVG multipágina e literal inteiro mínimo. Este lote não certifica toda a linguagem.
''')
print(json.dumps(dict(tests=suite['passed'],v16=120,peak_bytes=peak,changed=changed)))
