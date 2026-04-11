# Passo 50 — Diferenciação de Limites por Contexto (Inline vs Display)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/math/layout.rs` — `MathLayouter` (struct com estado), `layout_attach`
- `01_core/src/entities/content.rs` — `Content::Equation { body, block }`
- `01_core/src/rules/layout.rs` — sítio onde `MathLayouter` é instanciado a partir de `Content::Equation` (Passo 48)

Pré-condição: `cargo test` — 553 L1 + 96 L3 + 50 parity, zero violations.

---

## Contexto

No Passo 49, operadores grandes (`∑`, `∏`, `lim`, …) passaram a empilhar os
limites verticalmente, sempre. A regra tipográfica correcta é:

- Equação de bloco (`block == true`, `$ ... $` com espaços): limites verticais (Under/Over).
- Equação inline (`block == false`, `$...$`): limites à direita como sub/sup normais,
  para não expandir verticalmente a linha de texto.

Este passo propaga `block` até `layout_attach` e condiciona o empilhamento vertical.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. Campos actuais da struct MathLayouter
grep -A 10 "pub struct MathLayouter\|struct MathLayouter" \
  01_core/src/rules/math/layout.rs

# 2. Como MathLayouter é instanciado a partir de Content::Equation
# (onde block está acessível — Passo 48)
grep -n "MathLayouter::new\|MathLayouter {" \
  01_core/src/rules/layout.rs \
  01_core/src/rules/math/layout.rs | head -10

# 3. Assinatura actual de layout_attach (após Passo 46 adicionou tl/bl)
grep -A 3 "fn layout_attach" 01_core/src/rules/math/layout.rs

# 4. Como layout_attach é chamado dentro de layout_math_content
grep -n "layout_attach\|self\.layout_attach" \
  01_core/src/rules/math/layout.rs | head -10

# 5. Confirmar que is_large_operator e is_limit_function estão acessíveis
grep -n "is_large_operator\|is_limit_function" \
  01_core/src/rules/math/layout.rs | head -5
```

**Reportar o output antes de continuar.**

---

## Tarefa 1 — Adicionar `block` à struct MathLayouter (L1)

`MathLayouter` é uma struct com estado (tem `metrics`, `constants`, `size`).
A abordagem correcta é adicionar `block: bool` como campo da struct —
não propagar por parâmetro por toda a cadeia de chamadas.

```rust
// Em MathLayouter:
pub struct MathLayouter<M: FontMetrics> {
    // ... campos existentes ...
    /// True se a equação é de bloco (display mode); false se inline.
    pub block: bool,
}
```

No sítio de instanciação (onde `Content::Equation { body, block }` é processado):

```rust
let layouter = MathLayouter {
    // ... campos existentes ...
    block,
};
```

Nenhuma outra função além de `layout_attach` precisa de ler `self.block`
neste passo.

---

## Tarefa 2 — Condicionar empilhamento vertical em layout_attach (L1)

Em `layout_attach`, a condição introduzida no Passo 49 passa a incluir `self.block`:

```rust
// Antes (Passo 49):
let use_vertical_limits = is_large_operator(base_char)
    || is_limit_function(base_str);

// Depois (Passo 50):
let use_vertical_limits = (is_large_operator(base_char)
    || is_limit_function(base_str))
    && self.block;
