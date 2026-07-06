---
# P586 — Re-testar o documento de referência RTL depois das correcções de `font_size_pt`

> **Passo:** 586
> **Data:** 2026-07-05
> **Foco:** P577 mediu sobreposição de glifos no documento `الكتاب 42 على الطاولة` (árabe com um número no meio). P578 mostrou que a causa não era específica de RTL — era `font_size_pt` estático a dar altura de linha errada, corrigido depois em P579, P580, e P582. Desde então, ninguém voltou a testar o documento original de RTL para confirmar se o problema desapareceu por completo, ou se sobra ainda o segundo ponto que P578 deixou em aberto — `align_current_line_rtl()` a ser chamado de forma independente em `flush_line()` e em `finish()`.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P577 (medição original da sobreposição), P578 (causa geral encontrada), P579/P580/P582 (correcções do `font_size_pt` estático).

---

## Verificação

### Repetir exactamente o documento e o método de P577

```bash
cat > /tmp/p586-rtl.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p586-rtl.typ /tmp/p586.pdf
pdftotext -tsv /tmp/p586.pdf /tmp/p586.tsv
lab/typst-original/target/release/typst compile /tmp/p586-rtl.typ /tmp/p586-vanilla.pdf
pdftotext -tsv /tmp/p586-vanilla.pdf /tmp/p586-vanilla.tsv
mutool draw -o /tmp/p586.png -r 150 /tmp/p586.pdf
```

Construir a mesma tabela de posições (esquerda, topo, largura, por palavra) já usada em P563/P564/P566/P567/P569/P574/P577. Comparar directamente com a tabela de P577, que tinha a sobreposição.

### Perguntas a responder

1. As quatro palavras ficam todas na mesma linha, com o mesmo `top`?
2. A ordem visual está correcta (`الطاولة`, `على`, `42`, `الكتاب`, da esquerda para a direita)?
3. Há sobreposição visual na imagem, confirmada com `mutool draw`, ou não?
4. Se ainda houver algum problema: é o mesmo tipo de sobreposição vertical que P577 mediu, ou é outra coisa — o ponto que P578 deixou como pergunta em aberto (dois alinhamentos independentes em `flush_line()` e `finish()`)?

### Critério de fecho

- [ ] Tabela de posições construída e comparada com P577.
- [ ] Confirmado se a sobreposição desapareceu por completo.
- [ ] Se não desapareceu: confirmado se é o ponto em aberto de P578 (dois alinhamentos independentes), com a mesma disciplina de não assumir sem testar.

---

## Decisão

Se a sobreposição desapareceu e as posições batem com o vanilla, dentro de um limiar pequeno: a sequência de RTL, desde P484 até aqui, fica finalmente fechada, com prova, não com afirmação.

Se ainda sobrar algum problema: escrever o passo seguinte com a causa exacta, seguindo o mesmo método já estabelecido — sonda, prova, sem hipótese aceite sem teste.

---

## Critério de fecho do passo

- [ ] Verificação completa, com tabela de posições e imagem.
- [ ] Decisão registada: sequência RTL fechada, ou ponto específico ainda em aberto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p586.md`, com hash do commit.
- [ ] Tabela de estado da sequência RTL actualizada pela última vez, se fechar.
