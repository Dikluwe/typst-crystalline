# Passo 375 — Marco G: os três eixos em valores (recon read-only)

> **Tipo:** medição/recon. **Zero código de produto, zero L0.** Árvore limpa; lint inalterado.
> Caveat de stack: `RUST_MIN_STACK=33554432`. Pré-condição confirmada: P373 commitado
> (`cab587879`), HEAD pós-P374 (`98540fdf7`), branch `Tekt`.
> Lente `tekt-cargo-dsm` (ref `98d8f9e`): **não re-rodada** — o eixo 1 (acoplamento
> `content→elements`) usa o valor já medido pelo P374 (= 68), corroborado por contagem direta
> de imports (69 `use …elements::` em `content.rs` [medido]).

---

## Descoberta-chave que reposiciona os três eixos

A infraestrutura do Marco G **já existe e está em uso**:

- A variante `Content::Dynamic(Arc<dyn DynElement>)` já está no enum (`content.rs:119-1027`) [medido].
- O **trait `DynElement` (12 métodos)** + um **blanket impl** `impl<T: Element + …> DynElement
  for T` já existem (`entities/elements/dynamic.rs:45-124`) [medido]. **Todo** elemento que
  implementa `Element` é `DynElement` de graça.
- As **6 matches do hub já têm o arm `Dynamic`** que delega ao trait
  (`content.rs:1612, 1755, 1889, 1923, 2097, 2309`) [medido].
- Elementos custom (CalloutElem/BadgeElem) já passam **inteiros** por esse caminho: **não
  aparecem em nenhum arm** do hub nem do layout (`test_callout.rs`; registo único em
  `element_registry.rs:104`) [medido].

Consequência: o Marco G **não cria** trait nem tabela de dispatch para as 6 matches do hub —
elas colapsam sobre um arm que **já está lá**. O custo de criação/relocação concentra-se **na
camada de layout** (a match que lê campos concretos), não no hub.

---

## Eixo 1 — atomização líquida do núcleo

### O que sai do hub (`content.rs`) — REMOVIDO
| Item | Valor | Fonte | Tag |
|---|---|---|---|
| Variantes `Arc<*Elem>` (modelo D) no enum | **67** (de 77 totais; 9 primitivas ficam) | `content.rs:119-1027` | medido |
| Arms de dispatch monomórfico (linhas físicas) | **309** | ver tabela ↓ | medido |
| Matches distintas que colapsam | **6** | `is_empty/plain_text/eq/get_field/map_content/map_text` | medido |

Arms por match: `is_empty` 34 · `plain_text` 68 · `eq` 65 · `get_field` 4 · `map_content` 69 ·
`map_text` 68 = **309** linhas (`content.rs:1535-2317`) [medido]. Cada match colapsa em **1** arm
`Content::Dynamic(e) => e.dyn_*()` — **que já existe**. **Arms novos no hub = 0.**

### O que o Marco G cria/RE-CENTRALIZA
| Item | Valor | Fonte | Tag |
|---|---|---|---|
| Trait `DynElement` + blanket | **0 a criar** (já existem) | `dynamic.rs:45-124` | medido |
| Arms `Dynamic` nas 6 matches do hub | **0 a criar** (já existem) | `content.rs:1612…2309` | medido |
| **Match de layout** `layout_content` | **59 arms bespoke, 1857 linhas, SEM wildcard (exaustiva)** → **relocam** para tabela kind→handler ou método `Element::layout` | `engine/layout/mod.rs:527-2383` | medido |
| Walk do introspect | **43 arms nativos** → relocam | `rules/introspect.rs:156-460` | medido |
| Acoplamento `content→elements` | **68** (P374) — **persiste, relocado** para a tabela/trait que re-importa os 67 elementos | P374; `content.rs` 69 imports | medido |

### Saldo líquido
- **Hub `content.rs`:** encolhe **−309 arms −67 variantes** (genuíno: a infra de Dynamic já
  estava posta). **Atomiza.** [medido]
