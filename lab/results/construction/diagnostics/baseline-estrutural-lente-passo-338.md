# Baseline Estrutural — Lente `tekt-cargo-dsm` (Passo 338, re-medição)

> Medição estrutural por computação (DSM / grafo de dependências resolvido),
> não por narrativa. Re-corrida da lente sobre o produto após o ciclo F-4 do
> Passo 338 (colapso da dualidade `Styles`/`StyleDelta`, backing único).
> Sucessor de `baseline-estrutural-lente-passo-333.md` (medido em P332/HEAD
> `f83f78a13`); aqui o produto está em P338/HEAD `6139e3517`.
>
> **Natureza desta parte:** medição de leitura. Zero código de produto alterado;
> zero fix à lente. Apenas leitura do produto + escrita deste diagnóstico.

---

## 1. A lente — versão, build, comandos exatos

### Identidade da lente (registrada)

| Item | Valor |
|------|-------|
| Repo da lente | `/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm` |
| Commit | `98d8f9e` — `feat: --comparar a nível de item — trilha 0075→0078` |
| Branch | `main` |
| Working tree | 1 alteração não relacionada (não afeta o binário medido) |
| Binário | `target/release/lente` (crate `lente_app`) — já compilado |
| Fonte do grafo | fork `cargo-modules` `0.27.0`, commit `ddcd3ca`, em `~/.cargo/bin/cargo-modules` |

### Produto medido

| Item | Valor |
|------|-------|
| Repo | `/home/dikluwe/Documentos/Antigravity/typst-crystalline` |
| Commit HEAD | `6139e3517` — `Passo 338 — F-4 S4: registro e fecho` |
| Crate analisado | `typst-core` (L1, `01_core/`) |

### Comandos de medição (executados de dentro de `01_core/`)

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core
LENTE=/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm/target/release/lente

# (A) Estrutura — acoplamento de tipo genuíno (so-referencia), só o nosso código
RUST_MIN_STACK=33554432 $LENTE --pacote typst-core --estrutura --so-referencia --filtrar-stdlib

# (B) Estrutura — TODAS as uses (inclui imports de nível de módulo)
RUST_MIN_STACK=33554432 $LENTE --pacote typst-core --estrutura --filtrar-stdlib

# (C) Ranking de impacto
RUST_MIN_STACK=33554432 $LENTE --pacote typst-core --ranking --top 15 --filtrar-stdlib --text

# (D) DSM em HTML autocontido
RUST_MIN_STACK=33554432 $LENTE --pacote typst-core --estrutura --so-referencia --filtrar-stdlib --html --saida /tmp/typst-dsm.html

