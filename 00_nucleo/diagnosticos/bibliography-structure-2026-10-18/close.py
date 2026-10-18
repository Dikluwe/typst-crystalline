from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,difflib,re
p=Path(__file__).resolve().parent;r=Path.cwd();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()
def load(f,folder=p):return json.loads(gzip.decompress((folder/f).read_bytes()))
base=load('before-sources.json.gz');freeze=load('final-freeze.json.gz');summary=json.loads((p/'complete-summary.json').read_text());audit=load('audit.json.gz');lint=load('lint-diff.json.gz')
assert summary['passed']==9005 and summary['failed']==0 and summary['ignored']==7,summary
assert all(sha(f)==h for f,h in freeze['sources'].items())
equivalence=json.loads((p/'production-equivalence.json').read_text());assert equivalence['production_unchanged'] and len(equivalence['changed_after_broad'])==4
roles=json.loads((p/'prescription.json').read_text())['owners'];expected=set()
for owner in roles:
 if owner=='compiler/layout/bib_csl/plan.md':continue
 expected.add('00_nucleo/prompts/'+owner);consumer='mod.rs' if owner=='compiler/layout.md' else owner.removeprefix('compiler/layout/').removesuffix('.md')+'.rs';expected.add('01_core/src/compiler/layout/'+consumer)
changed={f for f,s in base['files'].items() if Path(f).read_text()!=s};assert changed==expected,changed
added=set(freeze['sources'])-set(base['files']);assert added=={'01_core/src/compiler/layout/bib_csl/plan.rs','00_nucleo/prompts/compiler/layout/bib_csl/plan.md','00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml','00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml','00_nucleo/prompts/_nuclei/layout/block-inline-region.toml'},added
assert not [x for x in lint['added'] if x['level'] in ('error','warning')]
doc=re.findall(r'^test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed;', (p/'complete-doctest.log').read_text(),re.M);assert sum(int(x[0]) for x in doc)==5 and all(int(x[1])==0 for x in doc)
resources=[json.loads(f.read_text()) for f in p.glob('*-resources.json') if not f.name.startswith('close')];peak=max(int(d['memory_after']['memory.peak']) for d in resources)
for d in resources:
 m=d['memory_after'];assert(m['memory.max'],m['memory.high'],m['memory.swap.max'])==('12884901888','8589934592','0');assert all(int(line.split()[1])==0 for line in m['memory.events'].splitlines())
o=p.with_name('bibliography-indent-2026-10-18');old=json.loads((o/'manifest.json').read_text())
for item in old['files']:
 path=item['path']
 if path in base['files']:assert hashlib.sha256(base['files'][path].encode()).hexdigest()==item['sha256'],path
 else:assert sha(path)==item['sha256'],path
