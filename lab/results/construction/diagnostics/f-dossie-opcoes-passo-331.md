# Dossiê do F — opções desenhadas, decisão NÃO tomada (P331 Fase 3)

> **Decisão de desenho: ZERO.** Este dossiê apresenta as opções completas; a
> escolha é do dono, no checkpoint. Consolida as 3 frentes do inventário
> (`f-inventario-1{a,b,c}-passo-331.md`), a rede de caracterização (Fase 2,
> `mod f_caracterizacao_estilo`) e o baseline 10× (`medicao-pre-f-passo-330.md`).
> Critério do dono (P329): **fidelidade é de comportamento, não de estrutura
> Rust.** O contrato comportamental é C1–C8 (1b).

---

## 0. O problema, em 3 factos

1. **Há 3 mecanismos de estilo desconexos** (1a §0): (a) `#set text` muta uma
   `StyleChain` em eval e **assa** o `TextStyle` em `Content::Text` (chain
   descartada); (b) `*bold*`/`emph` → `Content::Styled(Box, Styles)` re-resolvido
   numa **2ª `StyleChain`** reconstruída no Layouter; (c) as 4 `Set*` são
   marcadores opacos por **4 canais distintos** (2 via Introspector, `SetPage`
   muta `page_config`, `SetFigureNumbering` assa em `Figure`). Não convergem.
2. **O alvo é pequeno e fechado**: **10 propriedades** de estilo distintas, em
   alinhamento **1:1** `TextStyle` ↔ `enum Style` ↔ getters `StyleChain`
   (1c §3), com ponto único de merge (`layout/mod.rs:587–602`). Sem `PropMap`,
   sem vtable — `Style` é enum fechado manual (`style.rs:18-19`, divergência
   consciente do vanilla `#[elem]`).
3. **O custo do estado misto é ~50 linhas** de arms próprios no hub (1c §1; teto
   68 contando blocos `|`-agrupados), distribuídas por 8 das 12 não-migradas.
   `get_field` não tem arm para nenhuma das 12. **`content.rs` = 4960 linhas.**

O F decide **se e quanto** unificar estes 3 mecanismos — preservando C1–C8.

---

## 1. As opções (4)

Cada opção: desenho (1 página) · custo previsto (preditor de largura, 1c §2) ·
riscos / o que quebra primeiro · cobertura da rede Fase 2 · critério do dono.

### Opção A — **Declarar e congelar** (custo mínimo, unificação zero)

**Desenho.** Não unifica nada. Formaliza os 3 mecanismos como estão: as 4 `Set*`
ficam variantes do hub por desenho (como os 7 primitivos, P329); `Styled`
permanece o wrapper de Bold/Italic; o bake-in de `#set text` em `Content::Text`
fica declarado como decisão. O F vira um **documento de desenho**, não código.

- **Set*/Styled/folhas**: inalterados. `Styled` ganha (se o dono quiser) o arm
  `is_empty` em falta (D2/§bugs) — único toque possível.
- **Fluxo set→consumo**: os 4 canais permanecem.

**Custo previsto.** ~0 sites de produção (só registo + talvez 1 arm). 0 lotes.
**Risco / quebra primeiro.** Nada quebra (não mexe). Risco = **o estado misto
fica permanente**: 50 linhas de arms + 4 canais de `Set*` que qualquer feature
futura de estilo herda; `#show`/`Value::Styles` (D5) ficam bloqueados.
**Rede Fase 2.** Cobre tudo (nada muda). **Critério do dono.** ✓ trivial
(comportamento idêntico por construção).

### Opção B — **PropMap tipado fechado** (cristalino-nativo)

**Desenho.** Um `PropMap` próprio — struct com os **10 campos `Option<T>`** já
existentes em `StyleDelta` (não um `HashMap`, não type-erased) — vira o **único**
portador de estilo. `Styled(Box, PropMap)`; as 4 `Set*` viram entradas
genéricas `Set(PropMap-delta)` (ou propriedades settáveis na chain, à vanilla
C3); `#set text` **deixa de assar** — `Content::Text` carrega só a string e o
estilo resolve-se da chain no layout (paridade C5, alinha com vanilla 1b §3).
Uma só `StyleChain` (a do Layouter), alimentada por eval.

