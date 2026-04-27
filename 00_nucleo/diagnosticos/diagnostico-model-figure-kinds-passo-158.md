# Diagnóstico Model figure-kinds — Passo P158

Inventário diagnóstico precedendo materialização per **ADR-0034 +
ADR-0065** — análogo estrutural a P157 (table foundations) e
P156B (Layout). **Décima terceira aplicação consecutiva** do
padrão diagnóstico-primeiro; **segunda aplicação concreta** do
critério #5 ADR-0065 (scope determinado por inventário) após P157.

---

## §1 — ADR-0060 sobre figure-kinds

**Localização**: `00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md`.

### §1.1 Status declarado

ADR-0060 status `IMPLEMENTADO` (Fase 1 fechada P155; Fase 2
sub-passo 3 fechado P157C). Fase 2 prossegue per roadmap;
**figure-kinds** é sub-passo Fase 2 declarado no §"Plano de
materialização" e §"Decisão 2".

### §1.2 Definição literal de "figure-kinds"

ADR-0060 §"Decisão 2" — Fase 2:

> **Passo 158** (renumerado de P157 em P156B) — `figure` kinds
> extension (depende de Passo 157 para figure-table; M).

ADR-0060 §"Plano de materialização" linha P158:

> P158 (renumerado de 157 em P156B) | M | figure kinds | —

**Subset declarado**: extension de `figure` para suportar kinds
adicionais. **Sem ADR adicional necessária**. Tamanho declarado:
M. Dependência declarada: **P157 para figure-table**.

**Detalhes não declarados em ADR-0060**:
- Que kinds específicos materializar.
- Como diferenciar de implementação existente (P75/ADR-0041).
- Auto-detecção de kind vs kind manual.

ADR-0060 deixa scope concreto para diagnóstico P158 determinar.

### §1.3 ADRs auxiliares relevantes

- **ADR-0041** (activação show-rule) menciona
  `#show figure: ...` — show rules futuras com `figure.where(kind:)`
  selectors são scope-out per cristalino actual.
- **ADR-0017** (estratégia typst-library) — gradual; counters
  resolvem em walk per P58/P75.

Nenhuma ADR adicional cobre figure-kinds especificamente. ADR-0060
permanece a única referência arquitectural directa.

---

## §2 — Estado de Figure + dependências em código

### §2.1 `Content::Figure` factual

`01_core/src/entities/content.rs:202-211`:

```rust
Figure {
    body:      Box<Content>,
    caption:   Option<Box<Content>>,
    /// `kind` discrimina o contador: "image", "table", "raw", etc.
    kind:      String,
    numbering: Option<String>,
}
```

**Field `kind: String` arbitrário**. Default `"image"` aplicado em
`native_figure` (não no constructor — cristalino vs vanilla
`Smart<FigureKind>`).

**Counters por kind funcionais**: `01_core/src/rules/introspect.rs:279-292`:

```rust
Content::Figure { body, caption, kind, numbering } => {
    if numbering.is_some() && caption.is_some() {
        let counter = state.local_figure_counters
            .entry(kind.clone())
            .or_insert(0);
        *counter += 1;
        ...
    }
    ...
}
```

**Counters independentes por kind** já implementados — `figure(kind:
"image")` numera "1, 2, 3" independente de `figure(kind: "table")`.

### §2.2 `Content::Image` factual

`01_core/src/entities/content.rs:224-229`:

```rust
Image {
    path:   String,
    data:   PtrEqArc<Vec<u8>>,
    width:  Option<Box<Value>>,
    height: Option<Box<Value>>,
}
```

**Implementado** (Passo 71, DEBT-24).

### §2.3 `native_figure` factual

`01_core/src/rules/stdlib/figure_image.rs:26-63`:

```rust
let kind = args.named.get("kind")
    .and_then(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
    .unwrap_or_else(|| "image".to_string());
```

**Kind aceita Str arbitrário**; default `"image"`. **Sem
auto-detecção** baseada no body — user passa kind manualmente.

### §2.4 Stdlib funcs Model relevantes

