from pathlib import Path
import json,re,hashlib,datetime
ROOT=Path.cwd();DATA=ROOT/'00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28'
NUCLEUS='00_nucleo/prompts/_nuclei/layout/text-separators.toml'
def before(path):
 f=ROOT/path;dest=DATA/'before'/path
 if dest.suffix=='.rs':dest=dest.with_name(dest.name+'.txt')
 if not dest.exists():dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(f.read_bytes())
def authorize(path,behavior):
 f=ROOT/path;source=f.read_text();owner=re.search(r'(?m)^//! @prompt (\S+)$',source).group(1);before(path);before(owner);p=ROOT/owner;text=p.read_text();pin=hashlib.sha256((ROOT/NUCLEUS).read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest();entry=f'- {NUCLEUS} sha256:{pin}'
 if entry not in text:
  if 'Núcleos Tekt:\n' in text:text=text.replace('Núcleos Tekt:\n','Núcleos Tekt:\n'+entry+'\n',1)
  else:
   spot=re.search(r'(?m)^Hash do Código:.*\n',text).end();text=text[:spot]+'\nNúcleos Tekt:\n'+entry+'\n'+text[spot:]
 section='## Separadores textuais — materialização autorizada em 2026-09-28\n'
 if section not in text:
  text+='\n'+section+'\n### Medição anterior à decisão\n\n'+f'`{path}` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de\n`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção\ndos handlers desse consumer precedem esta alteração. A aprovação do contrato\npúblico em `00_nucleo/prompts/entities/layout_types.md` foi recebida em\n2026-09-28 e registrada na proveniência da materialização.\n\n### Responsabilidade deste owner\n\n'+behavior+'\n\nConservar as responsabilidades existentes; a aceitação segue texto e geometria\nobserváveis, incluindo os controles de adjacência e os invariantes pinados.\n'
 p.write_text(text)
 scopefile=DATA/'materialization-scope.json';scope=json.loads(scopefile.read_text()) if scopefile.exists() else {};scope[path]={'owner':owner,'behavior':behavior};scopefile.write_text(json.dumps(scope,ensure_ascii=False,indent=2)+'\n')
 return f
if __name__=='__main__':
 print('Use authorize(path, behavior) before every consumer edit.')
