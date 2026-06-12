# Prompt L0 — F sob a fronteira E1 (`Content::Dynamic` + chain única)
Hash do Código: 12c03c65

**Camada**: L1 · **Módulos**: `01_core/src/entities/{content,elements/mod,style,style_chain,value}.rs`
**Decisão de origem**: **ADR-0106** (fronteira de extensão E1) + ADR-0105 (modelo D
agora, F destino) + ADR-0104 (atomicidade) + ADR-0026 (enum fechado, complementada).
**DEBT**: 99.E (StyleChain não materializada) — F fecha-o sob esta fronteira.
**Experimento**: P332 (spikes E1/E2/E3) · **Spike-2**: P333 Parte 2 (`#show`, achados S*).
**Tipo**: especificação do F (propriedades reificadas + StyleChain + extensão por
elemento de utilizador). **Forward-looking**: descreve o desenho; a implementação é
**por lotes, pós-Trava**, com hash humano por lote.

> **Estatuto: APROVADO (P333 checkpoint) — ativação em F-1 (P334).** O dono
> aprovou este L0 (ADR-0106 §Aprovação da Trava). O código F-1 declara `@prompt
> entities/f_fronteira_e1.md` e o warning V7 (órfão) limpa. Trava arquitetural
> (CLAUDE.md) cumprida.

---

## §0 — Estatuto e sincronização de hash

**Aprovado pelo dono no checkpoint do P333** (ADR-0106 §Aprovação da Trava). O
**primeiro lote** (a fronteira, §3a — **F-1, P334**) materializa-o: o código
declara `@prompt entities/f_fronteira_e1.md` + `@prompt-hash <hash deste
ficheiro>`, e `crystalline-lint --fix-hashes .` sincroniza (o warning V7 órfão
limpa). **Exceção (F-1)**: `entities/elements/_comum.md` recebeu o método
defaultado `dyn_kind_name` no trait `Element` (§A.1.5 lá; hash re-sincronizado).
Os restantes L0 de estilo (`entities/style.md`, `entities/style_chain.md`,
`entities/content.md`) **permanecem intocados** (só citados) — o lado estilo (3b)
é F-2+.

**Ajuste de Fase A (P334) — `dyn_hash` removido**: `content_hash::hash_content`
serializa por `format!("{:?}")` (Debug estrutural), **não** por match. Logo
`Content::Dynamic(Arc<dyn DynElement>)` faz hash automático pelo `Debug` do `dyn`
(que `DynElement` já exige) — **sem arm em `content_hash` e sem `dyn_hash` no
trait**. `dyn_eq` + `as_any` permanecem (o `eq` do hub É um match — precisa do arm
`Dynamic`). Ver §3a.2/§3a.3 atualizados.

---

## §1 — O que F materializa (resumo de uma página)

F unifica **elemento** e **estilo** numa só obra (A ≡ F, §4 do P313), sob a
fronteira **E1**:

- **Lado elemento (3a)**: uma variante `Content::Dynamic(Arc<dyn DynElement>)` —
  a **única** porta de extensão. O elemento de utilizador implementa o **mesmo
  `trait Element`** dos 65 nativos (precedente vivo, ADR-0106); um **blanket impl**
  bridga `Element` → `DynElement` (object-safe) sem o utilizador saber. Os 65
  nativos **não mudam** (continuam monomórficos). O hub ganha **+1 variante** e
  **1 arm por match** (idêntico ao dispatch dos 65).
- **Lado estilo (3b)**: a chain única do Layouter passa a `(10 campos nativos
  fechados) + (mapa aberto PropKey → Value)`. As 4 `Set*` viram **entradas na
  chain** (o canal único — o "F-D" renascido). O `enum Value` fica **fechado por
  espelhar a linguagem typst** (escape `Custom` só com evidência — §3b.4).
- **Contrato (3c)**: C1–C8 (fidelidade comportamental, P329) + os S* do spike-2 +
  a rede de caracterização (+11, P331 Fase 2) + a trava da ADR-0105 cláusula 3
  (verificação mecânica reposta no caminho dinâmico).

