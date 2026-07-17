# P418 — Bibliography/Cite CSL: renderização de referências bibliográficas (XL)

**Título**: Bibliography/cite CSL — parsing de bib + CSL + renderização de citações e lista de referências  
**Tipo**: Materialização (XL) — consumer Bibliography + Cite + infraestrutura de loading CSL + hayagriva  
**Bloqueadores**: Nenhum externo; pré-condições internas verificáveis; dependência externa `hayagriva` a confirmar  
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), P248 (show rules), P417 (Selector::Where), DEBT-52 (text.font fechado), P295 (footnote)

---

## FASE A.0 — Sonda do substrato (obrigatória; 10 min)

Execute os 12 grep abaixo **antes de qualquer redação ou código**.  
**Critério de passagem**: 10/12 mínimo; itens 11 e 12 são nice-to-have. Se qualquer um dos 10 obrigatórios falhar → **parar imediatamente** e reclassificar o passo.

```bash
# 1. BibEntry tipo existe no projeto?
grep -rn "struct BibEntry\|enum BibEntry\|BibEntry" 01_core/src/entities/ | head -20

# 2. loading module existe para bibliografia?
grep -rn "mod loading\|loading::" 01_core/src/ | head -20

# 3. Cite elemento existe (Content::Cite)?
grep -rn "Content::Cite\|Cite" 01_core/src/entities/content.rs | head -20

# 4. Bibliography elemento existe (Content::Bibliography)?
grep -rn "Content::Bibliography\|Bibliography" 01_core/src/entities/content.rs | head -20

# 5. hayagriva está no Cargo.toml?
grep -rn "hayagriva" Cargo.toml 01_core/Cargo.toml 2>/dev/null

# 6. CSL style file loading existe (.csl, .xml)?
grep -rn "\.csl\|csl\|style" 01_core/src/engine/loading/ 2>/dev/null | head -20

# 7. Label/Ref infrastructure existe (para citações cruzadas)?
grep -rn "Label\|Ref\|label\|reference" 01_core/src/entities/content.rs | head -20

# 8. Counter infrastructure existe (para numeração de citações)?
grep -rn "Counter\|counter" 01_core/src/engine/layout/mod.rs | head -10

# 9. Show rule infrastructure aceita novos elementos (Bibliography, Cite)?
grep -rn "ShowRule\|show_rules" 01_core/src/engine/eval/rules.rs | head -20

# 10. File loading / path resolution existe (para .bib, .yaml, .json)?
grep -rn "load\|resolve\|path" 01_core/src/engine/loading/ 2>/dev/null | head -20

# 11. [NICE-TO-HAVE] Query infrastructure para citações (Introspector::query)?
grep -rn "Introspector\|query" 01_core/src/entities/introspector.rs | head -10

# 12. [NICE-TO-HAVE] State infrastructure para acumular citações (State::update)?
grep -rn "State\|state" 01_core/src/engine/eval/state.rs 2>/dev/null | head -10
```

**Output esperado**:
1. ≥1 hit com `BibEntry` (struct ou tipo)
2. ≥1 hit com módulo de loading
3. ≥1 hit com `Cite` ou `Content::Cite`
4. ≥1 hit com `Bibliography` ou `Content::Bibliography`
5. ≥1 hit com `hayagriva` em Cargo.toml (se não existir, decisão de adicionar dependência)
6. ≥0 hits (CSL loading pode não existir ainda)
7. ≥1 hit com `Label` ou `Ref` infrastructure
8. ≥1 hit com `Counter` ou `counter` no layout
9. ≥1 hit com `ShowRule` infrastructure
10. ≥1 hit com loading/resolução de arquivos
11. ≥0 hits (query é nice-to-have)
12. ≥0 hits (state é nice-to-have)

