# P1293 — medição causal independente do residual IC em `attach`

## Escopo e segregação

Auditoria read-only do produto sob o protocolo completo da skill
`tekt-materializacao-segregada`, por capacidades/artefactos, sem atestação de
isolamento técnico do filesystem compartilhado. A única escrita no repositório
é este recibo; fontes e outputs próprios vivem em
`/tmp/p1293-attach-ic-residual-*`.

Não foram lidos contrato, oracle, testes, gates, RED/discrimination privados,
manifesto, selo ou materialization adicional. Nenhum produto, L0 ou veredito
foi alterado. Este recibo não aprova o lote B.

Entrada pública autorizada: `p1293-implementation-receipt-b.md`, SHA-256
`c067ee6fb7238dc72862208fa18deec73b2fd9ea33c903838e2554daa9c9da6e`.

## Proveniência

- checkpoint: `2026-09-01T23:23:17.775634527-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada e não commitada;
- `git status --short | sha256sum`:
  `80b9ef2ef10f0926115871aa033ff5d2270aa82963a07fff6b852fb0ec5a7e1b`;
- `git diff HEAD --stat | sha256sum`:
  `aab5f7f6b94dc4985bff6dd6c36c2a86a6c18bf71e6097048c13a8da18584d87`;
- stat: `50 files changed, 4164 insertions(+), 402 deletions(-)`; este recibo
  não existia no checkpoint.

Entradas produtivas:

| Arquivo | SHA-256 |
|---|---|
| `01_core/src/compiler/layout/equation.rs` | `96139ff7ba16826a1ea6db217ae2d0a68b4fd7e729a292d04db5f70a39f25aea` |
| `01_core/src/compiler/math/layout/attach.rs` | `1a660cf5b86860414d58654e945c35d1ce6b243585baa8ef3b0aa848b078a526` |
| `03_infra/src/shaper.rs` | `1b7917b1efb442e1d21862d3117cd95196f640eabdede9ea83d9e78ca2c7074e` |
| `03_infra/src/font_metrics.rs` | `f18bff7c5637f0d94eb48c6d184090d04e0446ef9a71132f8dfef2a050fef990` |

L0 attach: `00_nucleo/prompts/compiler/math/layout/attach.md`, SHA-256
`fc189ea4f52436ca9c1adf52ba6767e3a2ff241211fb6edf4de9e4632cae6ff9`.

Vanilla pinado:

- `math/scripts.rs`: `d3f8a9fc8a4f58eafdc3edbac9cdb67279ff0f023dd9b4967f209e81165f286b`;
- `math/fragment/mod.rs`: `f88af5625e7d837df75d7666d1f9fec0fc76cf4874efadbec40d288fd648de90`;
- `NewCMMath-Book.otf`: `60346e6fc27773d96c3cc9d3ac8a550a85e3ca624437179bd7a88b4ca18f0a18`.

Binários:

- candidato `target/release/typst`:
  `612ac055aca6d18b06a200c00d74080a602e90ad0e766667077e91d1d65ccf34`;
- vanilla `/usr/local/bin/typst`:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

## Comandos

```sh
/usr/local/bin/typst compile --format svg SOURCE OUTPUT-vanilla.svg
target/release/typst compile --format svg SOURCE OUTPUT-candidate.svg
python3 -c '... extrai viewBox e transforms matrix dos SVGs ...'
python3 -c 'from fontTools.ttLib import TTFont; ... hmtx e MathItalicsCorrectionInfo ...'
hb-shape 03_infra/fixtures/fonts/NewCMMath-Book.otf tr
hb-shape 03_infra/fixtures/fonts/NewCMMath-Book.otf br
sha256sum /tmp/p1293-attach-ic-residual-*
```

## Reprodução pública

Fonte `02-attach-inline.typ` reproduzida byte-identicamente, SHA-256
`6c135e979542187d7cb286ea29d00ef5c401710e890348707c276afe3609face`:

```typst
#set page(width: auto, height: auto, margin: 0pt)
$#math.attach([x], t: [t], b: [b], tl: [tl], bl: [bl], tr: [tr], br: [br])$
```

- vanilla: `20.7614 × 9.7735pt`, SVG SHA-256
  `461397ee3dbd7b7f6239d6931326342c3f5009dcd642f36910a90f47bb896339`;
- candidato: `20.5854 × 9.7735pt`, SVG SHA-256
  `c96002af51d5fe3c3ce626d4ef112eb7fb218236996b54b64c2c14222e5604fb`;
- residual exclusivamente horizontal: `−0.1760pt`.

## Decomposição geométrica

No vetor completo, posições x:

| Item | Vanilla | Candidato |
|---|---:|---:|
| base `[x]` | 7.0378 | 7.0378 |
| `tl:[tl]` | 1.9019 | 1.9019 |
| `bl:[bl]` | 0.6160 | 0.6160 |
| `tr:[tr]` | 12.8458 | 12.8458 |
| `br:[br]` | 12.8458 | **12.6698** |
| upper limit `[t]` | 8.44415 | 8.53215 |
| lower limit `[b]` | 7.8012 | 7.7132 |

Métricas reais, sem constante de fixture:

- base `[x]`: `528du × 11/1000 = 5.8080pt`;
- `SpaceAfterScript`: `56du × 11/1000 = 0.6160pt`;
- script size: `7.7pt`;
- `tl`: `(389+278)du × 7.7/1000 = 5.1359pt`;
- `bl`: `(556+278)du × 7.7/1000 = 6.4218pt`;
- `tr`: `(389+392)du × 7.7/1000 = 6.0137pt`;
- `br`: `(556+392)du × 7.7/1000 = 7.2996pt`;
- IC crua de x: `16du × 11/1000 = 0.1760pt`.

Os math-kerns dos quatro quadrantes são zero neste witness, confirmado pelas
origens: `tr` começa exatamente em `pre + base = 7.0378 + 5.808 = 12.8458`;
antes da subtração de IC, `br` tem a mesma origem.

Contribuições:

```text
tl_pre = 0.6160 + 5.1359 = 5.7519
bl_pre = 0.6160 + 6.4218 = 7.0378  <- pre_width