A migração é **incremental** (ADR-0106 item 3): fronteira aditiva → canal `Set*`
→ `Styled`/de-bake/folhas por lote.

---

## §3a — Lado elemento: a variante `Dynamic` e a object-safety

### 3a.1 — O problema: `Element` não é object-safe

O `trait Element` atual (`elements/mod.rs:113`) **não** é object-safe, por dois
motivos:

1. **Supertraits** `Clone + PartialEq + Hash` — nenhum é object-safe (`Clone`
   devolve `Self`; `PartialEq<Self>`/`Hash` têm `Self` na assinatura).
2. **Métodos genéricos** `map_content<F: FnMut…>` e `map_text<F: FnMut…>` — método
   genérico não entra em vtable.

Logo `Arc<dyn Element>` **não compila**. O L0 resolve isto **sem tocar nos 65
nativos** e **sem o utilizador implementar um trait diferente** (preserva o
precedente, a razão de ADR-0106).

### 3a.2 — Resolução: trait object-safe `DynElement` + blanket impl

`Element` **permanece exatamente como está** (estático, genérico, os 65 nativos
intocados, despacho monomórfico, zero custo). Acrescenta-se um trait object-safe
`DynElement` que **ninguém implementa à mão** — um **blanket impl** deriva-o de
qualquer `Element`:

**Como construído (F-1, `entities/elements/dynamic.rs`):**

```rust
use std::any::Any;

/// Versão object-safe de `Element`. NÃO é implementada à mão: o blanket dá-a a
/// todo `Element + Send + Sync + 'static`.
///
/// Nomes `dyn_*` DISTINTOS dos de `Element` de propósito: o blanket torna todo
/// `Element` também `DynElement`; nomes coincidentes tornariam `elem.is_empty()`
/// num tipo concreto (com ambos em scope) AMBÍGUO (E0034). Com `dyn_*` o caminho
/// nativo usa `Element::*` e a folha dinâmica usa `DynElement::dyn_*`.
///
/// `Send + Sync`: o `Content` vive em contextos `Send + Sync` (introspector),
/// logo `dyn DynElement` (e o `Arc`) têm de o ser.
pub trait DynElement: std::fmt::Debug + Send + Sync + 'static {
    fn dyn_plain_text(&self) -> String;
    fn dyn_is_empty(&self) -> bool;
    fn dyn_map_content(
        &self,
        f: &mut dyn FnMut(&Content) -> SourceResult<Option<Content>>,
    ) -> SourceResult<Content>;
    fn dyn_map_text(&self, f: &mut dyn FnMut(&str) -> String) -> Content;
    fn dyn_get_field(&self, field: &str) -> Option<Value>;
    fn dyn_element_kind(&self) -> Option<ElementKind>;
    fn dyn_to_payload(&self) -> Option<ElementPayload>;
    fn dyn_kind(&self) -> &'static str;   // S1; vem de Element::dyn_kind_name
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, other: &dyn DynElement) -> bool;
    // sem `dyn_hash`: `Content::Dynamic` faz hash pelo `Debug` do `dyn` (§0).
}

