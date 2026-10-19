# Passo 367 — F-6: as 3 folhas (`Text`/`MathText`/`MathIdent`) pela chain — item (2)

> **O que faz.** Item (2) da auditoria P362: as 3 folhas provisórias — `Content::Text`,
> `MathText`, `MathIdent` (DEBT-58) — recebem estilo **pela chain** em vez de campos assados, no
> modelo de canal único do F. Justificativa = **princípio (fonte única / atomização)**, decidido
> pelo dono (P362), não demanda da lente. **A Fase A abre por uma verificação de fronteira**: o
> de-bake das folhas **toca** o `TextStyle` do `Content::Text` que o **F-5b deixou assado** (adiado
> no P366 por colidir com o α-fixpoint e o modelo de morfologia strong/emph)? Se tocar, o F-6
> **desvia dessa folha** ou **para** — não esbarra no mesmo bloqueio. **Varredura antes de remover
> campo** (Trava 1/6; S5b). **Content-preserving**: output idêntico (paridade pela rede de
> caracterização). **Não toca** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag P350c, o Marco G,
> nem o `TextStyle` do `Content::Text` (F-5b, adiado). **Design-first**: L0 + Trava antes de código.
> **Commita ao terminar** e **não emenda o seguinte** (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P367 (confirmar livre).
**Pré-condição**: P365 fechado (3 numbering com fonte única); **P366 fechado como adiamento
medido** (F-5b adiado — `Content::Text` `TextStyle` permanece assado, divergência registrada no
DEBT com a razão do α-fixpoint/colapso P101). Suíte **2738**, lint **0/0**, lente **66/0**, árvore
limpa. HEAD pós-P366. Lente `tekt-cargo-dsm` disponível (registrar versão/commit). Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: F-6 — **content-preserving** (output idêntico). A **rede de caracterização (+11, P331)** é
o oráculo. **Justificativa = princípio (fonte única)**, não demanda da lente (não re-litigar). Onde
uma **fixture** lia o estilo da folha de um campo, a asserção **vira** para a chain — **declarada e
justificada** (S5b).

---

## Fase A — abre pela fronteira com o F-5b, depois a varredura (a fonte vence; `file:line`)

**Antes de remover qualquer campo:**

1. **Reler** ADR-0107, ADR-0108, as **Travas anti-deriva**, e o princípio da fonte única.
2. **Verificação de fronteira com o F-5b (a PRIMEIRA coisa)** — o F-5b foi adiado (P366) porque
   de-bakar o `TextStyle` do `Content::Text` toca o **α-fixpoint** (o `morph_canon` serve o
   `rules.rs:229`) e o **modelo de morfologia** (`#set text(bold)` usa o mesmo campo tipado que
   `*bold*`). Medir, com `file:line`: o de-bake de **cada** uma das 3 folhas **toca** esse mesmo
   `TextStyle`/`node_style` ou o `morph_canon` dos campos tipados?
   - Se uma folha (provavelmente o `Content::Text`) **só** difere pelo `TextStyle` que o F-5b
     adiou → essa folha **fica fora do F-6** (é F-5b, não F-6); o F-6 cobre as outras.
   - Se as folhas de math (`MathText`/`MathIdent`) recebem estilo por **outro** caminho
     (`self.style` por parâmetro, sem campo tipado de `Styled`/`morph_canon`) → são de-bakáveis sem
     o bloqueio do F-5b. **Medir, não assumir.**
3. **A varredura das 3 folhas** — produtores e consumidores de `Text`/`MathText`/`MathIdent`, com
   `file:line` e a contagem (a auditoria citou "Text 4/7, MathText 6/5, MathIdent 2/5" — **confirmar
   no repo, não herdar**). Classificar cada produtor: **[produção, chain]** / **[produção, campo]**
   / **[fixture]**.
4. **A condição C1 de tampão (P338)** — se o F-6 rotear uma folha pela chain **mantendo** o campo
   assado (content-preserving), o caminho duplo **não pode dormir**: nasce com **gatilho de remoção**
   + **teste de paridade entre os dois caminhos**. Medir se o F-6 é remoção direta ou tampão; se
   tampão, a C1 é obrigatória.
5. **A infra do P363** (introspect-chain) + o merge do layout (`mod.rs:609-626`, já lê a chain) + o
   L0 `f_fronteira_e1.md` (DEBT-58). Sincronizar hashes antes do código.
6. **A rede de caracterização (+11)** como spec de paridade.

---

## A forma do de-bake — decidida pela varredura (não antes dela)

- **Folha que não toca o `TextStyle`/`morph_canon` adiado** (provavelmente `MathText`/`MathIdent`,
  param-styled) → de-bake: o campo sai, o consumidor lê a chain. Fonte única.
- **Folha que toca o bloqueio do F-5b** (o `Content::Text` `TextStyle`) → **fora do F-6** (é F-5b);
  registrar e não tocar.
- **Tampão (se inevitável)** → C1: gatilho de remoção + teste de paridade dos dois caminhos.
- **Fixture fora da chain** → asserção virada (declarada, justificada).
- Caso que não cabe → **parar e reportar** (re-escopo).

---

## Limites duros

- **Não tocar o `TextStyle` do `Content::Text`** — é o F-5b, adiado (P366); o F-6 desvia dele.
- **Não tocar o α / caso 2, o `morph_canon`/`==`, o caso 4, a flag P350c nem o Marco G.** Se a
  varredura mostrar que de-bakar uma folha exige tocar o `morph_canon` dos campos tipados (o
  bloqueio do F-5b), **parar e reportar** — essa folha é F-5b, não F-6.
- **Não re-litigar a demanda** (o dono decidiu pelo princípio).
- **Content-preserving.** Output idêntico via chain (paridade, rede). As únicas asserções que mudam
  são as de fixtures que liam o estilo da folha de um campo — declaradas.
- **Sem caminho duplo dormindo** — remoção direta, ou tampão com C1 (gatilho + teste de paridade).

---

## Estágios

### Estágio L0 — desenho (com a varredura na mão) + Trava (PARA aqui para o dono)
Com a fronteira-F-5b resolvida e a classificação dos produtores, desenhar: quais folhas o F-6 cobre
(e quais ficam para o F-5b), remoção direta vs tampão (com C1 se tampão), quais asserções de fixture
viram. Registrar no L0 `f_fronteira_e1.md` (DEBT-58). Sincronizar hashes. **TRAVA**: para no chat —
varredura + fronteira-F-5b + desenho + hashes para o dono aprovar. Nenhum campo removido antes.

### Estágio 1 — rotear/de-bakar as folhas cobertas (após aprovação)
Para cada folha que o F-6 cobre: o estilo passa a vir da chain; o campo assado sai (ou, se tampão,
o campo fica com o gatilho de remoção + teste de paridade — C1). **Fonte única** para as folhas
cobertas.

### Estágio Teste
- A **rede de caracterização (+11)** passa — spec de paridade. As asserções que mudam são **só** as
  de fixtures que liam o estilo da folha de um campo; cada uma justificada.
- **Novo / virado**: cada folha coberta lê o estilo **só da chain** (campo removido, ou tampão com
  C1); output **idêntico** (paridade).
- Confirmar que o `TextStyle`/F-5b não foi tocado; e que o α/caso 2, o `morph_canon`/`==`, o caso 4,
  a flag, e os 3 numbering (P364/P365) ficaram intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes, **commitar**: `git add -A && git commit -m "Passo 367 — F-6: folhas pela chain
(fonte única)"`. Árvore limpa e commitada. (Para na Trava → commita só o L0; reverte por contradição
→ commita só o relatório.)

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2738, paridade pela rede. Asserções alteradas = SÓ as de fixtures
  que liam o estilo da folha de um campo (declaradas, justificadas, listadas). Nenhuma outra muda.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável; oráculo = rede de caracterização / vanilla):
  - cada folha coberta: output idêntico (paridade), estilo lido SÓ da chain.
  - fonte única: o campo assado da folha REMOVIDO (ou tampão com C1: gatilho + teste de paridade).

