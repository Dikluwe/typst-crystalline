from pathlib import Path
import json,re,hashlib,gzip,datetime,shutil
root=Path('/tmp/tekt-linter-none-src');out=Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16')
before=json.load(gzip.open(out/'before.json.gz'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
log=(out/'green-doc.log').read_text();sources=sorted(set(re.findall(r'--> ([^:]+\.rs):',log)))
assert len(sources)==11
changes={
'01_core/contracts/prompt_provider.rs':{'[orphan_exceptions]':'`[orphan_exceptions]`'},
'01_core/entities/parsed_file.rs':{'[layers]':'`[layers]`','[l1_ports]':'`[l1_ports]`','[wiring_exceptions]':'`[wiring_exceptions]`'},
'01_core/entities/rule_traits.rs':{'// ref: <caminho>:<linha>':'`// ref: <caminho>:<linha>`','// spec: <norma> §<secção>':'`// spec: <norma> §<secção>`','// rationale: <frase>':'`// rationale: <frase>`'},
'01_core/rules/pub_leak.rs':{'[l1_ports]':'`[l1_ports]`'},
'01_core/rules/wiring_logic_leak.rs':{'[rules]':'`[rules]`'},
'02_shell/fix_hashes.rs':{'"Hash do Código: <hash>"':'`Hash do Código: <hash>`'},
'02_shell/n16_summary.rs':{'N16[α]':'`N16[α]`','N16[β]':'`N16[β]`','N16[γ]':'`N16[γ]`'},
'02_shell/refinement.rs':{'[[observable]]':'`[[observable]]`'},
'03_infra/hash_writer.rs':{'"Hash do Código: <hash>"':'`Hash do Código: <hash>`'},
'03_infra/prompt_walker.rs':{'[orphan_exceptions]':'`[orphan_exceptions]`','Box<str>':'`Box<str>`'},
'03_infra/walker.rs':{'[layers]':'`[layers]`'}}
assert sorted(changes)==sources
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sources':{},'diagnostic_log_sha256':sha(out/'green-doc.log'),'base_commit':before['base_commit']}
for source in sources:
 text=(root/source).read_text();assert sha(root/source)==before['linter_hashes'][source],source
 owner=re.search(r'^//! @prompt (.+)$',text,re.M).group(1)
 assert sha(root/owner)==before['linter_hashes'][owner],owner
 lines=[int(m[1]) for m in re.findall(r'--> ([^:]+\.rs):(\d+):',log) if m[0]==source]
 record['sources'][source]={'owner':owner,'before':text,'source_sha256':sha(root/source),'owner_before':(root/owner).read_text(),'lines':lines,'replacements':changes[source]}
 # L0 first: each source retains its sole owner and public/runtime contract.
 with (root/owner).open('a') as f:
  f.write('\n## Rustdoc estrito — 2026-10-16\n\nMedição anterior à correção: cargo doc --offline --locked --no-deps com RUSTDOCFLAGS="-D warnings" falhou neste consumer nas linhas '+', '.join(map(str,lines))+'. O diagnóstico tekt-v16-none-parser-2026-10-16 em typst-crystalline conserva green-doc.log (ensaio anterior), hashes base e53de6f e UTC em docs-before-code.json. Corrigir somente a apresentação dos comentários Rustdoc: representar como código literal '+', '.join('`'+x+'`' for x in changes[source])+'. Não alterar tokens executáveis, tipos, campos, assinaturas, mensagens de execução, ordem ou comportamento. É manutenção documental interna em fluxo contínuo; o RED é a compilação Rustdoc estrita registrada e o GREEN exige cargo doc sem advertências, mais suíte, Clippy e linhagem existentes. Resselar os dois sentidos somente dos owners editados.\n')
(out/'docs-before-code.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
for source,item in record['sources'].items():
 lines=item['before'].splitlines(keepends=True)
 for old,new in item['replacements'].items():
  n=0
  for i,line in enumerate(lines):
   if line.lstrip().startswith('///') and old in line:
    assert new not in line,(source,line)
    lines[i]=line.replace(old,new);n+=1
  assert n>0,(source,old)
 text=''.join(lines)
 norm=lambda s:'\n'.join(line for line in s.splitlines() if not line.lstrip().startswith(('///','//!')))
 assert norm(text)==norm(item['before'])
 (root/source).write_text(text)
for p in sorted(out.glob('green-*')):
 if p.is_file():shutil.move(p,out/('attempt-doc-'+p.name))
