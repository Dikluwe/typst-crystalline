from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,difflib,re
p=Path(__file__).resolve().parent;r=Path.cwd()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
load=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
base=load('before-sources.json.gz');freeze=load('freeze.json.gz');summary=json.loads((p/'summary.json').read_text());audit=load('audit.json.gz');lint=load('lint-diff.json.gz')
assert summary['passed']==9011 and summary['failed']==0 and summary['ignored']==7
assert audit['generic_fully_matching']==72 and len(audit['cases'])==78 and len(audit['extras'])==16
assert all(sha(f)==h for f,h in freeze['sources'].items())
changed={f for f,s in base['files'].items() if Path(f).read_text()!=s}
expected={f'{root}/{name}.{ext}' for root,ext in [('01_core/src/compiler/layout','rs'),('00_nucleo/prompts/compiler/layout','md')] for name in ['block','heading']}
assert changed==expected,changed
added=set(freeze['sources'])-set(base['files']);assert added=={'00_nucleo/prompts/_nuclei/layout/block-margin-priority.toml'}
assert not [x for x in lint['added'] if x['level'] in ('error','warning')]
doc=re.findall(r'^test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed;', (p/'doctest.log').read_text(),re.M);assert sum(int(x[0]) for x in doc)==5 and all(int(x[1])==0 for x in doc)
red=(p/'red3.log').read_text();assert '3 passed; 3 failed;' in red
assert sha('target/debug/typst')==summary['binary_sha256']
resources=[json.loads(f.read_text()) for f in p.glob('*-resources.json') if not f.name.startswith('close')]
peak=max(int(d['memory_after']['memory.peak']) for d in resources)
for d in resources:
 m=d['memory_after'];assert(m['memory.max'],m['memory.high'],m['memory.swap.max'])==('12884901888','8589934592','0');assert all(int(line.split()[1])==0 for line in m['memory.events'].splitlines())
o=p.with_name('bibliography-structure-2026-10-18');old=json.loads((o/'manifest.json').read_text());assert sha(o/'manifest.json')==base['predecessor_manifest_sha256']
for item in old['files']:
 f=item['path']
 if f in base['files']:assert hashlib.sha256(base['files'][f].encode()).hexdigest()==item['sha256'],f
 else:assert sha(f)==item['sha256'],f