- **Sistema:** **não encolhe.** As **1857 linhas / 59 arms** de layout + **43 arms** de
  introspect **relocam** para os elementos (ou uma tabela), que **re-importa** os 67 — o
  acoplamento `content→elements = 68` **muda de arquivo, não desaparece** (a lição P361:
  `content→elements→0` reloca, não elimina). [medido]
- **Arquivos do núcleo:** os 3 hubs (`content.rs`, `layout/mod.rs`, `introspect.rs`) perdem o
  toque por-variante; em troca a lógica de layout espalha-se pelos **70** ficheiros em
  `entities/elements/` (método `Element::layout`) **ou** concentra-se num módulo-tabela novo.
  Contagem de arquivos: **mantida-a-aumentada**, não diminuída. [inferido — depende do modelo
  α-tabela vs β-trait-method, que a 0026 e o P361 dizem ser decisão de modelo em aberto]

**Veredicto Eixo 1: ATOMIZA o hub (6 métodos genéricos, infra já presente); RELOCA o layout +
introspect bespoke (1857+ linhas movem, acoplamento persiste). NÃO é atomização pura — é
atomização-do-hub + relocação-do-resto.**

---

## Eixo 2 — leitura e algoritmo

### Leitura — o que melhora
- **−303 linhas** de boilerplate repetitivo (`Self::X(e) => e.foo()`): 309 arms → 6 arms
  `Dynamic` (já existentes). 1 molde em vez de 67. [medido]

### Leitura — o que piora (a exaustividade perdida)
- **3 das 6 matches do hub são totais hoje** (sem wildcard → o compilador exige todo kind):
  `plain_text`, `map_content`, `map_text`. As outras 3 (`is_empty`, `eq`, `get_field`) **já têm
  `_ =>`** — não perdem nada. [medido]
- A **match de layout (59 arms) e o walk de introspect são SEM wildcard** → hoje o compilador
  pega o elemento esquecido **exatamente onde o layout é bespoke e obrigatório**. Pós-Marco G,
  kind sem handler = **fallback de runtime** (body/plain_text genérico), silenciosamente errado.
  [medido]
- **Superfície que repõe** (ADR-0105 cl.3): 1 teste-varre-tabela **ou** regra de lint por eixo
  que perde exaustividade (o de layout é o crítico). Rede **de CI/runtime**, não de compilação —
  mais fraca que o `match` total. [inferido — superfície pequena mas não-nula]

**Saldo de leitura: TRADE.** −303 linhas de repetição (ganho) **vs** −exaustividade estática em
3 matches totais do hub + layout + introspect, reposta por test/lint mais fraco (perda). Menos
repetição, menos garantia estática.

### Algoritmo
- Hoje: `match` estático sobre o tag do enum → **jump table O(1)**, monomorfizado inline
  (`content.rs:1535`; `layout/mod.rs:533`). [medido]
- Pós-Marco G: despacho **dinâmico** via `Arc<dyn DynElement>` (vtable, modelo α — a 0026
  **rejeita** o vtable, P361) ou lookup PropMap (β). **Direção: igual-ou-pior** (indireção de
  vtable vs jump table inline); **sem ganho algorítmico**. [direção medida na fonte; magnitude
  perf **não-medida**, inferida]

---

## Eixo 3 — custo para a IA (adicionar/manter um elemento)

### Custo HOJE
| Caminho | Pontos | Arquivos | Fonte | Tag |
|---|---|---|---|---|
| Elemento **nativo** | **~17** | **13** | `Content::Heading` = 17 refs / 13 files | medido |
| Elemento **custom** (via Dynamic, já suportado) | **~2** | **1 (+1 linha de registo)** | `CalloutElem`: 0 arms no hub/layout; impl `Element` (5-7 métodos) + `element_registry.rs:104` | medido |

O nativo toca: variante + construtor + **6 arms do hub** + arm bespoke de layout + arms de
introspect + `eval/bindings` + `element_payload` + `resolved_label_store` + `style_chain`
(`content.rs`, `layout/mod.rs`, `introspect.rs`, …) [medido]. O custom **não toca nenhum arm**.

