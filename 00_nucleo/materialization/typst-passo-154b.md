# Passo 154B — `terms` + `divider` (Fase 1 Model, primeira sub-fase)

**Série**: 154B (passo **substantivo**; primeira materialização
da Fase 1 do roadmap ADR-0060).
**Precondição**: Passo 154A encerrado; ADR-0060 PROPOSTO com
roadmap Fase 1 / 2 / 3; DEBT-55 aberto (bibliography +
cite XL); 1113 tests; zero violations; 60 ADRs; 13 DEBTs
abertos; cobertura Model 32-36% (8/22).

**Numeração**: 154B segue 154A no padrão diagnóstico-primeiro
+ materialização (precedente 131A→131B, 132A→132B, 140A→140B).

**Natureza**: passo **substantivo**. Toca:
- L1 (`01_core/`): novos variants `Content::Terms`,
  `Content::TermItem`, `Content::Divider`; `native_terms`
  + `native_divider` em stdlib; possível regra `#show
  terms` mínima.
- L0 (prompts): spec dos variants e construtores.
- Testes em `01_core/src/rules/` cobrindo construção +
  comparação + `plain_text`.
- ADR-0060: anotação pós-PROPOSTO se Fase 1 primeiro
  sub-passo for materializado conforme proposta. **Status
  permanece `PROPOSTO`** até Fase 1 inteira fechar (analogia
  ADR-0055 + Passos 140B/141).
- `DEBT.md`: sem mudança esperada.
- README dos ADRs: sem mudança esperada (sem ADR nova).

**ADRs aplicáveis**:
- **ADR-0060** (PROPOSTO) — roadmap Fase 1 inclui
  `terms` + `divider`. Este passo materializa.
- **ADR-0026** + **ADR-0026-R1** — `Content` enum fechado;
  novos variants são adições aceitáveis com justificação.
- **ADR-0033** — paridade funcional para markup `terms` e
  `divider`.
- **ADR-0036** — atomização: cada nova feature consumer
  explícito.
- **ADR-0037** — coesão por domínio: `Content` permanece em
  `entities/content.rs`.
- **ADR-0038** — sistema de estilos: `Terms` e `Divider`
  não usam `Content::Styled` (per regra 7.2 do diagnóstico
  154A).

---

## Contexto

Diagnóstico 154A classificou:

- **`terms`**: ausente; **S**; alto valor (listas de
  definições em texto técnico). `TermsElem` + `TermItemElem`
  no vanilla `model/terms.rs`. Atributos: `tight`,
  `separator`, `indent`, `hanging-indent`, `body`.
- **`divider`**: ausente; **S**; médio valor. `DividerElem`
  no vanilla `model/divider.rs`. Sem atributos publicados;
  estrutural simples.

Recomendação 7.2 do diagnóstico: **variants novos** (não
`Content::Styled`). Razões:
- `Terms`: estrutura aninhada (Terms contém TermItems com
  par term/description).
- `Divider`: sem body; sem styling associado.

**Vanilla syntax**:
- `terms`: `/ Term: description` (sintaxe markup) ou função
  `terms()` em código.
- `divider`: `---` (markup).

**Cristalino actual**:
- Sem syntax `/` em parser; sem `terms()` em stdlib.
- Sem syntax `---` em parser para divider; `---` é
  reservada para metadados em alguns contextos do vanilla
  (verificar se cristalino já usa).

**Pergunta primária deste passo**: materializar variants +
construtores + tests **sem** introduzir syntax markup nova
para `terms`. Razão: syntax markup é trabalho de parser
que arrasta o passo para escopo Layout/Markup. Função
`terms()` em código (sintaxe Typst-lang `#terms((term1:
description1, term2: description2))`) é alternativa S
suficiente para Fase 1.

**Para `divider`**: similar. Confirmar em 154B.1 se `---`
já tem semantic em parser cristalino. Se sim, ajustar para
emitir `Content::Divider`. Se não, função `#divider()` ou
`#dline()` em stdlib é alternativa S.

**Limitação consciente**: este passo materializa
**construtores + variants**. Syntax markup nova
(`/ term: description` e/ou `---`) é trabalho separado se
priorizado.

---

## Objectivo

Ao fim do passo:

1. **`Content` enum estendido** em
   `01_core/src/entities/content.rs`:
   - `Content::Divider` (singleton; sem dados internos).
   - `Content::Terms { items: Vec<TermItem> }` ou similar
     forma estrutural (decidir em 154B.1).
   - `Content::TermItem { term: Box<Content>, description:
     Box<Content> }` ou similar.
   - **Total +3 variants** (Divider + Terms + TermItem).

2. **Cobertura exaustiva** dos arms de match em todos os
   sítios que iteram `Content`:
   - `Content::plain_text()` — Terms produz
     `term + ": " + description` por item; Divider produz
     `"\n"` ou `""` (decidir em 154B.2).
   - `Content::is_empty()` — Terms é não-vazio se tem
     items; Divider é sempre não-vazio.
   - `Content::map_content` — recurse em Terms.term e
     Terms.description; Divider é folha.
   - `Content::map_text` — análogo.
   - `PartialEq` — derivado.
   - Layouter (`03_infra/src/...`) — Terms render como
     lista vertical; Divider render como linha horizontal.
   - `introspect::materialize_time`/`walk` — recurse em
     Terms.
   - Show rules em `eval/rules.rs` — sem show específico
     para Terms/Divider neste passo.

3. **Stdlib funcs** em `01_core/src/rules/eval/mod.rs`
   ou similar:
   - `native_terms(args) -> Value::Content` — recebe
     dict ou array de pares (term, description); produz
     `Content::Terms`. Forma exacta em 154B.3.
   - `native_divider(args) -> Value::Content` — sem args;
     produz `Content::Divider`.
   - Registar em `make_stdlib`.

4. **Testes**:
   - **Unit em `01_core/src/entities/content.rs::tests`**:
     - `terms_constructor_devolve_variant_correcto`.
     - `terms_plain_text_concatena_pares`.
     - `terms_map_content_recurse`.
     - `divider_constructor_devolve_variant_correcto`.
     - `divider_plain_text_devolve_separador`.
   - **Unit em `01_core/src/rules/eval/tests`** (ou
     equivalente):
     - `eval_terms_construtor_typst_lang`.
     - `eval_divider_construtor_typst_lang`.
   - **Integração em `03_infra/src/integration_tests.rs`**:
     - `terms_render_em_pdf` (se layouter já cobrir).
     - `divider_render_em_pdf` (idem).

5. **L0 prompts actualizados**:
   - `prompts/core/content.md` (ou ficheiro equivalente que
     governa `Content`) — secção nova descrevendo Terms,
     TermItem, Divider e regras de tratamento em
     `plain_text`/`map_*`/`is_empty`.
   - Hash recalculado; propagado para
     `01_core/src/entities/content.rs` via header
     `@prompt-hash`.

6. **ADR-0060 anotada** com nota de progresso:
   - Linha no cabeçalho ou secção dedicada indicando que
     primeiro sub-passo da Fase 1 (`terms` + `divider`) foi
     materializado em P154B.
   - **Status permanece `PROPOSTO`** — Fase 1 ainda exige
     P155 (`quote`).

7. **Inventário 148 actualizado**:
   - Tabela A entrada Model: contagens recalculadas (3
   `implementado` aumenta para 5; `ausente` desce de 10
   para 8).
   - Cobertura Model: 32-36% → ~41% (10/22).
   - §7 entrada 7 (~14 elementos vanilla ausentes
   agregados): actualizar contagem.

8. **`Content` Tabela B no inventário 148**:
   - Adicionar 3 entradas novas (Terms, TermItem, Divider)
     como `implementado`.
   - Cobertura `Content` cristalino: 38 variants → 41 variants;
     contagem `implementado` aumenta proporcionalmente.

9. **README dos ADRs**: entrada nova em "Passos-chave da
   história dos ADRs" para P154B.

10. **Relatório do passo** em
    `00_nucleo/materialization/typst-passo-154b-relatorio.md`.

Este passo **não**:

- Implementa syntax markup `/ term: desc` ou `---`
  (trabalho de parser; passo separado se priorizado).
- Implementa `quote` (Passo 155).
- Implementa `table` (Passo 156).
- Implementa `bibliography`/`cite` (Passos 158+).
- Modifica DEBT-55.
- Cria ADRs novas.
- Toca série paridade ou `lab/parity/`.
- Exige crates novas.

---

## Decisões já tomadas

1. **Variants novos no `Content` enum** (não `Content::Styled`).
   Per recomendação 7.2 do diagnóstico 154A.