```

Se `use_vertical_limits == false`, o fluxo cai na ramificação de
right-scripts existente (sub/sup à direita) — sem alterações adicionais.

---

## Tarefa 3 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_limits_context {
    use super::*;

    #[test]
    fn sum_inline_contem_conteudo() {
        // Em modo inline, ∑ com limites não deve panicar e deve ter conteúdo
        let doc = layout_test("$sum_(i=0)^n$");
        let text = doc.plain_text();
        // Base e pelo menos um limite devem estar presentes
        assert!(
            text.contains('∑') || text.contains('i') || text.contains('n'),
            "conteúdo ausente: {}", text
        );
    }

    #[test]
    fn sum_inline_gera_pagina() {
        let doc = layout_test("$sum_(i=0)^n x_i$");
        assert!(!doc.pages.is_empty());
        assert!(!doc.pages[0].items.is_empty());
    }

    #[test]
    fn lim_inline_contem_conteudo() {
        let doc = layout_test("$lim_(x -> 0) f(x)$");
        let text = doc.plain_text();
        assert!(text.contains('f') || text.contains('x'),
            "conteúdo ausente: {}", text);
    }

    #[test]
    fn attach_normal_inline_nao_regride() {
        // x^2 inline — não é operador grande, não afectado
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
    }

    #[test]
    fn attach_normal_com_sub_inline_nao_regride() {
        let doc = layout_test("$x_i$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('i'));
    }

    // Regressão: testes do Passo 49 sobre empilhamento vertical ainda devem passar
    // (layout_test usa block=true por omissão — confirmar no diagnóstico)
    #[test]
    fn sum_block_contem_conteudo() {
        // Confirmar que block=true continua a funcionar após a alteração
        let doc = layout_test("$sum_(i=0)^n$");
        let text = doc.plain_text();
        assert!(
            text.contains('∑') || text.contains('i') || text.contains('n'),
            "conteúdo ausente em block: {}", text
        );
    }
}
```

**Nota sobre `layout_test`**: confirmar no diagnóstico se `layout_test` usa
`block = true` ou `block = false` por omissão. Se usar `block = false`, os
testes de regressão do Passo 49 (que testavam empilhamento vertical) precisam
de ser actualizados ou de usar um helper alternativo com `block = true`.

### Testes em L3

```rust
#[cfg(test)]
mod tests_limits_context_pdf {

    #[test]
    fn pdf_sum_inline_no_texto_nao_vazio() {
        // Equação inline com operador grande — limites à direita
        let pdf = compile_to_pdf("Soma $sum_(i=0)^n x_i$ no texto.");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_sum_inline_contem_bt_et() {
        let pdf = compile_to_pdf("$sum_(i=0)^n$");
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("BT"));
        assert!(s.contains("ET"));
    }

    #[test]
    fn pdf_lim_inline_nao_vazio() {
        let pdf = compile_to_pdf("O limite $lim_(x -> 0) f(x)$ existe.");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_attach_normal_nao_regride() {
        let pdf = compile_to_pdf("$x^2 + y_i$");
        assert!(!pdf.is_empty());
    }
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:

- [ ] `MathLayouter` tem campo `block: bool`
- [ ] `block` é passado na instanciação a partir de `Content::Equation { block, .. }`
- [ ] `layout_attach` usa `self.block` na condição `use_vertical_limits`
- [ ] Com `block == false`: operadores grandes usam right-scripts (sub/sup à direita)
- [ ] Com `block == true`: operadores grandes usam empilhamento vertical (Passo 49)
- [ ] Bases não-operador (`x^2`, `x_i`): comportamento inalterado em ambos os modos
- [ ] Todos os testes de regressão dos Passos 49, 47, 46, 44 passam
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- Campos actuais de `MathLayouter` (para confirmar que `block` não colide com nenhum campo existente)
- Se `layout_test` usa `block = true` ou `block = false` por omissão (impacta os testes de regressão do Passo 49)
- Profundidade da cadeia `layout_math_content` → `layout_attach` (para confirmar que `self.block` é acessível sem passar por parâmetro intermédio)

**Da implementação:**
- Se foi necessário actualizar testes do Passo 49 após confirmar o valor de `block` em `layout_test`
- Se a alteração de uma linha na condição foi suficiente ou se houve efeitos secundários

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 51:**
- **GO — MathAlignPoint**: se a diferenciação inline/bloco funciona, Passo 51 implementa `&` em equações alinhadas (`align` environment)
- **GO — Kern diferenciado por quadrante para left-scripts**: refinamento da simplificação do Passo 46
- **NO-GO — `layout_test` usa `block = false`**: se os testes de regressão do Passo 49 falharam porque `layout_test` cria equações inline, é necessário criar um helper `layout_test_block` antes de avançar
