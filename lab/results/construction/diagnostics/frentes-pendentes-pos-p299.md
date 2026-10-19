# Frentes pendentes pós-P299

**Data**: 2026-05-19
**Origem**: Passo 300 §3.A (caminho A consolidação).
**Propósito**: catálogo único de todas as frentes pendentes
registadas em relatórios P283-P299. Sem duplicação; agrupado
por categoria.

---

## 1. Frentes derivadas (sub-passos de feature)

### 1.1 — Footnote (P295 family)

| Frente | Magnitude | Estado | Bloqueador |
|---|---|---|---|
| **P295.1** nota corpo no rodapé | L | Registado P295 §8 | Requer 2-pass layout |
| **P295.2** overflow multi-página | M | Registado P295 §8 | Depende de P295.1 |
| **P295.X** footnote reference via `<label>` | XS+ | Registado P295 §8 | Bloqueado por scope methods stdlib |

### 1.2 — Curve (P293/P294 family)

| Frente | Magnitude | Estado | Bloqueador |
|---|---|---|---|
| **P-curve-scope-methods** sintaxe vanilla `curve.move(...)`/`curve.cubic(...)` | M | Registado P293 §9; P294 §9 | Bloqueado por scope methods stdlib |
| **P-native-path-svg** parser SVG path string | M | Registado P293 §9 | Frente vanilla separada |

### 1.3 — Cluster math (P296-P299 family)

| Frente | Magnitude | Estado | Bloqueador |
|---|---|---|---|
| **P296.X** toggles `inverted`/`cross` cancel | XS | Registado P296 §8 | Toggles binary não-cosméticos; refino directo |
| **P297.X** discriminator `UnderoverKind::Brace`/`Bracket`/`Paren`/`Shell` | XS+ | Registado P297 §8 | Refino visual fino (cosmético per ADR-0054 graded) |
| ~~**P298.X** operadores math pré-definidos~~ | ~~S-M~~ | **Resolvida P299** | — |
| **Auto-lookup math mode** (`$sin x$` sem prefix `math.`) | M | Registado P299 §8 | Requer modificação parser/eval math mode |

---

## 2. Cosméticos ADR-0054 graded (XS individual; XL agregado)

Scope-out consciente em múltiplos passos. Materialização individual
viável quando paridade vanilla específica for prioritária.

### 2.1 — Accent (P296)
- `size: Rel<Length>` (escalado relativo a base).
- `dotless: Smart<bool>` (remover ponto i/j ao colocar accent).

### 2.2 — Cancel (P296)
- `length: Rel<Length>` (comprimento linha relativo bbox).
- `angle: Smart<CancelAngle>` (rotação custom).
- `stroke: Stroke` (cor/espessura linha custom).

### 2.3 — Underover (P297)
- `style: Smart` (variantes brace/bracket/paren/shell — coberto por P297.X discriminator).

### 2.4 — Op (P298)
- Vanilla `OpElem` minimal — `text` + `limits`; sem extras cosméticos.

### 2.5 — Decorations (P284)
- `stroke: Stroke` refinos não-cobertos.
- `offset: Length` refinos.
- `extent: Length` refinos.

### 2.6 — Footnote (P295)
- `numbering: Numbering` (default `"1"`; arabic default implícito).

### 2.7 — SmartQuote (P287)
- `quotes: Smart<Option<EcoString>>` (delimitadores custom).
- `enabled: bool` (toggle global).

---

## 3. Frentes Style/Text (resolvidas P288-P292)

Sequência cumulativa P288-P292 fechou frentes principais:

| Frente | Estado | Passo |
|---|---|---|
| `text.lang` (en/pt/fr/it) | implementado P288 | P288 |
| `text.weight` (Bold/Regular) | implementado P289 | P289 |
| `text.tracking` (per-glyph Tc) | implementado P290 | P290 |
| `text.leading` (per-line peek) | implementado P291 | P291 |
| `text.font` (FontBook indirect) | implementado P292 | P292 |

**Refinos pendentes** (ADR-0054 graded; individuais XS-S):

| Refino | Categoria | Frente |
|---|---|---|
| `text.font` (string + dict combinations) | Style | DEBT-52 gap 8 |
| `text.weight` Bold variant-aware | Style | ADR-0055bis candidato |
| `text.lang` shaping (bidi/kern/lig) | Shaping | DEBT-53 (rustybuzz XL) |
| `text.region`/`text.script`/`text.dir` | Shaping | DEBT-53 |
| Soft hyphen (`\u{00AD}`) | Shaping | Passo dedicado |

---

## 4. Frentes em Tabelas A.6 (Model) ainda pendentes