impl<T: Element + Send + Sync + 'static> DynElement for T {
    fn dyn_plain_text(&self) -> String { Element::plain_text(self) }
    fn dyn_is_empty(&self) -> bool { Element::is_empty(self) }
    fn dyn_map_content(
        &self,
        f: &mut dyn FnMut(&Content) -> SourceResult<Option<Content>>,
    ) -> SourceResult<Content> {
        // `&mut dyn FnMut` não satisfaz `F: Sized`; um reborrow local torna
        // `F = &mut dyn FnMut` (que É Sized e FnMut).
        let mut g = f;
        Element::map_content(self, &mut g)
    }
    fn dyn_map_text(&self, f: &mut dyn FnMut(&str) -> String) -> Content {
        let mut g = f;
        Element::map_text(self, &mut g)
    }
    fn dyn_get_field(&self, field: &str) -> Option<Value> { Element::get_field(self, field) }
    fn dyn_element_kind(&self) -> Option<ElementKind> { Element::element_kind(self) }
    fn dyn_to_payload(&self) -> Option<ElementPayload> { Element::to_payload(self) }
    fn dyn_kind(&self) -> &'static str { Element::dyn_kind_name(self) }
    fn as_any(&self) -> &dyn Any { self }
    fn dyn_eq(&self, other: &dyn DynElement) -> bool {
        other.as_any().downcast_ref::<T>().is_some_and(|o| self == o)
    }
}
```

O id de kind (S1) é defaultado em **`Element::dyn_kind_name`** (`{ "" }`, em
`_comum.md` §A.1.5); o utilizador sobrepõe-no no seu `impl Element` (a fixture
`callout` devolve `"callout"`). O `DynElement::dyn_kind` apenas o repassa.

**Por que isto cumpre ADR-0106 à letra**: o utilizador escreve `impl Element` —
**o mesmo trait dos 65 módulos**, o precedente vivo, o menor custo-IA. O
`DynElement` é maquinaria invisível (blanket); o utilizador nunca o vê. Object-safety
resolvida; os 65 nativos **não tocados**; o `&mut dyn FnMut` custa **uma chamada
indireta por nó no caminho frio** de `map_*` (não é folha quente — ADR-0030 ok).

### 3a.3 — `eq`/`hash`/`clone` no caminho dinâmico (content-preserving)

- **`Clone`**: `Content::Dynamic(Arc<dyn DynElement>)` clona o **`Arc`** (O(1),
  ponteiro) — não precisa de `Clone` no `dyn`. Paridade com os 65 (que já usam
  `Arc`).
- **`PartialEq`**: o hub despacha `(Content::Dynamic(a), Content::Dynamic(b)) =>
  a.dyn_eq(b.as_ref())`. `dyn_eq` faz **downcast** ao tipo concreto e compara
  estruturalmente (`self == o` via o `PartialEq` derivado do `*Elem`). Tipos
  diferentes ⇒ `false` (paridade: elementos de kinds distintos nunca são iguais).
- **`Hash`**: `content_hash::hash_content` serializa por `format!("{:?}", content)`
  (Debug estrutural) — **não** é um match. Logo `Content::Dynamic` faz hash
  **automaticamente** pelo `Debug` do `dyn DynElement` (que `DynElement` exige como
  supertrait, delegado ao `#[derive(Debug)]` do `*Elem` concreto). **Sem arm em
  `content_hash`; sem `dyn_hash` no trait** (ajuste de Fase A P334, §0). Mantém-se a
  regra do modelo D (`_comum.md` §A.1.1.b): a **relação** preserva-se; valores
  absolutos são internos (dedup de introspecção), não observáveis no PDF.

### 3a.4 — Identidade dinâmica + leitura de campos para `#show` (S1, S7)

O spike-2 (`f-spike2-show-passo-333.md`) mediu na quarentena que o match de
`#show <elem>:` faz `target.elem() == *element` (selector.rs:134) — exige
**identidade de elemento estável e `Eq`** — e que o closure de `#show` **lê campos**
do elemento (`it => it.body`, content/field.rs). Logo o contrato dinâmico **tem
de** expor:

- **S1 — identidade estável**: o trait expõe um id de elemento **estável e
  comparável por igualdade**, idêntico para todas as instâncias do mesmo kind. O
  `element_kind()` (já no trait, default `None`) é o veículo natural **para os
  locatáveis**; para o casamento de `#show` de qualquer elemento dinâmico, o
  registro (3a.5) fornece o **nome do kind** (`dyn_kind_name()` → `&str` interned,
  `Eq`). **Trava-Q2(a): o L0 fixa que `dyn_kind_name` é obrigatório e estável.**
- **S7 — leitura de campos pelo closure**: o `get_field(&self, field) ->
  Option<Value>` (já no trait) é o que o closure de `#show` usa para `it.body`,
  `it.with(..)`. **Trava-Q2(b): sem `get_field` populado, `#show` sobre o elemento
  dinâmico é impossível** — o L0 exige que o elemento dinâmico declare os seus
  campos settáveis/legíveis via `get_field` (e, simetricamente, os recebe via o
  canal de set — 3b).