- `native_figure` (P75): existe.
- `native_image` (P71): existe.
- `native_raw` (em `stdlib/structural.rs`): existe (P156C).
- `native_table` (P157A): existe.
- `native_table_cell`, `native_table_header`, `native_table_footer`
  (P157B/C): existem.

**Todas as funcs necessárias para auto-detecção de kind já
existem**. Auto-detecção exige apenas inspecção do body para
detectar variant Content (Image/Table/Raw).

### §2.5 Tabela de cobertura factual (A.6 Model)

- `figure(body, caption, ...)`: **`implementado⁺`** (Passos 75,
  ADR-0041) — "numbering por kind; counters".
- `image(path, ...)`: **`implementado`** (Passo 71).
- `raw`: **`implementado`** (Passo 156C ou anterior).
- `table`: **`implementado`** (P157A).
- `table.cell`: `parcial` (P157B).
- `table.header`/`table.footer`: `parcial` (P157C).

**`figure` já é `implementado⁺`** — refino qualitativo possível;
não é entry "ausente".

### §2.6 Hashes actuais relevantes

- `entities/content.rs`: `ec58d849` (preservado P156L → P157C).
- `rules/stdlib/figure_image.rs`: `f6cc2443`.
- `rules/stdlib/structural.rs`: hash via lineage (preservado P157C).
- `rules/introspect.rs`: hash via lineage.

---

## §3 — Scope de "figure-kinds"

### §3.1 Vanilla "figure-kinds extension" — análise

Vanilla `model/figure.rs:191-379` documenta o que constitui
"figure-kinds extension":

1. **Auto-detecção de kind** baseado no body (image → "image",
   table → "table", raw → "code"). Vanilla usa
   `Selector::can::<dyn Figurable>()` para query (linha 341).
2. **Supplement automático** baseado no kind ("Figure"/"Table"/
   "Listing" prefix localizado por language).
3. **Selectors** `figure.where(kind: table)` para counter
   querying em show rules.

### §3.2 Subset MÍNIMO P158A — auto-detecção apenas

```rust
// native_figure (P158A) — pseudocódigo:
let kind = args.named.get("kind")
    .and_then(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
    .or_else(|| infer_kind_from_body(&body))  // P158A novo
    .unwrap_or_else(|| "image".to_string());

fn infer_kind_from_body(body: &Content) -> Option<String> {
    match body {
        Content::Image { .. } => Some("image".to_string()),
        Content::Table { .. } => Some("table".to_string()),
        Content::Raw { .. }   => Some("raw".to_string()),
        _ => None,  // sem detecção; fallback default
    }
}
```

**Características**:
- 1 helper privado novo (`infer_kind_from_body`).
- Sem alteração ao variant `Content::Figure` (estrutura inalterada).
- Sem alteração ao layout ou introspect (counters já funcionam).
- Sem dependência nova — Image/Table/Raw já existem.
- Granularidade preservada N=13.
- Tests esperados: ~6-8 (auto-detecção por kind + fallback +
  override manual).

### §3.3 Subset MÁXIMO P158A — auto-detecção + supplement

Inclui §3.2 + lógica de supplement automático ("Figura"/"Tabela"/
"Listing" prefix por kind). Exige modificação em `introspect.rs`
para emitir prefix correcto.

Cristalino actual usa "Figura {n}" hard-coded em introspect.rs:
311. Refactor exige:
- Mapeamento `kind → prefix_localizado(lang)`.
- Lookup de `lang` activo (já feito em P155 para quotes).
- ~10-15 linhas de refactor em introspect.rs.

**Tests esperados**: ~12-18.

### §3.4 Subset INTERMÉDIO — auto-detecção + supplement com lang fallback

Combina §3.2 + supplement minimal (ASCII fallback per lang
desconhecido; prefix em pt para `lang=Some("pt")`; em en para
default).

Tests esperados: ~10-14.

### §3.5 Recomendação para P158A

**Subset MÍNIMO §3.2** (auto-detecção apenas).

Justificação:
1. **Granularidade preservada N=13** (1 feature).
2. **Subset máximo §3.3** introduz supplement que toca em
   introspect.rs — refactor moderado, candidato a passo dedicado
   futuro se prioritário.
