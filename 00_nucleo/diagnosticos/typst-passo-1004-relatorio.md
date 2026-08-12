# Passo 1004 — Auditoria: `engine::stdlib`, facade deliberada ou acumulação sem decisão?

**Tipo**: Auditoria — entender, catalogar, não corrigir nem propor fatiamento ainda.  
**Estado da árvore**: commit `00dc949665317dedabc5ec01f0af5f4f4dc6cc80`, sem modificações rastreadas.  
**Data**: 2026-08-12.

---

## Fase A — L0 actual, lido literalmente

O `mod.rs` de `engine::stdlib` aponta para:

- **`00_nucleo/prompts/engine/stdlib/_comum.md`** (hash `df6fe846`, L1, atualizado 2026-06-22).

Citações relevantes do L0:

> "Enquanto `eval.rs` é o motor que **caminha pela AST**, este módulo contém as **ferramentas nativas** que Typst expõe no seu escopo global — funções implementadas directamente em Rust e registadas como `Value::Func` durante a inicialização do compilador."

> "**Separação de responsabilidades crítica:**  
> - `eval.rs`: sabe como avaliar `Expr::LetBinding`, loops, condicionais → produz `Value`  
> - `stdlib`: sabe o que `abs(-5)` retorna → implementa as funções que `eval` *chama*"

> "`stdlib` nativa mínima — Passo 17."  
> "Reestruturado por cluster em Passo 96.5 conforme ADR-0037."

> Notas de escopo de `_comum.md`:  
> "Este ficheiro guarda **só** o que é partilhado por várias funções; os prompts finos por função citam-no."  
> "`stdlib/structural.rs` tem agora o seu próprio prompt em `structural.md` (P430)."  
> "`stdlib/layout.rs` tem agora o seu próprio prompt em `layout.md` (P432)."  
> "... Com este fecho, **DEBT-57 está encerrado** — todos os ficheiros stdlib de L1 têm spec L0 dedicada."

**Observação**: não existe um `00_nucleo/prompts/engine/stdlib.md` (índice). O `_comum.md` menciona que `rules/stdlib.md` (agora índice) foi fatiado em P314, mas o índice não está presente no repo. O `mod.rs` foi selado com o prompt comum, não com um prompt que decida a arquitetura da facade.

---

## Fase B — Decisão de origem

### ADRs consultados

Comando usado:

```bash
grep -rln 'stdlib' 00_nucleo/adr/ | xargs grep -l 'facade\|central\|agregador'
```

Resultado: `00_nucleo/adr/README.md`, `ADR-0037`, `ADR-0054`, `ADR-0065`, `ADR-0100`.

**ADR-0037 — Coesão por domínio** (`typst-adr-0037-coesao-por-dominio.md`) é o ADR mais relevante. Diz explicitamente:

> "**`stdlib.rs`** (Passo 96.5) — um ficheiro por módulo da stdlib."

E estabelece a hierarquia de submódulos:

> "O `mod.rs` é o único a ter API pública para o resto do projecto. Submódulos são `pub(super)` ou `pub(crate)` conforme a visibilidade necessária."

**Nenhum ADR discute explicitamente a alternativa "facade central vs exportar por módulo como o vanilla"**. A decisão registada foi:

1. Decompor o monolito `stdlib.rs` em submódulos por domínio (Regra 1, Regra 3).
2. O `mod.rs` é o ponto de entrada público.

A **existência** de `engine::stdlib` como módulo agregador está, portanto, legitimada por ADR-0037. A **forma específica** da facade (um `mod.rs` que reexporta dezenas de nativas para `make_stdlib` em `eval/mod.rs`) não foi objeto de decisão própria — é a aplicação natural da Regra 3.

### Momento de origem

A estrutura nasceu no **Passo 96.5** (decomposição de `stdlib.rs` conforme ADR-0037). Não foi uma decisão isolada sobre facade; foi a aplicação de um padrão geral de decomposição por domínio. O `mod.rs` reexportador é o padrão Rust para expor uma API pública a partir de submódulos privados.

---

## Fase C — Mapeamento dos submódulos por natureza

Critério usado (consistente com o Passo 1002):

- **Imperativo**: toca `EvalContext`/scope/estado de runtime, faz I/O, ou tem efeitos (panic/assert).
- **Declarativo**: `Value → Value` puro, construtores de conteúdo/estilo, tabelas estáticas.

