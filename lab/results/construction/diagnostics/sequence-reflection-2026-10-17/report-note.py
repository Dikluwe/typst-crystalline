from pathlib import Path
import hashlib,json,gzip
p=Path(__file__).resolve().parent
f=p/'relatorio.md';text=f.read_text()
paragraph='''
A única nota nova é V19 em `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`: o braço `Content::Empty | Content::Sequence(_)` compartilha o schema children. O L0 proprietário, seção R1, prescreve esse agrupamento porque ambos expõem o mesmo campo requerido; os testes exercitam Empty, Sequence com um filho e Sequence com vários filhos. A nota foi mantida, sem allow/expect ou supressão. A contagem V19 passou de 1106 para 1107; V16 permaneceu 106 e V20 permaneceu 874.
'''
if paragraph not in text:
 text=text.replace('\n## Pendências explícitas\n',paragraph+'\n## Pendências explícitas\n');f.write_text(text)
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
freeze=json.loads(gzip.decompress((p/'complete-freeze.json.gz').read_bytes()))
assert all(sha(f)==h for f,h in freeze['sources'].items())
files=sorted(f for f in p.iterdir() if f.is_file() and (f.suffix in ['.py','.md'] or f.name.endswith('.json.gz') or f.name.endswith('.patch.gz') or f.name=='.gitignore'))
(p/'manifest.sha256').write_text('\n'.join(f'{sha(f)}  {f.name}' for f in files)+'\n')
print('Relatório complementado com a justificativa V19; fontes validadas intactas.')
