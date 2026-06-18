# Passo 371 — F-5b fatia (1): distinção de tipo `strong` / `emph` / `text` (fidelidade à 0107)

> **O que faz.** Primeira fatia do F-5b: devolve a **distinção semântica de tipo** que o colapso do
> P101 removeu — `strong`, `emph` e `#set text(bold)` deixam de ser todos `Content::Styled[Bold]`
> indistinguíveis e voltam a ser **tipos distintos**, dando à **0107** a fidelidade ao vanilla
> (`*bold* ≠ _italic_ ≠ #set text X ≠ X`, distinção por **tipo de elemento**, medida no P366/P370).
> **Não é emenda de ADR** — o P370 mediu que isto é o modelo que a **0026 (`:63`)** e a **0105-D**
> já **prescrevem**; o vtable que a 0026 rejeita **não** é usado (a distinção é por variante/
> discriminante estático, não dispatch dinâmico). No máximo uma **nota** de que o colapso P101 é
> superado. **A forma (variante própria vs discriminante no `Styled`) sai da medição da Fase A** —
> o P370 recomendou variante própria (mais aderente à 0026/0105) mas marcou a clareza como "a
> medir"; **não fixar de memória** (Trava 1). **A verificação do α é gate obrigatório**: a distinção
> **não pode reabrir o caso 2** (o `morph_canon` serve o α-fixpoint, `rules.rs:229`) — o P370 inferiu
> que não reabre; este lote **prova** (roda o caso 2 + a rede +11). **O de-bake do render `#set
> text` (fatia 2) fica FORA** — é separável (P370 §4). **Design-first**: L0 + Trava antes do código.
> **Commita ao terminar** e **não emenda o seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P371 (confirmar livre).
**Pré-condição**: P370 fechado (conflito 0026×0107 medido como **aparente**; a distinção por
variante/discriminante satisfaz a 0026; o de-bake do render é fatia separável). Suíte **2742**, lint
**0/0**, lente **66/0**, árvore limpa. HEAD pós-P370. Lente `tekt-cargo-dsm` disponível (registrar
versão/commit). Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: F-5b fatia (1) — **muda o `==` da linguagem rumo ao vanilla** (corrige a divergência
semântica, não é content-preserving no `==`). O **output de render** fica idêntico (paridade visual,
rede de caracterização): distinguir o tipo não muda como `strong`/`emph`/`#set text` **renderizam**
(continuam bold/italic). As asserções que mudam são as que codificam `strong == #set text`
(divergência atual) → viram para a distinção do vanilla, **declaradas e justificadas** (S5b).

---

## Fase A — medir a forma + o impacto no α (a fonte vence; `file:line`)

1. **Reler** a ADR-0026 (`:63` o modelo de variantes), a ADR-0105-D (uma variante por elemento + cl.3
   exaustividade), a ADR-0107 (a distinção semântica), as **Travas anti-deriva**, e o recon P370.
2. **Medir variante-própria vs discriminante** (a forma — não fixar de memória): para cada candidato,
   com `file:line` —
   - **(1) variantes próprias** `Content::Strong(Box)` / `Content::Emph(Box)` (reverte o colapso
     P101; o modelo prescrito por 0026 `:63` / 0105-D): quantos match arms voltam a existir, onde
     (layout, `plain_text`, `is_empty`, `map_*`, `morph_canon`, introspect, `==`), e o custo;
   - **(2) discriminante em `Styled`** `Styled{ kind: StyleKind, … }`: o custo, e se a clareza/
     exaustividade fica pior ou melhor que (1).
   Registrar a recomendação medida (o P370 favorece (1); confirmar ou refutar pela contagem).
3. **Medir o `==` rumo ao vanilla** (o que a distinção corrige): hoje `strong X == #set text X`
   (ambos `Styled[Bold]`); o vanilla dá `≠` (tipos distintos, `packed.rs:144`). Listar as asserções
   que codificam o `==` atual e que **viram** com a distinção, com `file:line` (S5b — declaradas).