### 3a.7 — Realização + guards: onde mora o lifecycle (S2, S3, S4, S6 · Trava-Q1)

O spike-2 mediu que o `#show` do vanilla precisa de: iterar recipes da chain
**innermost-first**, 1 transformação func por passe (S3); um **loop multi-passe
até fixpoint** com teto de profundidade (S4); e **guards por-nó** que fazem a
recursão terminar (S2) — o vanilla guarda no `meta().lifecycle` bitset do elemento
empacotado (content/mod.rs:148-156).

**Trava-Q1 (a única divergência estrutural de E1 vs vanilla)**: o `Arc<dyn
DynElement>` é **imutável e partilhado** — o guard set **não pode** viver dentro do
`dyn` (mudaria por-instância de realização, custaria um campo no trait a cada
utilizador, e quebraria a clonagem O(1) do `Arc`). **Resolução proposta no L0**
(decisão a confirmar pelo dono na Trava):

> O **lifecycle/guard set é estado da passagem de realização**, não do elemento.
> Mora numa camada de realização **nova em `rules/`** (eco do `typst-realize`
> separado do vanilla), como um **invólucro transparente** que pode embrulhar
> **qualquer** nó `Content` (nativo OU `Dynamic`) — exatamente o `Content::Guarded`
> que o spike usou. Assim: (i) os 65 nativos **não** ganham campo meta; (ii) o
> `Content::Dynamic(Arc<dyn DynElement>)` fica **limpo** (Arc O(1) preservado);
> (iii) o guard é **uniforme** nativo+dinâmico (S6 — o nó dinâmico é membro pleno
> da árvore de realização, não folha opaca). **Alternativa** (campo `meta` ao lado
> do `Arc` na variante `Dynamic`) fica registada — mais fiel ao vanilla, mas
> assimétrica (nativos sem meta). **O dono decide na Trava.**

O loop de realização (S4) vive em `rules/` (topologia — `entities` não conhece
`rules`). O nó dinâmico participa da árvore de realização como qualquer nativo
(S6); o seu `map_content_dyn`/children permite o walk (limite medido: o spike
tratou filhos de elemento dinâmico como folha já-realizada — o caso geral exige
`with_children`/walk no trait; **registado como item de F-2, não provado pelo
spike**).

### 3a.5 — O registro (os dois públicos, sem global — pureza L1)

- **Público Rust**: implementa `trait Element` no seu `*Elem` + (para o público
  typst) regista um **construtor por nome** (`"callout" → fn(args) -> Content`).
- **Público typst**: `#callout(...)` na linguagem resolve para o construtor
  registrado; `#set callout(tone: …)` e `#show callout: …` operam sobre o **nome
  dinâmico** (`dyn_kind_name`).
- **Pureza L1 (precedente do spike E1)**: o registro é **injetado** (passado
  explicitamente no contexto de eval/layout), **não** um `static`/`OnceLock`
  global (V13/MutableStateInCore). Eco do `Registry` passado à mão no spike.

### 3a.6 — O que o hub muda (aditivo, content-preserving)

- **+1 variante**: `Content::Dynamic(Arc<dyn DynElement>)`.
- **6 matches** ganham **1 arm cada**, idêntico ao dispatch dos 65:
  `Content::Dynamic(e) => e.plain_text()` / `e.is_empty()` /
  `e.map_content_dyn(f)` / `e.map_text_dyn(f)` / `e.get_field(field)`; `eq` →
  `a.dyn_eq(b.as_ref())`; `hash_content` → `e.dyn_hash(state)`.
- **Locatável dinâmico**: `extract_payload.rs` → `Content::Dynamic(e) =>
  e.to_payload()`; `locatable.rs` → `Content::Dynamic(e) => e.element_kind().is_some()`.
  O consumo por `ElementPayload` (`from_tags`/walk) é **inalterado** (matcheia o
  payload, não o `Content`) — precedente dos lotes locatáveis (P321+).
