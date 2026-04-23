# Passo 110.A — Inventário DEBT-45

**Data**: 2026-04-23
**Input**: estado pós-Passo 109 (Engine materializado; world fora de EvalContext).

---

## Parte 1 — Checks existentes

Grep por `check_*_depth` + `MAX_*_DEPTH` em `01_core/src/`:

### Constantes (em `entities/world_types.rs`)

```rust
impl Route<'_> {
    pub const MAX_SHOW_RULE_DEPTH: usize = 64;
    pub const MAX_LAYOUT_DEPTH:    usize = 72;
    pub const MAX_HTML_DEPTH:      usize = 72;
    pub const MAX_CALL_DEPTH:      usize = 80;
}
```

### Funções livres (todas em `entities/world_types.rs`)

| Função | Assinatura | Definida | Call sites produção |
|--------|------------|----------|:---:|
| `check_call_depth` | `fn(Tracked<'_, Route<'_>>) -> SourceResult<()>` | :278 | **1** (`closures.rs:98` em `apply_closure`) |
| `check_show_depth` | `fn(Tracked<'_, Route<'_>>) -> SourceResult<()>` | :278 | **1** (`rules.rs:66` em `apply_show_rules`) |
| `check_layout_depth` | `fn(Tracked<'_, Route<'_>>) -> SourceResult<()>` | :293 | **0** |
| `check_html_depth` | `fn(Tracked<'_, Route<'_>>) -> SourceResult<()>` | :307 | **0** |

Nota: tests em `world_types.rs` chamam `check_layout_depth` e
`check_html_depth` em asserções unitárias, mas **não** são call
sites de produção.

---

## Parte 2 — Sítios candidatos para integração

### `check_layout_depth`

Candidato no vanilla: função recursiva do Layouter.

**Análise do Layouter cristalino** (`01_core/src/rules/layout/`):

- Grep por `Route|Tracked|engine|Engine` em `01_core/src/rules/layout/`:
  **zero matches**.
- Assinatura pública: `pub fn layout(content: &Content, initial_state: CounterState) -> PagedDocument`.
  Sem `Route`, sem `Engine`.
- Recursão visível em `Layouter::layout_content(&mut self, content: &Content)`
  sobre `Content::Sequence`, `Content::Heading { body }`,
  `Content::Styled`, `Content::Labelled { target }`, `Content::Figure`,
  `Content::Transform`, `Content::Grid { cells }`, `Content::Align`,
  `Content::Place`.
- Recursão puramente sobre a árvore `Content` — não cruza ficheiros,
  não partilha Route com eval.

**Divergência arquitectural**: o layouter cristalino opera sobre
`Content` já avaliado (ADR-0026); não recebe `Route<'_>` nem `Engine<'_>`.
Propagar `Route` pelo Layouter seria refactor análogo aos Passos 92
(eval) e 109 (Engine) aplicado a 10+ funções de layout. O DEBT.md
existente admite:

> "A pipeline de layout do cristalino opera sobre `Content` já avaliado,
> sem receber `Route<'a>`. Propagar `route` para o `Layouter` é refactor
> análogo ao do Passo 92 mas num submódulo diferente; encaixa melhor no
> passo que materializar `Engine<'a>` (que alinha `route` + layout num
> só contexto). Fica para passo dedicado."

**Passo 109 materializou `Engine<'a>` mas *não* o levou ao Layouter**:
o Engine é exclusivo do eval (ver ADR-0044, relatório 109). O
Layouter permanece arquitecturalmente separado.

**Decisão**: `check_layout_depth` **não é aplicável** na arquitectura
actual do Layouter cristalino. Integrá-la exige propagar Route/Engine
pelo Layouter — refactor fora do âmbito do Passo 110 (explícito em
"se propagação > 2 funções, parar e reportar").

### `check_html_depth`

**Análise**: grep `Html|html` em `01_core/src/`: match apenas em
`world_types.rs` (a própria função `check_html_depth`). **Não existe
pipeline HTML no cristalino.**

**Decisão**: `check_html_depth` **não é aplicável** — não há
pipeline HTML para verificar.

---

## Parte 3 — Estado de `check_call_depth` vs `EvalContext`

O Passo 17 introduziu `EvalContext.depth` + `enter_call`. Passo 93
removeu estes campos (ver DEBT-45 §"Estado actual"):

