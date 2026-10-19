---
# P785b — Campos nativos ausentes em tipos de valor (`relative length`, `alignment`, etc.)

> **Passo:** 785b
> **Data:** 2026-07-20
> **Foco:** P785 confirmou que `(10pt + 50%).ratio`/`.length` e `(top + left).x`/`.y` falham no cristalino com `"field access não suportado em <tipo>"`, enquanto o vanilla resolve normalmente. Também confirmou divergência de mensagem de erro para campo inválido: vanilla diz `"<tipo> does not contain field \"<nome>\""`, cristalino diz genericamente `"field access não suportado em <tipo>"`. Este passo implementa os campos nativos ausentes e corrige a mensagem de erro para o caso de campo desconhecido.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M/L — pode afetar múltiplos tipos (`RelativeLength`, `Alignment`, e outros a confirmar).
> **ADR-0108 EM VIGOR** — levantar a lista completa de tipos/campos afetados antes de corrigir só os dois exemplos medidos.
> **Dependências:** P785 (achado, evidência de mensagens de erro).

---

## Sonda — levantar todos os tipos com campos nativos no vanilla

```bash
grep -rn "fn fields\b\|missing_field\|does not contain field" lab/typst-original/crates/typst-library/src/foundations/fields.rs 2>/dev/null
```

Confirmar a lista completa de tipos com campos nativos (não só `RelativeLength`/`Alignment` — P785 mencionou também `Length`, `Stroke`, `Version` como candidatos na sua conclusão). Para cada tipo, listar os campos e o comportamento esperado.

```bash
grep -n "field access não suportado" 01_core/src/rules/eval/bindings.rs 01_core/src/entities/*.rs 2>/dev/null
```

Confirmar todos os pontos no cristalino onde esse erro genérico aparece — mapear contra a lista do vanilla para saber exatamente quais tipos precisam de campos novos.

### Mensagem de erro para campo desconhecido

```bash
cat > /tmp/p785b-invalid.typ <<'EOF'
#let x = (10pt).invalid
EOF
lab/typst-original/target/release/typst compile /tmp/p785b-invalid.typ 2>&1
```

Confirmar o formato exato: `"<tipo> does not contain field \"<nome>\""` — replicar para todos os tipos, não só `Length`.

---

## Implementação

1. Para cada tipo confirmado pela sonda (`RelativeLength`, `Alignment`, e outros da lista): implementar os campos nativos faltantes, com o comportamento correto (não só "não dar erro" — confirmar o valor retornado bate com o vanilla).
2. Substituir a mensagem genérica `"field access não suportado em <tipo>"` por `"<tipo> does not contain field \"<nome>\""` no braço de fallback (campo realmente desconhecido, não um dos implementados no passo 1).

---

## Validação

```bash
cat > /tmp/p785b-test.typ <<'EOF'
#let r = 10pt + 50%
#assert.eq(r.ratio, 50%)
#assert.eq(r.length, 10pt)
#let a = top + left
#assert.eq(a.x, left)
#assert.eq(a.y, top)
EOF
./target/release/typst compile /tmp/p785b-test.typ 2>&1
```

Confirmar exit 0, sem erro.

```bash
cat > /tmp/p785b-invalid.typ <<'EOF'
#let x = (10pt).invalid
EOF
./target/release/typst compile /tmp/p785b-invalid.typ 2>&1
```

Confirmar mensagem idêntica ao vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Lista completa de tipos/campos nativos levantada (não só os dois exemplos originais).
- [x] Campos implementados para todos os tipos confirmados.
- [x] Mensagem de erro de campo desconhecido corrigida para todos os tipos, não só `Length`.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p785b.md`.

---

## Próximo passo

P785c (offset UTF-16), se ainda não feito.
