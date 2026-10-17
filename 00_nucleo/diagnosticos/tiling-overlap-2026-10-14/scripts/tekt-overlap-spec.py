from pathlib import Path
import re,json,gzip,hashlib,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tiling-overlap-2026-10-14');out.mkdir(exist_ok=False)
paths=['03_infra/src/export/'+p+'.rs' for p in ['tiling','pdf_tiling','svg/paints','stream','stream/shape','stream/draw','stream/text','stream/page','builder/resources']]
(out/'owned-sources.json').write_text(json.dumps(paths,indent=2))
owners={s:re.search(r'^//! @prompt (.+)$',Path(s).read_text(),re.M)[1] for s in paths}
names=paths+list(owners.values())
snapshot=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in names})
with gzip.open(out/'before.json.gz','wt') as f:json.dump(snapshot,f)
clauses={
 'tiling':'Fornecer iterador privado de contribuições que intersectam um período [0,pitch), na ordem y crescente e x crescente. Em cada eixo índices de 1-ceil(size/pitch) até zero inclusivos, translação índice*pitch. A célula já é recortada a size; o período é recortado a pitch. Sem vetor de cópias proporcional ao viewport. Dimensões/pitch finitos positivos são precondição do transporte; checar contagem representável antes de conversão. A integração de diagnóstico de limites extremos segue aberta, sem truncar repetições ou alegar sucesso.',
 'pdf_tiling':'Substituir a dependência de overlap nativo por Form XObject da célula, BBox=size e corpo vetorial emitido uma vez. Reservar dois IDs por ocorrência (pattern e form); PatternType1 tem BBox=pitch, XStep/YStep=pitch e referências traduzidas ao Form para cada contribuição do helper. Recursos do Form conservam fontes/imagens/ExtGState/gradientes e apenas descendentes de tiling; recursos do Pattern referenciam somente o Form. Célula Y-down, matriz de placement e isolamento semântico permanecem. Não inserir grupo de transparência isolado que mude a composição de folhas. O default user space de padrões aninhados deve permanecer o da célula; testar antes de promover.',
 'svg/paints':'Guardar o corpo vetorial recortado uma vez num g identificado dentro de defs. Pattern com viewport pitch e overflow hidden contém use traduzido para cada contribuição do helper, preservando ordem. Não depender de overflow visible para compor vizinhos. Manter fila de patterns aninhados, glyphs/fonts/clip IDs únicos, raiz local e exclusão de links decorativos. Cláusula anterior sobre overflow visible fica substituída.',
 'stream/shape':'Possuir helpers privados para opacidade sólida de cada folha FrameItem, par (fill,stroke) e nome determinístico por bits f32, sem quantização u8. Shape considera só Solid em cada role; texto/glyph/separator usam alpha da cor para ambos (incluindo faux-bold), Line usa stroke; demais folhas/containers opacos. Gradient/Tiling não herdam alpha de uma cor representativa. Gerar operador ExtGState somente se algum alpha difere de1. Esses helpers não criam objetos nem recursos.',
 'stream/draw':'Abrir q/ExtGState somente para folhas translúcidas, fechar após a folha sem contaminar a próxima. Top/local preservam ActualText: o número privado de Q pendentes inclui o envelope adicional e só é descarregado depois de EMC, sem busca textual de operadores. Aplicar também a glifos marcados que passam por caminho direto. Não alterar transformações nem marcações opacas.',
 'stream/text':'No prefixo verboso compartilhado (inclui runs fundidos), aplicar ExtGState da cor textual dentro de q já existente antes de BT. O dispatcher isola caminhos compactos e glifos diretos. Não mudar coalescência, métricas, ActualText ou posição. O par alpha é igual em fill e stroke para faux-bold.',
 'stream/page':'No canvas Solid translúcido, aplicar opacidade com q/Q envolvendo somente o fill da página. Não alterar projeção antiga de PageFill Gradient/Tiling nem fingir que esta etapa a corrige. Os itens seguintes começam opacos.',
 'builder/resources':'Coletar pares de opacidade das folhas via helper único no stream e percurso visual de recursos que desce células. Incluir canvas Solid e emitir ExtGState /ca /CA diretos, deduplicados em ordem determinística. Retornar os recursos no mesmo agregado privado usado pelos três builders e Forms de célula; não mudar API externa nem ICC/spot.',
 'stream':'Reexportar privadamente helpers de opacidade do owner shape para recursos/draw/text/page. Sem alterar PageContext ou APIs externas.'}
for s,p in owners.items():
 key=s.removeprefix('03_infra/src/export/').removesuffix('.rs')
 with Path(p).open('a') as f:f.write('\n## Composição de período e alpha sólido — 2026-10-14\n\n### Medição anterior à decisão\n\n`00_nucleo/diagnosticos/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.\n\n### Responsabilidade desta unidade\n\n'+clauses[key]+'\n\n### Aceitação\n\nCorreção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.\n')
# Shared invariants are owned by nuclei, not repeated normative definitions.
nuc=Path('00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml')
nuc.write_text('''tekt = 1
kind = "nucleus"
id = "pdf-solid-opacity"
title = "Opacidade sólida PDF e escopo gráfico"

[[claims]]
id = "leaf-alpha"
level = "must"
statement = "Opacidade sólida usa o alpha f32 da cor convertida, preservado sem quantização u8; nomes e pares de recursos seguem helper único. Fill e stroke são independentes; texto usa ambos para faux-bold. Gradientes e tiling não usam alpha representativo."

[[claims]]
id = "isolation"
level = "must"
statement = "Estado de alpha não vaza entre folhas, canvas e células. Em ActualText, restauração gráfica diferida permanece depois do fechamento semântico; recursos de páginas e células cobrem todos os operadores usados."
''')
digest=hashlib.sha256(nuc.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
for key in ['stream/shape','stream/draw','stream/text','stream/page','builder/resources']:
 p=Path(owners['03_infra/src/export/'+key+'.rs']);s=p.read_text();pin=f'- {nuc} sha256:{digest}'
 lines=s.splitlines();pins=[x for x in lines if x.startswith('- 00_nucleo/prompts/_nuclei/')];lines=[x for x in lines if x not in pins and x!='Núcleos Tekt:'];lines[2:2]=['','Núcleos Tekt:',*sorted(pins+[pin])];p.write_text('\n'.join(lines)+'\n')
print('L0 and before snapshot written')
