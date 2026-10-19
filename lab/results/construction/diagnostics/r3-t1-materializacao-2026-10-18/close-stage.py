from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,collections
p=Path(__file__).resolve().parent
root=p.parents[2]
def read(name):
 b=(p/name).read_bytes()
 return json.loads(gzip.decompress(b) if name.endswith('.gz') else b)
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git',*args],cwd=root,text=True)
resource=read('closure-candidate10-resources.json')
assert resource['exit']==0, 'Do not close as validated after a failed check'
freeze=read('candidate10-sources.json.gz')
assert all((root/f).read_text()==s for f,s in freeze['files'].items())
audit=read('audit-candidate10.json')
assert not audit['added_blocking'] and not audit['public_signature_differences']
suite=read('suite-summary-candidate10.json')
assert suite['failed']==0
utc=datetime.datetime.now(datetime.timezone.utc).isoformat()
head=git('rev-parse','HEAD').strip()
peak=int(resource['memory_after']['memory.peak'])/1024**3
previous_peak=int(read('final-candidate9-resources.json')['memory_after']['memory.peak'])/1024**3
before=read('before-sources.json.gz')['files']
scope=[]
for f in audit['changed_files']:
 scope.append(dict(path=f,kind='Rust' if f.endswith('.rs') else 'Nucleo' if f.endswith('.toml') else 'L0',before_sha256=hashlib.sha256(before[f].encode()).hexdigest() if f in before else None,after_sha256=sha(root/f)))
