# Relatório de Verificação — Passo 812: resto do achado #13 (`math::style`)

**Data:** 2026-07-21
**Status:** Concluído com Sucesso (4 sub-achados)
**Proveniência da Medição:**
- **Commit Base:** `c98ffc8ac` (HEAD) + working tree P808–P812
- **Hora da Medição:** 2026-07-21 ~21:30 (-0300)
- **Nota de localização:** este é o relatório canónico do passo (convenção: relatórios vivem em `00_nucleo/diagnosticos/`). O duplicado em `materialization/` foi removido por essa convenção.

---

## 1. Os Quatro Problemas

Do achado #13 de P810 (a corrupção do PDF ficou em P811): **(A)** `display`/`inline`/`script`/`sscript` sem efeito geométrico; **(B)** itálico de P809 perdido em wrappers de tamanho; **(C)** `scr` com bloco Unicode errado + sem variation selectors; **(D)** `NN`/`RR`/`ZZ`/`QQ`/`CC` ausentes.

## 2. Diagnósticos

- **A+B (entrelçados):** o factor de tamanho existia mas **P809 introduzira regressão** — `layout_equation` chamava `apply_math_style` no topo, que consumia os nós `MathStyled` (o handler do factor nunca corria); e `map_glyph` curto-circuitava size variants antes do default de itálico. Isolado com teste L1 em minutos (lição de P811: isolar a variável antes de corrigir).
- **C:** codex `to_roundhand = to_script + VS2(U+FE01)` e `to_chancery = to_script + VS1(U+FE00)` — `scr` usa os MESMOS codepoints de `cal`, diferenciados pelo selector. O cristalino mapeava `scr` para bold-script e o L0 consagrava a regra errada ("Roundhand = Bold Script", refutada por medição).
- **D:** codex `sym.txt:1243` (`NN ℕ`); cristalino `unknown variable`.

## 3. Soluções

- **A+B:** novo `apply_math_default` no topo (preserva `MathStyled`); size variants tratam-se como `Plain` no eixo glifo; composição por eixos ortogonais (outer size + inner glyph → glyph do inner, `script(bb(R))` → ℝ 0.7×; ambos size → outer vence, regra P311b.4 preservada).
- **C (opção (a) do prompt):** `map_glyph` normaliza `Roundhand → Chancery`; nova `map_glyph_vs` (VS1 `cal` / VS2 `scr`, só latinas) anexada pelos consumers.
- **D:** 5 entradas em `ident_to_unicode`; resto do alfabeto DS scope-out (on-demand).

## 4. Validação (literal, depois)

- A: script 𝑥 **7.7pt** (0.7×), sscript 𝑥 **5.5pt** (0.5×), display/inline 11pt — proporções idênticas ao vanilla (`mutool trace`). Scope-out registado: variante grande de operadores em display (`display(∑)`).
- B: `$script(x)$`/`$display(x)$` extraem `𝑥` U+1D465 == vanilla.
- C: `$scr(A) cal(A) scr(L) cal(B) scr(B)$` — sequência de codepoints **byte-idêntica** ao vanilla (`1D49C FE01 1D49C FE00 2112 FE01 212C FE00 212C FE01`); renderização por imagem sem caixas.
- D: `$NN RR ZZ QQ CC$` → `ℕℝℤℚℂ` idêntico.

## 5. Testes e Contagens

Novos (7): `p812a_script_aplica_factor_tamanho`, `p812a_sscript_aplica_factor_tamanho`, `p812b_italico_atravessa_wrapper_de_tamanho`, `p812b_tamanho_sobre_glyph_variant_preserva_glyph_e_factor`, `p812d_double_struck_conjuntos_numericos`, `p812c_scr_usa_bloco_script_nao_bold_script`, `p812c_variation_selectors_cal_scr`. Alterados para o observável medido: `roundhand_aliases_bold_script`, `p311b4_apply_math_style_size_variant_passthrough_glyph`.

```
Suite 'typst-core' (lib):  ANTES 4349 passed; 2 ignored → DEPOIS 4356 passed; 0 failed; 2 ignored (+7 ✓)
Suite 'typst-infra':       659 passed; 5 ignored (inalterado — nada tocou export)
Workspace: typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas
crystalline-lint . → exit 0
```
