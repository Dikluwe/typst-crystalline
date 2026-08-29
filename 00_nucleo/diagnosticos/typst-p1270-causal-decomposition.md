# P1270 — decomposição causal das divergências numéricas

**Estado:** EXECUTADO — diagnóstico causal, sem implementação
**HEAD medido:** `3bc6f5a683cd2df9b4281f1debf46f1b052ad463`
**Árvore:** não commitada
**Medição final do runner:** `2026-08-29T08:32:58-03:00`
**Regime Tekt:** protocolo completo com segregação causal/processual; sem
isolamento técnico independente, porque os papéis partilharam `/root` e o
mesmo workspace. Nenhuma alegação de atestação independente é feita.

## 1. Obrigação e inputs congelados

A obrigação é `typst-passo-1270.md`; a população normativa é exclusivamente
`p1269-owner-numeric-failure-freeze.tsv`: 24 fixtures e 28 métricas falhadas.
O manifesto `p1270-input-manifest.tsv` fixa passo, predecessor, freeze,
budgets, matriz, binários e owner adaptativo. Baseline, budgets, P1269 e L0
foram somente leitura. Nenhum código produtivo foi alterado.

O runner causal é `lab/parity/matrix/p1270_causal.py`; o probe
`p1270_probe` difere do probe P1266 em uma única variável: componentes de cor
continuam `f32`, mas o quinto campo, offset, é reparsado como `f64`.

Receipts brutos:

- `p1270-checkpoints.tsv` — SHA-256
  `cc346e754d181e3395eb73785af773c7dbb30de9b7418c296dfde14ba431bc13`;
- `p1270-counterfactuals.tsv` — SHA-256
  `94684b68b558752f8d4e7286dacd0b1af22523855d3f0329a1b4b41d0be7b731`;
- `p1270-fixture-causes.tsv` — SHA-256
  `8cbace14ff1b2b16924d46e01e5e582bc2c818b54e1c8ef5ce652a1635353f75`.

As 28 fresh limits reproduziram os limites congelados com delta absoluto
máximo `0.0`.

## 2. Medições antes das decisões

### 2.1 Checkpoints comuns

O vanilla amostra todas as variantes com `t: f64` em
`lab/typst-original/crates/typst-library/src/visualize/gradient.rs:846-860`
e `:1461-1484`. A decisão adaptativa usa threshold `0.001`, cap `64`, midpoint
exato, mistura sRGB, premultiplicação e bissecção em `:903-965`; o SVG
serializa `Ratio::repr` e `to_hex` em
`lab/typst-original/crates/typst-svg/src/paint.rs:249-281`.

O cristalino adaptativo recebe offsets `f64`, amostra no espaço nativo,
converte/clampa/premultiplica e decide a bissecção em
`03_infra/src/export/gradients/adaptive.rs:84-292`. Sob stops públicos e
sampling exato equivalentes ao vanilla, o seu stopset serializado preservou
as 28/28 métricas. Logo a primeira causa não está nesse owner.

### 2.2 C1 — carrier do adaptador diagnóstico

O probe P1266 reparsa os cinco campos como `f32` e só depois constrói
`Ratio(f64::from(values[4]))` em
`lab/parity/matrix/p1266_probe/src/main.rs:41-57`. Em S03, `1/27` muda de
`0.037037037037037` para `0.03703703731298447`. Mantendo todo o resto,
inclusive serialização e budget, `color_p95` muda de
`0.002608900768223896` (falha) para `0.002604143738174442` (preserva limite
`0.0026050877217766648`). O mesmo contrafactual fecha 16/16 métricas C1.

### 2.3 C2 — conversão pública sRGB → Oklab

Nos quatro witnesses S06/S22 Oklab, a primeira diferença é o componente 1 do
stop público 1: vanilla `0.000011444091796875`, cristalino `0.0`. Os
constructors chamam `Color::to_space` em
`01_core/src/compiler/stdlib/gradients.rs:544-548` e `:851-855`; o owner da
conversão é `01_core/src/entities/color.rs:533-540`, enquanto o vanilla delega
a `palette::Oklab::from_color` em
`lab/typst-original/crates/typst-library/src/visualize/color.rs:1758-1768` e
`:1784-1794`.

Com sampling vanilla fixo, manter metadata candidata deixa S06 linear Oklab
em `0.003296418699165581`; trocar apenas a metadata pela vanilla produz
`0.003161729858804235`, dentro do limite `0.003162729858804235`. O efeito
fecha 4/4 métricas C2.

### 2.4 C3 — repeat consome offsets já estreitados

