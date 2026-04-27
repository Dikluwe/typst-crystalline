# Passo P159D — `BibEntry` fields adicionais (Model bibliography+cite sub-passo 3)

Terceiro sub-passo substantivo de Bibliography + Cite per
candidato Bloco A do diagnóstico P159B §3.3. Materializa **expansão
de struct entity** `BibEntry` com 4 fields adicionais comuns
(`volume`, `pages`, `journal`, `publisher`). **Décima sétima
aplicação consecutiva de materialização** desde início da série
granular P156C.

Refino de tipo entity sem alteração ao variant Content (paridade
P159C que tocou `counter_state.rs` aditivamente). **Hash
`content.rs` preservado** (13º passo consecutivo se confirmado
via L0-baseline interpretation). **Hash `bib_entry.rs` quebrado**
(struct extensão).

**ADR-0065 critério #2** (escolha de tipo) terceira aplicação
concreta — decisão sobre 4 fields adicionais escolhidos vs
fields mais raros diferidos.

---

## Estado actual antes de começar

- 63 ADRs após P159C (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0062 reserva sem ficheiro mantida).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% (impl + impl⁺ inalterada).
  Cobertura ampla 77% inalterada.
- Hash actual `entities/content.rs`: `ec58d849` (preservado
  em **12 passos consecutivos** P156L → P159C via L0-baseline).
- Hash `entities/bib_entry.rs`: `5a2c0ebd` (P159A; **quebra
  esperada em P159D** — struct extensão).
- Hash `entities/citation_form.rs`: `677849cb` (P159C).
- Hash `entities/counter_state.rs`: `4b8e4f02` (preservado em
  P158B/P159C via doc-comment aditivo).
- 1443 tests (lib+integ+diagnostic; workspace 1464); zero
  violations linter.
- 58 variants Content; 48 stdlib funcs.
- Padrões consolidados pós-P159C: granularidade N=16;
  inventariar N=18; Smart→Option Caso A patamar N=6 com
  equilíbrio cross-domínio 50/50; §análise risco N=18;
  estabilidade hash L0 content.rs N=12; tipo entity em
  ficheiro próprio N=5; infraestrutura state lookup N=2;
  P155 cross-feature N=1.

**Diagnóstico P159B** §3.3 (esboço P159D):
- Refino: expandir `BibEntry` struct com 4 fields adicionais
  comuns (`volume`/`pages`/`journal`/`publisher`).
- Sem dependência cruzada hard.
- Hash `content.rs` preservado (struct extensão; não toca
  Content enum).
- Tipo entity refino — quebra hash `bib_entry.rs` (mas só do
  ficheiro próprio).
- Tests Δ: +5-8.
- Granularidade: S+ preservada.

