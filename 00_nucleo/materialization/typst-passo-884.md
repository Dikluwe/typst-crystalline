# Prompt — typst-passo-884: comprimir e reduzir a verbosidade dos content streams (frente grande de `06-long`)

**Origem**: sonda de P883 — o PDF cristalino de `06-long` tem 51 content streams somando 970 910 bytes descomprimidos; comprimidos com zlib caem para ~133 770 bytes (~13,78% do original, ganho potencial de ~837KB). Isso domina completamente a diferença de tamanho contra o vanilla (6.62×) — muito maior que qualquer ganho já obtido em fonte (P874/P882/P883, juntos menos de 2KB por documento).
**Estado**: aguardando execução — duas frentes dentro do mesmo problema, com pesos e riscos diferentes.

---

## As duas frentes, não confundir

1. **Compressão dos content streams com FlateDecode** — mesmo mecanismo já usado em outros streams do exportador (imagens, e agora fonte desde P883). Baixo risco: não muda o conteúdo do stream, só como ele é armazenado no arquivo. Ganho estimado pela sonda de P883: a maior parte dos ~837KB.
2. **Redução de verbosidade dos operadores PDF em si** (P883 notou: `BT...ET` repetido por bloco de texto, coordenadas com 3 casas decimais) — isso muda o conteúdo do stream antes da compressão. Risco maior: pode ter efeito colateral em precisão de posicionamento se cortar casas decimais, ou exigir reestruturação de como blocos de texto são agrupados. Ganho adicional sobre a compressão, mas não estimado ainda.

Fazer a frente 1 primeiro — é a maior parte do ganho, menor risco, e a mesma técnica que P883 já provou funcionar para fonte. A frente 2 só depois, com sonda própria do tamanho real do ganho adicional antes de decidir se vale o risco.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal. Contagem de testes discriminada por crate. Validação em poppler e ghostscript obrigatória (mesmo padrão de P874/P882/P883, já que isso mexe em como o PDF é estruturado).

## Passo 1 — Comprimir os content streams (frente 1)

1. Localizar onde os content streams de página são escritos no exportador (`03_infra/src/export/`, provavelmente `builder.rs` ou `stream.rs`) — confirmar se já existe o mecanismo `compress_zlib` usado por P883 para fonte, reaproveitável aqui.
2. Aplicar a mesma compressão (`/Filter /FlateDecode`) aos content streams de página, com o mesmo fallback de segurança que P883 usou (se a compressão falhar, emitir sem comprimir, PDF continua válido).
3. Confirmar se o vanilla já faz isso (provavelmente sim, dado que os PDFs vanilla são consistentemente menores em todos os cenários do benchmark) — não é preciso replicar exatamente como o vanilla comprime, só produzir um resultado válido e comprimido.

## Passo 2 — Validação da frente 1

1. Repetir a medição de `06-long` (tamanho de PDF) — confirmar que a razão cristalino/vanilla cai substancialmente, próxima da estimativa da sonda de P883 (de 6.62× para algo bem menor, já que ~837KB dos ~1MB atuais deveria desaparecer).
2. Validar em poppler (`pdftoppm`, `pdftotext`) e ghostscript (`gs`) — confirmar que o PDF comprimido abre e renderiza igual ao não-comprimido.
3. Confirmar que os outros seis cenários do benchmark não regridem em tamanho nem em tempo (comprimir tem custo de CPU — medir se isso afeta o tempo de compilação, especialmente em documentos maiores).
4. Repetir para todos os sete cenários do benchmark de P872/P880, não só `06-long` — content streams existem em qualquer documento com mais de uma página ou com conteúdo suficiente.

## Passo 3 — Sonda da frente 2 (redução de verbosidade), sem implementar ainda

1. Depois da compressão implementada, medir quanto do tamanho residual (se ainda houver diferença significativa contra o vanilla) vem de fato de verbosidade de operadores, não só de falta de compressão — repetir uma comparação de estrutura de content stream (não só tamanho) entre cristalino e vanilla para `06-long`, contando operadores `BT`/`ET` e comparando a precisão numérica usada.
2. Se o ganho estimado for pequeno (compressão já resolveu a maior parte), registrar isso e não prosseguir — não vale o risco de mexer em precisão de coordenadas por um ganho marginal.
3. Se o ganho estimado for grande o suficiente para justificar, registrar como achado separado com escopo e risco definidos, para um passo futuro dedicado — não implementar dentro deste mesmo passo, dado o risco maior envolvido.

## Relatório

`00_nucleo/diagnosticos/typst-passo-884-relatorio.md` com: a implementação da compressão de content streams (Passo 1), a validação completa nos sete cenários incluindo poppler/ghostscript (Passo 2), e a sonda da frente 2 com uma recomendação clara de prosseguir ou não num passo futuro (Passo 3) — sem implementar a frente 2 aqui.
