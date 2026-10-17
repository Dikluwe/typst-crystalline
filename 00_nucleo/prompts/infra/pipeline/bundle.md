# L0 — infra/pipeline/bundle
Hash do Código: e6f4d3a6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/bundle.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

## Current evidence

- `03_infra/src/pipeline/bundle.rs:13` — `introspect_bundle_with_runtime_for_pipeline`.
- `03_infra/src/pipeline/bundle.rs:55` — `BundleFile`.
- `03_infra/src/pipeline/bundle.rs:63` — `BundleCompileOptions`.
- `03_infra/src/pipeline/bundle.rs:77` — `bundle_error`.
- `03_infra/src/pipeline/bundle.rs:82` — `compile_to_bundle_files`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/bundle.rs`.

## Owned contract

- Own `BundleFile`, `BundleCompileOptions`, ordered entry collection, path validation/collision checks, global snapshot composition and per-format rendering. Return the complete collection or diagnostics, never partial success; filesystem publication belongs to wiring.
- Preserve all option fields and explicit absence (`None`) until the relevant format owner resolves document precedence. Keep PDF stream/serialization modes independent and `is_html_document` based on semantic format, not an asset extension.
- Evaluate the root as Bundle only with its explicit feature. Accept documents, assets, tags and whitespace separators at top level; reject loose content, invalid/duplicate normalized paths, unknown formats and multi-page image documents before publication. HTML additionally requires its own feature.
- Preserve ordered bodies and inherited styles. Use one global introspector and causal Location seed per body; delegate preparation of the complete set to `00_nucleo/prompts/infra/pipeline/context_stabilization.md`. Never export bodies from different validated generations or fall back to isolated snapshots.
- Keep heading/state/label history global and Page boundaries local per body. Explicit HTML Page updates remain observable. Do not fabricate metadata padding, derive seeds from element count, hardcode page 1 or repair rendered numbering strings.
- Font resource collection and acquisition follow the resource child owner, without an unconditional catalog request for an empty set.
- Paged bodies use seeded layout; HTML remains semantic except its intrinsic `html.frame` islands. Bind logical page numbers from the final global PageStore with an offset advanced only by paged PDF/SVG/PNG bodies; HTML and assets consume no pages. Misalignment is an error, not a physical-index fallback.
- For a paged body with nonzero offset, project only marginal `logical_numbers` and `visible_numberings` into the layout copy's local indices. Keep total, reference numberings, spans and supplements global and leave the original bundle introspector untouched.
- Treat `SpaceFromNewline` as a separator only outside top-level Document/Asset entries. Preserve its origin inside bodies, preparation and HTML semantic projection until the L1 normalization owner acts.
- Cross-document `.final()` generalization is not authorized by the Page-boundary contract. No new public API, target/default or partial-write policy is introduced here.

The resource owner's private matching-axis predicate governs the early instancer dependency check. Unknown-axis-only requests must not produce a missing-tool diagnostic; retain dependency errors for actual font axes and preserve export dispatch/options.

Para PDF dentro do bundle, delegar o gate e diagnóstico do instanciador ao
helper privado de `infra/pipeline/resources.md`. A medição causal nesse L0
mostra a frase inglesa divergente; preservar a mensagem portuguesa já usada
pelo owner de recursos e pelo PDF isolado. Não alterar o despacho PDF existente.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## PDF interno conserva validações do target — 2026-09-30

### Medição anterior à decisão

`00_nucleo/diagnosticos/auditoria-composicao-construcao-2026-09-29/probe.json` mostra um PDF/UA-1 sem título rejeitado como PDF isolado e aceito dentro do bundle, que publica `one.pdf` com `pdfuaid:part=1` e sem título. A fonte `03_infra/src/pipeline/bundle.rs:463-513` resolve standards e chama diretamente o exporter; `03_infra/src/pipeline/pdf.rs:404-426` executa primeiro as validações PDF-only. A hipótese de omissão seria refutada se a rota bundle chamasse os mesmos gates antes da exportação; não chama.

### Contrato deste owner

Para cada entrada PDF, após resolver páginas, tags e standards, delegar ao helper privado do owner `00_nucleo/prompts/infra/pipeline/pdf.md` as validações pré-exportação de título UA-1, alt UA-1 e caminhos de anexos duplicados, antes de resolver fontes e emitir bytes. Propagar o diagnóstico sem prefixo ou conversão e abortar a coleção completa, preservando a política de publicação atômica. Não replicar as regras PDF, não alterar HTML/SVG/PNG, defaults ou APIs públicas. A rota bundle pode conservar o seu despacho atual de fontes; a equivalência desse despacho exige medição própria.

### Aceitação

O mesmo body PDF/UA-1 inválido por título falha no bundle e no PDF isolado com mensagem e hint iguais; com título válido, o bundle publica o PDF. A ausência de standard UA-1 não cria obrigação de título. Gates de alt/anexos seguem o owner PDF. Fluxo contínuo ADR-0127: esta é a correção de um invariante PDF já vigente, sem contrato público ou fase nova; verificar RED→GREEN.

### Transporte dos metadados por documento — medição complementar

O primeiro GREEN do teste integrado falhou no controle com título: `03_infra/src/pipeline/bundle.rs:156-194` descarta `title/author/keywords` do `Content::Document`, e `render_bundle_document` não atribui `document.document_info`; o PDF isolado atribui `module.document_info()` em `03_infra/src/pipeline/paged.rs:463`. O construtor já valida e preserva `title: Content`, `author` e `keywords` em `01_core/src/compiler/stdlib/structural/document.rs:25-102` e no respectivo L0. A sonda `00_nucleo/diagnosticos/correcao-bundle-pdf-validacoes-2026-09-30/green.log` refuta a hipótese de que bastava delegar a validação: o bundle rejeita também o controle com título enquanto os metadados não forem transportados.

Projetar os metadados de cada `Content::Document` no `DocumentInfo` do seu próprio `PagedDocument` antes da validação/exportação. `title: Some(Content)` produz `Some` do seu texto plano, inclusive quando o conteúdo é vazio; ausência produz `None`. `author` e `keywords` preservam a ordem e são unidos por `, ` como os arrays de `set document` do percurso isolado. A data permanece no carrier existente, sem novo campo em `DocumentInfo` nesta correção. Não propagar o último título de um documento a outro e não inferir título pelo nome do arquivo. O `#set document` de escopo global não substitui os metadados explícitos por entrada nesta fatia.


