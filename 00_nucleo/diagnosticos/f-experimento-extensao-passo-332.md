# Experimento da fronteira de extensão (P332) — spikes E1/E2/E3 + medição

> **Zero decisão de desenho. Zero código de produto.** Spikes descartáveis em
> `lab/spikes/f-extensao/` (fora do gate de lint de produto; standalone crates
> com `[workspace]` próprio; nunca importam o produto nem a quarentena vanilla;
> podem **copiar** trechos para dentro). A decisão da fronteira é do dono, no
> checkpoint, com a tabela da Parte 3 na mão.
>
> Requisito que motiva (P332 Parte 0): extensibilidade **total** — elemento de
> utilizador = cidadão pleno (`#set`/`#show`/`query`/render) sem tocar o core;
> dois públicos (Rust + autor typst). Opção A do dossiê P331 **rejeitada**.
> Critérios do dono: atomização, separação de camadas, economia para IA.

---

## Parte 1 — Os candidatos (desenho)

Três candidatos de partida (ajustados ao inventário 1a/1b/1c). Cada um implementa
o **mesmo** elemento-brinquedo `callout { body, title, tone }` de ponta a ponta,
para medir o custo real em vez de o argumentar.

### E1 — Fronteira por trait (híbrido estático/dinâmico)

`Content::Dynamic(Arc<dyn Element>)` como **uma** variante de extensão; os 6
matches do hub despacham pelo trait `Element` **já existente** (precedente: os 65
módulos nativos). Os 65 nativos continuam **estáticos** (`Variant(Arc<Elem>)`,
despacho monomórfico). Propriedades: os 10 campos nativos **fechados** +
**mapa aberto** (chave de propriedade → valor) para props de utilizador.
`ElementKind`/payload: variante dinâmica por **nome/id registrado**.

- **Público Rust**: implementa `trait Element` + (opcional) `trait
  StyleableElement` para expor props settable; regista o construtor por nome.
  **0 ficheiros do core** (a variante `Dynamic` já existe).
- **Público typst**: `#callout(...)` resolve para o construtor registrado;
  `#set callout(tone:)` e `#show callout:` operam sobre o nome dinâmico.
- **Escala ~273 props**: as nativas ficam no enum `Style` fechado (zero custo
  marginal por já existirem); props de utilizador entram no mapa aberto (custo
  marginal = 1 entrada de mapa, sem recompilar o core).
- **Camadas**: o elemento de utilizador vive **fora** do core, importa só o
  `trait Element` + tipos públicos; o core não o conhece (despacho por `dyn`).

### E2 — Type-erased à vanilla (C honesta, mínima)

Chain erased: `enum Style { Property(Box<dyn>), Recipe(Box<dyn>) }`, registo de
elemento por **id**, resolução por **fallback** (walk-up). A estrutura do vanilla
(1b) em versão **mínima** suficiente para o `callout` — para medir o custo real
do `dyn`/type-erasure, não o rejeitar por impressão.

- **Público Rust**: define o elemento + as suas props como `Property`s
  type-erased `(ElementId, PropId) → Box<dyn Any>`; regista o id.
- **Público typst**: `#set`/`#show` empurram `Property`/`Recipe` na chain; o
  layouter resolve por fallback.
- **Escala ~273 props**: cada prop é um `(id, Box<dyn>)` na chain — custo
  marginal uniforme; sem enum a crescer. Mas paga `dyn`/downcast em cada leitura.
- **Camadas**: máxima flexibilidade (3os via `#[elem]`-equivalente), ao custo de
  vtable em folha quente (risco ADR-0029/0030).

### E3 — Registro aberto por kind (B destravada, sem `dyn` no Content)

`ElementKind` **extensível** (id dinâmico registrado) + **payload genérico**
(dados) + **PropMap aberto tipado-por-chave** (enum de valor fechado, chave
aberta). **Sem `dyn` no `Content`** — dados, não vtable. O comportamento de
utilizador entra por **dados + funções registradas** (tabela de vtable explícita
indexada por kind, fora do `Content`).

- **Público Rust**: regista um `ElementDescriptor { kind, props, layout_fn,
  show_default }` numa tabela; o `Content` carrega só `(kind_id, PropMap)`.
- **Público typst**: `#set callout(tone:)` escreve no `PropMap`; `#show` indexa a
  tabela de descritores por kind.
