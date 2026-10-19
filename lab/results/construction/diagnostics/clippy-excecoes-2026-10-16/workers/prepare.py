from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,re,collections
out=Path('00_nucleo/diagnosticos/clippy-excecoes-2026-10-16');prior=Path('00_nucleo/diagnosticos/clippy-locais-2026-10-16');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
validation=json.loads((prior/'validation-details.json').read_text());assert all(sha(p)==h for p,h in validation['source_hashes'].items())
inventory=json.loads((prior/'clippy-inventory.json').read_text())
classification=json.loads(Path('00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json').read_text())
exempt=[x for x in classification if x['decision']=='excecao-justificada'];assert len(exempt)==46
cases=[];plans=[];owners={};files={}
for x in exempt:
 source=Path(x['path']);owner=Path(x['owner']);text=owner.read_text()
 assert x['prescription'] in text and x['suppression']=='expect-local-com-reason',x['id']
 candidates=[]
 for c in inventory:
  if c['code']!='clippy::too_many_arguments' or c['spans'][0]['file_name']!=str(source):continue
  span=c['spans'][0];fragment=source.read_bytes()[span['byte_start']:span['byte_end']].decode()
  if re.search(r'\bfn '+re.escape(x['function'])+r'\b',fragment):candidates.append(c)
 assert len(candidates)==1,x['id']
 c=candidates[0];assert str(x['arity']) in c['message'],(x['id'],x['arity'],c['message'])
 attr=re.search(r'#\[expect\(clippy::too_many_arguments, reason=("(?:\\.|[^"\\])*")\)\]',x['prescription']).group(1)
 reason=json.loads(attr);p=dict(id=x['id'],source=str(source),owner=str(owner),function=x['function'],rule='clippy::too_many_arguments',reason=reason,rationale=x['rationale'],signature_now=source.read_bytes()[c['spans'][0]['byte_start']:c['spans'][0]['byte_end']].decode(),span=c['spans'][0],existing_prescription=x['prescription'])
 plans.append(p);cases.append(dict(id=x['id'],rule=c['code'],source=str(source),line=c['spans'][0]['line_start'],function=x['function'],reason=reason))
for source,fn,rule,reason in [
('01_core/src/entities/counter.rs','counter_key_str_e_selector_sao_distintos','clippy::mutable_key_type','CounterKey deste teste usa apenas Str e Selector::Kind imutáveis; o HashMap verifica namespaces distintos'),
('03_infra/src/fonts/coverage.rs','alias_memoization_respects_invalid_and_nonunicode_records','clippy::single_range_in_vec_init','Esperados são coleções de Range semiabertos de cobertura, preservando o intervalo único 2..6'),
('03_infra/src/fonts/coverage.rs','checked_native_reader_rejects_truncation_after_a_partial_group','clippy::single_range_in_vec_init','Fixture e esperado guardam um Range 40..42 para verificar append parcial e rollback de intervalos')]:
 owner=re.search(r'^//! @prompt (.+)$',Path(source).read_text(),re.M).group(1)
 plans.append(dict(id='COUNTER' if 'counter.rs' in source else 'RANGE-'+fn,source=source,owner=owner,function=fn,rule=rule,reason=reason))
 for c in inventory:
  if c['code']==rule and c['spans'][0]['file_name']==source:
   line=c['spans'][0]['line_start'];s=Path(source).read_text();fn_at=s.rfind('fn ',0,len('\n'.join(s.splitlines()[:line-1])))
   name=re.match(r'fn (\w+)',s[fn_at:]).group(1)
   if name==fn:cases.append(dict(id=plans[-1]['id'],rule=rule,source=source,line=line,function=fn,reason=reason))
assert len(cases)==51 and len(plans)==49,(len(cases),len(plans))
for p in plans:
 owners[p['source']]=p['owner']
 for f in [p['source'],p['owner']]:files[f]=dict(text=Path(f).read_text(),sha256=sha(f))
all_hashes={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
before=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,all_source_hashes=all_hashes,cases=cases,plans=plans,owners=owners,pending_corrections=[x for x in classification if x['decision']=='corrigir'])
assert not (out/'before.json.gz').exists()
with gzip.open(out/'before.json.gz','wt') as f:json.dump(before,f)
for name in ['run.py','seal.py','seal_owned.py','monitor.py']:
 s=(prior/'workers'/name).read_text().replace('clippy-locais-2026-10-16','clippy-excecoes-2026-10-16').replace('TEKT_LOCAIS_MEMORY_LIMIT','TEKT_EXCECOES_MEMORY_LIMIT');(out/'workers'/name).write_text(s)
for source,owner in owners.items():
 ps=[p for p in plans if p['source']==source];ids=', '.join(p['id'] for p in ps);spec=[]
 if any(p['rule']=='clippy::too_many_arguments' for p in ps):
  spec.append('Materializar somente as expectativas diretas nominais já justificadas neste owner: '+ids+'. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.')
 if source.endswith('/counter.rs'):
  spec.append('No teste counter_key_str_e_selector_sao_distintos, as únicas chaves efetivamente construídas são CounterKey::Str("heading") e Selector::Kind(Heading). Não possuem payload mutável na identidade; manter o HashMap, os dois inserts, len=2 e os dois gets. Autorizar somente #[expect(clippy::mutable_key_type, reason="CounterKey deste teste usa apenas Str e Selector::Kind imutáveis; o HashMap verifica namespaces distintos")] neste teste. Isto não declara segura a chave geral Selector/Value nem valida Eq para NaN. Uso de outra variante ou alteração das chaves reabre a decisão e exige teste de estabilidade de hash/igualdade antes de aceitar.')
 if source.endswith('/coverage.rs'):
  spec.append('Nos testes alias_memoization_respects_invalid_and_nonunicode_records e checked_native_reader_rejects_truncation_after_a_partial_group, os arrays/Vec são coleções de Range<u32>, não coleções dos inteiros abrangidos. Manter literalmente [2..6], vec![40..42], [40..42], seus limites, checkpoint, append parcial e rollback. Autorizar #[expect(clippy::single_range_in_vec_init, reason="motivo nominal de intervalo")] somente nesses dois testes, com os motivos completos registrados no plano desta etapa. A primeira anotação cobre os dois esperados [2..6]; a segunda, a fixture e esperado [40..42]. Não trocar tipos, enumerar pontos, criar helper de construção apenas para evitar o lint, modificar a extração de cobertura ou suprimir no módulo. Se deixar de existir a coleção de um intervalo, rever/remover a expectativa; unfulfilled_lint_expectations continua erro.')
 appendix='\n\n## Exceções nominais materializadas — 2026-10-16\n\n### Medição anterior à decisão\n\nO inventário validado em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `00_nucleo/diagnosticos/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.\n\n### Responsabilidade e aceitação\n\n'+'\n\n'.join(spec)+'\n\nFluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.\n'
 if source.endswith('/coverage.rs'):
  appendix+='\nPrescrições literais deste owner:\n\n'+''.join(f'- `{p["function"]}`: `#[expect({p["rule"]}, reason = {json.dumps(p["reason"],ensure_ascii=False)})]`.\n' for p in ps)
 Path(owner).write_text(Path(owner).read_text()+appendix)
(out/'l0-before-code.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),owners={p:sha(p) for p in owners.values()},sources_still_original=all(sha(p)==files[p]['sha256'] for p in owners)),indent=2)+'\n')
print(json.dumps(dict(cases=len(cases),attributes=len(plans),owners=len(owners),pending_arity=11)))
