# P1381 — preflight de materialização

Regime: protocolo completo **sem atestação de isolamento**. A mesma autoridade
possui contrato, teste, candidato, ataques e veredito; os hashes comprovam
ordem e identidade, não independência.

A medição bilateral precedeu a decisão. A hipótese congelada espelha três
escolhas escalares já observáveis em `cursor::flush_line`: top-edge inicial do
primeiro texto, ascent/descent do maior estilo com fallback exterior e leading
do último texto. A largura continua resolvida por fragmento e fecha o caminho
se houver wrap.

Ordem: L0/contrato/oráculos/mutantes → RED focal → candidato → GREEN focal →
matriz bilateral → resselo/lint. Budget: duas revisões focais; duas falhas da
mesma causa reabrem o desenho. Processos sempre seriais nos limites do passo.

Não são autorizados recursão, Vec auxiliar, reconstrução de Content, simulação
de layout ou ampliação além de `text.size`.

Revisão v2: a candidata v1 fechou as quatro páginas, mas deslocou `One` em
todos os casos. A fonte vanila mostrou que um primeiro fragmento maior altera
o estilo compartilhado do parágrafo e o colapso entre seu spacing e o `above`
do Block. Sem carrier para esse estilo, P1381 reclassifica primeiro-fragmento
maior como `Unknown`. O caso pequeno e o aumento posterior permanecem
decidíveis por métricas escalares. As diferenças internas de frame também não
são usadas como substituto de `yMin`; o oráculo final continua sendo o PDF.

Reabertura v3: a matriz da candidata v2 provou `small-prefix` exatamente, mas
o misto manteve `Lead` e `More` 4,606pt acima do vanila apesar de acertar
`One`, `After` e `Tail`. Encerrado o budget de duas revisões, o contrato foi
reduzido ao único fragmento textual menor ou igual ao exterior. Não há terceira
tentativa de fórmula; composto com mudança de tamanho permanece `Unknown`.
