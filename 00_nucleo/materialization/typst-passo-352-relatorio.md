# Passo 352 — Relatório de Fase A (medir antes de decidir, ADR-0108)

> **Veredito da Fase A.** A medição (fonte + sítios reais) **contradiz o enquadramento do
> passo** em dois pontos materiais e faz **caso 1 colidir com um limite duro**. Aplicou-se a
> fonte (ADR-0108 regra 1/5; precedente P351): a Fase A parou na Trava, o dono **aprovou a
> fatia re-escopada (caso 3 / show-set)**, e a execução está na **§8**. Caso 1 (composição)
> fica fora — decisão de desenho do dono. **Estágio 0 (transporte) dispensado** (medido).

**HEAD**: 138e0b0c2 (pós-P351). **Branch**: Tekt. **Suíte baseline**: `typst-core` **2729 /
0 falhas** (medido, `RUST_MIN_STACK=33554432`). Árvore limpa fora de `lab/` e materialization.

---

## 0 — Leituras da Fase A (fonte vence; `file:line`)

1. **ADR-0107** (`typst-adr-0107-...md`) — paridade é com a **linguagem**
   (semântica/sintaxe/morfologia), nunca a mecânica. Não construir infra à frente da demanda.
2. **ADR-0108** (`typst-adr-0108-...md`) — regra 1 (medir→decidir), regra 5 (desconfiar do
   enquadramento cômodo), regra 6 (aceitação no nível da língua, exceto onde a mecânica **é** o
   observável — a mensagem de erro). Aplicadas à letra abaixo.
3. **L0 da realização** — `00_nucleo/prompts/entities/f_fronteira_e1.md` (hash `12c03c65`).
   - §3c **S5**: o desenho prevê `Transformation = Content | Func | Style` (show-set não
     consome passe).
   - §3a.7-bis: **caso 2 fechado por α** (ponto-fixo morfológico, P348); **multi-passe/`Guarded`
     dispensados** para a recursão de element rules; caso 4 fechado P340.
   - §3a.8 (**fatia 1, P339**): o **transporte já existe** — `Content::Styled` reutilizado como
     o nó `StyledElem`-scoped + `Styles::push_custom`.
4. **L0 de `show.rs`** — `00_nucleo/prompts/entities/show.md` (hash `21e02485`): governa
   `ShowRule`/`Selector`. Um `Transformation::Style` tocaria **este** L0 (não só o `f_fronteira_e1`).
5. **Spike-2** — `f-spike2-show-passo-333.md`: B1 (caso 1, innermost-first, 1 regra/passe,
   ambas aplicam), B3/S5 (caso 3, show-set = `Transformation::Style`, `map.apply` + `continue`,
   não consome passe).
6. **Recon** — `f-recon-passo-337.md` A4/B: ~15 sítios de show-state; 20 testes de `#show`
   (contagem do P337, **não herdada** — re-medida abaixo).

---

## 1 — Semântica do vanilla (leitura da quarentena, `file:line`; nunca importada)

### Caso 1 — ordem de composição (multi-regra)
`lab/typst-original/crates/typst-realize/src/lib.rs:449-486` + `.../styles.rs:835` (`next_back`).
- Recipes iteradas **innermost-first** = **mais-recente-declarada primeiro** (`next_back` consome
  a cabeça em reverso).
- A **primeira** recipe func não-guardada vira o passo; as demais func são saltadas nesse passe
  (`if step.is_some() continue`, lib.rs:467-469).
- Cada recipe aplicada é **guardada por `RecipeIndex`** (`elem.is_guarded(index)`, lib.rs:472-474),
  guard **por-instância-de-elemento** (`meta().lifecycle` bitset, `content/mod.rs:148-156`).
- Loop multi-passe externo até fixpoint. **Resultado: N regras sobre o mesmo elemento aplicam-se
  TODAS, uma vez cada, innermost-first**, ao longo de N passes.

### Caso 3 — show-set
`lab/.../styles.rs:531-537` (`Transformation { Content, Func, Style }`), `lib.rs:458-464`:
- `Transformation::Style(t) => { if !prepared { map.apply(t) } continue; }` — **empurra os
  styles para a chain (`map`) e NÃO consome o passo**. O elemento renderiza sob a chain
  aumentada (`styles.chain(&map)`, lib.rs:358). É um mecanismo de **estilo-na-subárvore**, não
  uma transformação de conteúdo.

---

## 2 — Comportamento ATUAL do cristalino (medido empiricamente; probes temporários, revertidos)

