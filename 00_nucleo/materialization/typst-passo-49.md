# Passo 49 — Limites Verticais em Operadores Grandes (Under/Over)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/math/layout.rs` — `layout_attach`, `apply_axis_offset`, `offset_item`
- `01_core/src/rules/math/symbols.rs` — funções `is_*` existentes
- `01_core/src/entities/math_constants.rs` — `MathConstants`, campos actuais
- `01_core/src/rules/layout.rs` — `layout_content`, onde `block` está acessível (Passo 48)
- `03_infra/src/font_metrics.rs` — leitura de `math_table.constants`

Pré-condição: `cargo test` — 536 L1 + 91 L3 + 50 parity, zero violations.

---

## Contexto

`layout_attach` coloca sempre `sup` no topo-direito e `sub` no fundo-direito.
Para operadores grandes (∑, ∏, ∫, ⋃, …) em modo bloco, os limites devem ser
empilhados **verticalmente**: centrados acima e abaixo da base, não à direita.

A tabela OpenType MATH define constantes específicas para este espaçamento:
`upper_limit_gap_min` e `lower_limit_gap_min`. O Passo 44 já lê outras
constantes com o mesmo padrão — este passo segue o mesmo.

**Decisão arquitectural sobre `block`**: `layout_attach` não recebe `block`
directamente. Em vez de propagar `block` pela cadeia de chamadas, este passo
usa a heurística: aplicar limites verticais sempre que a base for um operador
grande, independentemente de `block`. Esta simplificação é consistente com o
Typst original em modo display. A diferenciação por `block` fica para Passo 50+.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. Nomes exactos das constantes de limite na tabela MATH do ttf-parser
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "upper_limit\|lower_limit\|LimitGap\|limit_gap" {} | head -20

# 2. Confirmar campos actuais de MathConstants em L1
cat 01_core/src/entities/math_constants.rs

# 3. Como math_table.constants expõe as constantes de limite em L3
grep -n "upper_limit\|lower_limit\|constants\." 03_infra/src/font_metrics.rs | head -20

# 4. Funções is_* existentes em symbols.rs
grep -n "pub fn is_" 01_core/src/rules/math/symbols.rs

# 5. Como base_char é extraído actualmente em layout_attach (Passo 44)
grep -n "base_char\|base_content\|MathIdent\|MathText" \
  01_core/src/rules/math/layout.rs | head -15

# 6. Assinatura actual de layout_attach após Passo 46
grep -n "fn layout_attach" 01_core/src/rules/math/layout.rs
```

**Reportar o output antes de continuar.**

Se `upper_limit_gap_min` / `lower_limit_gap_min` não existirem com esses
nomes no `ttf-parser`, reportar os nomes exactos encontrados e adaptar a
Tarefa 2 antes de codificar.

---

## Tarefa 1 — is_large_operator em L1

Em `01_core/src/rules/math/symbols.rs`, adicionar função que identifica se
um caractere é um operador grande que aceita limites verticais.

```rust
/// Retorna true se o caractere é um operador grande que deve receber
/// limites (sup/sub) empilhados verticalmente em vez de à direita.
pub fn is_large_operator(c: char) -> bool {
    matches!(c,
        // Somatório, produto, coproduto
        '∑' | '∏' | '∐' |
        // União, intersecção e variantes
        '⋃' | '⋂' | '⨄' | '⨅' | '⨆' |
        // Integrais
        '∫' | '∬' | '∭' | '∮' | '∯' | '∰' |
        // Outros operadores grandes comuns
        '⨁' | '⨂' | '⨀' | '⋀' | '⋁'
    )
}
```

Adicionar também suporte a texto para funções como `lim`, `max`, `min`:

```rust
/// Retorna true se o texto base de um MathIdent aceita limites verticais.
pub fn is_limit_function(s: &str) -> bool {
    matches!(s, "lim" | "max" | "min" | "sup" | "inf" | "limsup" | "liminf")
}
```

---

## Tarefa 2 — Constantes de limite em MathConstants (L1 e L3)

### L1 — adicionar campos a MathConstants

Em `01_core/src/entities/math_constants.rs`, adicionar os dois campos com
valores de fallback razoáveis:

```rust
pub struct MathConstants {
    // ... campos existentes ...

