# F-recon (P337) — dimensão dos 4 lotes restantes + a ordem F-5 ↔ F-realização

> **Tipo**: recon dimensionador (sem código de produção além das caronas C1/C2,
> commit `1be7083df`). Evidência: **só código** (`01_core/src/`, `lab/typst-
> original/`) com `file:line`; contagens **exatas** (ou a nota de por que não dá
> para contar ainda). Quatro agentes paralelos varreram um lote cada (precedente
> P335); **claims derivados de `00_nucleo/materialization/` foram descartados**
> (pasta restrita — alguns agentes tocaram-na; só o nível-código foi retido) e os
> números-chave foram **re-verificados à mão** (greps + leitura direta).

---

## Fase A — recon por lote (5 perguntas: onde / quantos / vanilla / DEBTs / riscos)

### A1 — F-4 `Styled`

**(i) Onde vive.** O Layouter tem **uma** chain: `self.chain: StyleChain` +
`self.style: TextStyle` (cache achatada) — `layout/mod.rs:97-98`, init em
`:374` (`StyleChain::default_chain()`). `Content::Styled` faz push/pop nela:
`layout/mod.rs:1248-1256` (Passo 100, ADR-0039). A chain de **eval** é separada:
`engine.styles` (mutada por `#set`, escopada por `local_styles`).

**(ii) Pontos de toque (exato).** Construtores **diretos** de `Content::Styled`
em produção: **2** — `Content::strong` (`content.rs:1035`) e `Content::emph`
(`content.rs:1043`); `*neg*`/`_ital_` e a stdlib `strong()`/`emph()`
(`stdlib/structural.rs:24,40`) passam por eles. **Preservadores** estruturais
(rebuild em map): `content.rs:2023` (`map_content`), `:2193` (`map_text`),
`introspect.rs:348` (walk). **Consumidores** de produção: layout push/pop
(`layout/mod.rs:1248`), `plain_text` (`content.rs:1578` região), `PartialEq`
(`content.rs:1785`), deteção bold/italic do `#show` (`rules.rs:113,115`),
`locatable` (`locatable.rs:131`), walk introspect (`introspect.rs:1205`).

**(iii) Vanilla.** `StyleChain<'a>` com referências `'a` (linked-list emprestada,
sem `Arc`): `lab/.../typst-library/src/foundations/styles.rs:564-807`; chaining em
`:683`. Layout segura `styles: StyleChain<'a>` (mesmo papel). Resolução por trait
`Resolve` a tempo de realize/layout.

**(iv) DEBTs/decisões.** **B2 confirmado** (registrado no plano P333): `is_empty()`
**não tem arm `Content::Styled`** — cai em `_ => false` (`content.rs:1566`). Logo
um styled-vazio reporta não-vazio (bug). **Dualidade de backing** (não "2ª chain"):
`StyleDelta` (10 campos fechados + canal `custom` do F-2) é o backing dos accessors,
e `Content::Styled` carrega `Styles` (enum) — a coexistência é **intencional até o
pipeline migrar** (`style_chain.rs:12`).