(p/'escopo-etapa.json').write_text(json.dumps(dict(utc=utc,head=head,baseline='before-sources.json.gz',current='candidate10-sources.json.gz',files=scope),ensure_ascii=False,indent=2)+'\n')
counts=collections.Counter(row['kind'] for row in scope)
contexts=read('context-frontiers-candidate9.json.gz')
context_rows='\n'.join('| '+c['label']+' | `'+c['results']['before']['text']+'` | `'+c['results']['after']['text']+'` | `'+c['results']['vanilla']['text']+'` |' for c in contexts['cases'])
report=f'''# Encerramento da sessão R3-T1 para organização do projeto

Encerramento solicitado pelo usuário; correções pausadas após validação final.
Este é um marco parcial validado, não a conclusão da paridade nem do objetivo
“Faça todas as correções”. Não houve commit, push, release, limpeza ou remoção de arquivos.

## Proveniência e escopo

Branch `Tekt`, HEAD `{head}`, working tree não commitado.
Encerramento UTC `{utc}`. O recibo `estado-encerramento.json.gz` conserva
status, arquivos não rastreados, diffstat exato e hashes do código no encerramento.
As medições possuem seus próprios freezes, horários, comandos e executáveis:
`before-sources.json.gz`, `candidate9-sources.json.gz`, `candidate10-sources.json.gz`
e os recibos `*-resources.json`. Os resultados da revisão 9 não foram
reclassificados como medições da revisão 10.

Aprovação: `00_nucleo/materialization/textual-realization-2026-10-18/approval-receipt.json`,
manifesto aprovado `47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302`.
A etapa altera {len(scope)} arquivos em relação ao seu freeze inicial:
{counts['Rust']} Rust, {counts['L0']} prompts L0 e {counts['Nucleo']} núcleo.
Lista e hashes em `escopo-etapa.json`; não confundir com todo o diff acumulado do projeto.

Oracle medido: vanilla `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
binário SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Linter pinado `12797073972e375d9ab078cf6750c8b30b71e2b9`.

## O que foi implementado

- Realização do pai antes dos filhos que sua regra show pode ocultar; coordenação
  causal privada em `compiler/realize/recipes.rs`, com owner L0 próprio.
- Distinção entre regras herdadas no ponto de inserção e regras locais do conteúdo.
- Precedência conjunta de literal/regex, seleção por posição e ordem lexical,
  produtos protegidos contra reaplicação e fronteiras de agrupamento preservadas.
- Show-set textual, restauração de ambiente nos retornos/erros e preservação de
  spans e identidade de aliases; integração com Sequence, Table e matemática.
- Correções de integração para fluxo de parágrafo, hifenização e contextos já
  cobertos pela suíte. Não equivalem a paridade universal de Context.
- Remoção dos dois novos V17 por desdobramento de guards, sem silenciamento.
  A revisão 10 só altera dois owners Rust e seus L0s em relação à revisão 9.
  Não foram introduzidas novas assinaturas públicas.

L0s foram atualizados antes do código e resselados. A auditoria final conserva
ownership e núcleos válidos; o resselo não autoriza ignorar V16.

## Validação e limites

| Evidência | Resultado | Estado medido |
|---|---|---|
| Suíte completa, incluindo doctests | {suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados | revisão 10, `suite-summary-candidate10.json` |
| Build e Clippy workspace/all-targets estrito | passaram | revisão 10, logs e `closure-candidate10-resources.json` |
| Rustdoc estrito, fmt, diff-check, linhagem direta/reversa | passaram | revisão 10, logs `*-candidate10.log` |
| V17 e V1/V5/V15/V26 | passaram | revisões 10, preflight e fechamento |
| PDF e HTML do corpus textual | 56/56 em cada target | revisão 9, `exports-check-candidate9.json` |
| Cores de show-set em SVG | 8/8; mais 6/6 de ordem entre funções e estilos | revisão 9, exports e `style-order-candidate9.json.gz` |
| Regressões de layout anteriores | 297 casos / 327 artefatos, 326 preservados e 1 correção comprovada | revisão 9, `mixed-regressions-candidate9.json.gz` |

Os 56 casos comparam aceitação, texto e erro principal; não provam geometria
PDF completa. A alteração de layout foi o alias de bibliografia: o recuo após
show passou de 26 para 44 pt, coincidindo com o vanilla. A prova específica está
em `alias-proof-candidate9.json.gz`; os demais PDFs foram comparados por
páginas/texto/geometria e SVGs por bytes. Estes corpora externos não foram
reexecutados na revisão 10; sua alteração de guards foi revista e a suíte
completa foi repetida. Os logs RED, erros de fixtures e candidatos intermediários
permanecem preservados; não são evidência GREEN.

Tekt global continua vermelho pelos **105 V16 herdados**. Resultado final:
105 V16:error, 1144 V19:note e 879 V20:note; nenhum novo erro/warning em relação
ao início desta etapa. `audit-candidate10.json` e `architecture-candidate10.log`
registram o diferencial. V19/V20 são observações, não bugs automaticamente.

## RAM

Fechamento executado sequencialmente com um job, MemoryMax=12 GiB,
MemoryHigh=8 GiB e swap zero. Pico do fechamento: **{peak:.3f} GiB**;
pico da validação ampla da revisão 9: **{previous_peak:.3f} GiB**.
Eventos completos nos recibos, sem OOM/OOM-kill nesses dois gates.

`ram-probe.json.gz` conserva a medição exploratória do produto na revisão 9.
Ela não demonstrou redução de RAM: o consumo ficou próximo do Tekt anterior.
Tekt foi medido em debug e vanilla em release, com duas repetições; essa amostra
não permite declarar ganho de produção ou comparação justa de desempenho.

## Pendências confirmadas de Context

As sondas adicionais da revisão 9 encontraram quatro divergências herdadas.
O controle de uma equação passa; os demais continuam abertos. Antes e depois
reproduzem a divergência; não são regressões introduzidas neste lote.
Evidência com fontes, comandos, horários e binários: `context-frontiers-candidate9.json.gz`.
Não houve nova medição externa destas sondas na revisão 10.

| Sonda | Tekt anterior | Tekt revisão 9 | Vanilla |
|---|---|---|---|
{context_rows}

Contagem/identidade de equações em contextos e regras textuais globais/locais
em Context são pendências de linguagem, não divergências mecânicas Tekt aceitas.
Retornos de callbacks fora de Content/String (R3-T2) também continuam abertos.
O corpus aprovado não cobre toda a linguagem. O restante do inventário e as
pendências de layout anteriores continuam válidos; ver `proximos.md`.

## Entrega para a limpeza

Ponto de entrada: `proximos.md`. Preservar código, L0s, aprovações, fontes dos
casos e evidências referenciadas. `outputs/` e `bin/` têm arquivos ignorados pelo
Git: não presumir que um commit os preserva. Classificar os artefatos antes de
remover; esta sessão não os apagou nem iniciou novos reparos.
'''
(p/'relatorio.md').write_text(report)
(p/'proximos.md').write_text('''# Entrega para limpeza e organização

A sessão de implementação está encerrada e o objetivo geral fica pausado por
pedido humano. Este documento é uma fila de organização; não autoriza código
fora dos L0s nem declara concluídas as frentes herdadas. Proveniência de cada
resultado no relatório e seus anexos, sempre distinguindo revisão 9 de 10.

## Ordem proposta para organizar o que temos

1. **Consolidar o estado local.** Usar `estado-encerramento.json.gz` e
   `escopo-etapa.json` para distinguir o diff acumulado dos arquivos deste lote.
   Preparar commits coerentes após revisão; não juntar todo o histórico em um
   commit genérico. Nenhum commit foi criado neste encerramento.
2. **Criar um índice único de situação.** Para cada frente registrar owner L0,
   aprovação aplicável, implementação, testes, prova CLI, pendências e dependências.
   Relatórios antigos permanecem históricos; não tratar seus “próximos passos”
   como estado vigente sem reconciliar com este relatório.
3. **Classificar os artefatos.** Separar evidência canônica, scripts reproduzíveis,
   rascunhos/propostas substituídas e caches reconstruíveis. Preservar as fontes
   exatas, dados importados, manifests e oráculos antes de apagar outputs/bin.
   Nenhuma lista de exclusão está aprovada ou executada.
4. **Reconciliar documentação de execução.** `andamento.md` e `progress.json`
   apontam agora ao encerramento; cópias anteriores foram preservadas. Relatórios
   mais antigos, como mixed-style/proximos e show-returns, conservam o contexto
   histórico e podem ter indicações de estado superadas por esta entrega.
5. **Priorizar o backlog por impacto e dependência**, usando a lista abaixo.
   Depois da organização, retomar apenas os lotes escolhidos, com L0 primeiro.

## O que ainda precisa ser feito

| Frente | Estado / próxima evidência necessária |
|---|---|
| Context: regras textuais | Corrigir herança global/local; as sondas context-global/context-local continuam em A contra R/L no vanilla. |
| Context: identidade/query | Corrigir contagem de equações em múltiplos contextos e após heading; não inventar Location sintética nem copiar contagem esperada no teste. |
| R3-T2: retornos de show | Investigação em ../show-returns-2026-10-18/relatorio.md; implementar coerção comum preservando morfologia Raw/Text/None/Module e regras aplicadas ao produto. |
| V16 | 105 erros herdados, auditoria por owner/L0; manter V19 informativo e testar alternativas pertinentes. |
| Bibliografia/layout herdado | Reconciliar ../mixed-style-metrics-2026-10-18/proximos.md: CSL plain/optimized, containers, regiões além do certificado, fontes, checkpoint com prefixo e decorações completas continuam abertos. O caso bib-content-alias foi corrigido neste lote. |
| Inventário geral de paridade | ../paridade-novo-vanilla-2026-10-17/inventario.md e ../sequence-r3-2026-10-18/escopo-geral.md continuam referências: RM1, RG, Figure/Paint/Symbol, reflexão, HTML/export e demais E01–E20 precisam de reconciliação por caso. Não considerar todos esses itens recém-medidos. |
| RAM de produção | Medir Tekt e vanilla em perfis comparáveis, com mesmos documentos/fontes/cache. O teto de 12 GiB foi respeitado; redução de RAM não foi demonstrada. |

## O que já pode ser usado como evidência

- Revisão 10: suíte completa, build, Clippy, Rustdoc, fmt e linhagem passaram.
- Revisão 9: corpus textual PDF/HTML, SVG de cores/precedência e preservação do
  corpus geométrico, com uma correção adicional de alias comprovada.
- V17 novos foram corrigidos; gate Tekt global segue bloqueado por V16.
- Aprovações e manifests anteriores continuam preservados. Não houve release.

Esta fila não é um Prompt L0 e não legitima materialização.
''')
for old,copy in [('progress.json','progress-before-closure.json'),('andamento.md','andamento-before-closure.md')]:
 if (p/old).exists() and not (p/copy).exists():(p/copy).write_bytes((p/old).read_bytes())
