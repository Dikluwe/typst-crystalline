# Passo 46 — Scripts à Esquerda em MathAttach (Tensores e Isótopos)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — variante `Content::MathAttach`
- `01_core/src/rules/eval.rs` — avaliação de `Expr::MathAttach`
- `01_core/src/rules/math/layout.rs` — função `layout_attach`
- `01_core/src/rules/layout.rs` — trait `FontMetrics`, método `math_kern()`
- `03_infra/src/font_metrics.rs` — leitura de `MathKernInfo` (implementada no Passo 44)

Pré-condição: `cargo test` — 518 L1 + 80 L3 + 50 parity, zero violations.

---

## Contexto

A notação tipográfica matemática suporta *pre-scripts* (índices e expoentes à
esquerda da base), usados em notação tensorial e isótopos
(ex: `_1^2 x` ou `{}^{14}_6 C`).

No Typst original, o nó `MathAttach` possui campos para `tl` (top-left) e
`bl` (bottom-left). No Cristalino, `Content::MathAttach` apenas tem `sub`
(bottom-right) e `sup` (top-right). Este passo adiciona os dois campos em
falta, actualiza o eval para os extrair do AST, e actualiza `layout_attach`
para posicionar a base com deslocamento horizontal quando existem scripts
à esquerda.

O kern dos quadrantes `top_left` e `bottom_left` (lido pela `FontBookMetrics`
no Passo 44 mas não usado) passa a ser aplicado neste passo.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. Confirmar API do AST original para os campos tl/bl de MathAttach
grep -n "fn tl\|fn bl\|fn top\|fn bottom" \
  lab/typst-original/crates/typst-syntax/src/ast.rs | head -10

# 2. Confirmar estrutura actual de MathAttach no Cristalino
grep -A 8 "MathAttach {" 01_core/src/entities/content.rs

# 3. Confirmar como o eval mapeia MathAttach actualmente
grep -A 15 "Expr::MathAttach" 01_core/src/rules/eval.rs

# 4. Confirmar assinatura actual de layout_attach
grep -n "fn layout_attach" 01_core/src/rules/math/layout.rs

# 5. Confirmar que math_kern está implementado e como é chamado em layout_attach
grep -n "math_kern\|top_left\|bottom_left\|top_right\|bottom_right" \
  01_core/src/rules/math/layout.rs | head -20

# 6. Confirmar helpers de teste disponíveis em L1 e L3
grep -n "fn layout_test\|fn compile_to_pdf\|fn compile_pdf_with_font" \
  01_core/src/rules/math/layout.rs \
  03_infra/src/integration_tests.rs | head -10
```

**Reportar o output antes de continuar.**

Se `attach.tl()` ou `attach.bl()` não existirem no AST original (alguns
ficheiros de sintaxe podem usar nomes diferentes), confirmar os nomes exactos
no diagnóstico antes de codificar a Tarefa 2.

---

## Tarefa 1 — Expandir Content::MathAttach em L1

Em `01_core/src/entities/content.rs`, alterar a variante `MathAttach` para
incluir os quadrantes esquerdos. Manter `Box<Content>` em todos os campos
opcionais para evitar recursão infinita.

```rust
MathAttach {
    base: Box<Content>,
    tl:   Option<Box<Content>>, // top-left (pre-superscript)
    bl:   Option<Box<Content>>, // bottom-left (pre-subscript)
    sub:  Option<Box<Content>>, // bottom-right (subscript)
    sup:  Option<Box<Content>>, // top-right (superscript)
},
```

Após esta alteração, todos os sítios que constroem `Content::MathAttach`
deixam de compilar até serem actualizados. Identificar esses sítios e
adicionar `tl: None, bl: None` onde ainda não existem (eval, testes).

---

## Tarefa 2 — Mapeamento no eval

Em `01_core/src/rules/eval.rs`, no arm `Expr::MathAttach`, extrair os
campos `tl` e `bl` do AST original.

```rust
Expr::MathAttach(attach) => {
    let base = eval_math_expr(scopes, ctx, attach.base())?;

    let tl  = attach.tl()
        .map(|e| eval_math_expr(scopes, ctx, e))
        .transpose()?
        .map(Box::new);
    let bl  = attach.bl()
        .map(|e| eval_math_expr(scopes, ctx, e))
        .transpose()?
        .map(Box::new);
    let sub = attach.bottom()
        .map(|e| eval_math_expr(scopes, ctx, e))
        .transpose()?
        .map(Box::new);
    let sup = attach.top()
        .map(|e| eval_math_expr(scopes, ctx, e))
        .transpose()?
        .map(Box::new);

    Ok(Content::MathAttach { base: Box::new(base), tl, bl, sub, sup })
}
```

Se o AST original usar nomes diferentes de `tl`/`bl` (confirmar no
diagnóstico), adaptar antes de codificar.

---

## Tarefa 3 — Geometria do layout_attach

Em `01_core/src/rules/math/layout.rs`, refactorizar `layout_attach` para
suportar scripts à esquerda. A função recebe agora os quatro campos opcionais.

### Lógica geométrica

**Passo 3a — Layout da base e dos left-scripts**:
Fazer o layout de `base`, `tl` e `bl` (se existirem) para obter as suas
métricas. As regras de posicionamento vertical dos left-scripts são as mesmas
dos right-scripts: `superscript_shift_up` para `tl`, `subscript_shift_down`
para `bl` (constantes já usadas no Passo 44).

**Passo 3b — Kern dos quadrantes esquerdos**:
Extrair `base_char` da base (padrão já estabelecido no Passo 44).

```rust
let base_kern = self.metrics.math_kern(base_char);