Probes adicionados a `eval/tests.rs`, corridos, **revertidos** (`git checkout`; suíte volta a 2729).

### Caso 1 — composição
- **`#show heading: it=>[A:]+it.body` ⨁ `#show heading: it=>[B:]+it.body`, `= T`**
  → cristalino: **`"A:T"`** (só a regra **A**, a **primeira declarada**, aplica; B nunca).
  Vanilla: aplica a **mais-recente (B)** primeiro → **`"B:T"`**. **Divergência de ordem.**
- **`#show heading: upper` ⨁ `#show heading: it=>[X:]+it.body`, `= titulo`**
  → cristalino: **`"TITULO"`** (só `upper`, primeira declarada). Vanilla: `[X:]+body` (mais
  recente) → **`"X:titulo"`**. **Divergência de ordem.**
- **Causa medida** (`rules/eval/rules.rs:121-189`): o loop α aplica a **primeira regra que casa
  em ordem de declaração** e `break`; a 2ª regra do mesmo kind nunca entra. Quando o output deixa
  de casar (vira `Sequence`), o loop termina com **1** regra aplicada.

### Caso 3 — show-set
- **`#show heading: set text(bold: true)`, `= titulo`**
  → cristalino: **ERRO** `"show rule com selector de tipo requer função ou Content, recebeu none"`.
- **Causa medida** (`rules/eval/rules.rs:586-668` + `mod.rs:524`): `eval_show_rule` avalia o
  transform por `eval_expr` → `Expr::SetRule` → `eval_set_rule`, que (i) **muta `engine.styles`
  globalmente na declaração** (escopo errado) e (ii) devolve `Value::None`. A `ShowRule` nasce com
  `transform: Value::None`; ao interceptar o heading, o arm `other =>` (rules.rs:178-186) **erra**.
  Não há `Transformation` no cristalino (`ShowRule.transform: Value`, `show.rs:49`).

---

## 3 — A pergunta central da Fase A: o transporte (Estágio 0) é exigido?

**Medido: NÃO. Estágio 0 (transporte novo) DISPENSADO.**
- O **transporte `StyledElem`-scoped já existe** (fatia 1, P339): `Content::Styled` +
  `Styles::push_custom` (`style.rs:225`, `style_chain.rs:175`). Caso 3 (show-set) **reutiliza-o**
  como carregador dos styles do set — não pede transporte novo.
- Caso 1 (composição) **não é um problema de transporte**: a divergência é de **ordem + modelo de
  guard** no loop `apply_show_rules`, não de carregar estilo na árvore.
- *O que refutaria:* se um caso de composição/show-set exigisse confinar estilo a uma subárvore de
  forma que `Content::Styled` não suporte (não medido — nenhum dos dois exige).

---

## 4 — A colisão de caso 1 com um LIMITE DURO (a descoberta que muda o passo)

O limite duro do passo: **«Caso 2 (recursão) não é tocado; mexer é regressão. O `morph_canon`/`==`
ficam intactos.»** O caso 2 é implementado **pelo próprio loop α** em `apply_show_rules`
(`rules.rs:97-248`): a **mesma** regra é **reaplicada** ao seu output em evolução até **ponto-fixo
morfológico** (teste `p348_show_recursao_converge_para_ponto_fixo`: a→b→c, `tests.rs:692`).

A composição fiel ao vanilla (caso 1) exige que **cada regra aplique no máximo uma vez por nó**
(guard por recipe-index, vanilla `lib.rs:472`), para que a 2ª regra entre. **Mas** um guard
«regra-aplicada-uma-vez» **quebra o α**: na recursão a→b→c a mesma regra precisa **reaplicar** ao
seu próprio output — guardá-la pararia em **b**, regredindo o teste P348.

- **Medido:** os dois modelos partilham o **mesmo** loop; o guard que o caso 1 precisa é
  **incompatível** com o re-apply que o caso 2 (α, fechado) precisa.
- **Inferido (marcado):** não há reconciliação trivial. Rastrear `(RuleId, morph_canon)` não
  resolve — na composição o output de A é uma morfologia nova, então A reaplica e B continua sem
  vez. *O que refutaria:* um modelo de loop que aplique cada regra distinta uma vez **e** preserve
  a→b→c, sem guard por-instância (a mecânica que P347d/P348 recusaram por ser «GEROU»). Não
  encontrado nesta Fase A.
