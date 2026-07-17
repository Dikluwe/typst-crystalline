---
# P590 — Remedir o documento de referência RTL com fonte neutra

> **Passo:** 590
> **Data:** 2026-07-05
> **Foco:** P589 criou a ferramenta (`documento_algoritmo`, fonte neutra `DejaVu Sans`) e catalogou cerca de 50 documentos de teste da sequência RTL sem fonte explícita, sem os remedir. Este passo usa a ferramenta no documento mais importante da lista — o de referência, usado desde P563 até P588 — para responder, de uma vez, se o algoritmo de RTL está certo, sem ruído de fonte.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Aplica-se a ADR de paridade de definições por defeito.
> **Dependências:** P589 (ferramenta criada), P563 a P588 (histórico de medições do mesmo documento, com fonte não neutra).

---

## Verificação

### Documento de teste, agora com fonte neutra

```bash
cat > /tmp/p590-rtl-neutro.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt, font: "DejaVu Sans")
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p590-rtl-neutro.typ /tmp/p590.pdf
pdftotext -tsv /tmp/p590.pdf /tmp/p590.tsv

cat > /tmp/p590-rtl-neutro-vanilla.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt, font: "DejaVu Sans")
الكتاب 42 على الطاولة
EOF
lab/typst-original/target/release/typst compile /tmp/p590-rtl-neutro-vanilla.typ /tmp/p590-vanilla.pdf
pdftotext -tsv /tmp/p590-vanilla.pdf /tmp/p590-vanilla.tsv
```

### Perguntas a responder

1. As quatro palavras ficam na mesma linha, nos dois lados?
2. A ordem visual está correcta?
3. As posições (esquerda, topo, largura) batem certo dentro de um limiar pequeno (por exemplo, menos de 1 ponto), agora que a fonte é a mesma dos dois lados?
4. Se ainda houver alguma diferença: é de algoritmo, ou aparece mais alguma coisa por trás — não assumir que "fonte neutra" resolve tudo sem confirmar.

### Critério de fecho

- [ ] Tabela de posições construída, fonte neutra dos dois lados.
- [ ] Comparação directa com o vanilla, número a número.
- [ ] Decisão registada: o algoritmo de RTL está correcto, com prova sem ruído de fonte, ou ainda sobra uma diferença a explicar.

---

## Decisão

Se as posições baterem certo dentro de um limiar pequeno: a sequência de RTL, do início (P484) até aqui, fica finalmente fechada quanto ao algoritmo, com a fonte já não a ser motivo de dúvida.

Se ainda sobrar diferença: essa diferença já não pode ser atribuída à fonte — é outra coisa, a investigar com o mesmo método já estabelecido nesta sequência.

---

## Critério de fecho do passo

- [ ] Medição feita com fonte neutra dos dois lados.
- [ ] Decisão registada, com número, não com suposição.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p590.md`, com hash do commit.
- [ ] Tabela de estado da sequência RTL fechada pela última vez, se confirmar.
