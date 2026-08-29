# Prompt L0 — `infra/export/gradients/adaptive` — Adaptive N multispace
Hash do Código: 694c5d3d

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/gradients/adaptive.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0091 §"Anotação cumulativa P274" Opção 1B

---

## Contexto

Refino qualitativo do número N de stops amostrados (P274):
- N=16 fixo (P270.1) substituído por adaptive baseado em ΔE Oklab.
- Aplicável a Linear+Radial RGB-family + perceptual; Conic preserved P272.

Helpers:
- `perceptual_distance_in_space` — ΔE entre duas cores num space dado.
- `adaptive_n_for_stops` — N adaptativo (16/32/64) baseado em max pair ΔE.

## Fórmula (P274)

| max ΔE | N |
|---:|---:|
| < 0.05 | 16 |
| < 0.30 | 32 |
| ≥ 0.30 | 64 (cap) |

## Restrições estruturais

- L3 puro. Lê `GradientStop`, `Color`, `ColorSpace`.
- `pub(crate)` — chamado por `super::super::builder::emit_gradient_objects`.
- Sem I/O. Cálculo puro.

## Interface

```rust
pub(crate) fn perceptual_distance_in_space(c1: Color, c2: Color, space: ColorSpace) -> f64;
pub(crate) fn adaptive_n_for_stops(stops: &[GradientStop], space: ColorSpace) -> usize;
```

## Invariantes

- N retornado é sempre potência de 2 entre 16 e 64.
- `perceptual_distance_in_space` em Oklab native (sem coerção a sRGB ΔE).

## Critérios de verificação

Tests `p274_*` (14 testes) em `super::super::tests`.

## P1229 — subdivisão adaptativa para SVG

Medição no vanilla ratificado `a51e02804`:
`typst-library/src/visualize/gradient.rs:905-964` subdivide cada intervalo
Linear/Radial por bissecção. Em cada segmento compara, no midpoint, a cor
exata do gradient com a interpolação sRGB entre as extremidades; a distância é
euclidiana em RGB premultiplicado, com limiar `0.001` e máximo de `64`
subdivisões. Aqui `to_rgb()` significa sRGB codificado; o tipo linear do
vanilla é `LinearRgb` e não participa da métrica de erro. O alpha é aplicado
por `premultiply()` antes da distância. `typst-svg/src/paint.rs:246-277` preserva os stops originais e
insere os intermediários resultantes.

O owner deve fornecer helper puro e interno para gerar os stops intermediários
de um intervalo de `Gradient::{Linear,Radial}`. Deve preservar offsets
originais, não atravessar intervalo de tamanho zero, manter alpha nas cores
amostradas e terminar deterministicamente no cap. Esta obrigação SVG é
separada do seletor histórico P274 `16/32/64`, que permanece inalterado para o
pipeline PDF.

Aceitação: contra oráculo independente, erro máximo em sRGB codificado
premultiplicado não excede `max(0.001, erro vanilla) + 1e-6`, percentil 95 não
excede o vanilla por mais de `1e-6`, e erro de alpha máximo/p95 não excede o
envelope medido do próprio vanilla por mais de `1e-6`. Em stops coincidentes,
o ponto exato é adjudicado estruturalmente pela ordem; a métrica contínua
começa em epsilon à direita. A quantidade literal de stops é mecânica; offsets declarados,
descontinuidades, ordem e envelope são observáveis.

## P1262 — gamut no critério adaptativo SVG

Medição no vanilla ratificado `a51e02804`:
`typst-library/src/visualize/gradient.rs:945-958` converte a cor exata e a
aproximação com `Color::to_rgb()` antes de premultiplicar. Para Oklab,
`typst-library/src/visualize/color.rs:1810-1821` usa
`palette::Rgb::from_color`; em `palette 0.7.6`, `FromColor` executa a conversão
sem clamp e depois aplica `Clamp`. Portanto, o vetor observado pela decisão de
bissecção é sRGB codificado em float com cada canal limitado a `[0, 1]`, ainda
antes de multiplicar pelo alpha e antes de qualquer quantização para `u8`.