4. **O impacto no α-fixpoint** (o gate duro): medir como o `morph_canon` trata o candidato escolhido.
   O P370 inferiu: o `morph_canon` já **mantém** o `Styled[Bold]` (morfologia não-vazia); mantê-lo
   como `Strong` próprio é equivalente para o α → a auto-igualdade e a terminação preservam-se.
   **Confirmar a forma** de o `morph_canon` tratar a nova variante/discriminante, com `file:line`, e
   declarar o que será rodado para **provar** (o caso 2 + a rede +11).
5. **A fronteira com a fatia (2)** — confirmar que esta fatia **não** toca o de-bake do render `#set
   text` (o `custom`-vs-tipado do `morph_canon`, P366 #1): a distinção de tipo é sobre
   `strong`/`emph` (morfologia), separável do render. Se a medição mostrar que se entrelaçam, **parar
   e reportar**.

---

## Limites duros

- **A distinção não pode reabrir o caso 2.** A verificação do α (caso 2 + rede +11 verdes) é gate de
  aceitação — se a distinção mudar a terminação do α-fixpoint, **parar e reverter** (não há fatia 1
  que valha reabrir o caso 2).
- **Não fazer o de-bake do render `#set text`** (fatia 2) — fora deste lote.
- **Não usar vtable/`dyn`/proc-macro** (a cláusula da 0026) — a distinção é por variante/
  discriminante **estático**.
- **Não tocar o caso 4, a flag P350c, o Marco G.**
- **Output de render idêntico** (paridade visual, rede de caracterização) — distinguir o tipo não
  muda como renderiza. As asserções que mudam são **só** as do `==` semântico (`strong == #set
  text` → `≠`), declaradas.
- **Não emendar ADR** — só a nota de que o P101 é superado pelo modelo de variantes (0026/0105).

---

## Estágios

### Estágio L0 — desenho (com a medição na mão) + Trava (PARA aqui para o dono)
Com a forma medida (variante vs discriminante) e o impacto no α confirmado, desenhar: a forma
escolhida, os match arms que voltam, como o `morph_canon` trata a distinção, e as asserções do `==`
que viram. Registrar no L0 (`f_fronteira_e1.md` + o entity de strong/emph) + a **nota** de que o
colapso P101 é superado (0026 `:63` / 0105-D). Sincronizar hashes. **TRAVA**: para no chat — a
medição + o desenho + a forma + os hashes para o dono aprovar. Nenhum código antes.

### Estágio 1 — a distinção de tipo (após aprovação)
Materializar a forma escolhida: `strong`/`emph` deixam de ser `Styled[Bold/Italic]` e passam a
variante própria (ou discriminante), per o desenho. O eval produz a forma nova; os consumidores
(layout, `plain_text`, `morph_canon`, introspect, `==`) ganham os arms. **Sem vtable.**

### Estágio 2 — verificação do α (o gate)
Rodar o caso 2 (os ~testes do α-fixpoint, P342–P350c) + a rede de caracterização (+11). **Têm de
passar sem alteração** — a distinção não muda a terminação. Se algum reabrir, **reverter**.

### Estágio Teste
- **Output de render**: a rede de caracterização passa — `strong`/`emph`/`#set text` renderizam
  idêntico (paridade visual).
- **`==` semântico (virado)**: `strong X ≠ #set text X` (a distinção do vanilla), as asserções que
  codificavam `==` viram, **declaradas e justificadas** (S5b, listadas).
- Confirmar que o caso 2 (α) e a rede passam; que o caso 4, a flag e o Marco G ficaram intactos; e
  que o de-bake do render (fatia 2) **não** foi tocado.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes (inclusive o α), **commitar**: `git add -A && git commit -m "Passo 371 — F-5b
fatia 1: distinção de tipo strong/emph/text (fidelidade 0107)"`. Árvore limpa e commitada. (Para na
Trava → commita só o L0; reverte por contradição/α → commita só o relatório.)

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2742, com as asserções do == semântico viradas (strong == #set
  text → ≠; declaradas, justificadas, listadas). A rede de caracterização (render) e o caso 2 (α)
  passam SEM alteração.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável; oráculo = vanilla 0.14.2):
  - distinção semântica: strong X ≠ emph X ≠ #set text X ≠ X (por tipo, como o vanilla).
  - render idêntico: strong/emph/#set text renderizam bold/italic como antes (paridade visual).
  - sem vtable: a distinção é variante/discriminante estático (cláusula 0026 respeitada).

