# ADR-0044 — `Engine<'a>` como agregador de estado de eval em L1

**Estado**: EM VIGOR (Passo 109.E, 2026-04-23)
**Data**: 2026-04-23
**Autor**: Passo 109
**Revoga**: nenhuma — complementa ADR-0036.
**Relacionadas**: ADR-0036 (atomização progressiva), ADR-0037
(coesão por domínio), ADR-0042 (Sink materializado), ADR-0043
(canal Sink→L3).

---

## Contexto

A ADR-0036 estabeleceu a atomização progressiva do `EvalContext`:
extrair campos que descrevem fluxo de eval para parâmetros
explícitos das funções `eval_*`. Cinco aplicações concretas
(Passos 92, 94, 95, 98, 107) extraíram:

1. `route` (Passo 92) — `Tracked<'r, Route<'r>>`.
2. `styles` (Passo 94) — `&mut StyleChain`.
3. `show_rules` + `active_guards` (Passo 95) — `&mut Arc<[ShowRule]>`
   + `&mut Vec<RuleId>`.
4. `current_file` + `figure_numbering` (Passo 98) — `FileId` e
   `&mut Option<String>`.
5. `sink` (Passo 107) — `&mut TrackedMut<'_, Sink>`.

Resultado: assinaturas das `eval_*` chegaram a **10 parâmetros +
`ctx: &mut EvalContext<'_>`**. Passo 107 registou a pressão
visual: "10 params + ctx é o limite visual" (relatório 107,
Lições §5). Passo 108 analisou alternativas e recomendou
consolidação (Candidato 5 — "Engine<'a> com Introspection stub
interno").

Este ADR documenta a decisão e o pattern.

---

## Decisão

Introduzir em L1 uma struct `Engine<'a>` em
`01_core/src/entities/engine.rs` com 8 campos que agregam todos
os parâmetros extraídos pela ADR-0036. Funções `eval_*` passam a
receber `engine: &mut Engine<'_>` em vez dos 8 parâmetros
individuais.

### Estrutura

```rust
pub struct Engine<'a> {
    // Handle externo
    pub world: &'a dyn World,

    // Fluxo de eval (ADR-0036)
    pub route: Tracked<'a, Route<'a>>,
    pub styles: &'a mut StyleChain,
    pub show_rules: &'a mut Arc<[ShowRule]>,
    pub active_guards: &'a mut Vec<RuleId>,
    pub current_file: FileId,
    pub figure_numbering: &'a mut Option<String>,

    // Efeitos laterais (ADR-0042, ADR-0043)
    pub sink: &'a mut TrackedMut<'a, Sink>,

    // Stubs futuros — documentam divergência face ao vanilla:
    // pub introspector: Introspector,    // Passo dedicado
    // pub routines: &'a Routines,         // Passo dedicado
    // pub traced: Tracked<'a, Traced>,    // Passo dedicado
}
```

### Ordem dos campos

Coesa por domínio (ADR-0037):

1. **Handle externo**: `world`.
2. **Fluxo de eval**: `route`, `styles`, `show_rules`,
   `active_guards`, `current_file`, `figure_numbering`.
3. **Efeitos laterais**: `sink`.

A ordem não bate com a ordem de campos do `typst-library::engine::Engine`
vanilla. Prioridade: ADR-0037 sobre paridade literal de ordem. Nomes
dos campos batem com vanilla (paridade funcional, ADR-0033).

### Lifetime

Um único `'a`, unificado em 109.A. `Engine<'a>` é paramétrica
apenas nesse lifetime. Detalhes em
`00_nucleo/diagnosticos/inventario-engine-passo-109.md`.

### Campos omitidos

Vanilla tem `introspector: Protected<Tracked<dyn Introspector>>`,
`routines: &Routines`, `traced: Tracked<'a, Traced>`. Cristalino
**ainda não** materializou estes subsistemas (ver
`00_nucleo/diagnosticos/cristalino-introspection-passo-108.md`).
Ficam como comentários no `Engine`, não como campos nem stubs.
Entrarão em passos dedicados (Passo 110+).

### `world` move de `EvalContext` para `Engine`

Consequência: `EvalContext` fica com **3 campos** (`loop_iterations`,
`max_loop_iterations`, `next_rule_id` — todos Regra 4 da ADR-0036:
contadores/alocadores monotónicos independentes de fluxo).
`EvalContext` perde o lifetime `'w`.

---

## Relação com ADR-0036 (inversão controlada)

A ADR-0036 extraiu 5 vezes do `EvalContext`. Este ADR **não
invalida** essas extracções. Reconhece que, depois de cada campo
ser identificado como "dependência de fluxo explícita", agregá-los
num **tipo agregador transparente** (`Engine<'a>`) é um **segundo
passo de organização** — com ganhos distintos:

- **Legibilidade**: 24 funções `eval_*` passam de `10 params + ctx`
  para `1 param + ctx`.
- **Extensibilidade**: adicionar um novo campo ao eval (ex:
  `introspector` no Passo 110+) é adicionar campo ao Engine, não
  propagar 11º parâmetro por 24 funções.
- **Consolidação de nomes**: `engine.styles` explicita a que tipo
  pertence o borrow.

Pattern: **extrair primeiro (ADR-0036), agregar depois
(ADR-0044)**. Justifica-se quando o número de extracções cruza um
limite pragmático (neste caso: 10 params — experimentalmente
considerado "limite visual" em 107).

A extracção não foi erro e a agregação não é retratação. São
fases de um mesmo processo: primeiro separar para compreender,
depois nomear colectivamente.

---

## Paridade com vanilla

- **Nomes dos campos**: idênticos aos do vanilla
  `typst-library::engine::Engine` (world, route, sink,
  styles/ShowRules via Styles). Paridade nominal.
- **Tipos dos campos**: seguem o cristalino. Ex: `&mut StyleChain`
  onde vanilla tem `StyleChain<'a>` com semântica próxima; `&'a
  dyn World` (não-tracked) onde vanilla tem
  `Tracked<'a, dyn TrackedWorld + 'a>`.
- **Ordem**: ADR-0037 (coesão por domínio). Diferente da ordem
  vanilla.
- **Tracking**: `Engine<'a>` **não** é `#[comemo::track]`. Os
  campos individuais (`world`, `route`, `sink`) já encapsulam
  tracking onde apropriado. Tracking do Engine inteiro seria
  complexo e não traz valor hoje.

---

## Alternativas rejeitadas

### R-1 — Manter 10 parâmetros individuais

**Rejeitada**. Pressão visual documentada no Passo 107. Próxima
aplicação de ADR-0036 (ex: introspector) adicionaria 11º
parâmetro.

### R-2 — Materializar Introspector/Routines/Traced antes do Engine

**Rejeitada**. Passo 108 mostrou que cada um é passo
arquitectural significativo (Location, Locator, multi-pass
convergência). Consolidar o que já existe entrega valor
independente e facilita os próximos passos.

### R-3 — Big-bang vs incremental

**Incremental-por-campo**: cada sub-passo migra um campo
isoladamente. Rejeitada porque 8 sub-passos produzem 8 Engine
incompletos + boilerplate temporário duplicado. Viabilidade baixa.

**Incremental-por-função**: Engine completo, funções migram uma
a uma. Rejeitada porque cross-calls (`eval_expr` ↔ `eval_set_rule`
↔ `eval_markup`) obrigam ou adaptadores frágeis ou migração
conjunta. 109.A avalia que migração conjunta é preferível.

**Big-bang** (escolhida): todas as 24 funções migram no mesmo
passo. Refactor mecânico de alto volume. Justificada em 109.A.

### R-4 — Excluir `route` do Engine

**Avaliada e rejeitada**. O `route` muda em `apply_closure` e
`eval_module_include` (via `Route::extend(route)` que cria novo
segmento). Isso obriga a **reconstrução** do Engine nesses
sítios — mas é equivalente à reconstrução exigida por `styles`
(CodeBlock) e `current_file` (module_include). Consolidar todos
os 8 campos evita dissonância de "alguns dentro, outros fora".
Ceremony concentrada em 5-7 sítios de scope.

### R-5 — `Engine` tracked via `#[comemo::track]`

**Rejeitada**. Tracking complexo sem valor hoje. Os campos já
tracked (`route`, `sink`) mantêm tracking individual. Pode ser
revisto se materialização de `Introspector` exigir.

---

## Consequências

### Positivas

1. Assinaturas `eval_*` passam de 10+ctx para 1+ctx. Legibilidade
   recuperada.
2. Adicionar novo campo de eval é tocar `Engine` + eval() — não
   propagar 24× pelas funções.
3. Nome explícito `engine.styles` → documentação implícita.
4. `EvalContext` volta a ser **só Regra 4** (3 campos
   monotónicos) — validação *a posteriori* da categorização da
   ADR-0036.

### Negativas

1. Scope-local reconstructions adicionam ~60 linhas de
   boilerplate em 5-7 sítios (CodeBlock, ContentBlock, strong/
   emph/heading, apply_closure, eval_module_include).
2. Novo lifetime explícito `'_` em 24 assinaturas.
3. Borrow-checker pode exigir ceremonia em sítios onde múltiplos
   campos do Engine são usados simultaneamente (disjoint borrows).
   Mitigável mas aumenta código.

### Neutras

1. Assinatura pública de `eval()` **inalterada**
   (`TrackedMut<Sink>` continua a entrar por valor — ADR-0043).
2. Testes de linha de base (803 L1 + 184 L3) são critério de
   sucesso — nenhum teste novo neste passo.

---

## Aplicação

Implementado no Passo 109.C — ver
`00_nucleo/materialization/typst-passo-109-relatorio.md`.

ADR promovida a **EM VIGOR** em 109.E.
