# Relatório — Passo 910: verificação e commit da reconciliação de numeração de ADRs (0119–0125)

**Data:** 2026-07-25
**Commit de partida:** `5cfb12430` (P908; nenhum código de produção tocado neste passo — só documentação)

---

## Resumo executivo

Passo administrativo. As seis ADRs preparadas (`0119`/`0121`/`0122`/`0123`/`0124`/`0125`) foram
confirmadas contra o repositório real (não presumidas a partir da conversa que as gerou), movidas
para `00_nucleo/adr/` com os nomes finais, com cabeçalho canónico corrigido (`# ADR-0NNN — <título>`,
`**Estado:**`), e `README.md` actualizado com as seis entradas. Três achados adicionais fora do
escopo original das seis ADRs foram confirmados e um deles corrigido (ADR-0063, reserva obsoleta).

## Fase A — achados por varredura real

### Ponto 1-2 — 0119-0125 livres; 0119 não reservado

Varredura de `ls 00_nucleo/adr/` confirma: `0110`-`0118` e `0120` ocupados (sequência contígua,
`0120` = TextShaped/rustybuzz, único número do meio), `0119`/`0121`/`0122`/`0124`/`0125` **ausentes**
antes deste passo. `0123` já estava presente (criado nesta mesma remessa, cabeçalho já correcto).
Busca por reserva de `0119` em `README.md` (`grep -n "0119"`) — **zero ocorrências** antes deste
passo. A própria `ADR-0123` (secção "Histórico de numeração") já registrava independentemente
"`0119` ficou vazio entre `ADR-0118` e `ADR-0120`" — corrobora, não é achado novo. Veredicto:
**lacuna genuína, não reserva.**

### Ponto 3 — "Decisão nova obrigatória"

Busca por conteúdo (`grep -ril "nenhum item aceite é permanente" 00_nucleo/`) encontrou **apenas
citações**, nunca a definição original: `regra-disciplina-verificacao.md` (agora ADR-0119),
`regra-proveniencia-medicao.md` (agora ADR-0121), e o próprio `typst-passo-910.md`. Busca mais larga
pelo nome (`grep -ril "decisão nova obrigatória"`) encontrou adicionalmente
`00_nucleo/handoff-novo-chat-p762.md` (2026-07-16) — que também só **cita** a regra por nome (item 1
de uma lista de 10), sem a definir com corpo próprio. A lista curada "Meta-regras em vigor" do
`README.md` (12 itens, todos com número ADR próprio) **não a inclui**.

**Veredicto: NÃO ENCONTRADA como ADR/documento próprio, em lado nenhum do repositório.** Ramo
aplicável do passo: o simples (sem recálculo de numeração — esse ramo só se aplicava se a regra
existisse como documento datado mais antigo que 2026-07-03). Correcção aplicada: `ADR-0119`
afirmava, antes desta correcção, que "Decisão nova obrigatória" era uma "(ADR anterior)" — frase
presumida, nunca confirmada, e agora sabida falsa. Corrigida com nota de reconciliação explícita em
`ADR-0119` (secção "Como isto se liga à regra anterior"); as quatro ADRs que também a citam
(`0121`/`0122`/`0124`/`0125`) ganharam anotação "(citada por nome; não materializada como ADR
própria — ver nota em ADR-0119)" na primeira ocorrência, para a nota não se perder por estar só
num dos seis ficheiros.

### Ponto 4 — colisão de conteúdo

Nenhuma — os cinco números (`0119`/`0121`/`0122`/`0124`/`0125`) não tinham ficheiro antes deste
passo, logo não há conteúdo prévio para colidir.

### Ponto 5 — `ADR-0026` duplicada

