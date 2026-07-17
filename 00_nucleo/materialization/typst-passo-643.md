---
# P643 — Escape unicode inválido preservado silenciosamente

> **Passo:** 643
> **Data:** 2026-07-09
> **Foco:** P633 confirmou dois casos (1, 2): um escape unicode inválido, como `\u{FFFFFFFF}`, em code string ou em markup, é descartado e o texto literal original é preservado, em vez de produzir erro. P638 confirmou que o vanilla já dá erro claro nos dois casos ("invalid Unicode codepoint: FFFFFFFF"). Este passo corrige os dois.
> **Tipo:** Implementação directa. Causa, localização, e mensagem do vanilla já confirmadas por P633/P638.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P633 (casos confirmados, com testes já escritos), P638 (mensagem de erro do vanilla já extraída por teste directo).

---

## Contexto

Dois locais, com o mesmo tipo de problema:

| # | Local | Contexto |
|---|---|---|
| 1 | `01_core/src/entities/ast/expr.rs:382` | Code string (`"\u{FFFFFFFF}"`) |
| 2 | `01_core/src/entities/ast/markup.rs:107` | Markup (`[\u{FFFFFFFF}]`) |

Os dois usam `u32::from_str_radix(sequence, 16).ok().and_then(std::char::from_u32)` — quando o valor não corresponde a um codepoint Unicode válido, `from_u32` devolve `None`, `.ok()` já tinha descartado qualquer erro de parsing antes disso, e o resultado final é o texto literal original preservado, sem aviso.

Mensagem do vanilla, já confirmada por P638: `invalid Unicode codepoint: FFFFFFFF`.

---

## Sonda mínima

### Confirmar se a correcção pertence ao parser/lexer ou ao eval

```bash
grep -n "fn.*escape\|SyntaxKind::Escape" 01_core/src/engine/lexer/*.rs 01_core/src/engine/parse/*.rs 2>/dev/null | head -10
```

P634 já tinha notado, ao investigar `#let x = 0xZZ`, que "propagar todos os erros de parser no entrypoint expôs 10 regressões" e que correcções deste tipo pertencem a um passo dedicado ao parser/lexer. Confirmar se este caso (escape unicode inválido) é da mesma natureza, ou se pode ser corrigido de forma isolada, sem o mesmo risco.

### Critério de fecho da sonda mínima

- [ ] Confirmado se a correcção pode ser feita isoladamente (só nestes dois locais), ou se arrasta o mesmo risco de regressão já visto em P634 para erros de parser em geral.

---

## Implementação

Se a sonda confirmar que é seguro corrigir isoladamente: alterar os dois locais para produzir erro quando `from_u32` devolve `None`, com a mensagem já confirmada pelo vanilla.

```rust
// Esboço, a confirmar contra a estrutura real:
let codepoint = u32::from_str_radix(sequence, 16)
    .ok()
    .and_then(std::char::from_u32)
    .ok_or_else(|| error!(span, "invalid Unicode codepoint: {}", sequence.to_uppercase()))?;
```

Se a sonda confirmar o mesmo risco de P634 (regressões em smart quotes, `#set` dentro de blocos, etc., já mencionadas no relatório de P634 como motivo para não avançar com a correcção de `0xZZ`): não implementar aqui, e escrever esse passo dedicado ao parser/lexer como P634 já tinha recomendado, cobrindo os dois problemas (código inválido em geral, não só escape unicode) de uma vez.

### Critério de fecho da implementação

- [ ] Escape unicode inválido em code string produz erro.
- [ ] Escape unicode inválido em markup produz erro.
- [ ] Mensagem igual à do vanilla, já confirmada.
- [ ] Testes de P633 (`p633_invalid_unicode_escape_preserved`, `p633_invalid_unicode_escape_markup_preserved`) invertidos.
- [ ] Escapes válidos sem regressão.

---

## Validação

```bash
cat > /tmp/p643-invalido.typ <<'EOF'
"\u{FFFFFFFF}"
EOF
./target/release/typst /tmp/p643-invalido.typ /tmp/p643.pdf
echo "Exit code: $?"
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa — confirmado se é seguro corrigir isoladamente.
- [ ] Se seguro: os dois casos corrigidos, testes invertidos.
- [ ] Se não seguro: passo dedicado ao parser/lexer escrito, cobrindo isto e o caso de P634 (`0xZZ`) juntos.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p643.md`, com hash do commit.

---

## Próximo passo

Casos 5, 6 (bibliografia) — os últimos dois da lista de P633.
