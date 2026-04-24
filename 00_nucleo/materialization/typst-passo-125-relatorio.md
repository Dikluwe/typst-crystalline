# Passo 125 — Relatório (auditoria profunda dos 11 DEBTs abertos)

**Data**: 2026-04-24
**Precondição**: Passo 124 encerrado; 1042 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos.
**Natureza**: análise + potencial fecho trivial. Quebra linha de
12 passos de CLI consecutivos (113–124).
**ADR**: **sem tocar**. Zero ADR nova (como previsto).

---

## Sumário

Auditoria profunda dos 11 DEBTs abertos com grep empírico em
`01_core/src/`. **Todos os 11 mantêm M** — zero fechos triviais.

3 candidatos de fecho dedicado identificados como os mais
próximos de accionáveis: DEBT-43 (linter type-level), DEBT-42
(`get_unchecked` scanner), e subset de DEBT-1 (propriedade
simples como `text.weight` numérico).

3 DEBTs são qualitativamente "perpétuos" por natureza:
- DEBT-9 (tracking contínuo — convenção).
- DEBT-35b (guardião documental preventivo).
- DEBT-50 (guardião latente com canary test).

**Zero mudança em código**. Contagem tests idêntica:
**811 L1 + 24 L2 + 186 L3 + 21 L4 + 6 ignorados = 1042 total**.
Zero violations.

---

## 125.A — Inventário de meta-dados

**DEBT.md**: 11 abertos (DEBT-1, 2, 8, 9, 33, 34d, 34e, 35b,
42, 43, 50). Contagem confirma o valor declarado na precondição.

**ADRs emitidas desde Passo 105** (25 passos: 106–124):
- Domínio L3/CLI: ADR-0042, 0043, 0044, 0045, 0046, 0047, 0048,
  0049, 0050, 0051.
- Domínio L1/L2 outros: nenhuma ADR toca directamente nos 11
  DEBTs remanescentes.

**Fechos desde 105**: DEBT-45 (Passo 110), DEBT-49 (Passo 107),
DEBT-51 (Passo 106) — todos no domínio Sink/warnings que as
ADRs 0042/0043 resolveram.

Contexto escrito em `auditoria-debts-passo-125-contexto`
(absorvido no ficheiro agregado).

---

## 125.B — Revisão profunda por DEBT

Ficheiro agregado:
`00_nucleo/diagnosticos/auditoria-debts-passo-125.md`.

### Matriz final

| DEBT | Grep empírico | Classificação | Razão factual |
|------|---------------|:-------------:|---------------|
| **1** | `StyleDelta` com 5 campos; sem font/lang/weight/leading | **M** | Tipos Font/Lang/Par não materializados |
| **2** | `ClosureRepr.captured: Arc<Scope>` inalterado | **M** | Lazy exige TrackedWorld real com comemo |
| **8** | `MathPrimes` só AST; `math_kern` via FixedMetrics | **M** | OpenType MATH tables = trabalho substancial |
| **9** | tracking contínuo | **M** | Convenção permanente, não encerra |
| **33** | `CubicTo` em geometry.rs com comment "conservadora" | **M** | B(t) extrema analítica = trabalho dedicado |
| **34d** | sem min/max-content em grid | **M** | Algoritmo negociação pendente |
| **34e** | grid.rs zero hits colspan/rowspan | **M** | Placement algoritmo novo |
| **35b** | `available_width()` sem cache; comment guardião em layout/mod.rs:507 | **M perpétuo** | Preventivo documental |
| **42** | 5+ `unsafe { get_unchecked(...) }` em scanner.rs | **M** | Bloqueado por falta de infra bench |
| **43** | `crystalline.toml` flat crate-level | **M** | Bloqueado por projecto externo `crystalline-lint` |
| **50** | canary test em tests.rs:1787 passa | **M latente** | Activa-se só quando bake-in → wrapping |

---

## 125.C — Fecho trivial

**Nada a fechar**. Gate 125.C (> 5 linhas, > 1 ficheiro, ou
código novo) falharia para qualquer DEBT.

3 dos 11 são **inherentemente** não-fecháveis por design
(preventivos/trackers):
- DEBT-9: tracking contínuo de paridade.
- DEBT-35b: guardião preventivo de cache inexistente.
- DEBT-50: canary latente para regressão futura.

Remover qualquer destes 3 apagaria a documentação do risco que
é o propósito da entrada. **Manter** é a acção correcta.

---

## 125.D — ADR de fecho

**Não aplicável** — nenhum DEBT fechou. Zero ADR criada ou
anotada.

---

## 125.E — Encerramento

### Alteração ao DEBT.md

