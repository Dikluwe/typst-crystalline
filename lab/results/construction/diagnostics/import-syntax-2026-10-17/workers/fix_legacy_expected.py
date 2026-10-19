from pathlib import Path
import json,gzip,hashlib,datetime,re,shutil
out=Path(__file__).resolve().parents[1];assert json.load(open(out/'legacy-module-probe.json'))['sources_unchanged'];v=json.load(open(out/'validation-result.json'));assert v['commands'][-1]['label']=='workspace' and v['commands'][-1]['exit']==101
b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();source='01_core/src/compiler/eval/tests.rs';owner='00_nucleo/prompts/compiler/eval/tests.md';assert all(sha(p)==b['hashes'][p] for p in [source,owner]);extension=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files={p:Path(p).read_text() for p in [source,owner]},hashes={p:sha(p) for p in [source,owner]})
with gzip.open(out/'scope-extension.json.gz','wt') as f:json.dump(extension,f)
archive=out/'legacy-expected-attempt';archive.mkdir(exist_ok=False)
for p in list(out.glob('validation-*.log'))+list(out.glob('*-resources.json'))+[out/'validation-result.json',out/'strict-details.json',out/'strict-clippy.log',out/'architecture-delta.json',out/'integrity-result.json',out/'after-probe.json.gz',out/'after-context-probe.json.gz',out/'observables-result.json',out/'differential-review.json']:
 if p.exists():shutil.copy2(p,archive/p.name)
for name in ['workspace','after-tekt-bundle','after-vanilla-bundle']:
 if (out/name).exists():(out/name).rename(archive/name)
