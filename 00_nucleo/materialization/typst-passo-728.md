---
# P728 — `and`/`or` sem short-circuit (bug de mecanismo central)

> **Passo:** 728
> **Data:** 2026-07-10
> **Foco:** P727 encontrou, durante a validação final de `cetz`, que `and`/`or` avaliam sempre os dois operandos antes de despachar (`eval/mod.rs:687-692`), em vez de parar assim que o resultado já está decidido. Caso mínimo confirmado, sem cetz: `type(a) == str and a.contains(".")` com `a` sendo um array — o vanilla dá `false` sem erro (nunca chega a avaliar `.contains`); o cristalino erra "campo desconhecido". Isto não é uma função em falta — é semântica fundamental da linguagem, usada no idioma comum "verificar antes de aceder". P727 notou ainda uma anomalia dependente da ordem de avaliação, com suspeita de memoização a esconder o erro nalguns casos — investigar isso junto.
> **Tipo:** Sonda + Implementação. Prioridade máxima — mecanismo central, alcance potencialmente amplo.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mudança em mecanismo central de avaliação; sonda obrigatória e cuidado redobrado.
> **Dependências:** P727 (onde o bug foi encontrado e isolado com reprodução mínima).

---

## Sonda

### Confirmar o comportamento completo do vanilla

```bash
cat > /tmp/p728-shortcircuit.typ <<'EOF'
#let a = (1, 2)
#(type(a) == str and a.contains("."))
#(type(a) == array or a.contains("."))
#(false and (1/0 == 0))
#(true or (1/0 == 0))
EOF
lab/typst-original/target/release/typst compile /tmp/p728-shortcircuit.typ /tmp/p728-vanilla.pdf
pdftotext /tmp/p728-vanilla.pdf -
```

Confirmar: `false and X` nunca avalia `X` (mesmo que `X` fosse um erro, como divisão por zero); `true or X` nunca avalia `X`. Confirmar também o caso inverso (`true and X`, `false or X` — aqui X tem de ser avaliado).

### Localizar o mecanismo exacto do vanilla

```bash
grep -n "BinOp::And\|BinOp::Or\|short.circuit\|lazy" lab/typst-original/crates/typst-eval/src/*.rs | head -20
```

### Confirmar o estado actual do cristalino, com `file:line`

```bash
grep -n "BinOp::And\|BinOp::Or" 01_core/src/rules/eval/mod.rs
```

### Investigar a anomalia de ordem/memoização notada por P727

```bash
cat > /tmp/p728-anomalia.typ <<'EOF'
#let f(a) = { type(a) == str and a.contains(".") }
#f((0, 0))
#f((0, 0))
EOF
./target/release/typst /tmp/p728-anomalia.typ /tmp/p728-anomalia-1.pdf 2>&1
echo "---"
cat > /tmp/p728-anomalia2.typ <<'EOF'
#let f(a) = { type(a) == str and a.contains(".") }
#f((1, 1))
#f((0, 0))
EOF
./target/release/typst /tmp/p728-anomalia2.typ /tmp/p728-anomalia-2.pdf 2>&1
```

Confirmar se chamar a mesma expressão duas vezes, ou com argumentos diferentes em sequência, produz resultados inconsistentes — se sim, confirmar se `#[comemo::memoize]` está envolvido em algum destes caminhos, e se está a cachear com uma chave que não distingue os casos correctamente.

### Critério de fecho da sonda

- [ ] Comportamento completo de short-circuit confirmado (`and`/`or`, casos onde o segundo operando não devia ser avaliado).
- [ ] Mecanismo do vanilla confirmado.
- [ ] Anomalia de ordem/memoização investigada e explicada, não só reproduzida.

---

## Implementação

Implementar avaliação lazy (short-circuit) para `and`/`or` — o segundo operando só é avaliado se o primeiro não já decidir o resultado. Isto provavelmente exige tratar `and`/`or` fora do despacho genérico de `Expr::Binary` (avaliar os dois operandos primeiro), com um caminho próprio que avalia o primeiro, decide, e só avalia o segundo se necessário.

Se a anomalia de memoização for confirmada como um bug real e relacionado: corrigir também, com a mesma prioridade.

### Critério de fecho da implementação

- [ ] `and`/`or` fazem short-circuit correctamente, testado com os casos que dependem disso para não errar.
- [ ] Anomalia de memoização corrigida, se confirmada como bug real.
- [ ] `and`/`or` com os dois operandos válidos (caso comum) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p728-shortcircuit.typ /tmp/p728-depois.pdf
pdftotext /tmp/p728-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

Dado o alcance potencial (qualquer documento com o idioma "verificar antes de aceder"), correr o corpus de testes já existente com atenção a regressão silenciosa.

### Reprodução final de `cetz` — este é provavelmente o último bloqueio

```bash
cat > /tmp/p728-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p728-cetz.typ /tmp/p728-cetz.pdf
mutool draw -o /tmp/p728-cetz.png -r 150 /tmp/p728-cetz.pdf
lab/typst-original/target/release/typst compile /tmp/p728-cetz.typ /tmp/p728-cetz-vanilla.pdf
mutool draw -o /tmp/p728-cetz-vanilla.png -r 150 /tmp/p728-cetz-vanilla.pdf
```

Diff de pixels entre os dois — se aproximar de zero (dentro de margem de anti-aliasing), a cadeia P678-728 fecha de vez.

---

## Critério de fecho do passo

- [ ] Sonda completa, mecanismo confirmado, anomalia investigada e explicada.
- [ ] Short-circuit implementado e testado.
- [ ] Anomalia de memoização corrigida, se confirmada.
- [ ] Sem regressão em `cargo test --workspace`, atenção a regressão silenciosa.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — diff de pixels final registado com número exacto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p728.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
- [ ] Se a cadeia P678-728 fechar de vez: resumo completo da cadeia (número de passos, principais categorias de bugs corrigidos) no relatório.