- **Exaustividade preservada** (ADR-0026): o enum continua fechado; `Dynamic` é
  **uma** variante. O dinâmico é o **conteúdo** da variante, não o enum.

---

## §3b — Lado estilo: chain única + mapa aberto + `Value`

### 3b.1 — A chain estendida (10 nativos fechados + mapa aberto)

A `enum Style` atual (`style.rs:33`) tem **10 variantes** 1:1 com `TextStyle`:
`Bold(bool)`, `Italic(bool)`, `Size(Pt)`, `Fill(Color)`, `HeadingLevel(u8)`,
`Lang(Lang)`, `Weight(u16)`, `Tracking(Length)`, `Leading(Length)`,
`Font(FontList)`. F mantém estas **10 fechadas** (zero custo aos nativos) e
acrescenta **um canal aberto** para props de utilizador:

```rust
pub enum Style {
    // ── 10 nativas fechadas (inalteradas) ──
    Bold(bool), Italic(bool), Size(Pt), Fill(Color), HeadingLevel(u8),
    Lang(Lang), Weight(u16), Tracking(Length), Leading(Length), Font(FontList),
    // ── canal aberto (props de elemento de utilizador) ──
    Custom { kind: EcoString, key: EcoString, value: Value },   // forma de partida
}
```

Resolução por **fallback instância → chain → default** (C1+C3 do contrato), com
**fold** onde aplicável (C2). As nativas resolvem-se como hoje (match direto); as
`Custom` resolvem-se por `(kind, key)` na walk-up da chain. **Decisão de forma
final (campo único `Custom` vs `PropMap` por entrada) fica para o desenho do lote
do estilo — aqui declara-se o contrato, não a representação ótima.**

### 3b.2 — O `enum Value` fechado, espelhando a linguagem (ADR-0106 item 2)

O `enum Value` (`value.rs:18`) tem ~21 variantes que **espelham os tipos de valor
da linguagem typst**: `None, Auto, Bool, Int, Float, Str, Array, Dict, Module,
Datetime, Func, Content, Length, Ratio, Angle, Color, Stroke, Fraction, Gradient,
Location, Align`. **Este conjunto é fechado pela própria linguagem**: uma prop de
utilizador de um tipo **já existente** (ex.: `tone: Str`, `width: Length`) custa
**0 ficheiros de core**; um **tipo de valor novo** = extensão de linguagem =
mudança de core **legítima e rara**. O teto do `Value` (medido em E3) fica assim
na fronteira **da linguagem**, não na do elemento — que é onde pertence.

### 3b.4 — O escape `Value::Custom(Arc<dyn …>)` — NÃO entra (evidência P333)

Uma variante de escape `Value::Custom(Arc<dyn …>)` (contrato `eq`/`hash`/`display`;
eco do `Value::Dyn` do vanilla) **só entra se** o spike-2 ou este L0 demonstrarem
necessidade — **não por precaução** (ADR-0106 item 2). **Estado atual da
evidência (P333)**: o spike-2 exercitou `#show`/`#set`/`query`/render do `callout`
com a prop `tone` como `Str` e o corpo como `Content` — **tudo coube no `Value`
fechado**; **nenhum caso exigiu um tipo de valor fora do enum**. Logo, **pela
evidência atual, `Value::Custom` NÃO entra** (fica fora por falta de necessidade
demonstrada, conforme ADR-0106 item 2). **Gatilho de reabertura**: o primeiro
elemento de utilizador real cuja prop precise de um tipo que não seja `None/Bool/
Int/Float/Str/Array/Dict/Module/Datetime/Func/Content/Length/Ratio/Angle/Color/
Stroke/Fraction/Gradient/Location/Align` — aí reavalia-se com o caso concreto. **A
confirmação final (manter fora) é item do checkpoint da Trava.**

### 3b.5 — O canal único das `Set*` (o F-D renascido) — primeira aplicação

