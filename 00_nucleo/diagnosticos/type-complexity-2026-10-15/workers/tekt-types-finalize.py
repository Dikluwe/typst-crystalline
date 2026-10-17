from pathlib import Path
import json,gzip,hashlib,re,datetime,subprocess,shutil,collections
root=Path.cwd();out=root/'00_nucleo/diagnosticos/type-complexity-2026-10-15';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=json.load(gzip.open(out/'before.json.gz'));assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head'];v=json.loads((out/'validation-details.json').read_text());r=json.loads((out/'validation-result.json').read_text());t=json.loads((out/'workspace/workspace-result.json').read_text())
assert all(x['exit']==0 for x in v['commands']) and len(v['commands'])==9 and r['exit']==t['exit']==0
assert t['sources_unchanged'] and v['sources_unchanged_during_validation']
assert all(sha(p)==h for p,h in v['source_hashes'].items())
assert v['clippy_delta']=={'clippy::type_complexity':-30} and v['clippy_total']==144
inventory=json.loads((out/'clippy-inventory.json').read_text());remaining=collections.Counter(c['code'] for c in inventory);assert remaining['clippy::too_many_arguments']==57 and remaining['clippy::type_complexity']==0
(out/'remaining-clippy-rules.json').write_text(json.dumps(dict(sorted(remaining.items())),indent=2)+'\n')
assert r['memory_after']['memory.max'].strip()=='12884901888'
assert r['memory_after']['memory.swap.max'].strip()=='0'
mem={k:int(n) for k,n in (line.split() for line in r['memory_after']['memory.events'].splitlines())};assert mem['oom']==mem['oom_kill']==0
subprocess.run(['python3','/tmp/tekt-types-report.py'],check=True)
alias='((FontList, FontVariant, FontVariations), Vec<u8>)';font='03_infra/src/export/render.rs';assert sha(font)==b['all_source_hashes'][font]
normalize=lambda s:re.sub(r'\s+','',re.sub(r'//[^\n]*','',re.sub(r'\bFontKey\b',alias,s)))
def signatures(text):return [normalize(s) for s in re.findall(r'^\s*pub fn\s+[^\{]+',text,re.M)]
api=[]
for p,old in b['files'].items():
 if not p.endswith('.rs'):continue
 a=signatures(old['text']);z=signatures(Path(p).read_text());assert a==z,p
 api.append(dict(path=p,public_functions=len(a),equivalent_after_existing_alias_expansion=True,final_sha256=sha(p)))
assert sum(x['public_functions'] for x in api)==80
(out/'api-equivalence-final.json').write_text(json.dumps(dict(utc=now(),head=b['head'],existing_font_key=dict(source=font+':37',declaration='pub type FontKey = '+alias+';',source_sha256=sha(font)),files=api,public_functions=80,new_public_types=False,new_public_fields=False),indent=2)+'\n')
html={}
for crate in ['typst_core','typst_shell','typst_infra','typst']:
 p=root/'target/doc'/crate/'index.html';assert p.exists() and '<html' in p.read_text()[:300].lower();html[str(p.relative_to(root))]=sha(p)