tr_post = 0.6160 + 6.0137 = 6.6297
br_post vanilla = 0.6160 + 7.2996 = 7.9156
br_post candidato = 0.6160 + 7.2996 - 0.1760 = 7.7396

total vanilla = 7.0378 + 5.8080 + 7.9156 = 20.7614
total candidato = 7.0378 + 5.8080 + 7.7396 = 20.5854
```

Os limites não dominam os máximos. Para `[t]`, half-width é `−1.40635pt`:
o candidato usa `delta=IC/2=0.088`, deslocando os lados para `−1.49435` e
`−1.31835`. Para `[b]`, half-width é `−0.7634`, deslocado para `−0.6754` e
`−0.8514`. Isso explica as posições ±0.088 acima sem causar o residual final.

## Refutadores

### Presença de `br`

| Probe | Vanilla | Candidato | Delta |
|---|---:|---:|---:|
| completo, sem `br` | 19.4755 | 19.4755 | 0 |
| apenas `tr:[tr]` | 12.4377 | 12.4377 | 0 |
| apenas `br:[br]` | 13.7236 | 13.5476 | -0.1760 |
| `tr:[tr] + br:[br]` | 13.7236 | 13.5476 | -0.1760 |

Logo nem equation-frame, pre-scripts, limits nem `tr` causam o residual.

### Caractere/width do `br`

Com base `[x]`, `br:[S]`, `br:[W]` e `br:[RR]` perdem todos exatamente
`0.1760pt`; a largura do subscrito não muda o termo. Com `tr:[WW]` maior que
`br:[i]`, ambos medem `22.2552pt`: o `tr_post` domina o máximo e mascara a IC.
Invertendo (`tr:[i]`, `br:[WW]`), o candidato volta a perder `0.1760pt`.

### IC da base

Bases `Content::Text` com `br:[br]`:

| Base | IC fonte | Vanilla | Candidato | Delta |
|---|---:|---:|---:|---:|
| `[x]` | 16du = 0.176pt | 13.7236 | 13.5476 | -0.1760 |
| `[f]` | 79du = 0.869pt | 11.2816 | 10.4126 | -0.8690 |
| `[R]` | 24du = 0.264pt | 16.0116 | 15.7476 | -0.2640 |
| `[A]` | 0 | 16.1656 | 16.1656 | 0 |
| `[1]` | 0 | 13.4156 | 13.4156 | 0 |

A igualdade exacta `delta = −IC(base)` em três ICs não-zero e dois controles
zero refuta compensação nominal.

Bases MathIdent/GlyphFragment `x`, `f` e `R`, com subscrito math `b r`, são
exatas bilateralmente (`14.9314`, `14.0294`, `16.9884pt`). Portanto a IC é
legítima para GlyphFragment e ilegítima somente quando a base é TextItem/frame.

### Inline/block-display

O mesmo attach com markup `[x]` em `math.equation(..., block:true)` mantém o
delta `−0.1760pt`; mudar o modo não neutraliza a classificação da base. Já o
probe display público com átomos math (`attach(x, ... br: b r)`) é exato em
`22.1925 × 19.4843pt`. O discriminante é FrameFragment versus GlyphFragment,
não inline versus display.

Hashes de probes seleccionados:

| Fonte/output | SHA-256 |
|---|---|
| `full.typ` | `6c135e979542187d7cb286ea29d00ef5c401710e890348707c276afe3609face` |
| `full-no-br.typ` | `6bcf73f45e8948f72365d86e9a351e9cffecfa2deeb5d98c7282dd56bca2768e` |
| `only-tr.typ` | `5e349da976128ad52e8bd9aa3dfdc001c619325ef2fd6f509d004f329406e7adcd` |
| `only-br.typ` | `e9f9ed92bef843615ed57ab5780351c4e79451593acdce5702ac0a344389978c` |
| `base-f-br.typ` | `429217a726eded5ea3ecbe42c0d3565a40c61b56d9daa083d4fa07e8e5cbd3d4` |
| `base-r-br.typ` | `bea25576b91cae8f08711020a4e247d9b7f1208d429f87499cf5e1ac99e4791d` |
| public display source | `e233f5519925a6b3e4fa45dcbba6e03e447864514331c52d804ec1a3ac762f15` |
| public display vanilla SVG | `f3bf5fbbb3e9fffc3e612d5d0585c26016ecb4f51a47e9901bf2f8b8f4bc197f` |
| public display candidato SVG | `a4860b8d3e81ed4b84685ecb0dd0208b5ca75897e538b928f8b8ae1369920d74` |

## Vanilla file:line e fórmula causal

Vanilla `math/scripts.rs:220-240` calcula:

```text
br_kern = math_kern(base, br, ...) - base.italics_correction()
br_post = SpaceAfterScript + br.width + br_kern
br_x = pre_width + base.width + br_kern
```

Assim, vanilla usa a mesma IC tanto na posição do `br` quanto na extensão do
frame; não há regra “posição sim, frame não”. A distinção está no valor
semântico de `base.italics_correction()`:

- `math/fragment/mod.rs:145-150` despacha por variante;
- GlyphFragment devolve sua IC tipográfica;
- FrameFragment devolve seu campo próprio;
- `FrameFragment::new` em `mod.rs:239-255` inicializa esse campo em zero.

`TextItem` é disposto como `FrameFragment`, portanto sua IC para scripts é zero,
mesmo quando o último caractere da fonte possui IC. MathIdent é GlyphFragment e
mantém a IC da fonte.

No candidato, `attach.rs:341-370` procura o último `FrameItem::TextShaped` ou
`Text` e lê a IC crua do glyph/char sem consultar a proveniência do fragmento.
Depois `attach.rs:372-424` subtrai esse valor de `br_kern`, `br_post` e
`total_width`. A fórmula é mecanicamente semelhante ao vanilla, mas recebe o
estado causal errado: `16du` onde o FrameFragment vanilla fornece zero.

## ADR-0107/0108, owner e estado

ADR-0107: posição do subscrito e largura final da fórmula/página são geometria
observável da linguagem. Enum de fragmento, `FrameItem`, despacho Rust e acesso
à tabela MATH são mecânica diagnóstica.

ADR-0108: medição precede a decisão e inclui refutadores. A conclusão seria
refutada se:

- algum TextItem com IC não-zero não produzisse `delta = −IC` quando `br`
  domina;
- um TextItem com IC zero divergisse;
- MathIdent/GlyphFragment com a mesma base passasse a divergir;
- vanilla deslocasse `br` por IC crua de TextItem; ou
- remover/fazer `tr` dominar não neutralizasse o delta final.

Owner 1:1 exacto:
`00_nucleo/prompts/compiler/math/layout/attach.md` →
`01_core/src/compiler/math/layout/attach.rs`. Não é necessário alargar para
equation, shaper ou font_metrics.

O L0 vigente está contradito/insuficiente. `attach.md:576-583` subtrai
`italics_correction(base)` sem declarar que esse valor pertence ao fragmento
matemático e que TextItem/FrameFragment fornece zero; `:591-594` preserva “IC”
sem a classificação morfológica. Antes de código, o L0 deve distinguir:
GlyphFragment usa IC tipográfica; FrameFragment/TextItem usa sua IC semântica
(zero salvo wrapper que a preserve explicitamente). Isso não autoriza apagar IC
globalmente: os controles MathIdent refutam essa mudança.

Trata-se de correção interna de fórmula/paridade no owner existente, sem
constante, lista de caracteres, API, default ou fase. ADR-0127 permite fluxo
contínuo após L0 atualizado/resselado e RED→GREEN, mas este auditor não propõe
código nem aprovação.

Estado causal: **isolado, não Unknown**. Estado do lote B: **não aprovado**.

O SHA-256 deste recibo deve ser calculado externamente após a gravação.
