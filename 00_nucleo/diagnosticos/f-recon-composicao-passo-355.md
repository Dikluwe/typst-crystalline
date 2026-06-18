# F-recon composição (P355) — composição same-kind é LÍNGUA; reconciliar (não declarar divergência)

> **Origem.** O P354 recomendou e o dono escolheu o **rumo (B)** (declarar a composição same-kind
> como divergência mecânica, ADR-0109). Na **Trava** de selagem do P355, o dono aplicou a ADR-0108 à
> própria ADR: *"a classificação língua-vs-mecânica que autoriza divergir só pode vir da fonte,
> medida — não de raciocínio. A acumulação de N regras same-kind é mecânica de realização, ou é
> semântica que o usuário do Typst conhece e usa?"* A classificação da ADR-0109 era **inferida**. A
> medição abaixo a **refuta**: composição same-kind é **língua documentada**. A ADR-0109 + edições de
> L0 foram **revertidas** (rumo B abandonado). Este recon mede e desenha a **reconciliação**.
>
> **Tipo**: recon read-only (probes compiladas e revertidas; suíte **2733/0**; árvore limpa; lint
> 0/0; `RUST_MIN_STACK=33554432`). **Não** decide sozinho a fatia de implementação — propõe.

---

## 1 — A medição de intenção (a fonte vence) — composição same-kind é LÍNGUA

**Doc de referência do Typst** (`lab/typst-original/docs/reference/language/styling.md`, secção
*Show rules*): documenta e **promove** múltiplas regras same-kind como feature deliberada —
```
#set heading(numbering: "(I)")
#show heading: set align(center)
#show heading: set text(font: "Inria Serif")
#show heading: it => block[ \~ #emph(it.body) #counter(heading).display() \~ ]
```
> *"we instead added them as separate show-set rules. **This is good practice because now these
> rules can still be overridden by later show-set rules in the document, keeping styling
> composable.** In contrast, set rules within a transformational show rule would not be overridable
> anymore."*

**Quatro** `#show heading` no mesmo elemento (3 show-set + 1 transform), chamados "composable" e
"good practice". → **Composição same-kind é intenção da língua, não mecânica de realização.** A
classificação "mecânica" da ADR-0109 é o **erro inverso-P347d** (tratar como mecânica algo que é
língua); refutada pela fonte.

**Fonte do mecanismo do vanilla** (`typst-realize/src/lib.rs`): num único `verdict` sobre as
recipes de um elemento — **todas** as show-set matching dobram na chain (`map.apply(transform);
continue`, `:458-464`, sem consumir o passe) e a **primeira** func vira `step`; o step aplica sob
`chained = styles.chain(&map)` (`:341,357`). → **as show-set ficam ativas quando a func realiza.**
Múltiplas func same-kind: guard por-`RecipeIndex` por-instância (`:472-474`; `content/mod.rs:148-156`),
cada uma uma vez, innermost-first. (O guard é a **implementação**; a composição é a **intenção**.)

---

## 2 — O que o crystalline faz hoje (probes P355, revertidas; árvore limpa)

| caso | crystalline | vanilla / doc | veredito |
|---|---|---|---|
| **múltiplos show-set** same-kind | **compõe ✓** — `#show heading: set text(bold)` ⨁ `set text(weight:700)` → **um** `Content::Styled` com `bold + weight` (fold via `collapse`, P352, `rules.rs:272`) | compõe (doc promove) | **PARIDADE** |
| **show-set + func** same-kind (o exemplo canônico do doc) | **show-set PERDIDO ✗** — `set text(bold)` ⨁ `it=>[X:]+it.body` → `"X:T"` com "X:" **não** bold (só o bold intrínseco do heading). O func roda 1º (`apply_all` loop α), o output vira Sequence, e o loop de show-set (`rules.rs:272`) checa o output **pós-func** → não casa heading → o show-set não entra | show-set dobra na chain, func aplica **sob** ela (`lib.rs:357`) → "X:T" todo sob o estilo | **DIVERGE** |
| **múltiplos func** same-kind | **uma efetiva ✗** (P354/PN-heading) | acumula (cada func uma vez, innermost-first) | **DIVERGE** |