2. **Sem syntax markup nova** neste passo. Construtores
   via stdlib `#terms()` e `#divider()` em Typst-lang.
3. **`Status` ADR-0060 permanece `PROPOSTO`** até Fase 1
   inteira fechar.
4. **Sem alteração de DEBT-55** (escopo Fase 2).
5. **Cobertura exaustiva de arms `match`** é obrigatória —
   `Content` é enum fechado per ADR-0026; não compilará sem.
6. **`Divider` é singleton sem dados** — não carrega
   `Box<Content>` nem atributos.
7. **`Terms` é estrutura aninhada** — `Vec<TermItem>` ou
   similar.

## Decisões diferidas (resolvidas neste passo)

8. **Forma exacta de `Content::Terms`**:
   - Opção A: `Terms { items: Vec<TermItem> }` — direct.
   - Opção B: `Terms(Vec<TermItem>)` — tuple variant.
   - **Default A** (named field; consistente com
     `Heading { level, body }`).

9. **Forma exacta de `TermItem`**:
   - Opção C: `TermItem { term: Box<Content>, description:
     Box<Content> }`.
   - Opção D: `TermItem(Box<Content>, Box<Content>)`.
   - **Default C** (named fields; legível).

10. **`plain_text` para `Terms`**:
    - Opção E: `term1: description1\nterm2: description2`.
    - Opção F: `term1\ndescription1\n\nterm2\ndescription2`.
    - **Default E** (mais compacto; preserva relação par).

11. **`plain_text` para `Divider`**:
    - Opção G: `""` (vazio).
    - Opção H: `"\n"` (newline).
    - Opção I: `"\n---\n"` (visualmente representativo).
    - **Default G** (vazio; coerente com `Divider` ser
      structural sem texto). Documenta-se que renderização
      no PDF é distinta.

12. **`native_terms` shape**:
    - Opção J: `#terms((term1: "desc1", term2: "desc2"))`
      — dict argument.
    - Opção K: `#terms(term1: "desc1", term2: "desc2")`
      — named args.
    - Opção L: `#terms("term1", "desc1", "term2", "desc2")`
      — positional pares.
    - **Default K** (alinha com `figure(caption: ...)` em
      cristalino; named args são idioma comum em Typst).

13. **`native_divider` shape**:
    - Opção M: `#divider()` — função sem args.
    - Opção N: `divider` — constante (não-função).
    - **Default M** (consistente com outras stdlib;
      function call uniforme).

14. **Atributos `tight`/`separator`/`indent`/`hanging-indent`
    de `terms`** vanilla:
    - **Decisão**: **scope-out** neste passo. ADR-0060 perfil
      graded permite. Atributos podem ser adicionados em
      passo futuro se priorizados; representação interna
      `Terms { items }` permite extensão posterior sem
      breaking change (passar a `Terms { items, tight, ... }`).

15. **Show rule `#show terms: ...`**: scope-out neste
    passo. Cristalino actual tem show rules para
    `heading`/`strong`/`emph` (Passo 103); estender para
    `terms` é trabalho mecânico mas adiciona ~15-30 linhas
    em `eval/rules.rs`. Decisão default: **omitir**;
    `terms` aparece via construtor directo. Se utilizadores
    pedirem, abrir DEBT-56.

---

## Escopo

**Dentro**:

- Edição de `01_core/src/entities/content.rs`: 3 variants
  novos + cobertura exaustiva de arms.
- Edição de `01_core/src/entities/content.rs::tests`: 5
  testes unit.
- Edição de `01_core/src/rules/eval/mod.rs`:
  `native_terms` + `native_divider` + registo em
  `make_stdlib`.
- Edição de `01_core/src/rules/eval/tests` (se existir
  módulo separado): 2 testes integração eval.
- Edição de `03_infra/src/integration_tests.rs`:
  até 2 testes integração render (se layouter cobrir).
- Edição de `prompts/core/content.md` (ou prompt L0
  equivalente): spec + hash recalculado.
- Edição de header de `01_core/src/entities/content.rs`:
  `@prompt-hash` actualizado.
- Edição de Layouter para Terms/Divider rendering
  básico (se exigido por testes).
- Anotação em ADR-0060 (sem mudança de status).
- Actualização de inventário 148 (Tabela A Model;
  Tabela B Content).
- Actualização de README dos ADRs (Passos-chave).
- Relatório do passo.

