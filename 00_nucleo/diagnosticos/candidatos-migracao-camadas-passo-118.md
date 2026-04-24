# Passo 118.E — Candidatos de migração de camadas

**Data**: 2026-04-23
**Input**: auditorias L1 (limpa), L2 (limpa), L3 (1 candidato óbvio
+ 3 fronteiriços), L4 (thin, 1 candidato dependente).

---

## Candidato 1 — `format_diagnostic` (L3 → L2) — **ÓBVIO**

**Origem**: `03_infra/src/diagnostic_format.rs:46-88`.
**Destino**: `02_shell/src/cli.rs` (ou novo `02_shell/src/diagnostic.rs`).

**Razão**:
1. Decide formato user-facing (palavras "warning:", "error:",
   "hint:") — presentation concern.
2. Aplica escapes ANSI — presentation concern.
3. Define indentação de hints — presentation concern.
4. L2 é a camada de CLI; diagnósticos ao utilizador são matéria
   de CLI.
5. Passo 117 corrigiu parcialmente — `ColorWhen` e `resolve_colored_with`
   migraram; o formatter ficou por migrar porque o tempo do passo
   não abrangia. **Correcção pendente.**

**Tamanho**: **XS-S** (~50 linhas código + 7 testes). Paleta ANSI
(6 const) migra junto.

**Interdependências**:
- `format_diagnostic` usa `Source::span_to_line_col` (L1) — ok, L2
  pode importar L1.
- `format_diagnostic` usa `Severity`, `SourceDiagnostic` (L1) — ok.
- L2 actualmente importa L1 via `clap` derive? Não — 02_shell já
  tem dep `typst-core` declarada mas não usada. Migração activa o
  uso.

**Bloqueios**: nenhum. L2 já tem infra (cli.rs exists); basta
adicionar o formatter.

**Consequência em L3**: módulo `diagnostic_format.rs` fica com
apenas `drain_diagnostics_to_stderr` (3 linhas) que depende do
formatter em L2. Isto cria **dependência L3 → L2 invertida** —
anti-pattern. Resolver com Candidato 2 (par acoplado).

**Passo sugerido**: **migrar junto com Candidato 2** no mesmo passo
(par acoplado). Refactor unitário, ~100 linhas tocadas.

---

## Candidato 2 — `drain_diagnostics_to_stderr` (L3 → dissolver) — **dependente de 1**

**Origem**: `03_infra/src/diagnostic_format.rs:94-104`.
**Destino**: **L4 inline** (3 linhas de `for ... eprint!(...)`).

**Razão**:
1. Se Candidato 1 migra, o formatter está em L2.
2. `drain_*` é só um loop sobre `format_diagnostic` + `eprint!`.
3. Manter `drain_*` em L3 após Candidato 1 criaria import cíclico:
   L3 importaria `typst_shell::cli::format_diagnostic`. L3 → L2
   é **inversão de dependência**.
4. Alternativas:
   - **(a)** Mover `drain_*` para L2 também. **Problema**: L2
     passa a fazer I/O (`eprint!`). Viola "L2 não escreve".
   - **(b)** Mover `drain_*` para L4 (inline 3 linhas). **Vantagem**:
     L3 perde função inútil; L4 cresce 3 linhas; dependência
     continua L4 → L2 (normal).
   - **(c)** Manter `drain_*` em L3 + importar L2 em L3.
     **Rejeitar**: inversão de dep.

**Tamanho**: **XS** (3 linhas em L4, remove 10 de L3).

**Passo sugerido**: **juntar com Candidato 1**.

---

## Candidato 3 — `eval_to_module_with_sink` (L3 → ?) — **FRONTEIRIÇO**

**Origem**: `03_infra/src/pipeline.rs:34-56`.

**Classificação**: **fronteiriço** — composição + esconde boilerplate
comemo. Não tem I/O directo (não toca filesystem nem stderr).

**Opções**:

- **(a) Manter em L3** como "pipeline braçal". Precedente: `layout.rs`
  que compõe `Layouter::new(...)` + métricas. Pattern estabelecido
  "L3 é o braço pesado".
