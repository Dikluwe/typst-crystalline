# Passo 366 — F-5b: de-bake do `TextStyle` (`Content::Text`) — fecha o item (1)

> **O que faz.** Fecha o **4º e último caminho duplo** do item (1) da auditoria P362: o `TextStyle`
> do `Content::Text`, hoje **assado** no `node_style` (via `#set text`) **E** disponível na chain
> (`self.style` merge no layout). O campo assado sai; o consumidor lê o estilo **só da chain** →
> **fonte única** (atomização). Justificativa = **princípio (fonte única)**, decidido pelo dono
> (P362). Este é o de-bake de **maior superfície** dos quatro, por uma razão medida (P353/P365): o
> **bold do heading** é assado no **mesmo `node_style`** do `Content::Text` (`markup.rs:85-86`),
> **não** numa chain — então remover o `TextStyle` e ler só a chain faria o **heading perder o
> bold** (a chain não o carrega hoje). Por isso a ordem interna é **bold-do-heading para a chain
> primeiro, depois o `TextStyle`**. A Fase A **varre esse arrasto antes de remover qualquer coisa**
> (Trava 1/6; S5b). **Content-preserving**: output idêntico (paridade pela rede de caracterização).
> **Não toca** o α/caso 2, o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G. **Design-
> first**: L0 + Trava antes de remover campo. **Commita ao terminar** e **não emenda o seguinte**
> (Trava 5).

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P366 (confirmar livre).
**Pré-condição**: P365 fechado (heading + equation + figura de-bakados, fonte única; os 3 caminhos
duplos de numbering fechados; commitado). Suíte **2738**, lint **0/0**, lente **66/0**, árvore
limpa. HEAD pós-P365. Lente `tekt-cargo-dsm` disponível (registrar versão/commit). Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: F-5b de-bake — **content-preserving** (output idêntico; remove a representação dupla). A
**rede de caracterização (+11, P331)** é o oráculo. **Justificativa = princípio (fonte única)**, não
demanda da lente (não re-litigar). Onde uma **fixture** lia o estilo fora da chain, a asserção
**vira** para refletir a leitura da chain — **declarada e justificada** (S5b), nunca silenciosa.

---

## Fase A — a varredura do arrasto vem PRIMEIRO (a fonte vence; `file:line`)

**Antes de remover qualquer campo:**

1. **Reler** ADR-0107, ADR-0108, as **Travas anti-deriva**, e o princípio da fonte única.
2. **O arrasto bold-do-heading × `TextStyle`** (a razão de o F-5b ter sido separado): varrer, com
   `file:line` —
   - onde o `TextStyle` é **assado** no `node_style` do `Content::Text` (`#set text` →
     `TextStyle::from(...)`; os sítios `mod.rs:344/354/371/401`, markup, etc. que o P353 apontou);
   - onde o **bold do heading** (e qualquer outro estilo de heading) entra no `node_style`
     **direto** (`markup.rs:85-86`), **fora** de um wrapper de chain;
   - quais outros elementos põem estilo no `node_style` do texto sem passar pela chain (a varredura
     não assume que é só o heading — mede).
   Classificar cada produtor: **[produção, chain]** (já põe o estilo na chain) / **[produção,
   node_style]** (assa direto → precisa migrar para wrapper de chain) / **[fixture]**.
3. **A separação estilo-resolvido vs morfologia** (ADR-0107/P345): confirmar que o de-bake move o
   **estilo de render** (o `TextStyle` resolvido) para a chain, e que isso **não** toca a
   **morfologia** (`morph_canon`/`==`) — o bold de render nunca foi morfologia (ADR-0107: `it.body
   == [a]` apesar do bold). Se a varredura mostrar que tocam, **parar e reportar**.
4. **A infra do P363** (o merge do layout já lê a chain, `mod.rs:609-626`) + o L0
   `f_fronteira_e1.md`. Sincronizar hashes antes do código.
5. **A rede de caracterização (+11)** como spec de paridade — em especial os fixtures de heading
   bold e de `#set text`.

---

## A ordem interna e a forma — decididas pela varredura (não antes dela)

- **Primeiro o bold do heading (e os estilos [produção, node_style]) para a chain**: o produtor que
  assa estilo direto no `node_style` passa a embrulhar num wrapper de chain (`Content::Styled` +
  custom), para a chain carregar o estilo antes de o campo sair. Aditivo.
- **Depois o de-bake do `TextStyle`**: remover o campo assado; o consumidor lê o estilo só da chain
  (o merge `self.style` já existe no layout).
- **Fixture fora da chain** → asserção virada (declarada, justificada).
- Se a varredura achar um produtor que **não** pode migrar para a chain sem quebrar outra coisa,
  **parar e reportar** (re-escopo).

---

## Limites duros

- **Não remover o `TextStyle` antes de o bold do heading (e os demais [produção, node_style]) estar
  na chain.** Senão o heading perde o bold (regressão). A ordem interna é medida, não opcional.
- **Não tocar a morfologia.** O de-bake move estilo de **render**; o `morph_canon`/`==` (P345) e o
  caso 4 ficam intactos (o bold nunca foi morfologia — ADR-0107).
- **Não tocar o α / caso 2, a flag P350c nem o Marco G.**
- **Não re-litigar a demanda** (o dono decidiu pelo princípio).
- **Content-preserving.** Output idêntico via chain (paridade, rede). As únicas asserções que mudam
  são as de fixtures que liam o estilo fora da chain — declaradas.

---

## Estágios

