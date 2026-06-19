# Backlog priorizado — o que falta (Passo 384)

Derivado do `diagnostico-recon-amplo-passo-384.md` (recon read-only). **Tamanhos:** XS<½dia ·
S≈1 passo · M≈2-3 passos · L≈vários lotes · XL≈frente inteira. **Este documento prioriza; não
decide** (os portões §7 são do dono). Ordem = sequência defensável, não obrigatória.

| # | Item | Domínio | Tamanho | Bloqueado por | Razão da prioridade |
|---|---|---|:--:|---|---|
| 1 | **Atomizar o cluster `stdlib/` + escrever os L0s** (layout/structural/foundations/calc/shapes/transforms/gradients/assert/…) | L1 stdlib | **L** | — | Maior monólito de produção (Eixo B) **e** fecha **DEBT-57** (L0 ausentes p/ ~70 fns). Continuação natural do arco de atomização (ADR-0109), forma já provada 44×. Não bloqueado. |
| 2 | **Refrescar o `adr/README.md`** (numera até 0094; faltam 0095-0109) | docs | **S** | — | Dívida documental barata; o índice de ADRs está ~37 ADRs atrás. Alta alavanca / baixo custo. |
| 3 | **DEBT-60 (b) — fix do supplement do outline** | L1 introspect/layout | **S** | — | Lote isolado (P359 já mapeado); divergência de numeração de heading no outline. User-facing. |
| 4 | **DEBT-59 — flag de erro completo (CLI `--full-error` → L1)** | L2/L4 + L1 | **M** | — | User-facing; desbloqueado; toca CLI(L2)+wiring(L4)+eval(L1). |
| 5 | **Cobertura Model (50%) — a mais baixa user-facing** | L1 model | **L** | parcial: DEBT-55 | Subir o domínio mais fraco (structural parcial: 7 parciais, 4 ausentes). Parte depende de bibliografia (#6). |
| 6 | **DEBT-55 — Bibliography + Cite (CSL real)** | L1 model | **XL** | **ADR-0062** (PROPOSTO, crate `hayagriva`) | User-facing grande; **requer decisão do dono** (autorizar crate externo, ADR-0062). Até lá fica no parody manual. |
| 7 | **Cobertura Visualize (54%)** | L1 visualize | **L** | — | 2.º domínio mais fraco (5 ausentes); gradientes já fortes, faltam formas/padrões. |
| 8 | **ADR-0066 — Introspection runtime (P160B minimal)** | L1 introspect | **M** | decisão do dono | Promove a reserva conceptual; subset minimal. |
| 9 | **Decisão de crates** (§7 do plano) | arquitetura | — | **decisão do dono** | A frente que seguia à varredura no plano original; só após o backlog estar à frente. |
| 10 | **content.rs / entities grandes** (hub + dados) | L1 entities | **M** | — | Baixa prioridade: `content.rs` é o hub (enum-dados + delegação-magra, P375); os `entities/*` grandes são dados/tipos — atomização menos óbvia que o stdlib. |

## Bloqueios e decisões do dono (resumo)
- **ADR-0062 (hayagriva)** bloqueia DEBT-55 (#6) → decisão de autorizar crate externo.
- **Decisão de crates** (#9) → frente arquitetural, pós-backlog.
- **ADR-0066** (#8) → decisão de materializar introspection-runtime.
- **DEBT-42/43** (infra: bench, linter) — fora da linha user-facing; baixa prioridade salvo necessidade.

## Diferidos / scope-out (não entram na linha)
- **Marco G / desacoplamento** — descartado (ADR-0109 não-meta).
- **ADRs 0008-0015 (inlining/perf)** — PROPOSTO deferido; perf, não correção.
- **DEBT-50** — latente (só dispara com de-bake bold/italic).
- **DEBT-58** — triado (decisão fechada, sem trabalho ativo).

## Recomendação de leitura (sem decidir)
A sequência de **menor risco e maior alavanca** começa em **#1 (stdlib + L0s)** — fecha um DEBT,
ataca o maior monólito, e usa a forma de atomização já provada. **#2 (README ADRs)** e **#3
(DEBT-60b)** são vitórias baratas em paralelo. As frentes user-facing grandes (#5/#6/#7) e as
decisões de crate/ADR (#6/#8/#9) são **do dono**, com estes dados na mão.