**Fora**:

- Syntax markup `/ term: desc` ou `---` (parser).
- Atributos vanilla (`tight`, `separator`, `indent`,
  `hanging-indent`).
- Show rules `#show terms`.
- Outras features Fase 1 (`quote`).
- Outras Fases (table, figure-kinds, bibliography).
- ADR nova.
- Modificação de DEBT-55.
- Trabalho em `lab/parity/`.
- Importação de crates.

---

## Sub-passos

### 154B.1 — Inventário pré-materialização

**A.1.1 — Confirmar enum `Content` actual**:

```bash
view 01_core/src/entities/content.rs   # entre linhas 1-200 para ver enum
grep -nE "^pub enum Content" 01_core/src/entities/content.rs
```

Listar variants existentes (esperado 38 per inventário 148).

**A.1.2 — Confirmar todos os sítios de `match content`**:

```bash
grep -rn "match \w*content\|match self\b" 01_core/src/ | grep -v test
```

Tipicamente: `plain_text`, `is_empty`, `map_content`,
`map_text`, `PartialEq`, `introspect`, layouter,
`materialize_time`, `walk`.

Esperado: ~10-15 match sites no L1; mais em L3.

**A.1.3 — Verificar parser para `---`**:

```bash
grep -rn "---\|\\bdash\\b\\|\\bhrule\\b" 01_core/src/rules/parse/
```

Confirmar se `---` já tem reconhecimento. Se sim, decisão
em 154B.5 se aproveitar; se não, scope-out parser.

**A.1.4 — Verificar `make_stdlib`**:

```bash
view 01_core/src/rules/eval/mod.rs   # localizar make_stdlib
```

Listar funções já registadas. Confirmar 29 funções nativas
+ módulo calc (per inventário 148).

### 154B.2 — Adicionar variants ao `Content`

```diff
 pub enum Content {
     Empty,
     Text(EcoString, TextStyle),
     Sequence(Arc<[Content]>),
     Styled(Box<Content>, Styles),
     Heading { level: u8, body: Box<Content> },
     // ... outros variants
+    Divider,
+    Terms { items: Vec<TermItem> },
+    TermItem { term: Box<Content>, description: Box<Content> },
 }
```

**Notas**:
- `Divider` é singleton — sem campos.
- `Terms` carrega `Vec<TermItem>`; ordem preservada.
- `TermItem` é também variant directo de `Content` (não
  apenas struct interno) — permite construir
  `Content::TermItem` directamente fora de `Content::Terms`
  se necessário (e.g. show rules futuras).

### 154B.3 — Cobertura exaustiva de arms

Para cada match site identificado em 154B.1:

**A.3.1 — `plain_text`**:

```rust
match self {
    // ... arms existentes
    Content::Divider => String::new(),
    Content::Terms { items } => items.iter()
        .map(|t| t.plain_text())
        .collect::<Vec<_>>()
        .join("\n"),
    Content::TermItem { term, description } => format!(
        "{}: {}",
        term.plain_text(),
        description.plain_text()
    ),
}
```

**A.3.2 — `is_empty`**:

```rust
match self {
    Content::Divider => false,
    Content::Terms { items } => items.is_empty(),
    Content::TermItem { term, description } =>
        term.is_empty() && description.is_empty(),
}
```

**A.3.3 — `map_content`**:

Recurse em `Terms.items` e em `TermItem.term`/`description`.
`Divider` é folha.

**A.3.4 — `map_text`**: análogo.

**A.3.5 — Layouter** em `03_infra/src/...`:

- `Divider` → emit horizontal rule (linha simples a 0.5pt
  ao longo da largura disponível). Forma mínima.
- `Terms` → loop sobre items, cada um emit como linha
  vertical: `term: description` em linhas separadas com
  identação (e.g. 2em).
- `TermItem` standalone → emit como linha única
  `term: description`.

Forma mínima viável; ADR-0060 perfil graded permite. Se
layouter actual não cobre, pode emitir `Group` com
sub-itens.

### 154B.4 — Stdlib funcs

Em `01_core/src/rules/eval/mod.rs::make_stdlib`:

```diff
 fn make_stdlib() -> Scope {
     let mut s = Scope::default();
     // ... funcs existentes
+    s.define("terms", Func::native("terms", native_terms));
+    s.define("divider", Func::native("divider", native_divider));
     s
 }
```

