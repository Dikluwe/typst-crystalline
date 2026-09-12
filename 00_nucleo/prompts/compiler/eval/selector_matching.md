# Prompt L0 — `compiler/eval/selector_matching` — matching de selectores de show rule
Hash do Código: 06c60a65

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/selector_matching.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval.md` (dono de `compiler/eval/rules.rs`)
**ADRs relevantes**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização)
**Técnica**: pattern matching estrutural

---

## Contexto

Este nó contém as operações puras de matching e conversão de selectores usadas pelo hub `rules.rs` durante a aplicação de show rules:

- converter um `entities::selector::Selector` (query) num `entities::show::Selector` (show rule);
- decidir se um selector de show rule casa com um nó de conteúdo;
- decidir se um selector viaja pela travessia de nós (`map_content`) ou é tratado noutro sítio;
- fatiar texto nas ocorrências de um padrão para aplicação de show rules de texto.

Extraído de `compiler/eval/rules.rs` no Passo 1011 conforme ADR-0109 (atomização — forma B, free function no arquivo da unidade).

---

## Instrução

### 1. Contrato público

```rust
pub(crate) fn query_selector_to_show_selector(
    sel: QuerySelector,
    span: Span,
) -> SourceResult<Selector>;

pub(crate) fn selector_matches(work: &Content, selector: &Selector) -> bool;

pub(crate) fn is_node_rule(selector: &Selector) -> bool;

pub(crate) fn splice_text_rule_matches(
    text: &str,
    pattern: &str,
    mut replacement: impl FnMut(&str) -> SourceResult<Content>,
) -> SourceResult<Option<Content>>;

pub(crate) fn splice_regex_rule_matches(
    text: &str,
    regex: &Regex,
    mut replacement: impl FnMut(&str) -> SourceResult<Content>,
) -> SourceResult<Option<Content>>;
```

- `query_selector_to_show_selector` — converte `QuerySelector` (de `heading.where(level: 1)`, combinadores `And`/`Or`, etc.) para `Selector` de show rule. Rejeita selectors não suportados com mensagem clara.
- `selector_matches` — casa um `Content` contra um `Selector`. Puro: não toca `Engine`/`EvalContext`.
- `is_node_rule` — decide se um selector deve viajar pela travessia de nós (`NodeKind`, `DynKind`, `Where`/`And`/`Or` sobre node-like). `Text`/`Regex`/`Label` retornam `false`.
- `splice_text_rule_matches` — fatia uma string nas ocorrências de `pattern`, substituindo cada match por `replacement(matched)`. Devolve `Ok(None)` se não houver match.
- `splice_regex_rule_matches` — faz o mesmo para todos os matches não vazios
  de uma regex, preservando as fatias não casadas e a ordem. Regex que só casa
  vazio devolve `Ok(None)`; a validação pública do selector ocorre no owner do
  constructor.

### 2. Comportamento

Manter exatamente o comportamento actual:

- `NodeKind`: casamento por tipo de nó, com regras especiais para `List`/`Enum` (sequence uniforme ou item isolado) e para origens sintáticas de `Strong`/`Emph`/`Subscript`/`Superscript`/`Highlight` via `is_styled_origin`.
- `DynKind`: casa `Content::Dynamic` com o mesmo `dyn_kind`.
- `Text`: em `selector_matches`, casa somente `Content::Text` que contém ao
  menos uma ocorrência literal não vazia do padrão.
- `Regex`: em `selector_matches`, casa somente `Content::Text` cujo texto tem
  ao menos um match não vazio da regex.
- `Label`: nunca casa em `selector_matches` (aplicado em `intercept_labelled`).
- `Where`: casa a base e verifica igualdade semântica do campo (`values_eq_semantic`, com coerção Int↔Float).
- `And`/`Or`: curto-circuito; vazios retornam `false`.
- Conversão de query selector: `Kind` → `NodeKind` para elementos nativos suportados; `Where` com base node-like; `And`/`Or` recursivos; resto rejeitado.

### 3. Helper privado

`values_eq_semantic(actual: &Value, expected: &Value) -> bool` move-se com o nó (usado apenas por `selector_matches`). Replica ADR-0025 (coerção Int↔Float) e ADR-0107 (paridade comportamental).

### 4. Gatilhos de reabertura

- Novo tipo de `Selector` ou `QuerySelector`.
- Novo tipo de nó `Content` com regras de matching especiais.
- Mudança de fase (eval ↔ layout) no processamento de show rules.

### P1285 — medição e limite do matching geral

Medição em 2026-08-30: o consumer estava byte-a-byte em HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`; no cristalino,
`selector_matching.rs:148-149` devolve `false` incondicional para Text/Regex,
enquanto os loops dedicados em `rules.rs:639-758` já observam ocorrências em
`Content::Text`. No vanilla ratificado,
`typst-realize/src/lib.rs:1320-1371` encontra regex sobre o texto realizado e
ignora matches vazios; `foundations/selector.rs:149-153` deixa regex fora do
matcher de elementos. Portanto a paridade de língua está no predicado textual,
não na distribuição mecânica entre helpers.