**Política "sem novas reservas" preservada** — P159D não
cria reservas para passos pós-P159D.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-expansao-159-passo-159b.md`
  — §3.3 esboço P159D.
- `00_nucleo/materialization/typst-passo-159a-relatorio.md` —
  precedente directo (`BibEntry` original com 4 fields).
- `00_nucleo/materialization/typst-passo-159c-relatorio.md` —
  precedente imediato (Cite refino estrutural).
- `00_nucleo/adr/typst-adr-0054-perfil-graded.md` —
  fundamento para subset minimal (4+4 = 8 fields; outros
  diferidos).
- `00_nucleo/adr/typst-adr-0065-inventariar-primeiro.md` —
  critério #2 (escolha de tipo) aplicável.
- `01_core/src/entities/bib_entry.rs` — struct actual
  (P159A).
- `01_core/src/rules/stdlib/structural.rs` —
  `extract_bib_entries` helper actual (P159A).
- `01_core/src/rules/layout/mod.rs` — render Bibliography
  actual (formato `"[{key}] {author}. {title} ({year})."`).
- `lab/typst-original/crates/typst-library/src/model/bibliography.rs`
  + `hayagriva::Entry` (vanilla, quarentena) — referência
  para fields universais.

---

## Natureza do passo

**Tamanho**: S+.

**Justificação**: Refino estrutural de tipo entity já
existente. 4 fields adicionais Optional (sem afectar
constructor existente — fields novos default `None`).
Modificação trivial em `extract_bib_entries` para parse de
fields novos. Modificação trivial em layout para render
formato extendido. Tests ~5-8.

Granularidade preservada: 1 feature → mantém N=17 do padrão.

**Risco baixo**:
- Sem alteração ao variant Content (paridade P158B; P159C
  parcial).
- Sem decisões arquiteturais-chave (decisões delegadas a .1
  são selecção de fields, não estruturais).
- Reuso de pattern `extract_bib_entries` (extensão directa).
- Tests pré-existentes (P159A) preservam-se via Optional fields.

---

## Decisões já tomadas

- **4 fields adicionais Optional**:
  ```rust
  pub struct BibEntry {
      // Existing (P159A):
      pub key:    String,
      pub author: String,
      pub title:  String,
      pub year:   u32,
      // Novos (P159D):
      pub volume:    Option<String>,
      pub pages:     Option<String>,
      pub journal:   Option<String>,
      pub publisher: Option<String>,
  }
  ```

  **Selecção de 4 fields universais** per ADR-0065 critério
  #2:
  - `volume` — universal em journals/proceedings/books.
  - `pages` — universal em qualquer publicação com paginação.
  - `journal` — universal em journals (overlap com title em
    livros — distinção semântica útil).
  - `publisher` — universal em livros/tech reports/manuals.

  Outros fields vanilla `hayagriva::Entry` (`url`/`doi`/
  `editor`/`series`/`note`/`isbn`/`location`/etc.) **diferidos**
  per ADR-0054 graded. Adicionáveis em refino futuro NÃO
  reservado.

- **Tipos `Option<String>`**: paridade P159A `key`/`author`/
  `title` (String não-Optional para fields obrigatórios) +
  Optional para fields adicionais (paridade pattern P158A
  `infer_kind_from_body` retornar `Option<String>`).

- **Default `None`**: preserva backwards compat — `BibEntry::new(...)`
  com 4 args originais continua a funcionar (constructor
  expandido tem signature nova `new_full` se necessário,
  ou constructor único com `Option` defaults).

- **Helper privado `extract_bib_entries` extendido**:
  - Continua a aceitar 4 fields obrigatórios (`key`/`author`/
    `title`/`year`).
  - Adiciona parsing opcional de 4 fields novos via
    `dict.get("volume")`/etc.
  - Validação: fields novos Optional — ausência aceite,
    string esperada se presente.

- **Stdlib `native_bibliography` inalterada**: aceita Vec<Dict>
  com fields novos opcionais; sem mudança à assinatura.

- **Layout Bibliography refinado**:
  Antes (P159A): `"[{key}] {author}. {title} ({year})."`.
  Depois (P159D): formatação extendida quando fields presentes:
  - Base: `"[{key}] {author}. {title}"`.
  - Se `journal` presente: append `" {journal}"`.
  - Se `volume` presente: append `" vol. {volume}"`.
  - Se pages presente: append `", pp. {pages}"`.
  - Se publisher presente: append `" {publisher},"`.
  - Final: `" ({year})."`.
  - Exemplo: `"[smith2024] Smith, J. Best paper Nature
    Communications vol. 12, pp. 1-10 (2024)."`.

  Decisão arquitectural-chave deferida a .1: ordem dos
  fields no string final + separadores exactos.

## Decisões diferidas

- **Mais fields além dos 4 escolhidos** (`url`/`doi`/`editor`/
  etc.): NÃO reservados. Candidatos a refino futuro.

- **Parsing de outros formatos** (datas estruturadas, lista
  de autores, etc.): NÃO reservado.

- **Promoção de constructor `new_full` ou pattern builder**:
  diferida — constructor único com Optional fields é mais
  simples. Reavaliar se complexidade crescer.

- **Estilo de citação configurável** (separadores, ordem):
  fora de scope — depende de hayagriva (Bloco B).

- **Algoritmo lookup Cite↔Bibliography** (P159C): inalterado.
  Continua a usar `state.bib_entries` populado por walk.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-bibentry-fields-passo-159d.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
expansão de struct + selecção de fields:

1. Assinatura vanilla `hayagriva::Entry` minimal — confirmar
   que `volume`/`pages`/`journal`/`publisher` são fields
   universais; identificar quais são `Option<String>` em
   vanilla vs estruturas mais complexas.
2. Comportamento observável (fields novos render só se
   presentes; ausência produz output P159A original).
3. ADR-0064 caso aplicável: NÃO directamente em P159D
   (fields novos são `Option<String>` directos sem mapping
   Smart<T>; trivial Optional).
4. Variants Content existentes a estender: nenhum.
   `Content::Bibliography` inalterado; `Content::Cite`
   inalterado.
5. Helpers stdlib reusáveis: `extract_bib_entries` extendido
   (helper privado N=2 usos no mesmo passo: parse 4 obrigatórios
   + 4 opcionais).
6. Limitações aceites (fields restantes vanilla diferidos
   per ADR-0054 graded — `url`/`doi`/`editor`/`series`/`note`/
   `isbn`/`location`/etc.).