scope=json.load(open(out/'scope.json'));scope['sources'].append(source);scope['owners'][source]=owner;(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
p=Path(owner);s=p.read_text();nucleus='00_nucleo/prompts/_nuclei/eval/source-validation.toml';effective=hashlib.sha256(Path(nucleus).read_bytes()+b'\x00TEKT-NUCLEUS-DEPS-V1\x00').hexdigest();needle='Núcleos Tekt:\n';assert s.count(needle)==1;start=s.index(needle)+len(needle);end=s.find('\n\n',start);pins=s[start:end].splitlines();pins.append('- '+nucleus+' sha256:'+effective);s=s[:start]+'\n'.join(sorted(pins))+s[end:]
s+='''

## Import/include — migração medida de expectativa P1306 (2026-10-17)

### Medição anterior à decisão

00_nucleo/diagnosticos/import-syntax-2026-10-17/legacy-expected-attempt/workspace/workspace.log
registra a falha isolada de p1306_named_module_contract_all_profiles em HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado e fontes
congeladas no validation-result.json daquele diretório. O teste exigia trace
vazio para imported-error, embora o erro surja em routes/error.typ. A fixture
exata está em legacy-module-probe/: import do arquivo, inner.absent, mensagem
module `std` does not contain `absent`, origem 59..65. legacy-module-probe.json
registra quatro perfis, UTC, hashes de fontes/binários/fixtures e stdout/stderr.
No pin 9f2b6e8715237cb086899a42873660fe744622e8 e no Tekt corrigido, o erro tem
Tracepoint::Import("routes/error.typ") na expressão importadora, range 2..36,
com mensagem/hints/origem iguais. Descontou-se apenas o literal de aviso de
arquivo vazio já conhecido; stderr restante idêntico. A regra é diagnóstico
observável da linguagem, não equivalência mecânica de Rust. O pressuposto de
trace vazio foi refutado pela sonda; não inferir intenção do teste antigo.

### Responsabilidade e aceitação

Migrar somente a expectativa de trace em p1306_named_module_contract_all_profiles.
Construir a Source da expressão uma vez e usá-la para conferir a origem do
trace. imported-error exige exatamente um Tracepoint::Import com caminho
routes/error.typ e range 2..36 da expressão. Todos os demais casos continuam
exigindo trace vazio. Conservar contagem unitária de erro, Severity::Error,
mensagem, hints vazios, origem/range do erro, side vazio, quatro perfis, ordem
normal/inversa e controles existentes. Não retirar assertivas para acomodar a
mudança. Sem novo teste público, supressão, lógica produtiva ou contrato Rust.
A política compartilhada vem do núcleo pinado source-validation e do owner
00_nucleo/prompts/compiler/eval/modules.md. Conferir teste legado focal,
regressão completa, Clippy/Rustdoc estritos, linhagem e SARIF diferencial no
novo source freeze sob MemoryMax12GiB/High10GiB/swap0/jobs1.
''';p.write_text(s)
# L0 is saved before the test expectation changes.
p=Path(source);s=p.read_text();a=s.index('        fn p1306_named_module_contract_all_profiles()');z=s.index('\n        #[test]',a);part=s[a:z];old='''                        let source = if imported {''';assert part.count(old)==1;part=part.replace(old,'''                        let expression_source = Source::new_with_parser(
                            world.main(),
                            expression.to_string(),
                            crate::compiler::parse::parse_code,
                        );
                        let source = if imported {''',1)
old='''                            Source::new_with_parser(
                                world.main(),
                                expression.to_string(),
                                crate::compiler::parse::parse_code,
                            )''';assert part.count(old)==1;part=part.replace(old,'                            expression_source.clone()',1)
old='''                        let range = source.span_byte_range(error.span);''';assert part.count(old)==1;part=part.replace(old,old+'''
                        let expected_trace_len = usize::from(imported);
                        let trace_matches = error.trace.len() == expected_trace_len
                            && error.trace.iter().all(|point| {
                                point.v == crate::entities::source_result::Tracepoint::Import("routes/error.typ".into())
                                    && expression_source.span_byte_range(point.span) == Some(2..36)
                            });''',1)
part=part.replace('|| !error.trace.is_empty()','|| !trace_matches',1).replace('hints/trace empty, source','hints empty, trace len {expected_trace_len} with import path/span when present, source',1);s=s[:a]+part+s[z:];p.write_text(s)
# Extend preserved-before views without rewriting the original baseline artifact.
for name in ['verify.py','finalize.py']:
 p=out/'workers'/name;s=p.read_text();needle="before=json.load(gzip.open(out/'before.json.gz'));" if name=='verify.py' else "b=json.load(gzip.open(out/'before.json.gz'));";var='before' if name=='verify.py' else 'b';assert needle in s;s=s.replace(needle,needle+var+"['files'].update(json.load(gzip.open(out/'scope-extension.json.gz'))['files']);",1);p.write_text(s)
# Reuse the same public-contract extractor for the newly included existing source.
s=(out/'workers/verify.py').read_text();a=s.index('def contract(');z=s.index('\ndef public_types',a);ns={};exec('import re\n'+s[a:z],ns);contracts=json.load(open(out/'public-contract-before.json'));contracts[source]=ns['contract'](extension['files'][source]);(out/'public-contract-before.json').write_text(json.dumps(contracts,indent=2)+'\n')
p=out/'workers/validate.py';s=p.read_text();s=s.replace("('fmt',['cargo'", "('legacy-green',['cargo','test','--offline','--locked','-p','typst-core','p1306_named_module_contract_all_profiles','--','--test-threads=1'],0),('fmt',['cargo'",1);p.write_text(s)
p=out/'workers/finalize.py';s=p.read_text().replace('Três pares consumer/L0: compiler/eval/mod.rs, modules.rs e novo syntax_diagnostics.rs.', 'Quatro pares consumer/L0: compiler/eval/mod.rs, modules.rs, novo syntax_diagnostics.rs e tests.rs para a expectativa legada.').replace('pinado nos três owners.', 'pinado nos quatro owners.');s=s.replace('A comparação inicial e o primeiro diferencial', 'A primeira suíte completa encontrou uma expectativa antiga de trace vazio. A fixture exata confirmou no pin, nos quatro perfis, que ela deve exigir Import("routes/error.typ") com span 2..36 da expressão. A expectativa foi atualizada preservando todas as demais assertivas e passou a conferir também caminho e span do trace. O L0 tests.md foi atualizado antes do teste; scope-extension.json.gz registra os textos/hashes originais. A primeira execução e seu source freeze estão em legacy-expected-attempt.\n\nA comparação inicial e o primeiro diferencial');p.write_text(s)
print('Legacy expectation migrated only after exact pinned probe; four owners now in scope')