## Realização com avaliação única

Coleta de entradas de bundle atravessa carriers internos visualmente, sem duplicar documento/asset por proveniência.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `00_nucleo/diagnosticos/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

A delegação transparente já owned de ShowRecipe.body também aceita Source.body, sem executar origem separada, publicar carrier ou atribuir Location. Preservar classificação/preparo/serialização conforme a operação desta unidade.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Carrier transparency repair — 2026-10-13

### Measurement before decision

`03_infra/src/pipeline/bundle.rs:308` projects headings but does not descend
through Source/Located/Realized/ShowRecipe at this operation. Global-introspection
bundle fixtures in `diagnosticos/auditoria-producao-tekt-2026-10-13/html-probes.json.gz`
are accepted by the verified vanilla and rejected by Tekt's HTML exporter.

During the final HTML semantic projection, traverse Source.body, ShowRecipe.body,
Located.body and Realized.visual before projecting headings and labels. Global
introspection has already been prepared; do not re-evaluate recipes, publish
original provenance or alter occurrence identity in the retained global snapshot.
Projection produces only the render copy. Preserve styles, ordered children,
heading diagnostics, and the unmodified HtmlFrame island. Unit REDs compare a
wrapped heading and a visual replacement with the unwrapped semantic projection;
the cross-document query CLI test must pass. No new API, pipeline phase or target
default is introduced (ADR-0127 internal parity correction).


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

- `C100`: `03_infra/src/pipeline/bundle.rs:379`, `render_bundle_document`, 10 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C100` — `render_bundle_document`: Exceção pontual justificada

Fronteira por documento do bundle recebe corpo/formato/metadados e o snapshot global, seed/offset causais, serviços/opções e warnings. Esses dados têm escopos distintos e o L0 exige não misturar documentos ou snapshots; não há necessidade demonstrada de novo agregador.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C100: Fronteira por documento do bundle recebe corpo/formato/metadados e o snapshot global, seed/offset causais, serviços/opções e warnings")], somente nesta função.

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


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

03_infra/src/pipeline/bundle.rs:160–176.
O snapshot before.json.gz em 00_nucleo/diagnosticos/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Consumir o sucesso PreparedBundle do owner context_stabilization por campos nomeados, mantendo prepared_bodies, bundle_intr e document_starts com os mesmos valores e movimentos. Preservar ordem/seed/offset, global candidate e publicação completa; nenhuma API externa muda.

Correção interna em fluxo contínuo ADR-0127: nenhuma entidade, assinatura pública
resolvida, default ou fase é alterada. Uma abreviação de tipo só é permitida com
expansão comprovadamente idêntica; incompatibilidade ou tipo público novo exige
outro gate. As exceções são nominais apenas para type_complexity; não autorizar
supressão em módulo/impl/crate nem elevar limiar. Esta permissão pontual supera
proibições genéricas anteriores somente para os três itens justificados neste lote;
continuam proibidas supressões dos demais casos destinados à correção.
Usar os controles funcionais existentes sem mudar seus esperados. O gate selecionado
deve passar RED→GREEN, incluindo unfulfilled_lint_expectations como erro; revalidar
build, Clippy all-targets, suíte workspace, formato, Rustdoc e linhagem com resselo
restrito. Não criar testes que apenas espelhem aliases ou nomes de campos.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/pipeline/bundle.rs:287 (clippy::replace_box). O snapshot `00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Reutilizar a Box existente do body ao aplicar o envelope Styled: atribuir ao conteúdo via **body o mesmo Content::styled(body.clone(), styles.clone()). O clone do corpo deve ser avaliado antes da substituição. Preservar envelope, ordem, estilos, origem e causalidade; endereço/alocação do Box externo é mecânica privada.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `00_nucleo/diagnosticos/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C100. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.
