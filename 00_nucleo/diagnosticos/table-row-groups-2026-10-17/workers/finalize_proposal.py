from pathlib import Path
import datetime,gzip,hashlib,json,subprocess
root=Path.cwd();out=Path(__file__).resolve().parents[1];package=root/'00_nucleo/materialization/table-row-groups-2026-10-17';load=lambda p:json.load(open(out/p));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(package/'scope.json'));graph=load('graph-result.json');assert graph['candidate_graph_valid'] and graph['sources_unchanged'] and graph['productive_bodies_changed']==0;assert load('graph-resources.json')['exit']==0;assert all(sha(package/'l0'/p)==h for p,h in graph['overlays'].items());assert all(sha(p)==h for p,h in b['hashes'].items());assert all(x['equal'] for x in load('reference-source-hashes.json')['git_pin_checks']);measurements={name:json.load(gzip.open(out/(name+'.json.gz'))) for name in ['measurement','supplement','lines','styles-phase','metadata','svg']};assert all(x['sources_unchanged'] for x in measurements.values());runs=sum(len(x['cases']) for x in measurements.values());cases=runs//2;assert runs==cases*2;resources={str(p.relative_to(out)):json.load(open(p)) for p in out.rglob('*-resources.json')};peak=max(int(r['memory_after']['memory.peak']) for r in resources.values())
for r in resources.values():
 assert r['memory_after']['memory.max']==str(12*1024**3) and r['memory_after']['memory.swap.max']=='0' and r['cargo_jobs']=='1';events=dict(x.split() for x in r['memory_after']['memory.events'].splitlines());assert all(events[k]=='0' for k in ['oom','oom_kill','oom_group_kill'])