---

## 3 — Classificação corrigida + as duas lacunas

**Composição same-kind = LÍNGUA** (medido §1). O crystalline **deve honrá-la**, não declarar
divergência. As lacunas, por custo e risco:

### Lacuna (i) — show-set + func (o exemplo canônico do doc) — **conserto de ordem, NÃO reabre o α**
**Causa medida** (`rules/eval/rules.rs`, `apply_show_rules`/`apply_all`): o loop α aplica a func
**primeiro** (sobre o nó cru), e o loop de show-set (que adicionei no P352, `:272`) embrulha **só se
o output pós-func ainda casa o seletor**. Como o output da func tipicamente muda de kind, o show-set
é perdido. O vanilla faz o **inverso**: dobra a show-set na chain e aplica a func **sob** ela.

**Desenho do conserto** (contido; **não** toca o loop α de recursão — show-set é
`Transformation::Style`, fora do guard/morph-fixpoint):
1. Em `apply_all`, **antes** do loop α de func: dobrar **todas** as show-set rules que casam o **nó
   original** (não o pós-func) num `Styles S` (reusar `collapse`, como hoje, mas sobre o **nó de
   entrada**).
2. Aplicar o loop α de func (inalterado).
3. Se `S` é não-vazio: embrulhar o **output da func** em `Content::Styled(output, S)` — espelhando
   `chained = styles.chain(&map)` do vanilla (a func realiza sob o estilo).
4. O caso show-set-sem-func (P352, `show_set_text_embrulha_heading…`) continua idêntico (fold sobre
   o nó, sem func, embrulha o nó). Os testes P352 ficam verdes; os testes func-only não têm show-set.

**Risco**: baixo. **0 testes** no combo show-set+func hoje; o conserto move a polaridade observável
em direção ao vanilla (a show-set alcança o output da func). **Não** reabre o caso 2/α (show-set não
recursa). *Sítios:* só `apply_show_rules`/`apply_all` em `rules.rs`. *L0:* `show.md` (a ordem
show-set-dobra-antes-da-func) + cross-ref.

### Lacuna (ii) — múltiplos func same-element — **A2/A3 (fatia seguinte, decisão pendente)**
Acumular N func same-kind exige o guard por-recipe: **A2** (por-`(RuleId, morph_canon)`) dá
composição preservando o α **mas diverge na ordem** das não-comutativas; **A3** (por-instância do
vanilla) dá paridade exata **mas reabre o α/caso 2 fechado**. Mais a sub-melhoria barata (inverter a
iteração de `node_rules` → innermost-first, "última-declarada vence"), que é polaridade, não
acumulação. **Fatia seguinte, com a decisão A2/A3 do dono** (o recon P354 §4 já a dimensionou).

---

## 4 — Recomendação (marcada) — a fatia de implementação é do dono

**Recomendo materializar a lacuna (i) (show-set + func) como o próximo lote** — é o **exemplo
canônico da doc**, **baixo risco**, **não reabre o α**, e honra a composição documentada. A lacuna
(ii) (múltiplos func, A2/A3) fica para a fatia seguinte, **medindo a demanda** (é o padrão menos
comum: dois `it => …` no mesmo elemento) e escolhendo A2/A3 conscientemente.

**O que NÃO fazer** (lições deste arco): não declarar a composição como divergência mecânica (é
língua — medido); não apresentar a inversão de ordem como "composição funciona" (S5b); não reabrir o
α para a lacuna (i) (desnecessário — show-set é estilo, não recursão).

**Estado**: ADR-0109 + edições de L0 **revertidas**; nenhum `.rs` de produto tocado; suíte 2733/0;
lint 0/0; árvore limpa fora de docs/`tools`. A fatia (i) aguarda o teu OK.