Decisão P1285: completar o helper geral com o mesmo predicado textual local,
sem alterar fase ou substituir os caminhos dedicados de show rule. O matcher
geral não desce por `Sequence`, não concatena nós e não transforma conteúdo;
ele responde apenas sobre o nó recebido. `is_node_rule(Text|Regex)` continua
`false`, logo a mudança não faz esses seletores viajarem pela travessia de
elementos e não duplica aplicação. Os loops dedicados permanecem donos de
precedência, splice, transformação e revogação.

Classificação ADR-0107/0108: “qual helper chama qual” é mecânica; a presença ou
ausência de ocorrência literal/regex é semântica do selector. A decisão seria
refutada se o matcher passasse a casar nó não textual, match vazio ou se um
show rule passasse a aplicar duas vezes.

Medição adicional após a primeira candidata: o loop dedicado de regex em
`rules.rs:639-699` entregava o nó textual inteiro uma única vez à recipe,
enquanto o literal já usava splice por ocorrência. As testemunhas vanilla
`#show regex("f.o")` e `#show regex(".")` entregam cada fatia casada,
preservando prefixo/sufixo e ignorando matches vazios. A unidade pura passa a
possuir também o splice regex; `rules.rs` continua dono de precedência,
revogação e chamada da recipe.

---

## Critérios de verificação