links=lambda p:str(root/p)
review=f'''# RG1 — revisão do contrato de grupos Table

**Estado: proposta pronta; implementação NÃO autorizada por este documento.** A raiz produtiva e os L0s vigentes permanecem intactos. Aprovar este pacote autoriza instalar os candidatos, escrever os testes RED e implementar somente o contrato descrito. Não inclui commit/push do Typst, merge, release ou pin novo.

## Evidência antes da decisão

Foram executados **{cases} casos bilaterais, {runs} invocações**: metadados, posições, papéis, erros, estilos e SVG. HEAD `{b['head']}`, working tree não commitado. [Inventário e estado anteriores]({out/'before.json.gz'}), [resumo inicial]({out/'summary.json'}), [relatório]({out/'relatorio.md'}). As fontes de referência consultadas foram comparadas byte a byte aos objetos Git do pin `9f2b6e8715237cb086899a42873660fe744622e8` ([hashes]({out/'reference-source-hashes.json'})). O oráculo executado tem SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

- Header/footer com colspan/rowspan geram padding e papéis errados porque o Tekt mede por entradas, não ocupação.
- Headers entre células do Body são movidos ao topo; o parser também funde níveis/repeat e rejeita níveis iguais/aliases que o pin aceita. Footer fixo pode admitir células lexicamente posteriores antes da sua faixa; a rejeição atual é antecipada.
- Grupos vazios e line-only são válidos no pin; linhas não são células.
- A validação estrutural ocorre **antes** da transformação Table, mesmo com `none`, `[]`, substituição textual ou panic da Table. Cell/body ocultados continuam sem executar. As duas ordens de show-set medidas têm o mesmo erro estrutural. Os controles anteriores de conteúdo válido ocultado permanecem válidos; sua generalização para estrutura inválida foi refutada.
- Header `level`/`repeat` omitidos são desconhecidos no valor raw; explícitos aparecem em fields. `children` é conhecido inclusive vazio. Header.children.first() cru é igual à Cell crua equivalente, e func deve ser a nativa verdadeira: a promoção Auto antecipada e a func substituta do Tekt falham nesses observáveis.
- H/V ignoram y/x e perdem presença. None em índice, auto em end ou position, e ranges inválidos têm rejeições específicas no pin; o Tekt aceita vários deles.
- SVG confirmou erasure/overhang: vermelho horizontal base 82 unidades →41 com None no pin e 82→82 no Tekt; azul após None 41→0 no pin e 40→40 no Tekt. A unidade é o eixo lógico do viewBox, em documentos de 100pt sem texto. Não comparar número de paths ou bytes entre renderers. [Dados e método]({out/'svg-analysis.json'}).

A diferença de `tbody` omitido/expresso no controle ordinário é sintaxe de container HTML; não foi contada como defeito de ocupação. O número inicial de comparações de rows diferentes inclui essa distinção e não significa 35 defeitos funcionais.

## Contrato proposto

1. **Fonte ordenada única:** Table.children guarda Cell/Header/Footer/HLine/VLine; remover os quatro campos públicos separados. O parser conserva ordem e tipos, sem fundir/pad/posicionar.
2. **Metadata lexical:** Header ganha nível positivo próprio e máscara; Footer ganha máscara. Linhas usam x/y opcionais tipados e máscara. Omissão, auto, none e false explícitos são distintos. As funções públicas ergonômicas mantêm assinatura; a migração de seus valores raw está descrita na API.
3. **Plano por intervalos:** substituir header_rows/footer_start por row_groups reais e linhas resolvidas. Cinco tipos públicos novos e um accessor is_complete. Um único planner calcula ocupação e faixas; o código físico e o HTML consomem o resultado.
4. **Preparação causal:** com show-set aplicável resolvido, sintetizar/validar a estrutura antes da transformação Table. Plano pending não é visual pronto. A receita lê filhos lexicais. Só a Table sobrevivente realiza Cell/body uma vez, depois publica complete.
5. **Tinta:** compor linhas/bordas antes de emitir; None remove contribuição prévia no intervalo, com overhang e limites corretos. Não pintar branco nem retirar shapes/texto indiscriminadamente. Preservar o caminho Grid deste pacote.

[API pública completa e migração]({package/'api.md'}) — seis carriers existentes, cinco tipos novos, 13 constantes de máscara e is_complete; inclui remoções/substituições de campos e mudança de Auto cru. Todas as formas públicas estão nos respectivos **L0s candidatos**, não legitimadas por este documento de processo.

## Inferência Tekt e limites

Nível do grupo e kind/nível explícito da Cell são conceitos diferentes e ambos serão conservados. Para placeholders Auto de Header, escolher o papel/nível efetivo do grupo é a decisão Tekt proposta. A fonte de referência `layout/grid/resolve.rs:1565–1569` usa nível1 na ramificação de grupo completamente vazio. Não se infere intenção dessa implementação nem se exige igualdade do enum Rust. **Inferência:** herdar o grupo evita reclassificar padding para um nível não pedido; será refutada por perda do kind explícito, relação semântica demonstravelmente errada ou regressão dos controles. Isso não certifica PDF/UA/aplicabilidade de Headers.

RG1 permanece incompleto no universo Table. **RG2/table-root-fields** completa os campos/defaults gerais da Table além de children (a sonda raw-table-fields continua sentinela). Defaults settable próprios de Header/Footer/linhas exigem RG3/group-line-style-defaults. Rich functions/paints/gutters, repetição multipágina e PDF/UA continuam etapas próprias; o HTML de Grid e as três pendências math/matching também ficam abertos. Não alterar seus esperados para declarar paridade.

## L0, núcleo e revisão

- **53 L0s** candidatos com owner exclusivo: 34 têm mudança de lógica/testes/adaptação e 19 apenas o pin compartilhado. [Escopo por caminho e SHA]({package/'scope.json'}).
- Refinar o núcleo `00_nucleo/prompts/_nuclei/table/cell-realization.toml`, com **43 consumidores diretos**, para separar validação estrutural de receita. Pin novo `{scope['phase_kernel']['new_pin']}`.
- Novo núcleo `00_nucleo/prompts/_nuclei/table/row-groups.toml`, dependência pinada do anterior, digest `{scope['row_kernel']['pin']}`. Não é owner nem alvo de @prompt.
- [Pins de headers previstos]({package/'header-pins.json'}); nenhuma lógica produtiva foi escrita. [Grafo candidato]({out/'graph-result.json'}) passou V1/V5/V15/V26 em cópia descartável; isso prova integridade/linhagem, **não implementação**. Os primeiros erros de ordem/posição dos blocos de pins foram corrigidos apenas no pacote e estão preservados.
- [R1–R20: testes e gates]({package/'test-plan.md'}). Confirmar RED funcional antes da implementação; erros de compilação não o substituem. Reexecutar build/suíte inteira/Clippy/Rustdoc/fmt/linhagem/reversa/diferencial, sob 12 GiB, swap zero, Cargo um job e targets de mutação isolados.

## Por que exige confirmação

A [ADR-0127 §2]({root/'00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md'}) determina paragem para “adicionar, remover ou alterar campo público” e “Mudança arquitectural de fase/ordem de pipeline”, além de compatibilidade. Este pacote faz as três coisas. Clientes Rust que constroem os carriers ou leem os campos removidos precisarão migrar. A autorização genérica para corrigir o projeto não substitui esse gate específico.
'''
(package/'review.md').write_text(review)
report=f'''# Diagnóstico e proposta — grupos Table — 2026-10-17

Etapa de medição/proposta concluída. **A implementação aguarda aprovação ADR-0127.** Nenhum arquivo produtivo, L0 vigente, kernel vigente ou configuração mudou nesta etapa. O objetivo geral e os **131 V16** permanecem abertos.

[Revisão para aprovação]({package/'review.md'}) · [API e migração]({package/'api.md'}) · [R1–R20]({package/'test-plan.md'}).

## Resultado

A coleta tem {cases} casos bilaterais e {runs} invocações. Revelou a causa documental/estrutural das divergências de spans e dos níveis: fontes separadas perdem a ordem e os metadados; padding por contagem não é ocupação; validação estrutural e receitas precisam de fronteiras distintas. O contrato anterior sobre conteúdo válido ocultado não provava que estrutura inválida poderia ser ignorada. O pin refuta isso com colspan, duplicação, footer e show-set.

Presença lexical e identidade também estavam incompletas: raw Header/Footer/linhas perdem fields/func; Auto era promovido antes de ser necessário. Índices x/y de linhas são ignorados; None de stroke deixa tinta existente e explicit line ignora extensão de overhang. Todos os inputs/stdout/stderr/argv/binários/fontes/UTC estão em measurement/supplement/lines/styles-phase/metadata/svg .json.gz e seus arquivos de casos. Não chamar cada diferença sintática de um defeito: tbody opcional do controle é identificado na revisão.

## Proposta concreta

A revisão contém a nova fonte única, máscaras/índices tipados, plano por intervalos, transição pending/completo, fronteira de validação antes da Table e realização de Cells sobreviventes depois. São 53 L0s candidatos, 34 consumers de lógica/testes e 19 de pin somente, núcleo compartilhado refinado e um novo núcleo dependente. Tipos/constantes/método públicos e a migração estão descritos antes de qualquer alteração de API. O grafo candidato passou V1/V5/V15/V26 em cópia temporária. Isso não é GREEN funcional: código novo e R1–R20 ficam para depois da aprovação.

Permanecem RG2/table-root-fields, RG3/group-line-style-defaults, rich properties/gutters, repetição/semântica multipágina/PDF-UA, HTML Grid, math-stretch-composite, matching-local-cjk e matching-identical-rule-precedence. O objetivo inteiro não foi reduzido ao RG1.

## Proveniência e recursos

HEAD `{b['head']}`, working tree não commitado; UTC/status/git diff HEAD --stat/fontes/L0/hashes em before.json.gz e proposal-base.json.gz. Binário oracle SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin 9f2b6e8715237cb086899a42873660fe744622e8; as quatro fontes locais consultadas conferem com objetos Git desse commit (reference-source-hashes.json). SHA do Tekt por rodada está nas coletas; não usar --version como prova.

MemoryMax=12 GiB, swap zero, Cargo um job; execuções sequenciais, pico observado {peak} bytes ({peak/1024**3:.3f} GiB), nenhum OOM. Contadores em *-resources.json. Não houve novo build de produção ou repetição da suíte, pois a raiz ficou intacta; a suíte anterior de 8742 testes continua histórica para o mesmo freeze. Sem commit/push/merge/release/pin CI novo.
'''
(out/'relatorio.md').write_text(report)
# Review index is a human-facing list; claims remain exclusively in their own L0/nuclei.
(package/'owners.md').write_text('# Owners candidatos RG1\n\n| Consumer | Prompt L0 proprietário | Mudança |\n|---|---|---|\n'+''.join(f'| [{data["source"]}]({root/data["source"]}) | [{owner}]({package/"l0"/owner}) | {data["classification"]} |\n' for owner,data in sorted(scope['owners'].items())))
(package/'review.md').write_text((package/'review.md').read_text().replace('[Escopo por caminho e SHA]',f'[Lista legível de owners]({package/"owners.md"}) · [Escopo por caminho e SHA]'))
final=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=b['hashes'],sources_unchanged=all(sha(p)==h for p,h in b['hashes'].items()),stage_complete='measurement-and-proposal',implementation_complete=False,approval_pending=True,goal_scope_incomplete=True,remaining_v16=131,cases=cases,runs=runs,resources=resources,peak_bytes=peak,graph=graph,package=str(package),open_named_debt=['table-row-group-contract','table-root-fields','group-line-style-defaults','math-stretch-composite','matching-local-cjk','matching-identical-rule-precedence'])
assert final['sources_unchanged']
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(final,f)
# The proposal digest binds API, every candidate owner/kernel, scope, tests and review.
files=sorted(p for p in package.rglob('*') if p.is_file() and p.name!='SHA256SUMS');(package/'SHA256SUMS').write_text(''.join(f'{sha(p)}  {p.relative_to(package)}\n' for p in files));r=subprocess.run(['sha256sum','--check','SHA256SUMS'],cwd=package,capture_output=True,text=True);assert r.returncode==0,r.stderr
proposal_digest=sha(package/'SHA256SUMS');(out/'proposal-manifest.json').write_text(json.dumps(dict(path=str(package/'SHA256SUMS'),sha256=proposal_digest,files=len(files),status='pending-human-confirmation'),indent=2)+'\n')
files=sorted(p for p in out.rglob('*') if p.is_file() and p.name!='SHA256SUMS');(out/'SHA256SUMS').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in files));r=subprocess.run(['sha256sum','--check','SHA256SUMS'],cwd=out,capture_output=True,text=True);assert r.returncode==0,r.stderr
print(json.dumps(dict(proposal_sha256=proposal_digest,case_pairs=cases,runs=runs,peak_bytes=peak,productive_changes=0,live_l0_changes=0,approval_pending=True)),flush=True)
