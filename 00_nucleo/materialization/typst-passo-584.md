---
# P584 — Criar o teste de escape/shorthand/linebreak que P581 afirmou existir

> **Passo:** 584
> **Data:** 2026-07-05
> **Foco:** P581 afirmou que a correcção de `Expr::Escape`, `Expr::Shorthand`, e `Expr::Linebreak` foi validada por um teste unitário chamado `p581_cobertura_de_escape_e_shorthand_em_layout`. P583 confirmou que esse teste não existe no código. A correcção em si pode estar certa — foi confirmada indirectamente pela extracção de texto em P581 — mas não tem nenhum teste automático a protegê-la de regressão futura. Este passo cria o teste que devia já existir.
> **Tipo:** Implementação de teste.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Um teste referido num relatório como existente e a passar tem de existir de facto — confirmar antes de aceitar.
> **Dependências:** P581 (onde a correcção foi feita, e onde a afirmação sobre o teste não corresponde ao código), P583 (onde a ausência do teste foi confirmada).

---

## Contexto

`Expr::Escape`, `Expr::Shorthand`, e `Expr::Linebreak` (em contexto de markup) foram corrigidos em P581 para deixarem de cair no braço genérico `_ => Ok(Value::None)`. A correcção parece funcionar, pela extracção de texto manual feita em P581. Mas não há teste automático — se alguém, sem saber disto, reintroduzir o mesmo bug (por exemplo, ao mexer no `match` de `eval_expr`), nada vai falhar para avisar.

---

## Implementação

### Teste unitário directo

```rust
#[test]
fn p584_escape_shorthand_linebreak_em_markup_preservados() {
    // Escape: \# \$ \& \* \\
    let content = eval_markup_str(r"\# \$ \& \* \\").unwrap();
    assert_eq!(content.plain_text(), "# $ & * \\");

    // Shorthand: -- (en-dash), --- (em-dash), ... (ellipsis)
    let content = eval_markup_str("a -- b --- c ... d").unwrap();
    // confirmar que os shorthands não desaparecem; o valor exacto do
    // carácter Unicode depende da tabela de shorthands já usada no
    // resto do projecto, confirmar contra ela, não inventar aqui.

    // Linebreak em markup: \
    let content = eval_markup_str("linha um \\ linha dois").unwrap();
    assert!(content.plain_text().contains("linha um"));
    assert!(content.plain_text().contains("linha dois"));
}
```

Ajustar `eval_markup_str` (ou o helper equivalente já usado nos testes existentes de `eval/mod.rs`) para o padrão real do módulo.

### Teste de regressão a nível de documento completo

```bash
cat > /tmp/p584-escape.typ <<'EOF'
Preço: \$50 \# \& \* \\
Intervalo: 10--20
Reticências: e assim por diante...
EOF
./target/release/typst /tmp/p584-escape.typ /tmp/p584.pdf
pdftotext /tmp/p584.pdf -
```

Confirmar visualmente que nada desaparece.

### Critério de fecho

- [ ] Teste unitário `p584_escape_shorthand_linebreak_em_markup_preservados` (ou nome equivalente) criado e a passar.
- [ ] Teste de documento completo confirmado com `pdftotext`.
- [ ] Teste adicionado ao corpus de paridade (`lab/parity/corpus/`), já que P583 confirmou que nenhum dos 90 ficheiros existentes cobre isto.

---

## Critério de fecho do passo

- [ ] Teste unitário criado, a passar, e confirmado por leitura directa do ficheiro — não por afirmação.
- [ ] Documento adicionado ao corpus de paridade.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p584.md`, com o nome exacto do teste e o ficheiro onde fica, confirmável por quem ler o relatório sem ter de confiar na palavra escrita.
