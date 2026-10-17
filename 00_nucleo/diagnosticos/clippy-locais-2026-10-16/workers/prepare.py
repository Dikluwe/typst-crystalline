from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,re
out=Path('00_nucleo/diagnosticos/clippy-locais-2026-10-16');prior=Path('00_nucleo/diagnosticos/clippy-default-2026-10-16')
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
state=json.loads((prior/'final-state.json').read_text());assert all(sha(p)==h for p,h in json.loads((prior/'validation-details.json').read_text())['source_hashes'].items())
plans={
'03_infra/src/export/svg.rs': 'Conservar SvgOptions público, campos e Default preexistente, apenas derivando Default no struct; pretty e render_bleed continuam false. Não adicionar nem retirar contrato Rust.',
'03_infra/src/export/mod.rs': 'Conservar os impls Default já públicos por meio de derive(Default) e #[default] explícito em StreamMode::Verbose e PdfTags::Enabled. Manter todos os outros derives, variantes e valores; não trocar o default de produção ADR-0126 nem PdfSerializationMode.',
'03_infra/src/shaper/cache.rs': 'No FaceCache::get, usar HashMap::entry(slot_idx).or_insert_with para executar World::font e CachedFace::new somente em miss. Guardar também None como resultado negativo; devolver empréstimo da face retida. Não antecipar consulta, duplicá-la, eliminar bytes owned que sustentam o borrow, alterar keys ou ampliar lifetime/escopo/API.',
'03_infra/src/pipeline/html.rs': 'Mover somente o módulo #[cfg(test)] p1465_tests para o fim do consumer, após os helpers produtivos. Manter nome, corpo, atributos, imports e todos os testes; nenhum helper muda de módulo/visibilidade. Ordem de declarações não muda o pipeline.',
'03_infra/src/pipeline/svg.rs': 'Mover somente o módulo #[cfg(test)] p1462_tests para o fim do consumer. Manter nomes, corpos, atributos, import super, APIs e ordem funcional de todos os helpers produtivos, sem extração/dispatch novo.',
'03_infra/src/export/tests/mod.rs': 'Substituir apenas o corpo de p273_6_layouter_parent_bbox_consumed_by_block_arm: o antigo Some(Rect) seguido de is_some/unwrap não exercita layout. O mesmo teste deve construir Shape com gradiente Parent dentro de Content::block 200x100 e executar layout real, observar bbox emitido 200x100, exportar PDF e conferir extensão horizontal de 200pt e diferença do mesmo Shape fora do bloco. Não inventar bbox manual como entrada da prova. Conservar o nome e todos os controles adjacentes/expectativas. Esta cláusula supersede preservação literal somente desse smoke test vazio de observação produtiva; não permite retirada de testes de bbox.',
'03_infra/src/watch.rs': 'Retirar somente o campo privado redundante BundleHttpServer.live e sua inicialização. O parâmetro público new(..., live) e o live capturado pelo worker e passado a serve_connection permanecem idênticos. bool não possui Drop/recursos causais; aquisição/HTTP/SSE/injeção, API e defaults não mudam.',
'03_infra/src/plugin_host/tests.rs': 'Retirar somente a constante privada I64 sem referências do encoder de fixtures WASM. Preservar bytes/opcodes das fixtures e todos os testes, inclusive recusas de assinatura; não alegar que I64 não seja suportado pelo produto.',
}
for number in ['1467','1468','1469']:
 p=next(Path('04_wiring/tests').glob(f'p{number}_*.rs'))
 plans[str(p)]='Preservar ProcessResult.stdout e sua captura; incluir stdout e stderr nos diagnósticos das assertions de sucesso existentes. Não usar Debug derivado como pretexto de leitura e não trocar verificações canônicas de stderr, status, PDF/precedência/fixtures, subprocessos ou timeouts. Esta leitura útil resolve dead_code sem supressão ou perda de informação de falha.'
files={};owners={}
for p in plans:
 owner=re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1);owners[p]=owner
 for f in [p,owner]:files[f]=dict(sha256=sha(f),text=Path(f).read_text())
all_hashes={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
cases=[dict(rule=c['code'],source=c['spans'][0]['file_name'],line=c['spans'][0]['line_start'],message=c['message']) for c in json.loads((prior/'clippy-inventory.json').read_text()) if c['spans'][0]['file_name'] in plans and c['code'] in ['clippy::derivable_impls','clippy::map_entry','clippy::items_after_test_module','clippy::unnecessary_literal_unwrap','dead_code']]
assert len(cases)==12,cases
before=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),all_source_hashes=all_hashes,files=files,cases=cases,plans=plans,owners=owners)
assert not (out/'before.json.gz').exists()
with gzip.open(out/'before.json.gz','wt') as f:json.dump(before,f)
for name in ['run.py','seal.py','seal_owned.py','suite.py','validate.py','monitor.py']:
 s=(prior/'workers'/name).read_text().replace('clippy-default-2026-10-16','clippy-locais-2026-10-16').replace('TEKT_DEFAULT_MEMORY_LIMIT','TEKT_LOCAIS_MEMORY_LIMIT')
 if name=='validate.py':s=s.replace('clippy-expressions-2026-10-16/clippy-inventory.json','clippy-default-2026-10-16/clippy-inventory.json')
 (out/'workers'/name).write_text(s)
for p,decision in plans.items():
 owner=Path(owners[p]);sources=[c for c in cases if c['source']==p]
 lines=', '.join(str(c['line']) for c in sources)
 text=f'\n\n## Manutenção local e observação de testes — 2026-10-16\n\n### Medição anterior à decisão\n\n`{p}` nas linhas {lines} contém os avisos registrados em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/before.json.gz`. O snapshot conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A inspeção dos consumidores e usos precedeu esta decisão; o gate Clippy selecionado deve falhar antes do código. Manutenção de representação/observação Rust, sem nova medição de intenção upstream ou paridade.\n\n### Responsabilidade e aceitação\n\n{decision}\n\nFluxo contínuo ADR-0127: sem contrato público, default de produto ou fase novos. L0 precede código e resselo restrito. Nenhuma supressão de lint nova. Conferir redução exata do inventário e ausência de avisos novos, gate Clippy RED→GREEN e unfulfilled_lint_expectations como erro; executar suíte workspace, build, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job. Não criar testes que apenas espelhem derive, Entry ou posicionamento de módulos. O caso de bbox exige observação real conforme especificado; os demais controles existentes mantêm fixtures e expectativas.\n'
 owner.write_text(owner.read_text()+text)
(out/'l0-before-code.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),owners={p:sha(p) for p in owners.values()},sources_still_original=all(sha(p)==files[p]['sha256'] for p in plans)),indent=2)+'\n')
print(json.dumps(dict(cases=len(cases),consumers=len(plans),owners=len(set(owners.values())))))