> "`EvalContext::check_call_depth`, `enter_call`, `leave_call`, e os
> campos `depth`/`max_call_depth` foram **removidos**."

Grep `enter_call` em `01_core/src/`: **zero matches**. Confirmado.

`check_call_depth` como **função livre sobre `Tracked<Route>`** é a
única implementação hoje. Sem duplicação.

---

## Parte 4 — Forma escolhida

Avaliação das opções:

**Opção A — Funções livres, só integrar**:
- As 2 checks aplicáveis (`call`, `show`) **já estão integradas**
  (closures.rs:98, rules.rs:66).
- As 2 pendentes (`layout`, `html`) são **não aplicáveis** — não há
  onde as chamar sem refactor fora do âmbito.
- **Sem ADR**. Passo vira puramente documental.

**Opção B — Método de `Route` tracked**:
- `Route` já tem `#[comemo::track] impl` (para `contains`/`within`).
- Limitações comemo (descobertas dos Passos 106-109) impediriam
  métodos `check_*_depth` nativos — constroem `SourceDiagnostic` que
  não é memoizable; o Passo 93 já justificou esta decisão:
  > "por motivos de arquitectura do comemo 0.4.0, os 4
  > `check_*_depth` foram refactorados de métodos `Route::check_*`
  > para funções livres".
- Retrocederia uma decisão arquitectural já estabelecida.
- **Rejeitada**.

**Opção C — Método de `Engine`**:
- `Engine::check_call_depth(&self, span)` etc. delegariam a
  `check_call_depth(self.route)`. Útil para call sites que têm
  `&mut Engine`.
- Não destranca nada no Layouter (que não tem Engine).
- Refactor cosmético — os 2 call sites actuais teriam de mudar de
  `route_check_*(engine.route)?` para `engine.check_*()?`.
- Mínima ADR necessária.
- **Viável mas baixo valor**. Não necessária para fechar DEBT-45.

### Decisão: **Opção A**, sem alterações de código de produção

- **2/4** checks aplicáveis **já integradas** (call, show).
- **2/4** checks documentadas como **não aplicáveis** (layout, html).
  Razão: divergência arquitectural do Layouter (sem Route/Engine) e
  ausência de pipeline HTML.
- **Sem ADR nova** (Opção A não a requer).
- **Passo puramente documental** — gate do spec 110.A activado
  positivamente.

Gate relevante:
> "**Gate**: se 110.A revelar que **todas** as checks pendentes são
> 'não aplicáveis no cristalino', o passo torna-se puramente
> documental — actualizar DEBT-45 para '2/4 aplicáveis, todas
> integradas; 2/4 não aplicáveis por ausência de pipeline
> correspondente' e fechar."

---

## Conclusão

| Check | Limite | Call site | Status |
|-------|-------:|-----------|--------|
| `check_call_depth` | 80 | `closures.rs:98` (`apply_closure`) | ✓ Integrada (Passo 93) |
| `check_show_depth` | 64 | `rules.rs:66` (`apply_show_rules`) | ✓ Integrada (Passo 93) |
| `check_layout_depth` | 72 | — | ✗ **Não aplicável** — Layouter cristalino não tem Route/Engine (divergência arquitectural) |
| `check_html_depth` | 72 | — | ✗ **Não aplicável** — sem pipeline HTML no cristalino |

**Forma**: Opção A (funções livres, sem mudança de código).
**ADR**: nenhuma necessária.
**Critérios DEBT-45**:
- [x] `check_call_depth` ✓
- [x] `check_show_depth` ✓
- **Não aplicável**: `check_layout_depth`, `check_html_depth`

**DEBT-45 fecha por documentação**, não por integração adicional.

### Trabalho futuro registado (não obrigatório)

- Se/quando o Layouter for refactored para usar `Engine<'a>` (ou
  equivalente com Route), `check_layout_depth` torna-se accionável.
  Passo dedicado futuro.
- Se/quando um pipeline HTML for materializado, `check_html_depth`
  torna-se accionável. Passo dedicado futuro.

Estas 2 funções permanecem definidas em `world_types.rs` — prontas
a ser usadas quando a arquitectura permitir. **Não são dead code**:
preservam paridade estrutural com o vanilla (ADR-0033) e são
testadas em `world_types.rs::tests`.
