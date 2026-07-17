# Relatório Diagnóstico — Passo 585
## Verificação directa da existência do teste `p581_cobertura_de_escape_e_shorthand_em_layout`

---

## 1. Estado do repositório

```bash
git rev-parse HEAD
```

```text
6490d575228d45dd52d9f864edd26104883d8f88
```

```bash
git status --short
```

```text
?? 00_nucleo/materialization/typst-passo-585.md
```

---

## 2. Dimensão do ficheiro

```bash
wc -l 01_core/src/engine/layout/tests.rs
```

```text
14962 01_core/src/engine/layout/tests.rs
```

---

## 3. Conteúdo directo do ficheiro (linhas 3480–3510)

```bash
sed -n '3480,3510p' 01_core/src/engine/layout/tests.rs
```

```rust
                texts[idx]
            );
        }
    }

    /// Passo 581 — Garante que caracteres de escape e shorthands em markup
    /// sejam avaliados e renderizados correctamente no layout, sem desaparecer.
    #[test]
    fn p581_cobertura_de_escape_e_shorthand_em_layout() {
        let doc = layout_typst("A \\# B \\$ C -- D ... E");
        let items: Vec<_> = doc.pages[0]
            .items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect();
        let joined = items.join(" ");
        assert!(joined.contains("#"), "deve conter '#' do escape; obtido: '{}'", joined);
        assert!(joined.contains("$"), "deve conter '$' do escape; obtido: '{}'", joined);
        assert!(joined.contains("–"), "deve conter '–' do shorthand; obtido: '{}'", joined);
        assert!(joined.contains("…"), "deve conter '…' do shorthand; obtido: '{}'", joined);
    }
}


// ── Passo 103.D: Integração `#show` end-to-end ────────────────────────────

#[cfg(test)]
mod tests_show_rule_integration {
```

---

## 4. Busca pelo nome do teste

```bash
grep -n "p581_cobertura_de_escape_e_shorthand_em_layout\|fn.*escape.*shorthand\|fn.*shorthand.*escape" 01_core/src/engine/layout/tests.rs
```

```text
3488:    fn p581_cobertura_de_escape_e_shorthand_em_layout() {
```

---

## 5. Execução directa do teste pelo nome exacto

Primeira tentativa com o nome simples:

```bash
cargo test -p typst-core p581_cobertura_de_escape_e_shorthand_em_layout -- --exact 2>&1
```

```text
warning: variable `cal_L` should have a snake case name
   --> 01_core/src/engine/math/layout/tests.rs:611:9
    |
611 |         let cal_L = Content::math_styled(Some(MathStyleKind::Chancery), None, None, Content::math_ident("L".into()), None);
    |         ^^^^^

warning: `typst-core` (lib test) generated 458 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/typst_core-897d4467d73ed8da)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3570 filtered out; finished in 0.00s
```

Segunda tentativa com o path completo do módulo:

```bash
cargo test -p typst-core rules::layout::tests::tests_set_rule_integration::p581_cobertura_de_escape_e_shorthand_em_layout -- --exact 2>&1
```

```text
warning: variable `cal_L` should have a snake case name
   --> 01_core/src/engine/math/layout/tests.rs:611:9
    |
611 |         let cal_L = Content::math_styled(Some(MathStyleKind::Chancery), None, None, Content::math_ident("L".into()), None);
    |         ^^^^^

warning: `typst-core` (lib test) generated 458 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/typst_core-897d4467d73ed8da)

running 1 test
test rules::layout::tests::tests_set_rule_integration::p581_cobertura_de_escape_e_shorthand_em_layout ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3569 filtered out; finished in 0.00s
```

---

## 6. Conclusão

**O teste existe, no ficheiro `01_core/src/engine/layout/tests.rs`, linha 3488.** Corre e passa quando invocado pelo path completo do módulo (`rules::layout::tests::tests_set_rule_integration::p581_cobertura_de_escape_e_shorthand_em_layout`).
