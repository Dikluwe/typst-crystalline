from pathlib import Path
import datetime,gzip,json,hashlib,collections,subprocess,tomllib,re
out=Path(__file__).resolve().parents[1];package=Path('00_nucleo/materialization/text-size-length-2026-10-17');b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());a=json.load(open(out/'audit.json'));assert (a['pairs'],a['divergent'],a['coincident_on_measured_observables'])==(64,45,19);scope=json.load(open(package/'scope.json'));assert all(sha(p)==h for p,h in scope['source_hashes'].items());assert len(scope['owners'])==len(set(scope['owners'].values()))==19
for line in (package/'SHA256SUMS').read_text().splitlines():h,name=line.split('  ',1);assert sha(package/name)==h
nuc=package/'candidates/00_nucleo/prompts/_nuclei/text-size-affine.toml';n=tomllib.loads(nuc.read_text());assert n['tekt']==1 and n['kind']=='nucleus' and len(n['claims'])==3;assert 'Hash do Código' not in nuc.read_text();pin='sha256:'+sha(nuc);cons=[str(p) for p in (package/'candidates').rglob('*.md') if pin in p.read_text()];assert len(cons)==4
resources={p.name:json.load(open(p)) for p in out.glob('*resources.json')};events=lambda x:dict((n,int(v)) for n,v in (s.split() for s in x.splitlines()))
for r in resources.values():
 m=r['memory_after'];assert m['memory.max']=='12884901888' and m['memory.high']=='10737418240' and m['memory.swap.max']=='0';assert events(m['memory.events'])['oom']==events(m['memory.events'])['oom_kill']==0
