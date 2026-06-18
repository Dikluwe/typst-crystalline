# Passo 368 — item (3): `#set <elemento-de-usuário>(prop:)` pelo mapa aberto — a extensibilidade

> **O que faz.** Fecha o **item (3)** da auditoria P362 — a **única lacuna que é extensibilidade
> prometida e não entregue** (os itens 1 e 2 eram atomização/limpeza). O F-B propôs a chain com os
> 10 campos nativos fechados **+ um mapa aberto** (`PropKey → Value`) para propriedades de
> **elementos de usuário**. A auditoria mediu: o canal (`StyleDelta.custom`) **existe**, mas só o
> numbering o usa; o `#set` de uma prop de elemento de usuário cai em **`unsupported_target_warn`**
> (`eval/rules.rs:56`) — as props de usuário só são setáveis pelo **construtor**, não por `#set` na
> chain. Este passo **liga** o `eval_set_rule` ao mapa aberto para alvos de usuário: `#set
> callout(cor: ...)` passa a pôr a prop no `StyleDelta.custom`, e o elemento de usuário a lê **pela
> chain** no layout. **É adicionar capacidade** (não remover campo) — a forma sai da medição do
> caminho inteiro + do **público typst** (o que um usuário faz puramente em `.typ`), com o **vanilla
> como oráculo** do que `#set` sobre elemento de usuário deve fazer. **Não toca** o α/caso 2, o
> `morph_canon`/`==`, o caso 4, a flag P350c, o Marco G, nem o `TextStyle`/F-5b. **Design-first**:
> L0 + Trava antes do código. **Commita ao terminar** e **não emenda o seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P368 (confirmar livre).
**Pré-condição**: item (1) 3/4 (3 numbering com fonte única, P364/P365; `TextStyle`/F-5b adiado,
DEBT-61); item (2) coberto (F-6 medido sem de-bake, P367). A **chain é a fonte única de estilo** que
o item (3) precisava — estabelecida pelos numbering. Suíte **2738**, lint **0/0**, lente **66/0**,
árvore limpa. HEAD pós-P367. Lente `tekt-cargo-dsm` disponível (registrar versão/commit). Caveat de
stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: **adição de capacidade** (extensibilidade) — **não** content-preserving no sentido dos
de-bakes (acrescenta um caminho que antes não existia: `#set` sobre alvo de usuário). A **rede de
caracterização (+11, P331)** prova que o existente não regride; **fixtures novos** provam a
capacidade nova (o `callout`, elemento fora dos 65, é o caso). **Justificativa = a extensibilidade
prometida do F-B** (o item do F não cumprido), com o **vanilla como oráculo** da semântica de `#set`
sobre elemento de usuário.

---

## Fase A — medir o caminho inteiro + o público typst (a fonte vence; `file:line`)

1. **Reler** ADR-0107, ADR-0108, as **Travas anti-deriva**, o L0 `f_fronteira_e1.md` (o F-B: a
   chain 10-fechados + o mapa aberto + o `Value`), e a auditoria P362 (item 3).
2. **O oráculo vanilla** (a semântica de `#set` sobre elemento de usuário): medir, em `lab/`, o que
   o vanilla faz quando um `#set <elem-de-usuário>(campo: valor)` é aplicado — como a prop entra na
   `StyleChain`, como o elemento a resolve (o padrão `#[ghost]` / `field(...)` resolvido da chain),
   e o que `#set` faz a um campo que o elemento não declara. **A semântica do cristalino tem de
   casar a do vanilla** (ADR-0107) — registrar com `file:line`.
3. **O caminho inteiro no cristalino**, com `file:line`:
   - **`eval_set_rule`** (`eval/rules.rs:56`) — onde o despacho por nome fixo
     (heading/equation/figure/page/par/text) cai em `unsupported_target_warn` para um alvo de
     usuário. O ponto a ligar.
   - **`StyleDelta.custom`** (`style_chain.rs:77`) — o mapa aberto; como a prop de usuário entra
     (a `PropKey` é o nome do campo? namespaced pelo elemento?), e como o `Value` a representa.
   - **A leitura pelo elemento** — como o elemento de usuário (o `callout`, via o trait) lê a prop
     da chain no layout (o `get_field`/`dyn_get_field` já existe — S7; medir se a leitura de `#set`
     pela chain reusa esse caminho ou precisa de um novo).
