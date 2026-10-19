---
# P737 — `counter`/`state` devem ser `type`, não `function`

> **Passo:** 737
> **Data:** 2026-07-10
> **Foco:** P731 confirmou que `counter`/`state` estão representados como `function` no cristalino, enquanto o vanilla os expõe como `type`. Mesma categoria de P736, mecanismo diferente (`function` chamável vs `type` chamável).
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P731 (onde a divergência foi confirmada), P685/P736 (mecanismo de tipo-como-valor-chamável).

---

## Sonda

### Confirmar o comportamento completo no vanilla

```bash
cat > /tmp/p737-counter-type.typ <<'EOF'
#type(counter)
#type(state)
#let c = counter("x")
#(type(c) == counter)
#let s = state("y", 0)
#(type(s) == state)
EOF
lab/typst-original/target/release/typst compile /tmp/p737-counter-type.typ /tmp/p737-vanilla.pdf
pdftotext /tmp/p737-vanilla.pdf -
```

Confirmar que `counter`/`state` são `type`, e que `type(instância) == counter` funciona (o valor devolvido por `counter("x")` tem o tipo `counter`, não outra coisa).

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p737-counter-type.typ /tmp/p737-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Comportamento completo confirmado, incluindo `type(instância) == counter`.

---

## Implementação

Converter `counter`/`state` de `Value::Func` para `Value::Type`, mantendo a chamabilidade (`counter("x")` continua a criar uma instância).

### Critério de fecho da implementação

- [ ] `type(counter)`/`type(state)` devolvem `type`.
- [ ] `counter("x")`/`state("y", 0)` continuam a funcionar, criando instâncias.
- [ ] `type(instância) == counter`/`== state` funciona.

---

## Validação

```bash
./target/release/typst /tmp/p737-counter-type.typ /tmp/p737-depois.pdf
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

Atenção a regressão silenciosa — `counter`/`state` são usados extensivamente (numeração de páginas, figuras, etc.).

---

## Critério de fecho do passo

- [ ] Sonda completa, comportamento confirmado.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`, atenção redobrada.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p737.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
