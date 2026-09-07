# Prompt L0 — `infra/query-helpers`
Hash do Código: 913886fe

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## P1307-R5 — snapshot de conteúdo consultado (proposta; gate ADR-0127 pendente)

### Medição anterior à decisão

Baseline R5 `00_nucleo/diagnosticos/p1307-r5-baseline.json`, SHA-256
`32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`:
HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não
commitado com diff/stat integral. A medição independente R5, SHA-256
`82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`,
preserva fontes, horários e executáveis; referência upstream `a51e02804`.

`03_infra/src/query_helpers.rs:459,511–523` retorna Vec<Content>, lê
somente element_at e reconstrói label pelo registry; esse caminho perderia
o snapshot mesmo se a query L1 fosse corrigida. L2 em `cli.rs:758`
preenche defaults de Heading em lugar dos campos públicos realizados.

### Decisão pública proprietária

Substituir apenas Vec<Content> por Vec<Value> na assinatura existente:

```rust
pub fn query_elements(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> (Result<Vec<Value>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

Clonar as entradas completas de intr.elements e construir LocatedContent
com sua Location exata. Para Heading Some usar a label já capturada; não
reconstruí-la pelo selector ou registry. Para famílias com None manter a
projeção legada contratada, inclusive o wrapper Content::Label existente:
se necessário criar nova entrada None com essa árvore rotulada, sem alterar
a entrada original. Nunca converter Some em None para usar esse fallback.

Parsing, ordem, cardinalidade, warnings e escolha atual de árvore pré-show/
realizada não mudam; QuerySummary e query_to_summary mantêm suas assinaturas.
Figure, Equation, Metadata e demais kinds locatáveis não voltam à restrição
headings-only. Nenhum JSON/YAML, repr ou realização de campos entra em L3.
Wiring recebe o vetor inferido e encaminha a L2, sem nova lógica.

Aceitação: mesma query pela linguagem e CLI preserva os fields do recorte,
label ausente/presente; --field/--one serão selecionados por L2, não aqui.
Esta mudança concreta da assinatura pública aguarda aprovação ADR-0127.

---


**Camada**: L3.
**Fase**: P206C / Vanilla integration.
**ADRs vinculantes**: ADR-0075 (vanilla integration via
pre-built CLI + comparação estrutural; PROPOSTO).
**Cross-references**: P206A diagnóstico (auditoria
empírica); P206C C2 = Caminho B (helper em workspace
cristalino); ADR-0073 §C6a fechada por F3 (P205B+C);
trait `Introspector` (M8 P204B); `entities/selector.md`
(P175 minimal).

---

## Contexto

P206A auditou vanilla integration empíricamente e fixou
ADR-0075 PROPOSTO. P206C materializa comparação
estrutural cristalino vs vanilla via JSON shape
compatível com `typst query`.

P206C C2 fixou **Caminho B** (helper em workspace
cristalino, não subcomando CLI exposto):

- Caminho A (subcomando CLI em `04_wiring/`) era **L**
  por exigir refactor cross-modular (`main.rs` +
  Selector parsing + JSON shape vanilla replication +
  L0 prompts updated).
- Caminho B é **M** — módulo dedicado L3 com selector
  parsing + dispatch a `Introspector::query_*` +
  output domain struct.
- Caminho C (helper em `lab/parity/`) contradiria
  clarificação inicial ("cristalino expõe helper").

C3 documenta resolução parcial da tensão: clarificação
"novo CLI cristalino" honrada via helper L3 (cristalino
expõe via API pública); CLI subcomando deferred para
sub-passo dedicado pós-P206.

---

## Decisão

### P1140.19 — transparência estrutural de `PageRun`

`Content::PageRun` é contentor lexical de página, não uma barreira para query.
Os walkers puros `has_any_text` e `count_variant` descem em `PageRun.body`, do
mesmo modo que descem em `Styled` e `Par`. A configuração de página não conta
como texto nem como variante consultada por esses helpers. Não interpretar,
aplicar ou restaurar `PageConfig` em L3; isso pertence ao consumer L1 de layout.

`03_infra/src/query_helpers.rs` — módulo L3 que expõe:

1. **Selector parsing** — string → enum interno
   discriminando entre:
   - **Kind** (ex: `"heading"`, `"figure"`,
     `"metadata"`) → `Selector::Kind(ElementKind)`
     via `ElementKind::from_str`.
   - **Label** (ex: `"<fig-alfa>"`) → label string
     extraído entre `<...>` → `Introspector::query_by_label`.
   - **Inválido** → erro com mensagem.

2. **Query execution** — pipeline integrado:
   - Eval source via `eval_to_module_with_sink`.
   - Extrair Content via `module.content`.
   - Construir TagIntrospector via `introspect`.
   - Aplicar selector → dispatching a método correcto.
   - **P494**: para os kinds `List`, `Enum`, `Par`, `Link`, `Raw`, `Quote`, `Footnote`, o count é obtido por análise directa do `Content` (função `count_element_in_content`), porque cristalino não materializa containers `List`/`Enum`/`Par` e os demais elementos (`Link`/`Raw`/`Quote`/`Footnote`) ainda não são locatable em L1. O `Introspector` é usado para os kinds tradicionalmente locatable (`heading`, `figure`, etc.) e para labels.
   - Retornar **`QuerySummary`** estrutura mínima.

3. **`QuerySummary`** — struct domain-level (sem
   serde):
   ```text
   pub struct QuerySummary {
       pub selector: String,         // input literal
       pub kind: SelectorKind,       // Kind | Label
       pub count: usize,             // count de matches
       pub kind_name: Option<String>, // nome do kind se Kind selector
       pub label_found: Option<String>, // label se Label selector com match
       pub metadata_values: Vec<String>, // se selector="metadata", plain text de cada value
   }
   ```

4. **`QueryError`** — enum:
   - `EvalFailed` (eval produziu erros).
   - `NoContent` (eval ok mas sem content).
   - `InvalidSelector(String)` (selector não parseável).
   - `WorldError(String)` (I/O ou source loading).

5. **P494 — Contagem por análise do `Content`**:
   Função pura `count_element_in_content(content: &Content, kind: ElementKind) -> usize`:
   - `ElementKind::List`: conta `Sequence`s cujos filhos são todos
     `Content::ListItem`. Se a raiz for um único `ListItem`, conta 1.
   - `ElementKind::Enum`: conta `Sequence`s cujos filhos são todos
     `Content::EnumItem`. Se a raiz for um único `EnumItem`, conta 1.
   - `ElementKind::Par`: retorna 1 se o `Content` contiver qualquer
     `Content::Text` (aproximação — cristalino não materializa
     `ParElem`).
   - `ElementKind::Link`: conta `Content::Link`.
   - `ElementKind::Raw`: conta `Content::Raw`.
   - `ElementKind::Quote`: conta `Content::Quote`.
   - `ElementKind::Footnote`: conta `Content::Footnote`.
   - Outros kinds: `0` (delegados ao `Introspector`).

5. **Função pública principal**:
   ```text
   pub fn query_to_summary(
       world: &SystemWorld,
       source: &Source,
       selector: &str,
   ) -> Result<QuerySummary, QueryError>;
   ```

6. **Funções auxiliares pub** (úteis para callers
   isolados):
   - `parse_selector(s: &str) -> Result<ParsedSelector, QueryError>`.
   - `summarize_query(intr: &TagIntrospector, content: &Content, parsed: &ParsedSelector) -> QuerySummary`.
     Recebe o `Content` para contagem dos kinds P494.

---

## Restrições

- **L3 zero deps externas novas** — sem `serde_json`
  em 03_infra (lab/parity converte via own dep).
- Domain struct (`QuerySummary`) não implementa
  `Serialize`; lab/parity usa `to_string()` ou
  manual JSON build.
- Selector parsing minimal: aceita Kind names
  (`ElementKind::from_name`) + label syntax `<label>`. P494 expande
  para 17 kinds incluindo `list`, `enum`, `par`, `link`, `raw`,
  `quote`, `footnote`. Outras formas vanilla
  (`heading.where(...)`, `figure.where(kind: image)`)
  → `InvalidSelector`. Documentado.
- Sem dependência circular: 03_infra usa 01_core; não
  invertido.

---

## Coerência arquitectónica

- Pattern paralelo a `pipeline.rs` (entry point para
  pipeline cristalino completo) — `query_helpers.rs` é
  entry point para pipeline + query.
- Reusa `eval_to_module_with_sink` (não duplicação).
- Reusa `introspect` (L1).
- L3 hosting é correcto: pipeline integration; sem I/O
  novo; complementa `pipeline.rs`.

---

## Tests

`#[cfg(test)] mod tests` com:

1. `parse_selector_kind_basico` — `"heading"` →
   `ParsedSelector::Kind(Heading)`.
2. `parse_selector_label_basico` — `"<fig-alfa>"` →
   `ParsedSelector::Label("fig-alfa")`.
3. `parse_selector_invalido` — `"weird.where()"` →
   `InvalidSelector`.
4. `parse_selector_kind_metadata` — `"metadata"` →
   `ParsedSelector::Kind(Metadata)`.
5. `summarize_kind_count` — corpus mínimo com 2
   headings → `count == 2`.
6. `summarize_label_match` — corpus com 1 label
   "fig-alfa" → `label_found == Some("fig-alfa")` +
   `count == 1`.
7. `summarize_metadata_values` — corpus com 3 metadata
   → `metadata_values.len() == 3`.
8. **P494** — `parse_selector` aceita `list`, `enum`, `par`, `link`,
   `raw`, `quote`, `footnote` → `ParsedSelector::Kind(...)`.
9. **P494** — `query_to_summary` retorna `count == 1` para corpus
   mínimo de cada novo selector (ex.: `#list([A])`, `#link(...)`).

---

## Cross-impl considerations

- TagIntrospector field `metadata: MetadataStore` (P169);
  `query_metadata()` retorna `&[Value]`; cada Value
  com `Content(c).plain_text()` ou similar para output
  textual.