peak=max(int(r['memory_after']['memory.peak']) for r in resources.values());proposal_hash=sha(package/'SHA256SUMS')
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],branch=b['branch'],working_tree='uncommitted',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=b['hashes'],productive_tree_unchanged=True,public_api_implemented=False,measurements=a,resources=resources,peak_bytes=peak,proposal=str(package),proposal_manifest_sha256=proposal_hash,proposal_status='pending explicit ADR0127 approval',global_v16=125)
with gzip.open(out/'audit-state.json.gz','wt') as f:json.dump(state,f)
(out/'relatorio.md').write_text(f'''# Auditoria — argumentos de text e tamanho relativo

Auditoria concluída; **nenhum código, L0 ou núcleo vigente foi alterado**. A implementação TS1 depende de aprovação pública. O objetivo geral continua aberto, com125 V16 e os demais defeitos registrados. Não houve commit/push, merge ou release.

## Medição e classificação

Foram medidos **64 pares / 128 invocações**:48 casos principais e16 suplementares, incluindo controles e repetições. **45 pares divergem** nos observáveis definidos;19 coincidem nesses observáveis, o que não certifica paridade universal. Critérios: exit/mensagens de erro e hints, páginas e palavras exatos; dimensões de página/xMin/yMin/yMax com tolerância0.002pt. Cor/pintura não foi certificada pelos bboxes. Todos os inputs, argv, stderr e PDFs/XML estão arquivados.

- Constructor não reconhece tamanho posicional: as seis permutações Length/Color/body, body antes do tamanho e alias são rejeitados indevidamente.
- Tamanho relativo perde em:2em vira0pt contra24pt;2em+3pt vira3pt contra27pt; nested18pt→2em vira0pt contra36pt. Set posicional é ignorado; múltiplos sets relativos e show-heading também divergem.
- Nomeado deve impedir a busca posicional daquele tipo; o posicional restante vira body ou argumento excedente, conforme a ordem observada. Body None é válido e está sendo rejeitado; mensagens de body ausente, tipo inválido e sobras divergem. Duplicatas sintáticas já são rejeitadas corretamente pelo parser.
- fill:none é aceito indevidamente; Gradient é rejeitado apesar de aceito pelo pin; Gradient relativo self exige erro/hint próprio. Paint textual é frente separada: não substituir gradiente por cor sólida.
- show text é rejeitado como seletor; isso impede controles relativos nesse caminho. Esse suporte requer seu próprio owner/contrato, além do tamanho.
- A segunda medição **refutou a hipótese de identidade pelos valores declarados**: vanilla considera text(size:2em)[A] igual a text(size:24pt)[A], enquanto Tekt não. content/mod.rs:779–782 do pin compara apenas child entre Styleds. Styled versus conteúdo cru continua distinto nos controles; isso não autoriza apagar todos os wrappers. Igualdade da linguagem não é PartialEq/ObservationEq Rust e precisa de correção específica.

## Proposta pública concreta TS1

[Revisão](../../materialization/{package.name}/review.md), [API](../../materialization/{package.name}/api.md), [escopo](../../materialization/{package.name}/scope.json) e [S1–S16](../../materialization/{package.name}/test-plan.md).

Dois contratos: **Style::Size(Pt) → Size(Length)** e **StyleDelta.size: Option<f64> → Option<Length>**. Preservar abs+em no backing único e dobrar funções afins antes de resolver Pt. TextStyle.size e StyleChain::size mantêm superfícies resolvidas. Migração externa exige Length::pt para valores absolutos. Isso aciona ADR-0127 §2.1/§2.4; não foi implementado antes da confirmação.

O pacote tem19 owners/candidatos e um núcleo TOML proposto com três claims, pinado em quatro L0 candidatos. Nenhum código aponta para núcleo; candidato não é L0 vigente. Corpus não é usado como prova de implementação inexistente. Plano S1–S16 cobre RED, API, transporte, captura, Raw/math, restauração, diagnósticos, memória e gates completos, sem release.

Memória atual medida pelo probe existente: Length16bytes; Option<Length>24; Option<f64>16; Style32; StyleDelta/Styles328. Aumento de payload do campo8bytes; tamanho futuro336bytes é **estimativa**, não medição materializada. Pico e alocações precisam ser medidos após aprovação/implementação.

Há quatro caminhos em comum com a proposta RG1 (apply e layout/tests, código+L0). RG1 e seu manifesto continuam intactos. TS1 não a aprova; a retomada deve rebasear os adapters preservando ambos os contratos e revalidando testes. Publicar proposta não equivale a publicar código/release.

Manifesto TS1: **{proposal_hash}**.

## Proveniência e recursos

HEAD **{b['head']}**, branch Tekt, working tree não commitado. before.json.gz registra status/diffstat e hashes dos2119 arquivos de fonte/L0/configuração do checkpoint; as medições têm UTC, argv e SHA de binários/inputs/fontes. audit-state.json.gz revalida que todos permanecem iguais. Oráculo9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-2567fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Probe sizes.json guarda rustc, rlib, fonte e executável medidos. As medições descrevem o working tree congelado, não somente o HEAD.

Execuções pesadas sequenciais com MemoryMax12GiB, MemoryHigh10GiB, swap zero e Cargo jobs1. Pico desta auditoria **{peak}bytes ({peak/1024**3:.3f}GiB)**, sem OOM/OOM-kill. Este pico não é uma medida da implementação proposta e não deve ser comparado ao da suíte anterior como melhoria de RAM.

A suíte8792/0 falhas/7 ignorados, Clippy, Rustdoc e linhagem da etapa Par/Box continuam evidência da mesma árvore produtiva, conferida por hashes. Não foram apresentados novos testes GREEN de uma API ainda não implementada. Enquanto TS1 aguarda aprovação, correções independentes de V16 podem continuar; igualdade Styled, Paint, show text, Box fracionário, SVG multipágina e RG1 permanecem abertos.
''')
# Normalize numeric spacing only in the generated process report.
p=out/'relatorio.md';s=p.read_text()
for old,new in [('com125','com 125'),(':48',': 48'),('e16','e 16'),(';19','; 19'),('tolerância0.002pt','tolerância de 0.002 pt'),('em:2em','em: 2em'),('vira0pt','vira 0pt'),('vira3pt','vira 3pt'),('contra24','contra 24'),('contra27','contra 27'),('contra36','contra 36'),('tem19','tem 19'),('Length16','Length 16'),('Length>24','Length> 24'),('f64>16','f64> 16'),('Style32','Style 32'),('Styles328','Styles 328'),('campo8','campo 8'),('futuro336','futuro 336'),('dos2119','dos 2119'),('Oráculo9f2','Oráculo 9f2'),('SHA-2567fa','SHA-256 7fa'),('MemoryMax12GiB','MemoryMax 12 GiB'),('MemoryHigh10GiB','MemoryHigh 10 GiB'),('Cargo jobs1','Cargo jobs=1'),('suíte8792','suíte 8792')]:s=s.replace(old,new)
p.write_text(s)
for p in (out/'workers').rglob('*.pyc'):p.unlink()
manifest=''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='SHA256SUMS');(out/'SHA256SUMS').write_text(manifest)
print(json.dumps(dict(pairs=a['pairs'],divergent=a['divergent'],unchanged=True,proposal_manifest_sha256=proposal_hash,audit_manifest_sha256=sha(out/'SHA256SUMS'),peak_bytes=peak)))
