# P447 — DSM Audit (alternativa `cargo modules`)

> **Data:** 2026-06-24  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Ferramenta:** `cargo modules` (subcomandos `dependencies` e `export-json`)  
> **Nota:** a ferramenta própria `tekt dsm` não se encontra disponível no ambiente de execução; o DSM foi gerado com a alternativa standard `cargo modules`, que produz grafos de dependências internas por crate em formato DOT e JSON.

---

## 1. Ficheiros gerados

| Crate | DOT | JSON |
|-------|-----|------|
| `typst-core` | `00_nucleo/dsm/dsm-p447-typst-core.dot` | `00_nucleo/dsm/dsm-p447-typst-core.json` |
| `typst-shell` | `00_nucleo/dsm/dsm-p447-typst-shell.dot` | `00_nucleo/dsm/dsm-p447-typst-shell.json` |
| `typst-infra` | `00_nucleo/dsm/dsm-p447-typst-infra.dot` | `00_nucleo/dsm/dsm-p447-typst-infra.json` |
| `typst-wiring` | `00_nucleo/dsm/dsm-p447-typst-wiring.dot` | `00_nucleo/dsm/dsm-p447-typst-wiring.json` |

---

## 2. Métricas de instabilidade (módulos alterados em P445-P446)

Métrica agregada a partir do grafo item-level do `cargo modules` (arestas `uses`/`owns` entre itens de ficheiros diferentes dentro de `01_core/src/`).

| Módulo | Fan-out | Fan-in | Instabilidade `I` | Observação |
|--------|---------|--------|-------------------|------------|
| `entities::content` | 84 | 127 | 0.40 | Hub estável do sistema. |
| `entities::show` | 5 | 4 | 0.56 | Adição de `NodeKind::Smallcaps` não altera significativamente o perfil. |
| `rules::eval::rules` | 20 | 2 | **0.91** | Já era um módulo de alto fan-out (orquestra selectors/show rules). P444/P445 adicionaram mais mapeamentos de function-pointer, mas nenhuma dependência cíclica nova. |
| `rules::lang::quotes` | 1 | 1 | 0.50 | Adição de `localize_single_quotes` (P445) mantém o módulo como folha estável. |
| `rules::layout::cursor` | 5 | 1 | 0.83 | P446 adicionou `layout_chunk`; o módulo continua a ser consumido essencialmente só por `layout::mod` (fan-in=1). |
| `rules::layout::mod` | 67 | 48 | 0.58 | Hub de layout; adição do flag `smallcaps` e do arm `SmallCaps` não aumentou o acoplamento de forma crítica. |
| `rules::layout::text` | 5 | 1 | 0.83 | P446 introduz a chamada a `layout_chunk` (de `cursor`) e a lógica de smallcaps; continua dependente exclusivamente de `layout::mod`. |
| `rules::stdlib::text` | 13 | 1 | **0.93** | Módulo de registo de funções nativas de texto; naturalmente alto fan-out porque importa muitas entidades. |

**Conclusão de thresholds:**
- `rules::eval::rules` (`I = 0.91`) e `rules::stdlib::text` (`I = 0.93`) excedem o threshold `I > 0.9` (vermelho). Este perfil **pré-existia** às mudanças de P445/P446; os passos apenas acrescentaram entradas em mapeamentos já existentes. Não emergiram dependências cíclicas nem acoplamentos transversais novos entre domínios distintos.
- `rules::layout::text` e `rules::layout::cursor` (`I ≈ 0.83`) estão no amarelo. São módulos helper do layout; o acoplamento é unidirecional `mod → text → cursor`, sem ciclo.

---

## 3. Drift de dependências observado

As mudanças de P445/P446 introduziram as seguintes utilizações (não novos módulos, mas novos call-sites dentro de dependências já existentes):

1. `rules::layout::mod` → `rules::layout::text`  
   - P446: o arm `Content::SmallCaps` delega a renderização fragmentada ao layout de `Text` com o flag `smallcaps`.
2. `rules::layout::text` → `rules::layout::cursor`  
   - P446: novo método `layout_chunk` chamado para emitir runs de tamanhos diferentes dentro da mesma palavra.
3. `rules::eval::rules` → `entities::show`  
   - P444/P446: novos `NodeKind::{Underline,Strike,Overline,Smallcaps}`; `eval::rules` já dependia de `entities::show`.
4. `rules::eval::rules` → `rules::lang::quotes`  
   - P445: `localize_single_quotes` é usado no `eval_markup`; `eval::rules` não usa directamente, mas o pipeline de eval mantém a dependência via `lang/quotes.rs`.

**Nenhuma dependência circular foi detectada** entre os módulos alterados. O grafo continua acíclico na direcção eval → entities/layout → infra.

---

## 4. Recomendações (não bloqueantes)

- Manter `rules::eval::rules` e `rules::stdlib::text` sob observação; se o fan-out continuar a crescer, considerar subdividir `eval::rules` em sub-módulos por tipo de selector (`show`, `regex`, `where`, etc.).
- `rules::layout::cursor` e `rules::layout::text` podem ser fundidos num único módulo de “text shaping” se o fan-in não aumentar, reduzindo a profundidade da cadeia `mod → text → cursor`.

---

## 5. Scope-out deste audit

- Não foi gerado output visual (PNG/SVG); os ficheiros DOT podem ser renderizados com `dot -Tsvg dsm-p447-typst-core.dot` se necessário.
- Não foram calculadas métricas de complexidade ciclomática nem de churn; o DSM é puramente estrutural.
