from pathlib import Path
import gzip,json,hashlib,datetime,shutil
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'working-baseline.json.gz'));scope=json.load(open(out/'scope.json'));new={'01_core/src/entities/elements/figure.rs':'00_nucleo/prompts/entities/elements/figure.md','01_core/src/entities/content/fields.rs':'00_nucleo/prompts/entities/content/fields.md'};sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
for source,owner in new.items():
 for p in [source,owner]:assert sha(p)==b['hashes'][p];b['files'][p]=Path(p).read_text()
 scope['sources'].append(source);scope['owners'][source]=owner
with gzip.open(out/'working-baseline.json.gz','wt') as f:json.dump(b,f)
(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
evidence='''
## Reflexão da legenda presente — 2026-10-17

Medição antes da decisão: caption-before.json e more-cast-after.json em
00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17 fixam UTC, fontes,
binários e o pin 9f2b6e8715237cb086899a42873660fe744622e8. A construção
conserva a caption, mas content/fields.rs:73 projeta só body de Figure;
FigureElem mantém o getter default None. O pin retorna FigureCaption para
caption presente (inclusive conteúdo vazio), null para none explícito e erro
not known at this point para omissão. O carrier público Option<Content> não
distingue none explícito de omissão; essa presença continua incompleta no FG1,
exige proposta pública, e não deve ser simulada com Empty/strings.
'''
for source,owner in new.items():
 contract=('''Implementar somente o override já existente Element::get_field para body e
caption. body retorna o Content original clonado. caption Some(body) projeta
Value::Content(Content::figure_caption(body.clone(),None,None)), conservando
origem e sem resolver defaults, sintetizar kind/supplement/counter ou executar
layout. caption None e campos desconhecidos continuam None do getter: não
inventar presença de campo. O novo override não adiciona método ao trait nem
muda assinatura pública; é correção interna de reflexão de linguagem em fluxo
contínuo ADR-0127. O campo numbering já foi removido em P365; a entidade atual
possui body/caption/kind, a numeração continua na chain.
''' if source.endswith('elements/figure.rs') else '''Trocar a projeção especial de Figure.body pela delegação estática de todos
os campos Figure ao getter do elemento proprietário. Não colocar montagem da
caption aqui nem alterar outros braços/transparência/pins. O getter existente
mantém a mesma assinatura pública. Sem registro dinâmico ou fallback de corpo.
''')
 p=Path(owner);p.write_text(p.read_text()+evidence+'\nResponsabilidade exclusiva e aceitação:\n\n'+contract+'\nL0 antes do teste/código. RED real da projeção de caption, GREEN e replay das\nseis sondas mais os casos simbólicos. Body, Source e campos ausentes continuam\npreservados. Revalidar a etapa integral sob 12 GiB/swap zero/jobs1, resselo\nsó dos owners após V15/V26 válido; sem API/fase/default novo nem release.\n')
# Preserve all artifacts from the failed verifier before a resumed validation overwrites them.
dest=out/'candidate-2';dest.mkdir(exist_ok=False)
for p in [*out.glob('*resources.json'),*out.glob('validation-*.log'),out/'validation-result.json',out/'more-cast-after.json']:
 if p.is_file():shutil.copy(p,dest/p.name)
(out/'caption-scope-expansion.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),added=new,source_sha256={p:sha(p) for p in new}),indent=2)+'\n')
