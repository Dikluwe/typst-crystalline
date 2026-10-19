# Relatório P330 — Lote 15 (`Block`, o último) + baseline de performance pré-F

**Pré-condição**: P329 fechado — triagem gravada, Lote 14 fechado, lint 0, suíte
verde (typst-core 2693). ✅ Verificado.
**Commits**: `Passo 330 — lote 15` e `Passo 330 — baseline de performance pré-F`
(isoláveis).

---

## Parte 1 — Lote 15: `Block` (o último lote do roteiro)

**Composição: `Block`(121)** — variante única, **a mais densa** (14 campos: body
+ 13 cosméticos), família L12/`Boxed`.

### Forma

| Aspeto | Resultado |
|--------|-----------|
| Campos | `body`, width, height, inset, breakable, outset, radius, clip, fill, stroke, spacing, above, below, sticky |
| **Locatável** | **não** (`locatable.rs:150`) |
| `is_empty`/`plain_text` | `body.is_empty()` / `body.plain_text()` (delegam) |
| `map_content`/`map_text` | recursam body, preservam os 13 cosméticos (`..(*self).clone()`) |
| `Hash` | **manual** (`Length`/`Sides`/`Corners`/`Color`/`Stroke`, f64) |
| Construtor | `block(...)` cobre **5/14** → construções completas via **Arc-wrap** (C2; materialize `..(**e).clone()`) |
| `\|`-combinados | **nenhum** — os arms de `Styled` e dos 7 primitivos ficam intactos |

### Consumo pela introspecção

O walk arm (`introspect.rs:1225`) **só desce no body** — `Block` **não** é
consumido para payload (contraste `Labelled`/P195D, L14). A migração só converte
o lado `Content::Block` dos arms (materialize, walk, layout ×3 + o
sticky-lookahead `if matches!(part, Content::Block(e) if e.sticky)`).

### C1…C2 — sites tratados à mão (zero conversões indevidas)

Skip-list: **42 sites**, aos dois transformadores. **Achado de tooling**: o
transformador de construções não tinha `Block` no dict (mesmo crash do P329 com
`Boxed`) → **adicionado** antes da passada. Construções: 43 Arc-wrap (37
layout/tests + 6 content). Tratados à mão: arms de produção (introspect
materialize struct-update/walk, layout ×3, stdlib/layout construção);
sticky-lookahead → guarda `matches!(… if e.sticky)`; **2 nested
`Block { stroke: Some(s), .. }`** (binding `s` perdido) → `let s =
e.stroke.clone().unwrap();`; Arc move-outs (`e.stroke.unwrap()` → `.clone()`).
**Verificação pós-passada**: ✓ interseção vazia.

### Medições do lote (ADR-0104)

- **`cargo build`**: limpo. **Suíte**: typst-core **2697** (era 2693; **+4**),
  0 failed; `typst-infra` 472, restantes verdes.
- **`content.rs`**: **5072 → 4960** (−112; **abaixo de 5000**). **Parte
  atómica**: `block.rs` = **136 linhas**.
- **`crystalline-lint`**: 0 drift; **✓ No violations found**.

---

## Balanço FINAL da migração D (consolidação)

**65 variantes migradas** em **15 lotes + piloto** (P316 → P330).

### Trajetória completa do hub (`content.rs`)

| Marco | `content.rs` | Acumulado |
|-------|-------------|-----------|
| P313 baseline | ~5782 | — |
| P316 piloto (3) | 5785 | +setup trait |
| … Lotes 2–11 … | 5785 → 5395 | −390 |
| P327 L12 bloco grid/table (4) | 5157 | −238 |
| P328 L13 `Figure` (1) | 5135 | −22 |
| P329 L14 `Labelled`+`Boxed` (2) | 5072 | −63 |
| **P330 L15 `Block` (1)** | **4960** | **−112** |

**Hub: 5782 → 4960 = −822 acumulado** (−14.2%). A parte atómica vive em **65
módulos** `entities/elements/*.rs` (testáveis isoladamente). Suíte typst-core:
**2521 (P313) → 2697 (P330)** = **+176 testes** unitários; 0 asserções existentes
alteradas.

### Estado declarado do que resta nos 6 matches (zero dívida não classificada)

Os 6 matches despacham as 65 migradas em 1 linha cada. O que **resta** são **12
arms por desenho/escopo-F**:

- **7 primitivos** (triagem P329, desenho declarado): definitivos `Sequence`/
  `MathSequence`/`Empty`/`Space`; provisórios `Text`/`MathText`/`MathIdent`
  (revisita no F).
- **`Styled`** → escopo do F (carrega `Styles`).
- **4 `Set*`** → escopo do F / DEBT 99.E.

Conta: 65 + 7 + 1 + 4 = **77** ✓. **Roteiro de lotes ENCERRADO** — nenhuma
variante "deferida sem dono".

---

## Parte 2 — Baseline de performance pré-F

**Zero decisão, zero otimização.** Mora em
`00_nucleo/diagnosticos/medicao-pre-f-passo-330.md` (cross-ref adicionado no
M3/P318, que fica **superseded** como baseline de performance).

- **Ferramenta**: `hyperfine` **indisponível** → fallback `/usr/bin/time`, 12
  runs (1ª descartada), média ± σ. rustc 1.92.0, release.
- **Corpus**: o **mesmo** do M3/P318 (7003 linhas) + **10×** (70030 linhas).
- **Números**:
  - **1×**: 0.0600 s ± 0.0000 — **quantizado** (granularidade 0.01 s); dentro do
    ruído do M3 (0.07 s).
  - **10×**: **0.6518 s ± 0.0057 s** (n=11; σ ≈ 0.9%) — **o baseline real**,
    longe da granularidade. **É este que o F usa como "antes".**
  - Extras: `cargo build --release` incremental **7.06 s**; suíte typst-core
    **0.37 s**.
- **Comparação**: M3 0.07 s (1×, quantizado ~15%) → P330 0.06 s (1×) é Δ de 1
  tick, **não conclusivo**; o 10× é o primeiro baseline com σ útil do roteiro.
  **Nada foi otimizado** (a migração D não tinha performance como objetivo).
- **Regra**: o "depois" do F refaz antes+depois **no par de commits**, mesma
  ferramenta/corpus, comparando **10×** (1× é inválido por quantização).

---

## Próximo passo do roteiro

O **diagnóstico do F** (conversa de desenho — StyleChain real / PropMap, DEBT
99.E). Consome **este baseline** (P330) e absorve `Set*` + `Styled` + revisita
`Text`/`MathText`/`MathIdent`. **Este passo não o inicia** — só deixa registrado
que tudo que o F precisa está pronto: migração D encerrada, primitivos
declarados, baseline com σ útil tirado.

## Fora de escopo (confirmado)

O diagnóstico do F em si; qualquer mudança em `Styled`/`Set*`/`Text`/`MathText`/
`MathIdent` (escopo do F); qualquer migração além de `Block` (não há — o roteiro
encerra); otimizações sugeridas pela medição (medir ≠ mexer). Caveat: stack
default em `recursao_infinita_*` — não é regressão (`RUST_MIN_STACK=33554432`).
