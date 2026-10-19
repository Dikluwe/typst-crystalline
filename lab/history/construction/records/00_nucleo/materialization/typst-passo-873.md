# Prompt — typst-passo-873: investigar a causa raiz de math (22×) e imagens (16×) mais lentos — começar pelo subsetting de fonte

**Origem**: P872 mediu math 22× mais lento (PDF 19× maior) e imagens 16× mais lento (PDF 7× maior). O próprio relatório já aponta a hipótese mais provável: subsetting de fonte ausente ou ineficiente. Este passo confirma ou refuta essa hipótese com medição direta, antes de qualquer tentativa de otimização.
**Estado**: aguardando execução — **isto é diagnóstico, prioridade alta**. Não otimizar sem confirmar a causa primeiro; otimizar o lugar errado é tempo perdido e pode mascarar o problema real.

---

## Por que a correlação PDF-maior ↔ mais-lento é a pista certa para seguir primeiro

Tempo de compilação de PDF tem duas fontes principais de custo: trabalho de CPU (layout, shaping, cálculo) e I/O/serialização (quanto tem que ser escrito no arquivo). Quando o tempo e o tamanho do arquivo crescem quase na mesma proporção (math: 22× tempo, 19× tamanho; ainda que imagens não bata tão exato, 16× tempo vs 7× tamanho, ainda é a maior pista disponível), a explicação mais simples é que o custo está em produzir/escrever mais bytes do que precisava, não em um algoritmo lento. Fontes matemáticas completas sem corte de glifos não usados (subsetting) são exatamente esse tipo de custo: cada equação nova, mesmo reusando a mesma fonte, pode estar arrastando a fonte inteira de novo se o subsetting não está deduplicado corretamente.

---

## Passo 1 — Confirmar (ou refutar) a hipótese de subsetting no caso de math

1. Extrair as fontes embutidas do PDF cristalino de `04-math.typ` (mesmo documento do benchmark de P872) usando `pdffonts`/`mutool extract` — comparar quantos glifos cada fonte embutida contém, e quantas vezes a mesma fonte aparece embutida (uma vez com subset correto, ou múltiplas vezes/vezes com o conjunto de glifos completo).
2. Fazer o mesmo para o PDF vanilla do mesmo documento — confirmar quantos glifos o vanilla embute (deve ser só os efetivamente usados no documento).
3. Se o cristalino estiver embutindo a fonte completa (milhares de glifos) em vez de só os usados (dezenas), isso confirma a hipótese — localizar no exportador PDF (`03_infra/src/export/`) onde o subsetting deveria acontecer e por que não está reduzindo o conjunto de glifos para matemática especificamente (o subsetting já funciona para texto comum, já que os outros cenários do benchmark não mostraram o mesmo problema — confirmar essa diferença entre o caminho de texto e o caminho de math).
4. Medir também se a mesma fonte matemática está sendo embutida **múltiplas vezes** no mesmo PDF (uma vez por equação, em vez de uma vez só, deduplicada) — isso seria uma causa adicional ou alternativa à falta de subsetting, e mudaria a correção.

## Passo 2 — Confirmar (ou refutar) a mesma hipótese para imagens

1. Mesmo procedimento: extrair as imagens embutidas do PDF cristalino de `03-images.typ` e comparar com o vanilla — confirmar se a mesma imagem repetida 50 vezes está sendo embutida 50 vezes (bytes duplicados) no cristalino, enquanto o vanilla deduplica e referencia o mesmo XObject de imagem múltiplas vezes.
2. Se for isso, é uma causa diferente da de matemática (deduplicação de recurso, não subsetting de fonte) — não assumir que as duas têm a mesma correção só porque os sintomas (lento + PDF grande) são parecidos.

## Passo 3 — Medir o custo de CPU separadamente do custo de I/O

Para isolar se o tempo extra vem de gerar mais bytes (I/O/serialização) ou de mais trabalho de cálculo (CPU) antes de qualquer correção: usar um profiler simples (`perf` se disponível no ambiente, ou até `time` decompondo user/sys/real) na compilação do documento de math, para ver se o tempo está concentrado em I/O (escrita do PDF) ou em CPU (algum cálculo repetido). Isso ajuda a confirmar qual das duas causas (subsetting ausente vs. algoritmo de layout lento) é a dominante, caso as duas estejam presentes ao mesmo tempo.

## Passo 4 — Relatório de diagnóstico (sem correção ainda)

Reportar a causa confirmada para cada um dos dois cenários (math e imagens), com números concretos (quantos glifos deveriam estar embutidos vs quantos estão, quantas vezes a imagem é duplicada, onde no código exatamente isso acontece). Não corrigir neste passo — o próximo passo (a ser escrito depois, com base no que este encontrar) faz a correção.

## Relatório

`00_nucleo/diagnosticos/typst-passo-873-relatorio.md` com: a medição de glifos embutidos (math) e duplicação de imagem (imagens), a confirmação ou refutação da hipótese de subsetting/deduplicação para cada caso, a decomposição CPU vs I/O do Passo 3, e a localização exata no código de onde a correção precisará mexer — sem implementar a correção ainda.