- **Escala ~273 props**: o `PropMap` é `HashMap<PropKey, Value>` (chave aberta,
  valor de enum fechado) — custo marginal = 1 entrada; sem `dyn`, sem recompilar.
- **Camadas**: o elemento vive fora do core; o core conhece só `(kind_id,
  PropMap)` + a tabela de descritores (injetada). Sem vtable em folha quente.

---

## Parte 2 — Spikes (paths + o que cada um demonstra/stuba)

Três crates standalone (`[workspace]` próprio vazio → fora do workspace do
produto, logo fora do gate `crystalline-lint`; **0 imports** do produto ou da
quarentena vanilla). Cada um implementa o **mesmo** `callout { body, title,
tone }` de ponta a ponta — definição **fora** do toy-core → registro → `#set
callout(tone:)` → `#show callout:` → `query(callout)` → render. Compila e corre
com `cd lab/spikes/f-extensao/eN && cargo run --release`.

| Spike | Path | `callout` (def utilizador) | Ficheiros do toy-core p/ +1 elemento | LOC total |
|-------|------|----------------------------|--------------------------------------|-----------|
| **E1** | `lab/spikes/f-extensao/e1/` | `src/callout.rs` 103 l (78 LOC) | **0** | 606 |
| **E2** | `lab/spikes/f-extensao/e2/` | `src/callout.rs` 103 l (66 LOC efetivas) | **0** | 500 |
| **E3** | `lab/spikes/f-extensao/e3/` | `src/callout.rs` 67 l | **0** | 457 |

**E1 — Fronteira por trait** (`Content::Dynamic(Arc<dyn Element>)`). Demonstra: o
elemento de utilizador implementa o **mesmo** `trait Element` dos 65 nativos
(precedente vivo, exemplo direto); os 65 nativos ficam **monomórficos** (sem
imposto de vtable — o `dyn` só toca a folha dinâmica); props de utilizador num
mapa aberto. **Stuba**: `#show` é `fn(&Content)->Content` fixo, regra única,
passe único; `Registry` passado explicitamente (pureza L1, sem global).

**E2 — Type-erased à vanilla** (`enum Style { Property(Box<dyn Any>), Recipe }`,
resolução por fallback `(ElementId,PropId)→Box<dyn Any>`). Demonstra: a forma
mínima da chain do vanilla (1b) que basta para o `callout`; custo do `dyn`
isolado à folha `callout` (nativos não pagam). **Stuba**: render→`String`;
`#show` sobre string renderizada; chain `Vec` plana; alocação de id sequencial.
**Risco não-local medido**: `#set` com tipo errado falha **silenciosamente**
(downcast → `None`, sem erro) — não há ligação compilada `PropId`↔tipo.

**E3 — Registro aberto por kind** (`(kind_id, PropMap)` no `Content`, **sem
`dyn`**; tabela `kind_id → ElementDescriptor` com `fn` pointers). Demonstra:
extensão por **dados + funções registradas**, sem vtable no `Content`; `PropKey`
**aberta** (o utilizador cunha `PropKey("tone")` sem tocar o core). **Stuba**:
sem eval/parse; mapa `(kind,key)→value` plano; `show_default` identidade; render
`String` não frames; placeholder para kind desconhecido. **Teto estrutural
medido**: o `enum Value` é **fechado** — prop nova de um tipo de valor já
existente é grátis, mas um tipo de valor **novo** (ex.: `Color`, `Length`) exige
editar o `Value` do core.

---

## Parte 3 — Medição comparativa

Critérios fixados na Parte 3 do passo (antes de medir). Os 3 spikes nas mesmas
colunas. **Limite de toda a coluna Performance**: os spikes **stubam** o
render/layout real — os números **NÃO** são comparáveis ao baseline do produto
(`0.6518 s ± 0.0057`, P330), que corre o pipeline verdadeiro. Servem só como
**comparação relativa entre spikes** (mesmo harness-brinquedo, mesmo doc). 12
runs, release, mesma máquina, medidos nesta sessão.