4. **O público typst** (o item dos dois públicos que a auditoria marcou como o que falta provar): o
   que um usuário escreve **puramente em `.typ`** para que `#set callout(prop:)` funcione — sem
   Rust. Medir o que já existe (o `callout` é fixture Rust; há um caminho `.typ` para declarar a
   prop setável?) e o que o passo precisa entregar para o público typst, não só o Rust.
5. **A fronteira com o F-5b e os limites duros**: confirmar que ligar o `#set` de usuário ao
   `custom` **não** toca o `TextStyle`/F-5b, o α/`morph_canon` (o `custom` é ignorado por
   `is_semantically_empty` → transparente à morfologia, como o numbering — P366), o caso 4 nem a
   flag. Se a medição mostrar que toca, **parar e reportar**.

---

## A forma da ligação — decidida pela medição (não antes dela)

- O `eval_set_rule`, para um alvo que é elemento de usuário registrado, em vez de
  `unsupported_target_warn`, põe `(PropKey, Value)` no `StyleDelta.custom` (a forma da `PropKey` e
  do `Value` saem da Fase A + do oráculo vanilla).
- O elemento de usuário lê a prop da chain no layout (reusando `dyn_get_field`/S7 se a medição
  mostrar que serve).
- O **público typst** recebe o que a Fase A medir como necessário para declarar a prop setável em
  `.typ` (se o vanilla o exige; se não, registrar que o público typst para aqui por fronteira
  declarada, como em P362-C).
- Caso que não cabe, ou que toca um limite duro → **parar e reportar**.

---

## Limites duros

- **Não tocar o `TextStyle`/F-5b** (adiado), o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag
  P350c nem o Marco G. O `custom` é transparente à morfologia (como o numbering); confirmar.
- **A semântica casa o vanilla** (ADR-0107) — `#set` sobre elemento de usuário faz o que o vanilla
  faz; não inventar comportamento. Se o vanilla e a fonte forem ambíguos, o vanilla compilado vence.
- **Não regredir o existente** — a rede de caracterização (+11) e os `#set` nativos
  (heading/equation/figure/page/par/text) passam sem alteração; a capacidade nova é **aditiva**.
- **Não inventar escopo de público typst** — entregar o que o vanilla exige; onde o público typst
  parar por fronteira (layout = Rust, P362-C), registrar, não fingir.

---

## Estágios

### Estágio L0 — desenho (com a medição na mão) + Trava (PARA aqui para o dono)
Com o caminho medido e o oráculo vanilla, desenhar: a forma da `PropKey`/`Value` para props de
usuário, a ligação no `eval_set_rule`, a leitura pelo elemento, e o que o público typst recebe.
Registrar no L0 `f_fronteira_e1.md` (a secção do mapa aberto / os dois públicos). Sincronizar
hashes. **TRAVA**: para no chat — a medição + o oráculo vanilla + o desenho + os hashes para o dono
aprovar. Nenhum código antes.

### Estágio 1 — ligar o `eval_set_rule` ao mapa aberto (após aprovação)
Para um alvo de elemento de usuário registrado, `eval_set_rule` põe `(PropKey, Value)` no
`StyleDelta.custom` em vez do warn. Aditivo — os `#set` nativos não mudam.

### Estágio 2 — a leitura pelo elemento + o público typst
O elemento de usuário lê a prop da chain no layout (per o desenho); o público typst recebe o que a
Fase A mediu.

### Estágio Teste
- A **rede de caracterização (+11)** e os `#set` nativos passam **sem alteração** (não-regressão).
- **Fixtures novos** (a capacidade nova): `#set callout(<prop>: <valor>)` num documento `.typ`
  aplica a prop ao `callout` via a chain, e o layout a lê — o output reflete a prop (paridade com o
  que o vanilla faria). Um fixture do **público typst** (puramente `.typ`) se o vanilla o suporta.
