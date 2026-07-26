# Passo 910 — verificação e commit da reconciliação de numeração de ADRs (0119–0125)

**Precede este passo**: as seis ADRs preparadas nesta conversa —
`typst-adr-0119-disciplina-verificacao.md`, `typst-adr-0121-proveniencia-medicao.md`,
`typst-adr-0122-paridade-defeitos-testes.md`, `typst-adr-0123-geometria-tipografica.md`,
`typst-adr-0124-checklist-sublayouts.md`, `typst-adr-0125-implementacao-vs-linguagem.md`. Ler os
seis antes de começar — o conteúdo já está escrito, este passo é sobre confirmar contra o
repositório real e fechar as pontas soltas, não sobre redigir de novo.

**Este passo é administrativo — documental, sem código de produção tocado.**

---

## Fase A — confirmar contra o repositório real (obrigatório antes de mover qualquer ficheiro)

1. Varredura real de `00_nucleo/adr/`: confirmar que `0119`-`0125` estão de facto livres, incluindo
   confirmar que `0120` (`TextShaped`/rustybuzz) continua a ser o único número ocupado no meio do
   intervalo. Não presumir a partir do que foi visto nesta conversa — os arquivos enviados podem não
   ter sido a totalidade do directório.
2. **Confirmar se `0119` foi pulado de propósito** (reserva para outra coisa, como `0063` está
   reservado no `README.md` para "column flow algorithm") ou é lacuna genuína. Se houver reserva
   documentada em qualquer lado (`README.md`, outro ADR, `DEBT.md`), este passo pára aqui e a
   numeração desta remessa desloca-se um número inteiro — não decidir isso sem confirmar.
3. **Procurar "Decisão nova obrigatória"** — a terceira regra citada por nome em `ADR-0119`,
   `ADR-0121`, `ADR-0122`, `ADR-0124`, `ADR-0125`, e ainda não localizada em nenhuma remessa desta
   conversa. Procurar por conteúdo (`grep -ril "nenhum item aceite é permanente" 00_nucleo/`), não só
   por nome de ficheiro — o padrão desta série mostra que os nomes de ficheiro não seguiam convenção
   de numeração antes de serem reconciliados. Se encontrada:
   - Confirmar a data — sendo citada como já existente em `ADR-0119` (2026-07-03), é provavelmente
     mais antiga que todas as seis desta remessa.
   - Se for mais antiga que 2026-07-03: esta regra ocupa o número mais baixo disponível antes de
     `0119` (candidato: o próprio `0119`, empurrando as seis actuais uma posição cada:
     `0119→0120` **conflitaria com `TextShaped`, já ocupado** — logo, se isto acontecer, a sequência
     completa desta remessa precisa de ser recalculada, não só inserido um número no meio).
   - Se não for encontrada: registar explicitamente, nas seis ADRs, que a busca foi feita e falhou
     (já parcialmente feito nesta remessa — confirmar que a nota fica clara e não desaparece em
     edições futuras).
4. Confirmar que nenhuma das seis colide com conteúdo real já existente nesses números no
   repositório (mesmo tipo de erro que gerou este passo — `0112` fui eu quem propôs sem verificar).

5. **`ADR-0026` duplicada** (achado do dono, fora do que esta conversa já tinha revisto). Confirmar
   qual dos dois casos é:
   - **Caso esperado**: uma das duas é `typst-adr-0026-....md` e a outra é
     `typst-adr-0026-R1-....md` (ou convenção equivalente) — o próprio `README.md` já documenta "63
     números únicos; `ADR-0026` tem variante `-R1` por revisão" (secção de abertura). Se for isto,
     não há nada a corrigir, só confirmar que os dois ficheiros seguem a convenção de nome
     (`-R1` no nome, não só no conteúdo) e que o `README.md` continua a descrever isto correctamente.
   - **Caso real de conflito**: dois ficheiros ambos reivindicando `0026` sem sufixo de revisão, ou
     conteúdo genuinamente não relacionado sob o mesmo número. Se for isto, tratar como a mesma
     classe de erro que gerou este passo inteiro (`0112` duplicado) — um dos dois precisa de
     renumeração para o primeiro slot livre confirmado por varredura, escolhendo qual fica no
     número original por ordem cronológica de criação (o mais antigo fica, o mais novo desloca).

6. **`ADR-0056` ausente**. Confirmar se está reservada (documentada em `README.md`, `DEBT.md`, ou
   noutro ADR, mesmo padrão de `0063` abaixo) ou se é lacuna genuína (mesmo tratamento de `0119`).
   Não presumir — procurar antes de decidir.