```rust
fn native_terms(args: &Args) -> SourceResult<Value> {
    let mut items = Vec::new();
    // Iterar named args: cada par (k, v) vira TermItem
    for (key, value) in args.named.iter() {
        let term = Content::Text(key.clone(), TextStyle::default());
        let description = match value {
            Value::Content(c) => c.clone(),
            Value::Str(s) => Content::Text(s.clone(), TextStyle::default()),
            // ... outros casos
            _ => return Err(SourceError::new(
                "terms: argumento inválido", args.span,
            )),
        };
        items.push(Content::TermItem {
            term: Box::new(term),
            description: Box::new(description),
        });
    }
    Ok(Value::Content(Content::Terms { items }))
}

fn native_divider(args: &Args) -> SourceResult<Value> {
    if !args.items.is_empty() || !args.named.is_empty() {
        return Err(SourceError::new(
            "divider: nenhum argumento esperado", args.span,
        ));
    }
    Ok(Value::Content(Content::Divider))
}
```

(Forma exacta dependente da API real de `Args` — confirmar
em 154B.1.A.4.)

### 154B.5 — Tests unit

Em `01_core/src/entities/content.rs::tests`:

```rust
#[test]
fn divider_constructor_devolve_variant_correcto() {
    let c = Content::Divider;
    assert!(matches!(c, Content::Divider));
    assert!(!c.is_empty());
}

#[test]
fn divider_plain_text_devolve_vazio() {
    assert_eq!(Content::Divider.plain_text(), "");
}

#[test]
fn terms_constructor_devolve_variant_correcto() {
    let items = vec![
        Content::TermItem {
            term: Box::new(Content::Text("a".into(), TextStyle::default())),
            description: Box::new(Content::Text("b".into(), TextStyle::default())),
        },
    ];
    let c = Content::Terms { items: items.clone() };
    assert!(matches!(c, Content::Terms { .. }));
}

#[test]
fn terms_plain_text_concatena_pares() {
    let c = Content::Terms {
        items: vec![
            Content::TermItem {
                term: Box::new(Content::Text("Apple".into(), TextStyle::default())),
                description: Box::new(Content::Text("fruit".into(), TextStyle::default())),
            },
            Content::TermItem {
                term: Box::new(Content::Text("Banana".into(), TextStyle::default())),
                description: Box::new(Content::Text("yellow".into(), TextStyle::default())),
            },
        ],
    };
    assert_eq!(c.plain_text(), "Apple: fruit\nBanana: yellow");
}

#[test]
fn terms_map_content_recurse() {
    // Test que map_content visita term e description.
    // ...
}
```

### 154B.6 — Tests integração eval

Em `01_core/src/rules/eval/tests` (ou equivalente):

```rust
#[test]
fn eval_divider_construtor_typst_lang() {
    let source = r"#divider()";
    let module = eval(source).unwrap();
    let result = module.scope().get("__resultado__"); // se aplicável
    // ou inspeccionar conteúdo do módulo
}

#[test]
fn eval_terms_construtor_typst_lang() {
    let source = r"
        #let __resultado__ = terms(
            apple: [fruit],
            banana: [yellow],
        )
    ";
    let module = eval(source).unwrap();
    let value = module.scope().get("__resultado__").unwrap();
    match value {
        Value::Content(c) => {
            assert!(matches!(c, Content::Terms { .. }));
        }
        _ => panic!("esperado Content::Terms"),
    }
}
```

### 154B.7 — Tests integração render (opcional)

Se layouter for estendido em 154B.3.A.5:

```rust
#[test]
fn divider_render_em_pdf() {
    let source = "Hello\n#divider()\nWorld";
    let pdf = compile_to_pdf_bytes(...);
    // Assert que PDF contém operador de linha.
}

#[test]
fn terms_render_em_pdf() {
    let source = r"
        #terms(
            apple: [fruit],
            banana: [yellow],
        )
    ";
    let pdf = compile_to_pdf_bytes(...);
    // Assert que PDF contém duas linhas com texto + indent.
}
```

Se layouter for **scope-out** neste passo, omitir estes
tests; documentar no relatório.

### 154B.8 — Edição L0

`prompts/core/content.md` (ou equivalente) ganha secção
"Variants estruturais (Passo 154B)":

