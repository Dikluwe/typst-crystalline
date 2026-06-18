# Passo 369 — público typst: definir elemento com props setáveis em `.typ` (a medição decide se há trabalho)

> **O que faz.** Resolve a **outra metade do público typst** — definir um elemento com props
> setáveis **puramente em `.typ`** (sem Rust). O P368 entregou o `#set` sobre elementos
> **registrados**; falta saber se **definir** o elemento em `.typ` é uma lacuna do F ou a
> **fronteira da própria linguagem** (o P362-C mediu "definição = Rust"). **A medição vem PRIMEIRO e
> é decisiva**: a Fase A mede, no vanilla (oráculo), se um usuário define um elemento **com layout
> próprio** em `.typ` puro, ou se isso exige código/plugin. **O resultado decide se este passo tem
> trabalho:**
> - **se o vanilla PERMITE** definir em `.typ` puro → o P369 **entrega** a definição em `.typ`
>   (escopo desenhado a partir da medição);
> - **se o vanilla também EXIGE código** para layout novo → o público typst está **completo pela
>   fronteira da linguagem**, e o P369 **fecha sem código**, registrando a fronteira e **confirmando
>   o F completo pelos princípios** (exceto o F-5b, DEBT-61).
>
> **A medição não é assumida em nenhuma das direções** (Trava 1/6: medir a fronteira antes de
> decidir se há mais um passo). **Não toca** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag
> P350c, o Marco G, nem o `TextStyle`/F-5b. **Design-first**: se houver trabalho, L0 + Trava antes
> do código. **Commita ao terminar** e **não emenda o seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P369 (confirmar livre).
**Pré-condição**: P368 fechado (o `#set` de props de usuário ligado ao mapa aberto; o público typst
**sobre elementos registrados** entregue; suíte **2742**, lint **0/0**, lente **66/0**). HEAD
pós-P368, árvore limpa. Lente `tekt-cargo-dsm` disponível (registrar versão/commit). Caveat de
stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: **medição-decisiva-primeiro**. A Fase A é read-only (leitura do vanilla em `lab/`, probes
revertidas) e **decide se há Estágio de código**. Se houver, é **adição de capacidade**
(extensibilidade), com o vanilla como oráculo. Se não, fecha read-only com a fronteira registrada.

---

## Fase A — a medição decisiva vem PRIMEIRO (a fonte/oráculo vence; `file:line`)

**A pergunta que decide o passo, medida no vanilla (`lab/`) ANTES de qualquer desenho:**

> No vanilla 0.14.2, um usuário define um **elemento novo com layout próprio** puramente em `.typ`
> (sem Rust/plugin), ou isso exige código?

1. **Reler** ADR-0107, ADR-0108, as **Travas anti-deriva**, o L0 `f_fronteira_e1.md` (os dois
   públicos), e o P362-C (a fronteira "definição = Rust" que ele mediu) + o P368 (a metade entregue).
2. **Medir no vanilla**, com `file:line`:
   - O que `#let meu(..) = {...}` (compor elementos existentes) **é** no vanilla — uma função que
     devolve content, ou um elemento de primeira classe? Recebe `#set`/`#show`/query como um
     elemento nativo?
   - Há um caminho em `.typ` puro para um elemento com **layout próprio** (não só composição de
     existentes)? Ou o layout próprio exige Rust/WASM-plugin?
   - O que `#set`/`#show` fazem sobre uma função definida em `.typ` vs sobre um elemento — a
     distinção do vanilla entre "função que devolve content" e "elemento".
3. **Mapear contra o cristalino**: o que o cristalino já permite em `.typ` (composição via `#let`,
   `#show` sobre registrados — P368) e o que faltaria para casar o que o vanilla permite em `.typ`.
   Marcar **[medido]** vs **[inferido]** e o que refutaria.

**A medição produz um de dois veredictos**, e o passo segue por um deles:

- **Veredito A — o vanilla permite definir em `.typ` puro** (além do que o cristalino já faz): há
  trabalho. Desenhar o escopo (o que entregar para casar o vanilla) → Estágio L0 + Trava → código.
- **Veredito B — o vanilla também exige código para layout novo**: o público typst do cristalino
  está **completo pela fronteira da linguagem** (a composição via `#let` + o `#set`/`#show` sobre
  registrados, que o cristalino já tem, é o que o `.typ` puro permite). **Sem código.** Registrar a
  fronteira no L0 e **confirmar o F completo pelos princípios** (exceto o F-5b).

---

## Limites duros

- **Não assumir o veredito** (nem A nem B) antes da medição (Trava 1/6). O passo não pré-decide se
  há trabalho.
- **Não tocar o α / caso 2, o `morph_canon`/`==`, o caso 4, a flag P350c, o Marco G nem o
  `TextStyle`/F-5b.**
- **A semântica casa o vanilla** (ADR-0107) — não inventar um caminho `.typ` que o vanilla não tem;
  não declarar fronteira que o vanilla não impõe.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Estágios

