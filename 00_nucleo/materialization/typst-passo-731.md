---
# P731 — Namespaces embutidos (`calc`, `sys`, ...) devem ser `Module`, não `Dict`

> **Passo:** 731
> **Data:** 2026-07-10
> **Foco:** P730 confirmou que `calc` está representado como `Value::Dict` no scope raiz do cristalino, enquanto o vanilla o expõe como `Value::Module` (`type(calc)` → `module`, não `dictionary`). Isto bloqueia `#import calc: min, max` — usado por `cetz` em `aabb.typ:18`, dentro do cálculo de bounds de `line`. Não é um problema em `#import`; é a representação errada na origem.
> **Tipo:** Sonda ampla + Implementação. Verificar todos os namespaces embutidos de uma vez, não um a um.
> **Tamanho:** M — depende de quantos namespaces estiverem afectados.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P730 (onde o problema foi encontrado com caso mínimo sem `cetz`), P679 (`#import`, que já rejeita `Dict` correctamente — não mexer nesse mecanismo).

---

## Sonda ampla

### Confirmar quais namespaces embutidos são `Module` no vanilla

```bash
cat > /tmp/p731-tipos.typ <<'EOF'
#type(calc)
#type(sys)
#type(std)
#(if "math" in (:) { none }) #type(math)
EOF
lab/typst-original/target/release/typst compile /tmp/p731-tipos.typ /tmp/p731-vanilla.pdf
pdftotext /tmp/p731-vanilla.pdf -
```

Ajustar a lista conforme os namespaces reais existentes no vanilla (confirmar via documentação ou grep na fonte, não assumir).

### Confirmar o estado actual do cristalino, um a um

```bash
cat > /tmp/p731-cristalino-tipos.typ <<'EOF'
#type(calc)
#type(sys)
#type(std)
EOF
./target/release/typst /tmp/p731-cristalino-tipos.typ /tmp/p731-cristalino.pdf
pdftotext /tmp/p731-cristalino.pdf -
```

Confirmar quais já são `Module` (por exemplo, `std`, implementado em P709 já como `Value::Module`) e quais são `Dict`.

### Localizar onde cada namespace é construído

```bash
grep -n "scope.define(\"calc\"\|scope.define(\"sys\"\|Value::Dict.*calc\|Value::Module.*calc" 01_core/src/rules/eval/mod.rs 01_core/src/rules/stdlib/*.rs
```

### Critério de fecho da sonda ampla

- [ ] Lista completa de namespaces embutidos confirmada, com o tipo real no vanilla para cada um.
- [ ] Estado actual do cristalino confirmado, namespace a namespace.
- [ ] Localização exacta de onde cada um é construído.

---

## Implementação

Converter cada namespace confirmado como `Dict` (mas devendo ser `Module`) para `Value::Module`, seguindo o mesmo padrão já usado por `std` (P709) e pelo namespace de `curve`/`table`/outros já correctos.

### Critério de fecho da implementação

- [ ] Todos os namespaces confirmados corrigidos para `Module`.
- [ ] `#import calc: min, max` (e equivalentes para outros namespaces) funciona.
- [ ] Acesso por field access directo (`calc.min(...)`, já funcional antes) sem regressão — confirmar que a mudança de `Dict` para `Module` não quebra esse caminho, já usado extensivamente em toda a stdlib.

---

## Validação

```bash
./target/release/typst /tmp/p731-cristalino-tipos.typ /tmp/p731-depois.pdf
pdftotext /tmp/p731-depois.pdf -
```

Comparar com o vanilla já obtido na sonda — todos devem agora dizer `module`.

```bash
cat > /tmp/p731-import.typ <<'EOF'
#import calc: min, max
#min(1, 2)
#max(1, 2)
EOF
./target/release/typst /tmp/p731-import.typ /tmp/p731-import-depois.pdf
pdftotext /tmp/p731-import-depois.pdf -
```

```bash
cargo test --workspace
crystalline-lint .
```

Dado que isto muda a representação de namespaces usados extensivamente em toda a stdlib, correr o corpus completo com atenção redobrada a regressão silenciosa.

### Reprodução final de `cetz` — diff de pixels

```bash
cat > /tmp/p731-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p731-cetz.typ /tmp/p731-cetz.pdf
mutool draw -o /tmp/p731-cetz.png -r 150 /tmp/p731-cetz.pdf
lab/typst-original/target/release/typst compile /tmp/p731-cetz.typ /tmp/p731-cetz-vanilla.pdf
mutool draw -o /tmp/p731-cetz-vanilla.png -r 150 /tmp/p731-cetz-vanilla.pdf
```

Diff de pixels — se aproximar de zero, a cadeia P678-731 fecha de vez.

---

## Critério de fecho do passo

- [ ] Sonda ampla completa, todos os namespaces confirmados.
- [ ] Implementado e testado, incluindo `#import` e field access directo.
- [ ] Sem regressão em `cargo test --workspace`, atenção redobrada dado o alcance.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — diff de pixels final registado, ou próximo bloqueio.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p731.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
- [ ] Se a cadeia fechar: resumo completo, número de passos, categorias de bugs corrigidos.