7. Tests planeados (constructor com fields novos + parse
   stdlib + render layout extendido + regression P159A
   sem fields — range 5-8 per esboço P159B §3.3).
8. **(Específico expansão struct)** Confirmar que constructor
   pattern continua único (vs `new` simples + `new_full`
   completo). Avaliar tradeoffs.
9. **(Específico ADR-0065 #2)** Documentar decisão sobre
   selecção de 4 fields universais vs alternativas (incluir
   `editor` em vez de `publisher`? incluir `url`/`doi`?).
   Justificação registada.
10. **(Específico layout formato)** Decidir ordem exacta dos
    fields no string final + separadores. Verificar
    expectativas tests existentes vs paridade vanilla.

### .2 Expandir struct `BibEntry`

`01_core/src/entities/bib_entry.rs`:
- Adicionar 4 fields Optional per assinatura em §"Decisões
  já tomadas".
- Constructor único `new(...)` continua a aceitar 4 args
  obrigatórios; fields novos default `None`. Alternativa:
  método `with_*(field, value)` builder pattern se preferido
  em .1.
- Derives mantidos: `Debug`, `Clone`, `PartialEq`, `Eq`.
- `pub mod bib_entry;` em `entities/mod.rs` (já existe;
  confirmar).
- **Hash quebra esperada**: aceitar (struct extensão).

### .3 Extender `extract_bib_entries`

`01_core/src/rules/stdlib/structural.rs`:
- Modificar parsing helper para aceitar 4 fields opcionais.
- Para cada field opcional:
  ```rust
  let volume = dict.get("volume")
      .and_then(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None });
  ```
- Validação: tipo `Value::Str` esperado se presente; outros
  tipos rejeitados com diagnóstico claro mencionando field
  específico.
- Constructor `BibEntry::new(...)` chamado com fields novos
  (ou via builder per decisão .1).

### .4 Refinar layout Bibliography

`01_core/src/rules/layout/mod.rs`:
- Pattern arm `Content::Bibliography` modificado:
  - Construir formatação extendida per §"Decisões já tomadas"
    + decisões finais de ordem em .1.
  - Concatenação condicional baseada em `Option::is_some()`.
  - Helper privado `format_bib_entry(entry: &BibEntry) -> String`
    para legibilidade.

- **`layout_grid` NÃO modificado** (paridade P157A/B/C +
  P159A/C).

### .5 Tests

- **Unit tests `BibEntry`** em `entities/bib_entry.rs` (~3):
  - Constructor com fields novos.
  - PartialEq cobre 8 fields agora.
  - Backwards compat: `new(...)` original com 4 args continua
    a funcionar.

- **Stdlib tests** em `stdlib/mod.rs` (~3):
  - Parse com fields novos.
  - Parse sem fields novos (regression P159A).
  - Tipo errado em field novo rejeitado.

- **Layout E2E tests** em `layout/tests.rs` (~2):
  - Bibliography com entry completa renderiza formato
    extendido.
  - Bibliography com entry mínima (só obrigatórios) renderiza
    formato P159A original (regression).

**Δ esperado**: +5-8 tests (alinhado com esboço P159B §3.3).

### .6 Propagação de hashes

`crystalline-lint --fix-hashes .`:
- `bib_entry.rs` hash quebra: `5a2c0ebd → novo`.
- `content.rs` hash preservado `ec58d849` (13º passo
  consecutivo via L0-baseline) — sem modificação.
- `counter_state.rs` hash inalterado.

Verificar se prompt L0 `bib_entry.md` precisa de actualização
(mencionar fields novos) — decidido em .1 ou subsequente
passo administrativo XS.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1443 + Δ** tests, zero falhas
   (Δ esperado +5-8).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **58** (inalterada — refino
   struct entity).
4. Contagem stdlib funcs: **48** (inalterada — `native_bibliography`
   modificada via helper extendido).
5. **Hash `entities/content.rs` permanece `ec58d849`** —
   **13º passo consecutivo** via L0-baseline interpretation.
6. **Hash `entities/bib_entry.rs` quebra**: `5a2c0ebd → novo`.
   Documentado.
7. Hash actualizado em prompts L0 (`crystalline-lint
   --check-hashes` passa) ou hash preservado (passo refino
   per padrão).
8. Decisão sobre constructor pattern (`new` único vs `new_full`/
   builder) documentada no relatório §"Decisões tomadas em .1".
9. Decisão de 4 fields escolhidos (volume/pages/journal/
   publisher vs alternativas) documentada com justificação
   per ADR-0065 critério #2.
10. Layout formato extendido decidido em .1 e documentado
    no relatório (ordem de fields, separadores).
11. **Sem novas reservas** criadas (paridade política P158).
12. Tests pré-existentes Bibliography (P159A) passam
    inalterados (regression — fields novos default None
    produz output P159A original).
13. `layout_grid` original NÃO modificado (paridade P157A/B/C
    + P159A/C).

---

## Critério de conclusão

- Verificações 1-13 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-159d-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=18 → 19).
  - Slope cumulativo Model (mesa P155-P159D).
  - ADR-0061 §"Aplicações cumulativas" anotada com P159D.
  - **Confirmação**: ADR-0065 critério #2 terceira aplicação
    isolada concreta (selecção de fields); estabilidade hash
    L0 content.rs N=12 → 13; quebra hash bib_entry.rs.
  - **Decisão de fields registada com justificação** —
    importante para refinos futuros (e.g. P159 outro sub-passo
    futuro adiciona url/doi).

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla usa estruturas complexas
  para fields novos (e.g. `journal: JournalRef` em vez de
  `journal: String`) → simplificar para String + documentar
  como graded per ADR-0054.