`resolved_stops` resolve todos os offsets em `Vec<f32>` e os reexpande para
`Ratio(f64)` em `01_core/src/compiler/stdlib/gradients.rs:333-343`; `repeat`
consome esse resultado em `:470-483`. O vanilla preserva `Ratio(f64)` ao
repetir em `lab/typst-original/crates/typst-library/src/visualize/gradient.rs:664-688`.
Em S16, a fronteira repetida `1/6` é `0.1666666666666665` no vanilla e
`0.1666666716337204` no cristalino.

Mantendo o sampling candidato, trocar apenas os stops repetidos pelos do
vanilla muda `color_p95` linear/LinearRgb de `0.0025028268085903692` para
`0.002491067655848523`, preservando o limite `0.002492069229077084`. Fecha
4/4 métricas C3.

### 2.5 C4 — sampling público Radial estreita `t`

`sample_gradient` preserva `t: f64` apenas para Linear; Radial executa
`v.sample(t as f32)` em `01_core/src/compiler/stdlib/gradients.rs:275-287`.
`Radial::sample` também resolve offsets e o parâmetro local em `f32` em
`01_core/src/entities/gradient.rs:640-717`. O vanilla mantém `f64` até criar
os pesos de mistura.

Em S11 radial/Oklab, com o mesmo stopset candidato f64 e o mesmo budget,
trocar apenas a malha exata candidata pela vanilla muda `color_max` de
`0.017216382675147714` para `0.017214349542560282`, preservando o limite
`0.017215349542555467`. O contrafactual fecha 4/4 métricas C4, incluindo
`alpha_max` de S12.

## 3. Decisões ADR-0107/0108

- **C1 = `CONTRACT-ERROR`.** É mecânica do adaptador diagnóstico, não
  comportamento da linguagem nem autorização para alterar produto.
- **C2, C3 e C4 = `PRODUCT-DIVERGENCE`.** Alteram componentes, offsets ou
  cores amostradas observáveis e, portanto, semântica/morfologia da linguagem;
  não são diferenças toleráveis da estrutura Rust.
- Não se inferiu intenção a partir do comportamento. A intenção vigente de
  paridade nos L0s é separada dos valores medidos acima. Cada inferência seria
  refutada se o seu contrafactual de uma variável não movesse ou não fechasse
  todas as métricas atribuídas; nenhum desses contraexemplos ocorreu.
- `UNKNOWN = 0`. Não houve alargamento de budget, edição de baseline ou
  promoção produtiva.

A tabela normativa por métrica é `p1270-fixture-causes.tsv`; métricas distintas
do mesmo fixture podem ter causas diferentes, como S06 radial/Oklab (C2 no
`color_p95`, C4 no `color_max`). Isso impede agrupar apenas pelo sintoma.

## 4. Propostas L0 e ordem de materialização

As propostas estão em `p1270-l0-proposals.tsv`; nenhum L0 foi editado neste
passo. A ordem é:

1. reparar C1 no contrato/probe e reexecutar o freeze;
2. materializar C2 no owner `entities/color`;
3. materializar C3 nos owners `entities/gradient` + `stdlib/gradients`;
4. materializar C4 nos mesmos owners, com witness RED independente.

Cada materialização futura deve editar L0 primeiro, ressellar os hashes,
demonstrar RED→GREEN apenas no seu cluster e reexecutar os demais clusters
para provar que uma correção não mascarou outra.

## 5. Ataques e veredito

`p1270-attacks.tsv` registra dez ataques. Em particular, o mutante que culpa o
adaptativo é rejeitado por 28/28 preservadas sob entradas equivalentes; remover
a serialização não é solução (8 métricas continuam falhando); e os quatro
mutantes inversos específicos fecham C1–C4 sem alterar budgets.

**Veredito:** P1270 está causalmente decomposto: 16 métricas de
`CONTRACT-ERROR`, 12 de `PRODUCT-DIVERGENCE`, zero `UNKNOWN`. O passo não
autoriza código produtivo por si; autoriza iniciar as materializações na ordem
acima, respeitando L0-first e testes RED→GREEN.

## 6. Verificação de fecho

- reexecução causal: 24 fixtures, 28 métricas, 224 linhas contrafactuais;
  checkpoints e contrafactuais coincidiram byte a byte com os receipts;
- probe diagnóstico: 5 testes passaram;
- `cargo build`: passou (warnings preexistentes);
- `crystalline-lint .`: exit `0` (avisos consultivos V16–V20 permanecem no
  repositório; nenhuma violação bloqueante);
- `git diff --check`: passou;
- estado exato da árvore e hashes dos ficheiros diagnósticos ignorados estão
  em `p1270-working-tree-snapshot.txt`.
