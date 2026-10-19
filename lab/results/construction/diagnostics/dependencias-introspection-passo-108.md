# Passo 108.C — Mapa de dependências e lacunas

**Data**: 2026-04-23
**Input**: 108.A (vanilla), 108.B (cristalino).
**Propósito**: identificar o **caminho crítico** mínimo para
activar funcionalidades novas no cristalino (numeração
hierárquica real, `query`, `@label` em eval), e as dependências
implícitas que cada componente traz.

---

## Tabela "precisa-de / é-precisado-por"

Por conceito vanilla, o que existe no cristalino e onde está a
lacuna concreta:

| Conceito vanilla | No cristalino | Lacuna face ao vanilla |
|------------------|---------------|------------------------|
| `Location (u128)` | **ausente** | Identidade única por elemento. No cristalino, identidade é pela posição na árvore + Label opcional. |
| `Tag (Start/End)` | **ausente** | Eventos emitidos pelo layout para construir Introspector. Cristalino processa `Content` directo. |
| `Locator` | **ausente** | Gera `Location`s hierarquicamente. No cristalino não há análogo. |
| `Introspector (trait)` | **ausente** (mas `CounterState` cobre alguns usos) | Índice global pesquisável (`query`, `query_count_before`, `labels`). |
| `Counter` | `CounterState` **funcional parcial** | `Counter::at(selector)` e `Counter::final_()` não existem. |
| `Selector` | `Selector::NodeKind` (só show rules) | Não consumido por Introspection; não suporta `Selector::Elem.where(field: value)`. |
| `Query` (função) | **ausente** | `query(heading)` não existe como função stdlib. |
| `State` | **ausente** | `state("k", init)` não existe como função stdlib. |
| `here()` | **ausente** | Sem `here()`, `counter.at(here())` é impossível. |
| `locate()` | **ausente** | Callback-based introspection. |
| `Convergence / History` | **ausente** (usa fixpoint limitado de páginas) | Multi-pass eval+layout+introspect. O cristalino só faz fixpoint de `label_pages`. |
| `Introspection (type-erasure)` | **ausente** | Não é god-struct — é wrapper para Sink. No cristalino, `Sink` existe mas não agrega introspecções. |
| `CounterUpdateElem` (elemento de layout) | `Content::CounterUpdate { key, action }` | Similar mas sem `Location`. |
| `Labelled` → `Locatable` | `Content::Labelled { target, label }` | Sem `Location`. Label é resolvida textualmente. |

---

## Grafo de dependências (vanilla)

```
                  Location
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
       Tag         Locator    CounterKey
        │            │            │
        └──►  Introspector  ◄─────┘
                     │
        ┌────────────┼────────────┬─────────┐
        ▼            ▼            ▼         ▼
     Counter       State        Query    Locator
        │            │            │         │
        └────────────┴────────────┴─────────┘
                     │
                     ▼
              Engine.introspect<I>
                     │
                     ▼
         Sink.introspections (Vec)
                     │
                     ▼
              History<T> (convergência)
```

### Observações

- **`Location` é raiz**. Sem `Location`, `Tag`, `Locator`, `Counter`,
  `State`, `Query` perdem a identidade de elemento. Todo o caminho
  crítico vanilla assume `Location` materializada.
- **Convergência é ortogonal**. Pode ser adiada — mesmo com
  `Location`/`Counter`/`Query` funcionais, single-pass já dá
  paridade para documentos "bem-comportados" (sem `counter.at()`
  que altere layout).
- **`Engine.introspect<I>`** é o **trampolim**. Todas as
  funcionalidades user-facing passam por lá.

---

## Caminho crítico por funcionalidade

### Objectivo 1 — `counter(heading).at(here())`

Requer, em ordem:

1. **`Location`** (u128 ou struct equivalente) — identidade única.
2. **`here()`** — função stdlib que retorna `Location` actual (do
   `context`).
3. **`context` em eval** — algum ponto onde `Location` actual é
   conhecida. No vanilla, `context` é produzido por
   `ContextualFunc::resolve_impl`.
4. **`Counter::at(loc)`** — método que consulta o `CounterState` no
   ponto `loc`.
5. **`Introspector::query_count_before(selector, loc)`** — para
   `at` funcionar correctamente, precisa saber quantos elementos
   do selector apareceram antes de `loc`.

**Tamanho estimado**: **grande**. O vanilla dedica múltiplos
ficheiros. Mesmo implementação mínima requer 4-5 conceitos novos.

### Objectivo 2 — `query(heading)`

Requer:

1. **`Selector` genérico** (não só `NodeKind`) — actualmente em
   `entities/show.rs` mas limitado.
2. **`Introspector::query(selector) -> Vec<Content>`** — índice
   pesquisável.
