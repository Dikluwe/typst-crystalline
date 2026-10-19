from pathlib import Path
import collections,datetime,gzip,hashlib,json,re,shutil,subprocess
root=Path.cwd();out=Path(__file__).resolve().parents[1];load=lambda p:json.load(open(out/p));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();b=json.load(gzip.open(out/'before.json.gz'));v=load('validation-result.json');assert v.get('validation_complete') and v['sources_unchanged_during_validation'];assert all(sha(p)==h for p,h in v['source_hashes'].items());scope=load('scope.json');suite=load('workspace/workspace-result.json');assert suite['exit']==0 and suite['passed']==8742 and suite['failed']==0 and suite['ignored']==7;sarif=load('architecture-delta.json');assert sarif['after']=={'V16:error':131,'V19:note':1059,'V20:note':867};green=(out/'validation-green.log').read_text();assert '22 passed; 0 failed; 0 ignored;' in green;assert load('red-result.json')['failed']==18;assert load('probe-baseline-result.json')['selected_matches']==0 and load('probe-green-result.json')['selected_matches']==18;assert load('guard-result.json')['unrelated_sources_L0_configuration_unchanged'];assert load('observables-result.json')['public_source_pairs_restored']
resources={str(p.relative_to(out)):json.load(open(p)) for p in out.rglob('*-resources.json')};assert resources['validation-resources.json']['exit']==0;peak=max(int(r['memory_after']['memory.peak']) for r in resources.values())
for r in resources.values():
 assert r['memory_before']['memory.max']==r['memory_after']['memory.max']==str(12*1024**3) and r['memory_before']['memory.swap.max']==r['memory_after']['memory.swap.max']=='0' and r['cargo_jobs']=='1'
 events=dict(line.split() for line in r['memory_after']['memory.events'].splitlines());assert all(events[k]=='0' for k in ['oom','oom_kill','oom_group_kill'])
docs={str(root/'target/doc'/name/'index.html'):sha(root/'target/doc'/name/'index.html') for name in ['typst','typst_core','typst_shell','typst_infra']};changed=[p for p,h in b['hashes'].items() if sha(p)!=h];assert sorted(changed)==sorted(scope['sources']+scope['owners']);snapshot=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],changed=changed,scope=scope,tests=suite,architecture=sarif,probes={p:load('probe-'+p+'-result.json') for p in ['baseline','green']},resources=resources,peak_bytes=peak,docs=docs,linter=b['linter'],new_tests=22,stage_complete=True,goal_scope_incomplete=True,remaining_v16=131,open_named_debt=['table-row-group-contract','math-stretch-composite','matching-local-cjk','matching-identical-rule-precedence'],no_commit_push_release=True)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(snapshot,f)
report=f'''# V16 — células e grupos de tabela — 2026-10-17

Etapa concluída localmente. O objetivo geral continua aberto: **131 V16** e as pendências nomeadas abaixo. Sem commit/push do Typst, merge ou release.

## Resultado medido

| Regra | Antes | Depois | Decisão |
|---|---:|---:|---|
| V16 (erro) | 141 | 131 | Dez avisos corrigidos; também fechadas projeções latentes. |
| V19 (nota) | 1041 | 1059 | Dezoito grupos explícitos justificados nos L0s; corpos compartilhados preservados. |
| V20 (nota) | 869 | 867 | Duas projeções de receita aninhadas substituídas por projeção rasa compartilhada. |

Não foram adicionados `allow`/`expect`, exceções ou ajustes de severidade. Os demais diagnósticos SARIF foram preservados com seus campos completos e remapeamento de linhas (`architecture-delta.json`). O gate global continua vermelho pelos V16 restantes.

## Correções

- `native_table_header` reconhece Cell através de Source, conserva placement, spans, presença, kind Data/Header explícito e estado preparado/guards. Só promove Auto; célula crua recebe origem causal antes do fallback de argumento.
- `native_table_footer` produz uma Cell crua por argumento, mantendo o conteúdo Sequence inteiro e a origem. Cell explícita permanece íntegra. Não executa show nem preparação no constructor.
- Coleta de níveis reconhece membros Source. Isso corrige a rejeição indevida de níveis distintos nessa representação; não resolve a separação de grupos por nível.
- Restauração de receita local/contextual reconhece Source(ShowRecipe), conserva precedência e shadow de valor local inválido, contexto, regras, guards e restauração também em erro.
- Preparação/projeção de grupos reconhece Sequence através de Source; diagnóstico de conflito conserva a primeira origem comprovada da Cell. Grid e o algoritmo de ocupação permanecem fora da mudança.
- `show ...: none` converte para conteúdo vazio. Outros escalares inválidos continuam rejeitados, com as mensagens legadas preservadas.

Os quatro owners L0 foram prescritos antes do código e ressellados bilateralmente. `scope.json`, `l0-before-code.json`, `implementation-replay.json` e `guard-result.json` registram a correspondência, o replay e a preservação das declarações públicas. Nenhum campo, assinatura ou trait público mudou.

## Prova e validação

- **RED funcional: 18 falhas e 4 controles aprovados; GREEN: 22 aprovados.** Duas preparações foram corrigidas: `with_source_span` numa Cell já assentada não construía o wrapper pretendido; `*gone*Z` era markup inválido. As tentativas estão em `attempt-source-fixture/` e `attempt-show-fixture/`. Helpers/entrada foram corrigidos, sem reduzir as asserções, e o RED funcional foi repetido antes da implementação válida.
- Variantes futuras reais em Content/Value/TableCellKind, cópia descartável do core e testemunha cfg(test): zero matches selecionados exigiam revisão antes; **18 matches finais exigem E0004** depois. Targets Cargo isolados; só dependências externas foram copiadas como cache.
- Os workers de diagnóstico foram ajustados para identificar a função pelo AST, desambiguar padrões iguais, separar o V19 novo de um grupo preexistente e comparar caminhos relativos de fixtures. As tentativas estão preservadas em `attempt-probe-*`, `attempt-differential-arm/` e `attempt-observable-path/`; código/L0s ficaram congelados e a validação retomou só no gate afetado.
- Sondas via APIs reais confirmaram origem/propriedades e snapshots. CLI bilateral contra vanilla pinado verificou células/atributos/efeitos e erros; os controles ocultados não executam panic de Cell, e a regra tardia do footer atua sobre uma Cell por argumento.
- Suíte workspace: **8742 aprovados, zero falhas, 7 ignorados**. Build, formato, Clippy all-targets com warnings e expectativas não cumpridas como erro, Rustdoc com warnings como erro/quatro índices, linhagem V1/V5/V15/V26, reversa, replay e diff-check aprovados. Logs/comandos em `validation-result.json` e `workspace/`.
- Todos os comandos pesados foram sequenciais: MemoryMax **12 GiB**, swap zero, Cargo um job. Pico observado **{peak} bytes ({peak/1024**3:.3f} GiB)**; nenhum OOM ou processo morto por OOM. Contadores completos por execução em `*-resources.json`.

## Limites e próxima ordem

1. **table-row-group-contract:** ainda divergem header colspan, footer explícito colspan e separação entre níveis de header. O algoritmo atual preenche por quantidade de entradas; isso não prova ocupação correta. Nível/repetição do grupo e kind semântico da Cell são conceitos distintos. Uma eventual mudança de entidade pública exige proposta concreta e o gate ADR-0127 antes do código. Não foi implementada nem aprovada neste lote.
2. **math-stretch-composite**.
3. **matching-local-cjk** e **matching-identical-rule-precedence**.
4. Continuar os demais V16 por owners, mantendo os V19 justificados. Casts ricos/linhas e compromissos históricos de tabelas não estão certificados por este resultado. Mensagens legadas dos outros tipos inválidos de show foram conservadas; paridade textual delas não foi reivindicada.

## Proveniência

HEAD `{b['head']}`, **working tree não commitado**. A lista exata anterior está em `before.json.gz`; UTC, status, `git diff HEAD --stat` e SHA-256 no início dos gates estão em `validation-result.json` e `workspace/workspace-pre.json.gz`; o estado final está em `final-state.json.gz`. Nos arquivos produtivos, L0s e configurações inventariados, as alterações desta etapa ficam restritas aos quatro consumers e seus quatro L0s. Alterações anteriores não foram descartadas.

Linter: commit `12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`; pin CI já aplicado, sem nova publicação nesta etapa. Oráculo: `9f2b6e8715237cb086899a42873660fe744622e8`, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. As sondas registram argv, fonte, binário e horários; não usam string de versão como prova.
'''
remaining=load('remaining-v16.json');groups=collections.defaultdict(list)
for row in remaining:
 loc=row['locations'][0]['physicalLocation'];groups[loc['artifactLocation']['uri'].removeprefix('./')].append(loc['region']['startLine'])