| Feature | Estado | Notas |
|---|---|---|
| `bibliography(...)` cosméticos CSL | parcial | XL com hayagriva |
| `cite` style override | parcial | XL com CSL |
| `link` render visual | parcial | escopo S |
| `state(key, ...)` introspection runtime | parcial | implementado P208B+C |
| `here()` / `locate()` / `query()` | implementado | P208B+C; tabela C 387 corrigida P295 |
| `par` element wrapper | parcial | passo dedicado |
| `document(...)` metadata wrapper | ausente | passo dedicado |

---

## 5. Frentes Visualize ainda pendentes

| Feature | Estado | Frente |
|---|---|---|
| `gradient.radial(...)` | implementado P264-P265 | concêntrico subset |
| `gradient.conic(...)` | ausente | ADR-0087 não cobre |
| `tiling(...)` | ausente | passo dedicado |
| `stroke(...)` (object) `paint`/`dash`/`cap`/`join` | parcial | passos próprios |
| `cmyk(...)`, `oklab(...)`, etc. | ausente | space-specific |

---

## 6. Frentes Math ainda pendentes (não cobertas P296-P299)

| Feature | Estado | Notas |
|---|---|---|
| `mat(...)` matriz refino | implementado⁺ | tolerância visual; cobertura suficiente |
| `cases` refino | implementado⁺ | idem |
| `lr(...)` delimiters | implementado⁺ | extensíveis |
| `equation.numbering` | parcial | só block equations |
| **Math shaping completo** | parcial | ADR-0054 perfil graded; XL |

---

## 7. Frentes Foundations/Layout

| Feature | Estado | Notas |
|---|---|---|
| `Length` em `Stroke` (vs `f64`) | parcial | refino tipo |
| `Counter` machinery customisation | implementado P60+ | sub-features menores pendentes |
| `Introspector::query` extensões (Selector::And/Or/Where) | parcial | P175 minimal só Kind |

---

## 8. Sub-padrões cumulativos preservados como ferramentas

Não são "frentes" em sentido material — são padrões metodológicos
preservados como ferramentas discricionárias:

| Sub-padrão | N | Próxima reavaliação |
|---|---:|---|
| §8.7' A.0.0 template | 7 | P301+ se N=8 com valor inequívoco |
| §8.3 refutação pragmática | 11 | quando puder ser prescritivo (não descritivo) |
| §8.6 A.5' anti-reflexão | 10 | instrumental interno; não promove |
| "Variant rico" Option estrutural | 1 (P297 genuíno) | N≥3 genuínos |
| "Cluster math handler dedicado" | 3 ambíguo | nova aplicação substantiva |
| "Cross-variant interaction" | 1 (P298) | N≥3 |
| "Module namespaced" | 2 (P283+P299) | N=3 |
| "Operadores pré-definidos via SSoT" | 1 (P299) | N≥3 |

---

## 9. Bugs latentes monitorizados

### 9.1 — NBSP em SmartQuote (P287)
- **Fixed P290**: substituído `layout_content(Content::Text(...))`
  por `layout_word(glyph)`.

### 9.2 — Outros bugs
Sem outros bugs latentes críticos conhecidos pós-P299.

---

## 10. Categorias arquitecturais ainda abertas

### 10.1 — Bloqueado por scope methods stdlib (XL)
- `curve.move`/`curve.cubic`/`curve.quadratic` sintaxe vanilla fiel.
- `footnote(<label>)` reference.
- Vários outros user-facing `scope.method` patterns.

### 10.2 — Bloqueado por 2-pass layout (L+)
- P295.1 nota corpo no rodapé.
- P295.2 overflow multi-página.
- Outras features requerendo measurement antes de positioning.

### 10.3 — Bloqueado por shaping (DEBT-53; rustybuzz XL)
- `text.lang` bidi/kern/lig.
- `text.region`/`text.script`/`text.dir`.
- Soft hyphen.

### 10.4 — Cosméticos ADR-0054 graded (XS individual)
- Vários atributos opcionais em variants existentes.
- Acumuláveis em "P-XXX cosméticos cleanup" agregado se prioritário.

---

## 11. Total agregado

**Total frentes pendentes catalogadas**: ~30 frentes distintas
incluindo cosméticos individuais; ~10 frentes principais
(magnitude S-XL).

**Sem fragmentação**: cada frente registada uma vez neste catálogo;
referências cruzadas entre relatórios P283-P299 consolidadas aqui.

**Próximas prioridades sugeridas** (sem ordem fixa):
1. **P295.1 nota corpo no rodapé** — fecha cluster footnote.
2. **Auto-lookup math mode** — integração natural P299.
3. **P296.X toggles cancel** — refino cluster math.
4. **Cosméticos cleanup agregado** — múltiplos XS num passo.

**Decisão de prioridade fica ao operador humano**. P300 não dita
P301.