(out/'final-state.json').write_text(json.dumps(dict(utc=now(),head=b['head'],working_tree='não commitado; alterações anteriores preservadas',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),sources_unchanged_since_validation=True,source_hashes=v['source_hashes'],documentation_index_hashes=html),ensure_ascii=False,indent=2)+'\n')
peak=int(r['memory_after']['memory.peak']);gib=peak/1024**3
checks='\n'.join(f"| {x['label']} | passou | `{ ' '.join(x['command']) }` |" for x in v['commands'])
report=f'''# Correções de type_complexity — 2026-10-15

Etapa concluída: 30 avisos de `clippy::type_complexity` tratados, sendo 27 por alteração de representação/nomenclatura e três por `#[expect]` pontual com justificativa no L0. O inventário Clippy passou de 174 para 144 avisos, sem avisos novos de outras regras. Isso não encerra a dívida de Clippy/Tekt nem torna todo o CI livre de avisos.

## Leitura humana e por IA

Os nomes tornam os dados locais identificáveis: `RowCell` distingue quatro índices/spans antes posicionais; `EvaluatedPass`, `PreparedBundle` e `PagedCompilation` nomeiam resultados e evitam confundir campos. `TiffReaders` nomeia leitores de endianness e elimina duas seleções duplicadas e um boolean sem uso. As 21 repetições do tipo de fontes usam o alias canônico `FontKey`, já existente.

Não houve medição de desempenho, precisão ou custo de leitura de IA. A justificativa é semântica explícita e consistência no código, sem afirmar ganho quantitativo para modelos. As três exceções preservam contratos já legíveis: `SYM_GROUPS` (nome/base/factory), `native_fn_addr` (assinatura nativa explícita) e `ACCESSIBLE` (tabela de standards/labels em L4). Não há supressão no crate/módulo ou mudança de limiar; o gate de expectativas não cumpridas também passou.

## Escopo e contratos

Os 30 casos estão na [lista completa](lista.md), no [CSV](cases.csv) e em [JSON](cases.json), com fonte anterior, fonte atual, L0 proprietário, ação e justificativa. Foram atualizados 15 owners e seus 15 consumers, incluindo três consumidores indiretos do resultado privado do pipeline. O [diff da etapa](stage.diff) compara contra o estado exato de entrada, preservando as alterações anteriores da working tree.

Os L0 foram editados antes da implementação e resselados de modo restrito; cada exceção foi prescrita nominalmente. Os registros novos são privados ao código que os usa. A comparação das 80 funções públicas nos arquivos afetados confirma assinaturas equivalentes após expandir o alias FontKey preexistente e inalterado ([prova](api-equivalence-final.json)). Não foram criados tipos/campos públicos nem alterados defaults ou fase do pipeline. Os 57 casos de `too_many_arguments` permanecem no estágio de auditoria anterior, sem materialização neste lote.

## Validação

O gate RED de type_complexity falhou antes da edição. O GREEN passou com `-D clippy::type_complexity -D unfulfilled_lint_expectations` ([recibo](green-clippy-final-result.json)). As tentativas intermediárias encontraram um retorno de documento vazio e um consumidor de teste ainda em tupla; ambos migrados para o registro privado conservando valores e assertions. O primeiro inventário completo encontrou dois imports redundantes, removidos antes da validação final; os logs dessa tentativa permanecem em `validation-initial/`.

| Verificação | Resultado | Comando |
|---|---|---|
{checks}

Suíte completa: **{t['passed']} testes passaram, {t['failed']} falharam e {t['ignored']} foram ignorados**; nenhum esperado foi relaxado e nenhum teste artificial de alias foi criado. O teste consumidor do resultado paginado foi adaptado, conservando programas, tamanhos e assertions. O inventário final tem **0 type_complexity e 144 avisos Clippy**, com delta exclusivamente de −30 nessa regra. A linhagem estrita V1/V5/V15/V26 e a reversa passaram. O Tekt completo registrou {json.dumps(v['architecture_levels'],ensure_ascii=False)}; avisos por regra: {json.dumps(v['architecture_warnings'],ensure_ascii=False)}. Sem novos avisos arquiteturais e sem erros; os demais avisos continuam pendentes.

Rustdoc passou com `RUSTDOCFLAGS="-D warnings"`; os quatro índices HTML foram confirmados depois da suíte. As fontes e os L0 permaneceram imutáveis durante a validação. [Comandos, horários, estado e resultados](validation-details.json), [suíte](workspace/workspace-result.json), [estado final](final-state.json).

## Memória e proveniência

A validação foi executada com **MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1**. Pico total do cgroup: **{gib:.3f} GiB ({peak} bytes)**, incluindo cache de arquivos; eventos OOM={mem['oom']}, OOM-kill={mem['oom_kill']}. [Recibo do serviço](validation-result.json). O limite cobre os processos da etapa, não outros aplicativos do host.

HEAD `{b['head']}`, working tree não commitado. Origem em `{b['utc']}`; validação final entre `{v['start']}` e `{v['end']}` (UTC). O [snapshot inicial](before.json.gz) e os snapshots de validação incluem hashes e a lista exata de arquivos alterados (`git diff HEAD --stat`), além do estado Git. A suíte verifica o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, e usa o linter CI pinado em `49c6bb0e584104a0c89270431d8402f5c6c61150`.

Os scripts de execução estão preservados em `workers/` (a receita original usa caminhos temporários `/tmp` e as mesmas ferramentas já instaladas). [Manifesto SHA-256](manifest.sha256). Sem commit, push ou publicação de release.
'''
report=re.sub(r'\]\((?!/)([^):]+)\)',lambda m:']('+str(out/m.group(1))+')',report)
(out/'relatorio.md').write_text(report)
for p in Path('/tmp').glob('tekt-types-*.py'):shutil.copyfile(p,out/'workers'/p.name)
manifest=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name!='manifest.sha256':manifest.append(f'{sha(p)}  {p.relative_to(out)}')
(out/'manifest.sha256').write_text('\n'.join(manifest)+'\n')
print(json.dumps(dict(report=str(out/'relatorio.md'),tests=t['passed'],clippy=v['clippy_total'],type_complexity=0,peak_gib=gib,levels=v['architecture_levels']),ensure_ascii=False))
