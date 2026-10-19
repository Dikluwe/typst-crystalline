# L0 — infra/pipeline/resources
Hash do Código: 96969b85

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/resources.rs`

## Current evidence

- `03_infra/src/pipeline/resources.rs:33` — `resolve_and_instantiate_fonts`.
- `03_infra/src/pipeline/resources.rs:93` — `collect_fonts_from_doc`.
- `03_infra/src/pipeline/resources.rs:123` — `collect_fonts_in_items`.
- `03_infra/src/pipeline/resources.rs:172` — `resolve_fonts`.
- `03_infra/src/pipeline/resources.rs:195` — `first_font_from_doc`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/resources.rs`.

## Owned contract

- Own ordered document font collection, fallback resolution, complete FontKey identity and variable-font instantiation. Keep first-occurrence deduplication and declared-family order.
- Traverse background, items, foreground and Group/Link/Semantic transparently. Preserve semantic kind/placement/alt; never flatten the wrapper or turn alt into visible text.
- Include Text, TextShaped and math Glyph resources. Resolve a Glyph's actual font via FallbackFontMetrics::resolve_font_combo, not just body style.font, so a companion MATH glyph cannot be omitted from export selection.
- Keys include FontList, FontVariant and explicit FontVariations. Derived axes plus explicit per-tag overrides determine variable-instancing need and separate resources, even if only an explicit axis differs.
- Require the Python/fontTools availability probe only when a merged request contains an axis actually present in the resolved variable face. Requests containing only unknown axes follow the instancing owner's unchanged-bytes contract; preserve the missing-tool error for matching axes. A private local predicate owns this resource guard without exposing a new API.
- An empty collected font set resolves to an empty resource set without requesting World::book or World::font. All PDF, bundle PDF and raster/SVG callers use this same private resolution boundary.
- A family wins only after selection and source loading both succeed; stale/missing sources continue to later families. Preserve unresolved-entry filtering and variable-instancing diagnostics. Keep test-only first-font helpers internal; production selection uses the complete resolved font set.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/pipeline/resources.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/core/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Recolher a fonte ativa de TextSeparator como recurso textual; conservar a mesma deduplicação de fonte/variante/variações.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Diagnóstico comum de instanciação PDF/bundle — 2026-09-30

### Medição anterior à decisão

Com a mesma fonte variável Ubuntu Sans e `TYPST_CRYSTALLINE_PYTHON` apontando
para executável inexistente, `03_infra/src/pipeline/pdf.rs:444-467` produz
`fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível`;
`03_infra/src/pipeline/bundle.rs:501-518` produz a frase inglesa correspondente.
`03_infra/src/pipeline/resources.rs:39-63` já possui a primeira mensagem para
SVG/PNG. Sem fonte ou com fontes estáticas, ambos exportam; com eixo existente,
ambos rejeitam. A mensagem de erro é observável da linguagem (ADR-0107/0108),
enquanto o número de ramos de exportação e os bytes PDF são mecânica. A
inferência de deriva por duplicação seria refutada se as três rotas já
delegassem o mesmo construtor de diagnóstico; as fontes citadas mostram que não.

### Contrato do owner

Concentrar num helper privado a decisão de dependência e o diagnóstico para
fontes resolvidas. Só consultar a disponibilidade do instanciador se algum
pedido fundido corresponder a eixo real da fonte. Preservar a primeira fonte
correspondente, mensagem portuguesa, span detached e ausência de hint vigentes.
PDF isolado, PDF de bundle e SVG/PNG chamam esse helper; não criar API pública,
mudar seleção de fontes ou alterar o despacho de exportação. O teste pode
injetar apenas o resultado da sondagem para cobrir indisponibilidade de modo
determinístico.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/core/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.

## Coleta textual sem construção de chaves duplicadas

### Medição anterior à decisão

lab/results/construction/diagnostics/font-keys-prioridade7-2026-10-08/before.json.gz registra
HEAD/árvore/UTC e release d7692649. resources.rs:127-134 cria FontList/axes
clonados antes de seen.contains em cada Text/TextShaped/TextSeparator. FontList
contém Vec<FontFamily>; FontVariations contém Vec de eixos, não Arc. O custo de
cópia é mecânica, não paridade da linguagem. Comparação/ordem de seleção e
identidade de fonte/variantes/eixos são observáveis e ficam sob o contrato acima.
Isso mostra construção descartável; o ganho será decidido só após medição.

