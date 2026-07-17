---
# P742 — Métodos de instância de cor (`red.lighten(20%)`) e fields `rotate`/`components`/`space`

> **Passo:** 742
> **Data:** 2026-07-10
> **Foco:** P736 confirmou, por inventário completo, que faltam métodos de instância de cor (`color_instance.lighten(...)`, etc. — só existem como funções estáticas desde P476/P477) e três fields do tipo `color` (`rotate`, `components`, `space`). Pré-existente desde P476, confirmado agora.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P736 (onde o inventário completo foi feito), P506 (métodos de instância já existentes para `counter`/`state`, padrão a seguir).

---

## Sonda

### Confirmar a lista completa de métodos de instância no vanilla

```bash
cat > /tmp/p742-color-methods.typ <<'EOF'
#red.lighten(20%)
#red.darken(20%)
#red.negate()
#red.rotate(90deg)
#red.mix(blue)
#red.components()
#red.space()
#red.saturate(20%)
#red.desaturate(20%)
EOF
lab/typst-original/target/release/typst compile /tmp/p742-color-methods.typ /tmp/p742-vanilla.pdf
pdftotext /tmp/p742-vanilla.pdf -
```

Confirmar a lista completa (P736 já mencionou 17 fields no tipo — confirmar quais são estáticos, quais são métodos de instância, e se há sobreposição).

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p742-color-methods.typ /tmp/p742-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Lista completa de métodos de instância confirmada.
- [ ] Assinatura de cada um confirmada (argumentos, tipo de retorno).

---

## Implementação

Adicionar os métodos de instância confirmados, seguindo o mesmo padrão de intercepção já usado para `counter`/`state` (P506) — despacho por identidade de método sobre `Value::Color`.

### Critério de fecho da implementação

- [ ] Todos os métodos confirmados implementados e testados.
- [ ] Funções estáticas equivalentes (já existentes, P476/P477) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p742-color-methods.typ /tmp/p742-depois.pdf
pdftotext /tmp/p742-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, lista e assinaturas confirmadas.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p742.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
- [ ] Confirmar se, depois deste passo, `achados-adiados-cetz.md` fica sem itens "Por resolver" — se sim, declarar isso explicitamente no relatório.
