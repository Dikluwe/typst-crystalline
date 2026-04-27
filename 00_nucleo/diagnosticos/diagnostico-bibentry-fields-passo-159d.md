# Diagnóstico BibEntry fields adicionais — Passo P159D

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
terceiro sub-passo Bibliography + Cite (Bloco A do diagnóstico
P159B §3.3). **Décima nona aplicação consecutiva** do padrão
diagnóstico-primeiro.

Refino estrutural de tipo entity `BibEntry` com 4 fields
adicionais Optional. **ADR-0065 critério #2** (escolha de tipo)
**terceira aplicação isolada concreta** — selecção de fields
universais. **Sem alteração ao variant Content** (paridade P158B;
P159C parcial — P159C tocou Content::Cite, P159D NÃO toca).

---

## 1. Assinatura vanilla `hayagriva::Entry` minimal

Per `hayagriva` (vanilla integração), `Entry` tem dezenas de
fields tipográficos: `title`/`author`/`editor`/`date`/`pages`/
`volume`/`issue`/`journal`/`publisher`/`location`/`url`/`doi`/
`isbn`/`series`/`note`/`type`/`organization`/etc.

**Subset minimal P159A**: 4 fields universais (`key`/`author`/
`title`/`year`).

**Subset extendido P159D**: +4 fields universais comuns
(`volume`/`pages`/`journal`/`publisher`).

**Selecção justificada** per ADR-0065 critério #2 (§9 abaixo).

---

## 2. Comportamento observável

**Vanilla**: render dependente de CSL style. APA típico:
```
Smith, J. (2024). Title of paper. Journal Name, 12(3), 1-10.
```

**Cristalino actual** (P159A; `layout/mod.rs:548-552`):
```rust
let line = format!(
    "[{}] {}. {} ({}).",
    e.key, e.author, e.title, e.year,
);
```
Output: `[smith2024] Smith, J.. On Crystal Math (2024).`
(note duplo "." cosmético — não fixado neste passo).

**Cristalino P159D** (proposto):
- Base inalterada quando todos os fields novos forem `None`
  (regression P159A).
- Fields novos appendados condicionalmente:
  - Se `journal` presente: append antes de `(year)`.
  - Se `volume` presente: append `vol. {volume}`.
  - Se `pages` presente: append `pp. {pages}`.
  - Se `publisher` presente: append `{publisher}`.

Exemplo completo (decidido em §10):
```
[smith2024] Smith, J.. On Crystal Math. Nature Communications vol. 12, pp. 1-10. ACM, (2024).
```

---

## 3. ADR-0064 caso aplicável

**NÃO directamente aplicável**. Fields novos são `Option<String>`
trivial (sem mapping `Smart<T>`); ausência é representada por
`None` directo. Não é Caso A canónico (vanilla não tem
`Smart<Option<String>>` para estes fields — fields simplesmente
existem ou não).

---

## 4. Variants Content existentes a estender

**Nenhum**. `Content::Bibliography` e `Content::Cite` ambos
inalterados estruturalmente. P159D toca apenas `BibEntry` struct
em `entities/bib_entry.rs`.

**Hash content.rs preservado esperado** — 13º passo consecutivo
via L0-baseline interpretation (paridade P158B; P159C tocou via
field aditivo `bib_entries` em CounterState mas content.rs
preservado).

**Hash bib_entry.rs quebra esperada** — struct extensão (4 fields
novos). Aceitar.

---

## 5. Helpers stdlib reusáveis

### 5.1 Helper `extract_bib_entries` (P159A) extendido

Localização: `rules/stdlib/structural.rs:507`. Extensão directa:
- Continua a parsear 4 fields obrigatórios (sem alteração).
- Adiciona parsing de 4 fields opcionais via padrão consistente:
  ```rust
  let volume = match dict.get("volume") {
      Some(Value::Str(s)) => Some(s.to_string()),
      Some(other) => return Err(vec![SourceDiagnostic::error(
          Span::detached(),
          format!("bibliography(entries: [{}].volume) espera string, recebeu {}", idx, other.type_name()),
      )]),
      None => None,
  };
  ```