O helper SVG deve espelhar essa fronteira localmente em dois pontos. A
aproximação sRGB do midpoint converte e limita a `[0, 1]` os canais RGB de
cada extremidade antes da soma ponderada `0.5/0.5`, como
`Color::mix_iter(..., Srgb)` através de `ProcessColor::to_space`. Depois, para
calcular o erro, a cor exata e a aproximação são novamente convertidas a sRGB
float limitado, premultiplicadas com o alpha float inalterado e comparadas por
distância euclidiana. O clamp não pode
ser antecipado para a amostragem ou para a interpolação no espaço nativo, nem
adiado para a serialização `u8`. `Color::to_rgba_f32`, `Color::to_srgb`, o cap
64, o limiar `0.001`, os offsets e os demais consumers permanecem inalterados.
Esta fronteira aplica-se somente ao espaço Oklab adjudicado pelo P1262;
LinearRgb e os demais espaços retêm o caminho P1229/P1261 até os seus próprios
clusters serem materializados. Esta é uma correção interna de paridade do
owner adaptativo, não uma nova política pública de gamut.

O parâmetro da bissecção e os offsets Oklab permanecem `f64` desde `Ratio`
até o cálculo do peso local. Só então `w0 = (1-local) as f32` e
`w1 = local as f32` são materializados separadamente; cada componente usa
`(w0*c0 + w1*c1)/(w0+w1)`, espelhando `sample_stops` + `Color::mix_iter` do
vanilla. Esta precisão vive no sampler privado do owner SVG: não altera o
método Rust histórico `Gradient::*::sample(f32)` nem cria API pública nova.
O mesmo sampler ponderado atende LinearRgb sem receber o clamp Oklab novo;
isso preserva bit a bit as 24 fixtures congeladas do P1261, inclusive os
empates de alpha, enquanto a decisão de erro LinearRgb continua no caminho
P1229/P1261.

Aceitação focal: os quatro stopsets Oklab saturados `base`, `coincident`,
`alpha-first` e `alpha-mid`, tanto Linear como Radial, usam o mesmo critério
do vanilla sob o limiar e cap congelados; os oito envelopes `color_max` e
`color_p95` ficam dentro do budget P1231/P1237 e alpha não regride. Comparação
apenas após `u8`, sucesso automático por saturação, coerção do espaço Oklab,
alargamento do budget ou do cap e alteração global de `Color` são proibidos.

## P1278 — precisão e gamut dos espaços polares SVG

Medição que precede a decisão: o certificado P1277 executou 144 fixtures
Linear/Radial × Oklch/Hsl/Hsv. Grafo e custo fecharam 144/144, mas somente
84/144 fecharam o envelope numérico. A fonte ratificada
`typst-library/src/visualize/gradient.rs:905-964,1461-1485` conserva `t` e os
offsets em `f64`, converte separadamente `(1-local)` e `local` para pesos
`f32`, divide cada soma pelo total dos pesos e, nos espaços com hue, corrige a
rota curta antes da soma. O caminho cristalino anterior reduzia `t` a `f32`
antes de chamar `Gradient::sample` e usava a forma algébrica `a+(b-a)*t`.
Essas mecânicas não são equivalentes nos limites congelados P1277.

Decisão interna: `sample_gradient_precise_for_svg` passa a atender também
Oklch, Hsl e Hsv. Ele resolve offsets em `f64`, calcula os dois pesos `f32`
separadamente, usa `(w0*c0+w1*c1)/(w0+w1)` por componente e aplica a correção
angular curta exatamente nos índices Oklch=2 e Hsl/Hsv=0 antes da soma. O
resultado permanece no espaço nativo. A rota P1262 de conversão sRGB float
limitada antes da premultiplicação também se aplica a Oklch, pois a mesma
fonte vanilla converte todo `Color::to_rgb()` por `palette::FromColor` com
clamp; Hsl/Hsv já produzem canais codificados no intervalo e conservam a
mesma fronteira sem clamp adicional observável.

Luma fica explicitamente fora deste refinamento: P1277 encontrou 32 fixtures
com divergência pública de alpha já classificada, que densidade, precisão ou
serialização de offsets não corrigem. CMYK continua `Unknown-ADR0097`. O
limiar `0.001`, cap 64, bissecção, custo por intervalo, ordem, coincidência,
right-continuity e algoritmo PDF P274 não mudam. O passo não promove nenhum
paint; somente torna o adaptador diagnóstico capaz de nova adjudicação.

Aceitação: os seis pares polares repetem as 24 famílias P1266 sem alargar
budgets P1277, tolerância ou cap; execução inversa e repetida é determinística;
qualquer par que não feche a conjunção permanece `Unknown`.
