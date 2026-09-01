# Prompt L0 — `entities/elements` — trait `Element` (modelo D, lote piloto)
Hash do Código: e962d8b0

**Camada**: L1 · **Módulo**: `01_core/src/entities/elements/`
**Decisão de origem**: ADR-0105 (modelo D incremental; F como destino) +
ADR-0104 (atomicidade). Diagnóstico: `diagnostico-modelo-elemento-passo-313.md`.
**Tipo**: refatoração de **comportamento idêntico** — a lógica de cada variante
muda de morada (dos 6 matches gigantes de `content.rs` para um módulo por
elemento), a semântica não muda.

Este `_comum.md` define **o trait, as regras partilhadas, o glossário e o estado
misto**. Cada prompt fino de elemento cita-o na linha de cabeçalho.

## P1291 — novos módulos math no índice (RASCUNHO PARA SELO)

### Medição anterior à decisão

O índice produtivo `01_core/src/entities/elements/mod.rs` declara
`math_cancel` e `math_matrix`, mas não declara owners para underline matemático
ou vetor. O vanilla ratificado mantém `UnderlineElem` math distinto do
`UnderlineElem` textual e `VecElem` distinto de `MatrixElem`.

### Decisão proposta

Depois do selo ADR-0127, o índice declara `math_underline` e `math_vec`, cada
qual com um único consumer e prompt proprietário:
`entities/elements/math_underline.md` e `entities/elements/math_vec.md`.
`math_cancel` continua sob `entities/elements/math_cancel.md`. Não haverá
re-export que fusione identidades, módulo genérico compartilhado nem owner
duplicado. As regras comuns do trait `Element` continuam neste arquivo; os
payloads e comportamento específico ficam nos prompts finos (ADR-0129).

---

## A.0 — Glossário (definições únicas; os prompts finos referem-se a esta secção)

Dois termos aparecem em dezenas de prompts de elemento sem definição local. Ficam
definidos aqui **uma vez**, por medição, e é a esta secção que os prompts finos
remetem. Escrever a frase solta sem esta referência é ambiguidade (Bloco 2 de
`auditar-spec.md`).

### A.0.1 — `Não-locatável`

Um elemento é **não-locatável** quando cumpre, ao mesmo tempo:

1. está no grupo `=> false` do `match` **exaustivo** de `is_locatable`
   (`01_core/src/compiler/introspect/locatable.rs:217` — o match não tem `_ => false`,
   logo acrescentar variante ao `Content` força revisão pelo compilador);
2. `extract_payload(c)` devolve `None` — é a invariante declarada no próprio módulo
   (`locatable.rs:10-11`): `is_locatable(c) == extract_payload(c).is_some()` para todo
   `c`;
3. no trait `Element`, mantém os **defaults** de `element_kind()` e `to_payload()`
   (ambos `None`) — ou seja, não sobrepõe nenhum dos dois.

**O que isso implica, mecanicamente** (medido em
`01_core/src/compiler/layout/mod.rs:993-994`, `advance_locator_if_locatable`): um
não-locatável **não consome slot do `Locator`**, **não recebe `Location`** (o
`current_location` não avança por causa dele) e **não entra em
`runtime.positions`**. Consequência de linguagem: não é alcançável por `query`, nem
serve de âncora a `counter`/`state` — não porque esteja "fora de escopo", mas porque
não tem identidade posicional nenhuma.

**Como confirmar num elemento concreto**: procurar a variante no match de
`locatable.rs`; se estiver na cadeia `|` que termina em `=> false`, é não-locatável.
Não inferir do nome nem do propósito.

### A.0.2 — `braço do hub` (antes escrito "braço actual")

O **hub** é o `enum Content` (`01_core/src/entities/content.rs`) com os seus `match`
exaustivos sobre `self`. O **braço do hub** de um elemento é a entrada desse `match`
para a sua variante — o código que existia (ou existe) inline no hub antes de a
lógica se mudar para o ficheiro do elemento.

Os matches que constituem o hub, medidos:

| Método | Local | Forma |
|---|---|---|
| `elem_name` | `content.rs:1258` | `match self` |
| `is_empty` | `content.rs:2566` | `match self` |
| `plain_text` | `content.rs:2671` | `match self` |
| `get_field` | `content.rs:3015` | `match (self, field)` |
| `map_content` | `content.rs:3058` | `match self` (sobre o valor processado) |
| `map_text` | `content.rs:3308` | `match self` |

A extracção de payload — o sexto eixo, e o único **fora** de `content.rs` — vive em
`01_core/src/compiler/introspect/extract_payload.rs` e é absorvida pelo trait via
`element_kind()`/`to_payload()` (A.1.3).

**"Comportamento idêntico ao braço do hub"** significa, então, uma afirmação
verificável e não uma fórmula de estilo: para a mesma entrada, o método do
`NomeElem` produz o mesmo resultado que o braço inline produzia nesses matches —
incluindo mensagens de erro e ordem de avaliação. É a mesma barra dos Critérios de
verificação comuns no fim deste ficheiro: a suíte existente passa **sem alterar
nenhum teste**.

---

## A.1 — Decisões de desenho (gravadas; não herdadas de suposição)

### A.1.1 — Assinatura do trait `Element`

O trait vive em `entities/elements/mod.rs` e depende **só** de tipos de
`entities/` (nunca de `rules/` — o layout NÃO entra no trait; ver A.1.2). Cada
variante migrada do `Content` passa a `Nome(Arc<nome::Nome>)`, e o struct
`nome::NomeElem` implementa:

```rust
pub trait Element: Clone + PartialEq + std::hash::Hash + std::fmt::Debug {
    /// Texto plano (para verificação/`plain_text` do Content).
    fn plain_text(&self) -> String;

    /// Vazio estrutural. Default `false` (a maioria dos elementos nunca é vazia).
    fn is_empty(&self) -> bool { false }

    /// Reconstrói-se com filhos transformados. Devolve `Content` (re-embrulha
    /// a própria variante). Genérico sobre F — despacho estático (o trait NÃO
    /// é object-safe, e não precisa de ser: o `match Content` conhece o tipo).
    fn map_content<F>(&self, f: &mut F) -> SourceResult<Content>
    where F: FnMut(&Content) -> SourceResult<Option<Content>>;

    /// Idem para transformação de texto terminal.
    fn map_text<F>(&self, f: &mut F) -> Content
    where F: FnMut(&str) -> String;

    /// Acesso a campo nomeado (show rules). Default `None`.
    fn get_field(&self, _field: &str) -> Option<Value> { None }

    /// Absorção do locatável (ver A.1.3). Default `None` (não-locatável).
    fn element_kind(&self) -> Option<ElementKind> { None }
    fn to_payload(&self) -> Option<ElementPayload> { None }

    /// Id estável do kind para o match de seletor `#show` (S1; F-2+).
    /// Default `""` — ver A.1.5. Adicionado no lote F-1 (P334).
    fn dyn_kind_name(&self) -> &'static str { "" }
}
```

**Métodos que viram trait** (dos 6 matches do hub): `plain_text`, `is_empty`,
`map_content`, `map_text`, `get_field`. **`eq`/`hash` NÃO são métodos do trait**
— ver A.1.1.b.

#### A.1.1.b — Desenho explícito de `eq`/`hash` (por causa do `Arc`)

- **`PartialEq`**: cada `NomeElem` faz `#[derive(PartialEq)]` (igualdade
  **estrutural** dos campos). O `Content` despacha:
  `(Content::Nome(a), Content::Nome(b)) => a == b`. Como `Arc<T>: PartialEq`
  compara o **valor apontado** (não o ponteiro) quando `T: PartialEq`, a
  igualdade é estrutural — paridade exacta com o comportamento atual.
- **`Hash` — interação com `content_hash` (registrada)**: `Content: Hash`
  delega a `content_hash::hash_content`, que hoje serializa via
  `format!("{:?}", content)` (Debug estrutural). Ao embrulhar a variante em
  `Arc<NomeElem>`, **o texto de Debug muda** (`Nome(NomeElem { .. })` em vez de
  `Nome { .. }`), logo os **valores absolutos** de hash dessas 3 variantes
  mudam. A **propriedade relacional preserva-se** (mesmos campos → mesmo Debug
  → mesmo hash; campos diferentes → hash diferente), porque `NomeElem` faz
  `#[derive(Debug)]` estrutural. **Trava de Fase B**: confirmar que nenhum
  teste fixa um valor **absoluto** de hash (os testes existentes verificam a
  relação, ex.: `payload_diferente_produz_hash_diferente`); a mudança de valor
  absoluto é interna (dedup de introspecção), não observável no output PDF.