assert sum(map(len,groups.values()))==131
inventory=[f'# V16 restantes — inventário por owner\n\n131 erros em {len(groups)} consumers, medidos no estado congelado de `final-state.json.gz`, HEAD `{b["head"]}`, working tree não commitado. Emissão integral em `remaining-v16.json`; UTC, hashes, status e diffstat em `validation-result.json`. Agrupamento por caminho; este inventário organiza a revisão dos próximos lotes.\n\n| Consumer e linhas | Quantidade | Prompt L0 proprietário |\n|---|---:|---|\n']
for source,lines in sorted(groups.items()):
 owner=re.search(r'^//! @prompt (.+)$',Path(source).read_text(),re.M).group(1);assert owner.startswith('00_nucleo/prompts/');inventory.append(f'| [{source}]({root/source}:{min(lines)}) — '+', '.join(map(str,sorted(lines)))+f' | {len(lines)} | [{owner}]({root/owner}) |\n')
(out/'inventario-v16-restante.md').write_text(''.join(inventory))
report+='\nLista por owner e linha: [inventario-v16-restante.md](inventario-v16-restante.md).\n'
(out/'relatorio.md').write_text(report)
# Private mutated artifacts are no longer needed; do not touch root target.
target=Path('/tmp/tekt-table-cells-probe-target');assert target.is_dir() and target.resolve()!=root/'target';shutil.rmtree(target);assert all(sha(p)==h for p,h in v['source_hashes'].items());(out/'cleanup.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),removed=str(target),main_target_preserved=(root/'target').is_dir(),sources_unchanged=True),indent=2)+'\n')
files=sorted(p for p in out.rglob('*') if p.is_file() and p.name!='SHA256SUMS');(out/'SHA256SUMS').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in files));r=subprocess.run(['sha256sum','--check','SHA256SUMS'],cwd=out,capture_output=True,text=True);assert r.returncode==0,r.stderr;print(json.dumps(dict(stage_complete=True,remaining_v16=131,tests_passed=8742,peak_bytes=peak,manifest_files=len(files),manifest_sha256=sha(out/'SHA256SUMS'))),flush=True)
