---
# P716 — `Access` genérico: atribuição a `dict.campo` e `arr.at(i)`

> **Passo:** 716
> **Data:** 2026-07-10
> **Foco:** P715 confirmou que `cetz` usa `arr.at(i) = valor` (mutação de elemento de array via método acessor) e `dict.campo = valor` (mutação de campo) como alvos de atribuição — confirmado em `hobby.typ`, `drawable.typ`. O mecanismo actual só suporta `Expr::Ident` como alvo de atribuição/desestruturação (P715); estas formas precisam de um mecanismo de referência mutável genérico (`Access`, como no vanilla), não substituição do valor completo.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M-L.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P715 (onde o mecanismo de mutação de `Ident` foi construído, e este scope-out identificado com `file:line` de `cetz`).

---

## Sonda

### Confirmar o mecanismo `Access` completo do vanilla

```bash
grep -n "fn access\|enum Access\|is_accessor_method" lab/typst-original/crates/typst-eval/src/access.rs | head -30
```

Confirmar a lista exacta de "accessor methods" reconhecidos (provavelmente `.at()` para array/dict, possivelmente outros) — não assumir que é só `.at()`.

### Confirmar o comportamento exacto com documentos reais

```bash
cat > /tmp/p716-access.typ <<'EOF'
#let d = (a: 1, b: 2)
#{ d.a = 10 }
#d

#let arr = (1, 2, 3)
#{ arr.at(1) = 20 }
#arr

#let arr2 = (1, 2, 3)
#{ arr2.at(5) = 20 }
EOF
lab/typst-original/target/release/typst compile /tmp/p716-access.typ /tmp/p716-vanilla.pdf
pdftotext /tmp/p716-vanilla.pdf -
```

Confirmar o erro exacto para índice fora de limites em atribuição, e se `.at(i, default:)` é aceite como alvo de atribuição (o `default:` não faz sentido nesse contexto — confirmar o que o vanilla faz).

### Confirmar exactamente onde `cetz` usa cada forma

```bash
grep -n "\.at(.*) *=\|^\s*[a-z_.]*\.[a-z_]* *=" ~/.cache/typst/packages/preview/cetz/0.5.2/src/hobby.typ ~/.cache/typst/packages/preview/cetz/0.5.2/src/drawable.typ
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p716-access.typ /tmp/p716-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Lista completa de accessor methods confirmada.
- [ ] Comportamento exacto de `dict.campo = valor` e `arr.at(i) = valor` confirmado, incluindo casos de erro.
- [ ] Uso real em `cetz` confirmado com `file:line`.

---

## Implementação

Estender `eval_assign`/`eval_destruct_assignment` (P715) para reconhecer `FieldAccess` (campo de dict) e `FuncCall` de métodos acessores (`.at()`) como alvos válidos, obtendo uma referência mutável ao elemento/campo em vez de reatribuir o valor completo.

### Critério de fecho da implementação

- [ ] `dict.campo = valor` funciona, mutando o dict original.
- [ ] `arr.at(i) = valor` funciona, mutando o array original.
- [ ] Índice fora de limites em atribuição produz erro, igual ao vanilla.
- [ ] Mecanismo de `Ident` (P715) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p716-access.typ /tmp/p716-depois.pdf
pdftotext /tmp/p716-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Repetir a reprodução de `cetz` (P700-715)

```bash
cat > /tmp/p716-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p716-cetz.typ /tmp/p716-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p716-cetz.png -r 150 /tmp/p716-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita por P688. Se não: registar o próximo bloqueio, usando o tempo de compilação como sinal de progresso.

---

## Critério de fecho do passo

- [ ] Sonda completa, mecanismo `Access` confirmado com a lista completa de accessor methods.
- [ ] Implementado e testado, incluindo casos de erro.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p716.md`, com hash do commit.
