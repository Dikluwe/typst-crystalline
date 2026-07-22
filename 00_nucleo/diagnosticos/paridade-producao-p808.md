# Relatório de Verificação — Passo 808: citações remanescentes da regra revogada do Passo 48

**Data:** 2026-07-21
**Status:** Concluído — zero citações activas; 1 achado novo registado (não corrigido, por regra do passo)
**Proveniência da Medição:**
- **Commit Base:** `c98ffc8ac` (HEAD; working tree P799–P807 não commitado na altura)
- **Hora da Medição:** 2026-07-21 ~18:15 (-0300)
- **Nota de localização:** este é o relatório canónico do passo (convenção: relatórios vivem em `00_nucleo/diagnosticos/`). O duplicado em `materialization/` foi removido por essa convenção.

---

## 1. O Objectivo do Passo

P800 revogou a regra do Passo 48 ("alinhar o eixo matemático à baseline do texto") por medição. Este passo era uma varredura de confirmação: (1) encontrar citações activas da regra antiga em código/L0/ADRs/dívidas; (2) confirmar que o eixo não é usado como offset vertical noutro ponto, incluindo equações em bloco (fora do escopo de P800).

## 2. Diagnóstico e Medição

Varredura textual reproduzível (`grep -rn "Passo 48" .`, `grep -rni "axis\|eixo matemático" 00_nucleo/adr/ 00_nucleo/diagnosticos/debt/DEBT.md 01_core/src/`): todas as ocorrências classificadas — (a) já corrigidas/anotadas por P800 (`equation.rs`, L0 `equation.md`, teste `#[ignore]`); (b) citações históricas inofensivas (secção de smoke tests em `integration_tests.rs:725`, `apply_axis_offset` — centrado interno legítimo, handoffs, prompts históricos P47–P50, DEBT.md). **Zero ocorrências da classe (c)** (citação activa que assume a regra errada).

Varredura de comportamento: `axis_pt`/`axis_height` só é consumido por `apply_axis_offset` (centrado interno). Bloco nunca teve shift. Teste de equação em bloco (`Antes $ x^2 $ Depois`, `mutool trace`, posições absolutas):

| Elemento | Cristalino | Vanilla |
|---|---|---|
| "Antes" baseline | 78.105 | 78.104 ✓ |
| math `x` baseline | 92.493 (+14.4) | 100.410 (+22.3) |
| "Depois" baseline | 104.736 (+12.2) | 120.969 (+20.6) |
| math `x` horizontal | 70.867 (margem esq.) | 291.993 (**centrado**) |

## 3. Resultado

- **Passo 1:** zero citações activas da regra revogada — assunto fechado documentalmente.
- **Passo 2 — ACHADO NOVO (registado, não corrigido, conforme a regra do passo):** equação em bloco no cristalino (1) **não é centrada horizontalmente** e (2) tem espaçamento vertical mais apertado que o vanilla. Pré-existente (o caminho de bloco não foi tocado por P800); candidato a passo dedicado próprio.

## 4. Testes Automatizados Persistidos

Nenhum (passo de confirmação; a suite manteve-se verde — `typst-core 4336 passed; 1 ignored` na altura).

## 5. Verificação de Sucesso do Workspace

`crystalline-lint .` → exit 0. Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
