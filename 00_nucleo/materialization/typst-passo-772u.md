---
# P772u — Colapso de espaço em Cantarell-VF (CFF2/HVAR) no shaper

> **Passo:** 772u
> **Data:** 2026-07-17
> **Foco:** P772o corrigiu o colapso de espaço para fontes `glyf`/`gvar` (Ubuntu Sans) ao aplicar variação de eixo em `advance()` (medição de layout), mas confirmou que `Cantarell-VF.otf` (CFF2+HVAR) continua a colar palavras mesmo com as métricas de `advance()` já corretas — a causa está noutro lugar, com hipótese não verificada de que o *shaper* (`rustybuzz`, via `shaper.rs`) não aplica os deltas de `HVAR`/CFF2 da mesma forma que `fontTools`/`ttf_parser`, desenhando glifos mais largos do que o layout (já correto) prevê. Este passo instrumenta `shaper.rs` para confirmar ou refutar essa hipótese.
> **Tipo:** Sonda instrumentada + Implementação se a causa for confirmada.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — instrumentar antes de corrigir, mesma disciplina de P772o.
> **Dependências:** P772o (achado, causa em `advance()` já eliminada para Cantarell, hipótese de `shaper.rs` registada).

---

## Passo 0 — Reconfirmar reprodução com o binário atual (pós-P772o)

```bash
cat > /tmp/p772u-test.typ <<'EOF'
#set text(font: "Cantarell", weight: 800)
Weight test here
EOF
./target/release/typst compile /tmp/p772u-test.typ /tmp/p772u-800.pdf
mutool draw -o /tmp/p772u-800.png -r 300 /tmp/p772u-800.pdf
```

Confirmar que o colapso ainda ocorre com o código já corrigido por P772o (isolando que esta é uma causa nova, não resíduo da mesma).

---

## Passo 1 — Instrumentar `shaper.rs`

```bash
grep -n "fn shape\|set_variations\|rb_face" 03_infra/src/shaper.rs
```

Adicionar instrumentação temporária que registe, para cada glifo shapeado no documento de teste (peso 800):
1. As posições/avanços devolvidos por `rustybuzz::shape` para cada glifo, incluindo o espaço.
2. As coordenadas de eixo (`axis_vars`) efetivamente passadas a `rb_face.set_variations`.

```bash
python3 -c "
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
f = TTFont('caminho/para/Cantarell-VF.otf')
instance = instantiateVariableFont(f, {'wght': 800})
# extrair advance do glifo espaço e das letras de 'Weight test here'
"
```

Comparar os avanços do `rustybuzz` (via instrumentação) com os do `fontTools` (chão de verdade, mesmo método usado em P772o) para o mesmo peso.

---

## Passo 2 — Confirmar ou refutar a hipótese CFF2/HVAR

Se os avanços do `rustybuzz` para Cantarell a peso 800 forem sistematicamente maiores que os do `fontTools`: hipótese confirmada — o shaper está a desenhar glifos mais largos do que o layout reserva.

Se os avanços baterem: a hipótese cai, e a causa está em outro lugar — considerar:
1. Posicionamento de glifo dentro do frame (`FrameItem::TextShaped`) não usar os avanços do shaper corretamente.
2. Alguma diferença entre a instância de fonte usada pelo shaper vs a usada para desenhar (ex: cache de instância variável desatualizado, mesma família de bug do `cached_face` já descartada para `advance()` em P772o, mas agora no caminho do shaper).

```bash
grep -n "fn.*text_shaped\|FrameItem::TextShaped" 01_core/src/engine/layout/*.rs 03_infra/src/export/*.rs 2>/dev/null | head -20
```

---

## Implementação (se a causa for confirmada)

Conforme o achado do Passo 2 — pode ser correção em `shaper.rs` (aplicação de variação), ou em outro ponto do caminho de desenho de glifos shapeados.

---

## Validação

```bash
./target/release/typst compile /tmp/p772u-test.typ /tmp/p772u-800-depois.pdf
mutool draw -o /tmp/p772u-800-depois.png -r 300 /tmp/p772u-800-depois.pdf
```

Confirmar visualmente e por medição de avanço que as palavras voltam a separar-se, em pesos 100-900.

```bash
cargo test --workspace
crystalline-lint .
```

Confirmar sem regressão em Ubuntu Sans (correção de P772o) nem em Noto Sans.

---

## Critério de fecho do passo

- [ ] Reprodução reconfirmada com o binário pós-P772o.
- [ ] `shaper.rs` instrumentado, avanços comparados com `fontTools` como chão de verdade.
- [ ] Hipótese CFF2/HVAR confirmada ou refutada com evidência.
- [ ] Se refutada: nova hipótese investigada (cache de instância, posicionamento de frame), não abandonado sem causa.
- [ ] Se corrigido: validado em todos os pesos testados, sem regressão em Ubuntu Sans/Noto Sans.
- [ ] Se não corrigido nesta rodada: registrado o que foi descartado, para o próximo passo não repetir.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772u.md`.

---

## Próximo passo

Conforme decidido em P772t: `table()` header/footer (extensão de P772i), seguido do resíduo de risco plausível do inventário (`foundations::target_`, `plugin_`, `image::pdf`, `layout::frame`, `math` — 23 itens).