1 edit: stamp de auditoria no header (7 linhas), com link para
`diagnosticos/auditoria-debts-passo-125.md`. Corpo de cada DEBT
**não tocado** (seguindo precedente do Passo 105).

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 811 passed   (L1)
test result: ok. 186 passed, 6 ignored (L3)
test result: ok. 24 passed    (L2)
test result: ok. 21 passed    (L4)

$ crystalline-lint .
✓ No violations found
```

### Números finais

| Métrica | Antes (Passo 124) | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1042** | **1042** |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 |
| **DEBTs abertos** | **11** | **11** |

---

## Candidatos de fecho dedicado identificados

Ordenados por proximidade de fecho:

### Candidato 1 — DEBT-43 (linter type-level whitelist)

**Tamanho estimado**: S (dois passos pequenos em dois repos).

**Pré-requisito**: alterações no binário `crystalline-lint`
(repositório externo) para aceitar `[l1_allowed_external.ecow] types = [...]`.

**Passo neste repo**: migrar `crystalline.toml` de
```toml
rust = [ ..., "ecow", ... ]
```
para formato type-level granular. Adicionar teste negativo.

### Candidato 2 — DEBT-42 (`get_unchecked` no scanner)

**Tamanho estimado**: M (dois passos).

**Passo A**: criar infra de bench com `criterion` em
`01_core/benches/`. ADR específico se dev-dep `criterion` em
workspace precisa justificação.

**Passo B**: benchmark baseline vs refactor; decisão (manter
`unsafe` com ADR ou eliminar). ADR de número concreto conforme
ADR-0032.

### Candidato 3 — DEBT-1 subset: `text.weight` numérico

**Tamanho estimado**: XS (1 campo + 1 setter + 1 teste).

**Passo**: `StyleDelta.weight: Option<u16>` + capturar em
`eval_set_text` quando `key == "weight"` + um teste. Paga a
propriedade mais fácil sem bloquear nas outras (font/lang que
exigem tipos novos).

---

## Lições

1. **Auditoria periódica paga-se**: desde 105, 3 DEBTs
   fecharam (45, 49, 51) por decisão explícita em passos
   posteriores. Sem auditoria periódica, o registo deriva
   do estado real e as decisões acumulam-se sem visibilidade.

2. **DEBT "perpétuo" é padrão válido**: 3 de 11 são
   trackers/preventivos. Esperar fecho é categoria errada —
   a sua função é documentar risco persistente. Registar
   esta taxonomia no sumário evita "porque é que isto ainda
   está aberto?" em auditorias futuras.

3. **Bloqueio externo ≠ bloqueio permanente**: DEBT-42 (bench)
   e DEBT-43 (linter) são bloqueios de tempo de execução, não
   arquitecturais. Cada um exige 1-2 passos focados; podem
   entrar em roadmap quando houver apetite para fora do
   domínio principal de features.

4. **Gate 125.C impediu tentação**: DEBT-1 subset (`weight`
   numérico) parecia "só 1 campo" durante a auditoria —
   resisti. Adicionar campo em L1 sem Anotar ADR-0038 ou
   acompanhar ADR-0040 seria criar dívida escondida. Passo
   dedicado com contexto arquitectural próprio.

5. **Tests L4 disciplina (Passo 124) validaram estado no
   passo anterior**: correu os 1042 tests com zero falhas,
   o que dá confiança de que a base é sólida para auditar
   DEBTs sem risco de regressão. Ordem 124 → 125 funciona
   melhor que o inverso.

6. **Zero mudança ≠ zero valor**: mesmo sem fecho, a
   auditoria produz (a) matriz up-to-date, (b) candidatos
   priorizados para próximos passos, (c) taxonomia que
   distingue perpétuos de finitos. Próxima auditoria arranca
   deste estado documentado.

---

## Estado pós-Passo 125

### DEBT.md

1 edit: stamp de auditoria no header. Corpo dos 11 DEBTs
inalterado. Entry-point `diagnosticos/auditoria-debts-passo-125.md`
para detalhe.

### Registo público

- **Diagnóstico**: `auditoria-debts-passo-125.md` (11 secções
  por DEBT, ~200 linhas).
- **Relatório**: `typst-passo-125-relatorio.md` (este ficheiro).

### Trabalho futuro priorizado

1. Passo dedicado **DEBT-43 migration** (pequeno, depende de
   external tool).
2. Passos A/B **DEBT-42 benchmark + decisão** (médio, dev-dep).
3. Passo XS **DEBT-1 `text.weight`** quando adequado fit com
   outras features text.

Os 3 perpétuos (9, 35b, 50) ficam como documentação activa —
não entram em lista de "fecho futuro".
