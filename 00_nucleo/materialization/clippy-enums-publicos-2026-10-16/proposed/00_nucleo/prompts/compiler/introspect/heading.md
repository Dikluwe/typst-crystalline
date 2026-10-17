# L0 — introspect/heading
Hash do Código: 73293b47

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/heading.rs`.
The current snapshot_fields body at `:22` and TOC projections at `:93`,
`:122` implement the helpers below. The parent calls snapshot_fields while
the causal chain/label is available (`01_core/src/compiler/introspect.rs:2016`);
the Heading branch applies inclusion gates after walking the body.
`01_core/src/compiler/stdlib/structural/heading.rs:127` and
`01_core/src/compiler/eval/rules.rs:1620` delimit the existing producers.
Implemented snapshots must not be described as an awaiting-materialization
proposal, nor as evidence of support for every parser-accepted heading option.

## Walk-branch refinement measurement

At the uncommitted state captured in
`00_nucleo/diagnosticos/atomizacao-introspect-heading-label-2026-09-30-before.json.gz`,
`01_core/src/compiler/introspect.rs:2083-2220` owns the Heading walk branch,
while this consumer already owns snapshot and TOC computations. The `introspect`
test filter passed 263 tests before movement. This is evidence of separable
behavior, not a new timing or bilateral vanilla measurement.

## Complete modeled snapshot

Keep the pure descendant helper receiving HeadingElem, StyleChain and optional
causal Label and returning the ordered public field map:

| Field | Captured value |
| --- | --- |
| level | The modeled heading level as an integer. |
| depth | The modeled level when HEADING_SET_DEPTH is present; otherwise 1. |
| offset | 0, without inventing an unmodeled custom offset. |
| numbering | Exact string only when heading.numbering is Bool(true) and heading.numbering.pattern is Str; otherwise None. |
| supplement | Text Content from the canonical heading language supplement owner at the causal language. |
| outlined | The stored boolean. |
| bookmarked | Explicit boolean, or Auto when absent; not is_bookmarked(). |
| hanging-indent | Auto for the currently modeled producer. |
| body | The frozen semantic body Content, not plain text. |
| label | Causal Label, appended last only when present. |

A stale ancestral pattern does not override a disabled gate.
For a synthetic active gate without a recoverable language string pattern,
capture numbering None; do not derive a pattern from a rendered prefix or
claim parity of that synthetic input. Function/internal non-string patterns
are not fabricated into strings.

Use only `compiler::lang::heading_supplement_for_lang`, owned by
`00_nucleo/prompts/compiler/lang/heading_supplement.md`. Missing language
uses English without mutating StyleChain or activating hyphenation.
Do not load translations at runtime, infer an environmental region or maintain
a competing table. French is Section through the canonical owner.
The pinned snapshot nucleus owns authoritative presence and causal immutability.

## TOC projections

Compute sequential auto-toc labels and location-aware heading numbers without
mutating the introspector. When numbering is active, the auto-label text and
outline prefix are the hierarchical number with a trailing dot; when inactive,
auto-label text is empty and the optional outline number is absent.
Do not add a heading supplement to the outline number.
Return the supplied frozen body and level with the auto label and optional
number. The parent walk allocates the real Heading Location and emits its real Start/End.
The Heading descendant owns its complete walk branch: increments the threaded
auto-label counter, captures numbering from the causal StyleChain, freezes the
body at the Heading Location, walks descendants once through the parent walk,
then emits derived auto-label, TOC and bookmark tags in that order at the same
Location. The descendant independently gates outlined and bookmarked records.
It calls the parent tag-store population helper before each derived Start and
keeps each paired zero-hash End. The parent match remains exhaustive and
delegates this branch by a static free-function call.
Body references' supplemented text belongs to
`00_nucleo/prompts/compiler/introspect/labelled.md`.

## Acceptance and modeled boundary

Cover default/markup/explicit levels, transported string numbering and set none,
causal language/label, snapshot reuse after style changes, auto-label sequencing
and numbered/un-numbered TOC projection. Preserve counter/TOC behavior.

A complete snapshot means complete fields of the modeled producer, not universal
heading parity. Custom supplement/Func/none, numbering Func, custom offset or
hanging indent, untransported region and all set-rule heading fields are not
made supported by this helper. Constructor outlined/bookmarked and explicit
level already exist; do not incorrectly classify those as future features.
Precedence of constructor numbering:none under an ancestral set and show-set
language effects across phases require separate evidence/owner decisions.
No new constructor, callback API, public field or phase is authorized here.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C009`: `01_core/src/compiler/introspect/heading.rs:147`, `walk_heading`, 8 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C009` — `walk_heading`: Exceção pontual justificada

Free function descendente da feature Heading sob ADR-0109: conserva os acumuladores emprestados e a chain causal, delegando a recursão ao walk. A aridade acompanha a atomização estática, sem transferir render para entidades.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C009: Free function descendente da feature Heading sob ADR-0109: conserva os acumuladores emprestados e a chain causal, delegando a recursão ao walk")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### Limite da exceção

Autorizar somente `#[expect(clippy::too_many_arguments, reason = "...")]` no item
nominal acima, com o motivo arquitetural indicado. Não aplicar no módulo, impl,
crate ou configuração global; não elevar o limiar nem abranger outro lint.
Nos casos com remoção de argumento morto, a exceção residual depende dessa correção.
Se a função deixar de exceder o limiar, retirar/revisar a expectativa; não silenciar
`unfulfilled_lint_expectations`. Esta permissão específica supera proibições
genéricas de supressão em seções anteriores somente para este lint e os itens
nominais justificados aqui. As demais obrigações de correção permanecem.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `00_nucleo/diagnosticos/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C009. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.


## Carriers compactos de introspecção/show — proposta 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em 00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16 registra UTC, fontes/L0, hashes, status e git diff HEAD --stat. Os owners atuais foram lidos antes de propor esta alteração. probe-details.json registra rustc 1.92, argv, rlib/binário/sonda com SHA-256 e saída; probe-result.json registra teto de 12 GiB, swap zero, um job e eventos. A sonda é sintética de carriers e do algoritmo de registro/cópia, não benchmark de compilação/layout/RSS nem distribuição de workloads reais. Os candidatos são espelhos com tipos reais de domínio; o caso inline usa Tag e ShowRule produtivos.

Tag mede 416 bytes; Start boxed e Arc medem 48. Para 4.096 pares Start/End com payload Heading sem heap, inline solicita 3.407.872 bytes numa alocação; Box solicita 2.031.616 em 4.097; Arc solicita 2.097.152 em 4.097. Clone de Tag boxed também aloca por Start; os caminhos produtivos lidos em introspect/fixpoint.rs:86–136 e entrypoints.rs:44–66 usam Vec<Tag> temporário e leituras emprestadas, sem clonar o vetor. Walk :325–356 popula o índice por referência antes de mover info para Start. A decisão não toma o clone sintético como frequência produtiva.

Transformation mede 328 e ShowRule 376 bytes; candidatos com Box/Arc<Styles> medem 120 e 168. rules/show.rs:464–470 copia to_vec antes de cada registro; apply.rs:108–112,349–352 filtra/clona regras. Em 128 registros 100% show-set simples sem reter snapshots antigos, inline solicita 12.276.960 bytes/384 alocações, Box 8.194.560/8.640, Arc 5.530.624/512. Com snapshots retidos, picos solicitados são 3.203.856, 4.141.744 e 1.477.808. No controle com canal custom, as cópias de 4.096 regras solicitam 4.107 alocações inline, 8.203 Box e 11 Arc (crescimento do Vec incluído). As leituras de estilos são emprestadas em apply/intercept/outline/table_cells; a aplicação ainda clona o delta quando necessário.

Contraexemplo obrigatório: construir uma única coleção de 4.096 regras 100% show-set simples pede 1.540.096 bytes inline, 2.031.616 Box e 2.097.152 Arc. Arc pode reter capacidade sobrante do canal custom; no controle rico de 128 registros sem snapshots antigos, o final vivo foi 78.064 bytes inline e 156.432 Arc. Não afirmar ganho universal ou ignorar custo atômico/indireção. É mecânica de armazenamento sob ADR-0030/0107, não paridade de linguagem ou ganho cognitivo por IA medido. A inferência favorável a Arc no ciclo de cópias seria refutada por regressão de memória/tempo em um workload real relevante; manter visíveis estes controles.

### Responsabilidade exclusiva proposta

Migrar somente a emissão Tag::Start(loc, info) para Tag::Start(loc, Box::new(info)) nos pontos já existentes. Preservar construção e valores de info, população do índice antes do move, loc, End/hash, ordem de walk e efeitos/contadores. O novo allocation é uma por Start; não clonar info, mover a população, alterar deduplicação, reserva, snapshot ou recursão. A forma pública pertence a 00_nucleo/prompts/entities/tag.md; este owner mantém exclusivamente seu comportamento existente.

### Gate e aceitação

Proposta não vigente até ratificação humana ADR-0127 §2.1/§2.4: há mudança de payload público e compatibilidade de construção/acesso. Não editar código ou owners ativos por este pacote antes da aprovação. Após ratificação instalar estes L0, testes primeiro e RED real, implementação e resselo estritamente dos owners alterados. Não silenciar large_enum_variant, alterar limiar ou adicionar dependências/flags/defaults/fases.

T1: contratos de construção Box<ElementInfo>/Arc<Styles> e orçamento relativo dos carriers (Tag até três Location; Transformation até Content+alinhamento) devem falhar no estado antigo e passar no novo, sem fixar ABI universal. T2: Start/End/Location/payload/label, igualdade e hashes estruturais por valor, clone independente da Box e mutation isolation preservados; hash não vira formato persistido. T3: testes existentes de introspecção/convergência, nesting, heading/label, state/counter/query/metadata continuam com os mesmos esperados e erros. T4: clone de Transformation::Style compartilha o Arc; Arc::make_mut em cópia preserva estilos do original, incluindo custom, e as variantes Func/Content/Str permanecem. T5: controles existentes de show-set/func, precedência, escopo, labels, outline, table.cell e textual/regex mantêm semântica/morfologia/erros; nenhuma regra executa a mais. T6: workspace completo, build, Clippy all-targets e expectativas/lints já fechados como erro, rustdoc -D warnings com quatro índices, fmt, diff, linhagem estrita/reversa e Tekt sem aumento; 12 GiB, swap zero, um job. Registrar HEAD/working tree/hashes/diffstat/UTC e distinguir tamanhos/bytes solicitados de RSS. Não publicar release. Alguma mudança de assinaturas, campos ou fase além destes payloads exige outra proposta.
