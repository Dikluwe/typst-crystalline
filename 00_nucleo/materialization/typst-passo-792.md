---
# P792 — Context/layout eval: `#layout`, `text.lang`, `here().position()`

> **Passo:** 792
> **Data:** 2026-07-20
> **Foco:** P786 confirmou duas divergências relacionadas a `#context`: (1) `#layout` como função (usada para medir dimensões disponíveis, ex: `layout(size => ...)`) é `unknown variable` no cristalino, quando o vanilla resolve normalmente; (2) `#context text.lang` e `#context here().position()` falham com `cannot access fields on type function`/`cannot access fields on type location` no cristalino, enquanto o vanilla resolve (`"lang=en"`, `"page=1"`). Também há divergência de wording no erro de `require`/contexto fora de escopo, a confirmar se está relacionada.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M/L — pode envolver mecanismos distintos (função `layout` ausente vs campos de tipo `function`/`location` ausentes).
> **ADR-0108 EM VIGOR** — confirmar se as três divergências têm a mesma causa raiz ou são independentes.
> **Prioridade:** Alta — funcionalidade central de `#context` (usada para responsividade de layout e introspecção de posição) ausente/quebrada.
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/b_context.typ`, `b_context_here.typ`, `b_layout.typ`, `b_context_err.typ`).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "fn layout\b" lab/typst-original/crates/typst-library/src/layout/measure.rs 2>/dev/null
grep -n "fn field\b" lab/typst-original/crates/typst-library/src/foundations/fields.rs 2>/dev/null | grep -i "func\|location"
```

Confirmar:
1. `layout()` é uma função nativa do escopo global, disponível só dentro de `#context`? Confirmar assinatura e comportamento (recebe closure, devolve resultado com dimensões).
2. `text.lang` — é campo do "estado de estilo actual" acessível via alguma função especial (`text` como pseudo-objeto dentro de `#context`, não literalmente o tipo função)? Confirmar mecanismo exato — o erro `cannot access fields on type function` sugere que `text` está sendo resolvido como a função `text()` em vez de um objeto de estilo contextual.
3. `here().position()` — `here()` retorna `Location`; `.position()` deve ser um campo/método nativo desse tipo. Confirmar se `Location` tem campos nativos ausentes no cristalino (mesma categoria do achado de P785, `foundations::fields`).

```bash
cat > /tmp/p792-layout.typ <<'EOF'
#context layout(size => [W=#size.width])
EOF
lab/typst-original/target/release/typst compile /tmp/p792-layout.typ 2>&1

cat > /tmp/p792-textlang.typ <<'EOF'
#context [lang=#text.lang]
EOF
lab/typst-original/target/release/typst compile /tmp/p792-textlang.typ 2>&1

cat > /tmp/p792-position.typ <<'EOF'
#context [page=#here().position().page]
EOF
lab/typst-original/target/release/typst compile /tmp/p792-position.typ 2>&1
```

### Estado atual do cristalino

```bash
grep -n "fn.*layout\b\|\"lang\"\|Location" 01_core/src/rules/stdlib/*.rs 01_core/src/entities/*.rs 2>/dev/null | head -30
```

---

## Decisão de âmbito

Se as três causas forem distintas: confirmar se cabem juntas neste passo ou se alguma precisa de passo dedicado (ex: se `layout()` como função de medição exigir um mecanismo de layout condicional/two-pass que não existe hoje).

---

## Implementação

Conforme a sonda revelar:
1. `layout()`: implementar a função nativa, disponível dentro de `#context`.
2. `text.lang`: confirmar se é campo de pseudo-objeto de estilo contextual, implementar o acesso correto (não confundir com a função `text()`).
3. `here().position()`: adicionar campos nativos a `Location` (mesma disciplina de P785b — levantar todos os campos, não só `.page`).

---

## Validação

```bash
./target/release/typst compile /tmp/p792-layout.typ 2>&1
./target/release/typst compile /tmp/p792-textlang.typ 2>&1
./target/release/typst compile /tmp/p792-position.typ 2>&1
```

Confirmar exit 0 e valores corretos nos três casos.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Mecanismo exato do vanilla confirmado para os três casos.
- [x] Confirmado se as três causas são independentes ou compartilham raiz.
- [x] `layout()` funciona dentro de `#context`.
- [x] `text.lang` (e campos irmãos de estilo contextual, se existirem) funcionam.
- [x] `here().position()` e campos nativos de `Location` funcionam.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p792.md`.

---

## Próximo passo

Numbering, smartquote, math/symbol scope, sys.version/CLI, ênfase/fontes — restam da lista de P786 §5.
