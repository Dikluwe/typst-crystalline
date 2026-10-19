---
# P686 — Resolver caminhos absolutos (`/src/process.typ`) em `#import`/`#include`

> **Passo:** 686
> **Data:** 2026-07-10
> **Foco:** P685 encontrou que `cetz` avança e falha em `include: ficheiro não encontrado: /src/process.typ` — um caminho que começa por `/`, que o Typst trata como relativo à raiz do projecto ou do pacote (não relativo ao ficheiro que faz o import), diferente do caminho relativo já suportado desde P679. Este é o próximo bloqueio confirmado de `cetz`.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P679/P680/P683 (`#import`/`#include` relativos, já implementados), P685 (onde este bloqueio foi encontrado).

---

## Sonda mínima

### Confirmar o comportamento exacto de caminhos absolutos no vanilla

```bash
mkdir -p /tmp/p686-projecto/src
cat > /tmp/p686-projecto/src/util.typ <<'EOF'
#let valor = "de util"
EOF
cat > /tmp/p686-projecto/main.typ <<'EOF'
#import "/src/util.typ": valor
#valor
EOF
lab/typst-original/target/release/typst compile /tmp/p686-projecto/main.typ /tmp/p686-vanilla.pdf
pdftotext /tmp/p686-vanilla.pdf -
```

Confirmar: `/src/util.typ` resolve relativo a quê exactamente — à raiz do projecto (definida por `--root`, ou o directório do ficheiro de entrada por defeito), ou a alguma outra referência?

### Confirmar o caso específico dentro de um pacote

```bash
grep -rn "include \"/\|import \"/" ~/.cache/typst/packages/preview/cetz/0.2.2/src/*.typ 2>/dev/null | head -10
```

Confirmar se, dentro de um pacote, `/src/process.typ` resolve relativo à raiz **do pacote** (o directório onde o `typst.toml` desse pacote está), não à raiz do projecto do utilizador — isto é importante, porque um pacote não deve poder aceder a ficheiros fora de si próprio usando caminho absoluto.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst --root /tmp/p686-projecto /tmp/p686-projecto/main.typ /tmp/p686-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda mínima

- [ ] Confirmado a que raiz um caminho absoluto resolve, fora de pacotes.
- [ ] Confirmado a que raiz um caminho absoluto resolve, dentro de um pacote (deve ser a raiz do próprio pacote, não do projecto do utilizador).
- [ ] Estado actual do cristalino confirmado.

---

## Implementação

Estender a resolução de caminho em `#import`/`#include` (reaproveitando o mecanismo já criado em P679) para reconhecer caminhos que começam por `/` como absolutos, resolvendo-os relativo à raiz apropriada:
- Se o ficheiro que faz o import pertence a um pacote: raiz do pacote.
- Caso contrário: raiz do projecto (a mesma já usada para resolver imagens e outros recursos, se esse conceito já existir no cristalino — confirmar).

### Critério de fecho da implementação

- [ ] Caminho absoluto fora de pacotes resolve correctamente, testado contra o vanilla.
- [ ] Caminho absoluto dentro de um pacote resolve relativo à raiz desse pacote, não do projecto do utilizador.
- [ ] Caminhos relativos (P679) sem regressão.

---

## Validação

```bash
./target/release/typst --root /tmp/p686-projecto /tmp/p686-projecto/main.typ /tmp/p686-depois.pdf
pdftotext /tmp/p686-depois.pdf -
```

Comparar com o resultado do vanilla já obtido na sonda.

### Confirmar que `cetz` avança mais

```bash
cat > /tmp/p686-cetz.typ <<'EOF'
#import "@preview/cetz:0.2.2": canvas, draw
#canvas({
  draw.line((0,0), (1,1))
})
EOF
./target/release/typst /tmp/p686-cetz.typ /tmp/p686-cetz.pdf
echo "Exit code: $?"
```

Mesma disciplina: registar o próximo estado com honestidade. Se `cetz` finalmente produzir um PDF, comparar visualmente com o resultado do vanilla (já confirmado a funcionar desde P678).

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, incluindo o caso específico de raiz de pacote.
- [ ] Caminho absoluto implementado, testado contra o vanilla nos dois contextos (projecto e pacote).
- [ ] Caminhos relativos sem regressão.
- [ ] `cetz` re-testado — se funcionar por completo, comparação visual com o vanilla; se não, próximo bloqueio registado com honestidade.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p686.md`, com hash do commit.
