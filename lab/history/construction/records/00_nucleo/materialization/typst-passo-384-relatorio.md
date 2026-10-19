# Passo 384 — relatório: recon amplo read-only

**Tipo:** diagnóstico read-only. **Data:** 2026-06-19. **HEAD:** `d47b57e1b` (pós-P383).
**Caveat de stack:** `RUST_MIN_STACK=33554432`.

## O que se fez
Fotografia factual do projeto inteiro + backlog priorizado, em 5 eixos. **Zero `.rs` alterado;
zero ficheiro movido; zero ADR/DEBT criado ou fechado.** Os 3 documentos grandes (cobertura ~316KB,
DEBT.md ~169KB, adr/README ~265KB) foram lidos por sub-agentes em paralelo; LOC/matches/divergências
medidos por grep.

## Artefactos (todos `.md`, commitados)
- `typst-cobertura-vanilla-vs-cristalino.md` — **nota de refresh P384** anexada (não reescrito): o
  doc já estava mantido até P299; delta P299→P384 = 0 features (arco P300-383 = de-baking +
  atomização content-preserving).
- `diagnostico-recon-amplo-passo-384.md` — os 5 eixos (cobertura, monólitos por camada, DEBTs, ADRs
  PROPOSTO, divergências) + critérios fixados.
- `backlog-priorizado-passo-384.md` — 10 itens, tamanho XS-XL, bloqueios, decisões do dono marcadas.
- este relatório.

## Achados-chave
- **Cobertura:** user-facing **~69%**, arquitetural **~82%**. A premissa do plano ("Introspection
  17%") está **obsoleta** — real **83%** (M3-M9 pós-P160). Mais baixos: **Model 50%**,
  **Visualize 54%**.
- **Monólitos:** o limiar foi fixado em **>800 LOC de produção** (degrau natural). A maior frente é
  o **cluster `stdlib/`** (`layout` 1451, `structural` 1289, `foundations` 989, `calc` 809, …) — que
  **coincide com DEBT-57** (L0 ausentes p/ ~70 fns). `stdlib/mod.rs` (8242) **não é monólito**: ~94
  prod + ~8147 testes. `content.rs` é o hub (enum-dados + delegação). `introspect`/`math` já fechados.
- **DEBTs:** 9 abertos (máx DEBT-61). Destaques: DEBT-57 (stdlib L0), DEBT-55 (bibliography, bloqueado
  por ADR-0062), DEBT-59 (full-error CLI), DEBT-60 (heading/outline).
- **ADRs PROPOSTO:** 11; o `adr/README.md` está **desatualizado** (numera até 0094; repo tem até
  0109) — dívida documental.
- **Divergências:** ~120 marcadores `ADR-0054 graded` registados; as não-registadas são dívida
  documental (auditoria é item de backlog).

## Backlog (topo)
1. Atomizar `stdlib/` + escrever L0s (L; fecha DEBT-57; maior monólito). 2. Refrescar `adr/README.md`
(S). 3. DEBT-60(b) outline supplement (S). 4. DEBT-59 full-error CLI (M). 5-7. Model/Visualize/
Bibliography (L-XL; #6 bloqueado por ADR-0062). 8-9. ADR-0066 / decisão de crates (dono).

## Gates
- **lint:** `crystalline-lint .` = 0 violações (cross-check; sem V10 — `lab/` isolado).
- **suíte:** não tocada (0 alterações de código → sem regressão).
- **ADR/DEBT:** zero criada/fechado. **árvore de código:** 0 `.rs` alterado.
- **mitigação do mecanismo externo:** os 4 `.md` commitados assim que escritos.

## Portões de decisão (fora deste passo — §7, do dono)
Quais monólitos atomizar primeiro (recomendação: stdlib) e a decisão de crates. O P384 entrega os
dados; a escolha é do dono. **Termina aqui — não emenda o seguinte (Trava 5).**