### Responsabilidade exclusiva

Comparar o key textual completo por empréstimo, conservando a mesma igualdade
de listas/variante/variações vigente, e construir o key owned só na primeira
ocorrência distinta. None e lista vazia de eixos equivalem; conservar igualdade
de f32, inclusive zero com sinal e NaN, sem HashSet ou normalização extra.
Um helper privado pode receber a construção em thunk FnOnce estático, executado
somente no miss; esse thunk é cópia mecânica local, nunca callback da linguagem.
Não mudar API, modelo público, fases, target/defaults, dedup/ordem ou resolução.
Preservar Glyph e seu resolve_font_combo; não inferir sua fonte de style.font.
Preservar recursão e background/items/foreground/containers.

### Aceitação e refutação

RED→GREEN no owner: mil ocorrências do mesmo key constroem um owned key; chaves
distintas seguem ordem, variantes/eixos separam e equivalências antigas permanecem.
Medir redução de construção e tempo separado da CLI; nenhuma contagem autoriza
percentual de tempo sem ensaio. Conservar código somente com benefício repetido
no caminho medido e saída/seleção/diagnósticos preservados. Comparar CLI/PDF/fontes
com release congelado, casos de múltiplas famílias/eixos/containers e matemática;
build/suíte pertinente/lint e resselo apenas deste owner. Experimento interno,
reversível, sem contrato novo: fluxo contínuo ADR-0127.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/pipeline/resources.rs` nas linhas 136 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Eliminar somente a linha física vazia entre a documentação de collect_fonts_in_items e a função a que ela pertence.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Resolver fontes usadas em texto/glifos das células preparadas de fill e stroke, recursivamente, preservando decisões de variações e fallback existentes.

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.


## Simplificações locais de Clippy — 2026-10-15

### Medição anterior à decisão

`03_infra/src/pipeline/resources.rs:213` clona FontVariant, cujo contrato implementa Copy.
`lab/results/construction/diagnostics/avisos-clippy-tekt-2026-10-15/before.json.gz`
conserva fontes, owners, HEAD, working tree não commitado, UTC e diffstat;
selected-red.json identifica os diagnósticos do compilador antes da edição.
São expressões Rust equivalentes, sem nova medição de paridade upstream.

### Responsabilidade e aceitação

Copiar a variante por desreferência ao construir a chave resolvida. Continuar clonando as listas de famílias e variações, conservando ordem, identidade completa e filtragem.

Manutenção interna em fluxo contínuo ADR-0127: conservar contratos públicos,
defaults e fases. Não adicionar supressões nem testes que espelhem sintaxe.
O gate dos avisos selecionados deve passar a GREEN; validar a suíte existente,
build, Clippy all-targets, formato, documentação e linhagem com resselo restrito.


## Inicialização explícita dos fixtures — field_reassign_with_default

### Medição anterior à decisão

`03_infra/src/pipeline/resources.rs` nas linhas 397 cria defaults e em seguida sobrescreve campos locais de testes. A fonte, os spans e os L0 anteriores estão em `lab/results/construction/diagnostics/field-reassign-2026-10-15/before.json.gz`, com HEAD, working tree, UTC e diffstat; initial-diagnostics.json conserva os inicializadores apontados. Esta manutenção trata representação Rust, sem nova medição de paridade ou mudança no default do produto.

### Responsabilidade e aceitação

No helper de teste que gera itens repetidos, inicializar font no literal TextStyle. Manter style mutável e o if axes subsequente que atribui variations; não transformar o ramo condicional em inicialização antecipada. Preservar tamanho da coleção, famílias, valores/ordem dos eixos e contagens de deduplicação.

Usar `..Tipo::default()` para os campos restantes e retirar `mut` apenas onde deixam de existir mutações. Sem novos tipos, builders, APIs ou supressões de lint. L0 precede código e resselo restrito (fluxo contínuo ADR-0127). O gate selecionado deve falhar antes e passar depois; preservar nomes, inputs, expectativas e tolerâncias dos testes. Revalidar Clippy all-targets, suíte workspace, build, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe do inicializador.
