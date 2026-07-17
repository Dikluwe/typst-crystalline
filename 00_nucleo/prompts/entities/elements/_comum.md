# Prompt L0 — `entities/elements` — trait `Element` (modelo D, lote piloto)
Hash do Código: c8fc4ad6

**Camada**: L1 · **Módulo**: `01_core/src/entities/elements/`
**Decisão de origem**: ADR-0105 (modelo D incremental; F como destino) +
ADR-0104 (atomicidade). Diagnóstico: `diagnostico-modelo-elemento-passo-313.md`.
**Tipo**: refatoração de **comportamento idêntico** — a lógica de cada variante
muda de morada (dos 6 matches gigantes de `content.rs` para um módulo por
elemento), a semântica não muda.

Este `_comum.md` define **o trait, as regras partilhadas e o estado misto**. Um
prompt fino por elemento do lote piloto cita-o: `divider.md`, `heading.md`,
`math_styled.md`.

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

O layout precisa de `&mut Layouter` (contexto de `engine/layout`). Pô-lo no trait
`Element` (em `entities/`) inverteria a topologia (entities não depende de
rules). **Decisão**: o trait é **só dados**; o layout de cada elemento **fica
onde está** em `rules/` — `engine/layout/mod.rs` (Divider, Heading) e
`rules/math/layout/mod.rs` (MathStyled). No piloto, o braço de layout apenas
muda de **destructuring** (`Content::Nome(e) => … e.campo …` em vez de campos
inline) — **mesma lógica, mesma morada**. Mover o layout para
`engine/layout/elements/` é um **segundo eixo de atomicidade**, separável e
**fora do piloto** (minimiza o toque agora). Como a spec de comportamento de
layout não muda, **`engine/layout.md` NÃO é editado nem fatiado neste passo**
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
