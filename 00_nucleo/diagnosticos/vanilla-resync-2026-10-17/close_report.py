from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
root=Path.cwd();out=Path(__file__).resolve().parent
read=lambda n:json.loads((out/n).read_text())
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
receipt=read('oracle-build.json');upstream=read('upstream.json');suite=read('workspace/workspace-result.json');matrix=read('matrix-summary.json');checks=read('validation-results.json');post=read('post-validation-results.json');integrity=read('validation-integrity.json')
resources={p.name:json.loads(p.read_text()) for p in out.glob('*-resources.json')}
for r in resources.values():
 assert r['memory_before']['memory.max']=='12884901888'
 assert r['memory_before']['memory.swap.max']=='0'
peak=max(int(r['memory_after']['memory.peak']) for r in resources.values())
rows=[]
for d in matrix:
 data=read('matrix-'+d['profile'].replace('+','-')+'.json')
 assert data['vanilla_sha256']==receipt['binary_sha256']
 assert data['vanilla_revision']==receipt['pin'][:8]
 assert data['total']==43
 c=d['summary'];rows.append('| '+d['profile']+' | '+' | '.join(str(c[k]) for k in ['MATCH','DIFFERENCE','VIOLATED','DISABLED_BY_PROFILE','UNKNOWN'])+' |')
helper=[]
for n in ['symbols','language-probes']:
 d=json.loads((out/(n+'.log')).read_text());helper.append(f"- {n}: `{json.dumps(d.get('counts',{}),ensure_ascii=False)}`.")