3. **Subset intermédio §3.4** mistura concerns (auto-detecção
   + lang) — viola "1 feature/passo".
4. **Auto-detecção sozinha já é melhoria substancial** —
   user pode escrever `figure(image("foo.png"))` sem `kind:` e
   counter independente activa automaticamente.
5. **Aplicação concreta de ADR-0064 Caso A** se kind for
   refactorado para `Smart<String>` → `Option<String>` (mas spec
   actual kind é `String` directo; refactor opcional).

### §3.6 Avaliação do precedente P157

P157 dividiu "table foundations" M+ em 3 sub-passos M cada
(P157A/B/C). P158 figure-kinds é diferente:
- **Não é estrutura monolítica** que justifica divisão N-way.
- **É uma única feature** (auto-detecção) sem componentes
  independentes naturais.
- Subset intermédio §3.4 e máximo §3.3 podem ser materializados
  em passos seguintes se prioritário (P158B supplement),
  **mas não pré-acordados como reservas**.

---

## §4 — Dependências bloqueantes

### §4.1 Dependências de variants Content

| Dependência | Estado | Bloqueia P158A subset minimal? |
|-------------|--------|:-------------------------------:|
| `Content::Image` | implementado (P71) | Não |
| `Content::Table` | implementado (P157A) | Não |
| `Content::Raw` | implementado (P156C ou anterior) | Não |
| `Content::Figure` infraestrutura | implementado⁺ (P75/ADR-0041) | Não — só refino |

### §4.2 Dependências de Introspection / counters

| Dependência | Estado | Bloqueia? |
|-------------|--------|:---------:|
| Counters por kind | implementado (introspect.rs:279) | Não |
| ADR-0017 Introspection runtime | adiada | Não — counters resolvem em walk single-pass |

### §4.3 DEBTs abertos relevantes

| DEBT | Descrição | Impacto P158A |
|------|-----------|---------------|
| DEBT-14 | numbering baked-in em eval (Passo 75) | resolvido para subset minimal |
| DEBT-15 | kind como counter discriminator (Passo 75) | resolvido para subset minimal |

### §4.4 ADRs em vigor relevantes

