# P421 — `repr()` Completo: representação de todos os variants (S)

> **Reclassificação retroativa (S→M):** a sonda A.0 revelou que `native_repr`
> não existia como infraestrutura completa; o passo foi reclassificado de S
> para M. O cabeçalho e a linha de bloqueadores abaixo descrevem o estado
> assumido antes da sonda. Ver ADR-0114 (gate sonda-antes-da-spec).

**Título**: repr() completo — match exaustivo de representação para todos os variants de Value, Content, Selector  
**Tipo**: Materialização (S) — consumer foundations + eval  
**Bloqueadores**: Nenhum externo; `native_repr` parcial/ausente — confirmado pela sonda A.0; ver ADR-0114  
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), P403 (Decimal), P405 (Duration), P406 (Version), P417 (Selector::Where), P418 (Bibliography/Cite)

---

## FASE A.0 — Sonda do substrato (obrigatória; 5 min)

Execute os 6 grep abaixo **antes de qualquer redação ou código**.  
**Critério de passagem**: todos os 6 produzem output conforme esperado. Se qualquer um falhar → **parar imediatamente** e reclassificar.

```bash
# 1. native_repr existe no projeto?
grep -rn "native_repr\|fn repr" 01_core/src/engine/stdlib/foundations.rs | head -10

# 2. Value enum tem repr() ou é matchado em native_repr?
grep -rn "Value::" 01_core/src/engine/stdlib/foundations.rs | grep -i "repr\|native" | head -20

# 3. Content enum tem repr() ou é matchado em native_repr?
grep -rn "Content::" 01_core/src/engine/stdlib/foundations.rs | head -20

# 4. Quais variants de Value NÃO têm repr?
grep -rn "enum Value" 01_core/src/entities/value.rs

# 5. Quais variants de Content NÃO têm repr?
grep -rn "enum Content" 01_core/src/entities/content.rs

# 6. Selector tem repr?
grep -rn "Selector" 01_core/src/engine/stdlib/foundations.rs | head -10
```

**Output esperado**:
1. ≥1 hit com `native_repr` ou `fn repr`
2. ≥0 hits (pode ser que `native_repr` já matcha alguns variants)
3. ≥0 hits (pode ser que `native_repr` já matcha alguns Content variants)
4. ≥1 hit com `enum Value` e lista de variants
5. ≥1 hit com `enum Content` e lista de variants
6. ≥0 hits (Selector pode não ter repr ainda)

**Se (1) falhar** → `native_repr` não existe; reclassificar para M (criar infraestrutura do zero).  
**Se (4) falhar** → `Value` enum não existe; reclassificar para XL (arquitetura quebrada).  
**Se (5) falhar** → `Content` enum não existe; reclassificar para XL.

**Análise dos gaps**: comparar a lista de variants em (4) e (5) com o que está coberto em (2) e (3). Os variants **não cobertos** são o escopo do P421.

---

## FASE A.1 — L0 (hash obrigatório)

**Documentar no L0** (`00_nucleo/prompts/engine/stdlib/foundations.md`):

### A.1.1 — Decisão arquitetural: paridade linguagem (ADR-0107)

No Typst vanilla, `repr(x)` é uma **construção linguística** de debugging/introspection:
- **Semântica**: produz uma string que representa o valor de forma legível e reproduzível (quando possível). Não é garantia de round-trip (`eval(repr(x)) == x`), mas é o objetivo aproximado.
- **Sintaxe**: `repr(value)` — função unária.
- **Morfologia**: cada tipo/variant tem uma representação canônica:
  - `repr(1)` → `"1"`
  - `repr(1.0)` → `"1.0"`
  - `repr("hello")` → `""hello""` (aspas escapadas)
  - `repr([hello])` → `"[hello]"` (colchetes)
  - `repr(heading)` → `"heading"` (nome do elemento)
  - `repr(heading(level: 1)[Title])` → `"heading(level: 1)[Title]"` (com fields e body)
  - `repr(none)` → `"none"`
  - `repr(auto)` → `"auto"`
  - `repr(1em)` → `"1em"`
  - `repr(rgb("#ff0000"))` → `"rgb("#ff0000")"` ou `"#ff0000"`

