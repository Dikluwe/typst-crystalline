# ⚖️ ADR-0060: Model (structural) roadmap — Fase 1 + Fase 2 + Fase 3

**Status**: `IMPLEMENTADO` (Fase 1 fechada; Fase 2 e Fase 3 prosseguem
como roadmap planeado e aplicam-se em passos subsequentes —
**P157/158/159** após renumeração registada em P156B; ver anotação
abaixo).
**Validado**: Passo 154A — diagnóstico; Passo 154B — sub-passo 1
(terms + divider); **Passo 155 — sub-passo 2 (quote); Fase 1 fechada**.
**Data**: 2026-04-25
**Autor**: Humano + IA
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-model-passo-154a.md`](../diagnosticos/diagnostico-model-passo-154a.md)

**Anotação Passo 154B (2026-04-24)**: primeiro sub-passo da Fase 1
materializado — `Content::Divider`, `Content::Terms`,
`Content::TermItem` adicionados ao enum `Content`; `native_terms`
e `native_divider` registadas em `make_stdlib`. Sem ADR nova.
Status permaneceu `PROPOSTO` aguardando Passo 155.

**Anotação Passo 155 (2026-04-25)**: segundo sub-passo da Fase 1
materializado — `Content::Quote { body, attribution, block, quotes }`
adicionado ao enum; `native_quote` registada em `make_stdlib`;
módulo novo `01_core/src/rules/lang/quotes.rs` com
`localize_quotes(lang)` cobrindo 6 idiomas (`pt`/`en`/`de`/`fr`/`es`/`it`)
+ default ASCII; `eval_markup` actualizado para tratar
`SyntaxKind::SmartQuote` (alternância open/close por sequência markup
emitindo glyph localizado). Regression test garante que `"..."` em
contexto de código (ex: `#let s = "..."`) continua a ser
`Value::Str`. **Fase 1 fechada**. Status `PROPOSTO → IMPLEMENTADO`.
Cobertura Model 41% → ~45%; arquitectural Content 75% → ~77%.
Plano Fase 2 (P156/157/158 — table foundations, figure kinds,
bibliography+cite com ADR-0061) inalterado.

**Anotação Passo 159G (2026-04-27)**: **segundo sub-passo
família 159 fora do Bloco A** do diagnóstico P159B (Bloco A
esgotado pós-P159F). Refino estrutural de tipo entity `BibEntry`
adicionando os **6 fields restantes mais comuns hayagriva**
(`editor`/`series`/`note`/`isbn`/`location`/`organization`) —
listados em P159D §9.3 como diferidos por menor universalidade.
**Pattern P159D replicado pela terceira vez** — **subpadrão
#16 cresce N=2 → 3** "refino de tipo entity sem alteração ao
variant Content" (P159D BibEntry 4 fields + P159E BibEntry 2
fields + **P159G BibEntry 6 fields**); patamar **atinge limiar
formalização N=3-4**; promoção a ADR meta possível em passo
administrativo XS futuro NÃO reservado. Builder pattern fluente
extendido em `entities/bib_entry.rs` (6 novos `with_*` métodos:
`with_editor`/`with_series`/`with_note`/`with_isbn`/
`with_location`/`with_organization`); constructor `new(4 args)`
original preservado (backwards compat trivial via fields novos
default `None`). Helper `extract_bib_entries` (P159A+P159D+P159E)
extendido em `stdlib/structural.rs`: helper inline `optional_str`
reusado para os 6 fields — **cumulativo N=4 P159D + N=2 P159E
+ N=6 P159G = N=12 usos** (largamente acima do limiar promoção
N=3-4). Layout `format_bib_entry` extendido em
`rules/layout/mod.rs` com concatenação condicional APA-like
extendida (decisões diagnóstico §8.2 ordem + §9 formatos
individuais): editor `(Ed. {editor})` após title; series
`({series})` após title; location: antes de publisher
(`{location}: {publisher}`); organization substitutivo a
publisher quando publisher ausente (decisão arbitrária per
ADR-0054 graded); isbn antes de url/doi com prefixo lowercase
`isbn:{isbn}` (paridade P159E doi prefix lowercase); note ao
final entre brackets `[{note}]`. **Sem alteração ao variant
`Content::Bibliography` ou `Content::Cite`** (estrutura
inalterada). Tests +11 (1230 → 1241; 4 unit bib_entry P159G +
4 stdlib parse + 3 layout E2E formato extendido/regression/
organization substitutivo; range esperado +8-12). Cobertura
Model agregada **inalterada** (~50%) — refino tipo entity.
Cobertura arquitectural **inalterada** 82%. Hash
`entities/content.rs` preservado `ec58d849` (**décimo sétimo
passo consecutivo** via L0-baseline interpretation). Hash
`entities/bib_entry.rs` preservado `5a2c0ebd` (paridade P159D+
P159E resultado — extensão via doc-comment do header não
modifica prompt L0 `bib_entry.md`). **BibEntry pós-P159G: 16
fields total** (4 obrigatórios + 12 opcionais; cobertura
~70-75% hayagriva universais). Status `IMPLEMENTADO` mantido.
**Sequência alfabética identificadores família 159
não-monótona**: A → B → C → D → F → E → G (facto histórico
registado; preserva slot E para refinos família 159 que
surgiram após P158C ocupar identificador alternativo).
**Política "sem novas reservas" preservada** — restantes fields
vanilla (`booktitle`/`address`/`chapter`/`type`/`institution`/
etc.), tipos estruturados (`Vec<Person>` editor, location codes,
ISBN validation), CSL real (depende hayagriva ADR-0062),
hyperlinks (Bloco C), promoção `optional_str` a helper público,
ADR meta subpadrão #16 (N=3 atinge limiar) permanecem candidatos
NÃO-reservados.