    /// Espaço mínimo entre a base e o limite superior (design units).
    /// Fallback: 100.0 (≈ 0.1 × upem típico de 1000).
    pub upper_limit_gap_min: f64,

    /// Espaço mínimo entre a base e o limite inferior (design units).
    /// Fallback: 100.0
    pub lower_limit_gap_min: f64,
}

impl MathConstants {
    pub fn fallback() -> Self {
        Self {
            // ... valores existentes ...
            upper_limit_gap_min: 100.0,
            lower_limit_gap_min: 100.0,
        }
    }
}
```

### L3 — ler da tabela MATH

Em `03_infra/src/font_metrics.rs`, no sítio onde as constantes MATH são
lidas (padrão do Passo 41), adicionar:

```rust
// Nomes exactos a confirmar no diagnóstico 1:
upper_limit_gap_min: constants.upper_limit_gap_min()
    .map(|v| v.value as f64)
    .unwrap_or(100.0),
lower_limit_gap_min: constants.lower_limit_gap_min()
    .map(|v| v.value as f64)
    .unwrap_or(100.0),
```

---

## Tarefa 3 — Empilhamento vertical em layout_attach (L1)

Em `01_core/src/rules/math/layout.rs`, modificar `layout_attach` para
detectar operadores grandes e ramificar para empilhamento vertical.

### Detecção da base

```rust
// Extrair char/str da base para identificar o tipo de operador
// (padrão já estabelecido no Passo 44 para base_char)
let is_limits = match base_content {
    Content::MathText(s) | Content::MathIdent(s) => {
        let ch = s.chars().next().unwrap_or('\0');
        is_large_operator(ch) || is_limit_function(s.as_str())
    }
    _ => false,
};
```

### Lógica de empilhamento vertical (quando `is_limits == true`)

```rust
if is_limits {
    // 1. Layout da base, sup e sub
    let base_box = self.layout_math_content(base_content);
    let sup_box = sup_content.map(|c| self.layout_math_content(c));
    let sub_box = sub_content.map(|c| self.layout_math_content(c));

    // 2. Largura total = max das três larguras
    let total_width = [
        base_box.width,
        sup_box.as_ref().map(|b| b.width).unwrap_or(Pt(0.0)),
        sub_box.as_ref().map(|b| b.width).unwrap_or(Pt(0.0)),
    ]
    .into_iter()
    .fold(Pt(0.0), |a, b| if b.0 > a.0 { b } else { a });

    // 3. Gaps em pt (du → pt)
    let upper_gap = self.size * Pt(self.constants.upper_limit_gap_min / self.constants.upem);
    let lower_gap = self.size * Pt(self.constants.lower_limit_gap_min / self.constants.upem);

    // 4. Posicionamento vertical:
    //   sup fica acima da base: y_sup = -(base_ascent + upper_gap + sup_descent)
    //   sub fica abaixo da base: y_sub = base_descent + lower_gap + sub_ascent
    //
    //   Centrado em X: x = (total_width - element_width) / 2.0

    // 5. Retornar MathBox com todos os items, ascent e descent totais
    // (padrão de hconcat/vstack já estabelecido — criar vstack se não existir,
    // ou compor os MathBoxes manualmente com offset_item)
}
// else: lógica existente de layout horizontal (right-scripts)
```

**Nota sobre `tl`/`bl`**: operadores grandes raramente têm left-scripts.
Se existirem, aplicar o left-script offset normal (Passo 46) antes do
empilhamento vertical — não é necessário tratar o caso combinado neste passo.

---

## Tarefa 4 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_limits {
    use super::*;

    #[test]
    fn is_large_operator_reconhece_sum() {
        assert!(is_large_operator('∑'));
    }

    #[test]
    fn is_large_operator_reconhece_integral() {
        assert!(is_large_operator('∫'));
    }

    #[test]
    fn is_large_operator_nao_reconhece_x() {
        assert!(!is_large_operator('x'));
    }

    #[test]
    fn is_limit_function_reconhece_lim() {
        assert!(is_limit_function("lim"));
    }

    #[test]
    fn is_limit_function_nao_reconhece_sin() {
        assert!(!is_limit_function("sin"));
    }

    #[test]
    fn layout_sum_com_limites_contem_conteudo() {
        // sum_(i=0)^n — base e limites presentes no output
        let doc = layout_test("$sum_(i=0)^n$");
        let text = doc.plain_text();
        assert!(text.contains('∑') || text.contains('i') || text.contains('n'),
            "operador ou limites ausentes: {}", text);
    }

    #[test]
    fn layout_sum_sem_limites_nao_regride() {
        let doc = layout_test("$sum$");
        let text = doc.plain_text();
        assert!(text.contains('∑'), "somatório: {}", text);
    }

    #[test]
    fn layout_attach_normal_nao_regride() {
        // x^2 não é operador grande — layout horizontal mantido
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
    }

    #[test]
    fn layout_integral_com_limites_nao_panica() {
        let doc = layout_test("$integral_(0)^1 f(x)$");
        assert!(!doc.pages.is_empty());
    }
}
```

