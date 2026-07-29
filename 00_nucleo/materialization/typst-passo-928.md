# Passo 928 — reduzir o custo absoluto do fallback de fonte para CJK/emoji (~7 segundos)

**Este prompt foi escrito para ser lido por um chat novo, sem acesso ao histórico dos passos
anteriores. Todo o contexto necessário está aqui dentro ou nos ficheiros indicados abaixo. Não
presumir conhecimento de passos anteriores além do que está escrito nestes ficheiros.**

---

## Leitura obrigatória antes de começar, nesta ordem

1. `00_nucleo/handoff-novo-chat-p927.md` — estado geral do projecto neste momento. Ler inteiro.
2. `00_nucleo/materialization/typst-passo-925-relatorio.md` — diagnóstico original do problema
   deste passo.
3. `00_nucleo/materialization/typst-passo-926-relatorio.md` — uma tentativa de correcção que
   **não funcionou** (thread de fundo) — ler para não repetir a mesma ideia.
4. `00_nucleo/materialization/typst-passo-927-relatorio.md` — a correcção que funcionou
   parcialmente (resolveu quem não precisa pagar; não resolveu o custo de quem precisa).

**Antes de continuar, confirmar uma coisa que pode estar por resolver**: o relatório de P927 tinha
uma pendência — os ficheiros `00_nucleo/prompts/infra/system-world.md` e
`00_nucleo/prompts/infra/wiring.md` podem não estar actualizados com o que o código de P927
implementou (`preload_coverage_if_needed`, `embedded_coverage_union`, `source_text_nodes`).
Verificar com `crystalline-lint .` e, mais importante, **ler os dois L0s e comparar manualmente
com o código real de `03_infra/src/world.rs`/`04_wiring/src/main.rs`** — hash sincronizado não
prova que o conteúdo do L0 descreve o código correctamente (isto já enganou um passo anterior). Se
ainda estiver desactualizado, corrigir isso **primeiro**, antes de tocar em mais código nesta área.

---

## O problema, explicado do zero

Quando o Typst compila um documento com um carácter que a fonte principal não tem (por exemplo,
um carácter chinês ou um emoji, numa fonte pensada para texto latino), o compilador precisa de
procurar esse carácter noutra fonte instalada no computador. Essa procura, no cristalino, é lenta:
mede-se **cerca de 7 segundos** para um documento pequeno com um único carácter CJK ou emoji.

O vanilla (o Typst original, usado como referência de comparação em todo este projecto) faz a
mesma coisa em menos de meio segundo.

**P927 já corrigiu metade do problema**: antes de P927, o cristalino fazia esta procura lenta
mesmo em documentos que **não têm** nenhum carácter CJK/emoji — pagando o custo à toa. Isso já
não acontece. Mas quando o documento **de facto** tem um carácter desses, a procura continua a
demorar os mesmos ~7 segundos de antes. É isso que este passo tenta reduzir.

## Onde a lentidão está, exactamente

Ficheiro: `03_infra/src/world.rs`, função `candidates_for_char` (por volta da linha 517 — o número
exacto pode ter mudado, confirmar por leitura directa do ficheiro, não confiar neste número).

O que essa função faz hoje: quando precisa de procurar um carácter em fontes de fallback, ela
**abre e lê o conteúdo de todas as fontes do sistema, uma de cada vez, em sequência**, até
encontrar uma que tenha esse carácter. Ler cada fonte demora tempo (é um ficheiro no disco,
precisa de ser interpretado). Com centenas de fontes instaladas (medido: ~1112 no ambiente onde
isto foi testado), isso soma até aos ~7 segundos.

## Duas ideias já propostas, ainda não tentadas — este passo é para tentar uma delas

### Ideia 1 — ler várias fontes ao mesmo tempo, não uma de cada vez

Em vez de ler as ~1112 fontes em sequência (uma depois da outra), usar várias "linhas de
execução" (chama-se *threads*, em Rust) para ler várias fontes ao mesmo tempo. Se o computador
tiver, por exemplo, 8 núcleos de processador, ler 8 fontes ao mesmo tempo pode reduzir o tempo
total para perto de 1/8 do que é hoje — não é garantido (depende de quanto do tempo é
gasto a ler do disco, que pode ser um limite partilhado entre as threads), mas é a primeira coisa
a medir.

### Ideia 2 — guardar o resultado da procura para reaproveitar depois

Guardar num ficheiro, no disco, o resultado desta procura (quais fontes têm quais caracteres) uma
vez feita. Da próxima vez que o compilador for chamado, em vez de reler todas as fontes, lê
primeiro esse ficheiro guardado — muito mais rápido. O ficheiro só precisa de ser recriado se as
fontes instaladas no sistema mudarem (o que é raro).

