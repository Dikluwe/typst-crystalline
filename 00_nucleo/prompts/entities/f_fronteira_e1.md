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

#### 3a.7-bis — Materialização P340 (fatia 2): o que aterrou e o que ficou medido-mas-parado

> **Modo autônomo (P340).** Este registro substitui a "Trava" do molde por
> medição→decisão→registro. Resolvido pela fonte (`lab/typst-realize/src/lib.rs`)
> e pela suíte herdada (2719).

**Aterrado — Caso 4 / `f3s3` (fatia 2a, confinamento de escopo).** O `#show` num
`ContentBlock` `[]` deixa de vazar: `eval/mod.rs` passa a clonar
`local_show_rules` (Arc O(1)), espelhando o `CodeBlock` `{}`. **Medição:** a
mudança toca **exatamente 1 teste** (o `f3s3`, único que usa `#[#show]`); todo o
resto content-preserving (suíte 3238/0). É o confinamento estrutural que faltava
ao eager no `[]` — paridade com o `StyledElem` do vanilla
(`content/mod.rs:744-752`). `f3s3` virou (de "vaza 2×" a "confina 1×"), a exceção
sempre planeada.

**Terminação (eager) — já existe.** O modelo eager termina por **dois**
mecanismos vivos e testados: (i) `active_guards` (stack de `RuleId`) — uma regra
não re-aplica ao próprio output (`show_rule_nao_recursiva_sem_stack_overflow`,
`f3s2_show_callout_anti_recursao_termina`); (ii) **teto de profundidade 64**
(`check_show_depth`, `world_types.rs:249`; paridade vanilla `lib.rs:401-402`). O
guard é tão eficaz que o teto-64 é um **backstop** — difícil de alcançar no eager
(o teste que o dispara é, na verdade, concern do multi-passe).

**Trava-Q1 — DECIDIDA (medida).** Representação do guard por-nó:
**`Content::Guarded` transparente** (invólucro que pode embrulhar qualquer
`Content`, nativo OU `Dynamic`), **não** campo `meta` no `Dynamic`. Razão medida:
o vanilla guarda no `meta().lifecycle` bitset do elemento (`content/mod.rs:148-156`),
mas no E1 o `Arc<dyn DynElement>` é imutável/partilhado (clone O(1)); um wrapper
transparente (i) não põe meta nos 65 nativos, (ii) mantém o `dyn` limpo, (iii) é
uniforme nativo+dinâmico (S6). Transparência a `plain_text`/`is_empty`/`map_*`/
closures é **requisito de teste quando o invólucro nascer**.

**PARADO — multi-passe / fixpoint (fatia 2b: casos 1/2, e os "2 passes" do caso 5).**
**Não materializado nesta corrida.** Razão **medida, não preguiça**: o modelo
cristalino é **eager por criação** (`intercept_content` intercepta no momento da
criação; compõe via cascata de interceção). A suíte herdada tem **18 testes de
`#show` que asseveram exatamente essa semântica eager** (ex.:
`show_rule_composicao_sem_loop` asserta `count==1` / "não reaplicada";
`show_rule_encadeamento_duas_regras`). Substituir o eager por um **loop externo
até fixpoint** (o modelo do vanilla: `realize→visit→visit_show_rules`, fixpoint na
**introspection loop**, `recipe-index` innermost-first, `lib.rs:335/401/472`)
**mudaria** a saída desses 18 testes — e P340 declara content-preserving
**inviolável** (única exceção: `f3s3`). Por **regra 6 (degradação segura)** +
**regra 3d (medição não decide barato → opção conservadora reversível)**: aterra-se
o seguro (caso 4), **estaciona-se** o multi-passe como **lote próprio futuro**
(onde a sua paridade se mede contra um conjunto de testes deliberadamente evoluído,
não retrofitado sob no-change estrito). O `Content::Guarded` (Trava-Q1) nasce
**nesse** lote, com a sua prova de transparência. **DECISÃO AUTÔNOMA PROVISÓRIA —
revisar:** confirmar que o multi-passe é lote próprio (não retrofit) é do dono.