### Estágio L0 — desenho (com a varredura na mão) + Trava (PARA aqui para o dono)
Com a classificação dos produtores e o arrasto medido, desenhar: quais estilos [produção,
node_style] migram para wrapper de chain, a ordem (bold-do-heading antes do `TextStyle`), e quais
asserções de fixture viram. Registrar no L0 `f_fronteira_e1.md`. Sincronizar hashes. **TRAVA**: para
no chat — varredura + desenho + hashes para o dono aprovar. Nenhum campo removido antes.

### Estágio 1 — migrar os estilos [produção, node_style] para a chain (após aprovação)
O bold do heading (e os demais que a varredura achou) passam a entrar pela chain (wrapper
`Content::Styled` + custom), não pelo `node_style` direto. Aditivo — o `TextStyle` ainda existe; só
se garante que o estilo chega à chain por todos os caminhos de produção.

### Estágio 2 — de-bake do `TextStyle`
Remover o campo `TextStyle` assado do `Content::Text`; o consumidor lê o estilo da chain (o merge
`self.style`). **Fonte única.**

### Estágio Teste
- A **rede de caracterização (+11)** passa — spec de paridade. As asserções que mudam são **só** as
  de fixtures que liam o estilo fora da chain; cada uma justificada.
- **Novo / virado**: o texto (e o heading bold) lê o estilo **só da chain** (campo removido); output
  **idêntico** (paridade). Um teste prova que o heading **mantém o bold** (o arrasto foi resolvido).
- Confirmar que o `morph_canon`/`==`, o caso 4, o α/caso 2 e a flag ficaram **intactos**; e que os
  3 numbering (P364/P365) não regridem.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

### Estágio de fecho — COMMIT (padrão)
Com os gates verdes, **commitar**: `git add -A && git commit -m "Passo 366 — de-bake do TextStyle
(fonte única); fecha o item (1)"`. Árvore limpa e commitada. (Para na Trava → commita só o L0;
reverte por contradição → commita só o relatório.)

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2738, paridade pela rede. Asserções alteradas = SÓ as de fixtures
  que liam o estilo fora da chain (declaradas, justificadas, listadas). Nenhuma outra muda.
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável; oráculo = rede de caracterização / vanilla):
  - texto e heading: output idêntico (paridade), estilo lido SÓ da chain.
  - o heading MANTÉM o bold (o arrasto foi resolvido — bold na chain antes de o campo sair).
  - fonte única: o TextStyle assado REMOVIDO do Content::Text; só a chain carrega o estilo.

INTACTOS (confirmar): morph ==/morph_canon (P345 — o bold de render não é morfologia), caso 4,
  α/caso 2, flag P350c, Marco G; os 3 numbering (P364/P365) — não regridem; o NÚMERO/contador.

lente (instrumento): content→elements = 66 (registrar; não é gate do F).
perf (critério 4): antes = baseline da mesma sessão (re-medir no início); depois reportado.
L0 (critério 5): f_fronteira_e1.md + hash sincronizado ANTES do código, Trava aprovada.
commit: árvore limpa e commitada ao fim.
```

---

## Válvula declarada

Se a varredura achar **muitos** estilos [produção, node_style] a migrar (a migração para chain + o
de-bake não cabem num lote), fatiar (ex.: migração dos estilos P366 / remoção do `TextStyle` P367).
Registrar a fatia e o número medido.

---

## O que NÃO fazer

- **Não remover o `TextStyle` antes de o bold do heading estar na chain** (regressão; ordem medida).
- **Não tocar a morfologia / `morph_canon` / `==` / caso 4** (o de-bake é estilo de render).
- **Não tocar o α / caso 2, a flag P350c nem o Marco G.**
- **Não re-litigar a demanda** (o dono decidiu pelo princípio).
- **Não pular a Trava** (varredura + L0 + hash do dono antes de remover campo).
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no commit; a decisão é do dono.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-366-relatorio.md` + resumo no chat)

A **varredura do arrasto** com `file:line` (onde o `TextStyle` é assado, onde o bold do heading
entra no `node_style`, e quais outros produtores; classificados produção-chain / produção-node_style
/ fixture, marcado medido vs inferido); o L0 com hash e a Trava aprovada; a migração dos estilos para
a chain e a remoção do `TextStyle`, com `file:line` e a ordem; a paridade pela rede e a **lista** das
asserções de fixture que viraram (com justificativa); a prova de que o heading mantém o bold, e de
que a morfologia/caso 4/α/flag e os 3 numbering ficaram intactos; a medição da lente; a perf; **o
commit de fecho** (hash); `git status` limpo; lint 0/0; o caveat de stack. **A nota de que o item
(1) da auditoria fecha com este lote.** Termina aqui — não emenda o seguinte.

## Próximo (fixado pela ordem, não a decidir)

Com o item (1) fechado, o próximo é o **item (2) — F-6 (3 folhas, DEBT-58)**, e depois o **item (3)
— o mapa aberto para `#set <elemento-de-usuário>(prop:)`** (a extensibilidade que depende da chain
ser a fonte única, agora estabelecida pelos de-bakes). Eu escrevo os dois em sequência; não há
decisão a tomar entre eles além do que a Fase A de cada um medir.

## Fora de escopo (confirmado)

O **F-6** (item 2 — próximo); o **`#set` de props de usuário** (item 3 — depois); o **Marco G** (não
é F); DEBT-59 (flag CLI); DEBT-60 contador; qualquer toque no α / `morph_canon` / `==` ou na flag.