| # | Critério | **E1** (trait/`dyn`) | **E2** (type-erased) | **E3** (kind+PropMap) |
|---|----------|----------------------|----------------------|------------------------|
| 1 | **Atomização** — ficheiros/linhas do utilizador; ficheiros do core por elemento novo (alvo 0) | 1 ficheiro, 103 l (78 LOC); **core: 0** | 1 ficheiro, 103 l (66 LOC efet.); **core: 0** | 1 ficheiro, **67 l**; **core: 0** |
| 2 | **Separação de camadas** — grafo de import; ciclo?; estilo atravessa por contrato ou acoplamento | importa só `trait Element` + tipos públicos; **sem ciclo**; contrato = trait | importa o tipo `Property` + ids; **sem ciclo**; contrato = convenção `(ElementId,PropId)` | importa `ElementDescriptor`+`PropKey`; **sem ciclo**; contrato = descritor (dados+fn) |
| 3a | **Custo-IA — superfície a ler** | 1 superfície: `trait Element` (7 métodos) + tipos, `core.rs:142` | tipo `Property`/`Recipe` + tabela de ids + invariante de downcast | `ElementDescriptor` (campos+`fn` ptrs) + `PropKey`/`Value` |
| 3b | **Custo-IA — raciocínio não-local** (contar) | **baixo**: ordem de resolução de `tone`, `kind()` por string | **alto**: downcast **silencioso** em tipo errado; alloc de id sequencial; convenção `(ElementId,PropId)` | **médio**: comportamento mora em `fn` ptrs (não no trait); `PropKey` aberta mas `Value` fechado |
| 3c | **Custo-IA — precedente** (os 65 módulos servem?) | **sim, direto** — é o mesmo `trait Element` dos 65 | não — `Property`/chain é forma nova | parcial — `PropMap` é forma nova; payload tem eco em `to_payload` |
| 4 | **Performance** (relativa entre spikes; stub) | native `0.375 ms` · mixed `0.889 ms` (~2.4×) | native `0.202 ms` · N-callout `0.541 ms` (~2.7×) | native `0.190 ms` · mixed `1.183 ms` (~6.2×) |
| 4* | **Imposto sobre nativos** (o que importa p/ ADR-0029/0030) | nativos **monomórficos**, `dyn` só na folha | nativos **não pagam**; downcast só na folha callout | nativos **não pagam**; tabela só consultada no arm `Dynamic` |
| 5 | **Escala até M1 (~273)** — custo marginal/prop | nativas no enum fechado (0 marginal); props utilizador = 1 entrada de mapa aberto, 0 core | cada prop = 1 `PropId` + 1 branch de downcast; **sem enum a crescer**; mas 273 pares `PropId↔tipo` **não verificados** | prop de tipo já-em-`Value` = 1 entrada, 0 core; **tipo de valor novo ⇒ editar `enum Value` do core** (teto estrutural) |
| 6 | **Fidelidade comportamental** (C1–C8 + `#show`) | `#set`/`#show`/`query`/render demonstrados; `#show` = 1 regra/1 passe (stub) | `#set`/`#show`/`query` demonstrados; `#show` sobre string; fallback walk-up real | `#set`/`#show`/`query` demonstrados; `#show` indexa descritor; eval/parse stub |
| 7 | **Custo de migração do estado atual** (preditor 1c) | `Dynamic` é **nova variante**, os 65 nativos **ficam**; `Set*`/`Styled`/3 folhas ⇒ podem ficar nativos ou virar props no mapa (escolha incremental) | exige **de-bake** de `#set text` + chain → ~283 sites (alinha c/ opção C do dossiê); `Set*`→`Property`s | `Set*`→entradas no `PropMap` (~108 sites, eco da opção D/B); `Styled`/folhas decidem-se por lote; `Content` ganha arm `(kind,PropMap)` |

**Notas de leitura dos números (4):**

- A coluna Performance mede **harnesses-brinquedo diferentes**, não o produto.
  A diferença native↔native (E1 `0.375` vs E2/E3 `~0.19`) vem sobretudo do
  render-stub de E1 fazer mais trabalho de string, **não** de imposto de
  infraestrutura — não tratar como overhead de desenho.
- O facto **robusto e comparável** é o **§4***: nos três, o caminho dinâmico
  **não onera os nativos** — `dyn`/downcast/tabela só tocam a folha de
  utilizador. Isto satisfaz ADR-0029/0030 para os 65 já migrados em qualquer dos
  três desenhos.
- O multiplicador mixed (E1 ~2.4× · E2 ~2.7× · E3 ~6.2×) ordena o **custo do
  caminho de extensão**: E3 paga construção de `HashMap` PropMap + lookup +
  resolve por callout; E2 paga downcast+vtable; E1 paga vtable monomorfizável.
  Ordem de grandeza, não veredito (stub).