- **Set***: 4 variantes → 1 mecanismo (`SetProperty`/entrada na chain). Os 4
  canais colapsam em 1 (resolução por fallback C1+C3).
- **Folhas provisórias**: `Text`/`MathText`/`MathIdent` deixam de carregar
  `TextStyle` assado → estilo da chain. Revisita os 3 primitivos provisórios
  (P329) — exatamente o que a triagem adiou para o F.

**Custo previsto.** Toca `Styled`(82) + as 4 `Set*`(Σ108) + `Text`(93) + o
ponto de merge + os 2 sites de chain. **~283 sites brutos** (1c §2) →
provavelmente **2–3 lotes** (modelo D já provado; a maior parte é
mecânica/skip-list). Risco real no **de-bake** de `#set text` (mexe no eval e em
quase todo o layouter de texto).
**Risco / quebra primeiro.** O de-bake quebra primeiro: hoje `#set text` resolve
em eval; movê-lo para layout muda **quem** vê o estilo (C5). A fold (C2) tem de
ser replicada no `PropMap` (size em-relativo compõe; `Option None` interno
respeitado). Extensibilidade de 3os (`#[elem]`) **não** suportada (enum fechado)
— aceitável se o cristalino não a quer (1b D1).
**Rede Fase 2.** Cobre numbering (C3), escopo de `Styled` (não-vaza), `SetPage`,
plain_text-vs-layout. **NÃO cobre**: fold em-relativo composto (C2), `#show`
(não existe hoje), precedência instância-vs-chain fina (C1 sub-casos).
**Critério do dono.** ✓ comportamento preservável; estrutura nova livre.

### Opção C — **StyleChain à vanilla** (type-erased, máxima fidelidade estrutural)

**Desenho.** Replicar o modelo vanilla (1b): chain lazy `head: &[Style], tail`,
`enum Style { Property, Recipe, Revocation }`, `Property` type-erased por
`(Element, id) + Box<dyn>`. Suporta **set + show + show-set + recipes** e
extensibilidade por `#[elem]` de 3os. `Styled`, `Set*`, show rules tornam-se
entradas da chain.

**Custo previsto.** O maior. Além dos sites da Opção B, introduz vtable/`dyn`,
`#[elem]`, guards de recipe (1b C4) e `Value::Styles` (D5). **>283 sites** +
infra nova (proc-macro de elemento?). **4+ lotes/passos**; toca a topologia L1
(o `dyn`/`Box` em folha quente reabre o risco ADR-0029/0030 que a migração D
evitou).
**Risco / quebra primeiro.** Igualdade-por-ponteiro da chain é load-bearing
para comemo (1b D2) — replicar a semântica de memoização é subtil. `style.rs:18-19`
diz que o enum fechado foi **escolha consciente**; esta opção reverte-a.
**Rede Fase 2.** Cobre o mesmo subconjunto da B; o grosso da fidelidade nova
(show/recipes) **não tem rede** (não existe comportamento hoje para fixar).
**Critério do dono.** ⚠️ tensão: é fidelidade **estrutural** ao vanilla — o dono
disse que estrutura **não** vincula. Só se justifica se o **comportamento** de
show/recipes for um requisito futuro real.

### Opção D — **Híbrido mínimo: unificar só as 4 `Set*`** (alvo cirúrgico)

**Desenho.** Não toca `Styled` nem o bake-in de `#set text`. Unifica **só** os 4
canais das `Set*` num mecanismo único de "config a partir deste ponto" (uma
entrada na `StyleChain` do Layouter, ou um `SetRegistry` análogo ao
`StateRegistry`/Introspector já existente). `SetHeadingNumbering`/
`SetEquationNumbering`/`SetFigureNumbering`/`SetPage` → 1 caminho de fallback
(C1+C3). Resolve D1 (produtor eval ausente de `SetEquationNumbering`) e D4 (2
produtores de `SetPage`) de caminho.

