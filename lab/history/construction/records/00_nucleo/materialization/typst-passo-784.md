---
# P784 — Verificação visual real: fallback de fontes matemáticas com glifo efetivamente ausente

> **Passo:** 784
> **Data:** 2026-07-17
> **Foco:** P783 implementou a cadeia de fallback matemático (`DEFAULT_FALLBACK_FONTS_MATH`) e a lógica de priorização em `shaper.rs`, mas nunca testou com um caso real que exercitasse o caminho — o documento de teste usado (`frac(a,b)`, `x^2_1`) só usa glifos ASCII já cobertos por `Libertinus Serif`, então o `mutool trace` não mudou e a correção nunca foi comprovadamente exercitada. "Está correta mecanicamente" não é evidência. Este passo encontra um glifo matemático real ausente em `Libertinus Serif` e confirma visualmente que a cadeia de fallback funciona.
> **Tipo:** Sonda de verificação. Sem nova implementação, a menos que a verificação revele que a correção de P783 não funciona.
> **Tamanho:** S/M.
> **ADR-0108 EM VIGOR** — não aceitar "correto mecanicamente" sem exercitar o caminho de fato.
> **Dependências:** P783 (implementação a verificar, commit `0c5ea946e`).

---

## Passo 0 — Encontrar um glifo matemático real ausente em Libertinus Serif

```bash
python3 -c "
from fontTools.ttLib import TTFont
f = TTFont('caminho/para/LibertinusSerif-Regular.otf')
cmap = f.getBestCmap()
candidatos = ['\u2115', '\u211D', '\u2124', '\u1D538', '\u1D4D0', '\u2A3F', '\u2A05']
for c in candidatos:
    presente = ord(c) in cmap
    print(f'{c!r} (U+{ord(c):04X}): {\"presente\" if presente else \"AUSENTE\"}')
"
```

Confirmar pelo menos um glifo genuinamente ausente (candidatos: blackboard bold como `ℝ`/`ℕ`/`ℤ` fora do BMP matemático padrão, operadores raros de N-ário, símbolos de teoria de categorias). Se os candidatos acima estiverem todos presentes, ampliar a busca:

```bash
python3 -c "
from fontTools.ttLib import TTFont
f = TTFont('caminho/para/LibertinusSerif-Regular.otf')
cmap = f.getBestCmap()
ausentes = [c for c in range(0x1D400, 0x1D800) if c not in cmap]
print(f'{len(ausentes)} ausentes no bloco; primeiros 10: {[hex(c) for c in ausentes[:10]]}')
"
```

Confirmar também que o glifo escolhido **existe** em `New Computer Modern Math` (a primeira fonte da cadeia de fallback), para que o teste seja conclusivo — se não existir em nenhuma das duas, o teste não prova nada.

```bash
python3 -c "
from fontTools.ttLib import TTFont
f = TTFont('caminho/para/NewCMMath-Regular.otf')
cmap = f.getBestCmap()
print(0x1D538 in cmap)  # ajustar ao candidato escolhido
"
```

---

## Passo 1 — Documento de teste real

```bash
cat > /tmp/p784-fallback-test.typ <<'EOF'
$ <glifo escolhido> $
EOF
lab/typst-original/target/release/typst compile /tmp/p784-fallback-test.typ /tmp/p784-vanilla.pdf
./target/release/typst compile /tmp/p784-fallback-test.typ /tmp/p784-cristalino.pdf
```

Confirmar que o vanilla compila com sucesso e renderiza o glifo (via fallback para `NewCMMath`).

---

## Passo 2 — Confirmar que a cadeia de fallback do cristalino é de fato percorrida

```bash
mutool trace /tmp/p784-cristalino.pdf | grep -i "font\|glyph"
```

Confirmar qual fonte foi efetivamente usada para o glifo — deve ser `New Computer Modern Math` (ou a próxima da cadeia que cobrir o glifo), não uma fonte de fallback genérica serif/sans.

```bash
mutool draw -o /tmp/p784-cristalino.png -r 300 /tmp/p784-cristalino.pdf
mutool draw -o /tmp/p784-vanilla.png -r 300 /tmp/p784-vanilla.pdf
compare -metric AE /tmp/p784-vanilla.png /tmp/p784-cristalino.png /tmp/p784-diff.png
```

Inspecionar visualmente: o glifo aparece (não um retângulo de "glifo ausente"/tofu), e a forma é reconhecível como o carácter esperado.

---

## Passo 3 — Se a verificação falhar

Se o glifo não renderizar, ou renderizar como tofu, ou usar a fonte errada: a implementação de P783 não funciona como descrito. Instrumentar `shaper.rs` (o ponto exato onde `math_fallback_font_list()` é consultado, confirmado por P783 §3.3) para confirmar se a função é de fato chamada e se as candidatas chegam ao `covering_run`. Não assumir onde está o problema — medir.

---

## Critério de fecho do passo

- [ ] Glifo matemático real ausente em `Libertinus Serif` mas presente em `New Computer Modern Math` identificado.
- [ ] Documento de teste real compilado nos dois compiladores.
- [ ] Fonte efetivamente usada pelo cristalino confirmada via `mutool trace` (deve ser `New Computer Modern Math`, não fallback genérico).
- [ ] Glifo renderiza corretamente, confirmado visualmente (não tofu).
- [ ] Se falhar: causa investigada por instrumentação, não suposição, e corrigida ou registrada como achado real.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p784.md`, com o glifo escolhido e a evidência visual/textual documentada.

---

## Próximo passo

Se confirmado funcionando: a série P765a-P784 fecha com o último débito realmente verificado, não só implementado às cegas. Momento de fazer o resumo final.
Se não funcionando: corrigir antes de considerar P783 fechado, mesmo padrão de rigor já aplicado ao longo de toda a série.
