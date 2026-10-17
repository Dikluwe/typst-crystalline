from pathlib import Path
import gzip,json,hashlib,datetime,base64,shutil
out=Path('00_nucleo/diagnosticos/block-auto-alignment-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(n):
 with gzip.open(out/n,'rt') as f:return json.load(f)
final=read('candidate-final.json.gz');pre=read('before-source.json.gz');suite=json.loads((out/'workspace-result.json').read_text());checks=json.loads((out/'followup-summary.json').read_text());rows=json.loads((out/'comparison.json').read_text())['rows'];parent=json.loads((out/'parent-comparison.json').read_text())
assert suite['exit']==0 and suite['sources_unchanged'] and checks['sources_unchanged'];assert all(sha(n)==v['sha256'] for n,v in final['files'].items());assert all(r['pixels_identical'] for r in parent)
repro=out/'reproduction';repro.mkdir(exist_ok=True)
for p in Path('/tmp').glob('tekt-auto-align-*'):
 if p.suffix in ['.py','.txt']:shutil.copyfile(p,repro/p.name)
(repro/'README.txt').write_text('Scripts históricos de inventory/l0/implement não são idempotentes. Não reaplicar ao estado corrigido. O script inicial de implementação parou na guarda de cardinalidade de avail_w_is_finite (duas ocorrências): a aplicação restante foi restrita ao primeiro método, layout_align. partial-implementation-test.log não é GREEN. current.patch é a alteração completa relativa a before-source; inclui teste de restauração do flag adicionado depois. comparison.json usa diferença RGB em canvas integral e máscara r>180,g<100,b<100; comparação Parent usa SHA dos pixels RGB dos PNGs nos dois estados. As fontes, binários, argv e UTC de cada sonda são preservados nos results.json.gz.\n')
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name!='evidence.json.gz':entries.append(dict(path=str(p),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:r=json.load(f)
assert all(hashlib.sha256(base64.b64decode(e['base64'])).hexdigest()==e['sha256'] for e in r['entries'])
reds=[r for r in rows if r['pin_red'] is not None];assert all(r['after_red']==r['pin_red'] and r['after_red_pixels']==r['pin_red_pixels'] for r in reds);full=sum(r['after_pin']['max']==0 for r in rows)
report=f'''# Block Auto: alinhamento geométrico tardio — 2026-10-14

Corrigido o refutador de Align geométrico que desaparecia sob clipping após
reduzir corretamente a largura do Block Auto. O filho é posicionado usando a
largura final do corpo com inset. O corpo continua sendo executado uma vez.
Isso não completa o transporte tiling, os exporters nativos nem a preparação
para produção.

## Proveniência

HEAD `{pre['head'].strip()}`, working tree não commitado. before-source,
red-state, candidate-final, pre-validation e workspace-pre conservam UTC,
diff HEAD --stat, fontes e SHA-256; os registros de sonda conservam também
status, comandos, fontes Typst e identidade dos binários. Não houve commit,
push, PR ou publicação.

Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário
`lab/typst-original/target/release/typst` SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O original block-auto-width permanece histórico e não foi reescrito.

## Medição e correção

before/results.json.gz reproduz zero pixels vermelhos em right/center/link
com clip, contra 3840 no pin, bbox [48,40,128,88] em 2px/pt. Sem clip, right
fica em [260,40,320,88], cortado pela página. A largura disponível era usada
por placement.rs como se fosse a alocação final, conhecida só ao fechar Block.
No pin, flow/block.rs:278–287 distingue constraint de expansão;
flow/distribute.rs:646–684 determina o tamanho antes de posicionar frames.
A necessidade é a geometria da linguagem, sem copiar a mecânica do upstream.

L0 foi atualizado primeiro. Núcleo block-auto-alignment.toml, pin
`ecf965471eedd8599209f0575811dd5b456dce1c43baa26881251e24e8ec6c6d`.
A mudança é interna/de paridade, em fluxo contínuo ADR-0127, sem API pública.
Layouter conserva um flag privado por Block; checkpoint salva/restaura e
subframe isolado limpa/restaura. Align fora da rota flow_aligned mantém
posição intrínseca e registra o fixup existente; Block resolve os paths
locais contra a largura final antes de recortar e introduzir paint_root.
O deslocamento usa os helpers existentes de Group/Link/Semantic.
A rota textual/grid certificada pelo classificador anterior não foi alterada.

red.log reproduz duas falhas reais: centro x77 em vez de x24 pt, nested x127
em vez de x27 pt. Uma aplicação parcial de script parou numa guarda de
cardinalidade; partial-implementation-test.log é falha intermediária, não
sucesso. green.log registra cinco testes passando (dois novos e três de
largura causal), incluindo left/center/right, irmão maior, nested/fixed,
Link, underline sobre forma e restauração de estado. Expectativas mantidas.
O resselo alterou somente os seis owners do manifesto e seus seis L0.

## Validação bilateral e limites

Corpus before/after: {len(rows)} pares SVG, canvas integral 160×120pt em
320×240 pixels, rsvg a 72dpi e zoom2, sem corte/alinhamento posterior.
As caixas do retângulo coincidem em {len(reds)}/{len(reds)} controles que o contêm;
{full}/{len(rows)} renders completos coincidem pixel a pixel. Essa última
medida é evidência localizada, não requisito de igualdade de bytes/Rust.
Com irmão de 70pt, right termina em [108,40,188,88] e center em
[78,40,158,88], como o pin; o restante do render ainda diverge em altura.
O caso simples com clipping foi inspecionado visualmente.

PDF-after contém sete pares, todos aprovados por qpdf. right/center/link e
underline externo sobre a forma coincidem integralmente; wider/nested/fixed
mantêm divergências de altura já conhecidas. O corpus Parent/Solid anterior
foi repetido: {len(parent)}/{len(parent)} renders Tekt PDF/SVG têm pixels
idênticos à entrada registrada no lote block-auto-width. Isso preserva o
controle PDF de gradiente parent corrigido naquele lote.

O texto simples deste corpus passa a coincidir porque o carrier composto
entra na rota geométrica; não inferir daí que a rota textual está completa.
Multilinha ainda difere em 153 pixels >16; underline interno em 78; matemática
em 8132 (canvas e estado acima). A matemática permanece inalterada nesta
rodada. A posição correta de uma forma sob underline não certifica linhas
de decoração: segmentos externos consumidos antes do fechamento exigem
transporte causal próprio. Place horizontal, nested Align em subframe,
altura Auto/backing, outset, Block fragmentado e Box fracionário seguem abertos.

## Verificação geral

Suíte completa de {suite['start']} a {suite['end']}:
**{suite['passed']} passaram, {suite['failed']} falharam, {suite['ignored']} ignorados**.
Fontes ficaram estáveis. Build offline/locked, rustdoc -D warnings,
all-target check, fmt/diff, V1/V5/V15/V26 estritos e linhagem reversa passaram.
Clippy: {checks['clippy_after']} avisos, delta por regra {checks['clippy_delta']};
isso é inventário e não passa o gate CI -D warnings. Arquitetura:
{checks['architecture'].get('error',0)} erros, {checks['architecture'].get('warning',0)}
warnings, {checks['architecture'].get('note',0)} notes. CI remoto e release
continuam sem certificação; não foi publicada release.

## Continuidade

Completar alinhamento por linha e matemática, transporte de decoração,
altura/outset e fragmentação com seus refutadores. T1c ainda migra
Shape.fill/stroke aos carriers aprovados; T2 implementa patterns PDF/SVG/PNG
e recursos recursivos; T3 completa geometria afim. As demais pendências do
objetivo integral permanecem abertas; este lote não redefine conclusão.

## Evidência

[evidence.json.gz](block-auto-alignment-2026-10-14/evidence.json.gz):
{len(entries)} entradas com caminho/SHA-256/base64, relidas e verificadas.
SHA-256 `{sha(out/'evidence.json.gz')}`.
'''
Path('00_nucleo/diagnosticos/block-auto-alignment-2026-10-14.md').write_text(report)
print(json.dumps(dict(entries=len(entries),sha256=sha(out/'evidence.json.gz'),passed=suite['passed'],checks=checks),indent=2))