- **Subconjunto seguro:** a **divergência de ordem** (probes acima) corrige-se sozinha invertendo a
  iteração de `node_rules` para innermost-first (mais-recente-primeiro) — **sem** tocar a semântica
  de recursão (testes de caso 2 usam 1 regra; reversão é no-op para eles;
  `show_rule_encadeamento_duas_regras` inalterado pois a regra de emph não casa heading). Mas isto
  cobre **só** o subconjunto «output deixa de casar»; o caso 1 canônico do spike (output **continua**
  a casar, 2 regras, ambas aplicam — B1) **continua** a exigir o guard que colide com α.

**Conclusão de caso 1:** parte é segura (ordem), parte colide com um **limite duro** (multi-apply
vs α). Não é uma «mecânica mais leve»; é redesenho do loop que implementa o caso 2 fechado.

---

## 5 — Caso 3 (show-set): bounded, seguro, exigido pela fonte

Implementável **sem tocar** o loop α (é um arm aditivo): (a) `eval_show_rule` deteta transform
`Expr::SetRule` e **captura o `StyleDelta` sem mutar `engine.styles`**; (b) `ShowRule` carrega um
`Transformation::Style` (S5, exige editar o L0 `show.md` + `f_fronteira_e1.md` §3c); (c) no
`apply_show_rules`, quando uma show-set casa, **embrulha em `Content::Styled(elem, styles)`** (o
carregador da fatia 1) e **`continue`** (não consome passe), espelhando `map.apply` do vanilla.
Não toca caso 2/α, caso 4, `morph_canon`/`==`, nem a flag P350c.

---

## 6 — Recontagem dos testes de `#show` (não herdada do P337)

`grep -nE "fn (show_rule_|eval_show_rule_|f3s2_|f3s3|f3s1_)" eval/tests.rs`: 22 funções.
Por caso (medido pela leitura de cada uma):
- **Caso 1 (composição multi-regra mesmo kind):** **0** testes hoje (os existentes são kinds
  distintos ou encadeamento por interceção aninhada). Lacuna real — o caso não está coberto.
- **Caso 3 (show-set):** **0** testes (não existe; o caminho **erra**).
- Caso 2 (recursão/α): `p348_*`, `f3s2_show_callout_anti_recursao_termina`,
  `show_rule_nao_recursiva...` — **fechados, intactos**.
- Caso 4 (escopo): `f3s3_caso4_escopo_show_confina_no_content_block` — **fechado P340, intacto**.

---

## 7 — Decisão proposta ao dono (a fonte aplicada; o dono audita a substância — ADR-0108)

1. **Estágio 0 dispensado** (medido): reutiliza-se o `Content::Styled` da fatia 1. Sem transporte novo.
2. **Inverter a válvula declarada do passo:** o passo propõe P352=caso 1, P353=caso 3. A medição
   diz o **inverso** quanto a risco: **caso 3 é o seguro e exigido pela fonte**; **caso 1 colide com
   o limite duro (α)**. Recomenda-se **P352 = caso 3 (show-set)**.
3. **Caso 1 fica para decisão de desenho do dono:** a composição fiel exige tocar o loop do caso 2
   (fechado) — fora dos limites duros deste passo. Precisa de um L0/ADR que reconcilie composição
   (multi-apply innermost-first) com α (re-apply morfológico) **ou** que aceite uma divergência
   consciente declarada. Não se escreve esse código sem o desenho.

A Fase A parou aqui (Trava do CLAUDE.md): L0 redigido, hash a sincronizar. **O dono aprovou
"executar o novo 352" (caso 3) — a execução está na §8.**

---

## 8 — Execução (pós-aprovação do dono): caso 3 / show-set materializado

### L0 (Trava cumprida; hashes sincronizados por `crystalline-lint --fix-hashes`)
- `entities/show.md` — reescrito: `Selector::DynKind`, modelo α, **`Transformation` enum**
  (`Func | Content | Str | Style`) com a semântica show-set; §«Fora de escopo» = caso 1.
  Código `show.rs` → hash `edc8666b`.
- `rules/eval.md` — bullet show-set (captura sem mutar `engine.styles`; arm aditivo).
- `entities/style.md` — `Styles::from_delta(StyleDelta)` (P352).
- `entities/style_chain.md` — `StyleChain::collapse() -> StyleDelta` (P352, read-only, top-wins).
- `f_fronteira_e1.md` — **sem mudança** (§3c S5 já especificava `Transformation`; hash intacto).