- Inventário .1 revela que `pages` em vanilla é range
  estruturado (e.g. `pages: PageRange { start, end }`) →
  manter como String formato livre per minimal subset.

**Cenários específicos**:
- Constructor `new(...)` original ter signature usada em
  múltiplos sítios e mudança de signature requerer
  cascading → manter signature original com fields novos
  default None; constructor adicional `new_full` se necessário.
- Layout formato extendido produzir output muito longo em
  uma linha → quebra de linha em formato per ADR-0033 mínimo;
  documentar.
- Tests pré-existentes (P159A) esperarem formato exacto que
  conflite com formato extendido → adaptar tests para
  verificar prefix `[key] author. title` apenas; sufixo
  variável.
- Helper `extract_bib_entries` ter complexidade superior
  com 4 fields opcionais adicionais → refactor para builder
  pattern privado; não-bloqueante.

---

## Notas operacionais

- **Décima sétima aplicação de materialização**. Patamar
  empírico forte. Padrão "0 reformulações mid-passo"
  preserva-se em N=16 aplicações de materialização.
- **§análise de risco no relatório** com peso baixo (refino
  estrutural sem alteração de variant Content). Décima nona
  aplicação consecutiva preserva precedente.
- **Tipo entity em ficheiro próprio N=5** mantido — `BibEntry`
  expande mas continua em `bib_entry.rs`. Subpadrão patamar
  forte (5 entries) — candidato a formalização ADR meta.
- **Estabilidade hash content.rs N=12 → 13**: refactor refino
  de struct entity ortogonal ao variant Content; preserva
  contrato L0 do enum.
- **ADR-0065 critério #2 N=2 → 3**: terceira aplicação
  isolada concreta (P159A BibEntry inicial; P159C
  CitationForm enum; **P159D fields adicionais**). Patamar
  cresce em diversidade — escolha de tipo aplica-se a
  inicial-vs-estendida.
- **Refino estrutural de entity**: precedente novo. P156L
  refinou Pad (variant Content); P159C refinou Cite (variant
  Content). P159D é primeiro a refinar um **tipo entity**
  (não variant) sem afectar Content. Subpadrão emergente
  candidato a registo se replicado.
- **Política "sem novas reservas" preservada** — refinos
  futuros (`url`/`doi`/`editor`/etc.) permanecem candidatos
  NÃO-reservados.

---

## Pós-passo

Após conclusão de P159D:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado** (refino estrutural de entity). **Hash
`entities/content.rs` provavelmente preservado** (13º passo
consecutivo via L0-baseline).

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- **P158C** — Refactor `kind: String → Option<String>` (XS;
  benefício marginal; quebra hash content.rs **inevitável**;
  ADR-0064 Caso A patamar N=6 → 7) — escolha co-ordenada
  com este passo.
- ADR-0062-create (XS administrativo; ainda pendente).
- P159F — Numbering numérico Bibliography (M; counter local).
- Mudança de módulo (Introspection P160).
- Outras direcções pendentes.

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

Padrão granularidade 1-2 features/passo (N=17 com P159D se
fechar sem reformulação) **NÃO** é formalizado em ADR.
Continua candidato.

**Pausa natural após P159D — BibEntry expandido com fields
universais; ADR-0065 critério #2 patamar N=3; estabilidade
hash content.rs N=13. Decisão humana sobre próxima direcção
tem máxima informação. P158C esperado como próximo passo
co-ordenado.**
