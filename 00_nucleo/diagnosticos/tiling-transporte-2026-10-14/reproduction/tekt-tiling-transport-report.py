from pathlib import Path
import json,gzip,hashlib,datetime,base64,re,subprocess
out=Path('00_nucleo/diagnosticos/tiling-transporte-2026-10-14');package=Path('00_nucleo/materialization/tiling-transporte-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(out/'parent-before/results.json.gz','rt') as f:baseline=json.load(f)
assert all(sha(p)==v['sha256'] for p,v in baseline['files'].items())
with gzip.open('00_nucleo/diagnosticos/raw-presenca-2026-10-14/candidate-final.json.gz','rt') as f:prior=json.load(f)
assert all(sha(p)==v['sha256'] for p,v in prior['files'].items())
inventory=[]
for top in ['01_core','02_shell','03_infra','04_wiring','benches']:
 for p in Path(top).rglob('*.rs'):
  s=p.read_text();hits=[dict(line=i,text=line) for i,line in enumerate(s.splitlines(),1) if any(x in line for x in ['FrameItem::Shape','FrameItem::Group','impl FrameVisitor','Paint::Tiling'])]
  if hits:
   m=re.search(r'^//! @prompt (.+)$',s,re.M);owner=m.group(1) if m else None
   inventory.append(dict(path=str(p),sha256=sha(p),owner=owner,owner_sha256=sha(owner) if owner and Path(owner).exists() else None,hits=hits))
with gzip.open(out/'consumer-candidates.json.gz','wt') as f:json.dump(dict(utc=now(),head=baseline['head'],diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),note='Textual candidates, not proof every file must change. Read and update each actual owner before implementation.',files=inventory),f)
repro=out/'reproduction';repro.mkdir(exist_ok=True)
for n in ['/tmp/tekt-tiling-parent-probe.py','/tmp/tekt-tiling-parent-solid-probe.py','/tmp/tekt-tiling-transport-proposal.py','/tmp/tekt-tiling-transport-report.py']:
 p=Path(n);(repro/p.name).write_bytes(p.read_bytes())
