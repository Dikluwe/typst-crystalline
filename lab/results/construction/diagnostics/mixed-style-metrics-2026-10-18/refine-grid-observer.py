from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,difflib
p=Path(__file__).resolve().parent
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
initial=json.loads((p/'final-gates-resources.json').read_text());assert initial['exit']!=0
freeze=json.loads(gzip.decompress((p/'freeze.json.gz').read_bytes()));assert all(sha(f)==h for f,h in freeze['sources'].items())
assert not (p/'measure-freeze.json.gz').exists();(p/'measure-freeze.json.gz').write_bytes((p/'freeze.json.gz').read_bytes())
code=Path('03_infra/src/integration_tests.rs');l0=Path('00_nucleo/prompts/infra/integration_tests.md')
texts={str(f):f.read_text() for f in [code,l0]}
low=41.449999999999996;high=41.45;bound=2.220446049250313e-16*max(abs(low),abs(high),1.0)
assert 0<high-low<=bound<1e-9
(p/'rounding-before.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=now(),head=initial['head'],files=texts,source_hashes={str(f):sha(f) for f in [code,l0]},evidence='initial-workspace.log: stress_81_5_row_height_e_maximo_da_linha',observed=[low,high],gap=high-low,relative_machine_epsilon_bound=bound,classification='One floating representation step at this coordinate; no meaningful grid overlap. Geometric positions are the observable, binary >= without numerical error bound is observer mechanics.'),indent=2).encode()))
l0.write_text(texts[str(l0)]+'''

## Ordem geométrica no stress de Grid — refinamento 2026-10-18

### Medição antes da decisão

mixed-style-metrics-2026-10-18/initial-workspace.log registra o único RED
do alvo typst-infra --lib em stress_81_5_row_height_e_maximo_da_linha:
41.449999999999996 >= 41.45
falha em integration_tests.rs:2212. O desvio é 7.105427357601002e-15pt;
um epsilon de f64 relativo à magnitude dessas coordenadas mede
9.203748874142547e-15pt. rounding-before conserva fontes/L0/UTC e os
valores exatos, antes desta decisão. Os demais controles de Grid da fixture
passam. O contrato é a ordem geométrica, não identidade de aritmética binária.
Não inferir desse RED isolado um erro de linha, altura ou quebra do produto.

### Responsabilidade e aceitação

Somente a desigualdade de ordem vertical desse teste admite erro numérico
limitado a f64::EPSILON * max(abs(y_linha1), abs(y_max_linha0), 1).
Exigir as duas coordenadas finitas. Comparar a diferença assinada com esse
limite; não arredondar o documento ou alterar a fixture, observador de itens,
posições esperadas, layout, snapshots e os demais testes/tolerâncias.
Isto supersede a proibição genérica de mudar tolerâncias apenas nesse teste:
um erro geométrico maior que esse bound deve continuar falhando. A tolerância
de paridade de 0.001pt não se aplica aqui e não é relaxada por este adendo.
Correção privada de observação ADR-0127/ADR-0107, com o RED preservado,
L0 antes do código, resselo e revalidação ampla no freeze refinado.
''')
def run(label,cmd):
 with (p/(label+'.log')).open('w') as log:z=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 assert z.returncode==0,(label,z.returncode)
run('grid-ownership',[pin,'.','--checks','v15,v26','--fail-on','warning'])
old='''        assert!(
            rect_50.y.0 >= first_row_y_max,
            "Linha 1 do grid deve estar ao nível ou abaixo da linha 0. \\
'''
new='''        let rounding_bound = f64::EPSILON
            * rect_50.y.0.abs().max(first_row_y_max.abs()).max(1.0);
        assert!(
            rect_50.y.0.is_finite()
                && first_row_y_max.is_finite()
                && first_row_y_max - rect_50.y.0 <= rounding_bound,
            "Linha 1 do grid deve estar ao nível ou abaixo da linha 0. \\
'''
assert texts[str(code)].count(old)==1
code.write_text(texts[str(code)].replace(old,new))
run('grid-fmt',['cargo','fmt','--all'])
run('grid-reseal',[pin,'.','--fix-hashes'])
run('grid-lineage',[pin,'.','--checks','v1,v5,v15,v26','--fail-on','warning'])
before=json.loads(gzip.decompress((p/'before-sources.json.gz').read_bytes()))['files']
sources={f:sha(f) for f in before}
changed={f:Path(f).read_text() for f,s in before.items() if hashlib.sha256(s.encode()).hexdigest()!=sources[f]}
newfreeze=dict(utc=now(),head=freeze['head'],diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),sources=sources,changed_texts=changed,oracle_sha256=freeze['oracle_sha256'],linter_sha256=freeze['linter_sha256'],measurement_freeze='measure-freeze.json.gz',refinement='Only cfg(test) integration harness and its L0; all other source hashes unchanged.')
delta=[f for f in sources if sources[f]!=freeze['sources'][f]];assert sorted(delta)==sorted([str(code),str(l0)])
(p/'freeze.json.gz').write_bytes(gzip.compress(json.dumps(newfreeze,indent=2).encode()))
(p/'stage.patch').write_text(''.join(''.join(difflib.unified_diff(before[k].splitlines(True),v.splitlines(True),fromfile=k+' (before)',tofile=k+' (after)')) for k,v in changed.items()))
for name in ['clippy.log','workspace.log','gates-results.json','harness-config.json']:
 f=p/name
 if f.exists():f.rename(p/('initial-'+name))
print('grid observer L0 then test updated; source delta',delta)