As 4 `Set*` atuais são marcadores opacos por **4 canais distintos** (inventário
1a, P331): `SetHeadingNumbering` (62 sites) e `SetFigureNumbering` (5) via
Introspector; `SetEquationNumbering` (16); `SetPage` (8, muta `page_config`). F
unifica-as como **entradas na chain única**:

- Cada `Set*` vira uma entrada `Style::…` (nativa para as de texto/numbering;
  `Custom` se a forma final preferir) resolvida por fallback léxico — fechando o
  DEBT 99.E (set rule passa a respeitar escopo de container, não global).
- **`SetPage` (o caso difícil, D4 do dossiê)**: tem **2 produtores** (Q4, P331) e
  muta `page_config` fora da chain. O desenho resolve **qual produtor** alimenta a
  chain e como `page_config` passa a ler da chain. ⟨detalhar no lote do canal⟩.
- **`SetEquationNumbering` (lacuna D1/Q3)**: sem produtor via eval puro (a rede de
  caracterização P331 registou que o efeito não é caracterizável por layout puro).
  O desenho fecha o produtor em falta. ⟨detalhar no lote do canal⟩.
- **Spec de paridade**: a rede de caracterização (+11 testes, P331 Fase 2) é o
  contrato comportamental que o canal `Set*` **não pode quebrar**.

### 3b.6 — `#show` no desenho (S3, S4, S5, S6 — validado pelo spike-2)

> **Correção do inventário 1a (Lote F-3, Fase A — registro (a)):** o P331 §1a
> gravou "`#show` não existe" no cristalino. **Falso.** A Fase A do F-3 achou um
> `#show` **eager** já em produção: `eval/rules.rs:182 intercept_content` →
> `:69 apply_show_rules`, aplicado na **criação** do conteúdo (não numa passagem
> de realização). Casa nativos por **endereço de função** do selector
> (`#show heading: …` → `native_heading` → `Selector::NodeKind(Heading)`,
> `eval/rules.rs:496 eval_show_rule`; selectors em `entities/show.rs` —
> `Selector::{Text, NodeKind}`). **Tem** anti-recursão por `active_guards`
> (stack de `RuleId`) + teto de profundidade 64 (`route_check_show_depth`,
> paridade `typst-realize:402`). É **exposto na linguagem** (`#show` é parseado).
> Logo o cristalino **já diverge** do vanilla aqui: **eager single-pass** vs
> **realização multi-passe**.
>
> **Divergência S2–S6 registada + gatilho (Lote F-3 — registro (c)):** o modelo
> **eager** do cristalino (com guards+depth) é a forma cristalina **por desenho**
> (fidelidade é **comportamental**, P329 — não estrutural). Os S2–S6 (guards
> por-nó, ordem innermost-first, loop multi-passe até fixpoint, `Transformation =
> Content|Func|Style`, nó dinâmico membro pleno) do spike-2 ficam como
> **diferença estrutural não-implementada**. **Gatilho concreto**: quando `#show`
> entrar na **cobertura de linguagem** (parmetrização real de show rules de
> utilizador), os **5 casos do spike-2** viram **testes de paridade** contra o
> **vanilla medido** (`lab/typst-original/`); **se o eager falhar** algum caso, a
> **realização multi-passe vira lote** nesse momento — não antes. Até lá, o eager
> basta.
>
> **Resultado do gatilho (Lote F-3 inc-2, S3 — Stage 0 executado):** `#show <dyn>`
> entrou na linguagem (`Selector::DynKind`). Decisão Stage 0 (dono, Opção 1):
> paridade contra a semântica **medida na fonte** pelo spike-2 (vanilla não tem
> elemento custom trivial nem binário pronto — não se roda o binário). **Veredito
> por caso:**
> - **Caso 2 (recursão/guard): PARIDADE ✅** — guard por `RuleId` termina (teste
>   `f3s2_show_callout_anti_recursao_termina`; vanilla `typst-realize:472-474`).
> - **Caso 5 (nativo+dyn): PARIDADE ✅** — mesma travessia (`f3s2_dyn_e_nativo_
>   coexistem`).
> - **Caso 4 (escopo): DIVERGÊNCIA — gatilho DISPARADO.** O eager **não confina**
>   `#show` ao bloco (muta `engine.show_rules` da declaração em diante; propriedade
>   **pré-existente**, afeta nativos também). Vanilla confina via `StyledElem`
>   (`content/mod.rs:744-752`). Teste-divergência checado:
>   `f3s3_caso4_escopo_eager_nao_confina_divergencia_registrada`.
> - **Caso 1 (composição multi-regra) + Caso 3 (show-set): FALTA-SUPERFÍCIE** —
>   precisam de multi-passe / `Transformation::Style`. Registrados.
>
> **Logo a `#show` léxica + realização multi-passe é agora um LOTE concreto** (não
> mais "se/quando") — ver fila em `f-plano-lotes-passo-333.md` (lote **F-realização**).
> Conforme (c): **não consertada inline**; decide-se com o dado. F-3 (a fronteira
> entrar na linguagem) **fecha** com o eager + a divergência registada.

