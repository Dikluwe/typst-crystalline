# Passo 969 — Fase A: desenho do módulo oráculo (PARADO no gate, ADR-0127)

**Data:** 2026-08-05 · **Estado da árvore:** HEAD = `2337dc435` (P968), working
tree limpa. Nenhum código escrito; nenhum L0 editado (ver §5).

## 1. Escopo (Fase A.1)

**Não** é um motor de layout paralelo. O oráculo é um conjunto de **funções
puras isoladas, uma por fórmula do vanilla já lida e confirmada nesta
frente**, cada uma:

- recebe os mesmos parâmetros primitivos que o código de produção usa
  (constantes MATH em pt, dimensões de conteúdo — ascent/descent/largura);
- devolve a posição/gap esperado segundo a **transcrição literal** da
  fórmula do vanilla, com `file:line` da fonte no doc-comment;
- não chama `MathLayouter`, `FontMetrics`, nem qualquer código de produção
  (dependência zero — o oráculo não pode falhar em conjunto com o motor).

O inventário completo das fórmulas confirmadas (38 entradas, com fórmula,
`file:line` do vanilla, ficheiro cristalino e passo de confirmação) foi
levantado dos relatórios P901–P968 e dos L0 de `math/layout/` — os
candidatos nomeados pelo passo estão cobertos: shift de limites de
operadores grandes (P959, `scripts.rs:290-313`), ancoragem/altura de
grelha (P945/P952, `table.rs:103-106`, `run.rs:137`, `table.rs:188`).

## 2. Localização (Fase A.2)

**`01_core/src/testing/math_oracle.rs`**, declarado como
`#[cfg(test)] pub(crate) mod testing;` em `lib.rs`.

- Dentro de L1 porque as funções são puras (respeita as restrições de L1
  por construção) e porque os consumidores são os testes de unidade em
  `engine/math/layout/tests.rs` — sem crate novo, sem wiring.
- `#[cfg(test)]` deixa explícito que **não é caminho de execução real**:
  o código de produção não compila o módulo, não o pode importar, e o
  binário não o carrega.
- Header de linhagem normal (`@prompt 00_nucleo/prompts/testing/
  math_oracle.md`), como qualquer ficheiro L1 — o linter lê o ficheiro
  mesmo sob `cfg(test)` (precedente: `tests.rs` existentes têm header).

Alternativa considerada e rejeitada: crate de desenvolvimento separado —
wiring novo no workspace sem benefício (o oráculo não precisa de I/O nem de
dev-dependencies), e afastaria os testes da fórmula que verificam.

## 3. Formato de uso (Fase A.3)

Cada teste de geometria passa a ter a forma:

```rust
let esperado = testing::math_oracle::large_operator_upper_shift(
    base_ascent, limite_descent, upper_gap_min, upper_rise_min);
assert!((y_obtido - esperado).abs() < 0.01);
```

Os valores hardcoded soltos nos testes actuais (ex.: os asserts numéricos de
P959/P945/P952) são substituídos pela chamada ao oráculo **com os mesmos
inputs** — a leitura do vanilla fica registada uma vez, em código, com
`file:line`, em vez de re-feita a cada achado.

## 4. População inicial (Fase A.4)

Primeiro lote (os candidatos do passo, todos com `file:line` exacto):

1. `large_operator_upper_shift` / `large_operator_lower_shift` —
   `scripts.rs:290-313` (P959).
2. `grid_total_height` — `table.rs:103-106` (P945/P921).
3. `grid_cell_dy` (ancoragem de célula, tradução baseline-relativa de
   `run.rs:137`) — P952.
4. `grid_axis_baseline` — `table.rs:188` (P919).

Segundo lote (mesma mecânica, file:line confirmado): short-fall de
delimitador (P912), laço/passo de assembly (P913/P957), gaps de fracção
(P920), gaps de acento (P922), shifts de scripts (P914), spaced-item
fallback (P903/P907/P967 — requer re-confirmar o file:line de
`item.rs::is_spaced`, ausente nos relatórios).

Excluída com nota: a fórmula #19 do inventário (`binom` como frac-like,
`resolve.rs:713-748`, P946) — lida e confirmada mas **não portada** no
cristalino; um oráculo dela daria RED permanente. Fica de fora até haver
passo que porte a mecânica.