**(v) Riscos / correção de recon.** A descrição do plano ("a **2ª StyleChain** do
Layouter colapsa") é **imprecisa**: o recon achou **uma** chain por fase
(eval `engine.styles` + layout `self.chain`) **mais** a dualidade `StyleDelta`-vs-
`Styled.Styles`. O alvo real do F-4 é **(a)** colapsar essa dualidade de backing
(`#set`/`#show`/Layouter lêem a `StyleChain` direto) e **(b)** fechar B2. Risco: se
F-4 não for feito antes do de-bake, o consumidor de-bakado lê uma chain cuja
unificação ainda não aconteceu.

### A2 — F-5 `de-bake`

**(i)/(ii) Onde vive + inventário exato — 4 pontos assados:**

| # | Propriedade | Baker (lê chain, assa) | Consumidor (lê campo assado) |
|---|-------------|------------------------|------------------------------|
| 1 | `HeadingElem.numbering_active: bool` | `eval/markup.rs` lê `styles.custom("heading.numbering")` | `layout/mod.rs:714`; `introspect.rs` (auto-TOC `compute_heading_auto_toc`) |
| 2 | `EquationElem.numbering_active: bool` | `eval/mod.rs` lê `styles.custom("equation.numbering")` (só bloco) | `layout/mod.rs:812` → `layout/equation.rs:28`; `introspect.rs` (gate de contador) |
| 3 | `FigureElem.numbering: Option<String>` | `eval/closures.rs:79-82` lê `styles.custom("figure.numbering")` | layout/introspect de figura |
| 4 | `Content::Text(_, TextStyle)` | `TextStyle::from(&StyleChain)` (`style_chain.rs`) assa na criação | `layout/mod.rs:599` (merge com `self.style`) |

**Crux do produtor (verificado):** `#set heading/equation/figure(numbering:)` faz
`*engine.styles = engine.styles.push_custom(...)` e devolve `Value::None` —
`rules.rs:242` (heading), `:265` (equation), `:297`-região (figure). **Não emite
`Content::Styled`.** Logo o valor vive **só** na chain de **eval**; chega ao
elemento **apenas por baking**; **nunca** entra na `self.chain` do layout (que só é
alimentada por `Content::Styled`, `layout/mod.rs:1248`). `SetPage` é a exceção:
**viaja como conteúdo** (`Content::SetPage{..}`, `rules.rs:297`; estado de região,
decisão do dono no F-2 S4).

**(iii) De-bake exige no consumidor:** a chain (com o `custom` do `#set`)
**disponível no ponto de layout/introspect**. Hoje ela **não está lá** (ver crux).
→ de-bake-para-chain exige **um caminho novo** de transporte do `custom` até o
consumidor (ver A-interseção, Fase B).

**(iv) Vanilla.** Resolve no realize/layout via `styles.get(Elem::field)` — **não
assa**: heading `lab/.../model/heading.rs`, equation `lab/.../math/equation.rs`,
texto resolvido a tempo de IR de `StyleChain` (`lab/.../math/ir/resolve.rs:249-295`).

**(v) Riscos.** `introspector.rs` retém **47** referências a `numbering_active` (a
API legada `is_numbering_active`/StateRegistry que o F-2 substituiu pelo campo
assado). **Questão aberta separada dos 4 pontos**: são consumidores **vivos** ou
**código morto** pós-F-2? O de-bake deve decidir isto antes de tocar (pode mascarar
um caminho duplo). Superfície de teste do campo assado: `layout/tests.rs` 22 refs +
`eval/tests.rs` 6 refs a `numbering_active` (a migrar com consciência — paridade,
não enfraquecer).

### A3 — F-6 `folhas` (Text / MathText / MathIdent)

**(i) Onde vive.** `Content::Text(EcoString, TextStyle)` `content.rs:122` (carrega
estilo assado, `TextStyle` em `layout_types.rs:123-139`: bold/italic/size/fill/
weight/tracking/leading/lang/font). `Content::MathText(EcoString)` `content.rs:170`
e `Content::MathIdent(EcoString)` `content.rs:167` — **sem** campo de estilo
(estilizados inline no math layout via param).

**(ii) Pontos de toque (agente, primário verificado):** Text **4** produtores / **7**
consumidores (arm primário `layout/mod.rs:599`); MathText **6** / **5** (arm
`math/layout/mod.rs:272`); MathIdent **2** / **5** (arm `math/layout/mod.rs:260`,
auto-itálico de var de 1 letra).

**(iii) Vanilla.** Não baka: resolve de `StyleChain` a tempo de IR
(`lab/.../math/ir/resolve.rs:249-319`); texto via `TextElem` + chain.

**(iv) DEBT-58.** `00_nucleo/DEBT.md:268-271`: *"Primitivos provisórios (3) —
`Text`, `MathText`, `MathIdent`: … Os campos que o vanilla lhes dá são estilo
(StyleChain) — território do F."* Plano P333:79 — *"recebem estilo via chain
(DEBT-58) | após F-5"*.

**(v) Independência.** F-6 **depende de F-4** (chain unificada para ler), **não** de
F-realização. É o **lote-tampão** candidato: pode rotear `Text.TextStyle` pela chain
**mantendo o campo assado** (redundante, content-preserving) — seguro enquanto a
decisão grande (F-5↔F-realização) descansa. Não pode vir **entre** F-5 e F-4 (sem
campo e sem chain unificada não há de onde resolver).

### A4 — `F-realização` (caso 4 escopo + caso 1 composição + caso 3 show-set)

**(i) Onde vive o eager + a raiz do caso 4.** `#show` é interceptado eager:
`intercept_content` (`rules.rs:193`) → `apply_show_rules` (`rules.rs`), aplicado na
**criação**. A regra é guardada por mutação: `rules.rs:588` `*engine.show_rules =
Arc::from(rules)` — **raiz do vazamento**. A **distinção fina** (verificada):
- **CodeBlock `{...}`** clona `local_show_rules` (`eval/mod.rs:399`) → `#show`
  **confina** (`show_rule_respeita_escopo_lexico` passa).
- **ContentBlock `[...]`** partilha `&mut *engine.show_rules` (`eval/mod.rs:460`) →
  `#show` **vaza** (= caso 4 / `f3s3`).
- `#set` clona `local_styles` em **ambos** (`eval/mod.rs:398` e `:454`) → já
  escopado (por isto o F-2 funcionou). **A assimetria `#show`(`[]` vaza) vs
  `#set`(escopado) é exatamente o caso 4.** Anti-recursão: `active_guards` (stack
  de `RuleId`) + teto 64 (`route_check_show_depth`).

**(ii) Toque nos nativos (afeta todos, não só dyn).** O eager é assumido em todo o
pipeline de criação: `intercept_content` é chamado em `markup.rs` (blocos),
`closures.rs:236` (corpo de closure), `eval/mod.rs` (sequências de código). O
estado de show-rule (`show_rules`/`active_guards`) é lido/mutado/threaded em ~15
sítios de eval (`rules.rs:198,206,586,588`; `markup.rs`; `closures.rs`;
`modules.rs`; `eval/mod.rs:399,407,460`).

**(iii) Vanilla multi-passe.** `lab/.../typst-realize/src/lib.rs`: `realize()`
(entry) → `visit` → `finish`; o fixpoint é **externo** (loop de introspeção
re-realiza até convergir). `Transformation { Content, Func, Style }`
(`foundations/styles.rs:531-554`); **`Style` = show-set** (caso 3), aplicado na
preparação sem consumir o passe. **Escopo por subárvore via `StyledElem`**
(`content/mod.rs:744-752`): a recipe viaja **dentro** do `StyledElem.child` e
**deixa de existir** ao sair do bloco — confinamento estrutural (o que falta ao
eager no `[]`).

**(iv) DEBTs.** Estende **DEBT 99.E** (escopo léxico) do `#set` para o `#show`.
Reescreve a anti-recursão `active_guards`/`RuleId` para o modelo de guard por
recipe-index do multi-passe. Âncora: `f3s3_caso4_escopo_eager_nao_confina_
divergencia_registrada` (`tests.rs:462`).

**(v) Impacto de teste (exato).** **20** funções de teste asseriam comportamento de
`#show` eager (`eval/tests.rs`): 15 `show_rule_*`/`eval_show_rule_*` + 4 `f3s2_*`
(dyn) + 1 `f3s3`. **1 vira por construção** (`f3s3` — quando o `[]` confinar);
`show_rule_respeita_escopo_lexico` (`{}`) **permanece válido**. As demais 18 pedem
**revisão consciente** (semântica guard-por-RuleId → guard-por-recipe; ordem de
composição innermost-first) — a reclassificação exata é trabalho **do** lote (não
se pré-julga sem o escrever). (`f3s1_*`, 2 testes de registry, **fora** do escopo.)

---

## Fase B — dimensão, dependência e ordem

### 1. Tabela de dimensão

| Lote | Pontos de toque (exato) | Testes afetados | DEBTs | Risco nomeado |
|------|-------------------------|-----------------|-------|---------------|
| **F-4 Styled** | 2 construtores prod. + 3 preservadores + ~7 consumidores; 1 chain/fase + dualidade de backing | testes de Styled/strong/emph (a medir no arranque) | **B2** (`is_empty` sem arm Styled, `content.rs:1566`) | descrição "2ª chain" imprecisa; é colapso de **dualidade de backing** |
| **F-5 de-bake** | **4** pontos assados (heading/equation/figure/text); produtor não emite Styled (`rules.rs:242/265/297`) | 28 refs de teste a `numbering_active` (`layout/tests.rs`+`eval/tests.rs`) | DEBT 99.E (consumo) | **47 refs** `is_numbering_active` em `introspector.rs`: vivo ou morto? |
| **F-6 folhas** | Text 4/7 · MathText 6/5 · MathIdent 2/5 | testes de layout de texto/math (a medir) | **DEBT-58** | depende de F-4; tampão se rotear-mantendo-assado |
| **F-realização** | ~15 sítios de show-state; `intercept_content` em todo o pipeline de criação | **20** testes de `#show` (1 vira; 18 revisão; 1 permanece) | DEBT 99.E (escopo→`#show`); reescreve `active_guards` | toca o eager dos **nativos todos**; lote maior/mais arriscado |

### 2. Grafo de dependência (com evidência)

```
            B2/dualidade            chain disponível no nó
   F-4  ───────────────►  (fundação: todos lêem a chain unificada)
    │
    ├──────────────►  F-6   (folhas pela chain; só precisa de F-4; TAMPÃO)
    │
    └──►  F-realização  ──────────────►  F-5  (de-bake)
              (StyledElem-scoped:            (consumidor lê a chain
               #set/#show viajam              que a realização garante
               confinados; chain no nó)       no nó — rewire único)
```

- **F-4 → (tudo)**: a chain unificada + B2 é o que os consumidores de-bakados e as
  folhas lêem. Evidência: `style_chain.rs:12` (dualidade intencional "até migrar"),
  `content.rs:1566` (B2).
- **F-realização → F-5 (a aresta cara)**: o de-bake-para-chain precisa do `custom`
  do `#set` **disponível no consumidor**; hoje **não está** (`#set` não emite
  Styled — `rules.rs:242/265/297`; layout só vê `Content::Styled` —
  `layout/mod.rs:1248`). O **transporte confinado** do `#set`/`#show` até o consumo
  é **exatamente** o `StyledElem`-scoped que a F-realização constrói
  (`lab/.../content/mod.rs:744-752`). **Interseção** = (a) o mecanismo
  `#set`→conteúdo (hoje: baking; alvo: chain confinada) **e** (b) os arms
  consumidores em `layout/mod.rs:714,812` + `introspect.rs` (hoje: campo assado;
  alvo: leitura da chain). A F-realização **constrói o transporte**; o F-5
  **religa os consumidores** a ele.
- **F-4 → F-6**: folhas lêem a chain unificada; **independente** de F-5/realização
  (pode ser tampão content-preserving).

### 3. Proposta de ordem (+ a alternativa rejeitada)

**Recomendada: `F-4 → F-realização → F-5 → F-6`.**
Justificativa: o de-bake (F-5) religa o **consumo** ao modelo de transporte
**final**. Se a F-realização vier **antes**, o `StyledElem`-scoped já existe e o F-5
é **um rewire único** dos arms consumidores. Satisfaz os dois critérios do P333:
**não des-assar duas vezes** e **não migrar consumo para um modelo que a
F-realização vai substituir**. F-4 primeiro (fundação barata que todos lêem); F-6
por último (ou como tampão a qualquer momento após F-4).

**Rejeitada: `F-4 → F-5 → F-realização → F-6` (de-bake antes da realização).**
Razão: com `#set` ainda sem emitir Styled, de-bakar **primeiro** obriga a construir
**agora** um caminho de transporte do `custom` até o consumo — ou um `Content::
Styled`-precursor (uma fatia da F-realização feita cedo) ou uma ponte ad-hoc. A
F-realização **reconstrói** esse transporte → o consumo de-bakado é **migrado duas
vezes**. É precisamente o erro caro que o critério do P333 nomeia.

**Tensão honesta para o dono:** a ordem recomendada **front-loada o risco** — a
F-realização é o lote **maior** (toca o eager dos nativos todos; 20 testes de
`#show` em revisão consciente). Quem prefira diferir risco pode antepor o **par
seguro de plumbing** (F-4 e F-6, que **não** dependem da realização) e só então
encarar o par caro `F-realização → F-5` — sem inverter a aresta cara.

### 4. Checkpoint do dono — DECIDIDO

**Ordem escolhida (P337): `F-4 → F-realização → F-5 → F-6`** (a recomendada).
Critério aceito: o de-bake religa o **consumo** ao modelo de transporte **final**
da F-realização — **não des-assa duas vezes**, **não migra consumo para um modelo
substituído**. O dono aceitou **front-loadar o risco** (a F-realização, o lote
maior, vem antes do de-bake). Alternativa rejeitada: `F-4→F-5→F-realização→F-6`
(de-bake antes da realização força transporte ad-hoc reconstruído depois).
Registrado na fila `f-plano-lotes-passo-333.md`.