| Submódulo | Linhas | Domínio | Natureza | Correspondência directa no vanilla |
|---|---|---|---|---|
| `assert.rs` | 190 | `assert`, `assert.eq`, `assert.ne` | Imperativo | `typst_library::foundations` (assert/panic) |
| `calc.rs` | 1460 | Matemática escalar (`abs`, `pow`, trig, combinatória) | Declarativo | `typst_library::foundations::calc` |
| `collections.rs` | 2083 | Métodos de instância `array`/`dict`/`str` | Declarativo | `typst_library::foundations::{array, dict, str}` |
| `color.rs` | 661 | Operadores e métodos de `color` | Declarativo | `typst_library::visualize::color` |
| `context.rs` | 35 | `context { expr }` | Imperativo | `typst_library::foundations::context` |
| `counter.rs` | 377 | `counter(...)` e métodos | Imperativo | `typst_library::introspection::counter` |
| `emoji.rs` | 568 | Tabela `emoji.*` | Declarativo | `typst_library::symbols::emoji` |
| `eval.rs` | 250 | `eval(source, mode:, scope:)` | Imperativo | `typst_eval::eval_string` |
| `figure_image.rs` | 190 | `figure`, `image` | Declarativo | `typst_library::model::{figure, image}` |
| `foundations.rs` | 2406 | `type`, `len`, `repr`, constructors de cores, state/counter/query/here/locate | **Misto/Imperativo** (contém state/counter/query) | `typst_library::foundations` + `typst_library::introspection` |
| `gradients.rs` | 474 | `gradient.*` | Declarativo | `typst_library::visualize::gradient` |
| `label.rs` | 55 | `label(...)` | Declarativo | `typst_library::model::label` |
| `layout.rs` | 1943 | `align`, `block`, `box`, `grid`, `pad`, `pagebreak`, `place`, `stack` | Declarativo | `typst_library::layout::*` |
| `loading.rs` | 1274 | `read`, `csv`, `json`, `yaml`, `toml`, `cbor`, `xml` | Imperativo (I/O via `World::read_bytes`) | `typst_library::loading::*` |
| `math_style.rs` | 280 | `bb`, `cal`, `frak`, etc. | Declarativo | `typst_library::math::style` |
| `mod.rs` | 13545 | Reexports e registo da stdlib | Facade | Não existe equivalente estrutural no vanilla |
| `numbering.rs` | 331 | `numbering(...)` e `format_pattern` | Declarativo | `typst_library::model::numbering` |
| `panic.rs` | 40 | `panic(...)` | Imperativo | `typst_library::foundations` |
| `pdf.rs` | 180 | Namespace `pdf.*` | Declarativo | `typst_library::pdf` |
| `plugin.rs` | 553 | `plugin(...)`, `plugin.transition` | Imperativo (carrega WASM) | `typst_library::loading::plugin` |
| `primitives_constructors.rs` | 1007 | `decimal`, `duration`, `version` | Declarativo | `typst_library::foundations` |
| `ref.rs` | 60 | `ref(...)` | Declarativo | `typst_library::model::reference` |
| `shapes.rs` | 1054 | `rect`, `circle`, `line`, `polygon`, `curve` | Declarativo | `typst_library::visualize::{path, shape}` |
| `state.rs` | 403 | `state(...)` e métodos | Imperativo | `typst_library::introspection::state` |
| `structural.rs` | 4116 | `strong`, `emph`, `raw`, `heading`, `par`, table/grid/header/footer/cell/hline/vline, bibliography/cite | Declarativo (com grande aglomeração de domínios) | `typst_library::model::{table, heading, par, quote, ...}` + `typst_library::layout::grid` |
| `sym.rs` | 807 | Tabela `sym.*` | Declarativo | `typst_library::symbols::sym` |
| `sys.rs` | 90 | `sys.version`, `sys.inputs` | Declarativo | `typst_library::foundations` |
| `text.rs` | 1212 | `text(...)`, `upper`, `lower`, `regex`, `lorem`, decorations | Declarativo | `typst_library::text` |
| `transforms.rs` | 200 | `move`, `rotate`, `scale`, `skew` | Declarativo | `typst_library::layout::{move, rotate, scale, skew}` |
| `visualize.rs` | 332 | `tiling(...)` | Declarativo | `typst_library::visualize::tiling` |

### Observações da Fase C