**Anotação Passo 159E (2026-04-27)**: **primeiro sub-passo
família 159 fora do Bloco A** do diagnóstico P159B (Bloco A
esgotado pós-P159F). Refino estrutural de tipo entity `BibEntry`
adicionando 2 fields opcionais identificadores digitais
(`url`/`doi`) — par natural identificado em P159D §9 como
candidato a sub-passo M futuro. **Pattern P159D replicado
fielmente** — **subpadrão #16 cresce N=1 → 2** "refino de tipo
entity sem alteração ao variant Content" (P159D BibEntry 4
fields + **P159E BibEntry 2 fields**); patamar atinge meio-caminho
do limiar formalização N=3-4. Builder pattern fluente extendido
em `entities/bib_entry.rs` (`with_url`/`with_doi`); constructor
`new(4 args)` original preservado (backwards compat trivial via
fields novos default `None`). Helper `extract_bib_entries`
(P159A+P159D) extendido em `stdlib/structural.rs`: helper inline
`optional_str` reusado para url/doi — **cumulativo N=2 P159D +
N=2 P159E = N=4** (atinge limiar promoção a `pub(super)` ou
helper público N=3-4; reavaliação em passo administrativo XS
futuro NÃO reservado). Layout `format_bib_entry` extendido em
`rules/layout/mod.rs` com concatenação condicional APA-like
(Opção C diagnóstico §8.2): url/doi após `(year).` per paridade
APA + backwards compat — quando ambos `None`, output P159D
preservado exactamente. Formato decidido em diagnóstico §9:
URL plaintext literal `https://example.com/paper`; DOI prefixo
`doi:10.1234/abc` (paridade APA estilo prose). **Sem alteração
ao variant `Content::Bibliography` ou `Content::Cite`**
(estrutura inalterada). Hyperlinks NÃO suportados — plaintext
simples per ADR-0033 + ADR-0054 graded; depende de Layout/PDF
infra cross-módulo (Bloco C — NÃO reservado). Tests +8 (1222 →
1230; 3 unit bib_entry url/doi + 3 stdlib parse + 2 layout E2E
formato extendido/regression; range esperado +5-8). Cobertura
Model agregada **inalterada** (~50%) — refino tipo entity.
Cobertura arquitectural **inalterada** 82%. Hash
`entities/content.rs` preservado `ec58d849` (**décimo sexto
passo consecutivo** via L0-baseline interpretation). Hash
`entities/bib_entry.rs` preservado `5a2c0ebd` (paridade P159D
resultado — extensão via doc-comment do header não modifica
prompt L0 `bib_entry.md`). Status `IMPLEMENTADO` mantido.
**Política "sem novas reservas" preservada** — restantes fields
vanilla (`editor`/`series`/`note`/`isbn`/`location`/`organization`),
tipos estruturados (`QualifiedUrl`/`Doi`), URL/DOI validation,
hyperlinks (Bloco C), promoção `optional_str` a helper público
permanecem candidatos NÃO-reservados.

**Anotação Passo 159F (2026-04-27)**: **quarto sub-passo
substantivo Bibliography + Cite materializado** (Fase 2 — **último
candidato Bloco A do diagnóstico P159B**). Refino comportamental:
counter local de bib entries + render numerado em Cite Normal/None.
Field aditivo `pub bib_numbers: HashMap<String, u32>` em
`CounterState` (paridade aditiva infraestrutura state lookup —
**subpadrão #15 cresce N=2 → 3** via `state.lang` P158B +
`state.bib_entries` P159C + **`state.bib_numbers` P159F**;
patamar atinge limiar de candidato a formalização ADR meta).
Walk arm `Content::Bibliography` em `introspect.rs` popula
contínuamente: `state.bib_numbers.entry(key).or_insert(len + 1)`
— multi-Bibliography preserva primeiro número (paridade
HashMap; comportamento determinístico). Layout arm
`Content::Cite { form: Normal/None }` em `layout/mod.rs` faz
lookup `state.bib_numbers.get(key)` → `[N]` ou fallback `[key]`
(regression P159A). Forms diferenciadas (Prose/Author/Year)
inalteradas — numeração só em Normal/None preserva semântica
forms diferenciadas (decisão diagnóstico §10). **Decisão
arquitectural-chave Opção C** (Cite.form interaction sem field
user-facing) escolhida com matriz multi-critério vs Opção A
(substituir sempre; rejeitada por quebrar tests P159A/C) e
Opção B (Bibliography.style field novo; rejeitada por alteração
estrutural sem ganho proporcional para style numeric único).
**Multi-Bibliography contínua** (paridade vanilla numeric style;
decisão diagnóstico §9). **Sem alteração ao variant
`Content::Cite` ou `Content::Bibliography`**. Helper inline
trivial — sem helper público novo (promoção diferida N=3-4).
Tests +8 (1214 → 1222; 2 unit counter_state bib_numbers + 6
layout E2E numbering — `cite_normal_renderiza_numero_quando_bib_populada`,
`cite_normal_fallback_placeholder_quando_bib_vazia`,
`cite_normal_multiple_entries_numeradas_em_ordem`,
`cite_form_prose_inalterada_com_bib_numerada`,
`cite_unknown_key_fallback_placeholder`,
`cite_normal_multi_bibliography_continua`; range esperado
+10-15 ligeiramente abaixo por helper inline trivial). Cobertura
Model agregada **inalterada** (~50%) — refino comportamental.
Cobertura arquitectural **inalterada** 82%. Hash
`entities/content.rs` preservado `ec58d849` (**décimo quinto
passo consecutivo** via L0-baseline interpretation). Status
`IMPLEMENTADO` mantido. **Política "sem novas reservas"
preservada** — outras styles (alphanumeric, author-date, CSL)
permanecem candidatos NÃO-reservados pendentes Bloco B
(hayagriva); `Bibliography.style` field user-facing (Opção B
refino futuro), numeração independente multi-Bibliography
permanecem candidatos NÃO-reservados. **Marca conceptual**:
P159F **esgota Bloco A** do diagnóstico P159B. Pós-P159F, Model
puro está saturado per recomendação do diagnóstico (~55-60%
estimado com 24 entradas parciais). Próximas direcções exigem
Bloco B (hayagriva ADR-0062 promovida), Bloco C (cross-módulo
DEBT-34e/56), refinos Model fora do Bloco A original, mudança
de módulo (Layout Fase 3 columns/colbreak ou Introspection P160),
ou passos administrativos XS.

