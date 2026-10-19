# Passo 1094 — Relatório: Auditoria de Literais de Conversão Truncados

## 1. Metodologia e Extração via Linter (Regra V21)

A auditoria utilizou o motor do `crystalline-lint` (regra V21 — *Escalar Contextual sem Proveniência*) e varredura sistemática sobre todo o código de produção (`01_core/src/`, `02_parser/src/`, `03_infra/src/`, `04_wiring/src/`).

Foram extraídos todos os literais de ponto flutuante do código de produção e comparados numericamente contra a tabela de razões e constantes físicas/matemáticas canônicas do ecossistema Typst.

---

## 2. Matriz de Comparação contra Razões Conhecidas

| Grandeza / Razão | Fórmula Canônica | Valor Canônico (f64) | Implementação no Crystalline | Delta em f64 |
|---|---|---|---|---|
| **Centímetro (`cm`)** | $3600 / 127$ | `28.346456692913385… pt` | `Length::PT_PER_CM` (P1093) | **Identidade binária em f64** ($|\Delta| < \epsilon_{	ext{f64}}$) |
| **Milímetro (`mm`)** | $360 / 127$ | `2.8346456692913385… pt` | `Length::PT_PER_MM` (P1093) | **Identidade binária em f64** ($|\Delta| < \epsilon_{	ext{f64}}$) |
| **Polegada (`in`)** | $9144 / 127$ | `72.0 pt` | `Length::PT_PER_IN` (P1093) | **Identidade binária em f64** ($|\Delta| = 0.0$) |
| **Ponto Tipográfico (`pt`)** | $127 / 127$ | `1.0 pt` | `Abs::pt(v)` | **Identidade binária em f64** ($|\Delta| = 0.0$) |
| **Ângulo: Graus → Radianos** | $\pi / 180$ | `0.017453292519943295… rad` | `f64::to_radians()` (std) | **Identidade binária em f64** ($|\Delta| < \epsilon_{	ext{f64}}$) |
| **Ângulo: Radianos → Graus** | $180 / \pi$ | `57.29577951308232… deg` | `f64::to_degrees()` (std) | **Identidade binária em f64** ($|\Delta| < \epsilon_{	ext{f64}}$) |
| **PNG pHYs (ppm → DPI)** | $1 / 0.0254$ | `0.0254 m/in` (definição SI) | `0.0254` (`image_sizer.rs:74`) | **Identidade binária em f64** ($|\Delta| = 0.0$) |
| **JPEG JFIF (ppcm → DPI)** | $1 / 2.54$ | `2.54 cm/in` (definição SI) | `2.54` (`image_sizer.rs:77`) | **Identidade binária em f64** ($|\Delta| = 0.0$) |
| **Bézier Circle $\kappa$** | $rac{4(\sqrt{2}-1)}{3}$ | `0.5522847498307936…` | `pdf_defaults::BEZIER_CIRCLE_KAPPA` | **Identidade binária em f64** ($|\Delta| < \epsilon_{	ext{f64}}$) |

*Nota metodológica*: Para razões que geram dízimas periódicas ($3600/127$) ou irracionais ($\pi$, $\kappa$), a representação é exata no limite de precisão do formato IEEE 754 em precisão dupla (`f64`, $\epsilon pprox 2.22 	imes 10^{-16}$).

---

## 3. Achados da Auditoria e Correções Executadas

### 3.1. Unidades de Comprimento e Ângulos (AST / Runtime)
- **Vanilla Typst** (`lab/typst-original/crates/typst-library/src/layout/abs.rs:265`): Utiliza `AbsUnit::raw_scale` com razões inteiras $\{127, 360, 3600, 9144\}$.
- **Crystalline**: Confirmado que após o P1093 todas as 9 variantes do enum `Unit` (`Pt`, `Mm`, `Cm`, `In`, `Rad`, `Deg`, `Em`, `Fr`, `Percent`) operam sobre as constantes canônicas centralizadas de `Length` e da biblioteca padrão Rust (`to_radians`).

### 3.2. Achado Real 2 — Duplicação e Truncamento de $\kappa$ em `render.rs` (Corrigido)
- **Problema**: Em `03_infra/src/export/render.rs:488, 489, 506` (rasterizador tiny-skia para cantos arredondados e elipses), existiam 3 literais decimais fixos `0.55228475` (truncados a 8 casas em vez de reutilizar a constante canônica), com resíduo de $+1.69 	imes 10^{-10}$ frente ao valor matemático exato:
  $$\Delta = 0.55228475 - 0.5522847498307936 = \mathbf{+1.6917 	imes 10^{-10}}$$
- **Correção Aplicada**: Importado `pdf_defaults` em `render.rs` e substituídos os 3 literais por `pdf_defaults::BEZIER_CIRCLE_KAPPA as f32`, unificando a fonte de verdade em todo o compilador (vetorial e raster).

---

## 4. Recomendação Formal ao Mantenedor do `tekt-linter`

Com base nos dois casos reais confirmados nesta investigação (literais aproximados inseridos ad-hoc em vez de constantes centrais: `28.346` no P1093 e `0.55228475` no P1094), recomendamos a inclusão de uma regra de linting dedicada:

- **Identificador Proposto**: `V25: truncated-canonical-constant`
- **Motivação**: Prevenir que literais numéricos com representação racional ou matemática exata sejam codificados com truncamento manual de ponto flutuante em qualquer camada do sistema.
- **Mecanismo**: Alertar quando um literal float estiver a uma distância $0 < |\Delta| < 10^{-3}$ de uma constante canônica conhecida da tipografia / PostScript ($3600/127$, $360/127$, $4(\sqrt{2}-1)/3$, $\pi$, $	ext{golden ratio}$), exigindo o uso da constante centralizada (`Length::PT_PER_CM`, `pdf_defaults::BEZIER_CIRCLE_KAPPA`, etc.).

---

## 5. Conclusão

- A auditoria identificou e saneou os dois pontos de truncamento de constantes do sistema:
  1. As conversões de unidades de comprimento no parser/eval (`28.346` / `2.8346` corrigidos no P1093).
  2. A duplicação de $\kappa$ de Bézier no rasterizador tiny-skia (`0.55228475` corrigido no P1094).
- O workspace compila sem erros e passa em **100% dos testes** (**5.955 testes aprovados**).
