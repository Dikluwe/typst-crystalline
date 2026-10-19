---
# P797 — Ênfase (`*bold*`/`_italic_`) sem efeito visual por falta de faces na fonte embutida

> **Passo:** 797
> **Data:** 2026-07-20
> **Foco:** P786 confirmou em `text::item` que `*negrito*`/`_itálico_` compilam sem erro (`pdftotext` idêntico ao vanilla — o texto está correto), mas não têm efeito visual algum. Causa já apontada pela inspeção de `pdffonts`: o vanilla embute três faces reais (`LibertinusSerif-Regular`/`-Bold`/`-Italic`), enquanto o cristalino embute três *subsets* sintéticos da mesma face única (`CrystallineFont1/2/3`) — o shaping separa corretamente os runs por estilo (por isso há 3 subsets), mas nenhuma das faces bold/italic reais está disponível para embutir. É bug de infraestrutura de fontes, não de `TextItem`/shaping.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M/L — toca a seleção/embutimento de faces de fonte, mecanismo compartilhado por todo texto estilizado.
> **ADR-0108 EM VIGOR** — confirmar exatamente onde a seleção de face bold/italic se perde antes de implementar.
> **Dependências:** P786 (achado, evidência `pdffonts` em `temp/temp_p786/a_textitem_emph.typ`), P753/P772o/P772u (contexto de infraestrutura de fontes já trabalhado nesta conversa — confirmar se há sobreposição).

---

## Sonda — onde a seleção de face bold/italic se perde

```bash
grep -n "fn.*select_face\|fn.*bold\|fn.*italic\|FontVariant" 03_infra/src/embedded_fonts.rs 03_infra/src/font_metrics.rs 2>/dev/null | head -30
```

Confirmar:
1. Se as fontes embutidas do cristalino (`LibertinusSerif`) têm, de fato, as variantes Bold/Italic disponíveis nos assets, ou se só a Regular está embutida no binário.
2. Se as variantes existem nos assets, onde a lógica de seleção falha ao tentar resolver `weight: bold`/`style: italic` para a face correspondente.
3. Se não existem nos assets: confirmar isso e tratar como uma lacuna de asset, não de lógica.

```bash
ls -la 03_infra/assets/fonts/ 2>/dev/null | grep -i libertinus
find . -iname "*libertinus*bold*" -o -iname "*libertinus*italic*" 2>/dev/null
```

```bash
cat > /tmp/p797-test.typ <<'EOF'
normal *negrito* _italico_
EOF
lab/typst-original/target/release/typst compile /tmp/p797-test.typ /tmp/p797-vanilla.pdf
./target/release/typst compile /tmp/p797-test.typ /tmp/p797-cristalino.pdf
pdffonts /tmp/p797-vanilla.pdf
pdffonts /tmp/p797-cristalino.pdf
```

---

## Decisão de âmbito

| Cenário | Decisão |
|---|---|
| Assets de fonte Bold/Italic existem mas não são selecionados | Corrigir a lógica de seleção — provavelmente pequeno |
| Assets de fonte Bold/Italic não existem no projeto | Decisão maior — adicionar os assets (peso/licença a confirmar) ou aceitar synthetic bold/italic (técnica de desenhar negrito/itálico sem a face real, usada por alguns renderizadores) como aproximação, registrando a divergência |

---

## Implementação

Conforme a sonda revelar.

---

## Validação

```bash
./target/release/typst compile /tmp/p797-test.typ /tmp/p797-cristalino-depois.pdf
pdffonts /tmp/p797-cristalino-depois.pdf
mutool draw -o /tmp/p797-vanilla.png -r 300 /tmp/p797-vanilla.pdf
mutool draw -o /tmp/p797-cristalino.png -r 300 /tmp/p797-cristalino-depois.pdf
compare -metric AE /tmp/p797-vanilla.png /tmp/p797-cristalino.png /tmp/p797-diff.png
```

Confirmar visualmente que negrito/itálico aparecem de verdade, não só que as faces estão listadas em `pdffonts`.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Causa confirmada: assets ausentes ou lógica de seleção quebrada.
- [ ] Decisão de âmbito registrada.
- [ ] Bold/italic renderizam com efeito visual real, confirmado por render (não só `pdffonts`).
- [ ] `cargo test --workspace` verde, contagem da suíte mostrada.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p797.md`, com comandos, saídas reais e nomes dos testes persistidos.

---

## Próximo passo

Com este fechado, os dez itens da lista de P786 §5 estão completos. Momento de reconfirmar `lacuna-inventario` (estilo P772e/P772t) ou fazer o resumo final pendente da série inteira (P765a em diante).
