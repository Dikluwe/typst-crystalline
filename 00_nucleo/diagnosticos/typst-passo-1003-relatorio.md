# Passo 1003 — Auditoria macro: candidatos de corte no `engine/` com DSM Tekt

**Tipo**: Auditoria mecânica (lente DSM `tekt-cargo-dsm`), read-only — catalogar candidatos, não decidir nem cortar.  
**Ferramenta**: `lente` do repo `/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm`, binário `target/release/lente`.  
**Comandos usados**:

```bash
# Cristalino (cwd: /repos/Antigravity/typst-crystalline)
/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm/target/release/lente \
  --pacote typst-core --estrutura > /tmp/lente_cristalino.json

# Vanilla (cwd: lab/typst-original)
/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm/target/release/lente \
  --pacote typst-library --estrutura > /tmp/lente_vanilla_library.json
/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm/target/release/lente \
  --pacote typst-eval --estrutura > /tmp/lente_vanilla_eval.json
```

**Estado da árvore**: commit `00dc949665317dedabc5ec01f0af5f4f4dc6cc80`, sem modificações rastreadas.  
**Aviso de âmbito**: a lente mede arestas ao nível de módulo. Conforme `ADR-0109`, **esta métrica não é gate de correção de um corte específico** — só serve para triagem macro de candidatos. A decisão fina exige o método do Passo 1002, aplicado hub a hub.

---

## Fase A — Top hubs no cristalino (`01_core/src/engine/`)

Métrica: **fan-in único** = número de módulos distintos de `engine/` que dependem do módulo; **fan-out único** = número de módulos distintos de `engine/` dos quais o módulo depende. Pesos = soma das arestas `Uses` (todas, não só referência).

### Top 15 por fan-in (módulos mais dependidos dentro de `engine/`)

| Módulo cristalino | fan-in único | fan-in peso | fan-out único | fan-out peso | Nota |
|---|---:|---:|---:|---:|---|
| `engine::layout::metrics` | 68 | 73 | 3 | 16 | Hub de tipos de medida — quase todo `engine/` o toca. |
| `engine::layout` | 48 | 110 | 23 | 106 | Dispatcher/layout central. |
| `engine::eval` | 36 | 352 | 31 | 88 | Entry point do eval + dispatcher. |
| `engine::scopes` | 15 | 82 | 3 | 19 | Gestão de scopes. |
| `engine::math::layout` | 12 | 21 | 8 | 93 | Layout de math. |
| `engine::stdlib` | 11 | 15 | 32 | 188 | Facade da stdlib. |
| `engine::layout::helpers` | 8 | 14 | 5 | 17 | Helpers de layout. |
| `engine::eval::closures` | 7 | 7 | 21 | 88 | Chamadas/closure application. |
| `engine::lexer` | 7 | 22 | 5 | 48 | Lexer. |
| `engine::parse::parser` | 6 | 64 | 6 | 37 | Parser central. |
| `engine::eval::bindings` | 5 | 5 | 25 | 232 | Field access, method calls, destructuring. |
| `engine::parse::code` | 5 | 10 | 9 | 38 | Parse de code. |
| `engine::stdlib::counter` | 4 | 15 | 19 | 58 | Contadores. |
| `engine::stdlib::state` | 4 | 12 | 16 | 48 | Estado. |
| `engine::stdlib::foundations` | 4 | 49 | 13 | 230 | Fundações da stdlib. |

### Top 15 por fan-out (módulos que mais tocam outros dentro de `engine/`)

| Módulo cristalino | fan-out único | fan-out peso | fan-in único | fan-in peso |
|---|---:|---:|---:|---:|
| `engine::stdlib` | 32 | 188 | 11 | 15 |
| `engine::eval` | 31 | 88 | 36 | 352 |
| `engine::eval::bindings` | 25 | 232 | 5 | 5 |
| `engine::layout` | 23 | 106 | 48 | 110 |
| `engine::eval::closures` | 21 | 88 | 7 | 7 |
| `engine::eval::rules` | 20 | 106 | 2 | 4 |
| `engine::stdlib::counter` | 19 | 58 | 4 | 15 |
| `engine::stdlib::structural` | 18 | 262 | 1 | 36 |
| `engine::stdlib::text` | 18 | 133 | 1 | 14 |
| `engine::eval::modules` | 17 | 41 | 0 | 0 |
| `engine::introspect` | 17 | 41 | 0 | 0 |
| `engine::stdlib::state` | 16 | 48 | 4 | 12 |
| `engine::stdlib::layout` | 16 | 157 | 2 | 21 |
| `engine::stdlib::eval` | 15 | 32 | 1 | 1 |
| `engine::layout::grid` | 15 | 37 | 0 | 0 |