receipt['source_verification_after_build']=read('source-final.json')
receipt['extractor_build']={'resources':resources['inventory-build-resources.json'],'binary_sha256':hashlib.sha256(Path('lab/typst-original/target/release/p1140-inventory').read_bytes()).hexdigest()}
(out/'oracle-build.json').write_text(json.dumps(receipt,indent=2)+'\n')
with gzip.open(out/'oracle-build.json.gz','wt') as f:json.dump(receipt,f,indent=2)
checkrows='\n'.join('| '+x['label']+' | '+str(x['exit'])+' |' for x in checks)
analysis=(out/'matrix-analysis.md').read_text() if (out/'matrix-analysis.md').exists() else ''
report=f'''# Commit do trabalho e re-sync vanilla — 2026-10-17

O trabalho anterior foi consolidado no commit local
`{upstream['checkpoint']}`. A referência de linguagem foi atualizada de
`{upstream['old']}` para
[`{receipt['pin']}`](https://github.com/typst/typst/commit/{receipt['pin']}).
A consulta oficial foi repetida antes do fechamento; HEAD e main continuam nesse pin.
O pedido humano autoriza commit e atualização da referência; não houve push ou release.

## Proveniência e escopo

Estado medido: HEAD `{upstream['checkpoint']}` com working tree não commitado
na atualização. UTC, lista exata de alterações, diffstat, hashes, comandos e
identidade dos executáveis estão em `validation-state.json.gz`,
`workspace/workspace-pre.json.gz` e no pacote `evidence.json.gz`.
`precommit-state.json.gz` registra o estado anterior à consolidação.
Fechamento do relatório: {now()}.

- Upstream oficial: `https://github.com/typst/typst.git`, branch main observada e depois fixada.
- Commit: `{receipt['pin']}`; árvore: `{receipt['upstream_tree']}`.
- Binário: `lab/typst-original/target/release/typst`.
- SHA-256: `{receipt['binary_sha256']}`.
- CLI auxiliar: `{receipt['version_auxiliary']}`.
- `sys.version` medido diretamente em ambos os produtos: `version(0, 15, 1)`.
- `PARITY_VERSION` e os contratos de produção permanecem com os mesmos valores.

O intervalo upstream contém {len(upstream['delta_log'].splitlines())} commits. A cópia
quarentenada foi atualizada por diff Git entre os pins e conferida por hash de blob.
O único arquivo upstream diferente é `Cargo.lock`, por causa do overlay local
preexistente. Os dois arquivos extras pertencem a `p1140-inventory`; a revisão
emitida pelo extrator foi atualizada e seu executável reconstruído. Os hashes do
overlay, receita e lockfile estão no recibo. O binário antigo P1489 foi preservado
localmente em `lab/typst-original/target/release/typst-9f2b6e871-p1489`.
O verificador usado pelo CI aceitou localmente um checkout independente e limpo
do novo commit/árvore. Os workflows remotos não foram executados nesta etapa.

Atualizações: pin e árvore em `tools/ci/build_oracle.py`, checkout do oráculo no CI,
fonte em `lab/typst-original`, referência vigente em CLAUDE/README e documentação
operacional, runner/schema/manifest da matriz, merge e extrator do inventário,
smoke vanilla e SHA das sondas. O builder do oráculo passou a um job.
A mensagem do harness p1293 foi atualizada com L0 primeiro e resselo restrito;
comparadores, esperados e tolerâncias não foram relaxados. Não houve alteração
funcional produtiva no compilador para acompanhar mecanicamente o vanilla.

Medições e propostas históricas conservaram seus pins. O manifesto da matriz
identifica a autoridade nova e mantém explícita a proveniência anterior de cada
caso. Antes de implementar qualquer diferença futura, reauditar o L0 proprietário
com a nova fonte conforme ADR-0107/0108/0127/0129.

## Validação local

Suíte workspace: **{suite['passed']} aprovados, {suite['failed']} falhas,
{suite['ignored']} ignorados**, com fixtures em `/dev/shm`, um job, uma thread de
teste e o oráculo novo verificado. Os hashes de fontes/L0 permaneceram congelados.
Smoke Rust separado do vanilla: 2 aprovados. Testes Python: 16 de CI, 85 da matriz
e 25 do inventário aprovados. Os quatro índices Rustdoc foram confirmados.

| Gate | Exit |
|---|---:|
{checkrows}

O exit 1 de architecture preserva os **106 V16** preexistentes; também foram
observadas 1099 notas V19 e 873 notas V20, sem alteração dessas contagens.
O gate arquitetural global continua bloqueado por esses V16. Linhagem estrita
V1/V5/V15/V26 e verificação reversa passaram; isso não equivale a arquitetura
inteiramente aprovada nem autoriza release.

## Matriz bilateral contra o pin novo

Cada perfil avaliou 43 casos e executou os casos ativos nas duas ordens. Resultados são locais
aos observáveis declarados de cada caso; não constituem percentual de paridade
global. DISABLED_BY_PROFILE não recebe crédito de paridade. Igualdade de bytes
ou da mecânica Rust não é requisito de linguagem (ADR-0107).

| Perfil | MATCH | DIFFERENCE | VIOLATED | DISABLED_BY_PROFILE | UNKNOWN |
|---|---:|---:|---:|---:|---:|
{chr(10).join(rows)}

{analysis}

Sondas adicionais, com o mesmo binário novo:

{chr(10).join(helper)}

Nos 16 TEXT_MATCH_CARRIER_DIFF, o glifo coincide, mas a observação pública
expõe `symbol` no vanilla e `text` no cristalino. É a pendência de morfologia
SY1 já registrada; esses casos não recebem MATCH integral nem são descartados
como igualdade mecânica Rust. O exit 0 do auxiliar não fecha essa diferença.
As 36 extensões são conservadas e não contam como ganho de paridade.

O inventário vanilla foi emitido novamente nos seis perfis, com revisão e hash
do produto verificados. Isso valida a identidade do lado vanilla; a comparação
global de cobertura do projeto continua dependente de inventários bilaterais
frescos, sem transportar percentuais históricos.

## Recursos e limites

Jobs pesados sequenciais, `CARGO_BUILD_JOBS=1`, MemoryHigh=10 GiB,
MemoryMax=12 GiB, MemorySwapMax=0. Pico medido dos cgroups desses jobs:
**{peak} bytes ({peak/1024**3:.3f} GiB)**. Não houve OOM nem atingimento de
MemoryMax; a validação acionou o controle MemoryHigh, registrado nos contadores.
Esse pico inclui o uso contabilizado pelo cgroup, não somente RSS de rustc.
Recursos completos por comando estão no pacote de evidências.

## Pendências e entrega

O delta upstream inclui igualdade com LooselySpanned, bibliografia com
hanging-indent, rastros de erro, correção de densidade de imagens e ajustes de
HTML/PDF/fontes. O diff de fonte os identifica como candidatos a sondas próprias;
não prova automaticamente divergência nem ganho de paridade no cristalino.
As frentes de paridade e os 106 V16 já abertos continuam nas etapas próprias.
O objetivo amplo permanece em pausa após esta entrega delimitada.

Artefatos: `oracle-build.json.gz` (recibo), `evidence.json.gz` (resultados e logs),
`validation-state.json.gz` (estado e hashes) e `manifest.sha256` (integridade).
'''
(out/'relatorio.md').write_text(report)
# Keep compact text/JSON evidence; generated images and executables remain local.
texts={}
for p in sorted(out.rglob('*')):
 if p.is_file() and ('-artifacts' not in str(p.relative_to(out))) and p.suffix in ['.json','.log','.patch','.diff']:
  texts[str(p.relative_to(out))]=p.read_text()
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump({'captured_at_utc':now(),'files':texts},f,ensure_ascii=False)
state={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'sources_unchanged_after_all_checks':all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in json.load(gzip.open(out/'validation-state.json.gz'))['source_hashes'].items())}
assert state['sources_unchanged_after_all_checks']
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f,indent=2)
paths=[p for p in out.iterdir() if p.is_file() and p.suffix in ['.md','.py','.gz']]
paths+=list((out/'workspace').glob('*.gz'))
manifest=''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(root))+'\n' for p in sorted(paths))
(out/'manifest.sha256').write_text(manifest)
print('Report and evidence sealed:',len(paths),'artifacts; peak GiB',peak/1024**3)