if let Some(tl_box) = &tl_box {
    let connection_height_du =
        tl_box.descent.val() * self.constants.upem / self.size.val();
    let kern_du = base_kern.top_left.kern_at(connection_height_du);
    tl_kern_pt = Pt(kern_du * self.size.val() / self.constants.upem);
}

if let Some(bl_box) = &bl_box {
    let connection_height_du =
        bl_box.ascent.val() * self.constants.upem / self.size.val();
    let kern_du = base_kern.bottom_left.kern_at(connection_height_du);
    bl_kern_pt = Pt(kern_du * self.size.val() / self.constants.upem);
}
```

**Passo 3c — Largura da coluna esquerda**:
A coluna esquerda ocupa `left_col_width`:

```
left_col_width = max(tl_width + |tl_kern_pt|, bl_width + |bl_kern_pt|)
```

Se não existirem `tl` nem `bl`, `left_col_width = Pt(0.0)`.

**Passo 3d — Posicionamento**:
- Base: posicionada em `x = left_col_width`.
- `tl` e `bl`: alinhados à direita da coluna esquerda (o seu `x` máximo é
  `left_col_width - kern_pt`). Os scripts colam à base com o kern a funcionar
  como gap negativo (aproximar) ou positivo (afastar).
- `sub` e `sup` (right-scripts): a lógica existente continua, mas as suas
  posições x partem de `left_col_width + base_width` em vez de `base_width`.

**Simplificação permitida**: se `tl` e `bl` existirem simultaneamente,
usar `left_col_width = max(tl_width, bl_width)` sem kern diferenciado por
quadrante — kern diferenciado por quadrante fica para Passo 47+.

---

## Tarefa 4 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_left_scripts {
    use super::*;

    #[test]
    fn attach_sem_left_scripts_nao_regride() {
        // Regressão: MathAttach sem tl/bl comporta-se como antes
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
    }

    #[test]
    fn attach_left_sup_contem_base_e_script() {
        // Pre-superscript: conteúdo do script e da base presentes
        let doc = layout_test("${}^2 x$");
        let text = doc.plain_text();
        assert!(text.contains('2'), "pre-sup: {}", text);
        assert!(text.contains('x'), "base: {}", text);
    }

    #[test]
    fn attach_left_sub_contem_base_e_script() {
        // Pre-subscript
        let doc = layout_test("${}_1 x$");
        let text = doc.plain_text();
        assert!(text.contains('1'), "pre-sub: {}", text);
        assert!(text.contains('x'), "base: {}", text);
    }

    #[test]
    fn attach_left_e_right_juntos() {
        // Scripts nos dois lados simultaneamente
        let doc = layout_test("${}_1^2 x_3^4$");
        let text = doc.plain_text();
        assert!(text.contains('1'));
        assert!(text.contains('2'));
        assert!(text.contains('x'));
        assert!(text.contains('3'));
        assert!(text.contains('4'));
    }

    #[test]
    fn attach_sem_base_explicitita_usa_empty() {
        // Base vazia ({}): não deve panicar
        let doc = layout_test("${}^{14}$");
        assert!(!doc.pages.is_empty());
    }
}
```

### Testes em L3

```rust
#[cfg(test)]
mod tests_left_scripts_pdf {

    #[test]
    fn pdf_pre_scripts_nao_vazio() {
        let pdf = compile_to_pdf("${}^2 x$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_pre_scripts_dos_lados_nao_vazio() {
        let pdf = compile_to_pdf("${}_1^2 x_3^4$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_pre_scripts_contem_bt_et() {
        let pdf = compile_to_pdf("${}^2 x$");
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("BT"));
        assert!(s.contains("ET"));
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

- [ ] `Content::MathAttach` inclui `tl` e `bl` (ambos `Option<Box<Content>>`)
- [ ] Todos os sítios que constroem `MathAttach` compilam com `tl: None, bl: None`
- [ ] O eval extrai `tl` e `bl` do AST quando presentes
- [ ] `layout_attach` calcula `left_col_width` e desloca a base para a direita
- [ ] `math_kern` é consultado para os quadrantes `top_left` e `bottom_left`
- [ ] Right-scripts (`sub`, `sup`) continuam correctos após o refactor
- [ ] Com `FixedMetrics`, kern é zero — comportamento previsível
- [ ] Todos os testes de regressão de attach/frac/sqrt/delimited passam
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- Nomes exactos dos métodos `tl`/`bl` no AST original (podem diferir)
- Se `layout_attach` já recebia `base_char` ou foi necessário extraí-lo de novo
- Se `left_col_width` com ambos os scripts simultâneos produziu layout visualmente correcto

**Da implementação:**
- Se a simplificação (kern sem diferenciação por quadrante quando ambos existem) foi usada
- Se foi necessário mudar a assinatura de `layout_attach` ou apenas o corpo

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 47:**
- **GO — Kern diferenciado por quadrante**: se os left-scripts funcionam, Passo 47 refina o kern para `tl` e `bl` independentes quando ambos existem
- **GO — MathPrimes**: suporte a `x'` e `x''` (prime/double-prime) como syntactic sugar para superscript
- **NO-GO — AST sem tl/bl**: se o AST original não tiver estes campos, o âmbito muda para emulação via `MathSequence`
