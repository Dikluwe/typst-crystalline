---
# P736 — `color`/`gradient` devem ser `type`, não `dictionary`

> **Passo:** 736
> **Data:** 2026-07-10
> **Foco:** P731 confirmou que `color`/`gradient` estão representados como `Value::Dict` no cristalino, enquanto o vanilla os expõe como `type` (o mesmo mecanismo de "tipo como valor" já implementado em P685 para `int`/`str`/`length`/etc.). Sem consumidor conhecido em `cetz`, mas é uma inconsistência de representação com o resto do sistema de tipos já construído.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M — `color`/`gradient` têm construtores associados (`rgb`, `luma`, `linear-gradient`, etc.) que precisam de continuar acessíveis.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P731 (onde a divergência foi confirmada), P685 (mecanismo de tipo-como-valor-chamável já estabelecido).

---

## Sonda

### Confirmar o comportamento completo no vanilla

```bash
cat > /tmp/p736-color-type.typ <<'EOF'
#type(color)
#type(gradient)
#(type(red) == color)
#color.rgb(255, 0, 0)
#color.linear-rgb(50%, 50%, 50%)
#gradient.linear(red, blue)
EOF
lab/typst-original/target/release/typst compile /tmp/p736-color-type.typ /tmp/p736-vanilla.pdf
pdftotext /tmp/p736-color-type.typ /tmp/p736-vanilla.pdf
```

Confirmar que `color`/`gradient` são `type`, que `type(x) == color` funciona para comparar, e que os construtores associados (`color.rgb`, `color.linear-rgb`, `gradient.linear`, etc.) continuam acessíveis através do valor-tipo — mesmo padrão de `int("5")` funcionar apesar de `int` ser um tipo.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p736-color-type.typ /tmp/p736-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Comportamento completo confirmado, incluindo construtores associados acessíveis através do tipo.

---

## Implementação

Converter `color`/`gradient` de `Value::Dict` para `Value::Type`, seguindo o mecanismo já estabelecido em P685, garantindo que os construtores associados continuam acessíveis (mesmo padrão de `int`/`str` chamáveis apesar de serem tipos).

### Critério de fecho da implementação

- [ ] `type(color)`/`type(gradient)` devolvem `type`.
- [ ] `type(x) == color` funciona.
- [ ] Construtores associados (`color.rgb`, etc.) continuam a funcionar, sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p736-color-type.typ /tmp/p736-depois.pdf
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

Dado que `color`/`gradient` são usados extensivamente em toda a stdlib (qualquer coisa com `fill:`/`stroke:`), atenção redobrada a regressão silenciosa.

---

## Critério de fecho do passo

- [ ] Sonda completa, comportamento confirmado.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`, atenção redobrada dado o alcance.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p736.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