**Se (1) falhar** → `BibEntry` não existe; reclassificar para L (criar tipo do zero).  
**Se (3) falhar** → `Cite` não existe no enum Content; reclassificar para L (adicionar variant).  
**Se (4) falhar** → `Bibliography` não existe no enum Content; reclassificar para L.  
**Se (7) falhar** → Label/Ref infrastructure ausente; reclassificar para L (citações dependem de labels).  
**Se (9) falhar** → Show rules não existem; reclassificar para XL (reabrir infraestrutura de show rules).  
**Se (10) falhar** → File loading não existe; reclassificar para L (bibliografia requer carregar .bib/.yaml).

**Se (5) falhar** (hayagriva não em Cargo.toml):  
→ **Decisão a tomar**: adicionar `hayagriva` como dependência, ou implementar parser próprio de BibTeX/BibLaTeX + CSL engine?  
→ **Critério ADR-0107**: paridade é com a linguagem (`.bib` → citações renderizadas), não com a mecânica (hayagriva vs parser próprio).  
→ **Critério ADR-0108**: medir o custo. Hayagriva é ~15k LOC de parsing + CSL. Parser próprio é >50k LOC.  
→ **Decisão recomendada**: adicionar `hayagriva` ao Cargo.toml. Se não for possível (licença, compatibilidade), reclassificar para XXL (implementar parser próprio).

---

## FASE A.1 — L0 (hash obrigatório)

**Documentar no L0** (`00_nucleo/prompts/entities/bibliography.md` + `cite.md` + `loading.md`):

### A.1.1 — Decisão arquitetural: paridade linguagem (ADR-0107)

No Typst vanilla, `#bibliography("refs.bib")` + `@key` + `#cite(<key>)` é uma **construção linguística** de citação bibliográfica. A paridade é:

- **Semântica**: `@key` produz uma citação no texto (formato depende do CSL style); `#bibliography("refs.bib")` produz a lista de referências no final (ou onde inserido); citações não referenciadas não aparecem na lista; a ordem da lista segue o CSL style.
- **Sintaxe**: `@key` (cite inline), `@key[p. 12]` (cite com supplement), `#cite(<key>)` (cite funcional), `#bibliography("refs.bib", style: "ieee")` (bibliography com CSL style).
- **Morfologia**: `Cite` e `Bibliography` são objetos da linguagem; o conteúdo de `Cite` é a chave + supplement opcional; o conteúdo de `Bibliography` é o path do arquivo + style opcional.

**O que NÃO é paridade (mecânica; diverge de propósito — ADR-0107 / P329):**
- O formato exato dos bytes de saída (CSL pode produzir leves diferenças de espaçamento entre implementações).
- A estrutura interna de dados do hayagriva (usamos o crate, mas a estrutura interna dele é irrelevante para paridade).
- A ordem de passes (vanilla faz multi-passe para resolver citações forward-reference; crystalline pode fazer em ordem de definição ou deferred resolution).
- O `==` mecânico entre `Cite` ou `Bibliography`.

### A.1.2 — Decisão arquitetural: hayagriva vs parser próprio (ADR-0108)

| Opção | Descrição | Magnitude | Risco | Nota |
|-------|-----------|-----------|-------|------|
| **α** — Hayagriva como dependência | Usar `hayagriva` crate (mesmo do vanilla) para parsing de .bib/.yaml/.json + CSL engine | XL | Baixo | Paridade mecânica com vanilla; reutilização de código testado; introduz dependência externa |
| **β** — Parser próprio de BibTeX + CSL engine mínimo | Implementar parser de BibTeX/BibLaTeX + YAML/JSON + CSL-JSON + CSL engine reduzido | XXL | Muito alto | >50k LOC; reinventa roda; não justifica para este projeto |
| **γ** — Subset: parser próprio de BibTeX + hayagriva para CSL | Parser próprio de .bib (mais simples) + hayagriva só para CSL rendering | XL+ | Alto | Complexidade intermediária; sem ganho real sobre α |

**Decisão recomendada**: **Opção α** — Hayagriva como dependência.  
**Razão ADR-0108**: medir o custo. Hayagriva é a solução do vanilla, é mantida, é a referência de facto. Reimplementar é XXL com risco de incompatibilidade silenciosa. A paridade é com a linguagem (input .bib → output citações), não com a mecânica interna. Usar hayagriva não viola ADR-0107 porque a paridade não é estrutural.