```markdown
### `Content::Divider`

Singleton; representa separador horizontal. Sem dados
internos. `plain_text` devolve `""`. Layouter emite linha
horizontal a 0.5pt.

### `Content::Terms { items: Vec<TermItem> }`

Lista de pares termo-descrição. Items preservam ordem.
`plain_text` concatena com `"\n"`.

### `Content::TermItem { term, description }`

Par individual. `plain_text` produz `"term: description"`.
Variants `term` e `description` são `Box<Content>` para
permitir aninhamento.
```

Hash recalculado:

```bash
sha256sum 00_nucleo/prompts/core/content.md
```

Header de `01_core/src/entities/content.rs`:

```diff
- //! @prompt-hash <hash-anterior>
+ //! @prompt-hash <hash-novo>
- //! @updated <data-anterior>
+ //! @updated 2026-04-25
```

`crystalline-lint .` confirma zero V5.

### 154B.9 — Anotar ADR-0060

```diff
+ **Anotação Passo 154B**: primeiro sub-passo da Fase 1
+ materializado — `Content::Divider`, `Content::Terms`,
+ `Content::TermItem` adicionados a `Content` enum;
+ `native_terms` e `native_divider` em stdlib.
+ Status permanece `PROPOSTO`; Fase 1 fecha após Passo 155
+ (`quote`).
```

Adicionar a secção "Materialização" se ela existir (ou
criar).

**Status `PROPOSTO` preservado**.

### 154B.10 — Actualizar inventário 148

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Tabela A** entrada Model: contagens recalculadas.
  - `implementado` 3 → 5 (terms, divider).
  - `ausente` 10 → 8.
  - Cobertura: 32-36% → ~41% (10/22).
- **Tabela B** entrada Content variants: 38 → 41.
  - Adicionar Divider, Terms, TermItem como `implementado`.
  - Cobertura arquitectural Content cristalino: 27/39 →
    30/42 = 71% (estima; recalcular exacto em 154B.10).

Actualizar §7 entrada 7 (~14 elementos vanilla ausentes
agregados): substituir por contagem corrigida (provável 12
após este passo, dependendo de classificação anterior).

### 154B.11 — Actualizar README dos ADRs

Em "Passos-chave da história dos ADRs":

```markdown
- **Passo 154B** — Fase 1 Model (primeira sub-fase):
  `terms` + `divider` materializados. `Content` enum ganha
  3 variants novos. Cobertura Model 32-36% → ~41%.
  ADR-0060 anotada (status `PROPOSTO` preservado).
```

Sem mudança na tabela "Estado por ADR" (sem ADR nova).
Sem mudança no total (60 ADRs).

### 154B.12 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-154b-relatorio.md`.

Secções:
1. Sumário executivo.
2. Inventário pré-materialização (resultado de 154B.1).
3. Variants adicionados (forma final + diff).
4. Cobertura exaustiva de arms (lista de match sites
   tocados).
5. Stdlib funcs (assinaturas finais).
6. Tests adicionados (lista + contagens).
7. Edição L0 + hash propagado.
8. ADR-0060 anotada (sem mudança de status).
9. Inventário 148 actualizado.
10. README dos ADRs actualizado.
11. Próximo passo: 155 (`quote`, Fase 1 segunda sub-fase).
12. Limitações registadas:
    - Sem syntax markup nova.
    - Sem atributos vanilla.
    - Sem show rules.
13. Verificação final.

---

## Verificação

1. ✅ `Content` enum estendido com 3 variants novos.
2. ✅ Cobertura exaustiva de arms `match` (~10-15 sites
   no L1).
3. ✅ `native_terms` + `native_divider` registados em
   `make_stdlib`.
4. ✅ 5 tests unit em `content.rs::tests`.
5. ✅ 2 tests integração eval.
6. ✅ Tests integração render (até 2; opcional consoante
   layouter).
7. ✅ L0 prompt actualizado; hash propagado.
8. ✅ ADR-0060 anotada (`PROPOSTO` preservado).
9. ✅ Inventário 148 actualizado (Tabela A + B).
10. ✅ README dos ADRs com entrada P154B em
    "Passos-chave".
11. ✅ Sem ADR nova.
12. ✅ Sem DEBT criado / fechado.
13. ✅ `cargo test --workspace --lib`: 1113 → 1120-1125
    (acréscimo +7 a +12 conforme tests integração render).
14. ✅ `crystalline-lint .` zero violations.
15. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. `Content::Divider`, `Content::Terms`, `Content::TermItem`
   compilam + testes unit passam.
