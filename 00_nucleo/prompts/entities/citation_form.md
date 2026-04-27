# Prompt L0 — CitationForm
Hash do Código: aa847167

## Módulo
`01_core/src/entities/citation_form.rs`

## Propósito

`CitationForm` é enum entity com forms de citação minimal —
vanilla `CiteForm` reduzido a 4 forms universais (Normal, Prose,
Author, Year). Adicionado em P159C (Model Bibliography + Cite
Fase 2 sub-passo 2) como suporte a `Content::Cite { ..., form,
.. }`.

## Divergência do original (subset minimal per ADR-0054 graded)

Vanilla `CiteForm` integra forms CSL extensas + variantes
específicas de styles. Cristalino reduz a **4 forms universais**
suportados por todas as styles bibliográficas:

- `Normal` — placeholder `[key]` (default).
- `Prose`  — `Author (Year)`.
- `Author` — apenas autor.
- `Year`   — apenas ano.

Forms vanilla **diferidos** per ADR-0054 graded (extensível sem
breaking change via adição de variants):
`Full`, forms CSL específicas, etc.

Render real CSL é diferido (depende `hayagriva` ADR-0062);
cristalino renderiza placeholder melhorado por form com lookup
Bibliography same-document via `CounterState::bib_entries`.

## Representação

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationForm {
    Normal,
    Prose,
    Author,
    Year,
}

impl Default for CitationForm {
    fn default() -> Self { Self::Normal }
}
```

## Interface pública

```rust
impl CitationForm {
    pub fn as_str(&self) -> &'static str;
}
```

`as_str()` produz strings lowercase ("normal"/"prose"/"author"/
"year") — útil para tests, debug e serialização inversa.

`Default` explícito como `Normal` (paridade vanilla
`CiteForm::Normal`); permite usar `unwrap_or_default()` em layout
para resolver `None ↔ Auto` per ADR-0064 Caso A.

## Uso

`Content::Cite { key, supplement, form: Option<CitationForm> }`
em P159C. Stdlib `native_cite` parseia `form: Str` named opcional
via helper privado `extract_citation_form` (strict matching;
case-sensitive; rejeita strings inválidas com mensagem listando
forms válidas).

Layouter resolve `form.unwrap_or_default()` (None ↔ Normal) e faz
match-arm:
- `Normal` (ou key não encontrada): `[key]` placeholder.
- `Prose`: `Author (Year)`.
- `Author`: `Author`.
- `Year`: `Year`.

Lookup Bibliography via `state.bib_entries` (paridade
infraestrutural P158B `state.lang`).

## Critérios de verificação

- `CitationForm::Normal.as_str()` devolve `"normal"`; idem para
  outros 3 variants.
- `CitationForm::default()` devolve `CitationForm::Normal`.
- `PartialEq`/`Eq`: forms diferentes não são iguais.

## ADRs aplicadas

- **ADR-0034**: diagnóstico cumprido em P159C §1-7 +
  específicos §8 (localização) + §9 (lookup) + §10 (quebra hash).
- **ADR-0054**: graded scope-out de forms adicionais (Full,
  CSL-specific) e CSL render real (depende hayagriva).
- **ADR-0064 Caso A**: `Smart<Option<CiteForm>>` →
  `Option<CitationForm>` (achatamento 2-níveis Smart → 1-nível
  Option). Patamar Caso A cresce N=5 → 6.
- **ADR-0065 critério #2** (escolha de tipo): enum dedicado vs
  Option<String> simples — enum mantém type-safety + lista
  exhaustiva em pattern-match. Decisão documentada em
  diagnóstico P159C §1.
- **ADR-0037** (coesão por domínio): ficheiro novo
  `entities/citation_form.rs` (5ª aplicação consecutiva do
  padrão "tipo entity em ficheiro próprio" — Sides P156C →
  Parity P156E → Dir P156I → BibEntry P159A → CitationForm
  P159C).