O spike-2 validou os 5 casos sobre `Content::Dynamic` (todos PASS) — **como
desenho de referência**, não como implementação obrigatória agora. O desenho do
`#show` no F, sob E1 (a materializar **se/quando** o gatilho acima disparar):

- **Recipes na chain única** (S3, S5): `Style` ganha um caso de **recipe**
  (transformação) ao lado das props. **`Transformation = Content | Func | Style`**:
  `Content` (substituição direta), `Func` (closure `it => …`), `Style` (o
  **show-set** `#show k: set …`, que empurra um `#set` scoped e **não consome** o
  passe — S5). **Validado** (caso 3).
- **Ordem innermost-first, 1 func/passe** (S3): a realização itera as recipes da
  chain do mais interno/recente para o mais externo, fixa a **primeira** recipe
  func não-guardada como o passo, ignora as restantes nesse passe. **Validado**
  (caso 1, 3 passes).
- **Loop multi-passe até fixpoint + teto** (S4): a realização re-realiza o
  conteúdo produzido até estabilizar; os **guards por-nó** (3a.7) garantem
  convergência; um **teto de profundidade** corta recursão patológica (eco do
  `route depth`/`check_show_depth` do vanilla). **Validado** (casos 1 e 2
  convergem; o spike provou empiricamente — o teto formal é desenho de F-2).
- **Escopo por subárvore** (S6): a recipe confina-se à subárvore que embrulha
  (precedente `StyledElem`); um irmão fora não a vê. O nó dinâmico é **membro
  pleno** da árvore de realização (pode ser filho de um nó estilizado e recebe a
  chain scoped). **Validado** (caso 4: o dinâmico respeita o escopo).
- **Uniformidade nativo+dinâmico** (caso 5): a **mesma** chain transforma um
  `heading` nativo **e** o `callout` dinâmico. O `#show` não distingue a porta.

**Marcado como argumentado (não provado a fundo pelo spike)**: o teto de
profundidade formal; o walk de **filhos de elemento dinâmico** (o spike tratou-os
como folha — o caso geral exige `with_children`/walk, item de F-2); selector
`.where(field:)`/labels/regex (o spike fez match só por kind); integração com
`comemo`/introspection loop/locations/tags (o loop do spike é `while changed`
puro).

### 3b.7 — Fronteiras declaradas (o que fica para lotes futuros)

- **`Styled`** (`Styled(Box, Styles)`): re-resolução numa 2ª StyleChain no
  Layouter (1a). Migra **por lote** quando o canal único o alcançar — não no
  primeiro movimento (ADR-0106 item 3). Gatilho de revisita: o lote do estilo.
- **De-bake de `#set text`**: hoje `#set text` **assa** `TextStyle` em
  `Content::Text` (chain descartada, 1a). O de-bake (a chain deixar de assar)
  é **lote dedicado** — gatilho: depois do canal `Set*` provar a chain léxica.
