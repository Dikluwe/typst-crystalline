---
# P723 — Namespace de `curve` (`curve.move`, `.line`, `.cubic`, `.close`)

> **Passo:** 723
> **Data:** 2026-07-10
> **Foco:** P720/P722 confirmaram que `cetz` usa `curve.move(...)`/`curve.line(...)`/`curve.cubic(...)`/`curve.close(...)` (`canvas.typ:151,156,159,166`), mas o `Value::Func` nativo `curve` não tem namespace registado no cristalino (`bindings.rs:1396`). Este é o bloqueio actual e único da reprodução de `cetz`.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S se o mecanismo de namespace de `Func` (já usado por `int`/`str`/`table`, etc.) só precisar de registar as entradas; M se `curve` precisar de construtores próprios ainda não implementados.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P720 (onde o bloqueio foi isolado com `file:line` exacto de `cetz`).

---

## Sonda

### Confirmar a lista completa de funções do namespace `curve` no vanilla

```bash
grep -n "impl.*curve\|pub fn move\|pub fn line\|pub fn cubic\|pub fn close\|pub fn quad" lab/typst-original/crates/typst-library/src/visualize/curve.rs 2>/dev/null | head -20
```

Não assumir que é só `move`/`line`/`cubic`/`close` — confirmar a lista completa (pode haver `quad`, por exemplo).

### Confirmar o comportamento de cada função

```bash
cat > /tmp/p723-curve.typ <<'EOF'
#curve(
  curve.move((0pt, 0pt)),
  curve.line((10pt, 10pt)),
  curve.cubic((15pt, 5pt), (20pt, 15pt), (25pt, 0pt)),
  curve.close(),
)
EOF
lab/typst-original/target/release/typst compile /tmp/p723-curve.typ /tmp/p723-vanilla.pdf
mutool draw -o /tmp/p723-vanilla.png -r 150 /tmp/p723-vanilla.pdf
```

Confirmar que produz um elemento de desenho visual, e o tipo de valor devolvido por cada função do namespace (provavelmente uma variante interna de "segmento de curva").

### Confirmar exactamente o que `cetz` usa em `canvas.typ`

```bash
grep -n "curve\." ~/.cache/typst/packages/preview/cetz/0.5.2/src/canvas.typ
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p723-curve.typ /tmp/p723-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Lista completa de funções do namespace `curve` confirmada.
- [ ] Comportamento de cada uma confirmado, incluindo o tipo de valor devolvido.
- [ ] Uso real em `cetz` confirmado.

---

## Implementação

Registar o namespace de `curve`, seguindo o mecanismo já usado para outros `Value::Func` nativos com namespace (P493b e sucessores).

### Critério de fecho da implementação

- [ ] Funções usadas por `cetz` implementadas e testadas.
- [ ] **[scope-out]** Qualquer função do namespace sem consumidor confirmado, com razão medida.

---

## Validação

```bash
./target/release/typst /tmp/p723-curve.typ /tmp/p723-depois.pdf
mutool draw -o /tmp/p723-depois.png -r 150 /tmp/p723-depois.pdf
```

Comparar visualmente com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Campos fixos de progresso — este pode ser o passo que fecha `cetz`

```bash
cat > /tmp/p723-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p723-cetz.typ /tmp/p723-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p723-cetz.png -r 150 /tmp/p723-cetz.pdf 2>/dev/null
lab/typst-original/target/release/typst compile /tmp/p723-cetz.typ /tmp/p723-cetz-vanilla.pdf
mutool draw -o /tmp/p723-cetz-vanilla.png -r 150 /tmp/p723-cetz-vanilla.pdf
```

Se produzir PDF: comparar com diff de pixels contra o PNG do vanilla, não só inspecção visual — esta é a validação final de toda a cadeia P678-723, se este for de facto o último bloqueio.

Se ainda falhar: registar o próximo bloqueio com os campos fixos habituais.

---

## Critério de fecho do passo

- [ ] Sonda completa, lista e comportamento confirmados.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — se for sucesso completo, diff de pixels contra o vanilla como prova final; se não, campos fixos de progresso registados.
- [ ] Grep às ADRs em vigor pelos termos centrais (`curve`, namespace) antes de fechar o texto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p723.md`, com hash do commit.