## 5. L0 e gate (Fase A.5)

Nenhum L0 existente precisa de edição nesta fase (o desenho não altera
nenhuma especificação vigente). O único artefacto L0 novo é o prompt do
módulo — **não o materializei em `00_nucleo/prompts/` de propósito**: um
prompt sem nenhum ficheiro L1–L4 a referenciá-lo gera um órfão V7 no
linter (o mesmo padrão do `package_version_resolution.md`), e o critério
primário é `crystalline-lint .` limpo. O rascunho completo segue abaixo
para revisão; na confirmação, a Fase B cria **num só gesto** o prompt, o
módulo com o header (hash resselado) e o primeiro lote — sem janela de
órfão.

---

### Rascunho do L0 — `00_nucleo/prompts/testing/math_oracle.md` (para o dono guardar)

```markdown
# testing/math_oracle — oráculo de fórmulas de posição do vanilla (P969)

**Data:** 2026-08-05 · **Passo:** 969 · **Camada:** L1 (`#[cfg(test)]`)

## Propósito

Módulo de verificação, **não** caminho de execução. Conjunto de funções
puras, uma por fórmula de posição do vanilla já lida e confirmada na frente
P901–P968, transcritas literalmente com `file:line` da fonte
(`lab/typst-original/...`) no doc-comment. Usado pelos testes de geometria
para comparar o motor de produção contra a fórmula de referência, em vez de
valores hardcoded soltos.

## Regras do módulo

1. `01_core/src/testing/math_oracle.rs`, declarado
   `#[cfg(test)] pub(crate) mod testing;` — nunca compilado em produção.
2. Funções puras sobre primitivos (`f64`/`Pt`): constantes MATH e dimensões
   entram como parâmetros; **proibido** importar `MathLayouter`,
   `FontMetrics`, `Content` ou qualquer código de produção — o oráculo não
   pode partilhar falhas com o motor que verifica.
3. Cada função documenta: fórmula literal, `file:line` do vanilla, passo
   que a confirmou.
4. Uma fórmula sem `file:line` confirmado **não entra** — re-ler a fonte
   primeiro (disciplina ADR-0108: medir antes de decidir).
5. Fórmulas lidas mas deliberadamente não portadas no cristalino (ex.:
   `binom` frac-like, P946) não entram — o oráculo espelha o que o motor
   pretende implementar, não o catálogo inteiro do vanilla.

## Funções do lote inicial

| Função | Fórmula vanilla | Fonte | Passo |
|---|---|---|---|
| `large_operator_upper_shift(base_ascent, t_descent, gap_min, rise_min)` | `base.ascent + max(rise_min, gap_min + t.descent)` | `typst-layout/src/math/scripts.rs:290-313` | P959 |
| `large_operator_lower_shift(base_descent, b_ascent, gap_min, drop_min)` | `base.descent + max(drop_min, gap_min + b.ascent)` | `scripts.rs:290-313` | P959 |
| `grid_total_height(ascents, descents, gap)` | `Σ(ascent_r + descent_r) + gap × (n−1)` | `typst-layout/src/math/table.rs:103-106` | P945/P921 |
| `grid_cell_dy(baseline_offset, row_ascent, cell_ascent)` | tradução baseline-relativa de `pos.y = size.y + row_ascent − sub.ascent` | `typst-layout/src/math/run.rs:137` | P952 |
| `grid_axis_baseline(height, axis)` | `height/2 + axis` | `table.rs:188` | P919 |

## Uso

Testes de geometria chamam a função com os mesmos inputs do código de
produção e comparam (tolerância 0.01pt). Valores esperados soltos em testes
existentes migrados para chamadas ao oráculo à medida que cada fórmula
entra.
```

---

**PARADO no gate** (ADR-0127 — infraestrutura nova de escopo a confirmar).
À confirmação do dono (com ou sem ajustes ao escopo/localização/lote),
executo a Fase B: criação do prompt + módulo + primeiro lote, migração dos
asserts correspondentes, suíte verde, linter limpo — e a Fase C usa o
oráculo na investigação de um dos achados 9.1–9.3.
