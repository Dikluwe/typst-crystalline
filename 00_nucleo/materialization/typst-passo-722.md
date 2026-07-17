---
# P722 — `Array * Int` (repetição)

> **Passo:** 722
> **Data:** 2026-07-10
> **Foco:** P720 encontrou, durante a sonda de `+`, que `(0,) * (n - 1)` (repetição de array) também falha ("cannot apply Mul to array and int"), nas mesmas linhas de `hobby.typ` (77, 78) que motivaram a correcção de `+`. Explicitamente não corrigido nesse passo por ser um operador distinto — este passo fecha-o.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P720 (onde foi encontrado e conscientemente adiado, com `file:line`).

---

## Sonda

### Confirmar o comportamento exacto no vanilla

```bash
cat > /tmp/p722-repeat.typ <<'EOF'
#((0,) * 3)
#(3 * (0,))
#((1, 2) * 0)
#((1, 2) * -1)
#((:) * 2)
EOF
lab/typst-original/target/release/typst compile /tmp/p722-repeat.typ /tmp/p722-vanilla.pdf
pdftotext /tmp/p722-vanilla.pdf -
```

Confirmar: `Int * Array` funciona (ordem inversa), ou só `Array * Int`? Contagem zero e negativa — erro, ou array vazio? `Dict * Int` existe?

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p722-repeat.typ /tmp/p722-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Comportamento completo confirmado (ordem, zero, negativo, `Dict * Int`).

---

## Implementação

Implementar `Array * Int`/`Int * Array` (repetição), seguindo o comportamento confirmado pela sonda.

### Critério de fecho da implementação

- [ ] Implementado e testado, incluindo casos de borda (zero, negativo).
- [ ] **[scope-out]** `Dict * Int`, se confirmado inexistente no vanilla ou sem consumidor.

---

## Validação

```bash
./target/release/typst /tmp/p722-repeat.typ /tmp/p722-depois.pdf
pdftotext /tmp/p722-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Campos fixos de progresso

```bash
cat > /tmp/p722-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p722-cetz.typ /tmp/p722-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p722-cetz.png -r 150 /tmp/p722-cetz.pdf 2>/dev/null
```

Registar: tempo de compilação, exit code, próximo bloqueio com `file:line` exacto — ou, se produzir PDF, diff de pixels com o vanilla.

---

## Critério de fecho do passo

- [ ] Sonda completa, comportamento confirmado.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — campos fixos de progresso registados.
- [ ] Grep às ADRs em vigor pelos termos centrais (`Mul`, repetição) antes de fechar o texto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p722.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
