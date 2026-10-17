from pathlib import Path
import json,gzip,datetime,hashlib,subprocess
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));new={
'01_core/src/compiler/stdlib/structural/math/accent.rs':'00_nucleo/prompts/compiler/stdlib/structural/math/accent.md',
'01_core/src/compiler/stdlib/structural/math/lr.rs':'00_nucleo/prompts/compiler/stdlib/structural/math/lr.md',
'01_core/src/compiler/stdlib/structural/mod.rs':'00_nucleo/prompts/compiler/stdlib/structural.md'}
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
for source,owner in new.items():
 for p in [source,owner]:assert sha(p)==b['hashes'][p];b['files'][p]=Path(p).read_text()
 scope['sources'].append(source);scope['owners'][source]=owner
with gzip.open(out/'working-baseline.json.gz','wt') as f:json.dump(b,f)
(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
pin='9f2b6e8715237cb086899a42873660fe744622e8';lr=subprocess.check_output(['git','show',pin+':crates/typst-library/src/math/lr.rs'],text=True);part=lr.split('const DELIMS:',1)[1].split('\n];',1)[0]
import re
starts=['⌈','⌊']+re.findall(r"^    \('(.)',",part,re.M);assert len(starts)==len(set(starts));(out/'lr-symbol-starts.json').write_text(json.dumps(dict(pin=pin,source='crates/typst-library/src/math/lr.rs:144-198',starts=starts),ensure_ascii=False,indent=2)+'\n')
probe=json.load(open(out/'more-cast-probe.json'));constructors=[];noconstr=[]
for r in probe['rows']:
 if r['binary']!='vanilla' or not r['name'].startswith('type-'):continue
 kind=r['name'][5:];line=r['stderr'].splitlines()[0]
 if line=='error: expected element':constructors.append(kind)
 else:assert line.startswith('error: type ') and line.endswith(' does not have a constructor');noconstr.append(kind)
# Location has no CLI construction outside a context; verify its pinned #[ty] metadata has no constructor.
location=subprocess.check_output(['git','show',pin+':crates/typst-library/src/introspection/location.rs'],text=True);assert 'constructor' not in location
noconstr.append('Location');assert len(constructors)+len(noconstr)==38
(out/'type-kind-classes.json').write_text(json.dumps(dict(constructors=constructors,no_constructor=noconstr,location_source='crates/typst-library/src/introspection/location.rs',pin=pin),indent=2)+'\n')
evidence='''
## Complemento de casts de figure — Type e Symbol, 2026-10-17

Medição anterior à decisão: type-kind-probe.json e more-cast-probe.json em
00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17 registram o candidato
que passou 5/6 testes mas falhou em kind:int. foundations/value.rs:647-658 do
pin 9f2b6e8715237cb086899a42873660fe744622e8 mede Func aceitando Type.constructor
ou Symbol.func; content/element.rs:210-214 exige to_element; symbol.rs:133-137,
math/accent.rs:189-214 e math/lr.rs:144-198 classificam a conversão simbólica.
Há 37 tipos exercitados no CLI; Location sem construtor foi conferido na
fonte pinada. type-kind-classes.json guarda a classificação completa dos 38.
Símbolo aceito como corpo/caption é convertido pelo pin a SymbolElem; o nosso
Content não tem essa variante e o markup atual conserva o valor como Text.
Esse carrier/morfologia exige evolução pública SY1, não pode ser declarado
resolvido pela preservação do texto nesta etapa.
'''
contracts={
'00_nucleo/prompts/compiler/stdlib/figure_image.md':'''Concluir o cast de kind: Value::Type com construtor erra expected element;
sem construtor erra type {nome longo} does not have a constructor. Classificar
Type exaustivamente conforme a medição, sem executar o construtor. Bool usa
boolean; Relative usa relative length. Value::Symbol callable como acento
conhecido ou delimitador LR erra expected element, pois wrappers não são
funções de elemento; símbolo não callable erra symbol {valor} is not callable.
Delegar a classificação de acentos/LR aos seus owners via bridges privados,
sem duplicar tabelas nem criar função/trait público. Preservar as âncoras do
kind. Body/caption Symbol conservam o grapheme efetivo como Content::Text,
igual ao cast de markup vigente; não continuar a convertê-los em vazio/erro.
SY1 no follow-ups.md completará SymbolElem, reflexão/morfologia após proposta
e gate público; esta preservação textual é explicitamente incompleta.
''',
new['01_core/src/compiler/stdlib/structural/math/accent.rs']:'''Adicionar apenas is_known_math_accent(&str)->bool pub(crate), consultando a
única ACCENT_ALIASES existente: codepoint canônico ou alias exato. Ao contrário
de normalize_math_accent, não aceitar qualquer char pelo fallback direct.
O helper é classificação pura para cast Function; não chama função, não
muda normalize_math_accent, construção de Accent nem layout.
''',
new['01_core/src/compiler/stdlib/structural/math/lr.rs']:'''Adicionar apenas is_callable_lr_symbol(&str)->bool pub(crate). String de um
codepoint presente no conjunto completo de inícios de DELIMS do pin, mais
ceil/floor, é callable; vazio, sequência de codepoints e fechamento não listado
não são. O inventário lr-symbol-starts.json é a medição. Isso classifica o cast,
não cria chamadas por símbolo nem modifica math.lr/size/body/algoritmo.
''',
new['01_core/src/compiler/stdlib/structural/mod.rs']:'''Reexportar diretamente, somente pub(crate), is_known_math_accent do owner
math/accent e is_callable_lr_symbol do owner math/lr. Não mover tabelas,
classificação ou lógica ao hub; nenhuma API externa ou registro novo.
'''}
for owner,contract in contracts.items():
 p=Path(owner);p.write_text(p.read_text()+evidence+'\nDecisão exclusiva deste owner:\n\n'+contract+'\nAceitação: L0 primeiro, RED funcional de Type/Symbol/valor textual antes do código,\nGREEN e revalidação integral desta etapa; 12 GiB/swap zero/jobs1. Fluxo contínuo\nADR-0127, sem contrato Rust público, fase ou default de produto novo. Resselo só\ndos owners após V15/V26; preservar testes de matemática e classificadores originais.\n')
with (out/'follow-ups.md').open('a') as f:f.write('\nSY1: completar o carrier SymbolElem e sua reflexão/morfologia, sem trocar a\npreservação textual desta etapa por alegação de paridade completa. Ler os owners\nde Content/travessia/render antes de proposta pública; depende do gate ADR-0127.\n')
(out/'scope-expansion.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),reason='Measured FromValue Type/Symbol branches exposed by failed test; math helpers stay in their owners.',added=new,owners_after_prescription={p:sha(p) for p in contracts}),indent=2)+'\n')