- `foundations.rs` é o maior exemplo de aglomeração: mistura funções puramente declarativas (`type`, `len`, `repr`, constructors de tipos) com funções de runtime state (`state`, `counter`, `query`, `here`, `locate`). No vanilla, estas vivem em módulos separados (`foundations` vs `introspection`).
- `structural.rs` é o maior ficheiro (4116 linhas) e aglomera markup estrutural, tabelas, grids e bibliografia. Já foi identificado no Passo 1003 como hub grande.
- `mod.rs` (13545 linhas) é quase todo `#[cfg(test)] mod tests` (testes E2E da stdlib). A facade de reexports propriamente dita ocupa apenas as primeiras ~180 linhas.
- A maior parte dos submódulos é declarativa; os imperativos são: `assert`, `context`, `counter`, `eval`, `foundations` (misto), `loading`, `panic`, `plugin`, `state`.

---

## Fase D — Classificação da pergunta central

### Veredicto

**Decisão deliberada parcialmente registada**.

### Evidência

- **Deliberada**: a existência de `engine::stdlib` como módulo agregador de funções nativas é uma decisão explícita, registada em ADR-0037 ("`stdlib.rs` (Passo 96.5) — um ficheiro por módulo da stdlib" e "O `mod.rs` é o único a ter API pública para o resto do projecto"). O `_comum.md` reforça a separação de responsabilidades entre `eval` e `stdlib`.
- **Parcialmente registada**: nenhum ADR nem L0 discute explicitamente a alternativa "facade central de reexports vs cada submódulo exportar diretamente para `make_stdlib`". A facade é uma consequência da aplicação da Regra 3 de ADR-0037, não uma decisão arquitetural independente.
- **Não é acumulação sem decisão**: ao contrário de casos como `content.md` ou alguns prompts órfãos do Passo 1001, a estrutura `engine::stdlib/mod.rs + submódulos` não nasceu organicamente — foi imposta por ADR-0037.

### Implicação para futuro fatiamento

- Remover completamente a facade (fazer cada submódulo exportar diretamente, como no vanilla) **violaria ADR-0037** (que exige um `mod.rs` como API pública do módulo).
- Reorganizar o que está por trás da facade (mover nativas entre submódulos, dividir `foundations.rs` e `structural.rs`) **não viola a facade** e é o tipo de refactor previsto por ADR-0037.
- Se, por alguma razão, se quiser alinhar com o vanilla removendo a facade central, seria necessário **revisitar ADR-0037 formalmente** (gate ADR-0127, se alterar contrato público).

---

## Fase E — Catálogo (output)

### Classificação final

| Aspecto | Veredicto |
|---|---|
| `engine::stdlib` existe por decisão? | Sim — ADR-0037 + `_comum.md`. |
| A facade de reexports foi decidida explicitamente? | Não — é consequência da Regra 3 de ADR-0037. |
| Classe | **Decisão deliberada parcialmente registada**. |
| Fatiamento livre por trás da facade? | Sim, desde que o `mod.rs` permaneça como ponto de entrada. |
| Remoção da facade exige gate? | Sim — revisão de ADR-0037 e possivelmente ADR-0127. |

### Submódulos por eixo Declarativo/Imperativo

| Declarativo (Value → Value / construtores / tabelas) | Imperativo (contexto/estado/I/O/efeitos) |
|---|---|
| `calc` | `assert` |
| `collections` | `context` |
| `color` | `counter` |
| `emoji` | `eval` |
| `figure_image` | `foundations` (misto) |
| `gradients` | `loading` |
| `label` | `panic` |
| `layout` | `plugin` |
| `math_style` | `state` |
| `numbering` | |
| `pdf` | |
| `primitives_constructors` | |
| `ref` | |
| `shapes` | |
| `structural` | |
| `sym` | |
| `sys` | |
| `text` | |
| `transforms` | |
| `visualize` | |

### Nota sobre semelhança com casos anteriores

`engine::stdlib` não é da mesma classe que `content.md` ou os prompts órfãos do Passo 1001. Nesses casos, a estrutura existia sem decisão registada (ou o prompt existia sem o código o citar). Aqui, a estrutura é **deliberada**, embora a **forma específica da facade** não tenha sido debate independente. O fato de `mod.rs` ter 13.5k linhas deve-se quase inteiramente a testes E2E agregados — excepção da Regra 5 de ADR-0037 —, não a lógica de negócio na facade.

---

## O que este passo NÃO fez

- Não decidiu fatiar `engine::stdlib`.
- Não removeu a facade.
- Não presumiu a resposta antes de procurar a evidência.
