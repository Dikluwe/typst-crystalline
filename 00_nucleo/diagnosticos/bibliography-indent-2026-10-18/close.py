from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,difflib
p=Path(__file__).resolve().parent;r=Path.cwd();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()
def load(f):return json.loads(gzip.decompress((p/f).read_bytes()))
before=load('before-sources.json.gz');freeze=load('complete-freeze.json.gz');summary=json.loads((p/'complete-summary.json').read_text());lint=load('lint-diff.json.gz');audit=load('audit.json.gz')
colors=load('colors-audit.json.gz');assert colors['direct_style_and_outside_preserved'] and colors['alias_style_still_open']
assert summary['passed']==8994 and summary['failed']==0 and summary['ignored']==7,summary
assert all(sha(f)==h for f,h in freeze['sources'].items())
changed={f for f,s in before['files'].items() if Path(f).read_text()!=s}
expected={'01_core/src/compiler/layout/bibliography.rs','01_core/src/compiler/eval/rules/set.rs','01_core/src/compiler/stdlib/structural/par.rs','01_core/src/compiler/eval/tests.rs','00_nucleo/prompts/compiler/layout/bibliography.md','00_nucleo/prompts/compiler/eval/rules/set.md','00_nucleo/prompts/compiler/stdlib/structural/par.md','00_nucleo/prompts/compiler/eval/tests.md'}
assert changed==expected,changed
added=set(freeze['sources'])-set(before['files']);assert added=={'00_nucleo/prompts/_nuclei/paragraph-hanging-validation.toml'}
assert not [x for x in lint['added'] if x['level'] in ('error','warning')]
assert sha('target/debug/typst')==summary['binary_sha256']
resources=[json.loads(f.read_text()) for f in p.glob('*-resources.json') if not f.name.startswith('close')];peak=max(int(x['memory_after']['memory.peak']) for x in resources)
for item in resources:
 assert item['memory_after']['memory.max']=='12884901888' and item['memory_after']['memory.high']=='8589934592' and item['memory_after']['memory.swap.max']=='0'
 assert all(int(line.split()[1])==0 for line in item['memory_after']['memory.events'].splitlines())
