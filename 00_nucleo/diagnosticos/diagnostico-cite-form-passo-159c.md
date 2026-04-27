# Diagnóstico Cite.form — Passo P159C

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
segundo sub-passo Bibliography + Cite (Bloco A do diagnóstico
P159B §3.2). **Décima oitava aplicação consecutiva** do padrão
diagnóstico-primeiro.

Refino estrutural-comportamental: enum `CitationForm` novo +
field `form: Option<CitationForm>` em `Content::Cite`. Aplica
**ADR-0064 Caso A** patamar N=5 → 6. **Quebra de padrão hash
content.rs** (11 passos consecutivos terminam).

---

## 1. Assinatura vanilla `CiteElem.form`

Per `lab/typst-original/crates/typst-library/src/model/cite.rs`
(quarentena), vanilla `CiteElem` campo `form: Smart<Option<CiteForm>>`
com default `Smart::Auto`. Enum `CiteForm`:
- `Normal` (default; `[Author Year]`)
- `Prose` (`Author (Year)`)
- `Author` (apenas autor)
- `Year` (apenas ano)

Forms vanilla adicionais: `Full` (entry completa inline), forms
CSL específicas (não-universais). **Subset minimal P159C**:
4 forms universais.

Fallback `Smart::Auto` resolve via:
1. Estilo CSL activo (cristalino N/A — sem CSL).
2. Default `Normal`.

---

## 2. Comportamento observável

**Vanilla**:
- `cite("smith")` (form auto/Normal) → `[Smith 2024]`.
- `cite("smith", form: "prose")` → `Smith (2024)`.
- `cite("smith", form: "author")` → `Smith`.
- `cite("smith", form: "year")` → `2024`.

**Cristalino actual** (`layout/mod.rs:557`):
```rust
Content::Cite { key, supplement } => {
    let placeholder = format!("[{}]", key);
    self.layout_content(&Content::text(placeholder));
    if let Some(s) = supplement { self.layout_content(s); }
}
```
Placeholder `[key]` independente de form (form não existe).

**Cristalino P159C** (proposto):
- `Normal` (ou None default): `[key]` + supplement (paridade
  P159A; inalterado).
- `Prose`: lookup entry; render `Author (Year)` + supplement;
  fallback `[key]` se key não encontrada em Bibliography activa.
- `Author`: lookup; render `Author` + supplement; fallback `[key]`.
- `Year`: lookup; render `Year` + supplement; fallback `[key]`.

**Decisão arquitectural-chave §1**: render é **placeholder
melhorado**, não CSL real. Suficiente para distinguir os 4 forms
em testes; refino CSL futuro depende hayagriva (ADR-0062).

---

## 3. ADR-0064 caso aplicável

**Caso A** directamente aplicável: `Smart<Option<CiteForm>>`
(vanilla 2-níveis Smart) → `Option<CitationForm>` (cristalino
1-nível). Achatamento: `Smart::Auto` ↔ `None` (resolvido em
layout para Normal).

**Patamar Caso A pós-P159C**: **N=5 → 6**.
- P156G Block.width
- P156H Box.width
- P156I Stack.spacing
- P157B TableCell.x/y
- P159A Bibliography.title + Cite.supplement (par)
- **P159C Cite.form** ← novo

Distribuição cross-domínio: 50% Layout (3) + 50% Model (3) —
**equilíbrio cross-domínio** atingido.

---

## 4. Variants Content existentes a estender

**`Content::Cite`** — expansão de field. Estrutura actual
(`content.rs:557`):
```rust
Cite {
    key:        String,
    supplement: Option<Box<Content>>,
}
```

P159C adiciona terceiro field:
```rust
Cite {
    key:        String,
    supplement: Option<Box<Content>>,
    form:       Option<CitationForm>,  // P159C novo
}
```

**Quebra hash content.rs esperada** — primeiro break em série
P156L → P158B (11 consecutivos preservaram).

**Sítios de pattern-match a actualizar** (audit completo via
grep `Content::Cite`):
1. `entities/content.rs:557` — variant declaration.
2. `entities/content.rs:836` — construtor `Self::cite(...)`.
3. `entities/content.rs:880` — `is_empty()` (Self::Cite { .. } =>
   false; **wildcard preserva, mas listagem agora tem 3 fields
   reais**).
4. `entities/content.rs:1031` — `plain_text()` (`{ key, supplement }`
   → `{ key, supplement, form }`; ignora form em plain_text).
5. `entities/content.rs:1171` — `PartialEq` (compara 3 fields agora).
6. `entities/content.rs:1473` — `map_content` (preserva form).
7. `entities/content.rs:1683` — `map_text` (preserva form).
8. `entities/content.rs:3411,3422` — testes pattern-match (2
   sítios; adicionar form na destructuring).
9. `rules/introspect.rs:216` — materialize_time (preserva form).
10. `rules/introspect.rs:454` — walk (`{ supplement, .. }` —
    wildcard preserva).
