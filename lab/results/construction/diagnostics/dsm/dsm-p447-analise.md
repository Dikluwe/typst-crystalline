# P447 — DSM Audit (ferramenta nativa `lente`)

> **Data:** 2026-06-24  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Ferramenta:** `lente --estrutura` (a lente própria do projeto; `tekt-cargo-dsm` é o nome da lente referenciada em `lab/parity/tools/decompor_so_vanilla.py`)  
> **Nota:** a primeira versão deste audit usou `cargo modules` como fallback porque o executável `lente` não foi localizado de imediato. Após confirmar que a ferramenta nativa é `lente`, o DSM foi regenerado com ela.

---

## 1. Ficheiros gerados

| Crate | Texto | HTML |
|-------|-------|------|
| `typst-core` | `00_nucleo/dsm/lente/typst-core.txt` | `00_nucleo/dsm/lente/typst-core.html` |
| `typst-shell` | `00_nucleo/dsm/lente/typst-shell.txt` | `00_nucleo/dsm/lente/typst-shell.html` |
| `typst-infra` | `00_nucleo/dsm/lente/typst-infra.txt` | `00_nucleo/dsm/lente/typst-infra.html` |
| `typst-wiring` | `00_nucleo/dsm/lente/typst-wiring.txt` | `00_nucleo/dsm/lente/typst-wiring.html` |

Os ficheiros DOT/JSON da primeira tentativa (`cargo modules`) permanecem em `00_nucleo/dsm/` para proveniência.

---

## 2. Sumário estrutural por crate

| Crate | Módulos | Ciclos | Observação |
|-------|---------|--------|------------|
| `typst-core` | 277 | 4 | Todos os ciclos pré-existem; nenhum envolve os módulos alterados em P445/P446. |
| `typst-infra` | 22 | 0 | Acoplamento concentrado em `export` e `gradients`. |
| `typst-shell` | 3 | 0 | Superfície CLI sem ciclos. |
| `typst-wiring` | 3 | 0 | Root de composição; depende apenas de `typst_shell::cli`. |

Ciclos detectados em `typst-core`:

1. **Mega-ciclo no domínio `entities`** (~96 módulos: `content`, `elements::*`, `value`, `style_chain`, `eval`, `scopes`, ...).
2. `entities::ast::{code, expr, markup, math}`
3. `rules::parse::{code, markup, math, patterns, rules}`
4. `rules::stdlib::{calc, foundations, layout, math_style, primitives_constructors, structural, text, stdlib}`

---

## 3. Métricas de instabilidade (módulos alterados em P445-P446)

Calculadas a partir do grafo módulo → módulo do `lente` (arestas `uses`).  
`I = fan-out / (fan-out + fan-in)`.

| Módulo | Fan-out | Fan-in | Instabilidade `I` | Observação |
|--------|---------|--------|-------------------|------------|
| `entities::content` | 88 | 127 | 0.41 | Hub estável do sistema; faz parte do mega-ciclo de entidades. |
| `entities::show` | 5 | 3 | 0.62 | Registo de `NodeKind::Smallcaps` mantém o perfil moderado. |
| `rules::eval::rules` | 20 | 1 | 0.95 | Alto fan-out; folha de entrada única (`eval::markup`). |
| `rules::lang::quotes` | 1 | 0 | 1.00 | Folha pura; introduzido em P445. |
| `rules::layout` | 19 | 45 | 0.30 | Hub de layout estável; consome submódulos. |
| `rules::layout::cursor` | 5 | 0 | 1.00 | Folha pura; novo método `layout_chunk` (P446). |
| `rules::layout::text` | 5 | 0 | 1.00 | Folha pura; lógica de smallcaps (P446). |
| `rules::stdlib::text` | 13 | 1 | 0.93 | Registo de funções nativas de texto; perfil já esperado. |

**Conclusão de thresholds:**
- `rules::eval::rules` (`I = 0.95`) e `rules::stdlib::text` (`I = 0.93`) excedem o threshold `I > 0.9` (vermelho). Este perfil **pré-existia** às mudanças de P445/P446; os passos apenas acrescentaram entradas em mapeamentos já existentes.
- `rules::layout::cursor`, `rules::layout::text` e `rules::lang::quotes` têm `I = 1.0` por serem folhas puras (nenhum outro módulo as consome directamente ao nível `uses` do `lente`). Isso é esperado para helpers/registos.
- **Não surgiu nenhum ciclo novo** nem acoplamento transversal novo entre domínios distintos em consequência de P445/P446.

---

## 4. Drift de dependências observado (vista `lente`)

Arestas `uses` envolvendo os módulos modificados:

1. `rules::layout::text` → `rules::layout`  
   - P446: smallcaps delega em `layout_chunk` do `Layouter` (`rules::layout`).
2. `rules::layout::cursor` → `rules::layout`  
   - P446: `layout_chunk` pertence ao cursor e usa helpers/métricas do layout.
3. `rules::eval::markup` → `rules::eval::rules`  
   - P445: smart quotes passam pelo pipeline de eval/rules.
4. `rules::eval::rules` → `entities::show`  
   - P444/P446: novos `NodeKind::{Underline,Strike,Overline,Smallcaps}`.
5. `entities::show` → `entities::content` / `func` / `regex` / `style` / `value`  
   - Registo dos show rules continua sem dependências cíclicas.

**Nenhuma dependência circular foi introduzida** pelos passos P445/P446.

---

## 5. Recomendações (não bloqueantes)

- `rules::eval::rules`: se continuar a crescer, considerar dividir em sub-módulos por tipo de selector (`show`, `regex`, `where`).
- `rules::layout::cursor` e `rules::layout::text`: como são folhas consumidas por `rules::layout`, avaliar se faz sentido fundi-los num único módulo de “text shaping” ou manter a separação cursor/texto.
- Mega-ciclo de `entities`: é arquitetural e conhecido; não deve ser atacado no âmbito destes passos.

---

## 6. Scope-out deste audit

- O `lente` gera também vistas HTML interativas (`lente/*.html`) para navegação visual.
- Não foram calculadas métricas de complexidade ciclomática nem de churn; o DSM é puramente estrutural.
