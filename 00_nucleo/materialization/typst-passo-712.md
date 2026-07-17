---
# P712 — `measure()` devolve sempre `0pt`, silenciosamente

> **Passo:** 712
> **Data:** 2026-07-10
> **Foco:** P711 encontrou, como achado lateral não relacionado ao bug corrigido, que `measure()` devolve sempre `0pt` — dentro e fora de `context`, sem erro nenhum. O vanilla exige `context` para `measure()` funcionar (erro claro se usado fora) e devolve dimensões reais dentro. Isto é um mecanismo central usado por qualquer documento com decisões de layout dinâmicas, silenciosamente inoperante até agora. Prioridade alta.
> **Tipo:** Sonda + Implementação. Prioridade alta, alcance potencialmente amplo.
> **Tamanho:** M-L.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mecanismo central produzindo valores errados sem erro.
> **Dependências:** P711 (onde o bug foi encontrado e confirmado como não relacionado à correcção de `context`).

---

## Contexto

Confirmado por P711:

```typst
#set text(size: 20pt)
#context [#let s = measure[Texto de teste]; #s.width]
```

Vanilla: `113.06pt`. Cristalino: `Abs(0.0)`, sempre.

Fora de `context`, o vanilla erra: `error: can only be used when context is known`. O cristalino aceita e devolve zero, silenciosamente — dois problemas em um: falta o gate de `context`, e o cálculo em si nunca funciona mesmo quando devia.

---

## Sonda

### Confirmar o mecanismo completo do vanilla

```bash
grep -n "fn measure\|pub fn measure" lab/typst-original/crates/typst-library/src/layout/measure.rs 2>/dev/null | head -10
```

Confirmar como `measure()` realmente calcula dimensões — provavelmente invoca um layout real (mesmo motor usado para o documento) sobre o conteúdo dado, numa página/região "infinita" ou de tamanho apropriado, e extrai o tamanho do frame resultante.

### Confirmar o estado actual do cristalino, `file:line`

```bash
grep -rn "fn native_measure\|\"measure\"" 01_core/src/rules/stdlib/*.rs 01_core/src/rules/eval/*.rs | head -10
```

Localizar exactamente onde o `0pt` é produzido — se é um valor hard-coded/placeholder nunca substituído, ou se há uma tentativa de cálculo real que está a falhar silenciosamente e a cair num default.

### Confirmar outros casos de teste

```bash
cat > /tmp/p712-measure.typ <<'EOF'
#set text(size: 12pt)
#context {
  let s1 = measure[Texto curto]
  let s2 = measure[Um texto bastante mais comprido do que o anterior]
  [Curto: #s1.width, Longo: #s2.width]
}
EOF
lab/typst-original/target/release/typst compile /tmp/p712-measure.typ /tmp/p712-vanilla.pdf
pdftotext /tmp/p712-vanilla.pdf -
```

Confirmar que textos diferentes produzem larguras diferentes e proporcionais (não só "não é zero", mas "é o valor certo").

### Critério de fecho da sonda

- [ ] Mecanismo do vanilla confirmado (como o layout real é invocado para medir).
- [ ] Localização exacta de onde o cristalino produz `0pt`, com `file:line`.
- [ ] Confirmado se falta só o cálculo, ou também o gate de `context`.

---

## Implementação

Implementar `measure()` para invocar um layout real do conteúdo dado (reaproveitando o motor de layout já existente, numa região apropriada), extraindo as dimensões reais do resultado. Adicionar o gate de `context` (erro claro fora de `context`, como o vanilla).

### Critério de fecho da implementação

- [ ] `measure()` dentro de `context` devolve dimensões reais, proporcionais ao conteúdo.
- [ ] `measure()` fora de `context` produz erro claro, igual ao vanilla.
- [ ] Testado com textos de tamanhos diferentes, confirmando que as larguras diferem correctamente.

---

## Validação

```bash
./target/release/typst /tmp/p712-measure.typ /tmp/p712-depois.pdf
pdftotext /tmp/p712-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

Dado o alcance potencial (qualquer documento que use `measure()`), correr o corpus de testes já existente, confirmando que nenhum documento anterior desta conversa dependia silenciosamente do valor zero.

### Repetir a reprodução de `cetz` (P700-711)

```bash
cat > /tmp/p712-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p712-cetz.typ /tmp/p712-cetz.pdf
echo "Exit code: $?"
```

---

## Critério de fecho do passo

- [ ] Sonda completa, mecanismo e causa exacta confirmados.
- [ ] `measure()` corrigido, com gate de `context` e cálculo real.
- [ ] Varredura ampla do corpus existente, sem regressão silenciosa.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado, estado registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p712.md`, com hash do commit.
