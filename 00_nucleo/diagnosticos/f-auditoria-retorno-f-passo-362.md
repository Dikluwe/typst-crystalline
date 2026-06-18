# Auditoria de retorno ao F (P362) — feito / adiado / não-construído, pelos princípios

> **Tipo**: auditoria read-only (leitura/grep + lente; zero código de produto, zero L0; suíte não
> re-rodada; árvore limpa; `RUST_MIN_STACK=33554432`). Julga o F pelos **princípios** (atomização,
> fonte única de verdade, dois públicos); a **lente é instrumento, não o critério**. **NÃO conclui
> "F fechou/não-fechou" a priori** — mede item a item. **NÃO conta o Marco G como F.** **Termina no
> relatório e PARA** — o que fazer é decisão do dono no próximo turno.
>
> **Veredito (medido):** o F está **substantivamente completo no eixo realização/extensão** (E1, F-D,
> F-4, casos 1–4, dois públicos no modo compor/#show), com **atomização INCOMPLETA** (4 caminhos
> duplos vivos, F-5 adiado) e **F-6 não-feito** — ambos limpeza **sem demanda medida** (P353/P360).
> Há **uma lacuna de extensibilidade**: `#set <elem-de-usuário>(prop:)` não está ligado ao mapa
> aberto. Nenhuma é correção observável.

**HEAD**: `8916829c2` (pós-P359/360/361-docs). **Lente** `tekt-cargo-dsm` `98d8f9e` (66/0). Suíte 2737/0.

---

## A — A fila de lotes (6) — estado medido

| Lote | Estado | Evidência (`file:line`) |
|---|---|---|
| **F-1** fronteira E1 (`Content::Dynamic` + trait `DynElement` + registro) | **FEITO** | `entities/elements/dynamic.rs` (trait+blanket); `Content::Dynamic(Arc<dyn DynElement>)` `content.rs:1014`; `element_registry.rs` (nome→ctor) |
| **F-2** canal único das `Set*` (F-D) | **FEITO** | `StyleChain.push_custom`/`StyleDelta.custom` (`style_chain.rs:77,175`); 3 numbering (`heading/equation/figure.numbering`) via o canal (`eval/rules.rs:359+`) |
| **F-3 / F-realização** `#show` casos 1–4 | **FEITO** (com α como divergência consciente) | caso 4 escopo P340 (`f3s3`); caso 2 recursão α P348; caso 3 show-set P352 (`Transformation::Style`); caso 1 ordem P358 (`node_rules.iter().rev()`) |
| **F-4** `Styled` (colapso da dualidade de backing) | **FEITO** | `Styles` fachada sobre `StyleDelta` (`style.rs:158`); `push_styles` sem reconversão |
| **F-5** de-bake (4 pontos assados) | **ADIADO** (P353/P360 — limpeza sem demanda) | os 4 pontos vivem como caminho duplo (ver E) |
| **F-6** 3 folhas (DEBT-58) | **NÃO-FEITO** (P360 — sem divergência observável medida) | `Text`/`MathText`/`MathIdent` (`content.rs:122/167/170`) |

## B — O lado estilo (F-B) — o que nunca foi verificado

- **10 campos nativos fechados**: **FEITOS** (`StyleDelta` bold/italic/size/fill/heading_level/
  weight/tracking/leading/lang/font, `style_chain.rs:36-77`).
- **Mapa aberto (`PropKey → Value`)**: **EXISTE mas só exercido para numbering.** O canal é
  `StyleDelta.custom: Vec<(EcoString, Value)>` (`style_chain.rs:77`), usado **só** pelas 3 chaves de
  numbering. **`#set <elem-de-usuário>(prop:)` NÃO está ligado** — `eval_set_rule` despacha por nome
  fixo (heading/equation/figure/page/par/text) e o resto cai em `unsupported_target_warn`
  (`eval/rules.rs:56`). As props de elemento de usuário são setadas **no construtor** (args
  posicionais) e lidas via `get_field` (S7) — **não** por `#set` na chain. **Lacuna**: o mapa aberto
  para SET de props de usuário **não foi construído**.
- **`enum Value`** espelhando a linguagem: **FEITO** (~21 variantes, `value.rs`).
- **Escape `Value::Custom`**: **NÃO ENTROU** (grep em `value.rs` = vazio) — coerente com a decisão da
  Trava (L0 §3b.4: só com evidência; o spike-2 coube tudo no `Value` fechado). Confirmado: sem necessidade demonstrada.

## C — Os dois públicos

- **Caminho Rust** (implementa `trait Element` no `*Elem` + registra nome→ctor): **FEITO** — provado
  pela fixture `callout` (`elements/test_callout.rs`, fora dos 65), com `#callout(...)` e `#show
  callout:` (f3s1/f3s2/f3s3).
- **Caminho typst** (puro `.typ`): **PARCIAL por desenho.** **Compor/`#show`** elementos registrados:
  **FEITO/testado** (f3s2 `#show callout: …`, f3s3 escopo). **Definir** um elemento novo em puro
  `.typ`: **não é possível** — o layout vive em `rules/` (topologia), logo definir exige Rust. Isto é
  **fronteira declarada do L0** (§Dois públicos: "layout primitivo novo = Rust"), **não** uma lacuna.

## D — As travas e o contrato