```
Dado selector NodeKind(Heading) e Content::heading → true
Dado selector Where(Heading, level=1) e Content::heading(1) → true
Dado selector Where(Heading, level=2) e Content::heading(1) → false
Dado combinador And vazio → false
Dado combinador Or vazio → false
Dado is_node_rule(Text) → false
Dado Content::Text("abc") e Text("b") → true
Dado Content::Text("abc") e Text("z") → false
Dado Content::Text("abc123") e Regex("[0-9]+") → true
Dado Content::Text("abc") e Regex("[0-9]+") → false
Dado Content não textual e Text/Regex → false
Dado splice_text_rule_matches("aXbXc", "X", …) → sequence ["a", repl, "b", repl, "c"]
Dado splice_regex_rule_matches("foo fxo", /f.o/, …) → sequence [repl("foo"), " ", repl("fxo")]
Dado regex cujo único match é vazio → nenhuma substituição
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.

## P1339 — conversão e matching de identidade nativa

### Medição anterior à decisão

Em HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer sem diff,
`selector_matching.rs:43-82` perde a conversão de identidades não presentes
em ElementKind; `:159-165` usa `Content::get_field` no filtro. Essa leitura
não cobre Text.text/Raw no estado medido (`content.rs:3522-3570`), embora
o acesso público já tenha projeções em `bindings/field_access.rs:996-1049`.
O helper `content_elem_func` nesse outro owner também tem fallback por nome
(`:1146-1171`), que não prova identidade real de builtin.

As fixtures e recibos `p1339-full-show-final-*` medem aplicação real de
filtros strong/emph/text no vanilla; `p1339-full-a2.md` separa rejeição
semântica de falhas de transporte CLI. Não se toma repr de conteúdo ainda
não realizado como prova de aplicação. Fonte de intenção:
vanilla `foundations/selector.rs:134-140`, identidade do elemento e valores
dos campos, não padrão textual.

### Decisão proprietária proposta

- Converter QuerySelector::Element para NativeElement da função, recoberto
  pelos filtros Where necessários. Grupo vazio resulta na base nativa, não
  em And vazio. A forma interna não precisa guardar a morfologia de repr.
- `is_node_rule(NativeElement(_))` é verdadeiro. Text literal, Regex e Label
  mantêm suas rotas; a função nativa text deve casar o nó textual completo,
  não fatias de um padrão e não o texto descendente de um nó arbitrário.
- Matching identifica a função nativa por reconhecimento estático tipado
  contra a identidade do conteúdo; não chama a função e não compara só seu
  nome. Onde já há NodeKind, reutilizar o predicado existente, inclusive
  origens semânticas de strong/emph e tratamento de listas. Preservar a
  distinção entre origem semântica e estilo de render assado.
- Ler os campos linguísticos apropriados do nó. Text.text, Raw.text/lang/
  block e body dos elementos medidos não podem ficar presos a um helper
  que não os projeta. A adaptação é interna à camada compiler; não alargar
  Content nem importar compiler em entities. Para filtros da cadeia cuja
  base é NativeElement, a comparação usa a regra de linguagem do owner
  equality, inclusive valores aninhados, não apenas o derive de Value.
  Where legado sem essa base conserva `values_eq_semantic` e sua projeção
  anterior; não substituir universalmente o comparador/projetor de todos os
  Where. Ausência de campo não é valor default inventado.
- Preservar os comportamentos das variantes antigas. Os casos especiais de
  aplicação por tipo, como Par em `rules.rs`, devem reconhecer a rota nova
  equivalente sem dupla aplicação; esse owner exige L0 próprio atualizado
  antes de código. Esta minuta não muda fase, guardas, precedência ou
  revisitação e não autoriza acomodar diferenças mudando o pipeline.

Testes posteriores: filtro vazio casa só o elemento correto; campos
coincidentes/divergentes em strong/emph/text/raw; coerção em campos e body
morfológico; equivalência de transformação e show-set quando aplicáveis;
NodeKind e NativeElement equivalentes não causam dupla aplicação.
Inferência: o carrier atual de conteúdo basta; caso a realização exija dado
público novo ou mudança de fase, reabrir o gate em vez de ampliar o contrato
implicitamente. Não reivindicar paridade geral de show ou query.

### Integração P1339 — entrada consultada com snapshot

Medição: `entities/introspector.rs:372` e `compiler/stdlib/foundations/query.rs:59`
já transportam IntrospectedContent, enquanto selector_matches recebe apenas
Content. Achatar a entrada antes de filtrar perderia os campos realizados
de Heading (P1307-R5) e a autoridade de Some descrita no Núcleo consumido
por `compiler/introspect.md`.

Este owner fornece helper interno pub(crate) para avaliar a folha
QuerySelector::Element contra IntrospectedContent. Reutilizar a mesma
identidade nativa estática do matching de show; não comparar apenas nomes.
Nos filtros, snapshot Some decide por lookup nessa sequência completa,
sem fallback de campo ausente; None reutiliza a projeção legada de Content.
Comparar valores por `compiler/eval/operators/equality.md`. Não executar
func, reconstruir defaults no caller ou exigir um novo campo em entidade.
O helper não transforma a entrada nem depende de Location para igualdade.
Teste independente deve distinguir duas árvores iguais com snapshots
realizados diferentes, e a ausência de campo de Value::None explícito.