**Anotação Passo 158C (2026-04-27)**: **quarto sub-passo
Model figure-kinds materializado** (Fase 2 continuação após
P158A auto-detect + P158B supplement por lang). **Refactor
cosmético** `Content::Figure.kind: String → Option<String>` per
**ADR-0064 Caso A estrito** (vanilla `Smart<Str>` → cristalino
`Option<String>`; None ↔ Auto; default `"image"` resolvido em
uso por callers, não em construção). **Patamar Caso A cresce
N=6 → 7** com **primeiro Caso A "estrito" em refactor** (não
em variant aditivo) — distribuição cross-domínio desloca-se
de 50/50 para 43/57 favorecendo Model (3 Layout + 4 Model
após P158C). **Subpadrão emergente N=1 NOVO** "refactor de
field para Option" — precedente novo distinto de variant
aditivo com Option<T> field; aplicação em refactor de tipo
existente; candidato a formalização se outros refactors
análogos forem feitos (e.g. `Heading.body: Box<Content> →
Option<Box<Content>>` se prioritário). Stdlib `native_figure`
adaptado para retornar `Option<String>` directamente em vez
de `String` com fallback hardcoded; `infer_kind_from_body`
(P158A) já retornava `Option<String>` — sem alteração.
~10 sítios callers em `introspect.rs` (counters por kind) e
`layout/mod.rs` (figure_progress + figure_numbers lookup)
adaptados via `kind.as_deref().unwrap_or("image")` em uso.
**Sem alteração observable** (output preservado; backwards
compat trivial via fallback nos callers; tests pré-existentes
P157A/P158A/B preservam após adaptação trivial de
destructuring `Some("...")` em vez de `"..."`). Tests +2
(1212 → 1214; 1 novo `figure_kind_auto_explicito_devolve_none` +
1 novo `introspect_figure_kind_none_resolve_para_image_no_counter`;
range esperado +2-4). ~5 tests existentes adaptados para
asserts `kind.as_deref() == Some(...)` em vez de `kind ==
"..."`. Cobertura Model agregada **inalterada** (~50%) —
refactor cosmético sem mover counts. Cobertura arquitectural
**inalterada** 82%. Hash `entities/content.rs` preservado
`ec58d849` (**décimo quarto passo consecutivo** via L0-baseline
interpretation — lição P159A/C/D internalizada: refactor de
tipo interno cosmético cabe na regra de preservação).
Status `IMPLEMENTADO` mantido. **Política "sem novas reservas"
preservada** — refactor análogo de outros String fields em
Content variants, helper público `kind_or_default(&Option<String>)`,
documentação completa de variants no L0 prompt content.md
permanecem candidatos NÃO-reservados.

**Anotação Passo 159D (2026-04-27)**: **terceiro sub-passo
substantivo Bibliography + Cite materializado** (Fase 2
continuação após P159A par acoplado + P159C cite.form).
**Refino estrutural de tipo entity** `BibEntry` adicionando 4
fields universais opcionais (`volume`/`pages`/`journal`/
`publisher`) per **ADR-0065 critério #2** (terceira aplicação
isolada concreta — selecção de fields universais; patamar
N=2→3). **Builder pattern fluente** (`with_volume`/`with_pages`/
`with_journal`/`with_publisher`) escolhido per Opção C
diagnóstico §8 (legibilidade superior em tests + backwards
compat trivial via constructor `new()` original com 4 args
preservado). Helper `extract_bib_entries` (P159A) extendido
em `stdlib/structural.rs` para parsing dos 4 fields opcionais
com validação tipo `Value::Str` e mensagem de erro mencionando
field específico. Helper privado novo `format_bib_entry` em
`rules/layout/mod.rs` para concatenação condicional APA-like:
`[key] author. title journal vol. volume, pp. pages. publisher
(year).`. Backwards compat trivial — fields opcionais default
`None` preservam exactamente output P159A original. **Sem
alteração ao variant `Content::Bibliography`** (estrutura
inalterada; expansão ortogonal de tipo entity). **Sem alteração
ao variant `Content::Cite`** (P159C inalterado). **Subpadrão
emergente N=1** "refino de tipo entity sem alteração ao variant
Content" (precedente novo — distinção vs P156L `Pad` e P159C
`Cite` que tocaram variants Content; P159D primeiro a refinar
**tipo entity puro** sem efeito no enum). Tests +8 (1204 →
1212; 3 unit bib_entry inclui builder pattern + PartialEq cobre
8 fields + backwards compat new() + 3 stdlib parse + 2 layout
E2E entry completa/mínima; range esperado +5-8). Cobertura
Model agregada **inalterada** (~50%). Cobertura arquitectural
**inalterada** 82%. Hash `entities/content.rs` preservado
`ec58d849` (**décimo terceiro passo consecutivo** via L0-baseline
interpretation). Hash `entities/bib_entry.rs` também preservado
`5a2c0ebd` (L0-baseline — prompt `bib_entry.md` não modificado;
extensão via doc-comment + referência cruzada ao `BibEntry`
struct extendido). Status `IMPLEMENTADO` mantido. **Política
"sem novas reservas" preservada** — fields restantes vanilla
(`url`/`doi`/`editor`/`series`/`note`/`isbn`/`location`/etc.),
tipos estruturados (`PageRange`, `JournalRef`), CSL real
(depende hayagriva ADR-0062), estilo configurável, promoção
`extract_bib_entries`/`format_bib_entry` a helpers públicos
permanecem candidatos NÃO-reservados.