GATE DO α (duro): o caso 2 (α-fixpoint, P342–P350c) + a rede +11 passam sem alteração. Se reabrir,
  REVERTER (a fatia 1 não vale reabrir o caso 2).

INTACTOS (confirmar): o de-bake do render #set text (fatia 2 — não tocado); caso 4, flag P350c,
  Marco G; os 3 numbering (P364/P365); o #set de props de usuário (P368).

lente (instrumento): content→elements = 66 (registrar; não é gate). edges elemento→elemento: se a
  variante nova adicionar aresta, registrar (a 0026/0105 preveem variantes; não é regressão de
  atomização — é o modelo prescrito).
perf (critério 4): antes = baseline da mesma sessão; depois reportado.
L0 (critério 5): f_fronteira_e1.md + entity strong/emph + nota P101-superado, hashes sincronizados
  ANTES do código, Trava aprovada.
commit: árvore limpa e commitada ao fim.
```

---

## Válvula declarada

Se a medição achar que reverter o colapso P101 (os match arms) não cabe num lote, fatiar (ex.:
`strong` primeiro, `emph` depois — como o P101 cogitou). Registrar a fatia e o número. O de-bake do
render `#set text` já é a fatia (2), separada.

---

## O que NÃO fazer

- **Não deixar a distinção reabrir o caso 2** (gate duro do α — reverter se reabrir).
- **Não fazer o de-bake do render `#set text`** (fatia 2).
- **Não usar vtable/`dyn`/proc-macro** (cláusula 0026).
- **Não fixar a forma (variante vs discriminante) de memória** — medir (Trava 1).
- **Não tocar o caso 4, a flag P350c, o Marco G.**
- **Não emendar ADR** (só a nota P101-superado).
- **Não pular a Trava** (medição + L0 + hash do dono antes do código).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no commit; a decisão é do dono.
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-371-relatorio.md` + resumo no chat)

A **medição variante-vs-discriminante** com `file:line` e a contagem de match arms (a forma
escolhida, com a justificativa vs o P370); a **lista das asserções do `==` que viraram** (strong ==
#set text → ≠, declaradas, justificadas vs vanilla); o L0 + a nota P101-superado com hash e a Trava
aprovada; a distinção implementada com `file:line` (sem vtable); **a prova do α** (o caso 2 + a rede
+11 verdes — o número, não "passou"); a paridade visual de render; a confirmação de que a fatia (2),
o caso 4, a flag e o Marco G ficaram intactos; a medição da lente; a perf; **o commit de fecho**
(hash); `git status` limpo; lint 0/0; o caveat de stack. **A nota de que esta é a fatia (1) do F-5b;
a fatia (2) é o de-bake do render `#set text`.** Termina aqui — não emenda o seguinte.

## Próximo (fixado pela ordem do P370, não a decidir)

Com a distinção de tipo feita, a **fatia (2)** é o **de-bake do render `#set text`** (o estilo de
render viaja transparente à morfologia pelo `custom`, como o numbering — P366; o `morph_canon`
distingue render-transparente de morfologia-mantida). Eu escrevo a fatia (2) quando o dono rodar
esta e subir o relatório.

## Fora de escopo (confirmado)

A **fatia (2)** do F-5b (de-bake do render `#set text`); o **Marco G** (não-F; também toca a 0026,
decisão separada); DEBT-59 (flag CLI); DEBT-60 (contador); qualquer toque no caso 4, no Marco G ou
na flag.