### Custo DEPOIS do Marco G
- impl `Element` + registo (como o custom hoje: ~2 pontos) **+** a **entrada na tabela/método de
  layout SE precisar de layout bespoke** (Grid/Math*/Block precisam; os simples herdam o
  fallback genérico). **~2-3 pontos.** [inferido a partir do caminho custom + a tabela que a
  relocação cria]
- **Saldo: ~17 → ~2-3 pontos.** Redução grande (o molde-único; o critério "economia para IA"
  do P332). [baseline 17 medido; alvo ~2-3 inferido]

### A perda de rede para a IA (o contra, igualmente real)
- Hoje o **compilador** pega o arm esquecido (exaustividade em plain_text/map_content/map_text +
  layout + introspect) [medido]. Pós-Marco G, kind sem handler de layout **compila** e falha em
  runtime (fallback genérico) até o test/lint pegar — e é **na camada de layout, a única que não
  generaliza**, que a rede some.
- **Pesagem:** molde-único (pró — 1 impl em vez de 17 toques espalhados) **vs** exaustividade
  estática perdida na relocação de layout/introspect (contra — sem rede do compilador onde o
  layout bespoke é obrigatório).

---

## Leitura honesta — ganho líquido / trade / perda, por eixo

| Eixo | Veredicto | Porquê (em valores) |
|---|---|---|
| **1 — atomização** | **Ganho no hub / trade no sistema** | `content.rs` −309 arms −67 variantes (infra Dynamic **já existe**) [medido]; mas 1857 linhas de layout + 43 arms de introspect **relocam** e o acoplamento `content→elements = 68` **persiste** (P361) [medido]. Atomiza o hub, reloca o resto. **Não é atomização pura.** |
| **2 — leitura/algoritmo** | **Trade (leitura) / perda leve (algoritmo)** | −303 linhas de repetição [medido] vs −exaustividade em 3 matches totais + layout + introspect, reposta por test/lint mais fraco [medido/inferido]. Algoritmo: vtable ≥ jump table, **sem ganho** [direção medida]. |
| **3 — custo-IA** | **Ganho grande p/ adicionar, com perda de rede estática** | 17 → ~2-3 pontos por elemento [medido/inferido]; mas perde-se a garantia do compilador na camada de layout, a única não-generalizável [medido]. Trade favorável-mas-não-grátis. |

**Síntese sem inflar (erro P346) nem descartar por reflexo:** o Marco G é um **ganho real e já
quase-pronto no hub** (a infra de Dynamic existe; 309 arms + 67 variantes saem por ~0 custo de
criação) e um **ganho grande de custo-IA** (17→~2-3 pontos). O preço, medido e não-trivial: a
lógica de **layout (1857 linhas, 59 arms exaustivos) reloca** para os elementos/tabela
re-importando os 67 (o acoplamento não some), o **despacho fica dinâmico** (vtable que a 0026
rejeita; sem ganho algorítmico), e a **exaustividade estática some justamente na camada de
layout** que não generaliza, trocada por um test/lint mais fraco.

---

## Verificação (gates)
- **read-only:** nenhum código de produto, nenhum L0; só lente-via-P374 + grep + leitura; suíte
  não re-rodada. **Árvore limpa** (untracked = backlog pré-existente; nada modificado). Lint
  inalterado. ✔
- **saída:** este ficheiro — 3 eixos em valores (saldo líquido com `file:line`), leitura honesta
  (ganho/trade/perda por eixo), marcado [medido]/[inferido]. ✔
- **lente:** ref `98d8f9e` registada; eixo 1 (acoplamento) via valor P374 (=68) + contagem de
  imports — não re-rodada. ✔

**Termina aqui — não desenha a execução nem decide.** O desenho das "formas elegantes e
precisas" (e a decisão de modelo α-vtable vs β-PropMap, que a 0026 e o P361 deixam em aberto) é o
passo seguinte, **se** o dono escolher, com estes valores na mão.