3. **Função stdlib `query`** — hoje ausente.
4. Opcional: `Location` se o utilizador quiser filtrar por
   posição (ex: `query(heading).before(here())`). Sem `Location`,
   `query(heading)` devolve todos os headings do documento
   (simplificação útil).

**Tamanho estimado**: **médio**. `Selector` já tem forma inicial;
`Introspector` teria de ser um novo tipo wrapping `CounterState +
headings_for_toc + figure_numbers + ...`.

### Objectivo 3 — `@label` resolvido em tempo de eval

Hoje `@label` vira `Content::Ref { target: Label }` e é resolvido
em `layout/references.rs` (via `resolved_labels`). Promover para
"resolvido em eval" requer:

1. `introspect()` correr antes de eval terminar — **conflito com
   arquitectura actual** onde introspect corre sobre o Content
   final.
2. Ou: introduzir um passe `CounterState::precompute(content)`
   durante eval (difícil sem Location).

**Tamanho estimado**: **grande e invasivo**. **Descartar** como
primeira materialização — quebra arquitectura sem trazer valor.

### Objectivo 4 — Fixpoint multi-pass completo

Requer:

1. `Sink` a colectar `Introspection`s (já tem infra base desde Passo
   104/106).
2. Re-eval com Introspector da passagem anterior — exige que eval
   possa consumir um Introspector como argumento.
3. Detecção de convergência por hash.

**Tamanho estimado**: **enorme**. Único valor visível é eliminar o
fixpoint limitado actual (label_pages). Não desbloqueia DEBTs
abertos. **Descartar** como primeira materialização.

### Objectivo 5 — `Location` mínima como chave

**Apenas** materializar `Location` como tipo (u128 opaco), sem
`Locator` nem `Introspector`. Utilidade: permitir que elementos
futuros ganhem identidade consistente entre passes. **Sozinho**
não traz valor user-facing; é **infra-estrutura**.

**Tamanho estimado**: **muito pequeno** (~100 linhas + testes).

### Objectivo 6 — `Introspector` mínimo wrapping `CounterState`

Materializar um tipo `Introspector` em L1 que **encapsula**
`CounterState` e expõe métodos limpos (`query_headings`,
`count_before`, `page_of_label`), **sem** `Location`. Serve como
rampa para Objectivo 1 e 2.

**Tamanho estimado**: **pequeno/médio**. Reorganização mais do que
código novo.

---

## Caminho crítico mínimo por objectivo

Tabela consolidada:

| Objectivo user-facing | Deps mínimas | Tamanho | DEBTs destrava |
|-----------------------|--------------|---------|----------------|
| `counter.at(here())` | Location + here + context + Counter.at + Introspector.count_before | **grande** | parte do DEBT-1 residual |
| `query(heading)` | Selector genérico + Introspector.query + stdlib | **médio** | nenhum aberto |
| `@label` em eval | re-arquitectura significativa | **enorme** | nenhum |
| Multi-pass convergência | Sink introspection aggregator + re-eval | **enorme** | nenhum |
| `Location` isolado | u128 wrapper | **muito pequeno** | prepara terreno |
| Introspector wrapping CounterState | reorganização | **pequeno/médio** | prepara Objectivo 2 |

---

## Lacuna arquitectural principal

**A lacuna não é de código — é de modelo**.

O cristalino modela introspecção como:
`Content → introspect() → CounterState → layout() → Frame`

O vanilla modela introspecção como:
`Content → eval → Tag stream → Introspector → [re-eval?] → layout → Frame`

Materializar qualquer parte do vanilla no cristalino obriga a
decidir: **ou absorver Location no Content (intrusivo), ou
manter Location como estrutura paralela (complexo), ou evitar
Location e reformular as novas APIs sem ela (diferente do
vanilla)**.

Essa decisão é arquitectural e toca ADR-0026 (Content enum
fechado). Não pode ser tomada pelo tamanho de estimativa — exige
uma conversa explícita a seguir ao 108.E.

---

## DEBTs desbloqueados por Introspection mínima

Cruzando os objectivos com os DEBTs abertos:

| DEBT aberto | Destrancável por |
|-------------|------------------|
| DEBT-1 (StyleChain resíduos — `counter.at(here())`) | Objectivo 1 (grande). Não por objectivos pequenos. |
| DEBT-45 (check_*_depth) | **Nenhum** destes objectivos. Ortogonal. |
| DEBT-2, 8, 9, 33, 34d, 34e, 35b, 42, 43, 50 | **Nenhum**. Não relacionados com Introspection. |

**Conclusão**: **nenhum DEBT aberto é completamente destravado por
uma Introspection mínima pequena** (Objectivos 5 ou 6). O DEBT-1
requer Objectivo 1 (grande). Isto é informação crítica para 108.D.
