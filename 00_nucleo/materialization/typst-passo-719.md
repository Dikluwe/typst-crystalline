---
# P719 — For-loop sobre `Dict` (`for (key, value) in dict`)

> **Passo:** 719
> **Data:** 2026-07-10
> **Foco:** P718 confirmou que `for (key, value) in dict {...}` falha no cristalino ("não é possível iterar sobre dictionary"), enquanto o vanilla itera pares chave-valor. Consumidor real e directo: `styles.typ:189,322,354` de `cetz`, usado na resolução de estilos.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S se o mecanismo de iteração já suportar array/outros e só faltar o braço de `Dict`; M caso contrário.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P718 (onde o bloqueio foi isolado com `file:line` exacto de `cetz`).

---

## Sonda

### Confirmar o comportamento exacto no vanilla

```bash
cat > /tmp/p719-for-dict.typ <<'EOF'
#for (k, v) in (a: 1, b: 2) [#k=#v ]
#for k in (a: 1, b: 2) [#k ]
#for v in (a: 1, b: 2).values() [#v ]
EOF
lab/typst-original/target/release/typst compile /tmp/p719-for-dict.typ /tmp/p719-vanilla.pdf
pdftotext /tmp/p719-vanilla.pdf -
```

Confirmar: qual é a ordem de iteração (inserção, como `IndexMap`, ou outra)? `for k in dict` (um só nome, sem desestruturação) itera as chaves, ou é erro?

### Confirmar o mecanismo de iteração actual do cristalino

```bash
grep -n "fn eval_for\|Content::Array\|Content::Dict\|iterate" 01_core/src/rules/eval/control_flow.rs | head -20
```

Confirmar como o `for` já itera arrays, para reaproveitar a mesma estrutura para dict.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p719-for-dict.typ /tmp/p719-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Ordem de iteração confirmada.
- [ ] Forma de um só nome (`for k in dict`) confirmada.
- [ ] Estrutura de iteração existente para array confirmada, para reaproveitar.

---

## Implementação

Adicionar suporte a `Value::Dict` no mecanismo de `for`, iterando pares `(chave, valor)` na ordem confirmada pela sonda, com destructuring de 2 elementos ligado directamente (reaproveitando o mecanismo de destructuring já corrigido em P715).

### Critério de fecho da implementação

- [ ] `for (k, v) in dict` funciona, ordem correcta.
- [ ] Forma de um só nome, se confirmada pela sonda, também funciona.
- [ ] Iteração sobre array (já existente) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p719-for-dict.typ /tmp/p719-depois.pdf
pdftotext /tmp/p719-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Campos fixos de progresso

```bash
cat > /tmp/p719-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p719-cetz.typ /tmp/p719-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p719-cetz.png -r 150 /tmp/p719-cetz.pdf 2>/dev/null
```

Registar: tempo de compilação, exit code, próximo bloqueio com `file:line` exacto — ou, se produzir PDF, diff de pixels com o vanilla, não só inspecção visual.

---

## Critério de fecho do passo

- [ ] Sonda completa, comportamento confirmado.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — campos fixos de progresso registados.
- [ ] Grep às ADRs em vigor pelos termos centrais (`for`, iteração, `Dict`) antes de fechar o texto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p719.md`, com hash do commit.
