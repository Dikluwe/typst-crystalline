# Passo 1056 — `par.spacing` diverge -6.05pt do vanilla (achado lateral do P1055)

**Tipo**: Investigar → gate (`ADR-0127`, categoria 2/3 — afecta qualquer documento com
2+ parágrafos, o caso mais comum de todos) → corrigir.
**Origem**: descoberto ao isolar o resíduo de `-0.088pt` do `divider()` no P1055. O
`divider` em si fecha a 0.000000pt de erro interno — o resíduo vinha de um bug
pré-existente e não relacionado, maior e mais grave.
**Medição já feita**: `Before\n\nAfter` (2 parágrafos, sem divider):
- Vanilla: gap = 20.438pt (inclui ascender+descender+leading+spacing, com margin
  collapsing).
- Cristalino: gap = 14.388pt ("modelo de flush diferente", ainda por decompor).
- `Δ = -6.050pt`.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1055.

---

## Fase A — Decompor o gap em componentes, não tratar como um número só

O vanilla descreve o gap como soma de várias partes (ascender + descender + leading +
`par.spacing`, com margin collapsing quando aplicável). Antes de mexer em qualquer
constante:

1. Medir cada componente separadamente no vanilla — `file:line` de onde cada parte vem
   (`typst-library/src/model/par.rs`, procurar `spacing`, não presumir que é só
   `leading` já confirmado no P1054).
2. Confirmar o mecanismo de "margin collapsing" — o vanilla soma o espaço abaixo do
   parágrafo anterior com o espaço acima do seguinte, ou faz *collapse* (usa o maior dos
   dois, não a soma)? Isto muda a fórmula inteira, não é detalhe.
3. Medir o "modelo de flush diferente" do cristalino — o que exactamente o mecanismo de
   `flush_line`/cursor faz hoje entre parágrafos, com os mesmos componentes separados.

## Fase B — Localizar a causa, não presumir que é uma constante

Pode ser: (a) `par.spacing` como constante separada de `leading`, nunca lida/aplicada;
(b) mecanismo de margin collapsing ausente (cristalino soma onde vanilla faz collapse,
ou vice-versa); (c) ascender/descender medidos diferente entre os dois motores
(ligação possível ao mesmo tipo de causa do offset do P1048, mas confirmar, não
presumir). **Não escolher (a) só porque já vimos esse padrão antes** — a magnitude
(-6.05pt em ~20pt, ~30%) é grande demais para ser só uma constante em falta; pode ser
mecanismo, não só valor.

## Fase C — Critérios de verificação

```
Dado dois parágrafos simples, sem overrides
Quando renderizado
Então gap entre eles bate com vanilla (20.44pt, ou o valor exacto confirmado na Fase A)

Dado parágrafos com #set par(spacing: X) explícito
Quando renderizado
Então gap respeita X, batendo com vanilla nesse caso também

Dado o caso do divider() já corrigido no P1055
Quando renderizado
Então o resíduo de -0.088pt desaparece (ou reduz à ordem de grandeza do offset já
  catalogado no P1048, não mais)
```

Não-regressão: **todos** os testes de layout que envolvem múltiplos parágrafos — este é
provavelmente o achado com maior superfície de teste afectada desta frente inteira,
validar com atenção redobrada.

## Fase D — Implementar e validar

```
crystalline-lint .
cargo test --workspace
```
Decalque contra o corpus canónico completo, não só um caso isolado — dado o alcance.

---

## Resultado esperado

Gap entre parágrafos a bater com vanilla, com a causa raiz identificada (constante ou
mecanismo, confirmado por medição, não presumido). Prioridade alta — é o achado de maior
alcance prático encontrado nesta frente até agora.