- **Trava ADR-0105 cl.3**: **PRESENTE mas PARCIAL.** `element_registry.rs:126
  varre_registro_todo_nome_constroi_e_despacha` varre o registro × os **6 métodos do hub** (plain_text,
  is_empty, map_text, map_content, eq/clone) + round-trip `get_field` (S7). **NÃO** varre × os
  **backends de layout/show/introspect** (o "tabela × backends" pleno da ADR-0105 cl.3) — porque o
  arm `Content::Dynamic` de layout é **trivial** (`layout/mod.rs:538`, só `body`); a tabela
  kind→handler de layout é item do **Marco G**, não construída. Suficiente para o caminho dinâmico
  atual (1 elemento de usuário, hub-methods); o sweep pleno é Marco G.
- **Contrato 3c (S1–S7 do spike-2)**: **S1** (identidade `dyn_kind`→`DynKind`) ✓; **S5** (show-set
  `Transformation::Style`) ✓; **S7** (`get_field`/`dyn_get_field`) ✓; **S2/S4** (guard por-instância
  + multi-passe até fixpoint) **NÃO portados** — substituídos pelo **α morfológico** (divergência
  consciente, ADR-0107, P348); **S3** (innermost-first, 1 func/passe) **parcial** — ordem
  innermost-first (P358) mas eager, não multi-passe; **S6** (nó dinâmico membro pleno) **parcial** —
  o callout funciona na árvore, mas o **walk de filhos de elemento dinâmico** ficou registado como
  item (o spike tratou filhos como folha). Rede de caracterização (+11, P331): em vigor.

## E — Os princípios, medidos

- **Atomização** (`edges(elemento→elemento)` = **0**, lente): ✓ **COMPLETA**.
- **Fonte única de verdade**: **INCOMPLETA** — 4 caminhos duplos vivos (mesmo dado em campo assado
  **e** na chain):
  1. `heading.numbering` (chain custom ↔ `HeadingElem.numbering_active` assado);
  2. `equation.numbering` (chain ↔ `EquationElem.numbering_active`);
  3. `figure.numbering` (chain ↔ `FigureElem.numbering`);
  4. `Content::Text` `TextStyle` (chain `self.style` merge ↔ campo `TextStyle` assado).
  Cada um é **atomização incompleta** — o que o **F-5 (de-bake)** fecharia. Medido pelo princípio,
  não pela demanda (que P353/P360 mediram **nula**: caminho duplo paridade-testada, sem divergência).
- **Acoplamento** (`edges(content→elements)` = **66**, lente): **fato da lente, NÃO gate do F.** O
  alvo `0` é o **Marco G** (P346 Saída 2: marco fora da fila F; `content→elements→0` foi expectativa
  **órfã da lente**, reconciliada como marco próprio, não compromisso do F). **Registrado, não conta
  contra o F.**

---

## O que falta para o F estar completo SEGUNDO OS PRINCÍPIOS

1. **Fonte única** — os 4 caminhos duplos (F-5 de-bake). Atomização incompleta enquanto vivos.
   *Estado*: adiado (limpeza, demanda nula medida).
2. **F-6** (3 folhas) — primitivos provisórios; a chain já alcança o render, mas as folhas não
   carregam os campos completos que o vanilla lhes dá. *Estado*: não-feito (sem divergência observável).
3. **Mapa aberto para `#set <elem-de-usuário>(prop:)`** — a extensibilidade de SET de props de
   usuário não está ligada (só numbering usa o canal; user props via construtor + get_field).
   *Estado*: não-construído (não medido como demanda; o spike usou tone via construtor + show-set).

**Nenhum é correção observável.** 1 e 2 são limpeza (fonte-única); 3 é uma extensibilidade prometida
no desenho (F-B) mas não exercida.

## Separação explícita: F vs não-F

- **É F**: a fila (F-1…F-6), o F-B (chain 10+aberto+Value), os dois públicos (compor/#show + Rust-define),
  a trava cl.3, o contrato 3c.
- **NÃO é F**: o **Marco G** (`content→elements→0`) — órfão da lente, marco próprio (P346), fora da
  fila F. `content→elements=66` é fato da lente, não gate do F.

## Recomendação marcada (a decisão é do dono — no próximo turno)

O F está **substantivamente entregue** no que prometeu como realização+extensão (E1, F-D, F-4, casos
1–4, compor/#show, Rust-define). Para "completo pelos princípios" faltam, por ordem de aderência ao
princípio:
- **(i) Fonte única** — F-5 (de-bake dos 4 duplos). É o que mais fere o princípio "fonte única".
  Mas demanda observável **nula** (P353/P360). Recomendo **registrar como atomização-incompleta
  conhecida** e só fechar quando uma demanda (ex.: de-bake necessário para outro lote) surgir.
- **(ii) Mapa aberto p/ `#set` de user-props** — a única lacuna de **extensibilidade** (vs limpeza).
  Se a extensibilidade plena dos dois públicos é critério de "F completo", **este é o item a fazer**
  (não o F-5/F-6). Pequeno-médio, contido a `eval_set_rule` + o canal já existe.
- **(iii) F-6** — limpeza sem demanda; adiar.

**A decisão é do dono.** Esta auditoria **termina aqui** — não emenda nem inicia o passo seguinte.

**Estado**: nenhum código/L0 tocado; suíte 2737; lente 66/0; árvore limpa.