- `query_by_label(&Label)` retorna `Option<Location>`;
  `count` = `0` ou `1`.
- `query_by_kind(ElementKind)` retorna `Vec<Location>`;
  `count` = `len()`.

---

## Não-objectivos

`query_helpers.rs` não:

- Implementa subcomando CLI (Caminho A; deferred).
- Replica formato JSON vanilla literal (tarefa de lab/parity
  consumer).
- Estende `Selector` enum em L1 (P175 minimal mantém-se).
- Adiciona `Serialize` derive em tipos cristalinos
  (separação via domain struct).
- Materializa `Selector::Label` ou `Selector::Where`
  em L1 (futuro; out-of-scope P206).

---

## P480 — `math.equation` alias em `parse_selector`

`parse_selector("math.equation")` → `ParsedSelector::Kind(ElementKind::Equation)`.

Vanilla rejeita `equation` standalone; aceita `math.equation` como namespace.
O alias é tratado antes do guard `.contains('.')` para não ser rejeitado
como selector complexo. `equation` standalone continua a funcionar internamente.

Novos testes:
- `p480_parse_selector_math_equation_resolve_equation_kind`
- `p480_parse_selector_equation_standalone_ainda_aceito`

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-05-08 | P206C: helper L3 para comparação estrutural cristalino vs vanilla | `query_helpers.rs`, `query-helpers.md` |
| 2026-05-12 | P480: alias `math.equation` em `parse_selector` | `query_helpers.rs`, `query-helpers.md` |
| 2026-06-29 | P494: expansão de selectores para `list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`; contagem por análise do `Content` para kinds sem container locatable em L1 | `query_helpers.rs`, `query-helpers.md`, `element_kind.rs`, `element_kind.md`, `foundations.rs` |
| 2026-08-10 | P992: `Content::MathLimitsOverride` (`limits()`/`scripts()`) — braço novo nas duas listas de variantes terminais sem texto próprio, mesmo tratamento de `MathAccent`/`MathCancel`/`MathClassOverride`/`MathOp` | `query_helpers.rs`, `query-helpers.md` |
| 2026-08-31 | P1292 amendment-1: `MathUnderline`, `MathVec` e `Flush` são terminais neutros em `has_any_text`/`count_variant` | `query_helpers.rs`, `query-helpers.md` |

## P1137-I-001 — promoção de resultados locatáveis para a CLI

### Medição anterior à decisão

Em 2026-08-23:

- este L0 ainda fixa o Caminho B e declara o subcomando CLI como não-objetivo;
- `query_to_summary` em `query_helpers.rs:419-437` descarta os elementos e
  devolve somente contagem/metadados textuais;
- `Introspector::element_at` (`entities/introspector.rs:705-710`) já devolve o
  `Content` locatável registrado pelo walk;
- no corpus `= First`, o vanilla ratificado retorna um array JSON com um
  heading; `--field level` retorna `[1]`; `--one` retorna o objeto sem array;
- o vanilla avisa que `query` é deprecated, mas mantém o comando e recomenda
  `eval 'query(...)' --in ...`.

Classificação: sequência e campos do elemento são semântica/morfologia da
linguagem; sintaxe do comando e JSON são CLI pública. Inferência: para headings,
os dados já existentes em `HeadingElem` + `element_at` bastam. Refutação: algum
match de heading não possuir `element_at`, perder ordem ou divergir nos campos
medidos.

### Decisão

A proibição anterior de subcomando CLI é substituída apenas para o escopo
P1137-I-001. Acrescentar:

