---
# P683 — `#import` a partir de módulo/field-access, não só de caminho literal

> **Passo:** 683
> **Data:** 2026-07-10
> **Foco:** P682 encontrou que `cetz` avança para lá da linha 1 e falha em `src/util.typ:2` (`#import deps.oxifmt: strfmt`) e `src/anchor.typ:5` (`#import util: typst-length`) — formas de `#import` que recebem uma expressão (valor de módulo, ou field-access sobre um módulo), não uma string literal directa. O `#import` do cristalino (P679) só aceita string literal. Este passo estende para as formas em falta.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P679/P680 (`#import` de ficheiros locais e pacotes, só string literal), P682 (onde este bloqueio foi encontrado).

---

## Sonda mínima

### Confirmar todas as formas de `source` que o vanilla aceita em `#import`

```bash
cat > /tmp/p683-a.typ <<'EOF'
#let valor = "de A"
EOF
cat > /tmp/p683-b.typ <<'EOF'
#import "p683-a.typ" as modulo_a
#import modulo_a: valor
#valor
EOF
lab/typst-original/target/release/typst compile /tmp/p683-b.typ /tmp/p683-b-vanilla.pdf
pdftotext /tmp/p683-b-vanilla.pdf -
```

Testar directamente o padrão exacto usado por `cetz`: um módulo importado com nome, depois usado como fonte de outro `#import`, e uma variante com field-access (`modulo.campo`) como fonte.

```bash
cat > /tmp/p683-fieldaccess.typ <<'EOF'
#import "p683-a.typ" as pacote
#import pacote.valor
EOF
```

Confirmar se `pacote.valor` (onde `valor` já é o valor final, não outro módulo) faz sentido como fonte de `#import`, ou se o padrão real é sempre "field-access que resolve para outro módulo".

### Confirmar exactamente a estrutura de `deps.oxifmt` no `cetz`

```bash
cat ~/.cache/typst/packages/preview/cetz/0.2.2/src/deps.typ 2>/dev/null | head -20
```

Confirmar como `deps` é definido — provavelmente um `#import` que agrupa vários pacotes/módulos sob um nome, e depois `deps.oxifmt` acede a um deles.

### Critério de fecho da sonda mínima

- [ ] Formas de `source` em `#import` confirmadas contra o vanilla (módulo simples, field-access).
- [ ] Estrutura real de `deps.oxifmt` no `cetz` confirmada, para saber exactamente o que suportar.

---

## Implementação

Estender `eval_module_import` (`01_core/src/rules/eval/modules.rs`) para aceitar, além de `Expr::Str`, qualquer expressão que avalie para `Value::Module` — incluindo identificadores simples (referência a um módulo já importado) e field-access sobre um módulo (que pode devolver outro módulo, se o campo acedido for ele próprio um módulo).

### Critério de fecho da implementação

- [ ] `#import modulo_a: valor` funciona (módulo já ligado a um nome, usado como fonte).
- [ ] `#import modulo.campo: valor` funciona, quando `campo` é outro módulo.
- [ ] String literal (P679) sem regressão.
- [ ] Import de pacote (P681) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p683-b.typ /tmp/p683-b-cristalino.pdf
pdftotext /tmp/p683-b-cristalino.pdf -
```

Comparar com o resultado do vanilla já obtido na sonda.

### Confirmar que `cetz` avança mais

```bash
cat > /tmp/p683-cetz.typ <<'EOF'
#import "@preview/cetz:0.2.2": canvas, draw
#canvas({
  draw.line((0,0), (1,1))
})
EOF
./target/release/typst /tmp/p683-cetz.typ /tmp/p683-cetz.pdf
echo "Exit code: $?"
```

Mesma disciplina de P682: não assumir que `cetz` está resolvido só porque este bloqueio específico desapareceu — se houver mais um problema adiante, registá-lo, não escondê-lo.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, formas confirmadas contra o vanilla e contra a estrutura real do `cetz`.
- [ ] `#import` de módulo e field-access implementado.
- [ ] Formas anteriores (string literal, pacote) sem regressão.
- [ ] `cetz` re-testado, próximo estado registado com honestidade.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p683.md`, com hash do commit.
