# Prompt — typst-passo-811: `frak()` sem argumento corrompe o PDF gerado

**Origem**: achado #13 de P810 (`math::style`), destacado isoladamente por ser o item mais grave da fila — não é divergência de comportamento, é geração de ficheiro inválido
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> `frak()` sem arg + **PDF corrompido**

Contexto adicional do mesmo achado, para orientar a sonda: o achado #13 no geral cobre `display`/`inline`/`script`/`sscript` sem efeito geométrico, itálico de P809 perdido em wrappers de tamanho, `scr` com bloco Unicode errado, e `NN`/`RR`/`ZZ`/`QQ`/`CC` ausentes. Este prompt cobre **só** a corrupção do PDF — os outros pontos de `math::style` ficam para um passo separado (não misturar, para poder validar e fechar este primeiro).

---

## Por que isto tem prioridade sobre o resto da fila

Um documento que produz um PDF corrompido falha de um jeito que o utilizador final só descobre ao tentar abrir o ficheiro — pior do que qualquer divergência de texto ou geometria, que pelo menos produz um ficheiro válido com conteúdo errado.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Cada afirmação do relatório tem de vir com comando exacto + saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer e bater com os testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Reproduzir com `#frak()` (chamada sem argumento) — compilar com os dois binários. Registar o comportamento de cada um: o vanilla deve rejeitar com erro de argumento em falta (`missing argument: body` ou equivalente, como já visto no padrão de `#par()` em P806); confirmar se o cristalino também rejeita na fase de avaliação ou se deixa passar e só corrompe o PDF na fase de export.
2. Se o cristalino já rejeita `#frak()` sem argumento na avaliação (erro de compilação, sem chegar a gerar PDF), então "PDF corrompido" não se refere a este caso isolado — reler o `.typ` original usado no achado #13 do relatório de materialização de P810 (`00_nucleo/materialization/typst-passo-810-relatorio.md`) para confirmar o caso exacto que gerou o PDF corrompido (pode ser `frak()` dentro de outro contexto, ex.: dentro de `$...$` vazio, ou combinado com outro elemento). **Não prosseguir com uma hipótese não confirmada** — usar o `.typ` exacto do relatório de materialização.
3. Depois de reproduzir o caso exacto: verificar se o PDF gerado pelo cristalino é sintaticamente inválido (`mutool clean` ou `qpdf --check` ou equivalente já usado no projecto) e registar a saída literal do verificador.
4. Localizar no código do cristalino o caminho de export/layout que trata `frak()` (ou o wrapper de estilo matemático de letra única que `frak()` usa — mesma família de `apply_math_style`/`map_glyph` tocada em P809) e identificar o ponto exacto que produz a estrutura inválida (ex.: stream com comprimento declarado errado, objecto referenciado que não existe, glifo fora do intervalo do subset).
5. Localizar o comportamento equivalente no vanilla (rejeição na avaliação, ou se aceitar, como gera o PDF sem corromper) e registar os dois pontos antes de tocar em código.

## Passo 2 — Implementação

Depende do Passo 1: se o caso correcto é `frak()` sem argumento devendo ser erro de avaliação (como `par()` sem body), implementar essa validação — mais barato e mais seguro do que corrigir o export para tolerar o caso. Se o vanilla aceita o caso e produz PDF válido, corrigir a rotina de export identificada no Passo 1.4 para não gerar a estrutura inválida.

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir o comando do Passo 1 e mostrar a saída literal — se a correcção for de validação, mostrar o novo erro; se for de export, mostrar o verificador de PDF a passar.
3. Adicionar caso de teste cobrindo o caso exacto reproduzido.
4. Correr a suíte `typst-core` completa (e a suíte de export/infra, se o ponto de correcção for lá) e mostrar comando + contagem de testes antes/depois.
5. Confirmar que os outros usos de `frak()` com argumento válido continuam a funcionar (controlo de regressão, ex.: `frak(A)`).

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-811-relatorio.md` com:
- O `.typ` exacto reproduzido, confirmado contra o relatório de materialização de P810.
- Comando + saída literal do Passo 1 (antes da correcção), incluindo a saída do verificador de PDF.
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
- Nota explícita de que os outros pontos do achado #13 (`display`/`script`/`sscript`, itálico em wrappers de tamanho, `scr`, `NN`/`RR`/`ZZ`/`QQ`/`CC`) continuam pendentes, não cobertos por este passo.
