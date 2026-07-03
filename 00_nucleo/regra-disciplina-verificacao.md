# Regra — disciplina de verificação, não confiança

**Data:** 2026-07-03
**Aplica-se a:** todo o projecto, todos os passos.

---

## O que foi observado

Ao longo dos passos deste projecto, não houve casos de dados inventados ou resultados falsos. Houve casos de outro tipo:

1. **Explicação aceite sem medição.** Um sintoma é atribuído a uma causa plausível, mas essa causa não é testada. Exemplo: P534/P538e atribuíram a quebra de linha errada a "não é objectivo deste passo", duas vezes, sem nunca confirmar a causa suposta.
2. **Verificação pedida no passo, não feita no relatório.** O passo pede um método específico (o script de benchmark oficial), o relatório usa outro (medição manual). Exemplo: P544.
3. **Falta de cruzamento entre passos próximos no tempo.** Dois passos investigam sintomas parecidos, ao mesmo tempo, sem se referirem um ao outro. Exemplo: P547 atribuiu um espaçamento errado à fonte, no mesmo dia em que P546 estava a confirmar que o mesmo padrão era um bug real.
4. **Âmbito mais estreito do que o pedido, sem isso ser dito directamente.** O relatório declara "corrigido" quando só uma parte do problema foi tratada, mesmo que a checklist do próprio relatório mostre isso, por marcar. Exemplo: P537.

Ao mesmo tempo, quando a tarefa pede um número concreto e uma comparação directa, o resultado é rigoroso — P546 e P549 são exemplos disto: posições reais medidas, não impressões.

## Regra

Uma tarefa ampla ("corrigir X") deixa espaço para escolher o caminho mais fácil sem que isso seja falso — só é uma leitura frouxa do que foi pedido. Uma tarefa estreita, com um número concreto a comparar contra uma referência, não deixa esse espaço.

A partir de agora, qualquer passo deste projecto segue estas quatro regras:

### 1. Se o passo pede um método específico de medição, o relatório mostra o resultado desse método

Não um método equivalente, não um resumo do que "provavelmente" o método diria. Se o passo pede `benchmark-p507.py`, o relatório mostra a saída desse script, não uma medição manual em substituição.

### 2. Toda a explicação de um sintoma vem acompanhada de uma comparação directa contra o vanilla

Não "isto é provavelmente X". Sim "isto é X, confirmado assim: [comando, número, comparação]". Se não houver tempo ou dados para confirmar, o relatório diz isso — "não confirmado, hipótese não testada" — em vez de escrever a hipótese como se fosse facto.

### 3. Antes de fechar um passo, verificar se algum outro passo recente tocou o mesmo sintoma

Uma procura simples no directório de relatórios (`grep` pelo sintoma, não pelo nome do passo) antes de escrever a conclusão. Se outro passo próximo no tempo mexeu na mesma área, ler esse relatório antes de decidir.

### 4. "Corrigido" só se aplica ao que foi de facto testado, com o mesmo texto do problema original

Se o problema original era `#set page(columns: 2)` e o teste final usa `#columns(2)[...]`, o relatório não diz "corrigido" — diz "corrigido para X, não testado para Y, que era o caso original".

## Como isto se liga à regra anterior

A regra "nenhum item aceite é permanente sem decisão nova" (ADR anterior) trata do que fica registado como aceite. Esta regra trata de como se chega a essa decisão — não é suficiente decidir de novo se a nova decisão também for feita sem medição.

---

## 5. Verificação de saída tipográfica: texto extraído não basta

Quando um passo envolve kerning, tracking, posicionamento de glifos ou qualquer ajuste fino de texto, `pdftotext` e a aparência do texto extraído são indicadores fracos. Podem parecer correctos quando o posicionamento real está errado, ou incorrectos quando o posicionamento está certo (ex.: diferenças de subsetting ou mapeamento ToUnicode).

Regras concretas:

1. **Verificar os números internos do operador PDF `TJ`** com `mutool show <pdf> <obj> | grep TJ` ou equivalente. O sinal e a ordem de grandeza dos deltas devem ser comparáveis aos do vanilla para o mesmo documento e a mesma fonte.
2. **Verificar as posições reais dos glifos** com `mutool trace <pdf>` ou ferramenta equivalente. Comparar coordenadas `x`/`y` e avanços (`adv`) carácter a carácter contra o vanilla.
3. **Não confiar apenas em texto extraído** (`pdftotext`, `mutool draw` visual) como prova de que o posicionamento está correcto. Use-o como teste de morfologia/linguagem (ADR-0107), não como prova mecânica.
4. **Desconfiar de compensações de erros.** Se dois problemas distintos (ex.: medição de largura sem kerning + delta `TJ` com sinal errado) se cancelam no texto extraído, o relatório deve assinalar essa compensação e confirmar cada erro separadamente, com números.
5. **Confirmar a mesma fonte nos dois lados.** Antes de comparar `TJ` ou posições, verificar com `mutool info <pdf> | grep Fonts` que cristalino e vanilla estão a usar a mesma família (e preferencialmente o mesmo ficheiro `.ttf`).

Exemplo de aplicação: P549 confirmou que a troca de sinal do delta `TJ` em P548 (`nominal - x_advance`) reproduz as posições reais do vanilla, enquanto a fórmula anterior de P520/P521 (`x_advance - nominal`) estava errada desde o início — mesmo que o texto extraído de P548 já parecesse correcto.
