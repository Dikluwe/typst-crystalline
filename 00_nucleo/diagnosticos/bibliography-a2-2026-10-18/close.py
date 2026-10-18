from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,difflib,re,collections,tarfile
r=Path.cwd();p=Path(__file__).resolve().parent;now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 h=hashlib.sha256()
 with Path(f).open('rb') as stream:
  for b in iter(lambda:stream.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def load(name):return json.loads(gzip.decompress((p/name).read_bytes()))
before=load('before-sources.json.gz');freeze=load('complete-freeze.json.gz');summary=json.loads((p/'complete-summary.json').read_text());diff=load('lint-diff.json.gz');geometry=load('geometry-controls.json.gz');audit=load('audit.json.gz');custom=load('custom-csl-audit.json.gz')
assert summary['passed']==8986 and summary['failed']==0 and summary['ignored']==7
assert all(sha(f)==h for f,h in freeze['sources'].items())
changed={f for f,s in before['files'].items() if Path(f).read_text()!=s}
assert changed=={'01_core/src/compiler/layout/convergence.rs','00_nucleo/prompts/compiler/layout/convergence.md'},changed
resources=[json.loads(f.read_text()) for f in p.glob('*-resources.json') if not f.name.startswith('close')];peak=max(int(x['memory_after']['memory.peak']) for x in resources)
for item in resources:
 assert item['memory_after']['memory.max']=='12884901888' and item['memory_after']['memory.high']=='8589934592' and item['memory_after']['memory.swap.max']=='0'
 assert all(int(line.split()[1])==0 for line in item['memory_after']['memory.events'].splitlines())
assert diff['before']=={'V16:error':106,'V19:note':1120,'V20:note':876}
assert diff['after']=={'V16:error':105,'V19:note':1120,'V20:note':876} and not diff['added']
assert len(diff['removed'])==1 and diff['removed'][0]['file'].endswith('layout/convergence.rs')
proof_cf=json.loads((p/'counterfactual-proof.json').read_text());assert sha(r/'target/debug/typst')==summary['binary_sha256']==proof_cf['candidate_binary_sha256']
t1=r/'00_nucleo/materialization/textual-realization-2026-10-18';t1manifest=json.loads((t1/'manifest.json').read_text());assert all(sha(r/f['path'])==f['sha256'] for f in t1manifest['files'])
t1owners=json.loads((t1/'baseline-owners.json').read_text());assert all(sha(r/f['path'])==f['sha256'] for f in t1owners['owners'])
with tarfile.open(p/'before-artifacts.tar.gz') as tar:assert tar.extractfile('outputs/audit.bib').read()==(p/'outputs/audit.bib').read_bytes()
patch=''.join(''.join(difflib.unified_diff(before['files'][f].splitlines(True),Path(f).read_text().splitlines(True),fromfile='before/'+f,tofile='after/'+f)) for f in sorted(changed));(p/'change.patch.gz').write_bytes(gzip.compress(patch.encode()))
proof={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'changed_from_before':sorted(changed),'changed_hashes':{f:sha(f) for f in changed},'validated_sources_count':len(freeze['sources']),'unaffected_sources_and_L0':len(before['files'])-len(changed),'candidate_binary_sha256':summary['binary_sha256'],'oracle_binary_sha256':sha(r/'lab/typst-original/target/release/typst'),'linter_binary_sha256':sha('/tmp/tekt-linter-none-target/debug/crystalline-lint'),'bib_fixture_sha256':sha(p/'outputs/audit.bib'),'custom_csl_fixture_sha256':sha(p/'outputs/custom.csl'),'memory_peak_bytes':peak,'memory_peak_GiB':peak/1024**3,'resource_stages':resources,'pending_R3_T1_manifest_sha256':sha(t1/'manifest.json'),'pending_R3_T1_files_and_owner_bases_unchanged':True,'goal_complete':False,'scope':'Only A2-CSL cache selection repaired; geometry/multi/alias style remain open.'}
(p/'final-state.json.gz').write_bytes(gzip.compress(json.dumps(proof,ensure_ascii=False,indent=2).encode()))
report=f'''# A2-CSL — preservação do estilo bibliográfico

A subetapa corrigiu a perda de MLA/IEEE e de CSL customizado sob receitas e
carriers. Bibliografias substituídas ou ocultadas continuam fornecendo o estilo
correto às citações. **E05/A2 e o objetivo geral continuam abertos**: recuos,
separação de blocos, múltiplas bibliografias e estilo de alias ainda têm diferenças.

## O que mudou

Somente [convergence.rs]({r}/01_core/src/compiler/layout/convergence.rs) e seu
[L0]({r}/00_nucleo/prompts/compiler/layout/convergence.md) mudaram em relação ao
freeze inicial desta etapa. O L0 foi atualizado antes dos testes/código.

A descoberta do primeiro estilo agora prioriza os snapshots de Bibliography
registrados no índice semântico da tentativa, conservando ordem, locale e a
style_key do CSL resolvido. Isso mantém a declaração efetiva mesmo depois de um
show trocar seu corpo por texto ou vazio. Sem snapshot, a entrada de layout cru
usa o visitante estático existente, por empréstimo, em vez de outro catálogo
parcial de containers. Realized.original não é ressuscitado nesse fallback;
Document/Asset continuam fronteiras. A primeira declaração sem style não é
saltada em favor de outra. A interpretação CSL e a política de fallback não
mudaram. Nenhuma API pública, fase, flag ou supressão de lint foi acrescentada.

## Evidência e validação

- RED funcional antes de implementar: cinco dos sete testes novos falharam;
  os dois controles restantes passaram. Depois, os sete ficaram verdes.
- Oito cenários PDF que perdiam CSL passaram a conservar o texto do vanilla:
  show-set bare/where, override de par, identidade, substituição, ocultação,
  Block e IEEE sob show-set. Os 16 pares de sondas continuam individualizados;
  a entrada multiple permanece divergente e block-hidden rejeita nos dois.
  Texto igual não é prova de geometria/morfologia geral.
- Cinco controles adicionais de texto com CSL customizado conferiram com o
  oracle. O sexto, com título padrão, continua **RED de layout**. O primeiro
  auditor estrito detectou-o, e seu resultado foi preservado; não conta como
  paridade aprovada. A reprodução exata anterior confirmou a mesma extração e
  as mesmas posições. [Contraprova]({p}/counterfactual-proof.json).
- Cores reais no PDF: BIBWORD recebeu `#ff4136` sob show-set bare/where, enquanto
  OUTSIDE ficou `#000000`, conforme o vanilla. Seis controles de geometria
  anteriores conservaram todas as caixas de palavras/páginas com delta máximo
  **0 pt**. Os 44 controles textuais R3 mantiveram resultados e primeiro erro.
- **{summary['passed']} testes workspace passaram, zero falhas, {summary['ignored']}
  ignorados; 5 doctests passaram.** Build, Clippy all-targets com `-D warnings`
  e `-D unfulfilled_lint_expectations`, Rustdoc `-D warnings`, formato, linhagem
  V1/V5/V15/V26, reversa (987 consumers) e diff-check passaram. Os quatro
  índices Rustdoc são typst_core, typst_shell, typst_infra e **typst**.
- **V16 106→105; V19 1120 e V20 876 inalterados.** O diferencial remove somente
  o wildcard neutro de convergence; não há erro, warning ou nota nova. O
  gate global continua vermelho pelos 105 V16 restantes.

Fontes/L0 ficaram congelados durante a validação. A contraprova posterior
restaurou temporariamente somente o par source/L0 inicial, reproduziu o
executável anterior e depois restaurou o candidato. Todos os hashes produtivos
foram conferidos e o executável restaurado é byte a byte o validado. Isso
permite conservar as suítes já passadas, sem fingir que o RED visual desapareceu.

Duas falhas do harness estão preservadas: a primeira fixture chamou `new` em
vez de `TagIntrospector::empty` e não contou como RED funcional; depois o
verificador documental esperou `typst_wiring/index.html`, embora o target bin
se chame typst. Rustdoc havia passado. A checagem corrigida consultou os alvos
Cargo e concluiu os gates. A tentativa estrita dos seis controles customizados
e sua classificação posterior também estão em final-checks-attempt1.json.gz.

## Limites medidos que seguem para o próximo lote

Com fonte 12 pt/margem 8 pt, continuações da MLA ficam em x=8 no Tekt,
x=26 no vanilla; o override `hanging-indent:3em` conserva x=8 no Tekt e usa
x=44 no vanilla. A primeira entrada também começa na linha da citação no Tekt
(y=5.168), enquanto o vanilla a inicia em y=27.463995. O caso custom-default-title
intercala visualmente partes da citação e da lista: permaneceu idêntico na
contraprova anterior/candidata, e ainda precisa de correção.

Multiple ainda duplica a lista MLA no Tekt. Alias-style já usava CSL, mas ignora
uma cor global posterior: BIBWORD fica preto antes/depois, contra vermelho no
vanilla. Os PDFs anteriores foram recuperados do arquivo e conferidos por
SHA-256; não é regressão atribuída sem prova. Esse caso integra os refutadores
de inserção/escopo R3-T1. [Medições geométricas/cores]({p}/geometry-controls.json.gz).

Prioridade seguinte: A2-Indent e fronteiras de bloco/título; depois A2-Multi.
R3-T1 depende da aprovação já solicitada; R3-T2/retornos e os demais E01–E20
permanecem no [inventário geral]({r}/00_nucleo/diagnosticos/paridade-novo-vanilla-2026-10-17/inventario.md).
Não houve validação HTML nova nem declaração de paridade completa de bibliografia.

## Recursos e proveniência

Pico dos jobs: **{peak} bytes ({peak/1024**3:.3f} GiB)**, abaixo de
MemoryMax **12 GiB**, MemoryHigh **8 GiB**, swap **0**. Cargo jobs1,
debug0, incremental0 e comandos pesados sequenciais. Eventos high/max/OOM/
OOM-kill zero em todos os jobs. É pico de cgroup, não consumo total da máquina
nem benchmark pareado de ganho de memória.

HEAD `{proof['head']}`, branch `{proof['branch']}`, working tree não commitado.
[before-sources.json.gz]({p}/before-sources.json.gz) preserva fontes/L0, UTC,
status e `git diff HEAD --stat`; complete-freeze registra os {len(freeze['sources'])}
hashes validados. [final-state.json.gz]({p}/final-state.json.gz) registra o
encerramento, os {proof['unaffected_sources_and_L0']} arquivos preservados e o patch
restrito. Comandos e horários individuais estão em complete-results.json e nos
recibos de recursos; os resultados completos estão em before/after.json.gz,
custom-csl-before/results/audit.json.gz, audit.json.gz e geometry-controls.json.gz.
A fixture audit.bib está preservada no arquivo before-artifacts e por SHA-256.

Oracle pinado **bd5ebec285505889e8ff1d4f68804ab147a699f3**, árvore
cac836c07e4dec2a89219370428292393a3216b5; binário
`{proof['oracle_binary_sha256']}`.
Tekt anterior reproduzido: `{proof_cf['before_binary_sha256']}`.
Tekt final: `{proof['candidate_binary_sha256']}`.

A proposta R3-T1 e seus 11 L0s-base continuam intactos, manifesto
`{proof['pending_R3_T1_manifest_sha256']}`. Esta etapa não altera sua aprovação
pendente. Sem commit, push, merge, publicação, release ou encerramento do objetivo.
'''
(p/'relatorio.md').write_text(report)
(p/'proximos-passos.md').write_text(f'''# Continuação do objetivo geral

1. **A2-Indent e blocos:** corrigir hanging-indent e fronteira de entrada da bibliografia, incluindo título padrão/custom-default-title, sem declarar o RED como paridade. Os recuos e coordenadas em geometry-controls.json.gz são a referência medida. Auditar L0s de par/set, bibliography/bib_csl e planejamento/emissão de linhas antes de código; preservar first-line, RTL, negativos, widths e estilos existentes mediante sondas próprias. Isto é passo, não L0 nem autorização de API nova.
2. **A2-Multi:** atribuição de citações e cache por ocorrência de Bibliography; o global único atual duplica listas. Medir target/ordem/fontes antes de decidir transporte ou contrato público.
3. **R3-T1:** aprovação pendente da [revisão]({t1}/review.md), manifesto {proof['pending_R3_T1_manifest_sha256']}. Acrescentar o alias-style bibliográfico aos refutadores de inserção; não sobrescrever candidatos ou pedir a mesma aprovação de novo.
4. **R3-T2:** concluir coerção de retornos de show a partir do diagnóstico show-returns-2026-10-18; coordenar os owners compartilhados com R3-T1 e preservar as diferenças de Raw/Text/None e o caso Symbol ainda aberto.
5. **Escopo integral:** manter E01–E20, frentes públicas RM1/RG/FG/Paint/Symbol e as lacunas explícitas do inventário; 105 V16 continuam bloqueando o gate global. O reparo CSL é progresso, não redefine o objetivo Faça todas as correções.
''')
print(json.dumps({'passed':summary['passed'],'ignored':summary['ignored'],'memory_peak_GiB':peak/1024**3,'V16':diff['after']['V16:error'],'changed':sorted(changed),'report':str(p/'relatorio.md')},ensure_ascii=False))