---

## Parte 4 — Leitura do executor + perguntas

**Leitura do executor (NÃO vinculativa — a decisão é do dono).**

Os três entregam os dois públicos e **0 ficheiros do core por elemento novo** —
o requisito de atomização do dono é satisfeito por qualquer um. A escolha
separa-se pelos critérios secundários:

- **E1 (trait/`dyn`)** é o que melhor pontua em **custo-IA** e **precedente**:
  reutiliza o `trait Element` que os 65 módulos já instanciam — uma IA (ou
  humano) escreve um `callout` por imitação direta, com o menor raciocínio
  não-local. Custo: introduz `dyn` na folha (aceitável por §4*; os nativos
  ficam monomórficos). Migração do estado atual é a **mais barata** (os 65
  nativos ficam; `Dynamic` é aditivo).
- **E3 (kind+PropMap)** é o mais **atómico** (67 l) e evita `dyn` no `Content`,
  mas tem dois custos: o comportamento migra do trait para `fn` ptrs (menos
  precedente) e o **`enum Value` fechado** vira teto — um tipo de valor novo
  toca o core, contra o alvo "0 ficheiros". Perf de extensão a mais cara (stub).
- **E2 (type-erased)** é o mais fiel ao vanilla **estruturalmente**, mas colide
  com o critério do dono (fidelidade é **comportamental**, P329) e traz o
  **downcast silencioso** — `#set` de tipo errado falha sem erro, o pior
  resultado de custo-IA da tabela. Migração mais cara (~283 sites, de-bake).

Se o eixo dominante for **economia para a IA + reaproveitar o precedente vivo**,
E1 lidera. Se for **eliminar `dyn` do `Content` a todo custo**, E3, aceitando o
teto do `Value`. E2 só se a fidelidade **estrutural** ao vanilla virar
requisito explícito — hoje não é.

**§Perguntas ao dono** (numeradas; a fronteira decide-se aqui):

1. **Eixo de desempate**: entre os três que zeram o custo-de-core, qual critério
   manda — custo-IA/precedente (favorece **E1**), ausência de `dyn` no `Content`
   (favorece **E3**), ou fidelidade estrutural ao vanilla (favorece **E2**)?
2. **`dyn` na folha**: E1 aceita `Arc<dyn Element>` numa **única** variante
   `Content::Dynamic`, com os 65 nativos monomórficos. Isso é aceitável face a
   ADR-0029/0030, dado que §4* mostra que os nativos não pagam imposto?
3. **Teto do `Value` (E3)**: aceita-se que um **tipo de valor novo** (não uma
   prop nova) toque o `enum Value` do core — violando "0 ficheiros" só nesse
   caso — em troca de não ter `dyn` no `Content`?
4. **Downcast silencioso (E2)**: o modo de falha "`#set` de tipo errado não dá
   erro" é eliminatório, ou mitigável (registo `PropId`→tipo verificado) ao
   ponto de manter E2 vivo?
5. **Destino das `Set*`/`Styled`/3 folhas**: a fronteira escolhida deve
   **absorvê-las** já (E2/E3 convidam a isso) ou deixá-las nativas e migrá-las
   incrementalmente depois (E1 permite)? Isto fixa o tamanho do primeiro lote do
   F (≈108 vs ≈283 sites, preditor 1c).
6. **`#show` na linguagem**: os spikes stubam `#show` (1 regra/1 passe ou sobre
   string). Antes de fixar a fronteira, quer um spike-2 que exercite `#show`
   multi-regra/multi-passe — ou `#show` fica como requisito **argumentado** e
   detalha-se no desenho de F-D/F-B?
7. **Forma final do F**: a fronteira escolhida vira o desenho de **F-D** (lado
   elemento) e **F-B** (lado StyleChain/DEBT 99.E)? Confirma que o próximo passo
   é **redigir o L0 do F** sob a fronteira escolhida (e só então código)?

**O passo termina aqui, no checkpoint.** A escolha da fronteira — e com ela a
forma final de F-D/F-B — é do dono, na conversa, com esta tabela na mão.

---

## Parte 4 — Leitura do executor + perguntas

_(preenchido no fim; não vinculativo; a decisão é do dono)_