`typst-adr-0026-content-divergencia.md` + `typst-adr-0026-R1-content-arc.md` — **caso esperado**,
convenção `-R1` de revisão, já documentada no próprio `README.md` ("63 números únicos; ADR-0026 tem
variante -R1 por revisão", linha 5; tabela "Estado por ADR" linhas 161-162). **Nada a corrigir.**

### Ponto 6 — `ADR-0056` ausente

Única menção em todo o `00_nucleo/`: `README.md:2823` (secção histórica de P266), listando "C.6 Font
subsetting PDF (candidato P267 Opção 2 — ADR-0056; M-L)" — explicitamente rotulada **"candidato"**,
nunca uma reserva formal (ao contrário de `ADR-0063` abaixo, que usa a palavra "reservada"
explicitamente). Confirmado que o trabalho de font subsetting foi de facto materializado, mas sob
**`ADR-0027`** (`typst-adr-0027-cidfont-subsetting.md`), não `0056`. **Veredicto: lacuna genuína** —
o candidato antigo nunca foi consumido nem bloqueia nada; não corrigido (é registo histórico de uma
possibilidade considerada e não tomada, não uma reserva activa a libertar).

### Ponto 7 — `ADR-0063` ausente

`README.md` (linhas 17-19, antes deste passo) reservava `0063` para "column flow algorithm... se ADR
dedicada for criada quando DEBT-56 for materializado". Confirmado em
`00_nucleo/diagnosticos/debt/DEBT.md:1326`: **DEBT-56 fechou no Passo 221** — mas a ADR dedicada de
column flow resultante foi a **ADR-0078** (`typst-adr-0078-column-flow-algorithm.md`), não `0063`
(corroborado pela entrada da tabela "Estado por ADR", `0061`/roadmap Layout Fase X, que também cita
P221 sem mencionar `0063`). **Veredicto: reserva obsoleta, `0063` de facto livre** — o evento que a
consumiria já aconteceu, sob outro número (o dono confirmou/actualizou o próprio `typst-passo-910.md`
com este mesmo veredicto entre a primeira e a segunda passagem deste passo, coincidindo com a
medição feita aqui).

**Corrigido em dois locais** (varredura confirmou um segundo local vivo a repetir a reserva, para
não deixar a correcção pela metade):
- `README.md` §"Reservas de números": entrada reescrita para `` ~~**ADR-0063**~~ **RESERVA OBSOLETA,
  `0063` LIVRE** `` (mesma convenção visual já usada pela entrada irmã `ADR-0062` **CONSUMIDA**
  acima), registando o achado sem apagar o texto da reserva original.
- `typst-adr-0066-introspection-runtime-adiada.md:171` (EM VIGOR) também reafirmava a mesma reserva
  ("Slot 0063 está reservada conceptualmente... preserva-se sem ficheiro per política 'sem novas
  reservas'") — texto original preservado (registo histórico, não reescrito), anotação cumulativa
  P910 adicionada logo a seguir com o mesmo veredicto e cross-reference ao README corrigido.

Entradas puramente históricas que mencionam a reserva antiga (`README.md` linhas ~661/688/1130/1737,
secção "Passos-chave da história dos ADRs") **não foram alteradas** — são registo do que era verdade
nesses passos, não afirmações vivas a corrigir (mesmo critério já aplicado ao achado do ponto 6).

## Fase B — aplicado

1. Cinco ficheiros movidos/renomeados para `00_nucleo/adr/typst-adr-0NNN-<slug>.md` (`0123` já
   estava correcto). Três via `git mv` (já tracked), dois via `mv` (ainda untracked no working tree
   — ficam para o `git add` do utilizador).
2. Nenhuma renumeração disparada pelo ponto 3 (ramo "não encontrada" confirmado).
3. `ADR-0123` confirmado `PROPOSTO` (inalterado, decisão do dono). As outras cinco confirmadas/
   corrigidas para `` `EM VIGOR` `` — duas (`0119`/`0121`) não tinham campo `Estado` nenhum antes
   deste passo (só `Data`/`Aplica-se a`); adicionado. As outras três já tinham "Estado: Em vigor"
   (minúsculas, sem backtick) — normalizado para `` `EM VIGOR` ``, convenção real usada pelos ADRs
   mais recentes (`0117`/`0118`/`0123`), não o template do próprio `README.md` (que usa `**Status**`
   + emoji — confirmado desactualizado face à prática real, não seguido).
4. `README.md`: seis entradas adicionadas à tabela "Estado por ADR" (após `0117`, única entrada mais
   recente já presente); reserva `ADR-0063` anotada como obsoleta (ponto 7). Lacuna `0118`/`0120`
   (também ausentes da tabela) **preservada de propósito** — fora do escopo deste passo.
5. `crystalline-lint .`: confirmado **V7 não se aplica a `00_nucleo/adr/`** — só ao warning
   pré-existente e não relacionado de `00_nucleo/prompts/infra/package_version_resolution.md`. Zero
   V7 novo nos seis ficheiros.

Cabeçalhos H1 de todos os cinco ficheiros renomeados corrigidos para `# ADR-0NNN — <título>`
(seguindo o padrão real recente — `0117`/`0118`/`0123` —, não o template `# ⚖️ ADR-NNNN:` do
`README.md`, confirmado como o único usado na prática antes de escolher). Todas as referências
cruzadas dentro de "Ligação às regras anteriores" (5 ficheiros) ganharam o número ADR ao lado do
nome da regra citada.

## Resumo por item

| Item | Veredicto | Estado |
|---|---|---|
| Seis ADRs no lugar final, números por varredura real | `0119`/`0121`/`0122`/`0124`/`0125` movidos+renomeados; `0123` já correcto | ✅ |
| "Decisão nova obrigatória" localizada ou busca documentada como falhada | **Não encontrada** como documento próprio (só citada); nota de reconciliação adicionada em ADR-0119 + 4 cross-refs | ✅ |
| `ADR-0026` | `-R1` legítimo, nada a corrigir | ✅ |
| `ADR-0056` | Lacuna genuína — candidato histórico nunca consumido (subsetting foi para `ADR-0027`) | ✅ |
| `ADR-0063` | Reserva **obsoleta, livre** — DEBT-56 fechou (P221) mas gerou `ADR-0078`, não `0063`; corrigido em 2 locais (README + ADR-0066) | ✅ |
| `README.md` com as seis entradas + correcção 0063 | Adicionado; lacuna `0118`/`0120` preservada de propósito (fora de escopo) | ✅ |
| `crystalline-lint .` sem V7 falso-positivo | Confirmado — V7 só se aplica a `00_nucleo/prompts/` | ✅ |
