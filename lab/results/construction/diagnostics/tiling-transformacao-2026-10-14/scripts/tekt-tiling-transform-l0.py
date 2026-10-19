from pathlib import Path
import gzip,json,subprocess,datetime,hashlib,re
out=Path('00_nucleo/diagnosticos/tiling-transformacao-2026-10-14');sources=['01_core/src/compiler/layout/tiling.rs','01_core/src/entities/tiling.rs','01_core/src/compiler/stdlib/visualize.rs'];owners=[re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) for p in sources]
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in sources+owners},workspace={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for top in ['01_core','02_shell','03_infra','04_wiring','benches'] for p in Path(top).rglob('*.rs')})
with gzip.open(out/'before-state.json.gz','wt') as f:json.dump(r,f)
nucleus=Path('00_nucleo/prompts/_nuclei/tiling-placement.toml');assert not nucleus.exists();nucleus.write_text('''tekt = 1
kind = "nucleus"
id = "tiling-placement"
title = "Offset como origem da rotação do tiling"

[[claims]]
id = "rotation-about-offset"
level = "must"
statement = "No referencial escolhido para o padrão, offset é a origem da rotação horária da grade: um ponto local q é colocado em offset + R(angle) q. Percentuais de offset são resolvidos contra tamanho da célula mais spacing. A entidade e o constructor conservam estes valores sem pré-rotacionar offset; a materialização não gira o ponto offset em torno de zero."

[[claims]]
id = "periodic-equivalence"
level = "must"
statement = "Eliminar períodos inteiros de translação preserva a pintura somente no referencial da grade: deslocamentos equivalentes diferem por R(angle) aplicado a múltiplos inteiros dos pitches. A redução modular acontece na base inversa da rotação, e não independentemente nos eixos da página quando o ângulo é não nulo."
''')
digest=hashlib.sha256(nucleus.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
measurement='''

## Origem da rotação de tiling — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-transformacao-2026-10-14/before/results.json.gz`
conserva fontes/owners, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado, diffstat, UTC e binários. Referência pinada
9f2b6e8715237cb086899a42873660fe744622e8. Para angle90 e offset(3pt,5pt),
o SVG vanilla emite matrix(0 1 -1 0 3 5); o Tekt emite (0 1 -1 0 -5 3).
No pin, visualize/tiling.rs:173-174 descreve rotação em torno do offset;
:375-377 concatena translate(offset) com rotate(angle), e
layout/transform.rs:334-343 calcula T*R. O texto histórico "offset antes de
angle" foi interpretado incorretamente como rotação do próprio offset.
Semântica geométrica é obrigação de linguagem; ordem de chamadas Rust não é.

before/render-comparison.json.gz discrimina targets: em quarter sem offset,
Tekt PDF e SVG coincidem exatamente com vanilla SVG, enquanto vanilla PDF
difere em 414 pixels >16. Não copiar essa diferença do PDF como requisito.
Em quarter-offset, os SVGs diferem em 1764 pixels >16. Rasterizações usam
canvas inteiro 240×192, branco, librsvg e Poppler, sem recortes/tolerâncias
criadas para aprovar. A evidência não certifica parent composto nem todo PDF.

### Responsabilidade deste owner

'''
contracts=[
'''Materializar o contrato do núcleo tiling-placement. placement_transform compõe
translate(offset - origem fornecida do shape para Parent) * rotate(angle).
Self/Auto não subtraem a origem. A origem do pai não é rotacionada junto com
a grade. Antes de gerar a grelha, transformar a translação para a base inversa
da rotação, reduzir pelos pitches, retornar à base de placement e manter
os coeficientes lineares. Validar finitude antes de normalizar ou iterar.
Conservar a cobertura pelo inverso dos cantos, recortes, tamanho auto, corpo
vazio transparente, layout único e limite 10000 do reparo anterior.
Não confundir a origem local disponível com o referencial composto do pai:
Place/aninhamentos continuam abertos até transporte e medição próprios.
O teste histórico que esperava offset girado passa a exigir pivô invariável;
novos RED discriminam Parent com rotação e periodicidade na base correta.
Recolher PDF e SVG e revalidar workspace, build e linhagem.
''',
'''Preservar todos os campos, tipos, valores, constructor e defaults da entidade
Tiling. A documentação de angle declara rotação horária em torno do offset,
sem implicar que o vetor offset é girado. O núcleo pinado contém a obrigação
compartilhada. Não introduzir frame resolvido ou novo tipo público; nenhum
método ganha comportamento de layout. A ambiguidade do texto de P1245 fica
retificada, sem alteração de contrato Rust ou fase.
''',
'''O constructor conserva offset e angle declarativos, inclusive percentuais,
sem os pré-compor ou materializar body. A fronteira de fase referencia o
núcleo pinado: o layout aplica a rotação em torno do offset. Nenhum ramo,
assinatura, erro, default ou decisão de validação do constructor é alterado.
Retificar somente a formulação ambígua de P1245 e o selo do owner.
''']
for owner,contract in zip(owners,contracts):
 p=Path(owner);s=p.read_text();s=re.sub(r'(Hash do Código: [^\n]+\n)',r'\1\nNúcleos Tekt:\n- '+str(nucleus)+' sha256:'+digest+'\n',s,count=1)
 if owner==owners[0]:
  s=s.replace('Relative offset uses pitch; apply offset before clockwise angle through the existing matrix order. Shapes\' Auto/Itself phase is local; explicit Parent cancels the shape-origin translation before pattern placement.', 'Use the pinned tiling-placement geometry contract. Shapes\' Auto/Itself phase is local; explicit Parent cancels the supplied shape-origin translation in the target basis, separately from grid rotation.')
  s=s.replace('Calcular o raio diagonal com a função pura libm::hypot (0.2.11 pinada), conservando a soma da dimensão da célula.', 'Historicamente, o raio diagonal usava libm::hypot (0.2.11 pinada). A fórmula por raio foi substituída pelo inverso dos cantos em Célula geométrica e cobertura da repetição; não é obrigação vigente de algoritmo.')
  s=s.replace('fase de self/parent e offset antes de ângulo;', 'fase periódica; a ordem do offset é retificada abaixo pelo núcleo tiling-placement;')
 elif owner==owners[1]:
  s=s.replace('`size:auto`, conserva spacing/offset/angle/relative e aplica offset antes de\nangle.', '`size:auto`, conserva spacing/offset/angle/relative e gira a grade em torno\ndo ponto offset (ordem esclarecida no núcleo tiling-placement).')
 else:
  s=s.replace('célula, derivar size auto, resolver percentuais e aplicar offset antes de angle.', 'célula, derivar size auto, resolver percentuais e aplicar a geometria do núcleo\ntiling-placement, com rotação em torno do offset.')
 s+=measurement+contract+'\nCorreção interna/de paridade ADR-0127 em fluxo contínuo: L0 primeiro,\nRED→GREEN e resselo restrito. Nenhuma mudança pública ou nova fase.\n';p.write_text(s)
print('nucleus',digest)