11. `rules/layout/mod.rs:557` — layout_content arm (expande por form).
12. `rules/stdlib/structural.rs:694` — construtor `Content::Cite { ... }`.
13. `rules/stdlib/mod.rs:2569,2583` — testes pattern-match (2
    sítios; destructuring update).

**Total**: 13 sítios a actualizar. Comparável a P157A (Table) e
P159A (par Bibliography+Cite); bem dentro do envelope normal.

---

## 5. Helpers stdlib reusáveis

### 5.1 Helper novo `extract_citation_form`

Localização: `rules/stdlib/structural.rs` (privado em ficheiro)
ou ajustado conforme convenção. Análogo a `extract_bool_with_default`
(P157C) — parsing de string para enum custom com validação hard.

```rust
fn extract_citation_form(val: Option<&Value>) -> SourceResult<Option<CitationForm>> {
    match val {
        None | Some(Value::Auto) | Some(Value::None) => Ok(None),
        Some(Value::Str(s)) => match s.as_str() {
            "normal" => Ok(Some(CitationForm::Normal)),
            "prose"  => Ok(Some(CitationForm::Prose)),
            "author" => Ok(Some(CitationForm::Author)),
            "year"   => Ok(Some(CitationForm::Year)),
            other    => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("cite(): form '{}' inválido (válidos: normal, prose, author, year)", other),
            )]),
        },
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("cite(): form espera string, recebeu {}", other.type_name()),
        )]),
    }
}
```

**N=1** sem candidato a reuso ainda; promoção diferida per
política consistente N=3-4.

### 5.2 Lookup Bibliography ↔ Cite (decisão §9)

Decidida em §9 abaixo.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P159C | Refino futuro |
|---------|--------------|---------------|
| 4 forms universais (Normal/Prose/Author/Year) | ✓ implementado | forms vanilla adicionais (Full, CSL-specific) NÃO reservados |
| Render placeholder melhorado por form | ✓ implementado | CSL real depende hayagriva (ADR-0062) |
| Lookup same-document | ✓ implementado | Cross-document refs bloqueado por ADR-0017 |
| `style: Str` per-Cite | ✗ scope-out | depende hayagriva |
| Validação hard form inválido | ✓ implementado | mensagem lista forms válidas |
| Case-insensitive matching | ✗ scope-out | strict matching mais previsível |

---

## 7. Tests planeados

### 7.1 Unit tests `CitationForm` (`entities/citation_form.rs`, ~3)

1. Constructor cada variant (4 variants).
2. PartialEq.
3. Default = Normal.

### 7.2 Unit tests `Content::Cite` com form (`entities/content.rs`, ~2)

1. Constructor com form=Some / form=None.
2. PartialEq cobre 3 fields.

(map_text e map_content já testados via PartialEq + outros tests.)

### 7.3 Stdlib tests (`stdlib/mod.rs`, ~6)

1. Parse "normal" → Some(Normal).
2. Parse "prose" → Some(Prose).
3. Parse "author" → Some(Author).
4. Parse "year" → Some(Year).
5. form=auto → None.
6. form="invalid" rejeitado com mensagem listando forms válidas.

### 7.4 Layout E2E tests (`layout/tests.rs`, ~4)

1. `cite_normal_renderiza_placeholder` (regression P159A).
2. `cite_prose_renderiza_author_year_quando_key_existe`.
3. `cite_prose_fallback_placeholder_quando_key_nao_existe`.
4. `cite_author_e_year_renderizam_correctamente` (combinado).

**Δ esperado**: +15 tests (3 + 2 + 6 + 4 = 15; range 12-17 da spec).

---

## 8. Localização do enum (decisão específica §8)

Per padrão consolidado N=4 ("tipo entity em ficheiro próprio"):
- `entities/sides.rs` (P156C)
- `entities/parity.rs` (P156E)
- `entities/dir.rs` (P156I)
- `entities/bib_entry.rs` (P159A)

**P159C adiciona N=5**: `entities/citation_form.rs` (ficheiro
novo; enum dedicado em ficheiro próprio, não inline em
content.rs ou bib_entry.rs).

`pub mod citation_form;` em `entities/mod.rs` (após `bib_entry`
em ordem alfabética; ou após `parity` em ordem semântica).

---

## 9. Algoritmo lookup Cite ↔ Bibliography (decisão específica §9)

### 9.1 Estado actual

`layout/mod.rs:557` — pattern arm `Content::Cite` é executado
durante walk single-pass do Layouter. Não tem acesso a estado
global Bibliography.

### 9.2 Opções avaliadas

**Opção A**: First-pass collect Bibliography entries → second-pass
render Cite com lookup. **Rejeitada**: requer modificação
arquitectural significativa do Layouter (segundo pass específico).

**Opção B**: Passar Bibliography activa via Layouter state
(`self.active_bibliography: Option<&[BibEntry]>`). **Rejeitada**:
Layouter já é stateful complexo; mais um campo cresce superfície.

