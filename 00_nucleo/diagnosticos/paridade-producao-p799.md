# Relatório de Verificação — Passo 799: `math::attach` — sub/superscript quebrado (achado P798 #13)

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD)
- **Working tree na sonda "antes":** limpa sobre esse commit
- **Working tree na validação "depois":** `00_nucleo/prompts/engine/math/layout/attach.md`, `01_core/src/engine/math/layout/attach.rs`, `01_core/src/engine/math/layout/tests.rs` (+121/-9)
- **Hora da Medição:** 2026-07-21 ~15:32 (-0300)
- **Relatório de materialização:** `00_nucleo/materialization/typst-passo-799-relatorio.md`

---

## 1. O Problema Relatado

Achado #13 de P798 (prioridade alta): `$integral_0^1 x^2_3$` extraía como `∫10x23` — sub/superscript "completamente quebrado" em qualquer documento com matemática.

## 2. Diagnóstico e Medição

Sonda com `mutool trace` (regra §5 — texto extraído não basta) sobre `$integral_0^1 x^2_3$`: com sub+sup **simultâneos**, o cristalino compunha os dois scripts **em sequência horizontal** (sup em x=74.882, sub em x=78.462) e a largura do attach ignorava o sub — o `x` seguinte sobrepos-se ao sub (mesmo x=78.462). Casos isolados (`$x^2$`, `$x_1$`) já estavam correctos, o que explica o achado só se manifestar na combinação.

Pontos exactos: vanilla `crates/typst-layout/src/math/scripts.rs:170-175` (`tr_x = br_x = pre_width + base_width + kern`); cristalino `01_core/src/engine/math/layout/attach.rs` (cursor único que avançava após o sup).

**Sobreposições testadas e refutadas como mesma causa** (obrigatório pelo passo): achado #7 (`#if true [Hello $x^2$]` — tratado em P800) e itálico matemático de P786 §7 (`αβ` vs `𝛼𝛽` — selecção de fonte/estilo, continua em aberto).

## 3. A Solução Implementada

L0 `attach.md` actualizado primeiro (nova regra P799: scripts laterais partilham a origem x; largura = base + max(sup, sub)); hash corrigido via `crystalline-lint --fix-hashes` (`b3b29f07` → `572a89a1`). No braço não-`is_limits`: ambos os scripts partem de `base_offset_x + base_width` (cada um com o kern do seu quadrante); `post_width = max(sup + kern_sup, sub + kern_sub)`.

Validação `mutool trace` depois: `1` e `0` ambos em x=74.882 (empilhados), `x` seguinte em 78.462 sem sobreposição — geometria equivalente ao vanilla.

## 4. Testes Automatizados Persistidos (com nomeação explícita)

- `math_attach_sub_sup_partilham_origem_x` (novo, `math/layout/tests.rs`): falhou antes (`sup_x=7.2 sub_x=12.24`).
- `math_attach_sub_sup_largura_max_nao_soma_nucleo_multi_char` (novo): núcleo multi-caractere `ab^22_333` + elemento seguinte; falhou antes (`sup_x=14.4 sub_x=24.48`).
- Cobertura pré-existente mantida: `math_attach_sup_elevado`, `math_attach_sub_baixado`.

## 5. Verificação de Sucesso do Workspace

```
Suite 'typst-core' (lib):  ANTES 4317 passed → DEPOIS 4319 passed; 0 failed (+2 testes ✓)
crystalline-lint . → exit 0 (zero violações)
```

Nota de limpeza registada: dois scripts descartáveis de P798 (`temp/run_p798_tests2.py`, `run_p798_tests3.py`, não commitados, conteúdo re-criável a partir dos `.typ` preservados) disparavam V8/V1 desde antes deste passo e foram removidos para repor o gate do lint.

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
