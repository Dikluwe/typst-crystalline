# Passo 953 — Relatório (vocabulário PDF: lacuna real vs codificação equivalente)

**Data**: 2026-08-03
**Estado da árvore**: commit base `279e6d0a3` (P951); alterações deste passo por cima.

---

## 1. Veredicto por capacidade (testada com documentos mínimos, `temp/p953/caps.typ`)

| Capacidade | Veredicto |
|---|---|
| **Cor mudando dentro do parágrafo** (`#text(fill:)` ×3) | **Funciona** — o cristalino emite a mudança de cor na transição, com codificação própria (fill por span) equivalente ao `cs`/`scn` do vanilla. Sem defeito. |
| **Escala de texto** (`#scale(150%)`) | **Era bug real** — factor posicional ignorado. **Corrigido neste passo.** |
| **Rotação de texto** (`#rotate(45deg)`) | **Funciona** — a direcção estava e está correcta (horária descendente, igual ao vanilla; ver §2 para a falsa suspeita). O cristalino usa `FrameItem::Group` + matriz afim no `cm` (não `Tm`), posições pré-calculadas — **codificação diferente mas equivalente** (ADR-0107). |
| **Conteúdo aninhado com transformação herdada** (`#box(fill)[#rotate[...]]`) | **Funciona estruturalmente** (o conteúdo renderiza transformado, com a matriz herdada), com lacunas de **placement** registadas (§3). |
| **PDF tagueado (`BDC`/`EMC`)** | **Scope-out mantido** (decisão de produto já tomada — ver §4). |

## 2. Correcção real implementada: parsing do factor de `scale()`

**Bug** (medido no render e no trace): `#scale(150%)[grande]` renderizava a 11pt —
`native_scale` só lia `x`/`y` nomeados e só `Float`/`Int`; o factor posicional
`Ratio` (a forma natural, aceite pelo vanilla como primeiro posicional —
`ScaleElem.x` com `#[positional]`, `layout/transform.rs:113-136`) era ignorado e a
matriz ficava identidade.

**Correcção** (`01_core/src/engine/stdlib/transforms.rs` + L0
`engine/stdlib/transforms.md` §P953): `native_scale` aceita o factor como
posicional (1º = x, 2º = y) e `Value::Ratio` em ambos os canais; nomeados
`x:`/`y:` mantidos. **Testes** (red → green, em `stdlib/mod.rs`):
`p953_native_scale_factor_posicional_ratio` (`#scale(150%)` →
`TransformMatrix::scale(1.5, 1.5)`), `p953_native_scale_dois_posicionais_e_ratio_nomeado`
(2 posicionais + nomeado Ratio). O layout/export já aplicava a matriz ao grupo —
a correcção era só no parsing.

**Registo honesto (a falsa suspeita de rotação)**: a meio da Fase A suspeitei
que `#rotate` estava com a direcção invertida e cheguei a alterar L1 e a fórmula
do `cm` no export. A medição final (zoom a alta resolução + transformação
efectiva do texto nos dois PDFs) mostrou que **o estado original já estava
correcto** (vanilla = `\` descendente, igual ao cristalino original) — a leitura
inicial dos crops estava errada. **Tudo foi revertido sem deixar rasto**; o L0
(`infra/export/stream.md` §P953) regista a convenção como verificada contra o
vanilla para não ser questionada de novo sem medição equivalente. Lição:
verificar a direcção do texto no render isolado a alta resolução antes de mexer
em convenções de matriz.

## 3. Lacunas registadas (não corrigidas neste passo — escopo arquitectural próprio)

1. **Placement de transformações é por bloco, não inline**: `transform.rs` faz
   sempre `flush_line()` e não avança `cursor_x` — conteúdo transformado quebra
   a linha mesmo quando o vanilla o colocaria inline, e desloca a posição dos
   elementos seguintes (ex.: sobreposição residual de ~alguns pontos entre
   `#rotate` e o elemento seguinte; a origem da rotação no vanilla é
   `center + horizon` por omissão, não implementada no cristalino).
2. **`#box(fill:)` com conteúdo transformado**: a largura da caixa deriva do
   avanço do cursor (≈ 0 para conteúdo transformado, por (1)) — o fundo fica
   com largura quase nula e o conteúdo escapa os limites da caixa (medido no
   render: rectângulo pequeno destacado + texto fora). Consequência directa
   de (1) — a correcção é a mesma frente arquitectural (transformações inline),
   candidata a passo dedicado (envolve `transform.rs`, `boxed.rs`, e a
   maquinaria P832/P908 de sub-frames).

## 4. PDF tagueado/acessibilidade — decisão

**Scope-out mantido.** A categoria `BDC`/`EMC` (conteúdo marcado) já era
scope-out deliberado de longa data (mesma lista de CJK vertical e pacotes
`@preview`). Não é um bug — é uma decisão de produto. Se se quiser promover a
prioridade activa, é uma decisão separada do dono, fora do âmbito técnico deste
passo. Registado aqui apenas como reconfirmação explícita, como o passo pede.

## 5. Validação

- `cargo test --workspace`: **9 suites verdes, 0 falhas** (incluindo os 2 testes
  novos P953).
- `crystalline-lint .`: **zero violations** (`stream.rs` resselado; resta só o
  V7 pré-existente alheio).
- Render end-to-end (`caps.typ`): rotação na direcção correcta, escala 150%
  aplicada, cores por transição, aninhado com matriz herdada — lado a lado com
  o vanilla (`temp/p953/caps-cmp-final.png`).
- Benchmark: ver tabela (hyperfine, "antes" = binário de `279e6d0a3` em
  worktree; corpus canónico; JSONs em `tools/perf/results/p953-*.json`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 88.54 | 87.91 | 0.993 |
| 02-lorem | 106.82 | 107.78 | 1.009 |
| 03-images | 95.71 | 95.46 | 0.997 |
| 04-math | 120.47 | 119.74 | 0.994 |
| 05-tables | 95.37 | 97.55 | 1.023 |
| 06-long | 300.01 | 300.72 | 1.002 |
| 07-context | 139.62 | 134.92 | 0.966 |

Ratio médio **0.998** — zero regressão (variação dentro do ruído de medição).