```rust
pub fn query_elements(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> (Result<Vec<Content>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

A função reutiliza `eval_to_module_with_sink`, o conteúdo original pré-show e
`introspect`; aplica `ParsedSelector`; resolve cada location com `element_at` e
preserva ordem. Selector inválido torna-se diagnóstico, não panic. Warnings de
eval são devolvidos separadamente.

Escopo deliberadamente incompleto: nesta entrega, a garantia pública é apenas
selector simples `heading` e label que resolva para heading. Kinds sem elemento
locatável, selectors complexos e serialização de outros `Content` continuam no
helper de resumo ou falham claramente. O passo que ampliar cada família deve
atualizar este L0 antes do código.

Testes RED→GREEN: um heading retorna um `Content::Heading` com corpo e nível;
dois headings preservam ordem; selector inválido retorna diagnóstico; selector
sem matches retorna vetor vazio.

## P1285 — resultados locatáveis não-heading

### Medição anterior à decisão

Em 2026-08-30, sobre HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, o consumer estava sem diff local.
As sondas públicas do subcomando recuperaram `Content::Metadata` e
`Content::Figure` e só falharam depois, no serializer L2 com
`query serialization currently supports headings only`. Logo a limitação
observada não está na recuperação L3. Na fonte pinada do vanilla,
`typst-cli/src/query.rs:71-123` recupera qualquer selector locatável, preserva
ordem e entrega os `Content` ao serializer genérico.

Classificação ADR-0107/0108: a lista e ordem de elementos recuperados são
semântica/morfologia; `Introspector`, `element_at` e a divisão L2/L3 são
mecânica livre. A inferência de que o helper já é suficientemente genérico
seria refutada por um kind locatável presente no `Introspector` cuja location
não pudesse ser resolvida para `Content`.

### Decisão

A garantia heading-only de P1137-I-001 é substituída: `query_elements` aceita
todo selector simples já reconhecido por `ParsedSelector` que corresponda a
elementos locatáveis no `Introspector`, e labels que resolvam para qualquer
desses elementos. Para cada location, devolve o `Content` original em ordem
documental. Quando o `LabelRegistry` associa a location a uma label explícita,
o helper preserva esse metadado envolvendo o resultado em `Content::Label`
com o elemento original como body; isso é transporte morfológico para L2, não
um match adicional nem um elemento extra. Não filtra por variante e não
prepara JSON/YAML em L3.

Kinds deliberadamente não locatáveis que só possuem contagem aproximada no
helper histórico de resumo não são promovidos artificialmente a `Content`.
Texto e regex continuam não locatáveis. Selectors complexos permanecem fora do
parser L3 enquanto não tiverem contrato próprio; isto não autoriza devolver
um heading substituto nem perder silenciosamente um elemento locatável.

Aceitação: heading permanece controlo; figure, equation e metadata são
testemunhas não-heading obrigatórias; labels para esses kinds preservam o mesmo
elemento; zero matches devolve vetor vazio; selector não locatável ou inválido
continua diagnóstico. Serialização e `--field` pertencem exclusivamente ao
owner L2 `shell/cli.md`.

## P1292 amendment-1 — neutralidade dos helpers para B/C/D

### Medição anterior à decisão

Os walkers puros atuais descem somente nos contentores enumerados e devolvem
`false`/`0` para a família math terminal (`MathSequence`, `MathFrac`,
`MathMatrix`, `MathCases`, `MathAccent`, `MathCancel`, overrides, underover,
op e styled) em `03_infra/src/query_helpers.rs:274-346,349-424`.
`MathUnderline`, `MathVec` e `Flush` ainda não estavam representáveis no
`match`. Seus L0s P1292 os definem como não-locatáveis; B/C são terminais math
para introspecção e D é sentinela de fluxo sem texto
(`entities/elements/math_underline.md`, `entities/elements/math_vec.md`,
`entities/elements/flush.md`; `compiler/introspect.md`, amendment-1).

No vanilla ratificado `a51e02804`, underline/vec são resolvidos dentro do IR
math (`math/ir/resolve.rs:198-211,1002-1025,1253-1261`) e `FlushElem` vira
`Child::Flush` no coletor, sem body (`layout/place.rs:179-213`;
`typst-layout/src/flow/collect.rs:79-91`). Não existe nessa medição conteúdo
textual próprio nem identidade locatável que estes helpers L3 devam inventar.

### Decisão

Nos braços terminais de `has_any_text`, `Content::MathUnderline(_)`,
`Content::MathVec(_)` e `Content::Flush(_)` devolvem `false`. Nos braços
terminais de `count_variant`, devolvem `0`, sem descer no body/children e sem
transformar a sentinela em variante consultável. O pre-check genérico que
retorna `1` quando o próprio `predicate(content)` é verdadeiro permanece
inalterado; nenhum selector aproximado suportado por
`count_element_in_content` tem B/C/D como predicate ou `ElementKind`.

Classificação ADR-0107/0108: `false`/`0` significa ausência de match textual ou
de elemento aproximado na superfície de query; a travessia Rust é mecânica.
A terminalidade é inferência compatível com os L0s P1292 e o tratamento
vanilla como IR math/sentinela de fluxo. Seria refutada se B/C/D ganhassem
`ElementKind` locatável neste owner, se a linguagem passasse a selecionar o
texto interno de um elemento math por este helper aproximado, ou se flush
adquirisse body/texto. Até essa refutação, recursão que conte filhos internos
seria um falso positivo de query.