### Código (L1, content-preserving exceto os 4 testes novos de show-set)
1. **`entities/show.rs`** — `enum Transformation { Func, Content, Str, Style }`; `ShowRule.transform:
   Transformation` (era `Value`). Construção contida (1 sítio: `eval_show_rule`).
2. **`entities/style.rs`** — `Styles::from_delta` (constrói a fachada a partir de um `StyleDelta`).
3. **`entities/style_chain.rs`** — `StyleChain::collapse` (dobra a cadeia num `StyleDelta`
   preservando `Option`; usado para extrair o efeito de um set sobre `empty()`).
4. **`rules/eval/rules.rs`**:
   - `selector_matches(work, sel)` extraído (partilhado pelo loop α e pelo show-set).
   - `capture_set_styles(set, …)`: **swap** de `engine.styles` por `StyleChain::empty()`, reusa
     `eval_set_rule` (fiel a todos os targets/warns/erros), `collapse` o efeito, restaura. **L1
     puro** (sem I/O; só RAM). O `engine.styles` global **não** é mutado (a divergência que o P352
     fecha — medida na Fase A, §2).
   - `eval_show_rule`: classifica o transform — `Expr::SetRule` → `Transformation::Style(captura)`
     (erro se selector for texto); senão `Value`→`Func|Content|Str`.
   - `apply_show_rules`: o loop α **salta** `Transformation::Style` (não consome passe); após o
     loop, embrulha o nó em `Content::Styled(elem, collapse(styles casados))` se algum show-set
     casa. Espelha `map.apply(transform); continue` do vanilla (`typst-realize:458-464`).

### Aceitação (observável, nível da língua — ADR-0107/0108)
- `#show heading: set text(bold: true)` **não erra** (antes errava — Fase A §2); o heading é
  embrulhado num `Content::Styled(bold=true)`, **o elemento sobrevive** (não substituído).
  Teste `show_set_text_embrulha_heading_em_styled_bold`.
- **Não consome o passe** (o heading permanece): `show_set_nao_consome_o_passe_preserva_o_elemento`.
- **Não vaza estilo global** (parágrafo de fora não fica bold — paridade-chave vs o eager antigo):
  `show_set_nao_vaza_estilo_global`.
- Show-set sobre selector de texto → **erro** (invariante): `show_set_em_selector_de_texto_e_erro`.
- *Nota de fronteira (render, fora de escopo F-5/F-6):* o `Content::Styled` carrega o estilo na
  **morfologia**; o efeito de **render** assado (`TextStyle` do `Content::Text`) só muda com o
  de-bake (F-5). Aceitação é ao nível da língua, não dos bytes de render (ADR-0107).

### Gates (todos verdes)
- **build**: workspace limpo (só warnings pré-existentes de import).
- **suíte** (`RUST_MIN_STACK=33554432`): **2729 → 2733** (+4 testes de show-set; nenhuma asserção
  existente mudou — refactor `Value`→`Transformation` content-preserving). Demais crates: 472/24/2/21, 0 falhas.
- **lint** `crystalline-lint .`: **✓ 0 violations** (0/0).
- **caso 2 (α) + caso 4 (escopo) + flag P350c + `morph_canon`/`==`**: **INTACTOS** — 0 regressões
  (todos os respetivos testes passam sem alteração).
- **Trava ADR-0105 cl.3**: caminho dinâmico inalterado (`DynKind` ainda guardado por `RuleId` +
  teto-64).
- **lente (critério 3)**: `edges(content → elements::*) = 66` **INALTERADO**; `edges(elemento →
  elemento) = 0` (medido com `lente --pacote typst-core --estrutura --filtrar-stdlib`). A fatia
  toca `rules/eval` + entidades de estilo/show — **não** a topologia content↔elemento.
- **perf (critério 4)**: **não re-medida** — o rig/fixture do P330 (0.6518 s ± 0.0057) **não está
  no repo** e um bench novo não seria comparável (ADR-0108: afirmar só o medido). Justificação
  estrutural: o caminho quente é **aditivo** — `apply_show_rules` mantém o early-return
  (`rules.is_empty`); o loop α ganha 1 `matches!` por regra (O(regras), negligível); o wrap só
  corre quando há regra show-set casada. Sem mudança de alocação no caminho sem show-set.

### Caso 1 (composição) — confirmado FORA, decisão de desenho do dono
Registrado em `show.md §Fora de escopo` e §4 deste relatório: composição fiel exige guard
por-regra incompatível com o re-apply de α (caso 2, fechado). Não materializado.
