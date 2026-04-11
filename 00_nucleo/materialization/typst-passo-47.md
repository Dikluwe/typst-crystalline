# Passo 47 — MathPrimes (Derivadas e Apóstrofos)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — arm `Expr::MathAttach` (alterado no Passo 46)
- `01_core/src/entities/content.rs` — `Content::MathAttach` com `tl`, `bl`, `sub`, `sup`
- `lab/typst-original/crates/typst-syntax/src/ast.rs` — `MathAttach::primes()`

Pré-condição: `cargo test` — 523 L1 + 83 L3 + 50 parity, zero violations.

---

## Contexto

A sintaxe `$x'$` e `$x''$` é mapeada pelo parser para um `MathAttach` onde o
método `primes()` retorna os tokens apóstrofo. O caractere tipográfico correcto
não é a aspa simples (`'`) mas o Prime Unicode:

| Count | Char | Unicode |
|-------|------|---------|
| 1 | ′ | U+2032 |
| 2 | ″ | U+2033 |
| 3 | ‴ | U+2034 |
| 4 | ⁗ | U+2057 |
| >4 | repetição de ′ | U+2032 × n |

Em termos de layout, primes são superscripts (`sup`). Se a expressão já tem
`sup` (ex: `$x'^2$`), os primes são concatenados antes do superscript
existente num `Content::MathSequence`.

A resolução é feita integralmente no `eval` — o `MathLayouter` não precisa
de saber o que é um prime; recebe apenas um `sup` já construído.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. Confirmar tipo de retorno de primes() e como contar os elementos
grep -A 8 "fn primes" \
  lab/typst-original/crates/typst-syntax/src/ast.rs

# 2. Confirmar estado actual do arm Expr::MathAttach após Passo 46
grep -A 20 "Expr::MathAttach" 01_core/src/rules/eval.rs

# 3. Confirmar construtores de Content::MathText e Content::MathSequence
grep -n "MathText\|MathSequence" 01_core/src/entities/content.rs | head -10

# 4. Confirmar helpers de teste disponíveis em L1 e L3
grep -n "fn layout_test\|fn compile_to_pdf" \
  01_core/src/rules/math/layout.rs \
  03_infra/src/integration_tests.rs | head -10

# 5. Confirmar como MathText é construído actualmente no eval
# (para usar o mesmo padrão ao criar o nó de prime)
grep -n "MathText\|MathIdent" 01_core/src/rules/eval.rs | head -10
```

**Reportar o output antes de continuar.**

Se `primes()` retornar um tipo opaco sem `count()`, usar `.into_iter().count()`
ou equivalente. Se o tipo de retorno diferir do esperado, adaptar a Tarefa 1
antes de codificar.

---

## Tarefa 1 — Extrair e converter primes no eval

Em `01_core/src/rules/eval.rs`, no arm `Expr::MathAttach`, após extrair
`tl`, `bl`, `sub` e `sup` (Passo 46), adicionar:

```rust
// Contar primes
let prime_count = attach.primes()
    .map(|p| p.count())   // ajustar conforme API confirmada no diagnóstico
    .unwrap_or(0);

// Converter count para caractere Unicode Prime
let prime_char: Option<Content> = if prime_count == 0 {
    None
} else {
    let s: EcoString = match prime_count {
        1 => "′".into(),   // U+2032
        2 => "″".into(),   // U+2033
        3 => "‴".into(),   // U+2034
        4 => "⁗".into(),   // U+2057
        n => "′".repeat(n).into(), // U+2032 × n para n > 4
    };
    Some(Content::MathText(s))
};
```

---

## Tarefa 2 — Merge com o superscript existente

Ainda no arm `Expr::MathAttach`, combinar `prime_char` com o `sup` extraído:

```rust
let sup_final: Option<Box<Content>> = match (prime_char, sup) {
    // Apenas primes, sem sup original
    (Some(p), None) => Some(Box::new(p)),

    // Apenas sup original, sem primes
    (None, Some(s)) => Some(s),

    // Ambos: primes primeiro, depois o sup original
    (Some(p), Some(s)) => Some(Box::new(Content::MathSequence(
        Arc::new([p, *s])
    ))),

    // Nenhum
    (None, None) => None,
};