### Testes em L3

```rust
#[cfg(test)]
mod tests_limits_pdf {

    #[test]
    fn pdf_sum_com_limites_nao_vazio() {
        let pdf = compile_to_pdf("$sum_(i=0)^n x_i$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_prod_com_limites_nao_vazio() {
        let pdf = compile_to_pdf("$prod_(k=1)^n a_k$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_lim_com_limite_nao_vazio() {
        let pdf = compile_to_pdf("$lim_(x -> 0) f(x)$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_integral_com_limites_nao_vazio() {
        let pdf = compile_to_pdf("$integral_(0)^1 f(x) dif x$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_attach_normal_nao_regride() {
        // x^2 não é operador grande — não deve ser afectado
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

- [ ] `is_large_operator(c: char) -> bool` existe em `symbols.rs`
- [ ] `is_limit_function(s: &str) -> bool` existe em `symbols.rs`
- [ ] `upper_limit_gap_min` e `lower_limit_gap_min` existem em `MathConstants` com fallback `100.0`
- [ ] L3 lê estes campos da tabela MATH (nomes confirmados no diagnóstico)
- [ ] `layout_attach` detecta operadores grandes e ramifica para empilhamento vertical
- [ ] Empilhamento vertical centra os três elementos pelo eixo X
- [ ] Espaçamento vertical usa as constantes `upper/lower_limit_gap_min`
- [ ] Right-scripts em bases não-operador continuam inalterados
- [ ] `lim`, `max`, `min` com subscript usam empilhamento vertical
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- Nomes exactos das constantes de limite no `ttf-parser` (podem diferir de `upper_limit_gap_min`)
- Se `is_limit_function` foi suficiente para capturar `lim`/`max`/`min`, ou se o eval já os trata de forma diferente
- Se foi necessário criar um método `vstack` novo ou se `offset_item` foi suficiente para compor verticalmente

**Da implementação:**
- Se o empilhamento vertical mudou visualmente o layout de `∑` (esperado: sim)
- Se `tl`/`bl` em operadores grandes causou algum problema (caso de teste não coberto)
- Se a simplificação "limites verticais sempre para operadores grandes" causou
  algum caso inesperado em equações inline

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 50:**
- **GO — Diferenciação por `block`**: se limites verticais funcionam, Passo 50 adiciona o parâmetro `block` a `layout_attach` para aplicar limites horizontais em equações inline e verticais em bloco
- **GO — MathAlignPoint**: suporte a `&` em equações alinhadas (`align` environment)
- **NO-GO — `vstack` ausente**: se a composição vertical não for possível com os métodos actuais, implementar `vstack` antes de avançar para Passo 50