(p/'progress.json').write_text(json.dumps(dict(utc=utc,status='SESSION_CLOSED_GOAL_PAUSED_FOR_ORGANIZATION',all_corrections_complete=False,source_state='candidate10-sources.json.gz',report='relatorio.md',backlog='proximos.md',suite=suite,resources='closure-candidate10-resources.json',global_tekt_clean=False,remaining_v16=105),ensure_ascii=False,indent=2)+'\n')
(p/'andamento.md').write_text('# Sessão encerrada para organização\n\nPor pedido humano, as correções estão pausadas. O estado vigente está em\n[relatorio.md](relatorio.md) e a entrega para organização em\n[proximos.md](proximos.md). O objetivo completo ainda tem pendências.\n\nO andamento intermediário anterior está preservado em andamento-before-closure.md.\n')
untracked=git('ls-files','--others','--exclude-standard','-z').split('\0');untracked=[f for f in untracked if f]
state=dict(utc=utc,head=head,branch=git('branch','--show-current').strip(),working_tree='uncommitted',status=git('status','--short'),diffstat=git('diff','HEAD','--stat'),tracked_modified=git('diff','HEAD','--name-only').splitlines(),untracked=[dict(path=f,sha256=sha(root/f)) for f in untracked if (root/f).is_file()],source_state='candidate10-sources.json.gz',source_state_sha256=sha(p/'candidate10-sources.json.gz'),binaries={str(f.relative_to(root)):sha(f) for f in [root/'target/debug/typst',root/'lab/typst-original/target/release/typst'] if f.is_file()})
(p/'estado-encerramento.json.gz').write_bytes(gzip.compress(json.dumps(state,ensure_ascii=False,indent=2).encode()))
entries={q.name:sha(q) for q in sorted(p.iterdir()) if q.is_file() and q.name!='manifest-encerramento.json'}
(p/'manifest-encerramento.json').write_text(json.dumps(dict(utc=utc,head=head,scope='Arquivos diretos deste diagnóstico; outputs/bin não incluídos; seus hashes/fontes estão nos recibos específicos.',files=entries),ensure_ascii=False,indent=2)+'\n')
assert all((root/f).read_text()==s for f,s in freeze['files'].items())
print(json.dumps(dict(report=str(p/'relatorio.md'),passed=suite['passed'],ignored=suite['ignored'],peak_gib=peak,scope=dict(counts),manifest_sha256=sha(p/'manifest-encerramento.json')),ensure_ascii=False))
