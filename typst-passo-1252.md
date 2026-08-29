# P1252 — preservar alpha Luma e documentar defeito upstream

**Estado:** FECHADO — `KNOWN-UPSTREAM-BUG` REVALIDADO; LUMINÂNCIA FECHADA POR P1239
**Predecessor:** P1236
**Escopo:** classificação da divergência não-Luma → Luma; sem promoção SVG.

## Objetivo

Fechar a divergência de alpha descoberta pelo P1236 sem copiar para o
cristalino a perda de transparência observada no vanilla ratificado
`a51e02804`. Documentar separadamente a paridade estrita, a intenção pública
do Typst e a escolha de engenharia do cristalino.

## Medição antes da decisão

Registrar, com commit/working tree, hora, comandos e hashes dos outputs:

1. `red.transparentize(60%)` possui alpha `40%` nos dois sistemas;
2. ao converter esse stop para o mixing space Luma, o vanilla devolve
   `(54.02%, 100%)` e o cristalino `(54.01%, 40%)`;
3. um stop que já nasce Luma mantém `40%` nos dois sistemas;
4. a fonte ratificada usa `Luma::from_color` somente na conversão entre
   spaces, enquanto o caso já-Luma retorna o próprio valor;
5. o PR upstream `typst/typst#3438` adicionou deliberadamente alpha a Luma;
6. o PR `typst/typst#4424` corrigiu outra falha de Luma, sem decidir esta;
7. a busca no rastreador oficial não encontrou issue específica para a perda
   de alpha durante não-Luma → Luma.

## Decisão a materializar

- Classificar a perda de alpha do vanilla como `Known-Upstream-Bug`, não como
  requisito de linguagem a copiar.
- Preservar no cristalino o alpha em conversões para Luma, inclusive na
  normalização de stops dos três constructors de gradient.
- Não alterar `Color::to_space`, `interpolate_luma` nem os constructors para
  forçar opacidade.
- Manter a divergência visível no mapa DSM como extra/correção cristalina,
  com ligação à evidência P1236 e ao eventual número de issue upstream.
- Separar o delta de luminância `54.01%` vs `54.02%`: na medição original ele
  era gap de precisão e não podia ser perdoado pela decisão de alpha. P1239
  corrigiu depois a fórmula L1 e fechou esse delta; essa resolução posterior
  não altera a classificação de alpha do P1252.

## Ataques obrigatórios

1. Mutante que força `alpha = 1.0` em todo stop Luma deve morrer.
2. Mutante que preserva alpha apenas quando a origem já é Luma deve morrer.
3. Mutante que usa a classificação de alpha para perdoar o delta de
   luminância deve morrer.
4. Mutante que promove qualquer par SVG por causa desta decisão deve morrer.
5. Mutante que trata ausência de issue como prova de comportamento correto
   deve morrer.

## Saídas

- diagnóstico P1252 com proveniência completa;
- entrada explícita no mapa DSM e na fila de implementação;
- teste normativo de preservação de alpha, se o owner L0 vigente ainda não o
  cobrir;
- referência à issue upstream caso o dono decida abri-la fora deste passo.

## Gate de fechamento

O passo fecha somente se alpha preservado estiver protegido, o delta de
luminância continuar aberto separadamente, nenhuma whitelist SVG mudar e o
mapa distinguir `Known-Upstream-Bug` de `Preserved`/`Unknown`.

Executar com materialização segregada. Se o ambiente não provar independência
forte, registrar literalmente `EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`.

## Resultado

- contrato público `luma(lightness, alpha: alpha)` e `luma(color)` materializado após
  confirmação humana;
- alpha preservado em `Color::to_space(Luma)` e nos stops Linear, Radial e
  Conic, protegido por testes públicos;
- cinco ataques obrigatórios rejeitados (`5/5`);
- delta histórico de luminância `54.01%` vs `54.02%` posteriormente resolvido
  pelo P1239, com 42/42 pares sem delta;
- whitelist SVG mantida sem Luma e sem nova promoção;
- evidência em `00_nucleo/diagnosticos/typst-p1252-luma-alpha.md` e artefactos
  TSV P1252.

Proveniência da medição inicial: HEAD
`697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`, working tree não commitada,
`2026-08-27T16:36:44-03:00`; fonte SHA-256
`e27b95523a535a1d68a27aaa62bd9462ee401edeefa8a76b43178c0c2f8ec9bf` e
texto vanilla SHA-256
`83ec0e82fda0faff795e079959caaa587bbb58889958e59832805449dbcab1fe`.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.

## Saneamento posterior ao P1239/P1251

O resultado original misturava duas dimensões independentes. O estado vigente
é: perda de alpha no vanilla continua `Known-Upstream-Bug`; luminância foi
corrigida e fechada por P1239 no fragmento Linear/Radial P1236; SVG Luma
continua `Unknown`. Os hashes do manifesto original identificam a primeira
execução e são históricos. A revalidação usa os hashes L0 atuais e publica
certificado aditivo, sem reescrever a proveniência antiga como se fosse atual.

## Revalidação executada — 2026-08-28

Os quatro testes P1252 passaram. O runner P1239 foi executado duas vezes com
saídas byte-idênticas e confirmou o summary vigente: 42/42 pares
Linear/Radial sem delta de luminância, preservando separadamente a divergência
de alpha do vanilla. Build do workspace, formatação, `git diff --check`,
V1/V5/V15/V26 e lint completo passaram.

O lint normal terminou com exit 0 e registrou V16=211, V19=358 e V20=635 sob
a política vigente. Nenhuma whitelist SVG foi alterada. O certificado
`00_nucleo/diagnosticos/p1252-revalidation-certificate.tsv` congela hashes,
comandos, estado da árvore e escopo do veredito.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