**ATERRADO P348 — recursão de element rules por PONTO-FIXO MORFOLÓGICO (modelo α).**
A cadeia P347→P347d mediu: reproduzir o vanilla **exato** (terminar por identidade de
instância) exigiria o multi-passe + guard-por-instância + Revocation; mas essa terminação
do vanilla é **GEROU** (mecânica, não promessa — P347b/c, commit #3327) e a Revocation é
**INTERNA** (P347d — só o motor a constrói, sem porta de usuário). Logo o cristalino **não**
porta o multi-passe: `apply_show_rules` (`rules/eval/rules.rs`) ganhou um **loop local de
revisitação** — o output de uma element rule que re-casa é re-alimentado no conjunto de
regras até **ponto-fixo morfológico** (`Content::morph_canon`, P345: para quando a regra é
no-op morfológico) ou até o **teto-64 backstop** (`MAX_SHOW_RULE_DEPTH`; não-convergente →
erro com a mensagem base do vanilla `"maximum show rule depth exceeded"` + hints,
byte-idêntica, ADR-0033). O `active_guards` permanece (anti-recursão DURANTE a chamada do
recipe — criação aninhada); a revisitação é o loop, **após** o recipe devolver. O caminho
comum (output não re-casa) **não paga `morph_canon`** (a checagem de ponto-fixo só entra da
2ª aplicação — M-trigger, P348). **Divergência consciente** (ADR-0107): `#show heading: it
=> [= Z]` **converge para "Z"** onde o vanilla erra — o vanilla termina por identidade de
instância (mecânica), o cristalino por morfologia. **Text rules não mudam** (passe único,
`map_text`; a Revocation servia só a elas no vanilla, e o cristalino já não as revisita) —
logo o `Content::Guarded` (Trava-Q1) e o multi-passe (fatia 2b) ficam **dispensados** para a
recursão de element rules: α resolve sem eles.

**Flag de erro completo — capacidade interna FEITA (P350c; forma C-com-origem intermédio).**
Quando ligada, o erro do teto ganha um **3º hint** num **canal separado** (`with_hint`)
classificando em **DOIS rótulos sólidos**: **cíclico** (uma morfologia do caminho repetiu —
fato medido pelo `==`/`morph_canon` do P345) e **não-convergente** (teto sem repetição). O
**terceiro rótulo "converge-fundo" foi CORTADO** (decisão do dono, ADR-0108 regra 4: afirmar
só o medido — distinguir divergente de converge-fundo adivinharia o futuro pós-corte). A
**mensagem base + os 2 hints do vanilla são byte-idênticos** (a flag só *acrescenta* o 3º).
**Caminho quente intacto**: o histórico de morfologias só é alocado/computado quando a flag
está ligada (atrás do `if ctx.full_error` em `apply_show_rules`/`apply_all`). **Origem→leitura**
(C-com-origem intermédio, P350b/c): a flag mora em `RunIntent` (L2, `cli.rs`, ao lado de
`colored`) → desce a `eval` pelo **sibling `eval_with_full_error`** (a assinatura **pública**
de L3 `compile_to_pdf_bytes` **não muda**; `eval()` continua o delegado com `false`) →
`EvalContext.full_error` (L1) é lida no ponto do erro. **L1 não lê env** (recebe resolvido).
**Débito nomeado (P350c):** (1) o **parsing CLI** (`--full-error` em `Args` → `RunIntent`);
(2) o **fio `RunIntent`→`eval_with_full_error`** pelo caminho **interno** de L3 (hoje a
produção entra com `false`; ligar o `RunIntent` real é débito junto com a CLI). Testável já
via `eval_with_full_error(…, true)`.

### 3a.8 — Fatia 1 da F-realização: a fundação do transporte `StyledElem`-scoped (aditivo, β1)

> **Estatuto.** Esta é a **fatia 1** da F-realização (P339). Constrói **só** o
> transporte; **não** religa consumidores (F-5), **não** confina `#show` (o eager
> fica intocado), **não** introduz guards/multi-passe (Trava-Q1 → fatia 2). As
> fatias 2 (composição/multi-passe) e 3 (show-set) são P340/P341.

**A lacuna medida (P339, da fonte).** Hoje os 3 `#set …(numbering:)` —
`heading` (`eval/rules.rs:242`), `equation` (`:266`), `figure` (`:313`) — só
fazem `engine.styles.push_custom("X.numbering", …)`. Esse custom vive **apenas**
na chain de **eval** (`engine.styles`), que é **descartada na fronteira
eval→layout**: `pub fn layout(content)` (`layout/mod.rs:2644`) e
`layout_with_introspector(content, introspector)` (`:2668`) **não** recebem
`engine.styles`; `self.chain` do Layouter nasce **fresca**
(`:374`, `default_chain()`) e é escrita **só** por `Content::Styled`/`Strong`
(`:1251`/`:1293`). Os 3 consumidores do gate leem o **campo assado**, não a
chain: `layout/mod.rs:714` (heading), `:812` (equation), `introspect.rs:817`
(heading auto-TOC). O introspect-walk atravessa `Content::Styled` mas **ignora**
os styles (`introspect.rs:1204`, arm `(body, _)`).

**Mecanismo escolhido — β1 (decisão do dono, P339).** Reusar `Content::Styled`
como o nó `StyledElem`-scoped (eco do vanilla `content/mod.rs:744-752`), em vez
de:
- **β3 (inserir na chain já fiada): inviável** — não há chain fiada que carregue
  o custom no ponto de consumo (a de eval morre na fronteira; a de layout nasce
  fresca e só `Content::Styled` a alimenta), e os consumidores nem leem a chain.
- **β2 (walk novo em `rules/realize`): rejeitado** — paridade neutra mas
  superfície maior (módulo novo) e ~6× o custo de passe medido (≈17.6µs/walk vs
  ≈2.8µs/wrap, P339).

**O que a fatia materializa:**

1. **Caminho `Styles`→custom (superfície mínima).** Adiciona-se
   **`Styles::push_custom(key, value)`** — espelho de `StyleChain::push_custom`
   (`style_chain.rs:175`), dobrando em `StyleDelta.custom`. **Sem** nova variante
   no enum `Style`: o canal custom implantado (F-2) é `StyleDelta.custom:
   Vec<(EcoString, Value)>` com chave codificada (`"heading.numbering"`), **não**
   o `Style::Custom { kind, key, value }` que o §3b.1 esboçou como "forma de
   partida". **Esta fatia resolve esse fork do §3b.1 a favor da representação
   `(key, value)` já implantada** (o `Style` permanece o vocabulário das 10
   nativas; o custom entra por `push_custom`, como na chain). **Edição L0
   acoplada**: `style.rs` é governado pelo `style.md` — adicionar
   `Styles::push_custom` à secção "Struct `Styles`" desse L0 (o método espelha
   `StyleChain::push_custom`, sem tocar o enum `Style`).

2. **Emissão wrap nos 3 sítios.** Cada `#set …(numbering:)` passa a **também**
   embrulhar o resto do escopo léxico num `Content::Styled(body, styles)` com
   `styles = Styles::new().push_custom("X.numbering", v)` — **além** do
   `engine.styles.push_custom` atual. O custom passa a viajar na árvore; a
   `StyleChain` reconstruída em layout (`:1251`) disponibiliza-o em `self.chain`
   **no nó**. O wrap espelha a produção do `StyledElem` do vanilla.

**Invariante aditivo (o que NÃO muda).** O **baking** permanece autoritativo:
`eval/markup.rs`/`closures.rs` continuam a assar `numbering_active` de
`engine.styles.custom`; os 3 consumidores continuam a ler o **campo assado**.
**Não** são religados — isso é o **F-5**. `#show` eager intocado → o teste-âncora
`f3s3` permanece divergente (vaza, 2×). `SetPage` (viaja como `Content::SetPage`)
e `#set text` (assa `TextStyle`) ficam **fora** (§3b.7).

**Caminho duplo + disciplina anti-morto (lição S5b/C1 do P338).** A fatia cria
caminho duplo chain↔assado, que **nasce com**: (i) **gatilho de remoção escrito**
= F-5 (o de-bake religa o consumidor à chain e remove o assado); (ii) **teste de
paridade** `chain.custom("X.numbering") ≡ campo assado` enquanto coexistem; (iii)
**zero** representação morta "por segurança".

**Content-preserving (medido, protótipo β1 P339).** O `Content::Styled`
adicionado é **transparente à saída observável**: layout `plain_text` idêntico,
introspect `kind_index`/counters/locations idênticos, matching de `#show` por
**selector** (`eval/rules.rs:88-176`) inalterado. A **única** diferença é
`Content::PartialEq` estrutural (o wrapper é um nó) — **sem dependente em
produção** (`Content ==` só em `source.rs:38`, por id+hash; selectors não usam
igualdade de árvore). **Gatilho de reabertura do wrapper β1** (carona C1, P340):
se igualdade-de-`Content` (PartialEq de árvore) **virar requisito de produção** —
ex.: dedup estrutural, memoização por conteúdo, ou um selector de `#show` por
igualdade de árvore — **reavaliar** o wrapper β1 (o nó visível passa a ter custo
observável). Até lá, fica. Custo de perf: ≈2.8µs/wrap/passe (negligível). A
**caracterização (Estágio T) congela a saída de nível** (layout/`plain_text` +
introspect) como o invariante; o wrapper é detalhe estrutural interno aceite.

**Resolução do gatilho (P345, ADR-0107):** a igualdade-de-`Content` **virou
requisito** (`it.body == [a]` no `==` da linguagem — Achado 2, P342). O gatilho foi
resolvido **pela via da linguagem, não pelo de-bake**: o `==` da linguagem passou a
ser **morfológico** (`Content::morph_canon`, `eval_binary_op`), e trata o transporte
β1 (`Content::Styled` semanticamente vazio, só `custom`) como **render — transparente**
(desce no body). Logo o wrapper β1 **continua a não ser observável** pela igualdade da
linguagem, e **fica** (sem de-bake). O `PartialEq` estrutural do Rust segue intacto
(dois sistemas, ADR-0025). Medido contra o vanilla (P345 N1: `#set numbering` não entra
na igualdade).

**Contrato S* desta fatia.** Materializa **S6** (escopo por subárvore via
`StyledElem`) **só no lado transporte** — o custom confina-se à subárvore
embrulhada; o **consumo** confinado é F-5. **S1–S5, S7** (identidade, guards,
recipes, multi-passe, show-set, leitura de campos) **não** são tocados.

**Dimensão exata (P339).** Sítios de emissão: **3** (`rules.rs:242/266/313`).
Tipo novo: **0** variantes de enum (`Styles::push_custom`, +1 método). Arms de
consumidor novos: **0** (reusa `Content::Styled`). Consumidores religados: **0**
(F-5).

**Assinatura na lente (registrada, não-surpresa).** A fatia opera **dentro** do
megaciclo de 90 — a assinatura **não** é queda de contagem de ciclos (isso é o
corte content→elemento, fora desta fila). Espera-se **delta de aresta** dentro do
megaciclo (ou ~nulo); `content→elements::*` permanece **66**; `elem→elem`
permanece **0**.

### 3a.9 — Fatia F-5a: de-bake de `heading`/`equation` numbering (fonte única)

> **Estatuto.** Realiza o **gatilho (i)** de §3a.8 (o de-bake que religa o
> consumidor à chain e remove o assado), movido pelo **princípio da fonte única
> (atomização)** — decidido pelo dono (P362), não demanda da lente; não se
> re-litiga. **Escopo P364: `heading` + `equation`** (os 2 gates bool, forma
> idêntica). **`figure` = P365** — o gate dela (`figure_numbering: Option<&str>`)
> threada a **assinatura partilhada das natives** (`stdlib/structural.rs`, ~10 fns),
> cascata separável; fatiada por elemento (válvula P364). O ponto 4 (`TextStyle`) é
> o **F-5b** (§3b.7).

**A varredura (P364 Fase A; `file:line`; medido).** Os **3 produtores de produção**
de numbering passam **todos** pelo transporte `Content::Styled` da fatia-1 (estão na
cauda que a fatia-1 embrulha): `heading` `eval/markup.rs:96` (`Content::heading_numbered`),
`equation` `eval/mod.rs:564` (`Content::equation_numbered`), `figure` `closures.rs:79-83`
(lê `custom("figure.numbering")`). → **zero produtores de produção a rotear**: a chain
**já** carrega o custom (P363 provou o gate disponível na chain sob `Styled`). A
representação dupla existe **só** porque o eval **também** baka o campo. Os **~37
fixtures** (`introspect.rs` 6 + `layout/tests.rs` 31) constroem numerado **direto**
(sem o `Styled`) → **[fixture, fora-do-transporte]**.

**O de-bake (heading+equation).**
- **Produção**: `markup.rs:96` e `eval/mod.rs:564` **deixam de bakar** — criam o
  elemento **simples** (`Content::heading`/`equation`); o transporte (fatia-1) carrega
  o gate. `Content::heading_numbered`/`equation_numbered` e os campos
  `HeadingElem.numbering_active`/`EquationElem.numbering_active` (+ payload da equation)
  **removidos** → **fonte única**.
- **Consumidores leem a chain**: layout (`mod.rs:714` `h.numbering_active` →
  `self.chain.custom("heading.numbering")`; equation idem) e introspect (sobre o
  **introspect-chain do P363** — `chain.custom("X.numbering")`). O gatilho (i) e o
  teste de paridade (ii) de §3a.8 ficam **resolvidos** para os 2.
- **Gate ≠ número (P335/DEBT-60).** O de-bake lê **só o gate**; o número
  (`formatted_counter_at`) e o **contador incondicional** (P335,
  `apply_hierarchical_at`) ficam **intactos**. Confirmado: `mod.rs:714` lê o gate
  (`numbering_active`) e **só então** o número.
- **Fixtures [fora-do-transporte]**: roteados ao **transporte** via helper de teste
  (`Content::Styled(elem, push_custom("X.numbering", Bool(true)))`) — **content-preserving**
  (seguem a testar elemento numerado, cobertura preservada). Virar a asserção para
  não-numerado **perde cobertura** → descartado. Cada fixture alterado é **declarado**
  (S5b); nenhuma asserção de comportamento muda (a rede de caracterização +11 passa
  inalterada).

**Dimensão.** Variantes de enum: **0**. Campos removidos: **2** (`numbering_active` de
heading/equation) + construtores `heading_numbered`/`equation_numbered`. Consumidores
religados: **2** (layout, introspect). Fixtures roteados: ~37 (declarados). `figure`
fora (P365).

**A forma de aninhamento (medida, P364).** Em produção a fatia-1 embrulha a **cauda** do
markup → o transporte fica **fora** do `Labelled`: `Styled{ Labelled{ Equation } }`. Isto
importa porque (i) o `compute_labelled` (introspect) inspeciona o **tipo do alvo** do
`Labelled` — com o transporte fora, o alvo é a própria `Equation`; e (ii) o
`custom("X.numbering")` entra na chain **antes** da emissão do payload (walk top). O
construtor de conveniência (`equation_numbered`/`heading_numbered`) embrulha **a folha**
(`Styled{ Equation }`); um fixture que o use como **alvo** de `Labelled`
(`Labelled{ Styled{ Equation } }`) inverte a ordem e quebra o `compute_labelled`. Logo
fixtures rotulados-e-numerados constroem a forma de produção explicitamente (transporte
fora do label) — declarado (S5b). O transporte permanece **transparente** ao descer
(P363), **sem** propagar `label_from_parent` (a forma de produção não exige; manter
mínimo, ADR-0108).

#### Figura (P365) — o 3º numbering + o colapso da assinatura partilhada

> **Estatuto.** Fecha a figura, o 3º caminho duplo. Igual em **princípio** a
> heading/equation (gate de numbering → fonte única na chain), mas com **mais
> superfície**, medida no P364/P365: (i) o gate é o **padrão `Option<String>`**, não
> um bool — porém o **layout só checa `Some`/`None`** (`mod.rs:872` `if let Some(_pattern)`,
> o valor é ignorado), e a chain carrega `Value::Str(padrão)`/`Value::None`; e (ii) o
> padrão era threadado pela **assinatura partilhada `figure_numbering: Option<&str>`**,
> o **último parâmetro do tipo `NativeFn`** (`func.rs:78/97`) — **141 ocorrências em 16
> ficheiros**, morto em todas menos `native_figure` (`figure_image.rs:54`). Decisão do
> dono (P365 Fase A): **de-bake + colapso da assinatura num lote só** (estado final
> limpo, sem parâmetro morto interino).

**O de-bake (figura).**
- **Campo removido:** `FigureElem.numbering` sai (`figure.rs:26` + clones). **Fonte
  única.**
- **Produção para de bakar:** `native_figure` (`figure_image.rs:54`) cria a `FigureElem`
  **sem** o padrão; `closures.rs:79-83` deixa de ler `custom("figure.numbering")` para
  alimentar a assinatura (o transporte da fatia-1 carrega o custom).
- **Consumidores leem a chain:** `layout/mod.rs:870` → o gate vem de
  `self.chain.custom("figure.numbering")` (`Some(Str)` = numerado). O introspect tira o
  `is_counted` do payload **da chain na emissão** (walk top, espelho da equation P364):
  `is_counted = chain.custom("figure.numbering").is_str() && caption.is_some()`. O
  `to_payload` põe placeholder.
- **`morph_canon`:** o arm `Content::Figure(e) if e.numbering.is_some()` (`content.rs:2096`)
  **removido** — subsumido pelo arm `Styled` transparente (mecanismo único, P345 N1).
- **Gate ≠ número:** o de-bake lê **só o gate** (padrão presente/ausente); o **número**
  (`figure_progress` + `figure_number_at_index`, `mod.rs:874-886`) e o contador ficam
  **intactos**.

**O colapso da assinatura partilhada (mesmo lote).** Como `figure_numbering` fica **morto
em todas** as natives (incluindo `native_figure` após o de-bake), o parâmetro **some** do
tipo `NativeFn` (`func.rs:78/97`), de todas as ~141 assinaturas, do dispatch
(`closures.rs:83`) e dos sítios de chamada diretos (testes `, None`). **Zero
comportamento** — é hygiene mecânica que a fonte única torna possível (disciplina
anti-morto: o parâmetro perde o único consumidor → sai). **Fixtures** que constroem
`FigureElem { numbering: … }` direto (`figure.rs` tests, `layout/tests.rs`) e os testes de
`to_payload`/`is_counted` **viram** para a forma de transporte / leitura-da-chain —
declarados (S5b).

**Dimensão (figura).** Campo removido: **1** (`FigureElem.numbering`). Param colapsado:
**1** (`figure_numbering`, ~141 ocorrências, 16 ficheiros). Consumidores religados: **2**
(layout, introspect). Arm `morph_canon` removido: **1**. Tipo `NativeFn` alterado: **1**
(menos um parâmetro).

**Achados de execução (P365, medidos).**
- **`compute_labelled` (introspect) não lê mais campo** — passa a confiar **só no
  contador** `figure:{kind}` (`flat_counter_at`, `n>0`), que só avança quando `is_counted`
  (padrão-da-chain && caption) na emissão. Espelho do arm `Equation` (que já confiava no
  contador). O gate de numbering chega via o contador, não via leitura de campo.
- **Aninhamento label×transporte (igual à equation, P364):** em produção o transporte fica
  **fora** do `Labelled` (`Styled{ Labelled{ Figure } }`); o construtor `Content::figure(..,
  Some)` embrulha a folha. Fixtures que faziam `Labelled{ figure_numerada }` invertiam a
  ordem → o helper de teste `labelled_prod` **levanta** o transporte transparente para fora,
  reproduzindo a forma de produção (declarado, S5b). **Sem** mudança em código de produção
  (mínimo, ADR-0108).
- **`is_counted` no payload** = placeholder `caption.is_some()` (de `to_payload`) **ANDado**
  com `chain.custom("figure.numbering").is_str()` no walk top (espelho da equation P364).

### 3a.10 — Fatia F-5b: de-bake do `TextStyle` (`Content::Text`) — fecha o item (1)

> **Estatuto.** Fecha o **4º caminho duplo** (item (1) da auditoria P362): o `TextStyle`
> assado em `Content::Text(EcoString, TextStyle)` sai; o consumidor (layout) lê o estilo **só
> da chain** (`self.style`). **Fonte única.** Decidido pelo dono (P362); escopo P366 = tudo
> num lote (transporte + remoção).

**A correção de enquadramento (P366 Fase A, ADR-0108).** O arrasto **não** é "o bold do
heading" (como o pré-passo supunha). Medido:
- **heading bold** — o arm Heading do layout (`mod.rs:704`) já **seta `self.style.bold=true`**
  (+ `size`) antes de descer no corpo → **já na chain**. Não é arrasto.
- **strong/emph** — `Content::strong/emph` = `Content::Styled([Bold/Italic])` → chain. Não é
  arrasto.
- **`#set text`** (size, fill, font, weight, tracking, leading, lang, bold, italic) — empurrado
  a `engine.styles` (`rules.rs:497/638`) e **assado** em `Content::Text` (`eval/mod.rs:344`),
  **sem** wrapper `Content::Styled`: a fatia-1 (`eval/mod.rs:413-419`) só embrulha quando muda
  um `NUM_KEYS` de numbering. **Este é o arrasto, e é o `#set text` tipado inteiro.**

**O mecanismo morph-safe: um canal de render separado dos campos tipados (corrigido pós-
obstáculo).** A raiz do obstáculo (bloco ⚠ abaixo): `#set text(bold:)` põe `delta.bold` — o
**mesmo campo tipado** que `*bold*`/strong (`content.rs:1040`); `morph_canon` não os distingue.
∴ o estilo de **render** do `#set text` **não pode** viajar nos campos tipados de um `Styled`.
Tem de viajar num **canal de render separado**, que `is_semantically_empty` **ignora** (→
`morph_canon` desce, transparente — como já ignora o `custom`). Separação conceptual limpa
(resolve a tensão ADR-0107 × ADR-0038): **campos tipados (`Style::Bold/Italic`) = morfologia de
markup** (strong/emph, mantidos por `morph_canon`); **canal de render = `#set text`**
(transparente).
- **Forma do canal** (a decidir no redesenho): (i) um slot `render: Option<…>` na `StyleDelta`,
  ignorado por `is_semantically_empty`; ou (ii) chaves `custom` por-prop (`"text.size"`, …) —
  mas o `Value` fechado não exprime `Color`/`Length`/`FontList` cômodo, logo (i) é a
  recomendação medida.
- **`#set text`** passa a popular o **canal de render** (não `delta.bold/italic/size/…`
  tipados). O transporte (fatia-1 generalizada) embrulha a cauda num `Content::Styled` com o
  canal de render diff. `morph_canon` desce (ignora o canal) → `#set text` **continua
  morfologicamente transparente** (content-preserving do `==`).
- **Layout** lê o estilo de render do **canal** (merge em `self.style`), não do `node_style`
  assado.

**O de-bake (após o canal de render levar o estilo à chain).**
- `Content::Text(EcoString, TextStyle)` → **`Content::Text(EcoString)`** (campo removido).
- `Content::text()` e ~90 sítios cascateiam (`Content::Text(s, style)`/`(s, _)` →
  `Content::Text(s)`).
- **Layout `mod.rs:599-632`**: o merge passa a ler o render-style do canal da chain; o
  `node_style` desaparece.
- `map_text`/`morph_canon` deixam de tocar estilo no `Text` (não o tem) — morfologia
  **intacta por construção** (o canal de render é transparente).

**Ordem (medida, não opcional).** Canal de render + `#set text` a popular o canal **primeiro**
(aditivo; o `TextStyle` assado ainda autoritativo) → de-bake **depois**. Senão regride.

**Intactos.** `morph_canon`/`==` (canal de render transparente **por construção**), caso 4,
α/caso 2, flag P350c, Marco G, os 3 numbering (P364/P365). **Content-preserving** (rede +11).

**Magnitude (corrigida).** Maior que o original: o canal de render é um **conceito de estilo
novo** (slot na `StyleDelta` + `is_semantically_empty` a ignorá-lo + `#set text` a popular o
canal + transporte + layout a ler do canal + remoção do `TextStyle` + cascata ~90 sítios).
Candidato a **fatiar** (canal+transporte / remoção). O dono reaprova o desenho antes de código.

> **⚠ OBSTÁCULO MEDIDO (P366 Estágio 1 — parar e reportar).** O transporte por **diff de
> delta tipado** acima **viola o limite duro "não tocar a morfologia"**, medido na fonte:
> - `morph_canon` (`content.rs`) mantém um `Content::Styled` **a menos que** seja
>   `is_semantically_empty()`; e `is_semantically_empty` (`style.rs:207`) conta **todos os
>   campos tipados** (`bold/italic/size/fill/weight/…`), ignorando só o `custom`.
> - Logo um `Content::Styled` carregando o `#set text` **tipado** (ex.: `size`) é
>   **semanticamente não-vazio** → `morph_canon` **mantém-no** → o `#set text` passa a ser
>   **morfologicamente significativo**. Hoje o `#set text` é **assado** no `Text` e
>   `morph_canon` o **descarta** (`Content::Text(s,_) => Text(s, default)`) — i.e. é
>   morfologicamente **transparente** (render, ADR-0107). O transporte tipado **inverteria**
>   isso (`[#set text(size:20) foo] == [foo]` passaria de **true** a **false**).
> - `#set text` usa os **mesmos** campos tipados (`delta.bold/italic/size/…`, `rules.rs:511+`)
>   que `*bold*`/`_italic_` (strong/emph), que são **morfologia** (ADR-0038, mantidos por
>   `morph_canon`). Estruturalmente **indistinguíveis** — `morph_canon` não consegue separar
>   render de morfologia se ambos vivem nos campos tipados de um `Styled`.
>
> **Direção corrigida (morph-safe, maior):** o transporte do estilo de **render** do `#set
> text` tem de ir pelo **canal `custom`** (que `is_semantically_empty` ignora →
> `morph_canon` **desce**, transparente), como o numbering já faz — **não** pelos campos
> tipados. Isso exige o **layout ler o estilo de render do `custom`** (não de `self.style`
> tipado), um mecanismo **diferente e maior** do que o aprovado na Trava. **É re-escopo.**
> Nenhum `.rs` foi escrito. Decisão do dono: redesenhar o P366 com o transporte por `custom`,
> ou fatiar/adiar. Ver relatório `typst-passo-366-relatorio.md`.

> **⚖ MEDIÇÃO VANILLA (P366, decisiva — refuta a premissa de transparência).** O dono pediu
> medir o **oráculo** (vanilla 0.14.2) antes de fixar o desenho. Medido em `lab/`:
> - `StyledElem::PartialEq` (`content/mod.rs:763`): `fn eq = self.child == other.child` — **o
>   wrapper compara só o child; IGNORA os styles**.
> - `#set text` no markup (`typst-eval/markup.rs:41`): `tail.styled_with_map(styles)` →
>   embrulha a cauda num **`StyledElem`**.
> - strong/emph são elementos **distintos** (`model/strong.rs`/`emph.rs`: `StrongElem`/
>   `EmphElem`), **não** `StyledElem`; `Packed::eq` compara id de elemento (tipos diferentes →
>   `false`).
> - **∴ vanilla:** `#set text(…) X` (StyledElem) **≠** `X` (TextElem) — **morfologicamente
>   significativo pela PRESENÇA do wrapper**; mas `#set text(a) X == #set text(b) X` (valores
>   ignorados). E `*bold* X` (StrongElem) ≠ `#set text X` (StyledElem) ≠ `X`.
>
> **Consequência (inverte a conclusão):** o comportamento **atual do cristalino**
> (`#set text X == X`, via assar+descartar) **DIVERGE do vanilla** (que dá `≠`). Logo:
> - O **canal de render transparente** (a "direção corrigida" acima) **preservaria a
>   divergência** → **errado** por ADR-0107 (paridade é com a língua/vanilla).
> - O de-bake **vanilla-faithful** faz `#set text X ≠ X` — i.e. `#set text` → wrapper
>   **morfologicamente significativo pela presença** (vindica a direção do transporte tipado
>   original), **com** `morph_canon`/`==` a comparar **só o child** do `Styled` (ignorar os
>   **valores** de estilo, espelho do `StyledElem::eq`). Hoje o `==` do cristalino sobre
>   `Content::Styled` compara os styles (derive) → precisa de normalização no `morph_canon`.
> - **Divergência pré-existente (fora de escopo):** o cristalino colapsou strong/emph em
>   `Content::Styled` (Passo 101), perdendo a distinção `StrongElem` vs `StyledElem` do vanilla
>   — isso já existe e não é deste de-bake.
>
> **Três opções para o dono** (o desenho mudou de novo com a medição):
> **(a) vanilla-faithful** — `#set text` → wrapper significativo-por-presença; `morph_canon`
> normaliza os valores do `Styled` (compara child). Corrige a divergência; **muda** o `==`
> atual do `#set text` (testes que o codifiquem viram). **(b) preservar o atual** (transparente)
> — adia a fidelidade ao vanilla; o de-bake fica bloqueado (não há transporte morph-transparente
> sem o canal custom, que é grande e *também* não-vanilla). **(c) adiar o F-5b** — o item (1)
> fica 3/4; seguir item (2)/(3). Nenhum `.rs` escrito.

> **⛔ BLOQUEIO ARQUITETURAL (P366, medido — fecha a questão; o dono escolheu (a) vanilla-faithful,
> mas a medição da normalização revela dois limites duros):**
> 1. **A normalização vanilla-faithful toca o α-fixpoint.** `morph_canon` é usado em **dois**
>    sítios: `operators.rs:79` (o `==`/`!=` da linguagem) **e `rules.rs:229`** (o **α-fixpoint
>    do `#show`**, P348: `out.morph_canon() == work.morph_canon()` decide a terminação). Mudar o
>    `morph_canon` para normalizar os valores do `Styled` **muda o α** — **limite duro** ("não
>    tocar o α / `morph_canon`").
> 2. **Distinguir render-Styled de strong/emph-Styled exige reverter o colapso do Passo 101.**
>    Vanilla: `StyledElem` (settext, valores ignorados) ≠ `StrongElem`/`EmphElem` (distintos). O
>    cristalino colapsou os três em `Content::Styled`; `#set text(bold:)` põe `delta.bold` — o
>    **mesmo** campo que `*bold*`. `morph_canon` **não os separa**. Normalizar **todos** os
>    valores equipararia `*bold* X == *italic* X` (vanilla: ≠); separar exige **marcador/variante**
>    novo (refinar o P101) — mudança de **modelo**, não de-bake.
>
> **∴ F-5b está bloqueado** no modelo de morfologia (colapso P101 + α-fixpoint) — ambos limites
> duros / arquitetura grande. **Recomendação medida: ADIAR.** O item (1) fica **3/4** (os 3
> numbering fechados; o `TextStyle` permanece assado); a extensibilidade (item 3) **não depende**
> deste de-bake. O F-5b volta como **lote arquitetural dedicado** (modelo strong/emph/styled + α),
> não como de-bake. Nenhum `.rs` escrito.

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
>
> **Emenda do termo + escalada (carona C2, fecho P337):**
> 1. **"Vanilla medido" redefinido** — para um **elemento custom** (não nativo),
>    "paridade contra o vanilla medido" significa **semântica extraída da fonte do
>    `lab/typst-original/` com `file:line`**, não execução. Rodar é inviável: o
>    vanilla não tem binário pronto neste repo nem um modo trivial de **definir**
>    um elemento custom (a sua extensão é o macro `#[elem]` em Rust, compile-time).
>    O que o spike-2 fez foi **ler a fonte** (`typst-realize`, `content/mod.rs`),
>    nunca correr — e é essa a medida de referência válida para o caminho dinâmico.
> 2. **Gatilho de 2º nível (escalada para execução)** — a redefinição (1) vale
>    enquanto a leitura-da-fonte for **inequívoca**. **Se** um teste de paridade
>    baseado em leitura conflitar com comportamento observado, **ou** a fonte for
>    ambígua num caso concreto, **então** escala-se: compila-se o `typst-cli` do
>    `lab/` e mede-se o **mecanismo compartilhado em disputa com os NATIVOS**
>    (`#show heading: …`, `#set`, escopo de bloco) — que o vanilla **sabe** exibir
>    sem elemento custom. A medição com nativos resolve a ambiguidade sem precisar
>    de um elemento custom no vanilla. Até esse conflito surgir, a leitura basta.

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

- **`Styled`** (`Styled(Box, Styles)`) — **✅ F-4 (P338).** Correção do recon
  P337: não havia "2ª StyleChain" no Layouter (há **uma** chain por fase — eval
  `engine.styles` + layout `self.chain`); o que havia era **dualidade de backing**
  (`Styles(Vec<Style>)` em `Styled` ↔ `StyleDelta` na chain, com `push_styles` a
  converter). F-4 colapsa pela **direção (ii)** (decisão do dono, P338 Fase A):
  `Styles` vira **fachada sobre `StyleDelta`** (backing único), o enum `Style`
  fica como vocabulário-construtor (`from_iter` dobra na borda, 1×), e o canal
  `custom` (F-2) fica **disponível no `Styled`** — a porta para `#set` viajar
  confinado que F-realização/F-5 usarão. **B2** (`is_empty` sem arm Styled)
  fechado no mesmo lote (S1). Detalhe em `style.md`.
- **De-bake de `#set text`**: hoje `#set text` **assa** `TextStyle` em
  `Content::Text` (chain descartada, 1a). O de-bake (a chain deixar de assar)
  é **lote dedicado** — gatilho: depois do canal `Set*` provar a chain léxica.
- **3 folhas provisórias** (`Text`/`MathText`/`MathIdent`, DEBT-58): os campos que
  o vanilla lhes daria são estilo (StyleChain) → resolvem-se **quando a chain
  léxica existir**. Gatilho: o lote do estilo.
  - **F-6 medido (P367) — não há de-bake de fonte única a fazer.** Medição
    (`content.rs:122/167/170`): **`MathText(EcoString)`/`MathIdent(EcoString)` são
    folhas NUAS** — só a string, **sem campo de estilo assado**; o estilo vem do
    **contexto de math** (o wrapper `MathStyled` transforma o glyph,
    `math/layout/mod.rs:711-723`), não de um campo da folha. ∴ **sem caminho duplo**
    → nada a remover, já são fonte única por construção. O **`Content::Text`** é a
    única folha com estilo assado (`TextStyle`) — mas isso é o **F-5b** (adiado,
    DEBT-61: bloqueio α-fixpoint + colapso P101), **fora do F-6** pela regra de
    fronteira do passo. Dar às folhas de math os **campos completos do vanilla** é
    **adição de feature** (sem divergência observável — auditoria P362), **não**
    de-bake de fonte única. **F-6 fecha sem código** (medido). Próximo: item (3),
    `#set` de user-props.
  - **Confirmação vanilla (P367, oráculo `lab/`):** o math text do vanilla é um
    `TextElem` cujos campos de estilo (`font`/`size`/`fill`/`weight`/…) são **`#[ghost]`**
    (`text/mod.rs:95+`) — **resolvidos da `StyleChain`, não guardados no elemento**; o
    layout de math (`typst-layout/math/text.rs:18`) faz `TextElem::packed(text)` (só a
    string) e estiliza **pela chain**. ∴ a folha de math do vanilla = **string nua +
    estilo-da-chain**, **idêntica** ao `MathText`/`MathIdent` nu do cristalino. **Sem
    divergência observável** → já são vanilla-faithful. (E o `TextStyle` **assado** do
    `Content::Text` é justamente a divergência do modelo `#[ghost]` do vanilla = o F-5b
    adiado, reforçando o DEBT-61.) **F-6 confirmado: nada a fazer.**

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