**Se hayagriva não estiver disponível** (licença, API breaking, não compila): reclassificar para XXL (parser próprio) ou scope-out parcial (suportar apenas CSL-JSON manual, sem .bib).

### A.1.3 — Decisão arquitetural: atomização forma B (ADR-0109)

A lógica de render de `Bibliography` e `Cite` vive na **camada de render**, não no arquivo do struct:

- `entities/bibliography.rs` — struct `BibliographyElem` (dados: path, style, title).
- `entities/cite.rs` — struct `CiteElem` (dados: key, supplement, form).
- `engine/layout/bibliography.rs` — free function `layout_bibliography(layouter, &BibliographyElem)` (forma B).
- `engine/layout/cite.rs` — free function `layout_cite(layouter, &CiteElem)` (forma B).
- `rules/eval/bibliography.rs` — free function `eval_bibliography(ctx, &BibliographyElem)` para loading e CSL processing.

**Não usar Opção A** (método `impl BibliographyElem { fn layout(...) }` em `entities/`) — cria import reverso `entities → rules::layout::Layouter` (ADR-0109, rejeitado).

### A.1.4 — Estrutura de dados

```rust
// entities/cite.rs
pub struct CiteElem {
    pub key: EcoString,           // chave da entrada bibliográfica
    pub supplement: Option<Content>, // "p. 12" opcional
    pub form: CiteForm,           // normal, author, year, etc.
    pub style: Option<EcoString>, // override de CSL style
}

pub enum CiteForm {
    Normal,
    Author,
    Year,
    Suppressed,  // @key[]
}

// entities/bibliography.rs
pub struct BibliographyElem {
    pub path: EcoString,          // "refs.bib"
    pub style: Option<EcoString>, // "ieee", "apa", "mla", ou path .csl
    pub title: Option<Content>,   // título customizado da seção
}
```

**Content enum** (já deve ter variants; se não, adicionar):
```rust
Content::Cite(CiteElem),
Content::Bibliography(BibliographyElem),
```

### A.1.5 — Algoritmo de renderização

1. **Loading (eval time)**:
   - `BibliographyElem` é evaluado; o path é resolvido via file loader existente.
   - O arquivo (.bib, .yaml, .json) é parseado via hayagriva → `Vec<Entry>`.
   - O CSL style é resolvido (default: "ieee" ou built-in; ou path para .csl file).
   - As entradas são armazenadas no `World` (ou `Introspector`) para acesso global durante o documento.

2. **Cite inline (layout time)**:
   - `CiteElem` é layoutado; a chave é lookup no `World` → `Entry`.
   - Se não encontrada: emitir warning + renderizar `[key?]` (paridade vanilla: "unknown citation").
   - Se encontrada: usar hayagriva CSL engine para formatar a citação inline (ex.: "[1]", "(Smith, 2020)", etc.).
   - O supplement é anexado ao output (ex.: "(Smith, 2020, p. 12)").
   - A citação é registrada no `Introspector` (ou state) para tracking de "cited keys".

3. **Bibliography list (layout time)**:
   - `BibliographyElem` é layoutado como um bloco de conteúdo.
   - Coleta todas as keys citadas no documento (via `Introspector` ou state acumulado).
   - Filtra as `Entry` carregadas: apenas as citadas aparecem na lista (paridade vanilla).
   - Ordena conforme CSL style (numérico, alfabético, etc.).
   - Renderiza cada entrada via hayagriva CSL → string/formato → `Content::Text` ou `FrameItem::Text`.
   - Numeração/bullet conforme CSL style.