Ok(Content::MathAttach {
    base: Box::new(base),
    tl,
    bl,
    sub,
    sup: sup_final,
})
```

**Nota**: o construtor de `MathSequence` deve seguir o padrão existente no
projecto — confirmar no diagnóstico se usa `Arc<[Content]>`, `Arc<Vec<Content>>`
ou outro. Adaptar antes de codificar se necessário.

Nenhuma alteração é necessária em `layout_attach` ou no `MathLayouter`.

---

## Tarefa 3 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_primes {
    use super::*;

    #[test]
    fn prime_simples_produz_conteudo() {
        let doc = layout_test("$x'$");
        let text = doc.plain_text();
        assert!(text.contains('x'), "base: {}", text);
        // ′ U+2032 deve estar presente como superscript
        assert!(text.contains('′'), "prime: {}", text);
    }

    #[test]
    fn double_prime_produz_conteudo() {
        let doc = layout_test("$x''$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('″'), "double prime: {}", text);
    }

    #[test]
    fn prime_com_sup_faz_merge() {
        // x'^2 — prime e superscript coexistem
        let doc = layout_test("$x'^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('′'), "prime: {}", text);
        assert!(text.contains('2'), "sup: {}", text);
    }

    #[test]
    fn prime_com_sub_e_sup() {
        // prime não interfere com subscript
        let doc = layout_test("$x'_i$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('′'));
        assert!(text.contains('i'));
    }

    #[test]
    fn sem_prime_nao_regride() {
        // Regressão: MathAttach sem primes não muda
        let doc = layout_test("$x^2_i$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
        assert!(text.contains('i'));
    }

    #[test]
    fn triple_prime_produz_conteudo() {
        let doc = layout_test("$f'''$");
        let text = doc.plain_text();
        assert!(text.contains('f'));
        assert!(text.contains('‴'), "triple prime: {}", text);
    }
}
```

### Testes em L3

```rust
#[cfg(test)]
mod tests_primes_pdf {

    #[test]
    fn pdf_prime_simples_nao_vazio() {
        let pdf = compile_to_pdf("$x'$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_double_prime_nao_vazio() {
        let pdf = compile_to_pdf("$f''(x)$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_prime_com_sup_nao_vazio() {
        let pdf = compile_to_pdf("$x'^2$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_prime_contem_bt_et() {
        let pdf = compile_to_pdf("$x'$");
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

- [ ] `attach.primes()` é chamado no eval e o count é extraído correctamente
- [ ] `prime_count` é mapeado para o caractere Unicode correcto (U+2032–U+2057)
- [ ] Prime sem `sup` existente: `sup_final = Some(MathText(prime_char))`
- [ ] Prime com `sup` existente: `sup_final = Some(MathSequence([prime, sup]))`
- [ ] Sem prime: comportamento idêntico ao Passo 46
- [ ] Nenhuma alteração em `layout_attach`, `MathLayouter`, ou L3 (excepto testes)
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- Tipo de retorno de `primes()` no AST original e como contar os elementos
- Se `MathSequence` usa `Arc<[Content]>` ou outro tipo de colecção
- Se `EcoString::repeat` existe ou foi necessária outra abordagem para n > 4

**Da implementação:**
- Se `plain_text()` inclui os caracteres prime nos testes (depende de como
  `MathText` é percorrido — confirmar que o teste de `prime_simples_produz_conteudo` passa)
- Se foi necessário qualquer ajuste na serialização de `MathSequence` para
  que `plain_text()` percorra os filhos

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 48:**
- **GO — Kern diferenciado por quadrante para left-scripts**: se primes funcionam, Passo 48 refina o kern de `tl` e `bl` independentes quando ambos coexistem (simplificação do Passo 46)
- **GO — MathAlignPoint**: suporte a `&` em equações alinhadas (`align` environment)
- **GO — Baselines em equações inline**: alinhar o eixo matemático com a linha de texto circundante quando `Equation { block: false }`