INTACTOS (confirmar): o TextStyle/F-5b (não tocado, adiado); α/caso 2, morph ==/morph_canon (P345),
  caso 4, flag P350c, Marco G; os 3 numbering (P364/P365) — não regridem.

lente (instrumento): content→elements = 66 (registrar; não é gate do F).
perf (critério 4): antes = baseline da mesma sessão (re-medir no início); depois reportado.
L0 (critério 5): f_fronteira_e1.md (DEBT-58) + hash sincronizado ANTES do código, Trava aprovada.
commit: árvore limpa e commitada ao fim.
```

---

## Válvula declarada

Se a varredura achar que as 3 folhas não cabem num lote, ou que mais de uma toca o bloqueio do
F-5b, fatiar (ex.: as folhas de math num lote; a folha de texto fica para o F-5b). Registrar a
fatia e o número medido.

---

## O que NÃO fazer

- **Não tocar o `TextStyle` do `Content::Text`** (F-5b, adiado) nem o bloqueio do α/`morph_canon`.
- **Não remover campo antes da varredura** (Trava 1/6; S5b).
- **Não tocar o α / caso 2, o `morph_canon`/`==`, o caso 4, a flag P350c nem o Marco G.**
- **Não deixar caminho duplo dormindo** (remoção direta, ou tampão com C1).
- **Não re-litigar a demanda** (o dono decidiu pelo princípio).
- **Não pular a Trava** (varredura + L0 + hash do dono antes de remover campo).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no commit; a decisão é do dono.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-367-relatorio.md` + resumo no chat)

A **verificação de fronteira com o F-5b** (quais folhas tocam o bloqueio adiado, quais não) com
`file:line`; a **varredura das 3 folhas** (produtores/consumidores, contagem confirmada, classes,
medido vs inferido); o L0 com hash e a Trava aprovada; o de-bake/roteamento das folhas cobertas
(remoção direta ou tampão com C1) com `file:line`; a paridade pela rede e a **lista** das asserções
de fixture que viraram (com justificativa); a prova de que o `TextStyle`/F-5b, o α/caso 2, o
`morph_canon`/`==`, o caso 4, a flag e os 3 numbering ficaram intactos; a medição da lente; a perf;
**o commit de fecho** (hash); `git status` limpo; lint 0/0; o caveat de stack. **Termina aqui — não
emenda o seguinte.**

## Próximo (fixado pela ordem, não a decidir)

Com o item (2) coberto (as folhas que o F-6 alcança), o próximo é o **item (3) — o mapa aberto para
`#set <elemento-de-usuário>(prop:)`** (a extensibilidade prometida que depende da chain ser a fonte
única de estilo, já estabelecida pelos numbering). O `TextStyle`/F-5b fica como **lote arquitetural
dedicado** (modelo strong/emph/styled + α-fixpoint), registrado no DEBT, quando o dono quiser
re-desenhar o modelo de morfologia.

## Fora de escopo (confirmado)

O **`TextStyle` do `Content::Text` / F-5b** (adiado — lote arquitetural dedicado); o **`#set` de
props de usuário** (item 3 — próximo); o **Marco G** (não é F); DEBT-59 (flag CLI); DEBT-60 contador;
qualquer toque no α / `morph_canon` / `==` ou na flag.
