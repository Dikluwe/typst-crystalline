from pathlib import Path
import json,gzip,hashlib,datetime,shutil,re,subprocess
out=Path('00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
archive=out/'first-validation';archive.mkdir(exist_ok=False)
for p in out.glob('validation*'):
 if p.is_file():shutil.move(str(p),archive/p.name)
shutil.copyfile(out/'before.json.gz',out/'initial-before.json.gz');before=json.load(gzip.open(out/'before.json.gz'))
source='03_infra/src/export/tests/mod.rs';owner='00_nucleo/prompts/infra/export/tests/mod.md'
for p in [source,owner]:assert sha(p)==before['all_source_hashes'][p],p;before['files'][p]=dict(text=Path(p).read_text(),sha256=sha(p))
plan='Migrar as cinco chamadas de fixtures aos antigos PageContext::cidfont/multifont para with_font_scenario e FontScenario existente nomeado. Preservar, na mesma ordem, todas as expressões dos quatro recursos, mapas, fontes, bitmap, modo e Box::leak já existentes nas fixtures; não limpar ou mudar lifetime neste passo. CIDFont mantém bitmap None; Multifont mantém bindings Some(LookupCache::default()) uma vez por contexto. Adicionar somente import privado FontScenario. Manter nomes, fixtures, asserts, strings e números esperados integralmente. Não restaurar wrappers antigos nem silenciar erro de compilação. As chamadas são internas ao módulo de testes, sem API de produto nova.'
before['owners'][source]=owner;before['plans'][source]=plan;before['scope_extension']=dict(utc=now(),source=source,owner=owner,matched_original_global_hashes=True,reason='Cinco chamadores cfg(test) não incluídos na pesquisa inicial; E0599 em all-targets.')
with gzip.open(out/'before.json.gz','wt') as f:json.dump(before,f)
Path(owner).write_text(Path(owner).read_text()+'\n\n## Migração das fixtures de PageContext — 2026-10-16\n\n### Medição anterior à decisão\n\nA primeira compilação all-targets em `00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16/first-validation/validation-clippy.log` encontrou cinco E0599 neste consumer (linhas 8166, 8229, 8309, 9753 e 9947), apesar do build produtivo aprovado. O arquivo e este L0 conferem com os hashes globais do snapshot inicial; conteúdos antes da edição e extensão de escopo estão preservados no diagnóstico. Os chamadores e cada expressão foram lidos antes desta decisão.\n\n### Responsabilidade e aceitação\n\n'+plan+'\n\nFluxo contínuo ADR-0127: L0 antes do código e resselo restrito deste owner/consumer. Conferir cada argumento antigo contra o campo novo e equivalência do restante do arquivo após mascarar exclusivamente as chamadas migradas/import/hash. Reexecutar compilação all-targets, suíte workspace completa e gates da etapa com 12 GiB, swap zero e um job. Não alterar esperados para obter GREEN nem incluir testes que espelhem o carrier.\n')
record=dict(utc=now(),owner_updated_before_code_sha256=sha(owner),source_original_sha256=sha(source),initial_before_sha256=sha(out/'initial-before.json.gz'),scope_extension=before['scope_extension']);s=Path(source).read_text();calls=[]
def split_args(s):
 args=[];start=0;depth=0
 for i,c in enumerate(s):
  if c in '([{':depth+=1
  elif c in ')]}':depth-=1
  elif c==',' and depth==0:
   if s[start:i].strip():args.append(s[start:i].strip())
   start=i+1
 if s[start:].strip():args.append(s[start:].strip())
 return args
matches=list(re.finditer(r'PageContext::(cidfont|multifont)\(',s));assert len(matches)==5
for m in reversed(matches):
 opening=m.end()-1;depth=1;end=opening+1
 while depth:depth+=int(s[end]=='(')-int(s[end]==')');end+=1
 args=split_args(s[opening+1:end-1]);kind=m[1];assert len(args)==(9 if kind=='cidfont' else 11),(kind,args)
 fields=['char_to_gid','glyph_mapping','glyph_to_nominal','bitmap'] if kind=='cidfont' else ['fonts','per_font_char_to_gid','per_font_glyph_mapping','per_font_glyph_to_nominal','per_font_glyph_reverse','per_font_bitmap']
 variant='Cidfont' if kind=='cidfont' else 'Multifont';named=',\n'.join(f'{name}: {value}' for name,value in zip(fields,args[4:-1]))+',\n'
 if kind=='multifont':named+='bindings: Some(crate::export::font_binding::LookupCache::default()),\n'
 new='PageContext::with_font_scenario(\n'+',\n'.join(args[:4])+',\nFontScenario::'+variant+' {\n'+named+'},\n'+args[-1]+',\n)'
 calls.append(dict(kind=kind,old_call=s[m.start():end],old_arguments=args,new_call_unformatted=new));s=s[:m.start()]+new+s[end:]
assert s.count('use super::*;')>=1;s=s.replace('use super::*;','use super::*;\nuse crate::export::stream::FontScenario;',1);Path(source).write_text(s);subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',source],check=True)
record.update(utc_end=now(),source_after_sha256=sha(source),calls=list(reversed(calls)));(out/'test-caller-migration.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
p=out/'workers/finalize.py';s=p.read_text().replace('em sete consumers','em oito consumers').replace('os sete L0','os oito L0');s=s.replace('## RAM e proveniência',"A primeira compilação all-targets detectou cinco fixtures do agregador ainda chamando os construtores retirados. A busca inicial foi incompleta. O owner foi incluído após conferir seus hashes contra o snapshot global original, e seu L0 foi atualizado antes de migrar cada chamada, mantendo todos os argumentos/esperados. A falha original, o snapshot inicial e a extensão de escopo foram preservados; os wrappers antigos não foram restaurados para esconder a migração. {link('test-caller-migration.json')} · {link('initial-before.json.gz')} · {link('first-validation/validation-details.json')}.\n\n## RAM e proveniência")
p.write_text(s);compile(s,str(p),'exec')
print('L0 atualizado antes da migração dos cinco chamadores; escopo agora inclui oito consumers.')
