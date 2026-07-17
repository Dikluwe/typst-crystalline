---
# P744 — Três achados finais: `space:` nomeado, `to-hex`/`transparentize`/`opacify`, repr de closure anónima

> **Passo:** 744
> **Data:** 2026-07-10
> **Foco:** Os três achados encontrados por P742 e ainda não corrigidos. Todos pequenos, independentes entre si.
> **Tipo:** Sonda + Implementação, três sub-partes independentes.
> **Tamanho:** M no total.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P742 (onde os três foram encontrados e adiados).

---

## Parte A — Argumento nomeado `space:` em `negate`/`rotate`/`mix`

### Sonda

```bash
cat > /tmp/p744a-space.typ <<'EOF'
#red.mix(blue, space: rgb)
#red.negate(space: oklab)
#red.rotate(90deg, space: oklch)
EOF
lab/typst-original/target/release/typst compile /tmp/p744a-space.typ /tmp/p744a-vanilla.pdf
pdftotext /tmp/p744a-vanilla.pdf -
```

Confirmar os espaços de cor aceites como valor de `space:`, e o default de cada função (já medido parcialmente em P742).

### Implementação

Adicionar `space:` como argumento nomeado opcional em `mix`/`negate`/`rotate`, seguindo o mecanismo já construído em P742 para a conversão entre espaços (`to_space`).

### Critério de fecho da Parte A

- [ ] `space:` funciona nas três funções, testado contra o vanilla.
- [ ] Defaults (já correctos) sem regressão.

---

## Parte B — `to-hex`/`transparentize`/`opacify`

### Sonda

```bash
cat > /tmp/p744b-hex.typ <<'EOF'
#red.to-hex()
#red.transparentize(50%)
#red.opacify(50%)
EOF
lab/typst-original/target/release/typst compile /tmp/p744b-hex.typ /tmp/p744b-vanilla.pdf
pdftotext /tmp/p744b-hex.typ /tmp/p744b-vanilla.pdf -
```

Confirmar a assinatura e comportamento exacto de cada um.

### Implementação

Adicionar os três métodos, seguindo o padrão de despacho de instância já estabelecido em P742.

### Critério de fecho da Parte B

- [ ] Os três métodos implementados e testados.

---

## Parte C — Repr de closure anónima

### Sonda

```bash
cat > /tmp/p744c-closure-repr.typ <<'EOF'
#repr((x) => x + 1)
#let f = (x) => x + 1
#repr(f)
EOF
lab/typst-original/target/release/typst compile /tmp/p744c-closure-repr.typ /tmp/p744c-vanilla.pdf
pdftotext /tmp/p744c-vanilla.pdf -
```

Confirmar o formato exacto (P742 mencionou `#function(...)` vs `(x) => x` como as duas formas em jogo — confirmar qual é o vanilla).

### Implementação

Corrigir `repr_value` para `Value::Func` (caso de closure) para produzir o formato confirmado.

### Critério de fecho da Parte C

- [ ] Repr de closure correcto, testado.

---

## Validação final (todas as partes)

```bash
cargo test --workspace
crystalline-lint .
```

Repetir a reprodução de `cetz` para confirmar que a cadeia já fechada continua sem regressão:

```bash
cat > /tmp/p744-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p744-cetz.typ /tmp/p744-cetz.pdf
mutool draw -o /tmp/p744-cetz.png -r 150 /tmp/p744-cetz.pdf
```

---

## Critério de fecho do passo

- [ ] Cada uma das três partes tratada individualmente.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` sem regressão.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p744.md`, com hash do commit.
- [ ] Cada item actualizado em `achados-adiados-cetz.md`.
- [ ] Declarar explicitamente o estado final da lista "Por resolver" — vazia, ou com os dois scope-outs reforçados (vértices `Ratio`, ordem no erro) como únicos itens conscientemente aceites.
