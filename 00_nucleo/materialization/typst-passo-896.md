# Passo 896 — centragem e numeração de equação quebradas sob `width: auto` com conteúdo de largura variável

**Precede este passo**: `typst-passo-895-relatorio.md`, Parte A, Fase A, ponto 2 (registo explícito
de fora de escopo) e a nota de "numeração: scope-out" na secção de implementação. Ler antes de
começar — os dois problemas abaixo já estavam previstos, não são descoberta nova.

**Confirmado pelo dono do projeto, visualmente**, comparando `test_crystalline.pdf`/
`test_vanilla.pdf` (as 30 secções completas, mesmo `.typ` de P894/895, agora sem a corrupção de
página): centragem de fórmulas errada no cristalino, e posição de algumas linhas/símbolos também.
Cruzando com o texto extraído dos dois PDFs: a partir da secção 20
(`#set math.equation(numbering: "(1)")`), o vanilla numera todas as equações seguintes (`(1)` a
`(44)`, até à secção 30); o cristalino não mostra número nenhum em nenhuma equação depois disso.

**Pré-condição de árvore**: `git status`. P893 continua parado no gate (STOP, Fase B não iniciada).
P894 (fix de `dot`) e P895 (offset_x infinito + catálogo de terceiros) — confirmar se já foram
commitados.

---

## Por que isto não é um bug pontual — é a arquitetura de uma passagem só a mostrar o limite

`page_config.width` só é resolvido para um valor finito em `finish()`/`new_page()` (confirmado em
P895) — **depois** de todo o conteúdo da página já ter sido posicionado. Para uma equação isolada,
isso degenera para "sem efeito" (a largura final acaba sendo a da própria equação, logo qualquer
fórmula de centragem dá zero). Para uma página com várias equações de larguras diferentes — o caso
real deste ficheiro de teste — isso não degenera: a largura final da página é determinada pela
equação **mais larga**, e todas as outras deveriam ficar centradas em relação a essa largura final,
não à margem. P895 corrigiu a corrupção (`infinito`) trocando por "fica na margem", que evita o bug
visível mas não produz centragem real neste caso — correto para o caso degenerado, incorreto para o
caso geral, e já foi registado assim.

O mesmo motivo explica a numeração: o número da equação (à direita, `right_x = regions.current.width
- margin - number_width`) também precisa da largura final da página, que não existe ainda no momento
em que a equação é posicionada — P895 escolheu não posicionar o número nesse caso, em vez de usar um
valor ainda não resolvido.

## Fase A — desenhar a solução (não implementar ainda)

1. Confirmar como o vanilla resolve isto arquitecturalmente (`lab/typst-original/`) — já mencionado
   em P895 como "modelo de duas passagens" (medir tudo, depois posicionar com a largura final
   conhecida), mas sem detalhe de implementação. Ler o código real: onde a primeira passagem
   acontece, o que ela mede, como o resultado é usado na segunda.
2. Avaliar até que ponto essa arquitetura de duas passagens é viável no cristalino sem uma reescrita
   grande do layout de página. Três desenhos candidatos, a avaliar por custo/risco antes de escolher:
   - **(a) Duas passagens completas**: replicar a arquitectura do vanilla. Provavelmente o mais
     correto, mas potencialmente o de maior risco/escopo — mexe na sequência geral de layout de
     página, não só em `equation.rs`.
   - **(b) Pré-scan localizado**: antes de posicionar qualquer equação de bloco numa página `auto`,
     fazer uma passagem leve só para medir a largura máxima de conteúdo de bloco nessa página
     (equações, e potencialmente outros elementos de bloco — confirmar quais outros tipos de
     conteúdo também são afectados por `width: auto`, não presumir que é só equação), e usar esse
     valor para centragem/numeração. Mais contido, mas pode não cobrir every caso que a arquitectura
     completa cobriria (por exemplo, se a largura final também for afectada por texto normal, não só
     blocos).
   - **(c) Passagem de correção pós-layout**: manter o posicionamento actual (na margem/sem número),
     e no fim de `finish()`/`new_page()` (quando a largura final já é conhecida), percorrer os itens
     da página e corrigir a posição de equações de bloco e números — um "segundo passe" mais barato
     que (a), aplicado só a estes dois casos, sem generalizar para outros tipos de conteúdo.
3. Escolher um dos três (ou propor um quarto) com justificação explícita de custo/risco/cobertura,
   registada no relatório. Não implementar sem essa escolha estar registada e, se mexer em L0s
   estruturais (sequência geral de layout de página), confirmar com o dono antes de prosseguir —
   mesmo gate dos passos anteriores.
4. Confirmar, com um caso de teste mínimo (2-3 equações de bloco de larguras bem diferentes, `width:
   auto`), qual é o resultado esperado exacto do vanilla (posições numéricas, não só "parece
   centrado") — usar como alvo de teste na Fase B.

## Fase B — Implementação (TDD, per `CLAUDE.md`)

Só depois da Fase A escolher o desenho.

1. Teste que falhe primeiro: caso mínimo da Fase A ponto 4, confirmando posições exactas (não só
   "não é infinito", que já P895 garante).
2. Implementar o desenho escolhido.
3. Suíte completa verde, discriminada por crate.
4. Recompilar o `.typ` completo de 30 secções (mesmo hash de P894/895) e confirmar visualmente que
   a centragem geral melhora, e que a numeração volta a aparecer a partir da secção 20. Comparar
   secção a secção com o vanilla onde for prático, não só as 2-3 primeiras.
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Esta correção mexe potencialmente na sequência geral de layout de página (dependendo do desenho
escolhido na Fase A) — benchmark completo, 7 cenários, atenção a `06-long` e `07-context` além de
`04-math`, já que qualquer mudança na resolução de largura de página pode afectar mais que só
equações.

## Nota separada — "posição de algumas linhas e símbolos"

O dono reportou isto também, sem apontar caso específico. Antes de investigar como achado à parte,
verificar se desaparece depois da correcção de centragem acima — é possível que seja o mesmo sintoma
visto de outro ângulo (conteúdo deslocado por estar ancorado à margem em vez de centrado). Se
persistir depois da Fase B, catalogar como achado novo, com caso mínimo isolado, mesma disciplina de
sempre — não presumir a causa sem isolar.

## Resultado esperado

- Relatório da Fase A com os três desenhos avaliados, escolha justificada, confirmação (ou pedido de
  confirmação, se tocar L0 estrutural) do dono antes da Fase B.
- Teste(s) novo(s) com posições exactas, não só ausência de infinito.
- Confirmação visual no documento completo de 30 secções, secção a secção onde prático.
- Veredicto sobre se "posição de linhas/símbolos" era o mesmo sintoma ou achado novo.
- Benchmark completo da Fase C.
