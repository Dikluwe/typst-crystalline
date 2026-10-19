# Passo 958 — Relatório (nomes gregos literais em `Nome(args)` + 13 nomes em falta na tabela)

**Data**: 2026-08-04
**Estado da árvore**: commit base `d472b523a` (P957); alterações deste passo por cima.

---

## 1. Fase A — causa confirmada (uma só, não duas)

Os 7 casos reportados (`Phi(𝑥)`, `chi(𝑀)`, `Gamma(𝑧)`, `zeta(𝑠)`,
`Psi(𝑥,`, `chi(𝐺)`, `omega(𝐺)`) têm **causa única** — não há padrão
maiúscula/minúscula: o fallback do braço `Expr::FuncCall` de
`eval_math_expr` (`eval/math.rs`) só consultava `lookup_math_op` antes de
cair no literal `Content::MathIdent(name)`. A tabela `ident_to_unicode`
sempre teve os 6 nomes — o caminho standalone (`$ Gamma $`) resolvia
correctamente; só o caminho `Nome(args)` falhava. Reproduzido:
`$ Gamma $` → Γ ✓, `$ Gamma(z) $` → "Gamma(𝑧)" ✗ (antes da correcção).

**Inventário contra o vanilla** (codex 0.3.0 `modules/sym.txt`): a tabela
cristalina tinha mais **13 nomes gregos canónicos em falta** —
minúsculos `digamma` (ϝ) e `omicron` (ο); maiúsculos `Chi`, `Eta`,
`Iota`, `Kappa`, `Mu`, `Nu`, `Omicron`, `Rho`, `Tau`, `Upsilon`, `Zeta`.
`$ Chi $` standalone dava `unknown variable` (ausente das duas tabelas
consultadas — medido antes da correcção).

## 2. Fase B — implementação (TDD directo)

- **L0 primeiro**: `math/symbols.md` §P958 + `engine/eval.md` §P958.
- **Testes RED** (4 a falhar, 1 guarda já verde):
  `p958_simbolo_grego_com_args_resolve_para_glifo` (os 6 nomes com args),
  `p958_nomes_gregos_em_falta_standalone` (os 13 nomes),
  `p958_nomes_gregos_em_falta_com_args`, `p958_gregos_em_falta_convertem_
  para_unicode` (tabela); guarda
  `p958_sin_parens_prioridade_operador_preservada` (já verde antes).
- **Correcção**: (1) `ident_to_unicode` ganha os 13 nomes (paridade codex);
  (2) o fallback de `FuncCall` passa a espelhar a cadeia standalone:
  `lookup_math_op` → `ident_to_unicode` → `sym_lookup` (com warning de
  depreciação P820) → literal `MathIdent` (P303 preservado para nomes
  desconhecidos; args preservados via `MathDelimited` como P302/P303).
- **Suite**: `cargo test --workspace` — **5687 testes, 0 falhas**.
- **Glifo** (Fase B.4): documento de 30 secções — zero literais
  `Gamma|zeta|Phi|chi|Psi|omega` no texto extraído; contagens de glifos
  gregos idênticas às do vanilla (3 Γ, 2 Φ, 4 Ψ — `pdftotext`); revisão
  visual lado a lado das secções 25/26/28 (`temp/p958/sec958.png`):
  Γ(z), ζ(s), Ψ(x,t), χ̂, χ(G) ≥ ω(G) todos correctos.

## 3. Fase C — revalidação

- **`compare.py`** (secções 25/26/28, as afectadas): med|dx| melhora nas
  três — sec 25: 4.029 → **1.477**; sec 26: 4.653 → **3.652**; sec 28:
  6.919 → **1.625**. O remanescente são as outras divergências já
  catalogadas (espaçamento de limites de operador grande — P959, etc.).
- **Benchmark**: ver tabela abaixo (hyperfine, "antes" = release do estado
  pós-P957; JSONs em `tools/perf/results/p958-canonical/`).

| Cenário | antes (ms) | depois (ms) | ratio |
|---|---|---|---|
| 01-hello | 86.83 | 88.62 | 1.021 |
| 02-lorem | 105.80 | 106.65 | 1.008 |
| 03-images | 93.01 | 94.53 | 1.016 |
| 04-math | 119.53 | 120.44 | 1.008 |
| 05-tables | 91.33 | 91.85 | 1.006 |
| 06-long | 293.90 | 297.84 | 1.013 |
| 07-context | 131.14 | 128.96 | 0.983 |

Ratio médio **1.008** — zero regressão dentro do ruído de medição.

## 4. Notas

- A dupla tabela de símbolos (`ident_to_unicode` em `math/symbols.rs` +
  `SYM_SIMPLE` em `stdlib/sym.rs`) é parcial nas duas — a união cobre o
  uso comum. P958 completou os nomes gregos nas duas pontas do caminho
  afectado; uma consolidação das duas tabelas numa SSoT é candidata a
  passo próprio (não bloqueante).