proposal=r/'00_nucleo/materialization/textual-realization-2026-10-18';manifest=json.loads((proposal/'manifest.json').read_text());assert all(sha(x['path'])==x['sha256'] for x in manifest['files']);overlay=json.loads((o/'r3-t1-overlay.json').read_text());assert sha(proposal/'manifest.json')==overlay['proposal_manifest_sha256'];assert sha(overlay['owner_bases_changed'][0])==overlay['current_eval_tests_L0_sha256']
assert sha('target/debug/typst')==summary['binary_sha256'];assert sha('/tmp/tekt-linter-none-target/debug/crystalline-lint')==load('final-state.json.gz',o)['linter_binary_sha256']
patch=''.join(''.join(difflib.unified_diff(base['files'].get(f,'').splitlines(True),Path(f).read_text().splitlines(True),fromfile='before/'+f,tofile='after/'+f)) for f in sorted(changed|added));(p/'change.patch.gz').write_bytes(gzip.compress(patch.encode()))
(p/'after-sources.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'freeze':freeze,'files':{f:Path(f).read_text() for f in freeze['sources']}}).encode()))
proof={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'changed_from_before':sorted(changed),'added':sorted(added),'changed_hashes':{f:sha(f) for f in changed|added},'validated_sources_count':len(freeze['sources']),'unchanged_baseline_sources':len(base['files'])-len(changed),'before_binary_sha256':sha(p/'bin/typst-before'),'grid_binary_sha256':sha(p/'bin/typst-grid'),'block_before_binary_sha256':sha(p/'bin/typst-block-before'),'admission_before_binary_sha256':sha(p/'bin/typst-admission-before'),'candidate_binary_sha256':summary['binary_sha256'],'oracle_binary_sha256':sha('lab/typst-original/target/release/typst'),'linter_binary_sha256':sha('/tmp/tekt-linter-none-target/debug/crystalline-lint'),'memory_peak_bytes':peak,'memory_peak_GiB':peak/1024**3,'resource_stages':resources,'pending_R3_T1_overlay':overlay,'goal_complete':False,'scope':'A2 structure/columns/paragraph admission/continuation; hanging, multiple bibliographies and full objective remain open'}
(p/'final-state.json.gz').write_bytes(gzip.compress(json.dumps(proof,indent=2,ensure_ascii=False).encode()))
report=f'''# A2 — estrutura CSL, colunas e continuação

Concluída esta subetapa; **a bibliografia e o objetivo geral continuam abertos**.
Prefixo e corpo ficam em colunas estáveis, o espaçamento entre entradas obedece
a par.spacing e a continuação conserva o recuo. O prefixo acompanha a página
real de admissão do corpo. A pontuação adjacente permanece unida na quebra.

## Implementação

- Um plano privado distingue Columns/Hanging/Plain e preserva prefixo, corpo e
  fronteiras de cada entrada. É preparado junto do cache e compartilhado por
  Arc nas tentativas/checkpoints. BibRenderCache e as duas fachadas públicas
  mantêm seus campos, assinaturas e Content legado.
- A renderização usa Blocks quebráveis e prefixos Place privados, emitidos na
  primeira folha admitida, após widow/orphan. A recusa de preparar a sequência
  exterior não se propaga ao parágrafo próprio de Block. Simple e optimized
  conservam suas escolhas; o corpo não é renderizado novamente para antecipar
  uma página, nem há Location/Grid semântico sintetizado.
- Block conserva sua geometria horizontal nas regiões seguintes e restaura a
  geometria atual do pai ao sair. Subframes suspendem/restauram os contextos;
  checkpoints conservam plano, prefixo pendente e região. Não houve API pública,
  flag ou fase nova. L0s precederam as alterações; ownership e núcleos passaram.
- {len(changed)} arquivos existentes alterados no lote (dez fontes e seus dez
  L0s), mais o novo módulo/owner e três núcleos. Os demais
  {proof['unchanged_baseline_sources']} arquivos-base produtivos/L0 foram preservados.

## Medições e controles

IEEE12: o corpo passou de x25.124 para x29.924; IEEE18, de x33.686 para x40.886,
como o oracle. O gutter é 0.65em. par.spacing 2em muda a distância entre entradas
em 24pt comparado a zero; cores de show-set bare/where e OUTSIDE foram preservadas.
A lista sem prefixo passa a ter um Block por entrada, com margens de parágrafo.
A continuação IEEE90 mantém x29.924, mas ainda tem y5.012 contra 5.167996;
a margem abaixo do título também permanece pendente. Esses valores são
[medições residuais]({p}/geometry-summary.json.gz), não tolerâncias ocultas.

O primeiro candidato Grid passou nos testes locais de coluna, mas a medição
IEEE90pt revelou três páginas contra duas. Ele foi rejeitado; seu binário,
PDFs e fontes exatas foram arquivados. A primeira revisão Block/Place corrigiu
a contagem mas deixou a continuação em x8 e podia deixar o prefixo na página
anterior ao corpo. Esses REDs foram reproduzidos e corrigidos: a continuação
agora usa x29.924 e o prefixo acompanha a admissão. Os seis controles de
paginação 90/150/250pt mantêm conteúdo e contagem do oracle.

A primeira auditoria completa também detectou 2020 no fim da linha e ponto
isolado no caso 18pt. O controle não foi relaxado: a inspeção confirmou adjacência
no Content e supressão herdada do preparo. O corpo de Block passou a abrir seu
escopo próprio. A fixture inicial 54pt não cruzava a fronteira; o probe público
calibrou 52pt, obteve RED funcional e depois GREEN. Uma fixture anterior de
paginação também foi corrigida: exigir sempre a primeira página desrespeitaria
widow/orphan; o contrato válido é prefixo acompanhar a primeira linha real.
Houve um import ausente na fixture de inset, corrigido antes de contar RED.
Todas as tentativas estão preservadas e não contam como provas que não foram.

[Auditoria bilateral]({p}/audit.json.gz),
[primeira tentativa]({p}/attempt1-artifacts.tar.gz),
[fontes Grid verificadas]({p}/grid-sources.json.gz),
[proveniência do probe]({p}/block-adjacent-probe.json).

## Validação final

**{summary['passed']} testes passaram, zero falhas, {summary['ignored']} ignorados;
cinco doctests.** Build workspace, Clippy all-targets com warnings e expectativas
não cumpridas negados, Rustdoc com warnings negados, quatro índices (incluindo
**typst**), fmt, V1/V5/V15/V26, reversa 988 e diff-check passaram.
Os {len(freeze['sources'])} hashes ficaram congelados em cada rodada. Depois da
suíte ampla, o Tekt apontou um V17 numa fixture. Foram alterados somente dois
módulos de teste e seus dois L0s: o predicado foi simplificado, as identidades
foram verificadas e a projeção Plain/Hanging ganhou cobertura explícita.
Os corpos produtivos dos dois arquivos foram comparados byte a byte, omitindo
somente o comentário de prompt-hash; todo o restante produtivo ficou idêntico.
Os 26 testes focados, Clippy e gates finais foram revalidados. A suíte ampla e
os PDFs permanecem provas do mesmo código produtivo, sem uma repetição sem
mudança de produto. [Comparação]({p}/production-equivalence.json).
As duas notas V19 novas são intencionais e prescritas: projeção pública comum
a Columns/Hanging/Plain e renderer temporariamente comum a Hanging/Plain.
Conservamos as notas e verificamos as alternativas; nenhuma supressão foi criada.

Além dos 25 controles anteriores deste lote (agora 26 com pontuação), foram
repetidos 12 casos de estrutura, 6 de paginação, 3 extras, 60 do corpus anterior,
44 textuais, 6 CSL customizados e 3 de cores. O relatório de cada caso distingue
texto/lista, geometria e divergências anteriores; igualdade de extração não
certifica toda a linguagem.

| Tekt | Antes | Depois |
| --- | ---: | ---: |
| V16 erro | {lint['before'].get('V16:error',0)} | {lint['after'].get('V16:error',0)} |
| V19 nota | {lint['before'].get('V19:note',0)} | {lint['after'].get('V19:note',0)} |
| V20 nota | {lint['before'].get('V20:note',0)} | {lint['after'].get('V20:note',0)} |

Nenhum erro/warning novo; o gate global continua bloqueado pelos V16 restantes.
O diferencial completo registra qualquer nota nova sem supressão.

## Pendências e próxima ordem

1. Aplicação física de hanging/default/overrides por ocorrência e parágrafos
   genéricos, preservando estratégias e direções. O plano já conserva o modo,
   mas Hanging ainda compartilha o ramo de Blocks simples; não foi silenciado
   o aviso de suporte incompleto em set par.
2. Margem entre título e corpo e diferenças verticais residuais medidas nos
   JSONs; não declarar paridade geométrica universal.
3. A2-Multi e clusters adjacentes de citações (MLA Silva/Doe ainda diverge).
4. A inspeção visual RTL refutou o suposto defeito de colchetes: ambos mostram
   [1]/[2] da mesma forma; a diferença de pdftotext é mecânica neste controle.
   Há outra diferença visível na posição do ponto final, já presente no Tekt
   anterior. Ela continua aguardando classificação pelo contrato de direção,
   sem inferir intenção ou copiar um possível defeito upstream.
   [Evidência e classificação]({p}/rtl-classification.json).
5. Alias-style ainda depende da realização/inserção R3-T1, cuja aprovação
   continua pendente. A proposta selada e suas bases mantiveram o estado
   registrado pelo overlay predecessor. E01–E20 e frentes públicas RM1/RG/FG/
   Paint/Symbol permanecem no objetivo integral.

## Recursos e proveniência

Pico dos jobs limitados: **{peak} bytes ({peak/1024**3:.3f} GiB)**;
MemoryMax 12 GiB/High 8 GiB/swap 0/jobs 1/debug 0/incremental 0. Nenhum evento
high/max/OOM/OOM-kill. Não é consumo total da máquina nem benchmark de ganho
pareado com o vanilla. Os comandos pesados foram executados em sequência.

HEAD `{proof['head']}`, branch `{proof['branch']}`, working tree não commitado.
before-sources.json.gz e after-sources.json.gz preservam fontes, L0s, UTC,
status e `git diff HEAD --stat`; change.patch.gz guarda o delta restrito.
complete-results.json/complete-freeze.json.gz preservam a rodada ampla;
final-gates-results.json/final-freeze.json.gz registram a rodada de fixtures
e gates, com sua comparação de produção. Os recibos têm comandos/UTC/recursos;
final-state.json.gz consolida o estado. O build final adicional e o diferencial
de lint com entrada explícita estão em supplement-results.json; o congelamento
final foi conferido novamente após esses comandos.
O manifesto predecessor foi verificado contra seus artefatos e, para arquivos
produtivos agora alterados, contra os bytes iniciais preservados deste lote.

Oracle pinado bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário
`{proof['oracle_binary_sha256']}`. Tekt final
`{proof['candidate_binary_sha256']}`. Pin/binário do linter preservado.
Sem commit, push, merge, publicação ou release.
'''
(p/'relatorio.md').write_text(report)
(p/'proximos-passos.md').write_text(f"""# Continuação do objetivo Faça todas as correções

A2-Structure/colunas/admissão/continuação foi concluído com as provas do relatório.
A2 inteiro, paridade e gate global continuam abertos. Não reduzir o objetivo.

1. **A2-HangingTransport/A2-ParIndent:** o plano Hanging está preservado, mas seu
   renderer ainda é Block simples. Completar default 1.5em e overrides causais,
   par(...) e set par, incluindo 0/negativos/abs+em, simple/optimized/hifenização,
   RTL e regiões. Ler L0s vigentes de par/set/paragraph prepare/select/hyphen/
   emit/plan antes de decidir. Não forçar fallback greedy por HSpace novo.
2. **Margens com forças diferentes:** geometry-summary mede título y33.3128
   coincidente, body y59.9 contra 54.499995. Investigar Block acima automático
   fraco versus below explícito de Heading(9pt) antes de alterar fórmula;
   fonte pinada flow/collect.rs:253–260 atribui força ao spacing. Usar controles
   explícito/auto/zero/nested/heading/body, sem inventar offset5.4.
3. **Baseline nas continuações:** IEEE90 segunda página x29.924 correto, mas
   y5.012 contra5.167996. Medir corpo integralmente itálico vs mistura lexical
   e estilos comuns; nova região usa estilo corrente em region_progress.
   É hipótese a refutar, não decisão nem alegação de causa já provada.
4. **RTL:** colchetes visualmente iguais, diferença pdftotext é mecânica nesta
   sonda. Ponto final em outra borda é diferença visual anterior; classificar
   por contrato/adversariais, sem atribuir intenção ou copiar defeito upstream.
   Evidência em rtl-classification.json, imagens e provenance.
5. **A2-Multi e clusters:** múltiplas bibliografias/cache por ocorrência e
   agrupamento de citações adjacentes continuam divergentes. full/target podem
   exigir contrato público; medir antes de propor e aplicar ADR0127 se necessário.
6. **Escopo integral:** E01–E20, RM1/RG/FG/Paint/Symbol e V16 restantes estão
   abertos. Inventário em paridade-novo-vanilla-2026-10-17/inventario.md.
7. **R3-T1:** aprovação existente pendente, manifesto 47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302.
   Não pedir duplicado nem sobrescrever adendo de eval/tests L0; overlay no
   relatório predecessor bibliography-indent-2026-10-18.

Manter 12 GiB/High 8 GiB/swap 0/jobs 1, L0 antes do código, RED funcional e validação.
Sem publicação autorizada nesta etapa. Provas e fontes do lote estão preservadas.
""")
print(json.dumps({'report':str(p/'relatorio.md'),'passed':summary['passed'],'peak_GiB':peak/1024**3,'changed':len(changed),'added':len(added),'lint':lint['after'],'goal_complete':False},ensure_ascii=False))