7. **`ADR-0063` ausente — RESOLVIDO por pesquisa nesta conversa, confirmar só a aplicação.**
   `DEBT-56` (column flow) **já está fechado** — `ADR-0078` ("Column flow algorithm:
   `Region/Regions` abstraction + multi-column consumer") nasceu `PROPOSTO` em P215 e transitou
   `IMPLEMENTADO` em **P221** (2026-05-12); `columns()`/`colbreak()` materializados desde então
   (`00_nucleo/DEBT.md`, entrada "DEBT-56 — ... ENCERRADO (Passo 221)"). O trabalho **não consumiu**
   a reserva do `0063` — usou `0078`, o próximo livre naquele momento. A reserva ficou órfã:
   `README.md` continua a descrever `0063` como "reservada para column flow" muito depois de o
   column flow já estar implementado noutro número.
   - **Não escrever ADR nova para column flow — já existe (`ADR-0078`).**
   - Acção deste passo: remover/corrigir a nota de reserva de `0063` em `README.md` (secção de
     abertura, "Reservas de números"), registando que a reserva ficou obsoleta e `0063` está de
     facto livre para uso futuro — não continuar a descrevê-lo como comprometido para algo já
     resolvido noutro sítio.
   - Confirmar, por varredura, que não há nenhum outro lugar (`DEBT.md`, outro ADR) ainda a tratar
     `0063` como reservado para column flow, para não deixar a correcção pela metade.

8. **Varredura sistemática de todos os documentos no formato "Prompt L0 — ADR: ..." em todo o
   projecto — não só os que apareceram por acaso nesta conversa.** Os cinco arquivos sem número
   reconciliados nesta remessa (`ADR-0119`/`0121`/`0122`/`0124`/`0125`) e o achado de
   `adr-stub-vs-fallback.md` (que já tinha virado `ADR-0113`, felizmente) só foram encontrados
   porque o dono os foi enviando um a um, à medida que se lembrava ou tropeçava neles. Isso não é
   uma forma confiável de garantir que não há mais nenhum solto — é amostragem, não varredura.

   Antes de fechar este passo:
   ```bash
   grep -rl "Prompt L0 — ADR:" 00_nucleo/ --include="*.md"
   ```
   Para cada resultado, confirmar um dos dois estados:
   - **Resolvido**: o campo "Ficheiro alvo" do documento já corresponde a um ficheiro real existente
     em `00_nucleo/adr/` com esse conteúdo (mesmo padrão de `adr-stub-vs-fallback.md` → `ADR-0113`).
     Nada a fazer, só confirmar e não voltar a tratar como pendente.
   - **Não resolvido**: o "Ficheiro alvo" não existe, ou existe mas com conteúdo diferente do
     documento-fonte. Tratar como os cinco desta remessa — localizar o número certo por varredura
     real (não assumir), renomear, reconciliar referências cruzadas.

   Fazer o mesmo grep também para variações de formato que possam ter sido usadas antes da
   convenção "Prompt L0 — ADR:" se estabilizar (por exemplo, ficheiros `.md` na raiz de
   `00_nucleo/` ou noutra pasta, sem o cabeçalho exacto, mas com conteúdo de decisão arquitectural
   reconhecível — mesmo critério usado para localizar as seis desta remessa, que também não tinham
   o cabeçalho padrão). Registar no relatório a lista completa de resultados do grep, mesmo os já
   resolvidos, para não ter de repetir esta varredura do zero num passo futuro.

## Fase B — aplicar (só depois da Fase A confirmar tudo)

1. Mover os seis ficheiros para `00_nucleo/adr/` com os nomes finais:
   - `typst-adr-0119-disciplina-verificacao.md`
   - `typst-adr-0121-proveniencia-medicao.md`
   - `typst-adr-0122-paridade-defeitos-testes.md`
   - `typst-adr-0123-geometria-tipografica.md`
   - `typst-adr-0124-checklist-sublayouts.md`
   - `typst-adr-0125-implementacao-vs-linguagem.md`
2. Se a Fase A encontrar "Decisão nova obrigatória" e isso deslocar a numeração: renomear antes de
   commitar, não depois — os nomes de ficheiro e os números internos (`# ADR-0NNN —`) têm de bater
   com o número final em todos os seis, incluindo as referências cruzadas entre eles (`ADR-0119`
   citado dentro de `ADR-0121`, etc.).
3. Confirmar `ADR-0123` continua `PROPOSTO` (decisão do dono, não deste passo, de manter esse
   número) — as outras cinco continuam `EM VIGOR`, estado que já tinham antes de serem numeradas.
4. Actualizar `00_nucleo/adr/README.md`: adicionar as seis entradas à tabela "Estado por ADR". O
   `README.md` já está desactualizado há vários passos (pára por volta de P441-443, enquanto os ADRs
   reais chegam a P486/`0120`) — **este passo não tem de fechar essa lacuna inteira**, só adicionar as
   seis entradas novas sem fingir que o resto do índice está sincronizado. Se decidir fechar a lacuna
   inteira, isso é um passo à parte, maior, não este.
5. `crystalline-lint .` — confirmar que nenhum dos seis ficheiros novos dispara `V7` (`OrphanPrompt`)
   incorretamente — ADRs não são prompts L0 referenciados por código, então confirmar se `V7` se
   aplica a este directório ou só a `00_nucleo/prompts/` (não presumir).

## Resultado esperado

- Seis ADRs no lugar final, números confirmados por varredura real, não por suposição.
- "Decisão nova obrigatória" localizada e numerada, ou busca documentada como falhada, sem ficar
  pendente silenciosamente.
- Veredicto sobre `ADR-0026`: revisão `-R1` legítima (nada a fazer) ou conflito real (um dos dois
  renumerado).
- Veredicto sobre `ADR-0056`: reserva documentada ou lacuna genuína.
- `README.md` corrigido para `0063`: reserva removida/marcada obsoleta, `ADR-0078` (column flow,
  `IMPLEMENTADO` desde P221) referenciada como onde o trabalho de facto aconteceu — já confirmado
  nesta conversa, este passo só aplica a correcção.
- `README.md` com as seis entradas novas (sem fingir sincronização total) — e corrigido também para
  `0026`/`0056`/`0063` se a Fase A encontrar algo a corrigir, não só as seis originais.
- **Lista completa do grep do ponto 8** registada no relatório — todos os "Prompt L0 — ADR: ..."
  do projecto, cada um com estado confirmado (resolvido/não resolvido), não só os que esta
  conversa encontrou por acaso. Qualquer não resolvido encontrado por esta varredura entra na
  mesma reconciliação de numeração deste passo, não fica para "depois".
- Relatório do passo registando explicitamente qual dos cenários da Fase A ponto 3 ocorreu
  (encontrada / não encontrada / numeração recalculada), mais o veredicto de cada um dos achados
  novos (pontos 5-8).