### Observações da fase A

- Os maiores hubs de conectividade total (`fan-in + fan-out`) são:
  1. `engine::layout::metrics` (71)
  2. `engine::layout` (71)
  3. `engine::eval` (67)
  4. `engine::stdlib` (43)
  5. `engine::eval::bindings` (30)
  6. `engine::eval::closures` (28)
- `engine::eval::rules` tem pouco fan-in (2) mas fan-out alto (20): é um módulo que *usa* muito mas é pouco *usado* diretamente, o que sugere que a lógica dele pode estar acoplada a demasiados domínios.
- `engine::stdlib::structural` e `engine::stdlib::text` têm fan-in quase nulo (1) e fan-out alto: são módulos monolíticos que agregam muita responsabilidade sem serem reutilizados internamente.

---

## Fase B — Vanilla, para comparação

### `typst-library` (equivalente funcional a grande parte do `engine/` + `entities/` cristalino)

| Módulo vanilla | fan-in único | fan-in peso | fan-out único | fan-out peso | Nota |
|---|---:|---:|---:|---:|---|
| `typst_library::diag` | 148 | 1127 | 5 | 9 | Diagnósticos — hub universal (esperado). |
| `typst_library::foundations::value` | 112 | 819 | 39 | 84 | Tipo `Value`. |
| `typst_library::foundations::cast` | 103 | 580 | 9 | 26 | Casting. |
| `typst_library::engine` | 103 | 517 | 7 | 40 | Engine/VM context. |
| `typst_library::foundations::styles` | 100 | 594 | 20 | 90 | Cadeia de estilos. |
| `typst_library::foundations::content` | 89 | 703 | 35 | 142 | Tipo `Content`. |
| `typst_library::foundations::args` | 78 | 325 | 14 | 69 | Args. |
| `typst_library::foundations::func` | 75 | 447 | 17 | 80 | Func/closure. |
| `typst_library::foundations::scope` | 73 | 157 | 9 | 56 | Scopes. |
| `typst_library::foundations::repr` | 58 | 81 | 3 | 6 | Repr. |
| `typst_library::foundations::content::element` | 55 | 134 | 16 | 50 | Element trait. |
| `typst_library::foundations::content::field` | 55 | 145 | 8 | 36 | Field access. |
| `typst_library::layout::length` | 47 | 346 | 12 | 39 | Tipos de comprimento. |
| `typst_library::foundations::content::packed` | 42 | 149 | 48 | 277 | Packed content. |
| `typst_library::layout::abs` | 40 | 185 | 4 | 6 | Abs. |
| `typst_library::text` | 33 | 67 | 50 | 258 | Texto + fontes. |
| `typst_library::layout::rel` | 37 | 196 | 11 | 34 | Rel. |
| `typst_library::layout::em` | 35 | 59 | 7 | 17 | Em. |
| `typst_library::layout::axes` | 27 | 118 | 13 | 56 | Axes. |
| `typst_library::layout::ratio` | 24 | 82 | 7 | 11 | Ratio. |
| `typst_library::introspection::location` | 20 | 117 | 16 | 73 | Location. |

### `typst-eval` (avaliador vanilla)