patch=''.join(''.join(difflib.unified_diff(base['files'].get(f,'').splitlines(True),Path(f).read_text().splitlines(True),fromfile='before/'+f,tofile='after/'+f)) for f in sorted(changed|added));(p/'change.patch.gz').write_bytes(gzip.compress(patch.encode()))
(p/'after-sources.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'freeze':freeze,'files':{f:Path(f).read_text() for f in freeze['sources']}}).encode()))
proof={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'changed':sorted(changed),'added':sorted(added),'changed_hashes':{f:sha(f) for f in changed|added},'candidate_binary_sha256':sha('target/debug/typst'),'oracle_binary_sha256':sha('lab/typst-original/target/release/typst'),'memory_peak_bytes':peak,'memory_peak_GiB':peak/1024**3,'goal_complete':False,'predecessor_verified':True,'scope':'Relative margin priority, not all bibliography/parity'}
(p/'final-state.json.gz').write_bytes(gzip.compress(json.dumps(proof,indent=2,ensure_ascii=False).encode()))
report=f'''# A2 — prioridade das margens de blocos e títulos

Concluída a correção das margens relativas automáticas versus explícitas.
O objetivo **Faça todas as correções** e a bibliografia inteira continuam abertos.

## Resultado

Margem explícita agora prevalece sobre automática, mesmo menor ou zero.
Margens da mesma classe conservam o máximo. A presença causal de above/below/
spacing determina a classe; não o valor numérico. Heading usa sua margem
explícita no mesmo protocolo. O título da bibliografia e seu primeiro corpo
passam às posições do vanilla nos seis controles de spacing/estilo medidos;
a correção também vale para blocos comuns.

- Com par.spacing2em e Block below0→Block auto, BODY passou de37.064 para
  13.064pt, como o oracle. Below3 passa de37.064 para16.064pt.
- Heading→Block auto deixou de acrescentar15pt indevidos; o corpo está em
  25.004pt. Block auto→Heading deixou de acrescentar2.4pt; título em33.3128pt.
- No controle IEEE2em, título35.7128→33.3128pt e primeiro corpo71.9→54.5pt.
  No controle padrão200pt da etapa anterior, foi removido o excesso de5.4pt
  abaixo do título. O controle regional2em/70pt passou de quatro para três
  páginas, coincidindo com o oracle, sem trocar a estratégia de quebra. Resíduos de métricas mistas não foram escondidos.

## Alterações e limites

Dois consumers existentes e seus dois L0s, mais um núcleo compartilhado.
Block é dono do helper puro privado collapse_margin; Heading o chama.
Não mudaram campos/assinaturas públicos, modos, flags, fases ou representação
pública. Não houve offset especial de bibliografia, supressão de lint ou
alteração de estratégias de quebra. Checkpoints e o protocolo de saída já
transportavam a classe; nenhum novo slot foi necessária.

L0s foram atualizados após medir, antes dos testes e da implementação.
A ADR0127 permite fluxo contínuo para esta correção interna de paridade.
O primeiro preparo de L0 tinha pin e ordenação incorretos, detectados pelo
V26 depois da escrita do patch: faltou executar esse gate antes da implementação.
O grafo foi corrigido antes de --fix-hashes, e a materialização final passou
no gate e na revalidação. A primeira tentativa não é apresentada como válida. A prova fonte→binário não depende de versão.
As três fontes vanilla consultadas foram comparadas byte a byte ao objeto
Git pinado; hashes em oracle-sources.json.

## Validação

**{summary['passed']} testes passaram, zero falhas, {summary['ignored']} ignorados;
cinco doctests.** Os seis novos testes incluem três REDs funcionais antes do
código e três controles que já passavam; depois todos passaram. A compilação
inicial da fixture exigiu f64 e uma premissa de fonte12 foi corrigida para
chain11, antes do RED causal. Essas tentativas não são prova de defeito de
produção; snapshots e logs estão preservados em notas-fixtures.md.

Os72 casos genéricos coincidem em texto, páginas, caixas e alturas com o oracle
(tolerância numérica0.001pt de extração). Outros seis de bibliografia verificam
título e primeiro corpo, sem certificar linhas posteriores ou hanging.
Os16 controles extras incluem nested/Box, regiões70/150pt e títulos com
simple/optimized e IEEE/MLA. Nos14 que não são MLA, a atribuição de página e a geometria passaram
pelo critério causal registrado em audit-method.md. Os dois MLA continuam falhas de paridade: a primeira auditoria
parou em Testing, já numa linha errada por falta do recuo francês. Remover 5.4pt
da origem melhora o início do corpo, mas aumenta a distância y absoluta dessa
palavra mal quebrada. A classificação posterior exige origem inicial coincidente
e translação única sem mudar nenhuma posição relativa, palavra ou página
interna. Isso foi provado, não é fechamento de hanging nem melhora universal
por palavra. first-audit.py/log e mla-residual-classification.json preservam
a falha e a análise. Nenhum código produtivo foi modificado após essa coleta.

A primeira suíte ampla teve40 falhas de harness em cinco targets:34 exigiam
TMPDIR explícito e seis exigiam TYPST_VANILLA. Faltaram essas entradas no comando
de validação; não foram alteradas expectativas. As falhas observadas ocorreram
antes dos asserts do produto afetados.
Os cinco targets foram repetidos com oracle verificado e TMPDIR em /dev/shm:
46 testes passaram. O total acima substitui os resultados desses targets,
reaproveitando os outros8965 testes aprovados sobre os mesmos hashes congelados,
sem dupla contagem. workspace.log preserva a rodada inicial;
workspace-retry.log, harness-config.json e finish-results.json guardam a correção.

Build workspace, Clippy all-targets com warnings/expectativas negados, Rustdoc
com warnings negados, fmt, V1/V5/V15/V26, reversa988 e diff-check passaram.
Os{len(freeze['sources'])} arquivos produtivos/L0 foram congelados durante toda
a validação. O manifesto predecessor foi verificado contra artefatos imutáveis
e, para fontes alteradas, contra os bytes iniciais deste lote.

| Tekt | Antes | Depois |
| --- | ---: | ---: |
| V16 erro | {lint['before'].get('V16:error',0)} | {lint['after'].get('V16:error',0)} |
| V19 nota | {lint['before'].get('V19:note',0)} | {lint['after'].get('V19:note',0)} |
| V20 nota | {lint['before'].get('V20:note',0)} | {lint['after'].get('V20:note',0)} |

No binário do pin do CI, nenhum erro/warning novo:105V16 anteriores continuam
bloqueando o gate. Uma execução pelo PATH global usou outro binário e apontou
114V16. Os nove achados adicionais estão em consumers idênticos ao estado
inicial deste lote; não são introduções do patch. Isso não os descarta: a
lista com L0s e hashes está em global-linter-findings.json, para investigação
própria. A revisão-fonte do executável global não foi provada. O diferencial
final foi repetido explicitamente com o binário pinado já verificado na etapa
anterior, sem alterar o pin do CI ou suprimir os achados globais. Zero violations
global não foi alcançado nem alegado.

## Pendências

O recuo francês (hanging-indent) continua sem aplicação física, assim como
seus overrides causais em par/set. Persistem diferenças de métricas mistas,
continuação, múltiplas bibliografias e clusters de citações. Não é prova de
paridade universal da bibliografia. R3-T1 permanece aguardando a aprovação
já solicitada. Os demais E01–E20, contratos públicos e V16 permanecem no escopo.
A ordem está em [proximos-passos.md]({p}/proximos-passos.md).

## Recursos e proveniência

Pico dos jobs limitados: **{peak} bytes ({peak/1024**3:.3f} GiB)**.
MemoryMax12GiB/High8GiB/swap0/jobs1/debug0/incremental0; nenhum evento
high/max/OOM/OOM-kill. Não é uso total da máquina ou benchmark comparado.
Comandos pesados sequenciais; recursos/UTC e comandos em *-resources.json e
results.json. A medição inicial usa o binário congelado bin/typst-before.

HEAD `{proof['head']}`, branch `{proof['branch']}`, working tree não commitado.
before-sources.json.gz e after-sources.json.gz guardam todos os fontes/L0s;
status/diffstat/UTC, freeze.json.gz, change.patch.gz e final-state.json.gz
registram a árvore exata. Fontes/argv, textos e caixas antes/depois constam dos
JSONs gzip; PDFs anteriores foram arquivados antes da nova coleta.

Oracle bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário
`{proof['oracle_binary_sha256']}`. Tekt final
`{proof['candidate_binary_sha256']}`.
Sem commit, push, merge, publicação ou release.
'''
# Keep the prose readable without changing any measured value.
for a,b in {'par.spacing2em':'par.spacing 2em','de37.064':'de 37.064','para16.064':'para 16.064','acrescentar15pt':'acrescentar 15pt','acrescentar2.4pt':'acrescentar 2.4pt','em33.3128':'em 33.3128','IEEE2em':'IEEE 2em','título35.7128':'título 35.7128','corpo71.9':'corpo 71.9','padrão200pt':'padrão 200pt','de5.4pt':'de 5.4pt','fonte12':'fonte 12','teve40':'teve 40','targets:34':'targets: 34','outros8965':'outros 8965','chain11':'chain 11','Os72':'Os 72','numérica0.001pt':'numérica 0.001pt','Os16':'Os 16','regiões70/150pt':'regiões 70/150pt','Nos14':'Nos 14','regional2em':'regional 2em','reversa988':'reversa 988','Os'+str(len(freeze['sources'])):'Os '+str(len(freeze['sources'])),'pelos105V16':'pelos 105 V16','novo:105V16':'novo: 105 V16','114V16':'114 V16','MemoryMax12GiB/High8GiB/swap0/jobs1/debug0/incremental0':'MemoryMax 12 GiB/High 8 GiB/swap 0/jobs 1/debug 0/incremental 0'}.items():report=report.replace(a,b)
(p/'relatorio.md').write_text(report)
print(json.dumps({'report':str(p/'relatorio.md'),'passed':summary['passed'],'peak_GiB':peak/1024**3,'lint':lint['after'],'goal_complete':False}))
