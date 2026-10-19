---
# P746 — As coordenadas do PDF são idênticas, ou há uma diferença de precisão real por trás do "anti-aliasing"?

> **Passo:** 746
> **Data:** 2026-07-10
> **Foco:** P745 confirmou que o padrão do diff residual (~0,14%) é consistente com anti-aliasing (concentra-se em fronteiras, diminui com a resolução), mas não confirmou se a **causa** é só o rasterizador a variar, ou se os dois PDFs têm coordenadas geometricamente diferentes que o anti-aliasing torna visíveis. Estas são duas explicações distintas — só a segunda envolve uma pergunta de "qual está mais correcto".
> **Tipo:** Sonda directa.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P745 (onde o padrão foi confirmado, mas não a causa exacta).

---

## Sonda

### Extrair e comparar directamente as coordenadas do content stream

```bash
cat > /tmp/p746-nativo.typ <<'EOF'
#line(start: (0pt, 0pt), end: (100pt, 50pt))
#circle(radius: 30pt)
EOF
./target/release/typst /tmp/p746-nativo.typ /tmp/p746-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p746-nativo.typ /tmp/p746-vanilla.pdf

mutool show /tmp/p746-cristalino.pdf 4 > /tmp/p746-cristalino-stream.txt 2>&1
mutool show /tmp/p746-vanilla.pdf 4 > /tmp/p746-vanilla-stream.txt 2>&1
diff /tmp/p746-cristalino-stream.txt /tmp/p746-vanilla-stream.txt
```

Usar o número de objecto correcto para o content stream em cada PDF (pode não ser `4` nos dois — confirmar primeiro com `mutool show <pdf> trailer` ou inspecção da árvore de páginas).

Comparar as coordenadas exactas dos operadores `m`/`l`/`c` (moveto/lineto/curveto) para a linha e o círculo. Confirmar: são numericamente idênticas, ou diferem, mesmo que ligeiramente (por exemplo, na última casa decimal)?

### Se diferirem: confirmar a causa

Para a linha (geometria simples, fácil de verificar à mão): confirmar se as coordenadas de `(0pt,0pt)` a `(100pt,50pt)` batem exactamente com o valor matemático esperado nos dois PDFs, ou se um dos dois arredonda de forma diferente.

Para o círculo (aproximado por curvas de Bézier): confirmar quantos segmentos de Bézier cada compilador usa para aproximar o círculo, e se a fórmula de aproximação (posição dos pontos de controlo) é a mesma. Um número diferente de segmentos, ou uma fórmula de aproximação diferente, produziria um círculo "correcto" nos dois casos, mas não byte-idêntico — isso já não seria um problema de precisão, seria uma escolha de implementação diferente e legítima.

```bash
grep -oE "[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+c" /tmp/p746-cristalino-stream.txt | wc -l
grep -oE "[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+[0-9.]+\s+c" /tmp/p746-vanilla-stream.txt | wc -l
```

Contar quantas curvas Bézier (`c`) cada um usa para o círculo.

### Critério de fecho da sonda

- [ ] Coordenadas do content stream comparadas directamente, não só pixels rasterizados.
- [ ] Confirmado se as coordenadas são idênticas, ou diferem.
- [ ] Se diferirem: causa identificada (arredondamento, número de segmentos de Bézier, fórmula de aproximação).

---

## Decisão

Se as coordenadas forem **idênticas**: a diferença é 100% do rasterizador (`mutool`), sem nenhuma questão de "qual é mais correcto" — os dois compiladores produzem exactamente o mesmo desenho vectorial, e a resposta à pergunta original é "nenhum dos dois, o PDF é idêntico".

Se as coordenadas **diferirem** por arredondamento simples (por exemplo, `50.0` vs `49.9999997`): comparar ambos com o valor matemático exacto esperado, para responder directamente "qual está mais próximo do correcto".

Se diferirem por uma **escolha de implementação diferente e legítima** (número de segmentos de Bézier diferente para aproximar um círculo, por exemplo): não é uma questão de correcção, é uma escolha de fidelidade/desempenho que ambos os lados podem justificar — registar como divergência de mecânica (ADR-0107), não de linguagem.

---

## Critério de fecho do passo

- [ ] Sonda completa, coordenadas comparadas directamente.
- [ ] Causa da diferença identificada com precisão.
- [ ] Resposta directa à pergunta "vanilla é mais correcto?" — sim, não, ou "a pergunta não se aplica" (coordenadas idênticas), com a evidência a sustentar.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p746.md`.