- **(b) Mover para L4**. Problema: L4 cresce ~25 linhas (boilerplate
  comemo). L4 passa de ~55 para ~80 linhas úteis. Perto do limite
  mental "thin".

**Razão factual para manter em L3**:
- Esconde `comemo::Track` de L4. L4 não vê `.track()` /
  `.track_mut()` / `Routines::new()` / `Traced::default()` /
  `Sink::new()` / `Route::root()`.
- Torna L4 mais thin (55 linhas vs 80).
- Análogo a `layout_with_font` (pipeline de layout em L3).

**Razão factual para mover para L4**:
- É composição pura de L1 (eval) + comemo (infra).
- L3 idealmente é "I/O bruto", e eval não é I/O.

**Recomendação**: **manter em L3**. Aceitar que L3 tem dois papéis:
I/O bruto + composição que esconde infra. Documentar como padrão
arquitectural.

**Passo sugerido**: **não migrar**. Aceitar como-está.

---

## Candidato 4 — `compile_to_pdf_bytes` (L3 → ?) — **FRONTEIRIÇO**

**Origem**: `03_infra/src/pipeline.rs:63-80`.

**Mesmas análise** que Candidato 3 — compõe eval + introspect +
layout + export_pdf.

**Recomendação**: **manter em L3** com `eval_to_module_with_sink`.
Par natural; migrar um sem outro divide o "pipeline braçal".

**Passo sugerido**: **não migrar**.

---

## Ranking final

Por critério combinado (clareza do erro × tamanho × independência):

| # | Candidato | Clareza | Tamanho | Independência | Recomendação |
|---|-----------|:-:|:-:|:-:|--------------|
| 1 | `format_diagnostic` → L2 | ★★★ | XS | depende de 2 | **Migrar** (par 1+2 em passo dedicado) |
| 2 | `drain_diagnostics_to_stderr` → L4 | ★★★ | XS | depende de 1 | **Migrar com 1** |
| 3 | `eval_to_module_with_sink` (fronteiriço) | ★ | S | — | **Manter** |
| 4 | `compile_to_pdf_bytes` (fronteiriço) | ★ | S | — | **Manter** |

---

## Candidatos adicionais (auditoria completa)

### L1: zero candidatos

Ver `auditoria-l1-passo-118.md`. L1 limpo.

### L2: zero candidatos

Ver `auditoria-l2-passo-118.md`. L2 correcto pós-117.

### L4: zero candidatos urgentes

Ver `auditoria-l4-passo-118.md`. L4 thin (85 linhas). Único
candidato dependente = Candidato 2 (remover `drain_*` import +
inline).

---

## Recomendação primária para passo seguinte

### Passo 119 sugerido: **migrar Candidatos 1+2 juntos**

Escopo:

1. Mover `format_diagnostic` + paleta ANSI (6 const) + testes
   `format_diagnostic_*` de L3 para L2.
2. Remover `drain_diagnostics_to_stderr` de L3.
3. Inline o drain em L4: 3 linhas de `for ... eprint!(format_diagnostic(...))`.
4. Actualizar imports: L3 perde `diagnostic_format` (ou fica só
   com paleta se mais algo emergir); L4 passa a importar
   `format_diagnostic` de L2.
5. Prompts: `diagnostic_format.md` de L3 migra ou desaparece;
   `shell/cli.md` descreve o novo formatter.

Tamanho estimado: **M** — comparável ao Passo 117.

Testes: **redistribuição** (6-7 `format_diagnostic_*` movem de L3
para L2). Total workspace inalterado.

ADR: sim. Corrigir ADR-0049 que deixou o formatter por
migrar (ou nova ADR "completa a migração do Passo 117").

### Candidatos 3 e 4: sem passo dedicado

Manter em L3. Documentar o padrão "pipeline braçal em L3" na
ADR-0049 do passo seguinte (como anexo clarificador).

---

## Conclusões

1. **1 candidato óbvio** — `format_diagnostic`.
2. **1 candidato dependente** — `drain_diagnostics_to_stderr`.
3. **2 fronteiriços** — recomendação **manter**.
4. **L1, L2, L4** sem candidatos.
5. **Passo seguinte focado**: migração do par 1+2.

Auditoria **não revelou crise**. Correcção pendente é pequena e
específica.