**Anotação Passo 159C (2026-04-27)**: **segundo sub-passo
substantivo Bibliography + Cite materializado** (Fase 2
continuação após P159A par acoplado). Refino estrutural-
comportamental de `cite` adicionando enum
`CitationForm { Normal, Prose, Author, Year }` em
`entities/citation_form.rs` (ficheiro novo; **5ª aplicação
consecutiva** do padrão "tipo entity em ficheiro próprio" —
Sides P156C → Parity P156E → Dir P156I → BibEntry P159A →
CitationForm P159C) + field `form: Option<CitationForm>` em
`Content::Cite` per **ADR-0064 Caso A** (patamar Caso A cresce
**N=5 → 6 atingindo equilíbrio cross-domínio 50/50 Layout/Model**
— terceira aplicação Model após P157B/P159A). 13 sítios
pattern-match Content actualizados (paridade P157A/P159A).
Helper privado novo `extract_citation_form` em
`stdlib/structural.rs` (strict matching case-sensitive; 4
forms válidos; mensagem de erro lista forms aceites). Layout
placeholder melhorado por form com lookup Bibliography via
novo field `pub bib_entries: Vec<BibEntry>` em `CounterState`
(paridade infraestrutural P158B `state.lang`); populado por
introspect walk; multi-Bibliography concatena na ordem de
aparecimento. Render: `Normal/None ↔ [key]`; `Prose ↔ Author
(Year)`; `Author ↔ Author`; `Year ↔ Year`; fallback `[key]`
quando key não encontrada (paridade Normal sem entry). **Sem
alteração ao número de variants Content** (estrutura
inalterada; expansão de field). **Subpadrão emergente N=2**
"infraestrutura state lookup" (P158B `state.lang` + P159C
`state.bib_entries`). Tests +15 (1189 → 1204; 8 unit +
2 cite com form + 6 stdlib parse + 4 layout E2E forms; range
esperado +12-17). Cobertura Model agregada **inalterada**
(~50%). Cobertura arquitectural **inalterada** 82%. Hash
`entities/content.rs` preservado `ec58d849` (**décimo segundo
passo consecutivo** via L0-baseline interpretation —
content.md prompt não modificado; refino arquitectural
documentado via doc-comments + referência cruzada
citation_form.md). Status `IMPLEMENTADO` mantido. **Política
"sem novas reservas" preservada** — forms vanilla adicionais
(Full, CSL-specific), CSL render real (depende hayagriva
ADR-0062), `style: Str` per-Cite, cross-document refs
(bloqueado ADR-0017), promoção `extract_citation_form` a
helper público permanecem candidatos NÃO-reservados.

**Anotação Passo 158B (2026-04-27)**: **segundo sub-passo
Model figure-kinds materializado** (Fase 2 continuação após
P158A). Refino qualitativo de `figure` — supplement
automático localizado por lang. Helper novo
`figure_supplement_for_lang(kind: &str, lang: Option<&Lang>)
-> String` em `rules/lang/figure_supplement.rs` cobrindo
6 langs (pt/en/de/fr/es/it) × 3 kinds (image/table/raw) =
18 entradas + fallback PT por kind + capitalização para kind
desconhecido. Field novo `pub lang: Option<Lang>` em
`CounterState` para lang resolution (default `None` →
fallback PT, paridade backwards compat com tests
pré-existentes que esperam "Figura"). Modificação trivial
em `introspect.rs` linha 334: `Some(format!("Figura {}", n))`
→ `Some(format!("{} {}", figure_supplement_for_lang(kind,
lang), n))`. **Sem alteração ao variant `Content::Figure`**
(estrutura inalterada). **Reuso explícito do padrão P155**
`localize_quotes(lang)` em `rules/lang/quotes.rs` —
**primeiro reuso cross-feature** (quotes → figure supplement);
estrutura paralela: tabela estática + lookup linear + fallback;
**subpadrão emergente N=1** "padrão P155 i18n reusado
cross-feature" (candidato a formalização N=3-4 mínima).
Tests +15 (1174 → 1189; 8 unit em figure_supplement.rs +
7 integration em introspect.rs; range esperado +12-15).
Cobertura Model agregada **inalterada** (~50%) — segundo
refino qualitativo consecutivo de `figure`. Cobertura
arquitectural **inalterada** 82% (refino de variant existente).
Hash `entities/content.rs` preservado `ec58d849` (**décimo
primeiro passo consecutivo**). Status `IMPLEMENTADO` mantido.
**Política "sem novas reservas" preservada** (estabelecida
em P158; respeitada em P158A/B): `supplement: Option<Content>`
field user-facing, mais langs além de 6 minimais, CSL-aware
format (depende hayagriva), region-specific supplements
permanecem candidatos NÃO-reservados.

