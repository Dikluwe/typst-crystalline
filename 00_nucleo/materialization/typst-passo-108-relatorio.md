# Passo 108 — Relatório: análise de `Introspection` e recomendação

**Data**: 2026-04-23
**Precondição**: Passo 107 encerrado; DEBT-49 fechado; 803 L1 +
184 L3 + 6 ignorados; zero violations.
**Natureza**: passo de **análise**. Zero código de produção,
zero ADRs novas, zero testes novos.

---

## Sumário executivo

`Introspection` no vanilla **não é um god-struct**. É uma **família
de tipos** (Location, Counter, State, Query, Tag, Locator,
Introspector trait) coordenada pelo `Engine` e colecionada pelo
`Sink` via type-erasure (`Introspection(Arc<dyn Bounds>)`), com
convergência por hash em até 6 iterações (convergence.rs).

No cristalino **já existe** uma versão simplificada e funcional:
`rules/introspect.rs` (689 linhas) + `entities/counter_state.rs`
(290 linhas). Faz **single-pass** sobre `Content`, popula um
`CounterState` plano que absorve papéis de Counter, labels, TOC,
figure numbers. Documenta a divergência do vanilla como
intencional (header de `counter_state.rs`). Suporta:
numeração hierárquica de headings, forward refs, TOC com
materialização de AST (DEBT-18 resolvido), figuras numeradas,
fixpoint limitado de `label_pages`.

**O que não suporta** (lacuna user-facing):
`counter.at(here())`, `query(...)`, `locate(...)`, `here()`,
`state(...)`, multi-pass convergência completa.

**Nenhum DEBT aberto** depende apenas de uma Introspection pequena.
O DEBT-1 residual ("propriedades adicionais... `counter.at(here())`")
depende de um candidato **grande**.

---

## Gráfico de dependências (simplificado)

```
Vanilla (ideal):
  Location → Locator → Tag → Introspector → Counter/State/Query
                                     ↓
                              Engine.introspect
                                     ↓
                              Sink + History

Cristalino (hoje):
  Content → introspect::walk → CounterState → layout → Frame
                                     ↑
                 (leitura plana, single-pass)
```

A lacuna **não é de código**; é de **modelo**:
- `Location` em vanilla é raiz. No cristalino, identidade é
  posicional/textual.
- Multi-pass em vanilla é normal. No cristalino, single-pass é
  design.
- Introdução de qualquer peça vanilla obriga a decidir:
  **intrusivo em Content** (ADR-0026) vs **paralelo** vs
  **reformulado sem Location**.

Detalhes em `00_nucleo/diagnosticos/dependencias-introspection-passo-108.md`.

---

## Candidatos a sub-escopo

Ranking com critérios: pequeno (≤ Passo 104), desbloqueia DEBT
aberto, viável sem nova arquitectura.

| # | Candidato | Tamanho | Desbloqueia DEBT? | Viável hoje? |
|---|-----------|---------|:---:|:---:|
| 1 | `Location` opaca como tipo | XS | Não | Sim |
| 2 | `Introspector` wrapping `CounterState` | S-M | Não | Sim |
| 3 | `query(heading)` stdlib | M-L | Não | Exige multi-pass |
| 4 | Location + here + counter.at | L | DEBT-1 parcial | Exige multi-pass + decisão arq. |
| 5 | `Engine<'a>` + Introspection stub interno | M-L | Não (remove pressão 107) | Sim |

**Nenhum** candidato satisfaz as três condições simultaneamente.

Detalhes em `00_nucleo/diagnosticos/candidatos-introspection-passo-108.md`.

---

## Recomendação primária

**Candidato 5 — `Engine<'a>` com Introspection stub interno**.

### Porquê

1. **Precondição empírica**: Passo 107 registou os 10 params + ctx
   dos `eval_*` como "limite visual". Candidatos 2/3/4 todos
   pressionam para 11º, 12º parâmetro. Engine absorve e estanca.