2. Stdlib funcs invocáveis via `#terms(...)` e `#divider()`
   em Typst-lang.
3. ADR-0060 anotada com `PROPOSTO` preservado.
4. Inventário 148 reflecte cobertura aumentada.
5. Próximo passo (155 = `quote`) tem âncora.
6. Sem regressão.
7. Relatório do passo escrito.

---

## O que pode sair errado

- **Cobertura exaustiva de arms quebra compilação em
  ficheiros não-óbvios**: `Content` é enum fechado; novos
  variants exigem arms em todos os sítios. Se algum sítio
  for esquecido, `cargo build` falha. Solução: deixar
  compilador guiar — adicionar arms onde reclama; iterar
  até passar.

- **Layouter não suporta Terms/Divider sem refactor**: se
  layouter actual usa pattern matching exhaustivo sobre
  `Content`, adicionar arms básicos (Group emit fallback).
  Se exige refactor maior (>50 linhas), **scope-out
  rendering** neste passo; emit Group de placeholder e
  documentar.

- **Sintaxe `#terms(named: args)` falha em parser**:
  cristalino pode ter limitação em named args. Se sim,
  fallback Opção L (positional pares). Documentar
  limitação.

- **`Args::named` API diferente do esperado**: probe em
  154B.1.A.4 confirma. Ajustar `native_terms` conforme.

- **Test integração render exige fontes específicas**:
  reusar fixture do P140B/P141 ou skip elegante (mesmo
  padrão `discover_any_system_fonts`).

- **`---` já tem semantic em parser cristalino**: se
  marcas como heading delimiter ou frontmatter,
  **não tocar**. `divider` permanece via construtor
  `#divider()` apenas. Documentar.

- **`#show terms` é exigido para teste passar**: improvável
  (teste deveria validar construção, não show). Se exigido,
  fallback é stub mínimo ou scope-out test.

- **Diff em `content.rs` cresce demasiado**: aceitável.
  Cobertura exaustiva de 10-15 match sites pode somar
  100+ linhas. Distribuído por arms novos pequenos.

- **`Box<Content>` em TermItem causa problemas com
  Send/Sync**: improvável (Content já tem Box em outros
  variants). Se ocorrer, debug específico.

- **`PartialEq` derivado falha**: improvável (Vec, Box já
  derivam). Se ocorrer, implementar manualmente.

- **Variant `TermItem` directo em `Content` confunde
  semântica**: per Decisão 9, é variant directo.
  Alternativa (struct interno) é refactor; trabalho extra.
  Manter variant directo (mais flexível).

---

## Notas operacionais

- **Modelo: substantivo análogo a P102 (`fill` activado)
  ou P139 (consumer weight)**: feature pequena, mas
  estabelece padrão para sub-passos seguintes (P155
  `quote`, P156 `table`).

- **Fase 1 Model em 2 sub-passos**: 154B (terms +
  divider) + 155 (quote). Após Fase 1, ADR-0060 transita
  `PROPOSTO → IMPLEMENTADO`. Fase 2 abre depois.

- **`Divider` singleton sem dados** é o variant mais
  simples possível em `Content`. Útil como template para
  futuros singletons (e.g. Linebreak se priorizado).

- **`Terms` aninhado** estabelece padrão para futuros
  variants compostos (e.g. Quote, Table com cells).

- **Sem ADR nova**. ADR-0060 já cobre roadmap; este passo
  materializa primeira sub-fase.

- **DEBT-55 inalterado**. Bibliography + cite continua
  aberto e fora do escopo.

- **Regra "não usar Content::Styled"** confirmada (per
  154A.7.2). `Terms`, `Divider`, `TermItem` exigem variants
  novos por terem semântica distinta de styling.

- **Pós-154B**: cobertura Model 41% (de 32-36%); cobertura
  arquitectural Content 71% (estimativa). Próximo passo
  (155 = quote) eleva para ~45%.

- **Antecipação**: P155 `quote` segue mesmo padrão (variant
  novo + native + tests). Atributos `attribution`, `block`,
  `quotes` exigem decisão (todos? subset?). 154B
  estabelece template; 155 reaproveita.

- **Show rules adiadas**: candidato a passo único
  (P154C ou P159+) que adiciona show rules para todas
  as features Fase 1 simultaneamente. Decisão futura.