t1=r/'00_nucleo/materialization/textual-realization-2026-10-18';m=json.loads((t1/'manifest.json').read_text());assert all(sha(x['path'])==x['sha256'] for x in m['files'])
overlay=json.loads((p/'r3-t1-overlay.json').read_text());assert sha(t1/'manifest.json')==overlay['proposal_manifest_sha256'];assert sha(overlay['owner_bases_changed'][0])==overlay['current_eval_tests_L0_sha256']
old=p.with_name('bibliography-a2-2026-10-18');oldmanifest=json.loads((old/'manifest.json').read_text());assert all(sha(x['path'])==x['sha256'] for x in oldmanifest['files'])
patch=''.join(''.join(difflib.unified_diff(before['files'].get(f,'').splitlines(True),Path(f).read_text().splitlines(True),fromfile='before/'+f,tofile='after/'+f)) for f in sorted(changed|added));(p/'change.patch.gz').write_bytes(gzip.compress(patch.encode()))
proof={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'changed_from_before':sorted(changed),'added':sorted(added),'changed_hashes':{f:sha(f) for f in changed|added},'validated_sources_count':len(freeze['sources']),'unchanged_baseline_sources':len(before['files'])-len(changed),'candidate_binary_sha256':summary['binary_sha256'],'before_binary_sha256':sha(p/'bin/typst-before'),'oracle_binary_sha256':sha('lab/typst-original/target/release/typst'),'linter_binary_sha256':sha('/tmp/tekt-linter-none-target/debug/crystalline-lint'),'memory_peak_bytes':peak,'memory_peak_GiB':peak/1024**3,'resource_stages':resources,'pending_R3_T1_overlay':overlay,'previous_A2_CSL_manifest_unchanged':True,'goal_complete':False,'scope':'A2 block boundaries and hanging-indent type validation; physical hanging/prefix/entry structure/multi remain open'}
(p/'final-state.json.gz').write_bytes(gzip.compress(json.dumps(proof,indent=2,ensure_ascii=False).encode()))
positions=load('position-controls-after.json.gz');positionrows={c['label']:c for c in positions['cases']};assert positionrows['heading-control']['results']['tekt']['stdout']=='[[{"page":1,"x":"0pt","y":"0pt"}]]\n'
# Every reported count below comes from the frozen validation and bilateral evidence.
report=f'''# A2 — fronteiras de bibliografia e tipo de hanging-indent

Esta subetapa foi concluída. **A2 e a paridade geral permanecem abertos.**
O corpo e o título da bibliografia deixam de começar na linha da citação
anterior. Valores inválidos de hanging-indent passam a produzir o erro de tipo
correto, inclusive por alias e quando falta o corpo do constructor.

## Correções

- Bibliografia fecha o segmento anterior com seu estilo causal e delega o corpo
  ao protocolo existente de Block, com dimensões automáticas e inset zero.
  O fallback conserva formato, ordem, prefixos e referências anteriores.
  Corpo vazio sem título não acrescenta margem nem quebra.
- set par identifica a função nativa efetiva, incluindo alias. set e constructor
  validam Length e conservam o span do valor e o hint de unidade do inteiro.
  O constructor valida antes de exigir body. Nenhum contrato Rust público,
  modo/flag ou ordem de realização foi alterado.
- Quatro L0s foram atualizados antes do código; três compartilham o novo núcleo
  paragraph-hanging-validation. Oito arquivos existentes mudaram neste lote
  (quatro fontes e seus quatro L0s), mais o núcleo. O patch relativo está em
  [change.patch.gz]({p}/change.patch.gz).

## Validação

RED funcional preservado: tipos inválidos aceitos, corpo cacheado/fallback
na linha da citação e título fora da margem. A primeira tentativa GREEN
identificou um bypass por alias; a correção da identidade manteve os critérios.
Depois, **15 testes focados passaram**, oito novos mais sete controles CSL.

A validação completa passou: **{summary['passed']} testes, zero falhas,
{summary['ignored']} ignorados; cinco doctests**. Build workspace, Clippy
all-targets estrito, Rustdoc com warnings negados, quatro índices documentais,
fmt, linhagem V1/V5/V15/V26, reversa com 987 consumers e diff-check passaram.
Os {len(freeze['sources'])} hashes produtivos/L0 ficaram congelados na validação.
[Comandos/resultados]({p}/complete-results.json).

Nas 60 sondas bilaterais repetidas, os 42 casos bibliográficos agora separam
citação/título/corpo e mantêm o texto do oracle; os dez negativos antes aceitos
agora rejeitam com a mesma primeira mensagem. Os oito controles de parágrafo
comum conservam a geometria anterior: isso comprova ausência de regressão
nesses controles, **não** a implementação do recuo. Seis controles adicionais
cobrem aliases/precedência de erro. Os seis casos CSL customizados conferem em
texto; o título padrão, antes intercalado, fica na ordem correta. Os 44 controles
textuais R3 mantêm seus resultados e primeiro erro. A checagem adicional de
cores PDF confirma BIBWORD vermelho e OUTSIDE preto sob show-set bare/where;
o alias-style continua preto no Tekt contra vermelho no oracle, como antes.
[Auditoria]({p}/audit.json.gz).

| Tekt | Antes | Depois |
| --- | ---: | ---: |
| V16 (erro) | {lint['before'].get('V16:error',0)} | {lint['after'].get('V16:error',0)} |
| V19 (nota) | {lint['before'].get('V19:note',0)} | {lint['after'].get('V19:note',0)} |
| V20 (nota) | {lint['before'].get('V20:note',0)} | {lint['after'].get('V20:note',0)} |

Os detalhes de entradas acrescentadas/removidas estão em
[lint-diff.json.gz]({p}/lint-diff.json.gz). Nenhum erro ou warning novo;
o gate global continua bloqueado pelos V16 ainda existentes.

## Geometria: ganho e diferença residual

Na MLA sem título, a primeira entrada passou de x41.84/y5.168 para
x8/y27.464; o oracle usa x8/y27.463995. O título padrão foi para
x8/y33.3128, como o oracle. Porém o corpo abaixo desse título fica em
y59.9 contra y54.499995: há aproximadamente 5.4 pt de espaço adicional.
As continuações permanecem em x8 contra x26 (default) ou x44 (override
3em) no vanilla. No IEEE, o corpo após o prefixo começa em x25.124
contra x29.924. Esses valores estão em before/after.json.gz, fonte 12 pt,
margem 8 pt, casos mla-none-base, mla-default-base, mla-none-relative
e ieee-none-base. A correção de fronteira não encerra essas diferenças.

## Limites e próxima ordem

1. **A2-Structure:** preservar hanging_indent, prefixo e corpo do CSL para
   compor entradas e colunas; o corpo ainda é um Content pré-formatado único.
2. **A2-HangingTransport / A2-ParIndent:** aplicar recuos válidos e sua prioridade
   por ocorrência, incluindo fonte relativa, zero, negativos e parágrafos
   comuns. O aviso existente em set continua; valores válidos ainda não têm
   implementação física completa. Não há declaração de paridade geométrica.
3. **A2-Multi:** resolver atribuição/cache de múltiplas bibliografias.
4. **Posições de query:** a divergência não é específica da bibliografia.
   Título de controle retorna x0/y0 no Tekt contra x8/y8 no oracle; o controle
   de block nem é encontrado no Tekt. Não inferir posição causal de um elemento
   a partir da caixa dos seus glifos nem declarar a causa já diagnosticada.
5. R3-T1 continua aguardando a aprovação já solicitada. A proposta selada
   permanece intacta; o L0 dos testes ganhou apenas o adendo deste lote.
   [Overlay de integração]({p}/r3-t1-overlay.json) determina preservar esse
   adendo ao aplicar a proposta aprovada. Os demais dez L0s-base não mudaram.
   E01–E20 e as frentes públicas do inventário geral permanecem no escopo.

## RAM e proveniência

Pico máximo medido dos jobs: **{peak} bytes ({peak/1024**3:.3f} GiB)**.
MemoryMax=12 GiB, High=8 GiB, swap=0, Cargo jobs1, debug0/incremental0;
nenhum evento high/max/OOM/OOM-kill. A validação completa executa os comandos
pesados sequencialmente no mesmo cgroup. Esse pico não é consumo total da
máquina nem prova pareada de ganho de memória.

HEAD `{proof['head']}`, branch `{proof['branch']}`, working tree não commitado.
Fontes iniciais, UTC, status e lista exata de alterações `git diff HEAD --stat`
estão em [before-sources.json.gz]({p}/before-sources.json.gz); os recibos
registram cada comando/hora. [final-state.json.gz]({p}/final-state.json.gz)
registra o estado final, os {proof['unchanged_baseline_sources']} arquivos-base
preservados e o pico. before/after.json.gz preservam fontes, binários, argv,
texto e caixas; os PDFs anteriores foram arquivados antes de sobrescrever.
O relatório e manifesto A2-CSL anterior foram verificados sem alterações.

Vanilla pinado bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário
`{proof['oracle_binary_sha256']}`.
Tekt anterior `{proof['before_binary_sha256']}`;
Tekt validado `{proof['candidate_binary_sha256']}`.
Sem commit, push, merge, publicação ou release.
'''
(p/'relatorio.md').write_text(report)
(p/'proximos-passos.md').write_text('''# Continuação

A subetapa A2-fronteiras/tipos está concluída, mas A2 permanece incompleto.
Seguir na ordem A2-Structure → A2-HangingTransport/A2-ParIndent → A2-Multi,
com L0s e REDs próprios antes do código, medindo precedência/RTL/estratégias.
Manter 12 GiB/swap0/jobs1 e as demais frentes E01–E20. Não encerrar o objetivo.

R3-T1 permanece na aprovação existente; consultar r3-t1-overlay.json antes
 de integrar, sem sobrescrever o L0 atual dos testes com o candidato completo
antigo. A proposta selada e os outros dez owners-base foram preservados.
Posições zeradas de query são divergência mais ampla, não encerrada neste lote.
''')
print(json.dumps({'passed':summary['passed'],'peak_GiB':peak/1024**3,'lint':lint['after'],'scope':proof['scope'],'report':str(p/'relatorio.md')},ensure_ascii=False))
