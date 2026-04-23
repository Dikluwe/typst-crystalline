# Passo 110 — Relatório de encerramento (DEBT-45 fechado)

**Data**: 2026-04-23
**Precondição**: Passo 109 encerrado; Engine<'a> materializado;
803 L1 + 184 L3 + 6 ignorados; zero violations.
**Natureza**: passo **puramente documental**. Gate 110.A activado
positivamente (todas as pendentes "não aplicáveis"). Zero código
de produção alterado.

---

## Sumário

DEBT-45 **ENCERRADO**. Forma escolhida: **Opção A** (funções livres,
sem refactor).

- **2/4 `check_*_depth` aplicáveis** — ambas já integradas no Passo 93.
- **2/4 `check_*_depth` não aplicáveis** — documentadas.
  - `check_layout_depth` — Layouter cristalino não tem Route/Engine
    (divergência arquitectural).
  - `check_html_depth` — cristalino não tem pipeline HTML.

Zero código tocado. ADR nova não necessária (Opção A).

---

## 110.A — Inventário

Inventário em
`00_nucleo/diagnosticos/inventario-debt45-passo-110.md`.

### Tabela final

| Check | Limite | Call site | Estado |
|-------|-------:|-----------|--------|
| `check_call_depth` | 80 | `closures.rs:98` em `apply_closure` | ✓ Integrada (Passo 93) |
| `check_show_depth` | 64 | `rules.rs:66` em `apply_show_rules` | ✓ Integrada (Passo 93) |
| `check_layout_depth` | 72 | — | ⊘ **Não aplicável** |
| `check_html_depth` | 72 | — | ⊘ **Não aplicável** |

### Razões de "não aplicável"

**`check_layout_depth`**: o Layouter cristalino opera sobre `Content`
já avaliado (`pub fn layout(content: &Content, initial_state:
CounterState) -> PagedDocument`). Grep `Route|Tracked|engine|Engine`
em `01_core/src/rules/layout/`: **zero matches**. Integrar esta check
exige propagar `Route` ou `Engine` por 10+ funções de layout —
refactor de magnitude equivalente ao Passo 92 (eval) ou 109 (Engine),
aplicado a um submódulo diferente. Fora do âmbito estrito do Passo
110 ("se propagação > 2 funções, **parar**").

**`check_html_depth`**: grep `Html|html` em `01_core/src/` retorna
apenas 1 match — a própria definição de `check_html_depth` em
`world_types.rs`. Sem pipeline HTML, não há onde chamar a check.

### Estado `check_call_depth` vs `EvalContext.enter_call`

Confirmado no inventário: `EvalContext.enter_call`,
`leave_call` e campos `depth`/`max_call_depth` foram **removidos**
no Passo 93. Hoje `check_call_depth` (função livre sobre
`Tracked<Route>`) é a única implementação. Sem duplicação.

---

## 110.B — ADR

**Não aplicável**. Opção A não requer ADR nova (conforme spec 110.B).

---

## 110.C — Implementação

**Nenhuma.** Forma A + todas as pendentes "não aplicáveis" =
passo documental. `cargo check -p typst-core`: inalterado.

---

## 110.D — Testes

**Nenhum teste novo** (conforme spec: "Para checks que eram 'não
aplicáveis no cristalino', sem testes. O DEBT é fechado por
documentação.").

Os 803 L1 + 184 L3 testes existentes — que já exercitam
`check_call_depth` e `check_show_depth` integradas — continuam a
passar.

---

## 110.E — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 803 passed; 0 failed; 0 ignored ...     (L1 inalterado)
test result: ok. 184 passed; 0 failed; 6 ignored ...     (L3 inalterado)

$ crystalline-lint .
✓ No violations found
```

### DEBT-45 ENCERRADO

Movido para Secção 2 de `00_nucleo/DEBT.md` com entrada detalhada
que documenta:
- 2 checks integradas (Passo 93).
- 2 checks não aplicáveis (Passo 110) + razões.
- Trabalho futuro registado (não bloqueia o DEBT).
- Padrão comemo herdado preservado.

### ADR

Nenhuma criada.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 803 | 803 (inalterado) |
| L3 tests | 184 | 184 (inalterado) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 44 | 44 (inalterado) |
| DEBTs abertos | 12 | **11** (−DEBT-45) |

---

## Lições

1. **Gates de escopo são precisos**: o spec 110 antecipou explicitamente
   o cenário "todas as pendentes não aplicáveis → passo documental".
   O gate disparou como desenhado; não houve tentação de expandir.

2. **"Não aplicável" não é "adiado"**: a divergência arquitectural do
   Layouter (ADR-0026, sem Route/Engine) é **decisão de design**,
   não pendência. Documentar como "não aplicável" preserva a
   clareza; registar "trabalho futuro" quando/se a arquitectura mudar.

3. **Paridade estrutural preservada mesmo sem uso**: as 2 funções
   não chamadas (`check_layout_depth`, `check_html_depth`) permanecem
   em `world_types.rs` e nos testes. Preservam paridade com
   vanilla (ADR-0033); ficam prontas para quando/se vierem a ser
   necessárias.

4. **Passo documental vale passo**: nem toda conclusão exige código.
   Fechar DEBT-45 por documentação clarifica o inventário de DEBTs
   abertos — 12 → 11 — e dá visibilidade a decisões arquitecturais
   que antes viviam implícitas.

---

## Estado pós-Passo 110

### DEBT-45 fechado sem código

```
Passo 91:  DEBT-45 aberto (4 checks definidas, 0 chamadas).
Passo 93:  DEBT-45 parcialmente pago (2/4 integradas: call, show).
Passo 110: DEBT-45 encerrado (2/4 documentadas não aplicáveis: layout, html).
```

### Trabalho futuro (não obrigatório)

Se/quando:
- O Layouter for refactored para receber `Engine<'_>` ou `Route`, um
  passo futuro pode integrar `check_layout_depth`.
- Um pipeline HTML for materializado, um passo futuro pode
  integrar `check_html_depth`.

Ambos são passos dedicados, não extensões do Passo 110.

### Sem ADR nova

A ADR-0033 (paridade funcional) já cobre a filosofia: "paridade
funcional, não estrutural". Documentar "não aplicável" é
consistente com ADR-0033 — preserva o nome/API vanilla mas aceita
que o contexto cristalino pode ainda não permitir o uso.
