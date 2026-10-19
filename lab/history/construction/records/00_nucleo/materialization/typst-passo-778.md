---
# P778 — Verificação: o ICC teve efeito real? Interpolação de rotação como causa do resíduo

> **Passo:** 778
> **Data:** 2026-07-16
> **Foco:** P777 reportou o mesmo AE residual (195/195/0/138/138/195/195 por orientação) antes e depois de implementar `/ICCBased`, e atribuiu o resíduo a imprecisão de margem de página (~0,01pt) — uma magnitude implausível para AE=195 e uma explicação que não justifica o padrão observado: o resíduo separa por **tipo de transformação** (flip/flip+rotação = 195; rotação pura 90°/270° = 138; sem transformação = 0), não é uniforme como uma constante de página produziria. Este passo confirma se o ICC teve algum efeito mensurável, e investiga interpolação/reamostragem de rotação como causa alternativa mais consistente com o padrão.
> **Tipo:** Sonda de verificação. Implementação só se causa nova for confirmada.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — não aceitar explicação cuja magnitude e padrão não batem com a medição.
> **Dependências:** P777 (medições reportadas, commit `6ba0a1042e6787da0d4be5f8f14850b7f58a3132`), P776 (medições anteriores ao ICC, para comparação byte a byte).

---

## Passo 0 — O ICC teve efeito real?

Comparar directamente os PDFs gerados antes (P776) e depois (P777) da mudança de color space, para a mesma orientação:

```bash
# PDF do cristalino ANTES do ICC (P776) vs DEPOIS (P777), mesma orientação, ex: orientação 2
diff <(mutool trace /tmp/p776-cristalino-orient2.pdf) <(mutool trace /tmp/p777-cristalino-orient2.pdf)
```

Se os PDFs (fora do objecto de color space em si) forem idênticos ao nível dos pixels desenhados, confirmar se a **imagem rasterizada final** (`mutool draw`) mudou nalgum pixel entre P776 e P777:

```bash
mutool draw -o /tmp/p778-p776-render.png -r 300 /tmp/p776-cristalino-orient2.pdf
mutool draw -o /tmp/p778-p777-render.png -r 300 /tmp/p777-cristalino-orient2.pdf
compare -metric AE /tmp/p778-p776-render.png /tmp/p778-p777-render.png /tmp/p778-antes-depois-diff.png
```

**Se AE=0 entre P776 e P777** (a imagem renderizada não mudou nada com a mudança de ICC): confirma que o ICC não teve efeito mensurável no render — a mudança foi estrutural no PDF (correcta e desejável para fidelidade de metadados), mas não é a explicação para nenhum resíduo, presente ou passado. Isso também significa que a atribuição original de P776 ("resíduo devido a color space") **também estava errada** — o resíduo sempre teve outra causa, só coincidiu com o color space estar em aberto na altura.

---

## Passo 1 — Investigar interpolação/reamostragem como causa

### Confirmar o algoritmo de reamostragem do vanilla vs cristalino para rotação 90°/270°

```bash
grep -n "rotate90\|rotate270\|interpolat\|resample\|nearest\|bilinear" lab/typst-original/crates/typst-pdf/src/image.rs lab/typst-original/crates/typst-library/src/visualize/image/*.rs 2>/dev/null | head -20
```

Lembrar: desde P776, o vanilla **não recodifica pixels** — aplica a rotação via matriz `cm`, sem tocar nos dados da imagem. Se isso for verdade tanto para o vanilla como para o cristalino (confirmado em P776), não deveria haver reamostragem nenhuma — a "interpolação" só entra no `mutool draw`/rasterizador ao desenhar a imagem rodada, não no PDF em si. Confirmar isto por leitura de código antes de investigar mais fundo — se ambos os PDFs usam matriz pura, a causa não pode ser reamostragem de pixels da imagem (isso não existe neste caminho), tem de ser outra coisa.

### Hipótese alternativa: composição da matriz `cm` em si

Se a rotação é só matriz (confirmado no passo anterior), o resíduo de 138/195 pode vir de:
1. Diferença de precisão na própria matriz `cm` para orientações 5/6/2/3/7/8 especificamente — reexaminar os valores exactos reportados em P776 (`170.866` vs `170.867`) para cada orientação, não só a orientação 5 mostrada como exemplo.
2. O rasterizador (`mutool`/`ghostscript`, o que estiver a montar a imagem final) usar amostragem diferente conforme o ângulo de rotação da matriz — isto seria uma diferença de ferramenta de comparação, não do cristalino/vanilla em si, e mudaria a interpretação de todo o passo.

```bash
mutool trace /tmp/p777-cristalino-orient6.pdf | grep -A3 "cm /"
lab/typst-original/target/release/typst compile <mesmo documento orientação 6> /tmp/p778-vanilla-orient6.pdf
mutool trace /tmp/p778-vanilla-orient6.pdf | grep -A3 "cm /"
```

Comparar as matrizes com mais casas decimais que P776 mostrou (P777 já aumentou a precisão para 5 casas — confirmar se isso reduziu ou não a diferença de matriz em si, separado do AE final).

---

## Critério de fecho do passo

- [ ] Confirmado se o ICC teve efeito mensurável na imagem renderizada (Passo 0) — se não, registar que a atribuição de P776/P777 ao color space estava incorrecta desde o início.
- [ ] Confirmado por leitura de código se há alguma reamostragem de pixels no caminho actual (deveria não haver, dado P776 usar matriz pura) — se houver, é uma regressão a corrigir; se não houver, a hipótese de "interpolação" cai e a causa tem de ser outra.
- [ ] Matrizes `cm` comparadas com precisão total (não arredondadas) para as orientações com resíduo (2,3,5,6,7,8) e sem (1,4).
- [ ] Causa real do padrão 195/138/0 identificada com evidência, não suposição.
- [ ] Se causa for correção do rasterizador/ferramenta de comparação (não do código do projecto): registar isso explicitamente — não é mais um bug de paridade, é uma característica de como `mutool` desenha rotações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p778.md`, com a comparação antes/depois do ICC e a causa final.

---

## Próximo passo

Se causa real encontrada no código do projecto: P779, correcção focada.
Se a causa for do rasterizador de comparação (não do cristalino/vanilla): fechar a linha de imagem (P769-P778) com o resíduo documentado como artefacto de medição, não de paridade, e não abrir mais passos para isto.