2. **Refactor mensurável**: 24 funções da 5ª aplicação ADR-0036,
   7 ficheiros. Sem "mystery box".
3. **Zero decisão arquitectural**: não exige decidir onde vive
   `Location`, nem multi-pass, nem novas variantes de Content.
4. **Precedente**: segue o padrão de passos anteriores que
   consolidaram concerns (ex: StyleChain do Passo 30, show_rules
   do Passo 68).

### O que fica para depois

- Candidato 2 (Introspector wrapping) — fica trivial depois de
  Engine. Próximo passo natural.
- Candidato 4 (counter.at(here())) — espera por decisão
  arquitectural sobre Location.
- Candidato 3 (query) — espera por decisão sobre multi-pass.

### Avisos sobre o que pode dar errado

**Engine<'a>**:
- **Risco**: refactor mecânico pode regredir testes subtis se o
  `&mut` ordering mudar. Mitigação: `cargo test` após cada
  ficheiro.
- **Risco**: lifetimes complexos (10 campos com lifetimes
  distintos). Mitigação: assumir `'r` lifetime único
  quando possível (como route já faz).
- **Risco**: um campo `TrackedMut<Sink>` dentro de struct tem
  lifetime que pode colidir com outros `Tracked<Route>`.
  Mitigação: testar construção mínima primeiro.

**Candidato 2 (Introspector) — se for a escolha em vez de 5**:
- **Risco**: renomear campos públicos do `CounterState` quebra
  muitos call sites (90+ no tests.rs do layout). Mitigação:
  manter aliases durante transição.

**Candidato 4 (se escolhido directamente)**:
- **Risco arquitectural**: decisão sobre Location em Content
  reverbera em ADR-0026. Requer ADR própria.
- **Risco de tamanho**: facilmente excede Passo 100 em scope.

---

## Se a decisão for "não fazer nada de Introspection agora"

Trabalhar noutros DEBTs abertos enquanto a decisão arquitectural
cozinha. DEBTs não relacionados:

- DEBT-2 (closures com captura tardia), DEBT-8, DEBT-9, DEBT-33,
  DEBT-34d, DEBT-34e, DEBT-35b, DEBT-42, DEBT-43, DEBT-45,
  DEBT-50.

Cada um sem dependência de Introspection.

---

## Ficheiros de diagnóstico produzidos

- `00_nucleo/diagnosticos/vanilla-introspection-passo-108.md` —
  inventário do vanilla (tipos, funcionalidades, grafo de deps).
- `00_nucleo/diagnosticos/cristalino-introspection-passo-108.md`
  — inventário do cristalino actual (CounterState, introspect.rs,
  DEBTs relacionados).
- `00_nucleo/diagnosticos/dependencias-introspection-passo-108.md`
  — tabela precisa-de/é-precisado-por, caminhos críticos por
  objectivo, lacuna arquitectural.
- `00_nucleo/diagnosticos/candidatos-introspection-passo-108.md`
  — cinco candidatos ranqueados, avaliação por critério.

Este relatório (`typst-passo-108-relatorio.md`) agrega as
conclusões.

---

## Verificação

- **Zero** alterações a código de produção.
- **Zero** ADRs novas.
- **Zero** testes novos ou removidos.
- `cargo test --workspace`: **803 L1 + 184 L3 + 6 ignorados**
  (inalterado).
- `crystalline-lint .`: **zero violations**.

---

## Saída deste passo

Este relatório **não toma decisão**. Serve como input para a
conversa onde o sub-escopo será escolhido. Pergunta directa para
o próximo turno:

> Confirma-se **Candidato 5 (Engine<'a> com stub)** como Passo
> 109? Ou prefere-se um dos alternativos (Cand.2, adiar Introspection
> para trabalhar outros DEBTs)?

Recomendação: **Candidato 5**. Razões em `candidatos-...passo-108.md`.