**Custo previsto.** Toca as 4 `Set*` (Σ108 sites) + os 4 consumidores. **~1–2
lotes** (mais barato que B; `Styled`/`Text` ficam fora). Sem de-bake.
**Risco / quebra primeiro.** `SetPage` é o único `Set*` com efeito **não**
introspectivo (muta `page_config`, `mod.rs:1102`) — unificá-lo com os 3
introspectivos exige cuidado (canais semanticamente diferentes). O bake-in de
`#set text` e as 2 StyleChains **permanecem** — unificação parcial.
**Rede Fase 2.** Cobre os 4 `Set*` diretamente (os testes novos são quase um
spec). **Critério do dono.** ✓ comportamento idêntico; estrutura dos `Set*`
livre. **Deixa `Styled`/folhas/`#show` para um F-2 futuro.**

---

## 2. Comparação rápida

| Opção | Unifica | Custo (preditor) | Lotes | de-bake `#set text` | `#show`/3os | Tensão c/ critério |
|-------|---------|------------------|-------|---------------------|-------------|--------------------|
| **A** Declarar | nada | ~0 | 0 | não | não | nenhuma |
| **B** PropMap fechado | tudo (chain única) | ~283 sites | 2–3 | **sim** | não | nenhuma |
| **C** StyleChain vanilla | tudo + show/recipes | >283 + infra | 4+ | sim | **sim** | ⚠️ fidelidade estrutural |
| **D** Híbrido `Set*` | só as 4 `Set*` | ~108 sites | 1–2 | não | não | nenhuma |

Baseline para o antes/depois do F: **10×: 0.6518 s ± 0.0057 s** (P330). Regra:
o "depois" refaz antes+depois no par de commits, mesmo corpus 10×.

---

## 3. Leitura do executor (claramente marcada — NÃO é a decisão)

Pela lente do critério do dono (comportamento, não estrutura) e da trajetória do
roteiro (modelo D provado, baixo risco, lotes pequenos): **D** e **B** são as que
mais alinham. **D** fecha o problema mais óbvio e mais barato (os 4 canais de
`Set*` desconexos, com 2 bugs de caminho D1/D4) sem o risco do de-bake; **B**
resolve a raiz (3 mecanismos → 1) mas paga o de-bake de `#set text`. **C** só se
o **comportamento** de `#show`/recipes for requisito — não é hoje (não existe).
**A** se a decisão for "o estado misto é aceitável e o F não vale o custo agora".
Sequência possível: **D agora** (cirúrgico) → reavaliar B/C quando `#show` virar
requisito. **Isto é leitura, não recomendação vinculativa.**

---

## §Perguntas ao dono (numeradas, acionáveis)

Formato dos checkpoints dos lotes. Acumulam as dúvidas das Fases 1–2.

- **Q1 (escopo).** Qual opção (A/B/C/D) ou combinação? Se D, segue um F-2 para
  `Styled`/folhas, ou fica?
- **Q2 (de-bake).** O achatamento eval-time de `#set text` em `Content::Text`
  (1a D3) é **decisão a preservar** ou **a unificar** com a chain de layout?
  (decide B/C vs A/D)
- **Q3 (`SetEquationNumbering`).** Sem produtor em eval (1a D1; §bugs) — é
  intencional (numbering de equação não exposto em `#set`) ou lacuna a fechar?
- **Q4 (`SetPage` ×2 produtores).** `eval/rules.rs:256` vs `stdlib/layout.rs:318`
  (1a D4) — redundância markup-vs-builtin ou um é legacy a remover?
- **Q5 (`#show`/`Value::Styles`).** `Value::Styles` está comentado
  (`value.rs:96`, "bloqueia show/set"); `#show` não existe. Reativar show é
  **escopo do F** (→ C) ou fica fora?
- **Q6 (extensibilidade 3os).** O cristalino quer suportar `#[elem]` de
  utilizador (1b D1)? Se **não**, o enum fechado (B) basta; se **sim**, só C.
- **Q7 (comemo/igualdade-ponteiro).** A igualdade-por-ponteiro da chain
  (memoização, 1b D2) é **contrato comportamental** ou detalhe interno livre?