### Estágio L0 — o veredito + (se A) o desenho + Trava
Registrar no L0 `f_fronteira_e1.md` (os dois públicos) o **veredito medido** (A ou B) com
`file:line`.
- **Se B**: registrar a fronteira da linguagem (definição com layout = código, igual ao vanilla) e
  a conclusão de que o público typst está completo. **Fechar** (sem Estágio de código). Commitar o
  L0 + relatório.
- **Se A**: desenhar o escopo da definição em `.typ`. **TRAVA**: parar no chat para o dono aprovar
  antes do código.

### Estágio 1–2 — (só no veredito A, após aprovação) a definição em `.typ`
Materializar o que a medição desenhou, com o vanilla como oráculo. Aditivo (capacidade nova);
fixtures provam a definição em `.typ` puro; a rede de caracterização e o existente não regridem.

### Estágio Teste — (veredito A)
Fixtures novos provam a capacidade; não-regressão do existente; intactos os limites duros.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
- **Veredito B**: commitar o L0 (a fronteira registrada) + o relatório — `git commit -m "Passo 369 —
  público typst completo por fronteira da linguagem (medido); F completo pelos princípios"`.
- **Veredito A**: com os gates verdes, commitar o código — `git commit -m "Passo 369 — definição de
  elemento em .typ (público typst)"`.
Árvore limpa e commitada. (Para na Trava → commita só o L0.)

---

## Verificação (gates)

```
Fase A: read-only (leitura do vanilla em lab/; probes revertidas; árvore limpa; suíte não re-rodada).
  Saída = o veredito medido (A ou B) com file:line.

Veredito B (sem código):
  build/suíte/lint INALTERADOS (nenhum .rs). Suíte 2742, lint 0/0.
  ACEITAÇÃO: o público typst do cristalino (composição via #let + #set/#show sobre registrados)
    casa o que o vanilla permite em .typ puro; definição com layout = código nos dois (fronteira).
  L0: f_fronteira_e1.md (a fronteira + a confirmação do F completo) + hash sincronizado.

Veredito A (com código, após Trava):
  build limpo; suíte 2742 + os fixtures novos da capacidade (rede +11 e existente inalterados —
    não-regressão); lint 0/0.
  ACEITAÇÃO (oráculo = vanilla): definir um elemento em .typ puro funciona como no vanilla.
  perf: antes/depois (mesma sessão).

INTACTOS (ambos): TextStyle/F-5b (DEBT-61); α/caso 2, morph ==/morph_canon, caso 4, flag P350c,
  Marco G; os 3 numbering; o #set de props de usuário (P368).
lente (instrumento): content→elements = 66 (registrar; não é gate do F).
L0 (critério 5): hash sincronizado ANTES de qualquer código, Trava cumprida (se A).
commit: árvore limpa e commitada ao fim.
```

---

## O que NÃO fazer

- **Não assumir o veredito** antes da medição (Trava 1/6).
- **Não inventar um caminho `.typ`** que o vanilla não tem, nem declarar fronteira que o vanilla
  não impõe (ADR-0107).
- **Não tocar o α / caso 2, o `morph_canon`/`==`, o caso 4, a flag P350c, o Marco G nem o
  `TextStyle`/F-5b.**
- **Não pular a Trava** (se veredito A: o desenho + hash do dono antes do código).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no commit; a decisão é do dono.
- **Não importar a quarentena.**

---

## Relatório (`typst-passo-369-relatorio.md` + resumo no chat)

A **medição do vanilla** (define-se elemento com layout em `.typ` puro, ou exige código?) com
`file:line` e a marca [medido]/[inferido]; o **veredito (A ou B)**; o mapeamento contra o cristalino
(o que ele já permite em `.typ`); o L0 com hash; **se B**: a fronteira registrada e a confirmação do
**F completo pelos princípios** (exceto o F-5b/DEBT-61); **se A**: o desenho aprovado, o código com
`file:line`, os fixtures da capacidade, a não-regressão; a prova de que os limites duros ficaram
intactos; a lente; a perf (se A); **o commit de fecho** (hash); `git status` limpo; lint 0/0; o
caveat de stack. **Termina aqui — não emenda o seguinte.**

## Estado do F após o P369 (a registrar no relatório)

Com o P369 (qualquer que seja o veredito), o **F está completo pelos princípios** — a
extensibilidade (P368 + a definição em `.typ` ou a fronteira medida), a atomização dos numbering
(P364/P365), as folhas confirmadas (P367) — **exceto o F-5b** (lote arquitetural dedicado, DEBT-61).
Os débitos **não-F** permanecem nomeados: DEBT-59 (flag CLI), DEBT-60 (contador), e o **Marco G**
(decisão de modelo α/β, fora do F, não-F).

## Fora de escopo (confirmado)

O **F-5b** (DEBT-61, lote arquitetural); o **Marco G** (não é F); DEBT-59 (flag CLI); DEBT-60
(contador); qualquer toque no α / `morph_canon` / `==` ou na flag.