### A.1.2 — Morada do layout (decisão: fica em `rules/`, fora do trait)

O layout precisa de `&mut Layouter` (contexto de `compiler/layout`). Pô-lo no trait
`Element` (em `entities/`) inverteria a topologia (entities não depende de
rules). **Decisão**: o trait é **só dados**; o layout de cada elemento **fica
onde está** em `rules/` — `compiler/layout/mod.rs` (Divider, Heading) e
`rules/math/layout/mod.rs` (MathStyled). No piloto, o braço de layout apenas
muda de **destructuring** (`Content::Nome(e) => … e.campo …` em vez de campos
inline) — **mesma lógica, mesma morada**. Mover o layout para
`compiler/layout/elements/` é um **segundo eixo de atomicidade**, separável e
**fora do piloto** (minimiza o toque agora). Como a spec de comportamento de
layout não muda, **`compiler/layout.md` NÃO é editado nem fatiado neste passo**
(o imposto não morde — ver ADR-0104; só se fatia quando morde).

### A.1.3 — Absorção do locatável (caso Heading)

O que `ElementKind`/`ElementPayload` davam passa a métodos do trait:
`element_kind()` e `to_payload()`. No piloto, **só o braço do Heading** migra:
- `extract_payload.rs` arm `Content::Heading {..}` → `Content::Heading(h) => h.to_payload()`.
- Os **enums `ElementKind`/`ElementPayload` permanecem** para as outras
  variantes locatáveis (Figure/Cite/Equation/…). **Estado misto esperado e
  permitido** (ADR-0105 migração incremental): os enums esvaziam lote a lote;
  o destino final é o trait fornecer tudo e os enums desaparecerem.

### A.1.5 — `dyn_kind_name` (fronteira E1; lote F-1, P334)

Método defaultado `fn dyn_kind_name(&self) -> &'static str { "" }`. Adicionado ao
trait `Element` no lote F-1 (ADR-0106; L0 `entities/f_fronteira_e1.md` §3a). É o
**id estável de kind** (S1 do spike-2) para o match de seletor `#show` (F-2+). O
default `""` mantém **os 65 nativos intocados** (despachados estaticamente pelo
`match Content`, nunca usam o id). Os elementos **dinâmicos** (`Content::Dynamic`,
utilizador) **sobrepõem** com o seu nome. A versão object-safe `DynElement`
(`entities/elements/dynamic.rs`, L0 próprio `f_fronteira_e1.md`) expõe o mesmo
método; o blanket `impl<T: Element + 'static> DynElement for T` busca-o de `Element`.

### A.1.4 — Forma compatível com F (trava ADR-0105)

Cada `NomeElem` agrupa **campos + defaults juntos** (não espalhados), de modo
que acrescentar no futuro `fn descriptor() -> &'static ElementDescriptor` seja
natural (os campos do struct são os "props" do descritor de F). Não se
implementa `descriptor()` agora — só se mantém a forma compatível.

---

## Estado misto (durante a migração)

O `enum Content` tolera, durante os lotes, **mistura** de variantes
`Nome(Arc<NomeElem>)` (migradas) e `Nome { … }` (por migrar). Os 6 matches do
hub despacham as migradas para o trait (`Content::Nome(e) => e.metodo(…)`) e
mantêm o braço inline das não-migradas. Isto é o esperado até os ~74 restantes
migrarem. O hub encolhe lote a lote.

## Critérios de verificação (comuns)

- Comportamento idêntico: a suíte existente inteira passa **sem alterar nenhum
  teste** (alterar teste para passar = mudança de comportamento = bug).
- `Content::Nome(a) == Content::Nome(b)` ⟺ campos iguais (estrutural via Arc).
- `plain_text`/`map_content`/`map_text`/`get_field` produzem o mesmo que os
  braços inline anteriores.