- Confirmar que o `TextStyle`/F-5b, o α/caso 2, o `morph_canon`/`==`, o caso 4 e a flag ficaram
  intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes, **commitar**: `git add -A && git commit -m "Passo 368 — #set de props de
usuário pelo mapa aberto (extensibilidade)"`. Árvore limpa e commitada. (Para na Trava → commita só
o L0; reverte por contradição → commita só o relatório.)

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2738 + os fixtures novos da capacidade (a rede e os #set nativos
  inalterados — não-regressão; as adições provam o #set de usuário).
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável; oráculo = vanilla 0.14.2):
  - #set <elem-de-usuário>(prop:) aplica a prop pela chain; o elemento a lê no layout; o output
    reflete a prop (paridade com o vanilla).
  - o público typst: um usuário faz o #set em .typ (o que o vanilla suporta); onde parar por
    fronteira (layout = Rust), registrado.
  - não-regressão: os #set nativos e a rede de caracterização inalterados.

INTACTOS (confirmar): o TextStyle/F-5b (adiado); α/caso 2, morph ==/morph_canon (o custom é
  transparente à morfologia), caso 4, flag P350c, Marco G; os 3 numbering (P364/P365).

lente (instrumento): content→elements = 66 (registrar; não é gate do F).
perf (critério 4): antes = baseline da mesma sessão (re-medir no início); depois reportado.
L0 (critério 5): f_fronteira_e1.md (mapa aberto / dois públicos) + hash sincronizado ANTES do
  código, Trava aprovada.
commit: árvore limpa e commitada ao fim.
```

---

## Válvula declarada

Se a Fase A medir que ligar o `eval_set_rule` + a leitura + o público typst não cabe num lote,
fatiar (ex.: o `#set` → `custom` + leitura pelo elemento como P368; o público typst como P369).
Registrar a fatia e o número medido.

---

## O que NÃO fazer

- **Não tocar o `TextStyle`/F-5b, o α / caso 2, o `morph_canon`/`==`, o caso 4, a flag P350c nem o
  Marco G.**
- **Não inventar a semântica** de `#set` sobre elemento de usuário — casa o vanilla (ADR-0107).
- **Não regredir** os `#set` nativos nem a rede de caracterização.
- **Não inventar escopo de público typst** além do que o vanilla exige; registrar a fronteira.
- **Não pular a Trava** (medição + oráculo + L0 + hash do dono antes do código).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no commit; a decisão é do dono.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-368-relatorio.md` + resumo no chat)

A medição do **caminho inteiro** (`eval_set_rule` → `StyleDelta.custom` → leitura pelo elemento) com
`file:line`; o **oráculo vanilla** (a semântica de `#set` sobre elemento de usuário) com `file:line`;
a medição do **público typst** (o que existe, o que o passo entrega, onde para por fronteira); o L0
com hash e a Trava aprovada; a ligação e a leitura, com `file:line`; os **fixtures novos** que provam
a capacidade (o `callout`, e o público typst se suportado); a não-regressão (rede + `#set` nativos);
a prova de que o `TextStyle`/F-5b, o α/caso 2, o `morph_canon`/`==`, o caso 4 e a flag ficaram
intactos; a medição da lente; a perf; **o commit de fecho** (hash); `git status` limpo; lint 0/0; o
caveat de stack. **A nota de que o item (3) — a extensibilidade prometida — fecha com este lote, e
com ele o que faltava do F pelos princípios** (com o F-5b registrado como lote arquitetural
dedicado, DEBT-61). Termina aqui — não emenda o seguinte.

## Fora de escopo (confirmado)

O **`TextStyle`/F-5b** (lote arquitetural dedicado, DEBT-61); o **Marco G** (não é F); DEBT-59 (flag
CLI); DEBT-60 contador; qualquer toque no α / `morph_canon` / `==` ou na flag.
