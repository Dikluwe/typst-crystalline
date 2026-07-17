# Diagnóstico P765b (lote 1) — Varredura da stdlib: módulo `typst_library::math::style`

**Data da medição:** 2026-07-15T17:00:00-03:00  
**Commit base:** `f36ca1abe2f95cffc2cf6fd2b3cf4947d2d9f846`  
**Working tree:** modificado (alterações de P765b e P766 em progresso)  
**Passo:** P765b (lote 1)  
**Objectivo:** Identificar o próximo módulo por tamanho dentro de `lacuna-inventario` (excluindo `foundations` e `diag`), classificar os seus itens e corrigir bugs reais comparando comportamento/saída com o vanilla.

---

## 1. Identificação do módulo

Lista `lacuna-inventario` (`00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt`), contagem por módulo (excluindo `foundations` e `diag`):

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn | head -10
```

| Posição | Módulo | Itens |
|---------|--------|-------|
| 1 | `typst_library::foundations::calc` | 45 (descartado — lacuna de granularidade) |
| 2 | `typst_library::math::style` | 32 |
| 3 | `typst_library::layout::grid::resolve` | 24 (infra-estrutura Rust) |
| 4 | `typst_library::foundations::ops` | 20 (descartado — `foundations`) |
| 5 | `typst_utils` | 14 (infra-estrutura Rust) |

**Módulo escolhido:** `typst_library::math::style` (32 itens).

---

## 2. Classificação dos 32 itens de `math::style`

| Item (TSV) | Estado no cristalino | Classificação |
|------------|----------------------|---------------|
| `MathSize` (enum) | Existe como `MathStyleKind` (com `Script`/`SScript`) | lacuna de granularidade / nomenclatura |
| `bb#1` / `bb#2` | Implementado (`native_bb`) | ✅ implementado |
| `cal#1` / `cal#2` | Implementado (`native_cal`) | ✅ implementado |
| `frak#1` / `frak#2` | Implementado (`native_frak`) | ✅ implementado |
| `mono#1` / `mono#2` | Implementado (`native_mono`) | ✅ implementado |
| `sans#1` / `sans#2` | Implementado (`native_sans`) | ✅ implementado |
| `scr#1` / `scr#2` | Implementado (`native_scr`) | ✅ implementado |
| `serif#1` / `serif#2` | Implementado (`native_serif`) | ✅ implementado |
| `upright#1` / `upright#2` | Implementado (`native_upright`) | ✅ implementado |
| `bold#1` / `bold#2` | Implementado (`native_bold`) | ✅ implementado |
| `italic#1` / `italic#2` | Implementado (`native_math_italic`) | ✅ implementado |
| `script#1` / `script#2` | Implementado, mas **rejeitava named arg `cramped`** | 🐛 bug real corrigido |
| `sscript#1` / `sscript#2` | Implementado, mas **rejeitava named arg `cramped`** | 🐛 bug real corrigido |
| `display#1` / `display#2` | **Não existia** no scope cristalino | 🐛 bug real corrigido |
| `inline#1` / `inline#2` | **Não existia** no scope cristalino | 🐛 bug real corrigido |
| `style_cramped` | Função interna de layout math do vanilla | infra-estrutura Rust, não símbolo de língua |
| `style_dtls` | Função interna de layout math do vanilla | infra-estrutura Rust, não símbolo de língua |
| `style_flac` | Função interna de layout math do vanilla | infra-estrutura Rust, não símbolo de língua |
| `style_for_denominator` | Função interna de layout math do vanilla | infra-estrutura Rust, não símbolo de língua |
| `style_for_numerator` | Função interna de layout math do vanilla | infra-estrutura Rust, não símbolo de língua |
| `style_for_subscript` | Função interna de layout math do vanilla | infra-estrutura Rust, não símbolo de língua |
| `style_for_superscript` | Função interna de layout math do vanilla | infra-estrutura Rust, não símbolo de língua |

> Nota: as funções `style_*` são internas do vanilla usadas pelo motor de layout math. Não fazem parte da API pública da língua Typst, pelo que a sua ausência no cristalino é aceitável enquanto o layout math completo não estiver implementado.

---

## 3. Bugs corrigidos

### 3.1 `script(..., cramped: ...)` e `sscript(..., cramped: ...)`

**Comportamento vanilla:** `script(body, cramped: true)` (default) e `script(body, cramped: false)` são válidos.

**Comportamento anterior cristalino:** rejeitava qualquer named arg com `argumento nomeado inesperado: 'cramped'`.

**Correção:** `wrap_math_style` em `01_core/src/engine/stdlib/math_style.rs` passou a aceitar o named arg `cramped` para `script`/`sscript`/`display`/`inline`, com default values conforme o vanilla (`true` para `script`/`sscript`, `false` para `display`/`inline`).

### 3.2 `display(...)` e `inline(...)`

**Comportamento vanilla:** `$display(x)$` e `$inline(x)$` forçam o tamanho da equação (`MathSize::Display`/`MathSize::Text`) e aceitam `cramped` named.

**Comportamento anterior cristalino:** `display` e `inline` não estavam registados; `$display(x)$` era interpretado como identificador/variável desconhecida.

**Correção:**
- Adicionados `MathStyleKind::Display` e `MathStyleKind::Inline` em `01_core/src/entities/math_style.rs` (variants de tamanho com factor 1.0).
- Adicionadas `native_display` e `native_inline` em `01_core/src/engine/stdlib/math_style.rs`.
- Registadas no root scope em `01_core/src/engine/eval/mod.rs`.
- Atualizado o L0 `00_nucleo/prompts/engine/stdlib/math_style.md`.

### 3.3 Limitação conhecida (não corrigida neste passo)

Named args em **math mode** (`$script("x", cramped: false)$`) ainda falham porque o eval math mode avalia os argumentos como conteúdo math e converte o resultado para `Value::Content`, em vez de `Value::Bool`. Fora de math mode (`#script("x", cramped: false)`) funciona corretamente, e os testes unitários confirmam a semântica. Esta limitação é do parser/eval math mode, não das funções `math_style`, e exige um passo dedicado ao math mode.

---

## 4. Validação

- `cargo test --workspace` — verde.
- `crystalline-lint .` — zero violações (exceto V7 esperado de `package_version_resolution.md`).
- Testes unitários novos em `01_core/src/engine/stdlib/mod.rs`:
  - `p765b_display_wraps_display_size`
  - `p765b_inline_wraps_inline_size`
  - `p765b_script_named_cramped_false`
  - `p765b_sscript_named_cramped_false`
  - `p765b_display_named_cramped_true`
  - `p765b_inline_named_cramped_true`
  - `p765b_bb_rejects_named_arg`
- Testes unitários em `01_core/src/entities/math_style.rs` para `Display`/`Inline` como size variants.

---

## 5. Conclusão

- O módulo `typst_library::math::style` foi inteiramente classificado.
- Foram corrigidos 4 bugs reais: named arg `cramped` em `script`/`sscript` e a ausência de `display`/`inline`.
- As funções internas `style_*` foram classificadas como infra-estrutura Rust, não dívidas de língua.
- O layout math completo continua scope-out; as correções focam na API de estilo acessível ao utilizador.