| ADR | Aplicação a P158A |
|-----|-------------------|
| ADR-0017 | Estratégia gradual — autoriza scope-out de show selectors |
| ADR-0033 | Paridade observável — auto-detecção produz output observable correcto |
| ADR-0034 | Diagnóstico obrigatório — cumprido por P158 (este doc) e replicado por P158A |
| ADR-0041 | Show rules — `figure.where(kind:)` selectors scope-out |
| ADR-0054 | Perfil graded — autoriza scope-out de supplement automático e selectors |
| ADR-0060 | Roadmap Model — autoriza P158 como sub-passo Fase 2 |
| ADR-0064 (P156K) | Smart→Option/default — Caso A potencialmente aplicável se kind for refactorado para Option<String>; subset minimal NÃO refactora |
| ADR-0065 (P156K) | Inventariar primeiro — aplicado por este passo (segunda aplicação concreta de critério #5) |

### §4.5 ADRs pendentes / candidatas

| ADR | Estado | Bloqueia? |
|-----|--------|:---------:|
| ADR-0061 (Layout roadmap) | PROPOSTO | Não |
| ADR-0062 (hayagriva) | reservada (não criada) | Não — P159 future |

### §4.6 Conclusão de dependências

**Zero bloqueios hard** para P158A subset minimal. Toda a
infraestrutura necessária (Image, Table, Raw, Figure, counters)
já existe e funciona.

---

## §5 — Esboço de P158A (passo substantivo seguinte)

### §5.1 Identificador

**P158A** — segue precedente da série P157A/B/C (sufixo letra
após número base).

### §5.2 Tamanho

**S+** ou **M-** (1 feature simples; helper de inferência ~10
linhas; sem refactor de variant; sem nova ADR).

Subset minimal mais leve que P157A/B/C porque infraestrutura
Figure já existe — apenas activação de detecção automática.

### §5.3 Subset concreto

**Auto-detecção de kind em `native_figure`**:
- Helper privado novo `infer_kind_from_body(body: &Content) ->
  Option<String>` em `stdlib/figure_image.rs`.
- Modificação em `native_figure` para fallback chain:
  `kind explícito > infer_kind_from_body > "image"`.
- Sem alteração a `Content::Figure` (estrutura inalterada).
- Sem alteração a `introspect.rs` (counters já funcionam).
- Sem alteração a layout (figure body renderiza como sempre).

### §5.4 Sub-passos previstos (alto nível)

1. **Inventário** (mínimo per ADR-0034 + ADR-0065) em
   `diagnostico-figure-kinds-passo-158a.md`. Confirma decisão
   de detecção (incluir Sequence wrapping?), kinds detectáveis,
   tests esperados.
2. **Helper `infer_kind_from_body`** em `stdlib/figure_image.rs`.
3. **Modificação `native_figure`** para incluir fallback.
4. **Tests**: ~6-8 (cada kind detectado + fallback + override
   manual + Sequence wrap se decidido em .1).
5. **Hashes**: `crystalline-lint --fix-hashes` (esperado
   "Nothing to fix" se refactor for puramente aditivo).

### §5.5 Granularidade

**Preservada N=13** (1 feature/passo). Cadência granular mantida
desde P156C. Padrão #1 cresce.

### §5.6 Padrões aplicáveis

- **ADR-0064 NÃO directamente aplicável** em subset minimal
  (kind continua String directo). Caso A potencialmente aplicável
  se refactor de `kind: String → Option<String>` for materializado
  em passo posterior (refactor do variant).
- **ADR-0065 critério #5** (scope) aplicado via este diagnóstico.
- **Reuso de infraestrutura existente** (Figure/Image/Table/Raw)
  — N=1 aplicação concreta deste subpadrão se documentado;
  candidato a formalização se P158B/C continuarem o padrão.

### §5.7 Risco estimado

**Baixo**:
- Reusos significativos (toda a infraestrutura existe).
- Inventário .1 cobre divergências antes de execução.
- Sem alteração a variant ou layout.
- Sem dependência nova.

### §5.8 Subset NÃO incluído em P158A (deferido sem reserva)

**Não criar reservas** per nota operacional do passo:
- **Supplement automático** ("Figura"/"Tabela"/"Listing" prefix
  por lang): refino M futuro se prioritário; **NÃO reservado**.
- **Show selectors `figure.where(kind:)`**: scope-out per
  ADR-0041 + ADR-0054 graded; **NÃO reservado**.
- **Refactor `kind: String → Option<String>`**: candidato a
  refino futuro per ADR-0064 Caso A; **NÃO reservado**.

Decisões sobre estes itens ficam para sessões futuras com
informação acumulada, sem pré-comprometimento.

---

## Resumo executivo

P158 confirma factualmente:

1. **ADR-0060 status**: `IMPLEMENTADO`. Fase 2 sub-passo 3
   fechado em P157C (table foundations). **figure-kinds é
   sub-passo seguinte declarado** mas sem detalhes concretos
   na ADR — scope decidido por diagnóstico.
2. **Estado em código**: `Content::Figure` já tem field `kind:
   String` arbitrário; counters por kind já funcionam (P75/P156C).
   Toda a infraestrutura (Image/Table/Raw) já existe.
3. **Subset declarado**: "extension" — sem detalhes.
   Subset minimal recomendado: **auto-detecção de kind baseada
   no body**.
4. **Recomendação de scope**: subset minimal §3.2 em P158A
   isoladamente (S+/M-; granularidade preservada N=13; tests
   ~6-8).
5. **Dependências bloqueantes**: zero hard. Toda a infraestrutura
   existe.
6. **P158A esboço**: helper `infer_kind_from_body` em
   `stdlib/figure_image.rs`; modificação trivial de
   `native_figure`; sem alteração a variant ou layout.
7. **Sem reservas criadas** per nota operacional — supplement,
   show selectors, refactor kind ficam como candidatos não-reservados.

**Auto-validação ADR-0065 critério #5**: este diagnóstico é
**segunda aplicação concreta** de critério #5 (scope) após P157.
Padrão consolidado: "scope determinado por inventário factual"
preserva precedente N=10 de inventariar-primeiro → N=11.