- Constructor `BibEntry::new(...)` actualizado ou `BibEntry { ... }`
  literal com 8 fields.

### 5.2 Helper privado novo `format_bib_entry`

`rules/layout/mod.rs` (privado em arm Bibliography) ou
`stdlib/structural.rs` (helper privado próximo a
`extract_bib_entries`). Decisão: **layout/mod.rs** — formatação
é responsabilidade do layouter; stdlib produz dados.

```rust
fn format_bib_entry(e: &BibEntry) -> String {
    let mut out = format!("[{}] {}. {}", e.key, e.author, e.title);
    if let Some(j) = &e.journal { out.push_str(&format!(" {}", j)); }
    if let Some(v) = &e.volume  { out.push_str(&format!(" vol. {}", v)); }
    if let Some(p) = &e.pages   { out.push_str(&format!(", pp. {}", p)); }
    if let Some(pb) = &e.publisher { out.push_str(&format!(". {}", pb)); }
    out.push_str(&format!(" ({}).", e.year));
    out
}
```

Helper privado N=1; promoção diferida.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P159D | Refino futuro |
|---------|--------------|---------------|
| 4 fields adicionais (volume/pages/journal/publisher) | ✓ implementado | fields adicionais (url/doi/editor/series/note/isbn/location/etc.) NÃO reservados |
| Fields como `Option<String>` directo | ✓ implementado | tipos estruturados (PageRange, JournalRef) NÃO reservados |
| Render formato livre concatenação simples | ✓ implementado | CSL real depende hayagriva (ADR-0062) |
| Validação tipo `Value::Str` apenas | ✓ implementado | tipos compostos NÃO reservados |
| Constructor `new()` único | ✓ implementado | builder pattern NÃO reservado (avaliação §8) |

---

## 7. Tests planeados

### 7.1 Unit tests `BibEntry` (`entities/bib_entry.rs`, ~3)

1. Constructor com fields novos via setters / builder.
2. PartialEq cobre 8 fields agora (cada divergente quebra).
3. Backwards compat: `new(...)` original com 4 args produz
   entries com fields novos `None`.

### 7.2 Stdlib tests (`stdlib/mod.rs`, ~3)

1. Parse com fields novos presentes.
2. Parse sem fields novos (regression P159A — apenas obrigatórios).
3. Tipo errado em field novo rejeitado com mensagem clara.

### 7.3 Layout E2E tests (`layout/tests.rs`, ~2)

1. Bibliography com entry completa renderiza formato extendido
   (todos os 4 fields novos presentes).
2. Bibliography com entry mínima renderiza formato P159A
   original (regression).

**Δ esperado**: +8 tests (3 + 3 + 2 = 8; range spec 5-8).

---

## 8. Constructor pattern (decisão específica §8)

### 8.1 Opções avaliadas

**Opção A**: Manter `new()` único com 4 args; fields novos via
field assignment directo `e.volume = Some(...)`.
- Simples; backwards compat trivial.
- Construtor incompleto — exige conhecimento dos campos.

**Opção B**: Adicionar `new_full(8 args)` constructor adicional.
- Backwards compat preserva `new(4 args)`.
- 8 args inline é verboso e propenso a erro de ordem.

**Opção C** (adoptada): Manter `new()` original + adicionar
métodos builder `with_volume(self, v) -> Self`, etc.
- Backwards compat preserva `new()`.
- Builder fluente legível: `BibEntry::new("k", "A", "T", 2024)
  .with_volume("12").with_pages("1-10")`.
- Preserva imutabilidade — cada `with_*` consome `self` e
  devolve nova `Self`.

### 8.2 Justificação

Builder pattern (Opção C) escolhido per:
- Legibilidade superior em tests.
- Backwards compat trivial.
- Padrão idiomático Rust.
- N=1 aplicação inicial (sem promoção a meta-padrão).

---

## 9. Selecção de 4 fields universais (decisão específica §9)

### 9.1 Critério de selecção (ADR-0065 #2)

**Universalidade**: presente em todas as principais styles
bibliográficas (APA/IEEE/MLA/Chicago/Vancouver/etc.).