| Módulo vanilla | fan-in único | fan-in peso | fan-out único | fan-out peso | Nota |
|---|---:|---:|---:|---:|
| `typst_eval::vm` | 11 | 32 | 2 | 3 | VM central. |
| `typst_eval::flow` | 4 | 6 | 5 | 5 | Controlo de fluxo. |
| `typst_eval::access` | 4 | 6 | 4 | 6 | Field access. |
| `typst_eval::methods` | 3 | 10 | 0 | 0 | Method resolution. |
| `typst_eval::call` | 2 | 3 | 6 | 14 | Chamadas/closure. |
| `typst_eval::binding` | 2 | 2 | 4 | 8 | Destructuring. |
| `typst_eval::import` | 1 | 1 | 3 | 4 | Importações. |
| `typst_eval::code` | 0 | 0 | 6 | 9 | Avaliação de code nodes. |
| `typst_eval::ops` | 0 | 0 | 5 | 9 | Operadores. |
| `typst_eval::markup` | 0 | 0 | 3 | 4 | Avaliação de markup nodes. |
| `typst_eval::math` | 0 | 0 | 3 | 3 | Avaliação de math nodes. |
| `typst_eval::rules` | 0 | 0 | 3 | 6 | Set/show rules. |

### Observações da fase B

- O `typst-eval` do vanilla é **muito mais fragmentado** do que o `engine::eval` cristalino: cada domínio (code, markup, math, ops, call, binding, access, methods, rules, flow, import) vive no seu próprio ficheiro pequeno, com fan-out de 3–6.
- O `typst-library::engine` (103 fan-in) concentra o contexto de execução, mas a lógica de eval em si está fora, em `typst-eval`.
- Tipos fundamentais (`value`, `content`, `func`, `scope`, `styles`, `args`, `cast`) são hubs inevitáveis em ambos os projetos.
- A família `typst_library::layout::{length, abs, rel, em, axes, ratio}` corresponde ao monolítico `engine::layout::metrics` do cristalino, mas está fragmentada em múltiplos ficheiros no vanilla.

---

## Fase C — Cruzamento cristalino vs vanilla

| Hub cristalino | fan-in/out cristalino | Equivalente(s) vanilla | fan-in/out vanilla | Nota |
|---|---|---|---|---|
| `engine::eval` | in 36 / out 31 | `typst_eval` (todo o crate fragmentado: vm, code, markup, math, ops, call, binding, access, methods, rules, flow, import) + partes em `typst_library::engine`, `foundations::value/func/scope/content` | vm in 11 / out 2; resto ≤ 6 | Hub central cristalino vs avaliador fragmentado no vanilla. |
| `engine::eval::bindings` | in 5 / out 25 | `typst_eval::access`, `typst_eval::methods`, `typst_eval::call`, `typst_eval::binding` | access 4/4; methods 3/0; call 2/6; binding 2/4 | Responsabilidades de acesso/chamada/destructuring estão separadas no vanilla. |
| `engine::eval::rules` | in 2 / out 20 | `typst_eval::rules` (3/6) + `typst_realize` (show rules) | rules 0/3 | Regras de show no vanilla são realizadas fora do eval (`typst-realize`). |
| `engine::eval::closures` | in 7 / out 21 | `typst_eval::call` (2/6) + `typst_library::foundations::func` (75/17) | call 2/6; func 75/17 | Aplicação de closures está dividida entre eval e foundations no vanilla. |
| `engine::stdlib` | in 11 / out 32 | Facade dispersa por `typst_library::{text, layout, visualize, math, model, introspection, loading, ...}` | text 33/50; layout varia | A stdlib cristalina usa uma facade central; no vanilla cada módulo de library exporta as suas funções nativas. |
| `engine::layout::metrics` | in 68 / out 3 | `typst_library::layout::{length, abs, rel, em, axes, ratio}` | length 47/12; abs 40/4; rel 37/11; em 35/7; axes 27/13; ratio 24/7 | Monolito cristalino vs família de tipos no vanilla. |
| `engine::layout` | in 48 / out 23 | `typst_library::layout` + `typst-layout` crate | — | Dispatcher/layout central; no vanilla layout é um crate separado. |
| `engine::stdlib::structural` | in 1 / out 18 | `typst_library::layout::grid`, `typst_library::model::table`, `typst-library/src/layout/grid/resolve.rs` | grid in 0/out 15; table in alta | Grid/table e linhas aglomerados num só ficheiro cristalino. |
| `engine::stdlib::text` | in 1 / out 18 | `typst_library::text` | in 33 / out 50 | Texto é um hub ainda maior no vanilla. |
| `engine::stdlib::counter` | in 4 / out 19 | `typst_library::introspection::{counter, state, location}` | location 20/16 | Contadores no vanilla estão no módulo de introspection. |
| `engine::scopes` | in 15 / out 3 | `typst_library::foundations::scope` | in 73 / out 9 | Tipo scope é hub universal no vanilla. |

