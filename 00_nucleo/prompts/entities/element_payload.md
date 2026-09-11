# Prompt L0 — `entities/element_payload`
Hash do Código: d3882e3e

**Extensão P1339:** `APPROVED_ADR0127_PENDING_INTEGRATION_GATES` para NativeElement.
A aprovação específica do dono está registrada em
`diagnosticos/p1339-where-payload-approval.json`; os gates de integração,
contrato, selo e RED do P1339 continuam obrigatórios antes de código.

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/element_payload.rs`
**Criado em**: 2026-04-30 (P161 sub-passo .7)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`ElementPayload` é a forma fechada e tipada dos dados específicos de cada elemento indexado pela introspecção. Uma variante por kind (`Heading`, `Figure`, `Citation`), com os campos exactos que o motor de introspecção precisa para cada um.

P161 sub-passo .1 confirmou os campos das variantes correspondentes em `Content`:

| Variant Content | Campos relevantes confirmados em content.rs |
|-----------------|---------------------------------------------|
| `Content::Heading` | `level: u8`, `body: Box<Content>` |
| `Content::Figure` | `body, caption: Option<Box<Content>>, kind: Option<String>, numbering: Option<String>` |
| `Content::Cite` | `key: String, supplement: Option<Box<Content>>, form: Option<CitationForm>` |

Adicionalmente:
- `body_hash` em `Heading` é populado pela função `hash_content` (em `entities/content_hash.rs`, P162 sub-passo .B) chamada por `extract_payload` (em `rules/introspect/extract_payload.rs`, P162 sub-passo .D). Pendência placeholder de P161 resolvida em P162.

---

## Restrições Estruturais

- Camada **L1**: enum puro.
- Sem referências para `Content` directamente (clones-pesados); apenas hashes (`u128`) e cópias leves (`u8`, `String`, `CounterUpdate`, `Option<…>`).
- `Clone` derivado para passar entre walk → tag → registry sem `Arc`.

---

## Interface pública

```rust
use crate::entities::counter_update::CounterUpdate;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementPayload {
    Heading {
        /// Nível do heading (1..=6 após clamp). Paridade com `Content::Heading.level`.
        depth: u8,

        /// Hash determinístico do `body` do heading (Box<Content>).
        /// Populado por `hash_content(body)` em `extract_payload`
        /// (P162 .D). Identifica univocamente o conteúdo do body
        /// para detecção de mudanças cross-iteration.
        body_hash: u128,

        /// Update implícito do contador "heading" associado a este nó.
        /// Tipicamente `CounterUpdate::Step` (avança ao nível `depth`).
        counter_update: CounterUpdate,
    },

    Figure {
        /// Discriminador do tipo de figura — `"image"` / `"table"` / `"raw"` / etc.
        /// Em paridade com `Content::Figure.kind: Option<String>`.
        /// `None` ↔ Auto (resolver no consumer via `kind.as_deref().unwrap_or("image")`).
        kind: Option<String>,

        /// Update implícito do contador da figura (kind-discriminado).
        counter_update: CounterUpdate,

        /// **P168 (M5 sub-passo 2)**: `true` se figura conta para
        /// numeração — predicado `figure.numbering.is_some() && figure.caption.is_some()`.
        /// Permite a `from_tags` indexar apenas figuras numeradas para
        /// `figure_label_numbers`, em paridade com walk arm `Content::Labelled`
        /// que aplica o mesmo filtro no `CounterStateLegacy.figure_label_numbers`.
        is_counted: bool,
    },

    Citation {
        /// Chave da citação. Paridade com `Content::Cite.key`.
        key: String,
    },

    /// **P169 (M9 sub-passo 1)** — payload de `metadata(value)`.
    /// `Box<Value>` para evitar cycle de tamanhos (Value contém Content
    /// que poderia conter Metadata via `Content::Metadata`).
    /// Consumer: `MetadataStore` populado por `from_tags`.
    Metadata {
        value: Box<Value>,
    },

    /// **P171 (M9 sub-passo 3)** — payload de `state(key, init)`.
    /// Init value para state runtime; populado em `StateRegistry::init`.
    State {
        key:  String,
        init: Box<Value>,
    },

    /// **P171 (M9 sub-passo 3)** — payload de `state.update(key, value)`.
    /// Aplicado em `StateRegistry::apply_update`. Apenas Set variant
    /// em P171; Func adiada.
    StateUpdate {
        key:    String,
        update: StateUpdate,
    },

    /// **P178** — payload de `Content::Outline`. Variant unit em P178
    /// (Opção α): `Content::Outline` é unit, e `query("outline")`
    /// minimal só precisa contar locations. Refino futuro pode capturar
    /// `depth: Option<usize>` e `title: hash` para queries mais ricas.
    Outline,

    /// **P181C** — payload de `Content::Bibliography`. Carrega
    /// entries completos (decisão P181A cláusula 2 — captura full por
    /// simetria com walk arm actual `state.bib_entries.extend(...)`);
    /// `from_tags` arm Bibliography (P181E pendente) extrai `entries`
    /// e popula `BibStore` via `add_bibliography(entries) +
    /// assign_number(key, n)` em loop. `BibEntry` deriva `Debug` —
    /// `impl Hash` manual de `ElementPayload` via `format!("{:?}", ...)`
    /// cobre a variant sem alteração de código.
    Bibliography {
        entries: Vec<BibEntry>,
    },

    /// **P186B** — payload de `Content::Equation`. Forma paralela a
    /// `Figure` (P184B) com `block` (display vs inline) +
    /// `counter_update` (sempre `Step` enquanto não houver equation
    /// set rule). `from_tags` arm Equation (P186E) popula
    /// `CounterRegistry` sob chave `"equation"` quando
    /// `block && state.value_at("numbering_active:equation", loc)
    /// == Some(Bool(true))`. Sem cláusula `is_counted` — equations
    /// não têm o predicado caption-based de figures; numbering
    /// activo é controlado externamente via state. Em produção
    /// (sem `Content::SetEquationNumbering`), gate nunca dispara →
    /// counter introspector vazio → P188 substitution-with-fallback
    /// cobre via legacy.
    Equation {
        block:          bool,
        counter_update: CounterUpdate,
    },

    /// **P195B** — payload de `Content::Labelled` emitido em
    /// **post-recursion** pelo walk arm (per ADR-0069). Pattern
    /// arquitectural novo distinto dos outros variants: estes vêm
    /// de `extract_payload` puro pre-recursion; `Labelled` é
    /// produzido directamente pelo walk arm após recursão no target
    /// porque `resolved_text` depende de state mutado durante walk
    /// recursivo (counter formatting via `state.format_hierarchical`,
    /// `state.get_flat`, `state.figure_numbers`, `state.lang`).
    ///
    /// Campos:
    /// - `label: Label`: chave para `intr.resolved_labels` populate.
    /// - `resolved_text: Option<String>`: texto pré-computed
    ///   ("Secção 1.2", "Equação (3)", "Figura 5"); `None` para
    ///   target types não-resolvíveis (catch-all `_ => None` em
    ///   walk arm legacy).
    /// - `figure_number: Option<usize>`: `Some(n)` apenas quando
    ///   target é Figure numerada+captioned. Permite popular
    ///   `intr.figure_label_numbers` em paralelo com P168 arm
    ///   Figure (write redundante mas inofensivo).
    ///
    /// `from_tags` arm Labelled (P195C) popula ambos sub-stores.
    /// Walk arm legacy (E4 P189B) **mantém** mutação directa em
    /// `state.resolved_labels` + `state.figure_label_numbers`
    /// durante janela compat M5; E4 fecha estruturalmente em P195;
    /// funcionalmente em M6.
    Labelled {
        label:         Label,
        resolved_text: Option<String>,
        figure_number: Option<usize>,
    },
}
```

**Nota sobre derives** (P169): `ElementPayload` deixou de derivar `Eq, Hash`
porque `Value` (em `Metadata` variant) não impl `Eq` (f64 NaN). `Hash`
é implementado manualmente via `format!("{:?}", self).hash()` (estratégia
consistente com `entities::content_hash::hash_content`). `Eq` é
declarada via `impl Eq for ElementPayload {}` (white-lie consistente
com PartialEq derive de Value, que tem mesma issue).

---

## Semântica

- `Heading`: representa um heading que será indexado pela introspecção. `depth` é o nível clamped (1–6). `body_hash` permite que o registry detecte alterações ao corpo entre iterações sem clonar `Content` inteiro (essencial para o fixpoint M2+). `counter_update` regista que tipo de update foi aplicado ao counter "heading".
- `Figure`: representa uma figura. `kind` é o discriminador para contadores independentes por tipo. `counter_update` regista o step do counter "figure:{kind}".
- `Citation`: representa uma citação. Apenas a `key` é relevante para introspecção (resolução para bib_entry posterior, em M9 ou semelhante).

---

## Invariantes

- Apenas 3 variantes em P161 — coerente com `ElementKind`.
- Cada variante tem **apenas** os campos confirmados em sub-passo .1. Não adicionar campos especulativos (e.g. `Heading.label`, `Heading.numbering`) — esses ficam em `ElementInfo` (label) ou são derivados pelo Layouter (numbering).
- `body_hash` em `Heading` é populado por `hash_content` (P162 .B); em P161 era placeholder `0`, resolvido em P162 .D quando `extract_payload` o chama.
- `Hash` derivado: necessário para o fixpoint detectar convergência.
- `Eq` derivado: igualdade exige todos os campos iguais.

---

## Consumers actuais

Nenhum em P161 — `ElementPayload` é infraestrutura passiva.

## Consumers planeados

- `entities/element_info.rs` (P161 sub-passo .8) — wrap `ElementPayload` + `Option<Label>`.
- `entities/tag.rs::Tag::Start(Location, ElementInfo)` (P161 sub-passo .9).
- `rules/introspect.rs` walk em P162 — constrói `ElementPayload` para cada Heading/Figure/Cite encontrado, emite `Tag::Start(loc, ElementInfo { payload, label })`.

---

## Sobre paridade

Vanilla não tem `ElementPayload` enum. A informação equivalente é distribuída por:
- `HeadingElem` struct com fields `level`, `body`, `numbering`, `supplement`, etc.
- `FigureElem` struct com fields `body`, `caption`, `kind`, `numbering`, `supplement`, etc.
- `CiteElem` struct com fields `key`, `supplement`, `form`, `style`.

Cristalino agrega num enum estreito por dois motivos:

1. Coerência com `Content` (enum) e `ElementKind` (enum) — sem proc-macros vtable.
2. Subset deliberado: só os campos que o motor de introspecção precisa de observar. Outros campos (numbering pattern, supplement) são responsabilidade do Layouter, não do Introspector.

Ver `desenho-introspection-fixpoint.md` §2.1 (referenciado em P161; documento ainda por localizar/produzir — registado como lacuna em `inventario-tipos-introspection-vanilla.md` 2026-04-30) para o contexto de design.

---

## Resultado Esperado

- `01_core/src/entities/element_payload.rs` — enum + tests unitários (construção de cada variante, igualdade, hash, clone).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P161 sub-passo .7: forma fechada por kind para introspecção M1 | `element_payload.rs`, `element_payload.md` |
| 2026-04-29 | P178: variant `Outline` unit adicionada para suporte de `query("outline")` | `element_payload.rs`, `element_payload.md` |
| 2026-05-03 | P186B: variant `Equation { block, counter_update }` adicionada (forma paralela a `Figure` P184B); suporta P186 plano (eixo 2 P183C); P186D adiciona arm em `extract_payload`, P186E adiciona arm em `from_tags` com gate `block && state numbering_active:equation`. | `element_payload.rs`, `element_payload.md` |
| 2026-05-01 | P181C: variant `Bibliography { entries: Vec<BibEntry> }` adicionada; suporta P181D (`extract_payload` arm Bibliography) e P181E (`from_tags` popula `BibStore`) | `element_payload.rs`, `element_payload.md` |
| 2026-05-04 | P195B: variant `Labelled { label, resolved_text, figure_number }` adicionada com pattern arquitectural novo "post-recursion tag emission for state-dependent payload" (ADR-0069 PROPOSTO). **Sem** `extract_payload` arm — payload depende de state mutado durante walk recursivo, impossível em função pura. Walk arm Labelled (P195D) emite Tag manualmente após recursão. `from_tags` arm popula `intr.resolved_labels` + `intr.figure_label_numbers`. P195B = stub no-op em from_tags; P195C estende. | `element_payload.rs`, `element_payload.md`, `from_tags.rs`, `from_tags.md`, `typst-adr-0069-post-recursion-tag-emission.md` |

---

## §P788 — `ElementPayload::Heading` ganha `numbering_active: bool`

**Decisão:** o payload de Heading passa a carregar `numbering_active`,
**baked da chain no momento da emissão** (`walk`, mesmo padrão já usado por
Equation/Figure/Table — `chain.custom("heading.numbering") ==
Some(Value::Bool(true))`, canal de `rules.rs`). Necessário para o erro
vanilla `cannot reference heading without numbering` (layout_references.md
§P788): o counter de heading aplica-se incondicionalmente (P335), logo
"tem counter" ≠ "tem numbering" — a flag explícita é a única fonte fiel.
O construtor (`HeadingElem::to_payload`) inicia a `false`; o walk sobrepõe.

## P1140.4-C — transporte locatável do suplemento de equação

### Medição antes da decisão

No vanilla, a síntese usa os styles da própria equação
(`math/equation.rs:173-188`), antes da referência consumir o resultado. Locale
e callback pertencem ao alvo. O payload cristalino atual
(`element_payload.rs:146-159`) só transporta numbering.

### Decisão

O payload `Equation` passa a transportar a especificação efetiva de
`supplement` (`auto`, vazio/`none`, conteúdo ou função), a língua capturada da
chain e conteúdo suficiente para reconstruir o argumento público da callback.
O conteúdo materializado pertence ao sub-store por Location, não ao payload.
Todos os braços exaustivos, `Hash` e testes do payload devem ser atualizados.

## P1140.5-A — captura de `alt` e visão pública da equação

### Medição antes da decisão

`typst query 'math.equation'` no vanilla devolveu sempre os defaults efetivos
e `alt`: ausência/`none` → null, vazio → `""`, set-rule → string herdada. O
payload cristalino ainda não carrega `alt` nem number-align efetivo.

### Decisão

O payload Equation ganha `alt: Value` (`Str | None`) e o alinhamento efetivo
necessário à visão realizada. O walk captura ambos da chain, com defaults
medidos. Esses campos alimentam introspecção/query e emissão semântica; não são
copiados para `EquationElem`. Hash/eq e matches exaustivos incluem os campos.

## P1339 — ocorrência nativa sem novo ElementKind

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto SHA-256
`35fcddfd22a08c824c146a82b5feaea93c18d5751d256875b1550cb8255411a9`:
o enum não possui payload para Strong/Emph. `compiler/introspect.rs:1278-1363`
aloca Location e entrada de conteúdo somente se extract_payload retorna Some;
`:1398-1401` atravessa Strong/Emph sem ocorrência própria. O L0 de locatable
exige equivalência entre is_locatable e presença de payload. Portanto um
Selector que conserve a função não cria por si só a ocorrência consultável.

As sondas `p1339-where-integration-probe-runs.json` e
`p1339-where-integration-probe-supplement-runs.json`, UTC respectivamente
`2026-09-10T00:18:55.842594+00:00`–`00:18:59.537177+00:00` e
`00:20:17.650961+00:00`–`00:20:19.215281+00:00`, fixam o working tree
não commitado e o vanilla ratificado `a51e02804`. Query/counter aceitam
Strong/Emph e aplicam filtros de body; Text é rejeitado como não localizável.
Fonte normativa: `model/strong.rs:21`, `model/emph.rs:26`, Locatable/Tagged;
`introspection/counter.rs:338-357`, contagem filtrada por campos.

`compiler/introspect.rs:1944-1948` já emite Tag::End com hash_content do nó;
`compiler/introspect/convergence.rs:25-28` inclui a tag final no hash da
sequência. A forma unit do evento inicial não elimina essa evidência de
mudança do conteúdo. A ADR-0069 possui precedente de payload sem novo Kind,
mas sua emissão pós-recursão excepcional não se aplica a estes nós puros.

### Decisão aprovada — uma variante pública adicional

Adicionar somente esta variante unit ao enum ElementPayload:

```rust
NativeElement,
```

Ela marca a existência de uma ocorrência de elemento nativo cujo conteúdo
completo fica no store canônico por Location. Em P1339, os únicos produtores
novos com unidade de ocorrência identificada são Content::Strong e
Content::Emph próprios. Uma representação alternativa em Styled só pode ser
promovida após medição de equivalência de ocorrência e L0 proprietário de
extração/walk que fixe cardinalidade, ordem, parent e ausência de dupla
alocação. Flags de origem usadas pelo matcher de show, isoladamente, não
provam uma ocorrência: flags Strong/Emph simultâneas ou Styled envolvendo
Strong próprio não autorizam emitir Locations adicionais automaticamente.
Estilo de render assado não é origem semântica. Não é catch-all para os demais Content, não
promove Text, elementos dinâmicos ou tipos futuros implicitamente.

Não adicionar ElementKind::NativeElement nem Kind por função. A ocorrência
usa o par normal Tag::Start/End, Location e ElementInfo já existentes;
identidade e campos são projetados do Content/snapshot armazenado, nunca
inferidos da unidade NativeElement, de hash, nome lexical ou payload falso
de Heading/Metadata. Label causal permanece em ElementInfo. O Hash de
conteúdo é mecanismo de convergência, não comparador de identidade/filtros
da linguagem; não duplicá-lo no payload nem preenchê-lo com constante.

Esta cláusula sucede somente para NativeElement a correspondência histórica
uma variante por Kind. Preservar variantes, campos e derives/implementações
vigentes do enum; o exemplo antigo de Interface pública não remove variantes
posteriores. A entidade permanece dado puro, sem compiler, callback, registry,
mapa de propriedades ou método público adicional.

### Gates e alcance

A variante quebra matches Rust externos exaustivos; o gate específico
ADR-0127 foi aprovado em p1339-where-payload-approval.json, separadamente
da aprovação de Selector/ShowSelector. A promoção completa
exige L0s próprios de extração, locatability, walk, consulta, contagem e
sincronização das Locations com layout antes de código; esta entidade não
legitima esses consumers sozinha. Não muda fase de avaliação de callbacks.

Aceitação posterior: ocorrências Strong/Emph distintas e ordenadas, query de
body match/miss, counter filtrado e updates por chave, labels/parents/posições
sem regressão; mudança de body deve continuar detectável entre iterações por
Tag::End. Clone/hash/construção unit não bastam como prova de linguagem.
Inferência de suficiência: os dados completos e a tag final tornam desnecessário
novo campo no payload para o recorte. Refutam-na perda de identidade/campos
no store, hash final ausente, alteração de fase ou necessidade de outro dado
público; nesses casos reabrir o desenho, sem ampliar esta variante em silêncio.
O hash do Content nu não prova estabilidade de campos derivados somente da
chain; o owner de captura deve medir esse caso e preservar a evidência causal
na integração. Duplicar o mesmo hash no payload não soluciona essa perda.
Esta proposta não afirma paridade geral de query/counter nem fecha P1339.