**Não decidir sozinho qual das duas implementar** — ler as duas com atenção, medir o esforço e o
risco de cada uma (a Fase A abaixo pede isso), e levar a decisão ao dono do projecto antes de
escrever qualquer código, mesmo que uma das duas pareça obviamente melhor.

---

## Fase A — medir antes de decidir

1. Confirmar, por leitura directa do código actual (não pelos números deste prompt, que podem
   estar desactualizados), o estado exacto de `candidates_for_char` e do que P927 já mudou à
   volta dela.
2. Para a Ideia 1 (threads): confirmar quantos núcleos de processador o ambiente de teste tem
   disponíveis (comando `nproc` no Linux). Medir, com um protótipo temporário (não guardado no
   projecto ainda, só para medir), quanto tempo a procura demora com threads paralelas, comparado
   com o tempo actual. Confirmar se o ganho é proporcional ao número de núcleos ou se fica limitado
   por outra coisa (leitura de disco, por exemplo).
3. Para a Ideia 2 (guardar em disco): confirmar onde seria razoável guardar esse ficheiro (uma
   pasta de cache do sistema operativo, por convenção — confirmar qual é a convenção certa para
   Linux/macOS/Windows, já que este projecto pode correr nos três). Confirmar como detectar que as
   fontes do sistema mudaram desde a última vez (por exemplo, comparando datas de modificação dos
   ficheiros de fonte, ou um número de versão do sistema de fontes) — sem essa detecção, o
   cristalino pode continuar a usar um resultado desactualizado depois de o utilizador instalar uma
   fonte nova.
4. Registar, para as duas ideias: quanto tempo de implementação parece razoável, que risco cada
   uma traz (por exemplo: threads podem ter bugs de concorrência difíceis de testar; cache em disco
   pode ficar desactualizado ou ocupar espaço). Não implementar ainda.

## Fase A.1 — decisão, com o dono

Apresentar as duas medições da Fase A ao dono do projecto. Esperar decisão sobre qual
implementar (pode ser uma só, ou as duas juntas, ou nenhuma se o ganho medido não compensar o
risco). **Não avançar para a Fase B sem essa decisão confirmada.**

## Fase B — implementar a opção escolhida

1. Antes de escrever qualquer código, verificar se a mudança precisa de alterar a forma pública de
   alguma função ou estrutura de dados usada por outras partes do projecto (isto chama-se, neste
   projecto, "contrato público"). Se precisar: primeiro escrever/actualizar a documentação
   correspondente em `00_nucleo/prompts/` (chamados "L0" neste projecto), e só depois escrever o
   código — nunca ao contrário. Isto é uma regra fixa deste projecto ("Trava Arquitectural"),
   quebrada por engano no próprio passo anterior (P927) — não repetir o erro.
2. Escrever um teste automatizado que meça o tempo (ou a contagem de ficheiros abertos, que é mais
   fiável que tempo de relógio) **antes** de escrever a correcção, confirmar que esse teste falha
   (mostra o problema), só depois escrever a correcção, e confirmar que o teste passa depois.
3. Rodar todos os testes automatizados do projecto (`cargo test --workspace`) e confirmar que
   nenhum começou a falhar — reportar os números exactos, separados por cada parte do projecto
   ("crate"), não só "passou tudo".
4. Confirmar visualmente/por medição que a correcção reduziu o tempo dos casos lentos
   (documento com carácter CJK; documento com emoji) — com números reais de antes e depois, não
   com a frase "ficou mais rápido".

## Fase C — confirmar que nada mais piorou

Repetir a mesma medição de tempo usada nos passos anteriores (7 documentos de teste padrão, mais
os casos específicos de CJK/emoji/latim/grego usados em P925-927), comparando o tempo **antes**
desta correcção com o tempo **depois** — não comparar contra o Typst original (isso responde a
uma pergunta diferente: "quem é mais rápido", não "este passo piorou alguma coisa").

## Resultado esperado

- As duas ideias medidas e comparadas, com números, antes de qualquer decisão.
- Decisão registada, confirmada com o dono do projecto.
- Uma das duas (ou as duas) implementada, com teste automatizado, sem quebrar nada que já
  funcionava.
- Números reais mostrando que os casos de CJK/emoji ficaram mais rápidos.
- Confirmação de que os 7 documentos de teste padrão não ficaram mais lentos.
- Se a mudança tocar qualquer coisa usada por outras partes do projecto: a documentação
  correspondente (`00_nucleo/prompts/`) foi actualizada **antes** do código, não depois — e isso
  fica escrito explicitamente no relatório final deste passo, com os nomes exactos dos ficheiros
  actualizados.