**Cobertura**: cada field cobre uma classe importante de
publicações (journals, books, papers, manuals).

**Discriminação semântica**: distinto dos 4 originais (não
duplica `title`/`author`/`year`).

### 9.2 4 fields escolhidos

1. **`volume`** — universal em journals (e.g. "12"), proceedings,
   livros multi-volume.
2. **`pages`** — universal em qualquer publicação com paginação
   (e.g. "1-10", "23", "iii-vii").
3. **`journal`** — universal em artigos de journal; distingue
   semanticamente vs `title` em livros (livro: title=`A Brief
   History`, journal=None; artigo: title=`On Crystals`,
   journal=`Nature Communications`).
4. **`publisher`** — universal em livros, tech reports, manuals,
   proceedings (ACM, IEEE, MIT Press, etc.).

### 9.3 Alternativas consideradas

- **`url`/`doi`** — modernos e crescentemente importantes,
  mas mais comuns em formato digital. Diferidos para refino
  futuro N=4 (4+4+4 fields).
- **`editor`** — útil em proceedings/anthologies, mas não
  universal. Diferido.
- **`series`/`note`/`isbn`/`location`/`organization`** —
  específicos a sub-classes; menor universalidade. Diferidos.
- **`type`** — meta-classificação; útil mas requer enum
  separado (paridade P159C `CitationForm`). Diferido.

**Decisão registada** para refinos futuros: P159 sub-passo M
adicional pode adicionar `url`/`doi` como par natural.

---

## 10. Layout formato (decisão específica §10)

### 10.1 Ordem dos fields

Per convenção CSL semelhante a APA:
```
[key] author. title journal vol. volume, pp. pages. publisher (year).
```

Justificação:
- `journal` segue `title` (paridade APA: "Title. Journal").
- `vol.` e `pp.` seguem `journal` (modificadores do journal).
- `publisher` antes de `(year)` (paridade APA livros).
- `(year)` no final (paridade P159A — preserva backwards
  compat de prefixo).

### 10.2 Separadores

- `". "` entre title e journal (paridade APA).
- `" "` entre journal e `vol.` (espaço simples).
- `", "` entre volume e `pp.` (paridade APA).
- `". "` entre pages e publisher.
- `" "` entre publisher e `(year)`.

### 10.3 Backwards compat

Quando todos os 4 fields novos são `None`:
```
[key] author. title (year).
```
Formato P159A preservado exactamente — tests existentes
passam inalterados.

---

## Resumo executivo

P159D materializa **expansão de struct entity `BibEntry`**:
- 4 fields novos `Option<String>` (`volume`/`pages`/`journal`/
  `publisher`) per ADR-0065 critério #2.
- Builder pattern `with_volume()`/etc. (Opção C).
- Helper `extract_bib_entries` extendido para parsing dos 4
  fields opcionais.
- Helper privado novo `format_bib_entry` em layout para
  formatação extendida.

**Decisões arquitecturais P159D**:
- **Subset minimal 4 fields** (volume/pages/journal/publisher).
- **Builder pattern** para constructor extendido (legibilidade).
- **Backwards compat trivial** — fields novos default `None`
  preservam output P159A.
- **Layout formato APA-like** — ordem e separadores padronizados.

**Decisões diferidas (NÃO reservadas)**:
- Mais fields (`url`/`doi`/`editor`/etc.).
- Tipos estruturados (`PageRange`).
- Estilo de citação configurável (depende hayagriva).
- Promoção builder pattern a meta-padrão.

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0033: paridade observable estrutural.
- ADR-0054: graded scope-out de fields restantes.
- ADR-0060: refino qualitativo Fase 2 Model.
- ADR-0064: NÃO directamente aplicável (sem Smart<T>).
- ADR-0065: critério #2 terceira aplicação isolada concreta.

**Tests planeados**: Δ +8 (range spec 5-8).

**Risco**: baixo. Refino estrutural de entity sem alteração ao
variant Content + reuso de pattern `extract_bib_entries`.
Backwards compat trivial via Optional fields.