4. **Forward references**:
   - Vanilla suporta `@key` antes de `#bibliography` (forward ref). O crystalline pode resolver isso via:
     - **Opção A**: multi-passe (complexo, não recomendado — ADR-0107 mecânica livre).
     - **Opção B**: deferred resolution — `Cite` emite placeholder no layout; `Bibliography` faz segundo pass de resolução (complexo).
     - **Opção C**: single pass com acumulação prévia — `Cite` registra no `Introspector` durante eval; `Bibliography` lê no layout (recomendado; paridade comportamental).
   - **Decisão recomendada**: Opção C — acumulação em `Introspector` durante eval, resolução em layout. Não requer multi-passe mecânica.

### A.1.6 — Paridade vanilla

- Vanilla: `@key` → citação inline formatada por CSL; `#bibliography` → lista de referências apenas com entradas citadas; ordem e formato seguem CSL style; forward references funcionam; supplement funciona; múltiplos CSL styles built-in (ieee, apa, mla, chicago).
- Cristalino P418: paridade estrutural (citação inline, lista de referências, CSL style, supplement, forward refs via Introspector); built-in styles: "ieee", "apa" (scope-out para outros se necessário; podem ser adicionados via .csl file).

### A.1.7 — Scope-out explícito

- CSL style custom via URL (download em runtime) — scope-out; apenas path local ou built-in.
- Múltiplos arquivos de bibliografia (`#bibliography("a.bib")` + `#bibliography("b.bib")` em mesmo doc) — scope-out; suportar um único `#bibliography` por documento.
- `cite.form` avançado (author-only, year-only, etc.) — scope-out parcial; `Normal` e `Suppressed` implementados; outros adiados.
- CSL locales (idioma do CSL style) — scope-out; usar locale default.
- Bibliography sorting custom (além do CSL) — scope-out; seguir CSL.
- `BibliographyElem::title` customizado — implementar se trivial; scope-out se complexo.
- BibLaTeX features avançadas (crossref, xdata, etc.) — scope-out; hayagriva suporta, mas não expor diferenças.

---

## CHECKPOINT A

**Só prosseguir para Fase B quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `bibliography.md` + `cite.md` + `loading.md`
2. Sonda A.0 produziu 10/12 OK (mínimo)
3. Decisão α (hayagriva) validada — crate disponível e compatível
4. Decisão C (forward refs via Introspector) validada — `Introspector` existe e aceita acumulação
5. `Content::Cite` e `Content::Bibliography` existem ou plano de adição está claro

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Código

### B.1 — Cargo.toml (se hayagriva não estiver)

```toml
[dependencies]
hayagriva = "0.8"  # ou versão compatível com o vanilla
```

**Verificar compatibilidade**: compilar `hayagriva` com as features necessárias (bibtex, yaml, csl). Se falhar, ajustar versão ou features.

### B.2 — Entities

1. **`entities/cite.rs`** (novo arquivo):
   ```rust
   use ecow::EcoString;

   #[derive(Clone, Debug, PartialEq)]
   pub struct CiteElem {
       pub key: EcoString,
       pub supplement: Option<Box<Content>>, // Box para recursão Content
       pub form: CiteForm,
       pub style: Option<EcoString>,
   }

   #[derive(Clone, Debug, PartialEq)]
   pub enum CiteForm {
       Normal,
       Suppressed, // @key[]
   }
   ```

2. **`entities/bibliography.rs`** (novo arquivo):
   ```rust
   use ecow::EcoString;

   #[derive(Clone, Debug, PartialEq)]
   pub struct BibliographyElem {
       pub path: EcoString,
       pub style: Option<EcoString>,
       pub title: Option<Box<Content>>,
   }
   ```

3. **`entities/content.rs`** — adicionar variants:
   ```rust
   Cite(CiteElem),
   Bibliography(BibliographyElem),
   ```
   **Nota ADR-0109**: se o `match` em `content.rs` ficar muito grande, atomizar o `Display`/`Debug`/`PartialEq` para free functions na camada de render (forma B). Mas o enum em si permanece fechado.

### B.3 — Parser / AST

1. **Sintaxe `@key`**:
   - Se o parser já suporta `@key` como cite inline: verificar.
   - Se não: adicionar no lexer como token `CITE_REF` (`@` + identificador).
   - AST: `Expr::CiteRef { key, supplement }`.

