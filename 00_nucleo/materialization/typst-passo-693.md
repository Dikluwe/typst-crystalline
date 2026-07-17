---
# P693 — `str.match()` deve aceitar `str`, não só `regex`

> **Passo:** 693
> **Data:** 2026-07-10
> **Foco:** P692 encontrou, como débito, que `match()` singular (P689) só aceita `regex`, enquanto o vanilla aceita `str | regex`. É a mesma classe de violação já corrigida várias vezes nesta cadeia (nome igual, aridade/tipos aceites diferentes). O próprio `matches()` (plural, P692) já implementa o padrão `str | regex` correctamente — este passo aplica o mesmo a `match()`.
> **Tipo:** Implementação directa. Pequeno, padrão já estabelecido.
> **Tamanho:** XS-S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P689 (`match()` original), P692 (`matches()`, já com o padrão correcto a copiar).

---

## Sonda mínima

### Confirmar o comportamento exacto de `match()` com string

```bash
cat > /tmp/p693-match-str.typ <<'EOF'
#("abcabc").match("bc")
#("abc").match("z")
EOF
lab/typst-original/target/release/typst compile /tmp/p693-match-str.typ /tmp/p693-vanilla.pdf
pdftotext /tmp/p693-vanilla.pdf -
```

Confirmar: com `str`, `captures` fica vazio (como acontece em `matches()` com string, já confirmado por P692)?

### Critério de fecho da sonda mínima

- [ ] Comportamento de `match(str)` confirmado contra o vanilla.

---

## Implementação

Alargar `str_match` (`01_core/src/rules/stdlib/collections.rs`) para aceitar `Value::Str` além de `Value::Regex`, seguindo exactamente o mesmo padrão já usado em `str_matches` (P692) — provavelmente reaproveitando directamente a lógica de resolução de padrão já lá construída.

### Critério de fecho da implementação

- [ ] `match(str)` funciona, devolvendo o mesmo tipo de dict já usado para `match(regex)`.
- [ ] `match(regex)` sem regressão.
- [ ] `matches()` (P692) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p693-match-str.typ /tmp/p693-depois.pdf
pdftotext /tmp/p693-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa.
- [ ] `match()` aceita `str | regex`, testado contra o vanilla.
- [ ] `matches()` e `match(regex)` sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p693.md`, com hash do commit.
- [ ] Com este passo, a cadeia de correcções de `str` (P689-P693) fica declarada fechada, com a lista completa de métodos oficiais implementados e sem nenhuma variação de assinatura conhecida por corrigir.
