# P783 — Relatório: Fallback de Fontes Matemáticas

**Data:** 2026-07-18  
**Commit de referência:** `0c5ea946e` (working tree limpa)  
**Medição de referência:** `git diff HEAD --stat` = 0 ficheiros alterados (após commit)

---

## 0. Achado original reconstruído (P772w §3.5)

Texto exato do `00_nucleo/diagnosticos/paridade-producao-p772w.md` linha 180–189:

> `families()` (vanilla) define a cadeia de fallback **específica de modo matemático**:
> `"new computer modern math"` → `"libertinus serif"` → fontes de emoji.
> `03_infra/src/fallback_fonts.rs` só tem cadeias genéricas serif/sans (P538e/P555),
> sem entrada específica para matemática. Impacto: quando a fonte de matemática
> primária não tem um glifo, o cristalino cai na cadeia serif/sans genérica em vez
> da cadeia matemática do vanilla — só manifesta em glifos ausentes da fonte primária
> (caso relativamente raro).

**Natureza do gap confirmada (ADR-0108):** não é a tabela MATH OpenType (já lida — P772y),
não é MathConstants (já lidas), não é espaçamento por classe (P772y). É exclusivamente
a **cadeia de prioridade de fallback de glifo** quando a fonte primária não cobre um
caractere matemático específico.

---

## 1. Mecanismo do vanilla (medido em source)

Ficheiro: `lab/typst-original/crates/typst-library/src/math/mod.rs:176`

```rust
pub fn families(styles: StyleChain<'_>) -> impl Iterator<Item = &'_ FontFamily> + Clone {
    let fallbacks = singleton!(Vec<FontFamily>, {
        [
            "new computer modern math",
            "libertinus serif",
            "twitter color emoji",
            "noto color emoji",
            "apple color emoji",
            "segoe ui emoji",
        ]
        .into_iter()
        .map(FontFamily::new)
        .collect()
    });

    let tail = if styles.get(TextElem::fallback) { fallbacks.as_slice() } else { &[] };
    styles.get_ref(TextElem::font).into_iter().chain(tail.iter())
}
```

**Decisão de linguagem:** a cadeia do vanilla é usada para **toda a resolução de fonte**
em modo math — não apenas quando a fonte declarada não existe, mas como lista de famílias
candidatas passada ao FontBook. No cristalino, o mecanismo é diferente (shaper + CandidateSet),
mas o efeito deve ser equivalente.

---

## 2. Estado antes de P783

- `fallback_fonts.rs`: só tinha `DEFAULT_FALLBACK_FONTS_SERIF` e `DEFAULT_FALLBACK_FONTS_SANS`
- Quando a fonte primária (ex: `Libertinus Serif`) não cobria um glifo matemático,
  o `CandidateSet::covering_all` percorria o FontBook na ordem de índice:
  1. Fontes texto embutidas (Libertinus Serif, NewCM10)
  2. Fontes do sistema (DejaVu Sans, Liberation, etc.)
  3. **Fontes math/code** (`New Computer Modern Math`, DejaVu Sans Mono)
  
  → As fontes do sistema (2) eram tentadas antes de `NewCMMath` (3), invertendo
  a prioridade esperada

- **Bug paralelo** em `embedded_fonts.rs`: `embedded_font_group()` usava
  `lower.starts_with("newcmmath")` para detectar `NewCMMath`, mas o nome de família
  OpenType real é `"New Computer Modern Math"` (não começa com `"newcmmath"`).
  O resultado final era correto por acidente (ramo `else` → `math_code`), mas
  a intenção estava errada e o teste `p754` verificava um predicado que nunca era verdadeiro.

---

## 3. Implementação

### 3.1 `fallback_fonts.rs` — nova constante e função

Adicionado `DEFAULT_FALLBACK_FONTS_MATH` espelhando a cadeia do vanilla:

```rust
pub(crate) const DEFAULT_FALLBACK_FONTS_MATH: &[&str] = &[
    "New Computer Modern Math",
    "Libertinus Serif",
    "Twitter Color Emoji",
    "Noto Color Emoji",
    "Apple Color Emoji",
    "Segoe UI Emoji",
];

pub(crate) fn math_fallback_font_list() -> &'static [&'static str] {
    DEFAULT_FALLBACK_FONTS_MATH
}
```

Testes adicionados: `p783_math_fallback_list_starts_with_new_computer_modern`,
`p783_math_fallback_list_contains_libertinus_serif`.

### 3.2 `embedded_fonts.rs` — correcção de heurística

`embedded_font_group()` corrigido:

```diff
- } else if lower.starts_with("newcmmath") || lower.starts_with("dejavu sans mono") {
+ } else if lower.contains("new computer modern math")
+        || lower.contains("new computer modern mono")
+        || lower.starts_with("dejavu sans mono") {
```

Teste `p754_newcm_math_is_not_in_text_group` corrigido para usar `contains("new computer modern math")`.

### 3.3 `shaper.rs` — fallback math como primárias adicionais

Em `shaped_width` e `try_shape`: após resolver as primárias com sucesso, verificar se
a primeira primária tem tabela MATH OpenType via `face_cache.get()`. Se sim, adicionar
`math_fallback_font_list()` como candidatas primárias adicionais.

**Efeito:** quando a fonte primária tem tabela MATH (ex: `Libertinus Serif`), as fontes
`New Computer Modern Math`, `Twitter Color Emoji`, etc. são adicionadas às primárias e
têm prioridade sobre o fallback global (todo o FontBook em ordem de índice). O `covering_run`
tenta primeiro as primárias — agora incluindo `NewCMMath` — antes do fallback lazy global.

---

## 4. Decisão de âmbito

**Implementado** — gap de escopo estreito, correcção cirúrgica em L3.

**Verificação visual** — o trace diff `mutool trace` não alterou porque o caso de teste
básico (`$ frac(a, b) $`, `$ x^2_1 $`) usa apenas glifos ASCII que `Libertinus Serif`
já cobre. O gap só manifesta com glifos matemáticos especiais ausentes na fonte primária.
A correcção está correcta mecanicamente mas não tem um caso de teste visual trivial sem
glifos especializados.

---

## 5. Validação

| Check | Resultado | Commit |
|-------|-----------|--------|
| `cargo build -p typst-infra` | ✅ zero errors | `0c5ea946e` |
| `cargo test --workspace` | ✅ 4259+649+... passed, 0 failed | `0c5ea946e` |
| `crystalline-lint .` | ✅ zero violations (V7 pré-existente) | `0c5ea946e` |
| Testes P783 (2 novos) | ✅ ok | `0c5ea946e` |
| Testes P555 (4 existentes) | ✅ ok | `0c5ea946e` |
| Testes P754 (corrigidos) | ✅ ok | `0c5ea946e` |

---

## 6. Critério de fecho

- [x] Achado original de P772w reconstruído com texto exato (§3.5, linha 180–189 do relatório).
- [x] Mecanismo exato do vanilla confirmado (`math/mod.rs:176`).
- [x] Decisão de âmbito registrada: implementado (gap estreito).
- [x] Bug paralelo em `embedded_fonts.rs` corrigido.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violations.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p783.md`.

---

## 7. Próximo passo

Com este fechado, todos os débitos conhecidos de P772w e seus desdobramentos
(P772x, P772y, P780, P781, P782, P783) estão endereçados. Momento natural para
um resumo final da série completa P765a–P783.