- **Q8 (`Styled.is_empty`).** Adicionar o arm `is_empty` em falta para `Styled`
  (1a D2; §bugs) agora, ou deixar para o F?
- **Q9 (contagem de custo).** O custo dos arms remanescentes é **50** (próprias)
  ou **68** (com blocos `|`-agrupados) para a decisão? (1c D1)
- **Q10 (props efetivas).** Na prática só Bold/Italic criam `Styled` real no
  pipeline (1a D6); as outras 8 props só via `#set text`/testes. O F dimensiona
  pelos **10** ou pelos **~2** efetivos? (1c D4)

---

## §Bugs / divergências (da Fase 2 e do inventário — NÃO consertados)

Registo; conserto é decisão do dono.

- **B1 — `SetEquationNumbering` sem produtor em eval** (1a D1; teste
  `carac_set_equation_numbering_estado_atual` só fixa que o corpo renderiza). O
  efeito de numeração de equação **não é caracterizável via `layout` puro** —
  depende de Introspector pré-populado. Divergência potencial vs vanilla (que
  expõe `#set math.equation(numbering:)`).
- **B2 — `Styled` sem arm `is_empty`** (1a D2): cai no fallback `_ => false`, ao
  contrário de `plain_text`/`map_*` que delegam ao body. Um `Styled(Empty)`
  reporta `is_empty()==false`. Caracterizado como estado atual (a rede não
  asserta este caso para não fixar um possível bug).
- **B3 — `world_types::Styles(())` é stub morto** (1a §1.1): tipo duplicado, só
  usado pelo seu próprio smoke-test. Candidato a remoção (decisão do dono;
  conserto oportunista proibido neste passo).

---

## Adendo P332 — requisito de extensibilidade reabre a decisão

**Não é fecho — é reabertura.** Após este dossiê, o dono fixou que
**extensibilidade total é requisito do projeto** (elemento de utilizador =
cidadão pleno: `#set`/`#show`/`query`/render, sem tocar o core; dois públicos:
Rust e autor typst). Consequências:

- **Opção A (Declarar e congelar) está REJEITADA** — congelaria contra o
  requisito (o estado misto não suporta elementos de 3os).
- **B/C/D re-avaliam-se sob o requisito pelo EXPERIMENTO** (spikes E1/E2/E3,
  P332 — `f-experimento-extensao-passo-332.md`), **não por argumento**. A
  pergunta Q6 (extensibilidade de 3os) deixou de ser opcional: é o eixo da
  decisão.
- A leitura do executor acima (D/B alinham; C só se `#show`) fica **suspensa**
  até a medição — o experimento decide se o custo do `dyn`/type-erased (C/E2) se
  paga pela extensibilidade, ou se um registo aberto por dados (E3) a entrega
  sem `dyn`.

Ver `debt-stylechain-nao-materializada.md` (§Requisito de extensibilidade) e o
experimento P332.

## Adendo final P333 — decisão tomada: fronteira E1

O experimento P332 mediu E1/E2/E3 e o dono **decidiu**: **fronteira E1**
(`Content::Dynamic(Arc<dyn Element>)`, despacho pelo `trait Element` existente,
65 nativos monomórficos). Gravado em **ADR-0106**. Mapeamento para as 4 opções
deste dossiê: E1 **não** é nenhuma das A/B/C/D puras — é a forma que entrega a
extensibilidade total (que A não dá, que C dá ao custo de `dyn` em tudo) com o
PropMap aberto **só** na folha dinâmica (não no caminho nativo). O F materializa-se
sob E1: chain única (10 campos fechados + mapa aberto), `Set*` como primeira
aplicação (eco de **D**), `de-bake` de `#set text` e `Styled` por lotes
incrementais (eco parcial de **B**). **C rejeitada** (≡E2: downcast silencioso,
fidelidade estrutural não-requisito). **A rejeitada** (Parte 0 P332). Este dossiê
fecha aqui; a forma do F passa a viver no **L0 do F** (P333 Parte 3).