# (E) Modo --diff (mapeia diff git → nós tocados)
RUST_MIN_STACK=33554432 $LENTE --diff --repo /home/dikluwe/Documentos/Antigravity/typst-crystalline --vista resumo --text
```

Observações de execução:
- Todos os 4 modos (A/B + C, D, E) terminaram com **EXIT 0**. A lente buildou,
  rodou e produziu números coerentes — sem regressão face a P333.
- (E) `--diff` reportou **0 tocados** (a árvore só tem `.md` de materialização
  não-fonte e `target/`/`__pycache__` untracked; nenhum `.rs` em diff).
- DSM HTML gerado em `/tmp/typst-dsm.html` (≈210 KB, fora do produto, descartável).

---

## 2. Os números estruturais

### 2.1 Métricas globais do crate `typst-core`

| Métrica | so-referencia (tipo real) | todas-as-uses (inflado) |
|---------|---------------------------|--------------------------|
| Módulos | **219** | 219 |
| Arestas módulo→módulo | **675** | 986 |
| Ciclos (SCC não-triviais) | **2** | 4 |
| Tamanhos dos ciclos | **[90, 4]** | [93, 7, 5, 4] |

### 2.2 Hub `entities/content.rs` (o enum central, ~4990 linhas)

| Métrica do hub | Valor (so-referencia) |
|----------------|------------------------|
| Fan-in (módulos que dependem DE content) | **87** |
| Fan-out (módulos de que content depende) | **83** |

Decomposição do fan-out (83):
- **66 → módulos de elemento** (`content` importa cada `*Elem` concreto)
- 17 → não-elementos: `bib_entry, citation_form, color, counter_update, dir,
  func, geometry, label, layout_types, math_style, parity, ptr_eq_arc, sides,
  source_result, state_update, style, value`

Decomposição do fan-in (87):
- **65 ← módulos de elemento** (cada elemento importa `Content`)
- 22 ← não-elementos: `content_hash, element_payload, element_registry, func,
  introspector, module, page_store, value`, e submódulos de `rules::{eval,
  introspect, layout, math, stdlib}`.

**Leitura:** `content` permanece o maior emissor e maior receptor do crate. O
acoplamento bidirecional content↔elemento mantém-se ~total (66 e 65) — a
assinatura estrutural do **modelo de enum fechado (D)** intacta.

### 2.3 Independência dos módulos de elemento (`entities/elements/*`)

| Métrica | Valor |
|---------|-------|
| Módulos de elemento com aresta | **66** |
| Arestas **elemento → elemento** | **0** |
| Ciclos *entre* elementos | **0** |
| Fan-out médio de um elemento | **3.29** módulos distintos |
| Maior fan-out de elemento | `block`, `boxed` = 7 deps |

**Atomização horizontal: confirmada por cálculo, inalterada.** Zero arestas
mútuas entre os elementos, zero ciclos entre eles. Cada elemento continua folha
lateral apontando só para o núcleo e tipos compartilhados.

### 2.4 Ciclos — o achado central

A lente reporta **2 ciclos (SCCs)** em modo so-referencia. Tamanhos: 90, 4.

| Ciclo | Tamanho | content dentro? | nº de elementos dentro |
|-------|---------|-----------------|------------------------|
| 0 | **90** | **sim** | **65** (todos) |
| 1 | 4 | não | 0 — é `ast::{code, expr, markup, math}` |

Os 25 não-elementos do ciclo 0:
`args, content, element_payload, element_registry, engine, func, geometry,
gradient, introspector, layout_types, metadata_store, module, page_store, paint,
position, scope, sealed_positions, show, state_registry, state_update, style,
style_chain, value, rules::eval, rules::scopes`.

**O ciclo de 8 não desapareceu — foi absorvido.** Em P332 havia 3 ciclos
`[81, 4, 8]`; o ciclo 2 (8 módulos: `geometry, gradient, layout_types, paint,
position, sealed_positions, style, style_chain`) agora aparece **inteiro dentro
do ciclo 0**, junto de `element_registry` (novo). É por isso que a contagem caiu
3→2 e o megaciclo cresceu 81→90 (+9 = os 8 da geometria/estilo + `element_registry`).
A unificação `Styles`→`StyleDelta` do F-4 ligou o anel de estilo ao anel de
`content` em vez de o dissolver: os módulos de estilo passaram a ser
alcançáveis a partir de content e vice-versa, fundindo os dois SCCs num só.
O ciclo 1 (`ast`, 4) permanece ortogonal e intacto.

**Consequência para o F:** o ciclo de 90 continua causado pela **direção
content→elemento**, não pela atomização (perfeita). Se o F remove os 66 imports
`use elements::*Elem` de `content.rs`, o fan-out de content cai de 83 → 17, os
elementos deixam de ser alcançáveis de volta, e o ciclo colapsa para o núcleo
estilo+sintaxe residual. A fusão estilo↔content do F-4 não altera esse veredito —
apenas engrossou o anel que o F-1 vai cortar.

### 2.5 Ranking de impacto (top 15 — `--ranking`)

Os nós de maior alcance transitivo continuam a ser os tipos sintáticos de base:

| # | Impacto | Classe | Path |
|---|---------|--------|------|
| 1 | 2307 | Base | `entities::span::Span` |
| 2 | 2252 | Base | `entities::syntax_text::SyntaxText` |
| 3 | 2233 | Base | `entities::syntax_kind::SyntaxKind` |
| 4–9 | ~2178–2228 | Intermediário | nós de `syntax_node::*` (SyntaxError, ErrorNode, LeafNode, InnerNode, NodeKind, SyntaxNode) |
| 10 | 1397 | Base | `entities::color::Color` |
| 11 | 1387 | Base | `layout_types::Pt` |
| 12 | 1362 | Base | `rustc_hash::FxBuildHasher` |
| 13–14 | ~1355–1358 | Base | `layout_types::{Ratio, Abs}` |
| 15 | 1351 | Base | `entities::color::ColorSpace` |

(`content` não entra no top-15 por item — o peso está distribuído pelos `*Elem`.)

### 2.6 Artefato visual

DSM HTML autocontido gerado em `/tmp/typst-dsm.html` (fora do produto, descartável).

---

## 3. Deriva face ao baseline de P333 (P332 → P338)

| Métrica | Baseline (P332) | Agora (P338) | Δ |
|---------|-----------------|--------------|---|
| Módulos | 217 | **219** | +2 |
| Arestas (so-referência) | 667 | **675** | +8 |
| Arestas (todas-as-uses) | 970 | **986** | +16 |
| Ciclos (SCC) | 3 `[81,4,8]` | **2 `[90,4]`** | −1 (fusão) |
| `content` fan-out | 82 | **83** | +1 |
| `content → elements::*` | 65 | **66** | +1 |
| `content` fan-in | 85 | **87** | +2 |
| `elements → content` | 65 | **65** | = |
| arestas elem→elem | 0 | **0** | = |
| fan-out médio de elemento | 3.29 | **3.29** | = |

**Leitura da deriva:**
- **Atomização preservada** ao longo de 6 passos de migração: 0 arestas
  elemento→elemento, fan-out médio idêntico. A propriedade que importa não
  regrediu. ✅
- **O ciclo de 8 (geometria/estilo) foi absorvido** pelo megaciclo de content
  (não eliminado) — efeito colateral estrutural do colapso `Styles`→`StyleDelta`
  do F-4. Engrossa o anel que o F-1 cortará; não o invalida.
- **A fronteira F continua não executada:** `content → elements::* == 66`
  (era 65; +1 por um elemento novo). O alvo do F é **0**. Esperado — o F-1
  (remoção dos `use elements::*Elem` do núcleo) ainda não correu.

---

## 4. A lente no contrato de verificação do F (inalterado)

A métrica-gate do F permanece computável lote a lote:
`(arestas content → entities::elements::*) == 0` após cada lote F migrado, via
`--estrutura` (campo `dependencias`, filtrando `de==content & para∈elements`).
Hoje = **66**; alvo = **0**; o `--comparar --antes/--depois` dará o delta direto
quando o primeiro lote F existir.

Limites declarados (R1–R5 do baseline de P333) continuam válidos e não foram
exercitados/corrigidos aqui:
- **R1** — sem vista workspace que cruze L1↔L2↔L3↔L4 (mede um crate por vez).
- **R2** — sem métrica de instabilidade de Martin nem flag de aresta-que-viola-camada.
- **R3** — granularidade module-level (content.rs = 1 nó; **298** sítios
  `match self/=>`, `ElementKind` = **12** variantes não são descidos).
- **R4** — sem distinção import-de-trait vs import-de-struct-concreta na aresta.
- **R5** — `--comparar` ainda não validado num par antes/depois real de lote F.

---

## Apêndice — proveniência dos números

| Número | Origem (comando / artefato) |
|--------|------------------------------|
| 219 módulos, 675 arestas, 2 ciclos [90,4] | comando (A), JSON `modulos`/`dependencias`/`ciclos` |
| 986 arestas / 4 ciclos (todas-uses) | comando (B) |
| content fan-in 87 / fan-out 83 (66/65 elementos) | comando (A), filtro sobre `dependencias` |
| 66 elementos, 0 arestas elem→elem, fan-out médio 3.29 | comando (A), filtro `de,para ∈ elements` |
| 65 elementos todos no ciclo 0 (90); 25 não-elementos | comando (A), interseção SCC × elements |
| ranking top 15 (Span 2307, …) | comando (C) |
| 298 sítios `match/=>` em content.rs (4990 linhas); ElementKind=12 | leitura direta da fonte (sem alteração) |
| DSM HTML | comando (D), `/tmp/typst-dsm.html` |
| `--diff` 0 tocados | comando (E) |

Lente: `tekt-cargo-dsm@98d8f9e` (main) · fork `cargo-modules@ddcd3ca` (v0.27.0).
Produto: `typst-crystalline@6139e3517`. Nenhum ficheiro de produto tocado.