---

## Fase D — Candidatos ordenados a próximo protótipo (após `operators.rs`)

Critério de ordenação: **fan-in alto** (muitos módulos dependem do hub) + **divergência grande face ao vanilla** (vanilla fragmentou o equivalente em vários ficheiros pequenos, cristalino mantém um só monolito). A ordem reflete *potencial de ganho*, não urgência de negócio.

| # | Candidato | Por que aparece no topo | Equivalente vanilla | Risco / nota |
|---|---|---|---|---|
| 1 | `engine::eval` | Hub central (in 36 / out 31); praticamente todo o eval vive aqui e é importado por quase todos os módulos de engine. | `typst_eval` fragmentado em ~12 ficheiros + partes em `typst_library`. | Corte complexo — envolve dispatcher, entry points e submódulos já existentes. Requer Passo 1002 dedicado. |
| 2 | `engine::eval::bindings` | Fan-out 25 (tocam 25 módulos de engine) com fan-in apenas 5; aglomera acesso, method calls, destructuring e counter/state/query. | `typst_eval::access`, `methods`, `call`, `binding`. | Corte mais limpo — divisão por responsabilidade já existe no vanilla. |
| 3 | `engine::eval::rules` | Fan-out 20, fan-in 2; contém set/show rules, font-dict, realização de flow/node. | `typst_eval::rules` (pequeno) + `typst_realize` (show rules). | Atenção: show rules no vanilla são realizadas em `typst-realize`, não em eval. Corte pode exigir mudar fase de pipeline. |
| 4 | `engine::eval::closures` | Fan-out 21; aplicação de funções, merge de args, plugins, location methods. | `typst_eval::call` + `typst_library::foundations::func`. | Parte da lógica pertence naturalmente a `func` (foundations), não a eval. |
| 5 | `engine::layout::metrics` | Fan-in 68 — o módulo mais dependido de todo `engine/`. | `typst_library::layout::{length, abs, rel, em, axes, ratio}`. | Corte relativamente mecânico: separar tipos de medida. Não é "eval", mas é o hub estruturalmente mais carregado. |
| 6 | `engine::stdlib::structural` | Fan-out 18, aglomera `grid`, `table`, header/footer/cell/hline/vline. | `typst_library::layout::grid` + `typst_library::model::table` + `typst-layout/src/grid/lines.rs`. | Corte já parcialmente antecipado pelos prompts órfãos `grid_hline.md`, `table_hline.md`, etc. |
| 7 | `engine::stdlib::text` | Fan-out 18; aglomera texto, fontes, raw, regex, lorem. | `typst_library::text` (hub ainda maior no vanilla: out 50). | Vanilla não fragmentou texto — é um hub grande. O ganho de divisão pode ser menor, a menos que se alinhe com `ADR-0109` para layout de texto. |
| 8 | `engine::stdlib` (facade) | Fan-out 32 — facade que reexporta tudo. | Não há facade equivalente; cada módulo de `typst_library` exporta nativo diretamente. | "Corte" aqui pode ser desmantelar a facade em reexports por domínio, não mover lógica. |

---

## O que este passo NÃO faz

- Não decide onde cortar dentro de nenhum ficheiro.
- Não usa a métrica da lente como critério de correção de um corte específico.
- Não corrige nenhum código.

## Resultado entregue

Lista ordenada de candidatos a próximo protótipo de atomização/refatoração, com números reais de fan-in/out do DSM e mapeamento para a estrutura vanilla correspondente. Próximo passo recomendado: aplicar o método do Passo 1002 ao candidato #1 (`engine::eval`) ou, se o risco for elevado demais, ao candidato #2 (`engine::eval::bindings`) para validar o método num escopo menor.
