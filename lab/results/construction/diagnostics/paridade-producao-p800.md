# Relatório de Verificação — Passo 800: `syntax::kind` — output diverge em `#if` com math inline (achado P798 #7)

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD)
- **Working tree na sonda "antes":** alterações de P799 (irrelevantes para este achado — `$x^2$` isolado já posicionava correctamente antes de P799, verificado por trace)
- **Working tree na validação "depois":** P799 + P800 (8 ficheiros, +213/-31)
- **Hora da Medição:** 2026-07-21 ~15:55 (-0300)
- **Nota de localização:** este é o relatório canónico do passo (convenção: relatórios vivem em `00_nucleo/diagnosticos/`). O duplicado em `materialization/` foi removido por essa convenção.

---

## 1. O Problema Relatado

Achado #7 de P798: `#if true [Hello $x^2$]` — "ordem/conteúdo do output completamente diferente do vanilla" (cristalino `2`/`Hello x` em duas linhas vs vanilla `Hello 𝑥2`).

## 2. Diagnóstico e Medição

Isolamento da variável (obrigatório pelo passo): `#if true [Hello]` idêntico nos dois; `Hello $x^2$` **sem `#if`** mostra a mesma divergência — `#if` é irrelevante. `mutool trace` de `Hello $x^2$`: texto "Hello" na baseline y=763.785, math "x" em y=769.285 — **5.5pt = `axis_pt` acima da baseline do texto**. Vanilla: texto e math na mesma baseline (spans com y=0 na mesma transform). O desalinhamento fazia o `pdftotext` partir a linha (a "ordem diferente" do achado).

Causa: `01_core/src/engine/layout/equation.rs` aplicava `offset_y = cursor_y - axis_pt` (regra do Passo 48: "alinhar o eixo matemático à baseline do texto") — **refutada por medição**: o vanilla alinha a **baseline**; o eixo fica `axis_height` acima e só governa o centrado interno (`apply_axis_offset`). A regra errada estava consagrada no L0 `equation.md` e no teste `equacao_inline_sobe_em_relacao_ao_baseline`. Os items de `MathLayouter::layout_equation` já vêm baseline-relativos (y=0 na baseline), logo a integração correcta é somar `cursor_y`.

**Fusão com #13 (P799)?** Não — causas distintas (composição interna do attach vs offset global equação↔texto).

**Nota de processo:** uma primeira versão (`offset_y = cursor_y - math_ascent`, com método novo) piorou o desvio (8.8pt); os testes novos apanharam-na e foi revertida dentro do próprio passo.

## 3. A Solução Implementada

L0 `equation.md` reescrito (baseline com baseline; regra Passo 48 revogada com referência à medição) + nota P800 em `math/layout/_comum.md`; hashes corrigidos (`equation.rs` → `d63d4e36`). Código: `offset_y = self.regions.current.cursor_y` (bloco inalterado — já era `cursor_y`).

Validação `mutool trace` depois: math "x" em y=763.785 — **mesma baseline** do "Hello"; sup 3.98pt acima (análogo ao vanilla ~4pt). `pdftotext`: `Hello x2` vs vanilla `Hello 𝑥2` — ordem/posição iguais; residual `x` vs `𝑥` = itálico matemático (P786 §7, causa distinta, **continua em aberto**).

## 4. Testes Automatizados Persistidos (com nomeação explícita)

- `equacao_inline_baseline_coincide_com_texto` (novo): falhou antes com a diferença exacta de `axis_pt` (`texto_y=78.567 math_y=73.067`).
- `if_com_math_inline_attach_baseline_coincide` (novo): caso original `#if true [Hello $x^2$]`.
- `equacao_inline_sobe_em_relacao_ao_baseline` (existente): consagrava a regra refutada — marcado `#[ignore]` com justificação e ponteiro para o substituto (não apagado).

## 5. Verificação de Sucesso do Workspace

```
Suite 'typst-core' (lib):  ANTES 4319 passed → DEPOIS 4320 passed; 0 failed; 1 ignored (total 4321 = +2 ✓)
crystalline-lint . → exit 0
```

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