- **3 folhas provisórias** (`Text`/`MathText`/`MathIdent`, DEBT-58): os campos que
  o vanilla lhes daria são estilo (StyleChain) → resolvem-se **quando a chain
  léxica existir**. Gatilho: o lote do estilo.

---

## §3c — Contrato de verificação

- **C1–C8** (fidelidade comportamental, inventário 1b/P331; P329): aplicáveis ao
  caminho dinâmico e ao canal `Set*`. ⟨listar C1–C8 concretos do inventário 1b no
  lote — aqui referencia-se a fonte.⟩
- **S1–S7** (spike-2, `f-spike2-show-passo-333.md` §3): cada achado vira asserção
  de teste no lote correspondente —
  **S1** id de elemento estável `Eq` (match de seletor);
  **S2** guard/lifecycle por-nó na camada de realização, não no `dyn` (3a.7);
  **S3** recipes na chain, innermost-first, 1 func/passe;
  **S4** loop multi-passe até fixpoint + teto de profundidade;
  **S5** `Transformation = Content | Func | Style` (show-set não consome passe);
  **S6** nó dinâmico é membro pleno da árvore de realização;
  **S7** `get_field` legível pelo closure de `#show`.
  **Trava-Q1** (sítio do guard) e **Trava-Q2** (id+`get_field` no contrato) são
  decisões do dono no checkpoint — resolução proposta em 3a.7/3a.4.
- **Rede de caracterização existente** (+11, P331 Fase 2, `rules/layout/tests.rs`
  `mod f_caracterizacao_estilo`): **spec de paridade** — nenhum lote do F a altera
  (alterar teste para passar = mudança de comportamento = bug).
- **Trava da ADR-0105 cláusula 3** (verificação mecânica reposta): no caminho
  dinâmico o compilador deixa de garantir exaustividade por elemento (o `dyn`
  esconde o tipo). O mecanismo concreto — **teste que varre o registro × os
  backends** (cada kind registrado tem layout/show handler) **ou** **regra nova do
  `crystalline-lint`** — dimensiona-se com o **M2** (acesso a campo/propriedade que
  F troca por lookup). **A lente `tekt-cargo-dsm` NÃO substitui esta trava**
  (P333 Parte 5 §3): é módulo-level e estrutural — vê o edge, não a tabela de
  handlers nem o acesso a campo (R3/R4). A lente serve de **trava de camada
  complementar** (mede `edges(content→elements::*)` por lote, via `--comparar`),
  mas a trava da cláusula 3 fica em teste/lint mantido à mão até R3+R4 fecharem na
  sessão da lente. Ver `baseline-estrutural-lente-passo-333.md`.
- **Object-safety**: um teste de compilação (`Arc<dyn DynElement>` construível +
  os 6 dispatches) garante que a porta E1 não regride para não-object-safe.

---

## §Dois públicos demonstrados (no papel)

- **Caminho Rust** (programador): escreve `struct CalloutElem { … }`, `impl
  Element for CalloutElem`, regista `"callout" → construtor`. Ganha
  `#set`/`#show`/`query`/render. **0 ficheiros de core.** Precedente: os 65
  módulos `entities/elements/*`.
- **Caminho typst** (autor, sem Rust): usa elementos registrados via pacote;
  escreve `#show`/`#set`/composição na linguagem. **O que é possível sem Rust**:
  compor, estilizar, mostrar elementos existentes (nativos ou de pacotes). **O que
  exige Rust**: **definir** um elemento novo com layout próprio (o layout vive em
  `rules/`, fora do trait — topologia). **O `#show` puro-linguagem** (sem Rust)
  pode **re-compor e re-estilizar** um elemento existente — o closure recebe o nó e
  devolve `Content` arbitrário (S7), lendo campos por `get_field` (validado no
  spike-2, caso 5: `#show` reescreve nativo e dinâmico na mesma chain). O que o
  `#show` puro-linguagem **não** dá é um **layout novo** primitivo (isso é o
  caminho Rust). Fronteira limpa: **aparência/composição** = linguagem;
  **layout primitivo novo** = Rust.