2. **Sintaxe `#cite(<key>)`**:
   - Se `FuncCall` genérico já existe: adicionar `cite` como função nativa.
   - Se não: adicionar `Expr::CiteFunc { key, supplement, form }`.

3. **Sintaxe `#bibliography("refs.bib")`**:
   - `FuncCall` nativa `bibliography` com args `path` (positional) e `style` (named).

### B.4 — Eval (`rules/eval/`)

1. **`rules/eval/cite.rs`** (novo arquivo — forma B):
   ```rust
   pub(super) fn eval_cite(ctx: &mut EvalContext, elem: &CiteElem) -> Value {
       // Registra a citação no Introspector/State
       ctx.introspector.register_citation(&elem.key);
       // Retorna Value::Content(Content::Cite(elem.clone()))
       Value::Content(Content::Cite(elem.clone()))
   }
   ```

2. **`rules/eval/bibliography.rs`** (novo arquivo — forma B):
   ```rust
   pub(super) fn eval_bibliography(
       ctx: &mut EvalContext,
       elem: &BibliographyElem,
   ) -> Result<Value, EvalError> {
       // 1. Resolver path via file loader
       let resolved = ctx.world.resolve(&elem.path)?;
       // 2. Carregar via hayagriva
       let entries = hayagriva::load(&resolved)?;
       // 3. Resolver CSL style
       let style = resolve_csl_style(ctx, elem.style.as_deref())?;
       // 4. Armazenar no World/Introspector
       ctx.introspector.set_bibliography(entries, style);
       // 5. Retornar Content::Bibliography
       Ok(Value::Content(Content::Bibliography(elem.clone())))
   }
   ```

3. **`resolve_csl_style()`**:
   - Built-in: "ieee" → hayagriva built-in; "apa" → hayagriva built-in.
   - Path: resolver `.csl` file via file loader → parse XML → hayagriva `Style`.
   - Default: "ieee" se nenhum especificado.

### B.5 — Consumer: Layout (`engine/layout/` — forma B, ADR-0109)

1. **`engine/layout/cite.rs`** (novo arquivo):
   ```rust
   pub(super) fn layout_cite<M: FontMetrics, S: ImageSizer>(
       layouter: &mut Layouter<M, S>,
       elem: &CiteElem,
   ) {
       // 1. Lookup key no Introspector/World
       let entry = match layouter.world.get_bibliography_entry(&elem.key) {
           Some(e) => e,
           None => {
               // Emitir warning + renderizar [key?]
               layouter.emit_warning(format!("unknown citation: {}", elem.key));
               layouter.push_text(format!("[{}?]", elem.key));
               return;
           }
       };

       // 2. Formatar via hayagriva CSL
       let style = layouter.world.get_csl_style();
       let citation = hayagriva::format_citation(&entry, &style, elem.form.to_hayagriva());

       // 3. Anexar supplement se presente
       let output = if let Some(sup) = &elem.supplement {
           format!("{}, {}", citation, sup) // ou formato CSL de supplement
       } else {
           citation
       };

       // 4. Emitir como texto inline
       layouter.push_text(output);
   }
   ```

2. **`engine/layout/bibliography.rs`** (novo arquivo):
   ```rust
   pub(super) fn layout_bibliography<M: FontMetrics, S: ImageSizer>(
       layouter: &mut Layouter<M, S>,
       elem: &BibliographyElem,
   ) {
       // 1. Obter keys citadas
       let cited_keys = layouter.world.get_cited_keys();

       // 2. Obter entradas e style
       let entries = layouter.world.get_bibliography_entries();
       let style = layouter.world.get_csl_style();

       // 3. Filtrar apenas citadas
       let cited_entries: Vec<_> = entries.iter()
           .filter(|e| cited_keys.contains(&e.key))
           .collect();

       // 4. Ordenar conforme CSL style
       let ordered = hayagriva::sort_entries(&cited_entries, &style);

       // 5. Renderizar título se presente
       if let Some(title) = &elem.title {
           layouter.layout_content(title); // ou push_text
       }

       // 6. Renderizar cada entrada
       for (i, entry) in ordered.iter().enumerate() {
           let formatted = hayagriva::format_reference(entry, &style);
           // Numeração/bullet conforme CSL
           layouter.push_text(format!("{}. {}", i + 1, formatted));
           layouter.new_line();
       }
   }
   ```

