# Prompt L0 — `entities/tag`
Hash do Código: db0d0c0a

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/tag.rs`
**Criado em**: 2026-04-30 (P161 sub-passo .9)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`Tag` marca início e fim de um elemento indexável durante a passagem de introspecção. Forma análoga a `lab/typst-original/crates/typst-library/src/introspection/tag.rs::Tag` (linha 12), com simplificação: apenas duas variantes `Start` e `End`.

Walk em P162 percorrerá `Content` e, para cada `Content::Heading`/`Content::Figure`/`Content::Cite`, emitirá `Tag::Start(loc, ElementInfo)` antes de descer para o body e `Tag::End(loc, content_hash)` ao subir. A sequência de tags é o registo cronológico do walk.

P161 cria apenas a definição do tipo. Walk não emite tags ainda — isso é P162. Em P161 nenhum consumer ergue `Tag`.

---

## Restrições Estruturais

- Camada **L1**: enum puro.
- `Clone` derivado para que stream de tags possa ser clonado/passado por valor entre fases.
- Sem `Arc` interno — `ElementInfo` é clonado por valor (custo aceitável: payload + Option<Label>, sem Content).

---

## Interface pública

```rust
use crate::entities::element_info::ElementInfo;
use crate::entities::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    /// Início de um elemento indexável.
    /// Carrega Location única (gerada por Locator) e o payload+label do elemento.
    Start(Location, ElementInfo),

    /// Fim do elemento. Carrega a Location correspondente ao Start
    /// (para emparelhamento) e um content_hash u128 — paridade vanilla
    /// para detecção de duplicação cross-iteration.
    End(Location, u128),
}
```

---

## Semântica

- `Tag::Start(loc, info)`: marca o ponto onde o walk entra num elemento indexável. `loc` é única na sequência (gerada por `Locator::next()`). `info` carrega o payload e a label opcional.
- `Tag::End(loc, content_hash)`: marca a saída do elemento. `loc` é a mesma do `Start` correspondente (emparelhamento Start↔End). `content_hash` é o hash do conteúdo (Content) do elemento, populado em `walk` via `hash_content` (`entities/content_hash.rs`, P162 .B+.E). O vanilla também guarda o hash no `End` — mas por razão de tamanho de memória, não de queries; ver bloco abaixo.

> **Correcção P1031 — o motivo atribuído ao vanilla está refutado pela própria fonte.**
>
> A redacção anterior dizia: *"Paridade com vanilla, onde o hash é guardado no End para
> **optimização de queries**."* O vanilla afirma exactamente o contrário quanto ao motivo.
> Citação literal do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/introspection/tag.rs:10-22`:
>
> ```rust
> /// Marks the start or end of a locatable element.
> pub enum Tag {
>     /// The stored element starts here.
>     ///
>     /// Content placed in a tag **must** have a [`Location`] or there will be
>     /// panics.
>     Start(…),
>     /// The element with the given location and key hash ends here.
>     ///
>     /// Note: The key hash is stored here instead of in `Start` simply to make
>     /// the two enum variants more balanced in size, keeping a `Tag`'s memory
>     /// size down. There are no semantic reasons for this.
>     End(…),
> }
> ```
>
> — *"simply to make the two enum variants more balanced in size, keeping a `Tag`'s memory
> size down. **There are no semantic reasons for this.**"*
>
> **Duas consequências.** (1) O *facto* (hash no `End`) confirma-se e é citação literal; o
> *motivo* estava inventado e foi corrigido. (2) Mais importante: como o vanilla declara que
> **não há razão semântica**, isto não é matéria de paridade de linguagem — é uma escolha de
> empacotamento de memória, terreno onde o cristalino diverge de propósito (ADR-0107 /
> ADR-0030). Copiar a colocação é aceitável, mas não pode ser justificado como paridade nem
> tratado como restrição.
>
> **Nota adicional da mesma fonte**, não registada neste L0: o `Start` do vanilla exige
> `Location` sob pena de panic (*"Content placed in a tag **must** have a `Location` or
> there will be panics."*), e existe um `TagFlags { introspectable, tagged }`
> (`tag.rs:47-56`) que distingue o que entra no `Introspector` do que é apenas *tagged* —
> distinção que o `Tag` do cristalino não modela. Lacuna documentada.

---

## Invariantes

- Cada `Start(loc, _)` é seguido (eventualmente) por exactamente um `End(loc, _)` na sequência produzida por um walk completo. Sub-walks aninhados produzem Start/End correctamente aninhados.
- A `Location` em `Start` e `End` correspondentes é exactamente a mesma (igualdade `Eq` no `u128` interno).
- Fora de uma sequência de walk, `Tag`s podem ser comparados por valor (`Eq`/`Hash` derivados) — útil para tests de regressão.

---

## Consumers actuais

Nenhum em P161 — walk não emite tags ainda. `Tag` existe como definição passiva.

## Consumers planeados

- `rules/introspect.rs` walk em P162 — emite `Vec<Tag>` em paralelo ao `CounterStateLegacy` actual. Walk continua a fazer o que faz hoje (avançar contadores, popular labels) e adicionalmente passa a coleccionar tags.
- `Introspector` (M3) — consome a sequência de tags para construir o índice de elementos.
- Snapshot tests em P163 — comparação determinística de sequências de tags entre execuções equivalentes.

---

## Sobre paridade

Vanilla `Tag` em `tag.rs` linha 12:

```rust
pub enum Tag {
    Start(Content, TagFlags),
    End(Location, u128),  // u128 = key hash
}
```

Diferenças cristalino P161:

- Vanilla `Start(Content, TagFlags)`: carrega o `Content` inteiro do elemento e flags (`introspectable`, `tagged`). Cristalino `Start(Location, ElementInfo)`: separa Location explícita (vanilla extrai-a do Content) e usa `ElementInfo` (subset focado) em vez de Content completo.
- Vanilla `TagFlags` (struct com 2 bools): scope-out em P161. Não há tagged-PDF nem distinção introspectable/non-introspectable porque todos os elementos no enum cristalino `ElementKind` são introspectable.
- Vanilla `TagElem` (`#[elem]` wrapper): scope-out — cristalino não usa vtable elements; tags fluem como `Vec<Tag>` do walk para o consumer.
- `End` é estruturalmente idêntico (`(Location, u128)`).

Ver `lab/results/construction/diagnostics/inventario-tipos-introspection-vanilla.md` (2026-04-30) §3 para classificação de `TagFlags` (concept needed; forma vanilla aplicável quando relevante) e `TagElem` (vtable — scope-out).

---

## Resultado Esperado

- `01_core/src/entities/tag.rs` — enum + tests unitários (construção Start/End, igualdade entre tags equivalentes).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P161 sub-passo .9: marcadores Start/End para introspecção fixpoint M1 | `tag.rs`, `tag.md` |


## Carriers compactos de introspecção/show — proposta 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16 registra UTC, fontes/L0, hashes, status e git diff HEAD --stat. Os owners atuais foram lidos antes de propor esta alteração. probe-details.json registra rustc 1.92, argv, rlib/binário/sonda com SHA-256 e saída; probe-result.json registra teto de 12 GiB, swap zero, um job e eventos. A sonda é sintética de carriers e do algoritmo de registro/cópia, não benchmark de compilação/layout/RSS nem distribuição de workloads reais. Os candidatos são espelhos com tipos reais de domínio; o caso inline usa Tag e ShowRule produtivos.

Tag mede 416 bytes; Start boxed e Arc medem 48. Para 4.096 pares Start/End com payload Heading sem heap, inline solicita 3.407.872 bytes numa alocação; Box solicita 2.031.616 em 4.097; Arc solicita 2.097.152 em 4.097. Clone de Tag boxed também aloca por Start; os caminhos produtivos lidos em introspect/fixpoint.rs:86–136 e entrypoints.rs:44–66 usam Vec<Tag> temporário e leituras emprestadas, sem clonar o vetor. Walk :325–356 popula o índice por referência antes de mover info para Start. A decisão não toma o clone sintético como frequência produtiva.

Transformation mede 328 e ShowRule 376 bytes; candidatos com Box/Arc<Styles> medem 120 e 168. rules/show.rs:464–470 copia to_vec antes de cada registro; apply.rs:108–112,349–352 filtra/clona regras. Em 128 registros 100% show-set simples sem reter snapshots antigos, inline solicita 12.276.960 bytes/384 alocações, Box 8.194.560/8.640, Arc 5.530.624/512. Com snapshots retidos, picos solicitados são 3.203.856, 4.141.744 e 1.477.808. No controle com canal custom, as cópias de 4.096 regras solicitam 4.107 alocações inline, 8.203 Box e 11 Arc (crescimento do Vec incluído). As leituras de estilos são emprestadas em apply/intercept/outline/table_cells; a aplicação ainda clona o delta quando necessário.

Contraexemplo obrigatório: construir uma única coleção de 4.096 regras 100% show-set simples pede 1.540.096 bytes inline, 2.031.616 Box e 2.097.152 Arc. Arc pode reter capacidade sobrante do canal custom; no controle rico de 128 registros sem snapshots antigos, o final vivo foi 78.064 bytes inline e 156.432 Arc. Não afirmar ganho universal ou ignorar custo atômico/indireção. É mecânica de armazenamento sob ADR-0030/0107, não paridade de linguagem ou ganho cognitivo por IA medido. A inferência favorável a Arc no ciclo de cópias seria refutada por regressão de memória/tempo em um workload real relevante; manter visíveis estes controles.

### Responsabilidade exclusiva proposta

Substituir exclusivamente Start(Location, ElementInfo) por Start(Location, Box<ElementInfo>); End(Location,u128), derives e conjunto fechado de variantes permanecem. Conservar propriedade exclusiva e clone profundo do payload, sem Arc no Tag. Campos de ElementInfo/ElementPayload não mudam. As justificativas históricas de payload pequeno não são prova do custo atual; o novo contrato supersede apenas a forma inline anterior de Start. Atualizar os construtores de testes deste consumer com Box::new, sem alterar dados/esperados. Nenhum helper público novo, dispatch dinâmico ou arquivo de domínio com I/O. A API Rust externa deve envolver info em Box::new; leituras emprestadas usam auto-deref, consumo do payload pode usar *info. A ordem/pareamento Start/End, Locations, conteúdo indexado e decisões de convergência permanecem da linguagem vigente.

### Gate e aceitação

Proposta não vigente até ratificação humana ADR-0127 §2.1/§2.4: há mudança de payload público e compatibilidade de construção/acesso. Não editar código ou owners ativos por este pacote antes da aprovação. Após ratificação instalar estes L0, testes primeiro e RED real, implementação e resselo estritamente dos owners alterados. Não silenciar large_enum_variant, alterar limiar ou adicionar dependências/flags/defaults/fases.

T1: contratos de construção Box<ElementInfo>/Arc<Styles> e orçamento relativo dos carriers (Tag até três Location; Transformation até Content+alinhamento) devem falhar no estado antigo e passar no novo, sem fixar ABI universal. T2: Start/End/Location/payload/label, igualdade e hashes estruturais por valor, clone independente da Box e mutation isolation preservados; hash não vira formato persistido. T3: testes existentes de introspecção/convergência, nesting, heading/label, state/counter/query/metadata continuam com os mesmos esperados e erros. T4: clone de Transformation::Style compartilha o Arc; Arc::make_mut em cópia preserva estilos do original, incluindo custom, e as variantes Func/Content/Str permanecem. T5: controles existentes de show-set/func, precedência, escopo, labels, outline, table.cell e textual/regex mantêm semântica/morfologia/erros; nenhuma regra executa a mais. T6: workspace completo, build, Clippy all-targets e expectativas/lints já fechados como erro, rustdoc -D warnings com quatro índices, fmt, diff, linhagem estrita/reversa e Tekt sem aumento; 12 GiB, swap zero, um job. Registrar HEAD/working tree/hashes/diffstat/UTC e distinguir tamanhos/bytes solicitados de RSS. Não publicar release. Alguma mudança de assinaturas, campos ou fase além destes payloads exige outra proposta.

## Ratificação humana — 2026-10-16

Proposta aprovada pelo humano, manifesto SHA-256 `282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`. O contrato acima passa a vigente; inclui materialização/testes T1–T6, sem release. Registro em `lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`.