**O que NÃO é paridade (mecânica; diverge de propósito):**
- O formato exato das aspas (simples vs duplas, escaping de `"` vs `\`).
- A ordem dos fields em elementos (vanilla pode ordenar alfabeticamente; crystalline pode ordenar por declaração).
- A presença/ausência de espaços (`"heading(level: 1)"` vs `"heading(level:1)"`).
- O `repr()` de tipos internos não expostos ao usuário (ex.: `FrameItem`, `Region`) — scope-out.

### A.1.2 — Decisão arquitetural: atomização forma B (ADR-0109)

A lógica de `repr()` vive na **camada de eval/foundations**, não nos structs:

- `entities/value.rs` — `Value` enum; **nenhum método `repr()` no struct**.
- `entities/content.rs` — `Content` enum; **nenhum método `repr()` no struct**.
- `rules/stdlib/foundations.rs` — `native_repr` como free function que faz `match` exaustivo sobre `Value` e delega para free functions por variant (forma B).
- `rules/eval/repr.rs` — free functions `repr_value(v: &Value) -> String` e `repr_content(c: &Content) -> String` (forma B, novo arquivo se necessário).

**Não usar Opção A** (`impl Value { fn repr(&self) -> String }` em `entities/`) — cria lógica de representação no arquivo de dados (ADR-0109, rejeitado). O `repr()` é uma **operação de linguagem** (eval), não uma propriedade do dado.

### A.1.3 — Estratégia de implementação

| Tipo | Estado atual | Escopo P421 |
|------|-------------|-------------|
| `Value::Int` | Provavelmente já coberto | Verificar |
| `Value::Float` | Provavelmente já coberto | Verificar |
| `Value::Str` | Provavelmente já coberto | Verificar |
| `Value::Bool` | Provavelmente já coberto | Verificar |
| `Value::None` | Provavelmente já coberto | Verificar |
| `Value::Auto` | Provavelmente já coberto | Verificar |
| `Value::Content` | Provavelmente delega para `Content::repr` | Verificar |
| `Value::Array` | Provavelmente já coberto | Verificar |
| `Value::Dict` | Provavelmente já coberto | Verificar |
| `Value::Func` | Pode estar scope-out | Implementar se trivial |
| `Value::Args` | Pode estar scope-out | Implementar se trivial |
| `Value::Type` | Pode estar scope-out | Implementar se trivial |
| `Value::Module` | Pode estar scope-out | Implementar se trivial |
| `Value::Dyn` | Pode estar scope-out | Implementar se trivial |
| `Value::Decimal` | P403 | Verificar |
| `Value::Duration` | P405 | Verificar |
| `Value::Version` | P406 | Verificar |
| `Value::Selector` | P417 | Implementar |
| `Content::Text` | Provavelmente já coberto | Verificar |
| `Content::Styled` | Provavelmente já coberto | Verificar |
| `Content::Sequence` | Provavelmente já coberto | Verificar |
| `Content::Linebreak` | Provavelmente já coberto | Verificar |
| `Content::Space` | Provavelmente já coberto | Verificar |
| `Content::Parbreak` | Provavelmente já coberto | Verificar |
| `Content::Strong` | Provavelmente já coberto | Verificar |
| `Content::Emph` | Provavelmente já coberto | Verificar |
| `Content::Heading` | Pode estar incompleto | Completar fields |
| `Content::Figure` | Pode estar incompleto | Completar fields |
| `Content::Table` | Pode estar incompleto | Completar fields |
| `Content::List` | Pode estar incompleto | Completar fields |
| `Content::Enum` | Pode estar incompleto | Completar fields |
| `Content::Link` | Pode estar incompleto | Completar |
| `Content::Ref` | Pode estar incompleto | Completar |
| `Content::Footnote` | P304/P305 | Completar |
| `Content::Cite` | P418 | Implementar |
| `Content::Bibliography` | P418/P419/P420 | Implementar |
| `Content::Raw` | Pode estar incompleto | Completar |
| `Content::Math` | Pode estar incompleto | Completar |
| `Content::Lorem` | Pode estar incompleto | Completar |
| `Content::Image` | Pode estar incompleto | Completar |
| `Content::Line` | Pode estar incompleto | Completar |
| `Content::Rect` | Pode estar incompleto | Completar |
| `Content::Square` | Pode estar incompleto | Completar |
| `Content::Ellipse` | Pode estar incompleto | Completar |
| `Content::Circle` | Pode estar incompleto | Completar |
| `Content::Polygon` | Pode estar incompleto | Completar |
| `Content::Path` | Pode estar incompleto | Completar |
| `Content::Place` | Pode estar incompleto | Completar |
| `Content::Align` | Pode estar incompleto | Completar |
| `Content::Pad` | Pode estar incompleto | Completar |
| `Content::Block` | Pode estar incompleto | Completar |
| `Content::Box` | Pode estar incompleto | Completar |
| `Content::Stack` | Pode estar incompleto | Completar |
| `Content::Grid` | Pode estar incompleto | Completar |
| `Content::Columns` | Pode estar incompleto | Completar |
| `Content::Colbreak` | Pode estar incompleto | Completar |
| `Content::Pagebreak` | P156E | Completar |
| `Content::Repeat` | Pode estar incompleto | Completar |
| `Content::Move` | Pode estar incompleto | Completar |
| `Content::Scale` | Pode estar incompleto | Completar |
| `Content::Rotate` | Pode estar incompleto | Completar |
| `Content::Skew` | Pode estar incompleto | Completar |
| `Content::Hide` | Pode estar incompleto | Completar |
| `Content::Highlight` | Pode estar incompleto | Completar |
| `Content::Overline` | Pode estar incompleto | Completar |
| `Content::Underline` | Pode estar incompleto | Completar |
| `Content::Strike` | Pode estar incompleto | Completar |
| `Content::Smallcaps` | P408 | Completar |
| `Content::Upper` | Pode estar incompleto | Completar |
| `Content::Lower` | Pode estar incompleto | Completar |
| `Content::Datetime` | Pode estar incompleto | Completar |
| `Content::Symbol` | Pode estar incompleto | Completar |
| `Content::Equation` | Pode estar incompleto | Completar |
| `Content::Pattern` | Pode estar incompleto | Completar |
| `Content::Frame` | Pode estar incompleto | Completar |
| `Content::Meta` | Pode estar incompleto | Completar |
| `Content::Tag` | Pode estar incompleto | Completar |
| `Content::Styles` | Pode estar incompleto | Completar |

**Nota**: O P421 é S, mas a lista acima é longa. A estratégia é:
1. Verificar o que já está coberto (sonda A.0).
2. Implementar apenas os **faltantes** — não reescrever o que já funciona.
3. Se a lista de faltantes for > 20 variants, reclassificar para M.

### A.1.4 — Algoritmo de repr

Para cada variant de `Value`:
```rust
match value {
    Value::Int(v) => v.to_string(),
    Value::Float(v) => format!("{}", v), // ou com precisão controlada
    Value::Str(v) => format!(""{}"", v.escape_debug()), // aspas + escaping
    Value::Bool(v) => v.to_string(),
    Value::None => "none".to_string(),
    Value::Auto => "auto".to_string(),
    Value::Content(v) => repr_content(v),
    Value::Array(v) => format!("({})", v.iter().map(repr_value).join(", ")),
    Value::Dict(v) => format!("({})", v.iter().map(|(k,v)| format!("{}: {}", k, repr_value(v))).join(", ")),
    Value::Decimal(v) => format!("{}", v), // P403
    Value::Duration(v) => format!("{}", v), // P405
    Value::Version(v) => format!("{}", v), // P406
    Value::Selector(v) => repr_selector(v), // P417
    // ... outros
}
```

Para cada variant de `Content`:
```rust
match content {
    Content::Text(t) => format!(""{}"", t.text.escape_debug()),
    Content::Styled(s) => format!("{}.{}", repr_content(&s.child), repr_styles(&s.styles)),
    Content::Sequence(seq) => format!("[{}]", seq.iter().map(repr_content).join("")),
    Content::Linebreak => "linebreak".to_string(),
    Content::Space => "" "".to_string(), // ou "space"
    Content::Parbreak => "parbreak".to_string(),
    Content::Strong(s) => format!("*{}*", repr_content(&s.body)),
    Content::Emph(e) => format!("_{}_", repr_content(&e.body)),
    Content::Heading(h) => format!(
        "heading(level: {})[{}]",
        h.level,
        repr_content(&h.body)
    ),
    Content::Figure(f) => format!(
        "figure({})[{}]",
        repr_fields(&f), // ou omitir se vazio
        repr_content(&f.body)
    ),
    Content::Cite(c) => format!(
        "cite(<{}>)",
        c.key
    ), // ou @key
    Content::Bibliography(b) => format!(
        "bibliography("{}")",
        b.path.as_ref().map(|p| p.as_str()).unwrap_or("")
    ),
    // ... outros
}
```

**Princípio**: a representação deve ser **reconhecível** (o usuário identifica o que é), não necessariamente **reproduzível** (round-trip perfeito). Campos com valor default podem ser omitidos para brevidade.

### A.1.5 — Paridade vanilla

- Vanilla: `repr(heading)` → `"heading"`; `repr(heading(level: 1)[Title])` → `"heading(level: 1)[Title]"`; `repr(1em)` → `"1em"`; `repr(rgb("#ff0000"))` → `"rgb("#ff0000")"`.
- Cristalino P421: paridade aproximada (reconhecível, não byte-exata). Aspas, escaping, e ordem de fields podem divergir levemente.

### A.1.6 — Scope-out explícito

- `repr()` de tipos internos (`FrameItem`, `Region`, `Layouter`, `Frame`) — scope-out; não são `Value`/`Content`.
- `repr()` de `Func` com closure captures — scope-out; mostrar apenas nome da função.
- `repr()` de `Module` com todos os exports — scope-out; mostrar apenas nome do módulo.
- `repr()` de `Dyn` (tipo dinâmico) — scope-out; mostrar apenas type name.
- Round-trip perfeito (`eval(repr(x)) == x`) — não é garantido; scope-out.
- Representação de fields com valor default — podem ser omitidos; scope-out de exaustividade.

---

## CHECKPOINT A

**Só prosseguir para Fase B quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `foundations.md`
2. Sonda A.0 produziu 6/6 OK
3. Lista de variants faltantes identificada e contada
4. Se faltantes > 20 → reclassificar para M

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Código

### B.1 — Eval / Foundations (`rules/stdlib/foundations.rs` ou `rules/eval/repr.rs`)

1. **Verificar `native_repr` existente**:
   - Se já existe e cobre alguns variants: preservar; adicionar arms faltantes.
   - Se não existe: criar `fn native_repr(args: Args) -> Result<Value, EvalError>`.

2. **Implementar `repr_value(v: &Value) -> String`** (free function, forma B):
   ```rust
   pub(super) fn repr_value(v: &Value) -> String {
       match v {
           Value::Int(i) => i.to_string(),
           Value::Float(f) => {
               // Evitar "1.0" vs "1" ambiguidade; usar format que mostra decimal
               let s = format!("{}", f);
               if s.contains('.') { s } else { format!("{}.0", s) }
           }
           Value::Str(s) => format!(""{}"", s.escape_debug()),
           Value::Bool(b) => b.to_string(),
           Value::None => "none".to_string(),
           Value::Auto => "auto".to_string(),
           Value::Content(c) => repr_content(c),
           Value::Array(arr) => {
               let items: Vec<String> = arr.iter().map(repr_value).collect();
               format!("({})", items.join(", "))
           }
           Value::Dict(dict) => {
               let items: Vec<String> = dict.iter()
                   .map(|(k, v)| format!("{}: {}", k, repr_value(v)))
                   .collect();
               format!("({})", items.join(", "))
           }
           Value::Decimal(d) => d.to_string(), // P403
           Value::Duration(d) => d.to_string(), // P405
           Value::Version(ver) => ver.to_string(), // P406
           Value::Selector(sel) => repr_selector(sel), // P417
           Value::Func(_) => "function".to_string(), // scope-out detalhes
           Value::Args(_) => "arguments".to_string(), // scope-out detalhes
           Value::Type(t) => format!("{}", t.name()), // ou similar
           Value::Module(m) => format!("module {}", m.name()), // ou similar
           Value::Dyn(d) => format!("{}", d.type_name()), // scope-out detalhes
       }
   }
   ```

3. **Implementar `repr_content(c: &Content) -> String`** (free function, forma B):
   ```rust
   pub(super) fn repr_content(c: &Content) -> String {
       match c {
           Content::Text(t) => format!(""{}"", t.text.escape_debug()),
           Content::Styled(s) => {
               let child = repr_content(&s.child);
               let styles = repr_styles(&s.styles);
               if styles.is_empty() { child } else { format!("{}.{}", child, styles) }
           }
           Content::Sequence(seq) => {
               let inner: String = seq.iter().map(repr_content).collect();
               format!("[{}]", inner)
           }
           Content::Linebreak => "linebreak".to_string(),
           Content::Space => "" "".to_string(),
           Content::Parbreak => "parbreak".to_string(),
           Content::Strong(s) => format!("*{}*", repr_content(&s.body)),
           Content::Emph(e) => format!("_{}_", repr_content(&e.body)),
           Content::Heading(h) => {
               let body = repr_content(&h.body);
               if h.level == 1 {
                   format!("= {}", body.trim_matches('"'))
               } else {
                   format!("heading(level: {})[{}]", h.level, body)
               }
           }
           Content::Figure(f) => {
               let body = repr_content(&f.body);
               format!("figure[{}]", body)
           }
           Content::Table(t) => {
               let rows = t.rows.len();
               let cols = t.cols.len();
               format!("table(rows: {}, cols: {})[...]", rows, cols)
           }
           Content::List(l) => {
               let items: Vec<String> = l.children.iter().map(repr_content).collect();
               format!("list[{}]", items.join(", "))
           }
           Content::Enum(e) => {
               let items: Vec<String> = e.children.iter().map(repr_content).collect();
               format!("enum[{}]", items.join(", "))
           }
           Content::Link(l) => format!("link("{}")", l.url),
           Content::Ref(r) => format!("ref(<{}>)", r.target),
           Content::Footnote(f) => {
               let body = f.body.as_ref().map(repr_content).unwrap_or_default();
               format!("footnote[{}]", body)
           }
           Content::Cite(c) => format!("cite(<{}>)", c.key),
           Content::Bibliography(b) => {
               let path = b.path.as_ref().map(|p| p.as_str()).unwrap_or("");
               format!("bibliography("{}")", path)
           }
           Content::Raw(r) => format!("```{}```", r.text),
           Content::Math(m) => format!("${}$", m.text),
           Content::Lorem(l) => format!("lorem({})", l.words),
           Content::Image(i) => format!("image("{}")", i.path),
           Content::Line(_) => "line".to_string(),
           Content::Rect(_) => "rect".to_string(),
           Content::Square(_) => "square".to_string(),
           Content::Ellipse(_) => "ellipse".to_string(),
           Content::Circle(_) => "circle".to_string(),
           Content::Polygon(_) => "polygon".to_string(),
           Content::Path(_) => "path".to_string(),
           Content::Place(_) => "place".to_string(),
           Content::Align(a) => format!("align({})[...]", a.align),
           Content::Pad(_) => "pad".to_string(),
           Content::Block(_) => "block".to_string(),
           Content::Box(_) => "box".to_string(),
           Content::Stack(_) => "stack".to_string(),
           Content::Grid(_) => "grid".to_string(),
           Content::Columns(_) => "columns".to_string(),
           Content::Colbreak => "colbreak".to_string(),
           Content::Pagebreak => "pagebreak".to_string(),
           Content::Repeat(_) => "repeat".to_string(),
           Content::Move(_) => "move".to_string(),
           Content::Scale(_) => "scale".to_string(),
           Content::Rotate(_) => "rotate".to_string(),
           Content::Skew(_) => "skew".to_string(),
           Content::Hide(_) => "hide".to_string(),
           Content::Highlight(_) => "highlight".to_string(),
           Content::Overline(_) => "overline".to_string(),
           Content::Underline(_) => "underline".to_string(),
           Content::Strike(_) => "strike".to_string(),
           Content::Smallcaps(_) => "smallcaps".to_string(), // P408
           Content::Upper(_) => "upper".to_string(),
           Content::Lower(_) => "lower".to_string(),
           Content::Datetime(d) => format!("datetime({})", d.value),
           Content::Symbol(s) => format!("sym.{}.{}", s.name, s.variant), // ou similar
           Content::Equation(e) => format!("$${}$$", e.text), // ou similar
           Content::Pattern(_) => "pattern".to_string(),
           Content::Frame(_) => "frame".to_string(),
           Content::Meta(_) => "meta".to_string(),
           Content::Tag(_) => "tag".to_string(),
           Content::Styles(_) => "styles".to_string(),
       }
   }
   ```

4. **Implementar `repr_selector(sel: &Selector) -> String`** (free function, forma B):
   ```rust
   pub(super) fn repr_selector(sel: &Selector) -> String {
       match sel {
           Selector::Elem(kind) => format!("{}", kind.name()),
           Selector::Label(l) => format!("<{}>", l),
           Selector::Regex(r) => format!("regex("{}")", r.as_str()),
           Selector::Where { base, field, value } => {
               format!("{}.where({}: {})",
                   repr_selector(base),
                   field,
                   repr_value(value)
               )
           }
           Selector::Or(a, b) => format!("{} | {}", repr_selector(a), repr_selector(b)),
           Selector::And(a, b) => format!("{} & {}", repr_selector(a), repr_selector(b)),
           // ... outros
       }
   }
   ```

### B.2 — Tests

**Mínimo 10 tests**:
- 2 unit `repr_value`: `Int`/`Float`/`Str`/`Bool`/`None`/`Auto`/`Array`/`Dict`/`Decimal`/`Duration`/`Version`/`Selector`
- 2 unit `repr_content`: `Text`/`Sequence`/`Linebreak`/`Space`/`Parbreak`/`Strong`/`Emph`/`Heading`/`Figure`/`Cite`/`Bibliography`/`Link`/`Footnote`/`List`/`Enum`/`Table`/`Image`/`Raw`/`Math`/`Lorem`/`Line`/`Rect`/`Square`/`Ellipse`/`Circle`/`Polygon`/`Path`/`Place`/`Align`/`Pad`/`Block`/`Box`/`Stack`/`Grid`/`Columns`/`Colbreak`/`Pagebreak`/`Repeat`/`Move`/`Scale`/`Rotate`/`Skew`/`Hide`/`Highlight`/`Overline`/`Underline`/`Strike`/`Smallcaps`/`Upper`/`Lower`/`Datetime`/`Symbol`/`Equation`/`Pattern`/`Frame`/`Meta`/`Tag`/`Styles`
- 3 unit `repr_selector`: `Elem`, `Label`, `Regex`, `Where`, `Or`, `And`
- 3 E2E: `repr(1)`, `repr("hello")`, `repr([hello world])`, `repr(heading(level: 1)[Title])`, `repr(cite(<key>))`, `repr(bibliography("refs.bib"))`

---

## FASE C — Validação

```bash
cargo test -p typst-core --lib -- repr
# → 10 passed; 0 failed; 0 ignored

crystalline-lint .
# → 0 errors
# → 0 drift nos prompts tocados
```

**Critério de fecho**:
- [ ] 10 tests verdes
- [ ] Lint zero errors; drift sincronizado
- [ ] `repr(1)` → `"1"`
- [ ] `repr(1.0)` → `"1.0"`
- [ ] `repr("hello")` → `""hello""`
- [ ] `repr([hello])` → `"[hello]"` (ou equivalente reconhecível)
- [ ] `repr(heading(level: 1)[Title])` → reconhecível como heading
- [ ] `repr(cite(<key>))` → reconhecível como cite
- [ ] `repr(bibliography("refs.bib"))` → reconhecível como bibliography
- [ ] `repr(selector)` → reconhecível para todos os variants
- [ ] Nenhum vtable/`dyn` introduzido
- [ ] `match` exaustivo preservado (todos os variants de Value, Content, Selector cobertos)
- [ ] Lógica atomizada em free functions (forma B)
- [ ] L0 hashado e propagado

---

## Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A.0 identifica exatamente quais variants estão faltantes. Se a lista for > 20, reclassificar para M. Não assumir que é S antes de medir.
- **Paridade linguagem (ADR-0107)**: `repr()` é debugging — a paridade é "reconhecível", não "byte-exata". Aspas, escaping, e ordem de fields são mecânica livre.
- **Atomização (ADR-0109)**: `repr_value`, `repr_content`, `repr_selector` são free functions na camada de eval/foundations. Nenhum método adicionado a `Value`, `Content`, ou `Selector`.
- **Honestidade epistêmica**: Se algum variant for complexo demais para `repr()` (ex.: `Func` com closure, `Module` com exports), scope-out honestamente com `"function"` ou `"module"`.
- **Próximo passo P422**: `link` render visual (S) ou `text.lang` rustybuzz (XL, scope-out) ou outro gap identificado no Inventário.