### B.6 — Introspector / World extension

```rust
// entities/introspector.rs ou world.rs
pub struct BibliographyState {
    entries: HashMap<EcoString, Entry>,        // key → hayagriva Entry
    style: Option<Style>,                        // CSL style ativo
    cited_keys: HashSet<EcoString>,             // keys citadas no documento
}

impl World {
    pub fn register_citation(&mut self, key: &EcoString) { ... }
    pub fn set_bibliography(&mut self, entries: Vec<Entry>, style: Style) { ... }
    pub fn get_bibliography_entry(&self, key: &EcoString) -> Option<&Entry> { ... }
    pub fn get_cited_keys(&self) -> &HashSet<EcoString> { ... }
    pub fn get_csl_style(&self) -> &Style { ... }
}
```

### B.7 — Tests

**Mínimo 20 tests**:
- 4 unit `CiteElem` / `BibliographyElem`: construção, clone, debug, partial_eq semântico
- 3 unit hayagriva loading: parse .bib, parse .yaml, parse CSL style
- 4 unit eval: eval_cite registra key, eval_bibliography carrega entries, resolve style built-in, resolve style path
- 4 unit layout cite: key encontrada, key não encontrada, supplement, form suppressed
- 5 E2E: documento com @key → citação inline, #bibliography → lista, forward reference, múltiplas citações, style override

---

## FASE C — Validação

```bash
cargo test -p typst-core --lib
# → <baseline> + 20 verdes
# → 0 failed

crystalline-lint .
# → 0 drift nos prompts tocados
# → 0 violations
```

**Critério de fecho**:
- [ ] 20 tests verdes
- [ ] Lint zero drift
- [ ] `@key` produz citação inline formatada
- [ ] `#bibliography("refs.bib")` produz lista de referências
- [ ] Apenas entradas citadas aparecem na lista
- [ ] Forward references funcionam (cite antes de bibliography)
- [ ] Supplement funciona (`@key[p. 12]`)
- [ ] Hayagriva compilando sem warnings críticos
- [ ] Nenhum vtable/`dyn` introduzido (ADR-0109 / ADR-0026)
- [ ] `match` exaustivo preservado (ADR-0105 cl.3)
- [ ] Lógica atomizada em free functions na camada de render (ADR-0109 forma B)
- [ ] L0 hashado e propagado

---

## Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A.0 verifica 12 pré-condições antes de qualquer decisão. Se hayagriva não compilar, o passo para antes de gastar LOC.
- **Paridade linguagem (ADR-0107)**: O contrato é `.bib` + `@key` → citações renderizadas. A mecânica (hayagriva, ordem de passes, estrutura interna) é livre. O forward reference é comportamental (funciona), não mecânico (não precisa ser multi-passe).
- **Atomização (ADR-0109)**: `Cite` e `Bibliography` são structs em `entities/`. A lógica de eval vive em `rules/eval/cite.rs` e `rules/eval/bibliography.rs`. A lógica de layout vive em `engine/layout/cite.rs` e `engine/layout/bibliography.rs`. Todos são free functions (forma B). O `match` no consumer fica magro (1 linha por variant).
- **Honestidade epistêmica**: Este é o maior passo XL do projeto. Se hayagriva apresentar problemas de integração (API instável, features faltantes), documentar imediatamente e reclassificar. Não "forçar" a integração.
- **Próximo passo P419**: `repr()` completo (S) ou `link` render visual (S) ou `text.lang` shaping rustybuzz (XL, scope-out antigo).