(repro/'README.txt').write_text('Scripts históricos; probes exigem diretório de saída novo e hashes de binários validados. proposal.py é criação exclusiva, não reaplicar sobre pacote existente. Candidatos de consumidor são inventário textual, não autorização de editar todos. Nenhum source produtivo/L0 canônico foi modificado neste lote.\n')
entries=[]
for p in sorted(list(out.rglob('*'))+list(package.rglob('*'))):
 if p.is_file() and p.name!='evidence.json.gz':entries.append(dict(path=str(p),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(utc=now(),entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:r=json.load(f)
assert all(hashlib.sha256(base64.b64decode(x['base64'])).hexdigest()==x['sha256'] for x in r['entries'])
text=f'''# Tiling: transporte, referenciais e controles de geometria — 2026-10-14

O código atual precisa transportar a célula pronta até os exporters para remover
o fallback de contornos e o limite de cópias dos padrões densos. As novas medições
separam erros de fase do padrão de erros gerais de geometria: parent em box/place
diverge com shape sólido correto; rotate e box-place também divergem em Solid.

## Proveniência

HEAD `{baseline['head'].strip()}`, working tree não commitado. `parent-before/results.json.gz`
e `solid-controls/results.json.gz` registram UTC, status, git diff HEAD --stat,
fonte integral por caso, comandos, stdout/stderr, hashes de fontes/L0/binários e
artefatos. Binário Tekt: `{baseline['binaries']['tekt']['sha256']}`.
Oráculo vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Medição principal: {baseline['start']} → {baseline['end']}.
Nenhum source produtivo ou L0 canônico foi alterado; hashes do lote Raw anterior
e dos owners inspecionados foram reconferidos. A suíte Raw anterior não é atribuída
a uma implementação de tiling, que ainda não existe.

## Evidência e classificação

| Controle SVG, canvas integral 240×192 | Pixels com diferença >16 | Interpretação |
|---|---:|---|
| place-parent | 1194 | Solid correspondente: zero diferença; erro do padrão isolado |
| box-fixed-parent / box-auto-parent | 1192 cada | Solid correspondente: zero diferença; erro do referencial de paint |
| block-fixed-parent / block-auto-parent | 2 cada, máximo 23 | Controle próximo, sem alegar identidade universal |
| box-place Solid | 3200 | Defeito de geometria independente do padrão |
| rotated-box Solid | 5296 | Defeito de geometria independente do padrão |
| rotated-soft Solid | 3645 | Defeito de geometria independente do padrão |

As imagens usam fundo branco e librsvg no canvas inteiro. As diferenças são
medição diagnóstica, não tolerância criada para aprovar um candidato. PDF também
foi gerado e validado por qpdf em todos os casos, sem certificar paridade pelo
simples sucesso. Casos, arquivos e matrizes SVG preservados. Inspeção visual do
par box-fixed-parent confirma fases distintas, além da diferença numérica.

Vanilla SVG emite T(offset)*R(angle) e referência parent com a inversa da
transformação acumulada. Fonte pinada typst-svg/src/lib.rs:346–358 distingue
frames Soft/Hard; typst-layout/src/inline/box.rs:35–49 estabelece raiz local e
inclui inset no frame. Não se deduz intenção apenas de pixel nem se impõe a
mecânica upstream. Preservar a divergência já comprovada em quarter: Tekt PDF/SVG
concorda com vanilla SVG, enquanto o PDF vanilla difere.

Tekt layout/tiling.rs:124–127 limita a geração a10000 células e retorna None;
shape.rs:151–185 prepara apenas fill. export/stream/shape.rs:151/201 usa cor em
Tiling. FrameItem::Shape/Group não transporta célula/root. Raster render.rs:217
reduz fill a cor e :579 ignora clip_mask. A futura implementação precisa cobrir
esses caminhos; apenas elevar limite ou ajustar offset não atende produção.

## Proposta pronta para aprovação

[Revisão](../materialization/tiling-transporte-2026-10-14/review.md) e
[contrato completo](../materialization/tiling-transporte-2026-10-14/contract.md).
Manifesto `{sha(package/'manifest.json')}`.

Criar FramePaint/TilingFrame/FrameStroke; substituir fill/stroke do FrameItem::Shape
pelos carriers resolvidos; adicionar Group.paint_root. Paint/Stroke/Tiling da
linguagem permanecem declarativos. FramePaint conserva variantes Solid/Gradient;
Tiling contém somente célula preparada. A API pública Rust de Shape/Group muda;
construções externas precisam migrar. Os exporters conservam assinaturas e defaults.

T1–T3 incluem migração, layout, roots, PDF/SVG/raster, recursos/shaping, clipping,
strokes completos, padrões densos e validação dos controles gerais de geometria.
T4 é etapa futura explícita para texto/fills de caixas/linhas Color-only, com
contrato próprio. Não é retirada do objetivo geral; este pacote não promete
fechamento universal de tiling nem de produção.

A confirmação foi solicitada por ADR-0127 §2.1/§2.4: “adicionar, remover ou alterar
campo público” e quebra de compatibilidade. **Pendente; nenhum candidato foi
instalado como L0 canônico e nenhum código foi implementado.** Manifesto, hashes,
TOML, ordem/header de pins e ownership dos candidatos passaram checagem estática;
isso não substitui o linter canônico após a materialização. Claims compartilhadas
ficam no novo núcleo tiling-frame-transport; os owners reais de cada consumer
serão lidos/atualizados antes de código. O inventário textual não é lista automática
de edições nem autorização para ignorar contratos vigentes.

## Evidências

[evidence.json.gz](tiling-transporte-2026-10-14/evidence.json.gz): {len(entries)} entradas
com caminho, SHA-256 e bytes em base64, relidas e verificadas. Inclui fontes,
medições, imagens/PDF/SVG, candidatos, manifesto, validação estática e scripts
históricos. Nenhuma alteração foi publicada/commitada.
'''
(out.with_suffix('.md')).write_text(text)
print(json.dumps(dict(report=str(out.with_suffix('.md')),entries=len(entries),archive_sha256=sha(out/'evidence.json.gz'),sources_unchanged=True),indent=2))