**Anotação Passo 159A (2026-04-27)**: **par acoplado
Bibliography + Cite minimal materializado** (Fase 2 continuação
após figure-kinds P158A). **Estrutura A adaptada** per diagnóstico
P159 §3.5 — par num único passo M+ sem hayagriva. Tipo entity
novo `BibEntry { key, author, title, year }` em
`entities/bib_entry.rs` (4 fields universais; ADR-0065 critério
#2 escolha de tipo primeira aplicação isolada concreta).
Variants `Content::Bibliography { entries: Vec<BibEntry>,
title: Option<Box<Content>> }` + `Content::Cite { key: String,
supplement: Option<Box<Content>> }` adicionados ao enum (56 →
58 variants). Stdlib `native_bibliography` + `native_cite`
em `stdlib/structural.rs` (continuação Model). **Naming
`bibliography` e `cite` flat** (paridade decisão P157B
naming flat). **Helper privado novo `extract_bib_entries`**
parseia `Value::Array<Value::Dict>` para `Vec<BibEntry>` com
validação hard de 4 fields obrigatórios. **ADR-0064 Caso A**
aplicado em title (Bibliography) e supplement (Cite) — patamar
Caso A cresce **N=4 → 5** (P156G/H/I + P157B + P159A; 60% Layout
+ 40% Model). Layouter renderiza placeholder per ADR-0033 +
ADR-0054 graded — Bibliography como lista
`"[{key}] {author}. {title} ({year})."` per linha; Cite como
`"[{key}]"` + supplement. **Sem validação cross-reference**
`Cite.key ∈ Bibliography.keys` per ADR-0017 Introspection
runtime adiada. **Sem hayagriva** — input cristalino literal
`Vec<BibEntry>`; ADR-0062 mantém-se reserva sem ficheiro.
Tests +27 (1147 → 1174). Cobertura Model agregada **inalterada**
(50%); cobertura ampla impl+impl⁺+parcial cresce
(22 → 24 entradas parciais — `cite` e `bibliography` movem
`ausente → parcial`); cobertura arquitectural **80% → 82%**
(2 variants novos). Status `IMPLEMENTADO` mantido. **Granularidade
quebrada honestamente** N=13 → M+ com precedente P156C par
lógico pad+hide. **DEBT-55 contribuído mas NÃO fechado** —
refinos futuros (hayagriva integration, CSL, form variants
Normal/Prose, numbering schemes, cross-document forward refs)
**NÃO reservados** per política P158. Hash `entities/content.rs`
mantém-se `ec58d849` (nono passo consecutivo — hash refere-se
ao prompt L0 que permanece inalterado; refino futuro pode
actualizar L0 com documentação dos novos variants).

**Anotação Passo 158A (2026-04-27)**: **primeiro sub-passo
Model figure-kinds materializado** (Fase 2 continuação após
table foundations fechado P157C). Refino qualitativo de
`native_figure`: helper privado novo `infer_kind_from_body(body:
&Content) -> Option<String>` em `stdlib/figure_image.rs` cobrindo
Image/Table/Raw + recursão limitada a `Content::Sequence`
(paridade vanilla parcial per ADR-0033 — vanilla usa
`query_first_naive` recursivo profundo; cristalino limita a
Sequence per decisão P158A §8). Fallback chain 3 níveis em
`native_figure`: `kind:` explícito > inferência > default
`"image"` (precedência absoluta para `kind:` explícito preserva
tests pré-existentes). **Sem alteração ao variant `Content::Figure`**
(estrutura inalterada; `kind: String` continua directo).
**Sem alteração a `introspect.rs` ou layout** (counters por
kind continuam funcionar inalterados — refino vive só na
origem do valor `kind`). Tests +6 (1141 → 1147; range esperado
+6-8). Cobertura Model agregada **inalterada** (~50%) — refino
qualitativo. Hash `entities/content.rs` preservado `ec58d849`
(sétimo passo consecutivo). Status `IMPLEMENTADO` mantido.
**Política "sem novas reservas" preservada** (estabelecida em
P158): supplement automático por lang, show selectors
`figure.where(kind:)`, refactor `kind: String → Option<String>`
permanecem candidatos NÃO-reservados.

**Anotação Passo 157C (2026-04-26)**: **terceiro e último
sub-passo Fase 2 Model — "table foundations" fechado**.
Par simétrico `Content::TableHeader { body, repeat: bool }` +
`Content::TableFooter { body, repeat: bool }` adicionados ao
enum (54 → 56 variants); `native_table_header` e
`native_table_footer` registadas em `make_stdlib` em
`stdlib/structural.rs` (continuação P157A/B). **Naming
`table_header`/`table_footer` flat** (paridade decisão P157B).
**Primeira aplicação concreta de ADR-0064 Caso D em domínio
Model** (`bool` directo com default `true` paridade vanilla;
P156D weak / P156G breakable / P156J justify aplicaram-no em
Layout). **Saturação cross-domínio cross-caso ADR-0064**: após
P157C, todos os 4 casos canónicos A/B/C/D validados em Layout;
3/4 (A, C, D) validados em Model. Layouter renderiza body
no contexto actual; **`repeat` armazenado mas ignorado em layout**
per ADR-0054 graded — **DEBT-56 permanece aberto** (algoritmo
de repetição em page breaks fica para refactor multi-region).
**Divergência aceite per ADR-0033**: `body: Box<Content>` em
vez de vanilla `#[variadic] children: Vec<TableItem>` para
uniformidade com containers cristalinos existentes; `level`
(Header) e `repeat-rows` scope-out per ADR-0054 graded. Helper
privado novo `extract_bool_with_default(args, fn, field, default)`
parametrizado (genérico no key e no default; distinto de
`extract_weak` específico por field/default — separação de
domínios per ADR-0037). Tests +26 (1353 → 1379). Cobertura
Model agregada **inalterada** (50% mantém-se — sub-entradas
qualitativas paridade P157B); cobertura arquitectural **78%
→ 80%** (variants Content vanilla extra ausentes desce de ~1
a 0). Status `IMPLEMENTADO` mantido. **Padrão cross-domínio
fortalecido**: 3 sub-passos Model consecutivos sem reformulação;
**N=12 materialização**. **"Table foundations" integralmente
materializado** com P157A + P157B + P157C (3 sub-passos M cada;
granularidade preservada N=10/11/12). Promoção a revisão R1
da ADR-0060 candidata (passo administrativo XS) se decisão
humana for prioritária.

**Anotação Passo 157B (2026-04-26)**: **segundo sub-passo
Fase 2 Model materializado** — `Content::TableCell { body,
x: Option<usize>, y: Option<usize>, colspan: Option<usize>,
rowspan: Option<usize> }` adicionado ao enum (53 → 54 variants);
`native_table_cell` registada em `make_stdlib` em
`stdlib/structural.rs` (continuação P157A). **Naming `table_cell`
flat** (não vanilla `table.cell` — FieldAccess actual não suporta
namespacing de funcs `Value::Func.subname`; divergência intencional
documentada per ADR-0033 em diagnóstico P157B §8). **Primeira
aplicação concreta de ADR-0064 Caso A em domínio Model**
(`Smart<usize>` → `Option<usize>` para x/y); **terceira aplicação
global de Caso C** com **primeira variação `usize`**
(NonZeroUsize default 1 → Option<usize> com None ↔ default 1
para colspan/rowspan; zero rejeitado em stdlib paridade vanilla).
Layouter renderiza body no contexto actual; **x/y/colspan/rowspan
armazenados mas ignorados em layout** per ADR-0054 graded —
**DEBT-34e permanece aberto** (placement algorítmico Grid
completo fica para refactor dedicado). Helper privado novo
`extract_usize_or_none_min(val, fn, field, min)` parametrizado
(min=0 para x/y; min=1 para colspan/rowspan). Tests +18
(1335 → 1353). Cobertura Model agregada **inalterada** (50%
mantém-se — `table.cell` é sub-entrada de `table` per padrão
P154A; ganho qualitativo via expansão estrutural). Status
`IMPLEMENTADO` mantido. **Padrão cross-domínio reforçado**:
2 sub-passos Model consecutivos (P157A/B) sem reformulação;
N=11 materialização. TableHeader/Footer ficam para **P157C**.

**Anotação Passo 157A (2026-04-26)**: **primeiro sub-passo
Fase 2 Model materializado** — `Content::Table { columns:
Vec<TrackSizing>, rows: Vec<TrackSizing>, children: Vec<Content> }`
adicionado ao enum (52 → 53 variants); `native_table` registada
em `make_stdlib` em **`stdlib/structural.rs`** (decisão de
módulo Model existente, não novo `stdlib/model.rs` — per
diagnóstico P157A §8). Subset minimal per ADR-0054 graded:
3 fields críticos; ~9 atributos vanilla scope-out; TableCell
estruturado diferido para **P157B**; TableHeader/Footer
diferidos para **P157C**. Layouter delega a `layout_grid`
clone simples per Decisão 4 (sem modificação de `grid.rs`).
Helper `extract_tracks` promovido a `pub(super)` para reuso
cross-módulo (N=2; subpadrão emergente). Tests +16
(1319 → 1335). Cobertura Model 45% → 50% (entrada `table`
transita `ausente → implementado`). Status `IMPLEMENTADO`
mantido (Fase 1 fechada P155 não muda; Fase 2 prossegue per
roadmap). **Padrão cross-domínio confirmado**: granularidade
N=10 estendida de Layout (P156C-L) a Model (P157A) sem
reformulação.

**Anotação Passo 156B (2026-04-25)** — **renumeração de Fase 2**:
P156A foi consumido pelo historiograma (passo administrativo);
P156B é o diagnóstico Layout (este passo de origem da anotação).
Consequentemente Fase 2 Model desloca-se uma posição:

| Antes (ADR-0060 original) | Depois (pós-P156B) |
|---------------------------|---------------------|
| P156 = Model table foundations | **P157** |
| P157 = Model figure-kinds | **P158** |
| P158 = Model bibliography (XL) | **P159** |
| ADR-0061 = autorização hayagriva | **ADR-0062** |

ADR-0061 foi **reocupada** por P156B para roadmap Layout
(`typst-adr-0061-layout-fase-x-roadmap.md`, status `PROPOSTO`).
`hayagriva` passa a reserva ADR-0062 (sem ficheiro criado;
documentado em README ADRs e DEBT-55). Decisão 2 desta ADR-0060
(Fase 2 — `Content::Bibliography` + `Content::Cite` com
autorização `hayagriva`) lê-se agora "ADR-0062 + Passo 159"
em vez de "ADR-0061 + Passo 158". DEBT-55 actualizada em P156B.

Bloqueio adicional documentado em P156B: **`footnote()` (Decisão 2
desta ADR-0060) requer page model com footnote area** — desbloqueado
pela Fase 1 do roadmap Layout (ADR-0061 nova, Decisão 1 + Decisão 5;
Passo 156C).

---

## Contexto

Inventário 148 §A.6 declara categoria Model (structural) com
**21 entradas** e cobertura 38% (impl + impl⁺). P154A
investigou empiricamente: contagem real = **22 entradas**;
cobertura empírica = **32-36%** (revisão para baixo).

Decomposição empírica (P154A §2):

- 3-4 `implementado` (heading, emph, strong, outline).
- 4 `implementado⁺` (figure, ref, numbering, heading com
  ressalva).
- 5 `parcial` (link, list, enum, par, caption inline).
- **10 `ausente`** (bibliography, cite, footnote, quote,
  terms, table, document, divider, asset, title).

Top divergência 7 do inventário 148 ("~14 elementos `Content::*`
vanilla ausentes") agrega Model + Layout + Visualize. Para
Model especificamente, 6 dessas entradas são alto valor:
`bibliography`, `cite`, `footnote`, `quote`, `terms`,
`table`. Restantes (`document`, `divider`, `asset`, `title`)
são baixo valor ou divergência intencional.

`Content::Styled` (ADR-0026 perfil) é **inadequado** para
Model structural — estas features têm semântica que excede
styling.

ADR-0017 (estratégia typst-library) declarou progressão
gradual; este roadmap operacionaliza a continuação.

## Decisão

ADR-0060 propõe **3 fases** com prioridades explícitas:

### Decisão 1 — Fase 1 (S+M; sem novas crates)

3 sub-passos:

- **Passo 154B** — `Content::Terms` + `Content::TermItem` +
  `Content::Divider` (S agregado).
- **Passo 155** — `Content::Quote` com atributos
  `attribution`, `block` (M).
- **Passo 157** (renumerado de P156 em P156B) —
  `Content::Table` foundations: variant nova + sub-elementos
  `TableCell`, `TableHeader`, `TableFooter` (M+; reaproveita
  `Content::Grid` parcial para layout).

Cobertura post-Fase 1: ~50% (8/22 → 11-12/22).

### Decisão 2 — Fase 2 (com ADR de autorização)

3 sub-passos:

- **Passo 158** (renumerado de P157 em P156B) — `figure` kinds
  extension (depende de Passo 157 para figure-table; M).
- **ADR-0062 + Passo 159** (renumerados de ADR-0061+P158 em P156B)
  — `Content::Bibliography` + `Content::Cite` com autorização
  `hayagriva`. ADR-0062 documenta autorização (precedente
  ADR-0024 ecow, ADR-0023 indexmap, ADR-0057 hypher). Crate
  `hayagriva 0.9.1` já em cache local (per P152). **Nota**:
  ADR-0061 foi reocupada por P156B para roadmap Layout; reserva
  hayagriva passou para ADR-0062.
- **Passo dedicado footnote** — `Content::Footnote` desbloqueado
  por Fase 1 Layout (ADR-0061 nova; Passo 156C). Page model
  ganha `footnote_area` minimalista.

Cobertura post-Fase 2: ~68% (11-12/22 → 15-16/22).

### Decisão 3 — Fase 3 (condicional / divergência intencional)

- **`asset`**: alt-text + scaling sobre `Image`. Acessibilidade.
- **`document`**: divergência intencional cristalino emite
  metadata em export PDF directamente; sem wrapper Content.
- **`title`**: depende de `document`; mesma divergência.

Cobertura potencial: ~77-82% (com restantes em scope-out
declarado).

### Decisão 4 — `Content::Styled` vs variant novo

Para cada feature Fase 1/2: **variant novo** no `Content`
enum.

Razão: Model structural tem semântica que excede styling
(numbering, attribution, cells, citations). `Content::Styled`
(ADR-0026 perfil) cobre apenas estilos visuais simples.
Todas as Fase 1/2 features exigem variants dedicados.

### Decisão 5 — Relação com `lab/parity` corpus

Cada sub-passo Fase 1/2 deve **adicionar 1-3 ficheiros** ao
corpus `lab/parity/corpus/visual/` ou `corpus/markup/`
exercitando a feature nova. Suite layout_parity (P150)
detecta automaticamente; matriz P3 cresce.

Quando vanilla integration fechar (DEBT-53 + DEBT-54), as
mesmas features ganham comparação real.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **Fases 1+2+3 ranqueadas** ✓ | Materialização gradual; cobertura predictível; ADRs por trabalho específico | Trabalho longo (5+ passos) |
| Atacar tudo num passo XL | Único output | Risco alto; mistura concerns; dificil revisão |
| Adiar Model completo até DEBT-53 + DEBT-54 fechar | Foco na série paridade primeiro | Cobertura observacional cristalino-only fica fraca; impede eval real do gap |
| Apenas Fase 1 com ADR limitada | Mínimo risco | Não responde a "trabalho real necessário" |
| ADR única para todas as fases (sem 0061) | Menos ADRs | `hayagriva` exige autorização explícita conforme precedente |

**Escolha**: 3 fases com Fase 2 ganhando ADR-0061 dedicada
para `hayagriva`. Fase 3 condicional sem ADR (decisão
humana posterior).

## Consequências

### Positivas

- **Roadmap explícito** para sair de cobertura Model 32%
  para ~68% sem comprometer ADR-0017 (estratégia gradual).
- **Cada sub-passo tem escopo S/M definido** (excepto
  bibliography Fase 2 = XL com ADR-0061).
- **Corpus paridade cresce automaticamente** com cada
  sub-passo (Decisão 5).
- **Footnote desacoplado** da Fase 1 — não bloqueia features
  simples.
- **`hayagriva` em cache** (probe P152): risco de fetch
  reduzido.

### Negativas

- **5-7 sub-passos entre P154B e P158+** — investimento
  significativo de tempo.
- **Fase 3 condicional**: documentos com `#document(...)`
  ou `#title(...)` continuam não-suportados. Aceitável
  conforme inventário 148 e ADR-0033 perfil graded.
- **`hayagriva` em L1**: precedente ADR-0024 (ecow) +
  ADR-0057 (hypher) cobrem; ADR-0061 invocará.

### Neutras

- Inventário 148 ganha referências cruzadas para ADR-0060
  (per Decisão 5 + actualização P154A).
- `Content` enum cresce: 38 variants → ~46 variants
  pós-Fase 2. ADR-0026-R1 (`Arc<[T]>` em `Sequence`) cobre
  performance de clone.

## Plano de materialização

5 passos no caminho crítico (Fase 1 + Fase 2):

| Passo | Escopo | Features | ADR adicional? |
|-------|--------|----------|-----------------|
| 154B | S | terms, divider | — |
| 155 | M | quote | — |
| 157 (renumerado de 156 em P156B) | M+ | table foundations | — |
| 158 (renumerado de 157 em P156B) | M | figure kinds | — |
| ADR-0062 + 159 (renumerados em P156B) | XL | bibliography + cite | ADR-0062 (era ADR-0061 antes da reocupação por Layout em P156B) |
| (futuro pós-156C) | M-L | footnote | — (Layout Fase 1 ADR-0061 desbloqueia) |
| (Fase 3) | S | asset | — |
| (Fase 3) | divergência | document, title | — |

**ADR-0060 transitou `PROPOSTO → IMPLEMENTADO`** em Passo 155
ao fechar a Fase 1 (terms + divider em P154B; quote em P155).
A Fase 2 (table/figure-kinds/bibliography) e a Fase 3
(asset/document/title) prosseguem como planeado em P156–P158+
sem necessidade de re-abertura desta ADR.

## Referências

- **ADR-0017** — estratégia typst-library gradual.
- **ADR-0026** + **ADR-0026-R1** — `Content` enum fechado
  com `Arc<[T]>` para sequences.
- **ADR-0033** — paridade funcional para cada feature
  materializada.
- **ADR-0034** — diagnóstico obrigatório (cumprido por
  P154A).
- **ADR-0036** — atomização progressiva.
- **ADR-0037** — coesão por domínio.
- **ADR-0038** — `Content::Styled` para styling estrutural.
- **ADR-0054** — perfil observacional graded.
- **DEBT-55** (P154A; actualizada por P156B) — bibliography
  + cite XL com plano **ADR-0062 + Passo 159** (era ADR-0061
  + Passo 158 antes da renumeração).
- **ADR-0061** (P156B) — Layout Fase X roadmap; reocupou o
  número antes reservado para hayagriva.
- **DEBT-56** (P156B) — Column flow Fase 3 Layout L+; aberto
  por P156B.
- **DEBT-34d / DEBT-34e** — grid cell layouting (Passo 80);
  trabalho similar mas distinto de `Content::Table`.
- **Inventário 148** (`typst-cobertura-vanilla-vs-cristalino.md`)
  — Tabela A linha "Model"; §7 entrada 7.
- **Diagnóstico 154A** (`diagnostico-model-passo-154a.md`)
  — Tabelas §2, §3, §6, §7 com plano detalhado.

---

## Anotação cumulativa P258 — Cobertura Model ~73% confirmada empíricamente (Cenário B1 Fase A)

**P258 audit empírico** (`diagnostico-model-fase-a-passo-258.md`)
confirmou cumprimento cumulativo Bloco A Model entre P155 e
P252:

### Cobertura por audit empírico (Tabela B Fase A)

| Estado | P154A | Audit P258 | Δ |
|--------|-------|------------|---|
| implementado | 4 | 4 | 0 |
| implementado⁺ | 4 | **10** | **+6** |
| parcial | 5 | 4 | -1 |
| ausente | 10 | **4** | **-6** |

**Cobertura ponderada linear**: P154A 48% → Audit P258 **~73%**
(Δ **+25pp**).

### Promoções cumulativas detectadas (P155-P252)

- **heading** → implementado⁺ (P182C SetHeadingNumbering).
- **figure** → implementado⁺ preservado (caption + 4 fields).
- **numbering** → implementado⁺ reforçado (3 variants Set*Numbering
  cumulativos via P182C + P199B).
- **caption inline** → implementado⁺ (parte integral Figure).
- **bibliography** → implementado⁺ (P159A-G `bib_entry.rs` 413
  LoC + paridade manual hayagriva sem crate).
- **cite** → implementado⁺ (P159A par acoplado).
- **terms / divider / quote** → implementado (P154B + P155).
- **table** → implementado⁺ (P157A-C + cumulativos fields
  P227/P247/P248/P250).

### Pendentes residuais pós-P258

- **footnote** (ausente) — pendência real isolada; P156C
  desbloqueio Layout preservado mas variant Content + stdlib
  func não materializados. Candidata refino futuro P-Footnote-N
  (M; +10-15 tests).
- **document / title / asset** (ausente) — **Fase 3 condicional
  ADR-0060 §"Fase 3 condicional"** preservada; sem prioridade
  designada.
- **link / list / enum / par** (parcial) — refinos atributos
  vanilla preservados como scope-out informal P258.
- **Bloco B hayagriva** — scope-out implícito P258 (paridade
  manual P159A-G cumpriu user-facing; ADR-0062 PROPOSTO
  preservada para promoção futura quando consumer exigir CSL
  styling completo).

### Status ADR-0060

**`IMPLEMENTADO`** preservado literal (Fase 1 fechada P155).
**Bloco A cumulativamente cumprido** P155-P252. **Cenário B1
P258** confirma fecho conceptual Model agregado ~73% cobertura.
**Fase 2 + Fase 3 preservadas como roadmap** (paridade pattern
ADR-0061 P221 columns/colbreak; ADR-0079 P253 D.2-D.6 roadmap).

### Cumulativo "auditoria condicional" pattern

**N=4 cumulativo P258** (P192A + P255 + P257 + **P258**) —
limiar formalização interno N=5 quase atingido (próximo passo
admin XS candidato a formalizar pattern em ADR meta).

### DEBT-55

**PARCIALMENTE RESOLVIDO P258** (via paridade manual P159A-G).
Permanece aberta para CSL styling completo (hayagriva real)
quando consumer exigir.