**Opção C** (adoptada): **Lookup via CounterState** — adicionar
field `pub bib_entries: Vec<BibEntry>` em `CounterState` populado
durante introspect walk (encontra `Content::Bibliography`,
copia entries para state). Layouter já recebe `state` (paridade
P158B `state.lang`). Lookup é `state.bib_entries.iter().find(...)`.

**Justificação Opção C**:
- Reusa infraestrutura `state` (paridade P158B padrão).
- Sem segundo pass — introspect já faz uma pass.
- Compatível com walk single-pass actual (introspect existente
  já visita `Bibliography` em `materialize_time` se aplicável).
- Multi-Bibliography: usa todas as encontradas (concatenadas em
  ordem de aparecimento).

### 9.3 Implementação

`introspect.rs::walk` arm para `Bibliography` adiciona entries:
```rust
Content::Bibliography { entries, .. } => {
    state.bib_entries.extend(entries.iter().cloned());
    // recurse em title (já existente via materialize_time)
}
```

`layout/mod.rs::Cite` arm faz lookup:
```rust
let entry = state.bib_entries.iter().find(|e| e.key == *key);
let form  = form.unwrap_or_default();  // Normal default
let text  = match (form, entry) {
    (CitationForm::Normal, _) => format!("[{}]", key),
    (CitationForm::Prose, Some(e)) => format!("{} ({})", e.author, e.year),
    (CitationForm::Author, Some(e)) => e.author.clone(),
    (CitationForm::Year, Some(e)) => e.year.to_string(),
    (_, None) => format!("[{}]", key),  // fallback
};
```

**Limitação aceite**: Layouter não tem acesso a `state` directo
em todos os arms. Se for este o caso, ajustar para passar `state`
via parâmetro auxiliar ou usar `self.state` se já existe.

### 9.4 Verificar acesso state em Layouter

A confirmar em sub-passo .5: como Layouter acede a CounterState
durante `layout_content`. Se já tem `self.state`, usar directo.
Se não, considerar abordagem alternativa (e.g. passar `bib_entries`
como parâmetro para `layout_cite_arm`).

---

## 10. Quebra padrão hash content.rs (decisão específica §10)

**11 passos consecutivos** preservaram `entities/content.rs`
hash `ec58d849` (P156L → P158B). P159C **quebra explicitamente**
este padrão por expansão de field em `Content::Cite`.

**Reset do contador**: pós-P159C, contador hash content.rs
reinicia a 0; novo hash a propagar via `crystalline-lint --fix-hashes`.

**Subpadrão emergente**: "expansão de field em variant Content
já existente". Precedentes: P156F skew (TransformMatrix expandido);
P156L pad sides (Pad expandido); **P159C cite.form** (Cite
expandido). N=3; promoção a subpadrão consolidado diferida até N=4.

---

## Resumo executivo

P159C materializa **enum `CitationForm` + field `form` em
`Content::Cite`**:
- Enum novo `CitationForm { Normal, Prose, Author, Year }` em
  `entities/citation_form.rs` (5ª aplicação de "tipo entity em
  ficheiro próprio").
- Field `form: Option<CitationForm>` em `Content::Cite` (ADR-0064
  Caso A; patamar N=6).
- 13 sítios pattern-match Content actualizados.
- Helper privado `extract_citation_form` em stdlib/structural.rs.
- Layout placeholder melhorado por form (Prose/Author/Year via
  lookup Bibliography state).
- Field novo `pub bib_entries: Vec<BibEntry>` em CounterState
  para lookup (paridade infraestrutural P158B `state.lang`).

**Decisões arquitecturais P159C**:
- **Subset minimal 4 forms** (Normal/Prose/Author/Year).
- **Lookup via CounterState** (Opção C; reusa infraestrutura).
- **Fallback `[key]`** quando key não encontrada (paridade
  P159A; Normal sem entry produz mesmo output).
- **Strict matching** (case-sensitive; sem abreviações).

**Decisões diferidas (NÃO reservadas)**:
- Forms vanilla adicionais (Full, CSL-specific).
- CSL render real (depende hayagriva ADR-0062).
- `style: Str` per-Cite (depende hayagriva).
- Cross-document refs (bloqueado ADR-0017).
- Promoção `extract_citation_form` a helper público.

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido.
- ADR-0033: paridade observable estrutural (placeholder
  melhorado).
- ADR-0054: graded scope-out de forms/style/CSL.
- ADR-0060: refino qualitativo Fase 2 Model.
- ADR-0064: Caso A patamar N=5 → 6.
- ADR-0065: critério #5 (scope) implícito; critério #2 (escolha
  de tipo enum vs Option<String>).

**Tests planeados**: Δ +15 (range spec 12-17).

**Risco**: baixo-médio. Refino aditivo de variant existente +
quebra de padrão hash + decisão de lookup arquitectural. ADR-0064
Caso A já validado N=5; risco de ambiguidade arquitectural baixo.
