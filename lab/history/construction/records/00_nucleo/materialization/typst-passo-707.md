---
# P707 — `Arguments` com métodos (`.pos()`, `.named()`, ...)

> **Passo:** 707
> **Data:** 2026-07-10
> **Foco:** P706 isolou que `args.pos()` falha no cristalino ("campo desconhecido em arguments: 'pos'"), porque `Value::Args` expõe `positional`/`named` como campos (P504), enquanto o vanilla usa `.pos()`/`.named()` como métodos. Confirmar a assinatura completa de `Arguments` no vanilla antes de assumir que só falta `.pos()` — a mesma lição já repetida várias vezes nesta cadeia (P706 encontrou um gap maior do que P705 assumia).
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P706 (onde o bloqueio foi isolado com `file:line` de `cetz`).

---

## Sonda

### Confirmar a assinatura completa de `Arguments` no vanilla

```bash
grep -n "impl.*Args\|pub fn " lab/typst-original/crates/typst-library/src/foundations/args.rs | head -40
```

Não assumir que só falta `.pos()` — listar todos os métodos públicos, e testar cada um.

```bash
cat > /tmp/p707-args.typ <<'EOF'
#let f(..args) = (
  args.pos(),
  args.named(),
  args.pairs(),
)
#f(1, 2, x: 3, y: 4)
EOF
lab/typst-original/target/release/typst compile /tmp/p707-args.typ /tmp/p707-vanilla.pdf
pdftotext /tmp/p707-vanilla.pdf -
```

Ajustar a lista de métodos testados conforme o que o `grep` acima revelar (pode haver mais além de `pos`/`named`/`pairs` — confirmar `at`, `len`, outros).

### Confirmar exactamente onde `cetz` usa cada um

```bash
grep -n "\.pos()\|\.named()\|\.pairs()\|args\." ~/.cache/typst/packages/preview/cetz/0.5.2/src/coordinate.typ ~/.cache/typst/packages/preview/cetz/0.5.2/src/drawable.typ ~/.cache/typst/packages/preview/cetz/0.5.2/src/util.typ
```

### Confirmar o estado actual do cristalino, método a método

```bash
./target/release/typst /tmp/p707-args.typ /tmp/p707-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Assinatura completa de `Arguments` confirmada, não só `.pos()`.
- [ ] Cada método usado por `cetz` confirmado, com `file:line`.
- [ ] Estado actual do cristalino confirmado para cada método.

---

## Implementação

Decidir entre renomear os campos existentes (`positional`/`named`) para métodos, ou interceptar `.pos()`/`.named()`/etc. como chamadas de método que delegam ao dado já exposto pelo campo (mesmo padrão de intercepção já usado em P702 para `.with()`).

### Critério de fecho da implementação

- [ ] Todos os métodos confirmados pela sonda implementados.
- [ ] Campos antigos (se mantidos por compatibilidade interna) não colidem com os métodos novos.

---

## Validação

```bash
./target/release/typst /tmp/p707-args.typ /tmp/p707-depois.pdf
pdftotext /tmp/p707-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

### Repetir a reprodução de P700-706

```bash
cat > /tmp/p707-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p707-cetz.typ /tmp/p707-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p707-cetz.png -r 150 /tmp/p707-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita por P688. Se não: registar o próximo bloqueio, usando o tempo de compilação como sinal de profundidade de progresso (como P706 fez, salto de 7s para 30,7s).

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, assinatura completa de `Arguments` confirmada.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p707.md`, com hash do commit.
