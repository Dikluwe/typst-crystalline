# L0 — compiler/eval/rules/textual/space
Hash do Código: fe38fe23

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/space.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the working tree captured in `lab/results/construction/diagnostics/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual.rs:335-463` contains space origin, neighbor visibility, wrapper text, normalization and Unicode classification. The `p1494` filter passed 17 tests on that state. This is a structural baseline, not a timing or new vanilla measurement.

## Decision and behavior

Own `normalize_markup_spaces` and its helpers in this descendant module. Before literal/regex show on realized flow/root, collapse adjacent spaces to one occurrence whose newline origin is the OR of its components. Discard only an inferred newline occurrence when adjacent internal textual characters satisfy the pure pinned Unicode nonspacing classification (Han/Bopomofo/Hiragana/Katakana/Yi and measured ideographic punctuation, not Korean or ambiguous curly quotes). Use immutable data and the authorized icu_properties dependency. Keep explicit space between supporting texts when no component has newline. Explicit Linebreak is destructive and consumes adjacent spaces, including explicit ones, but remains Linebreak. Parbreak and actual nontext content remain causal boundaries. A textless style wrapper does not hide adjacent characters for normalization. Do not use global trim/split_whitespace, document language or target-specific space heuristics.

The coordinator calls this algorithm only when its existing `normalize_spaces` flag is true. Preserve indices by replacing consumed slots with Empty. The Unicode control and styled-space tests are owned here. No phase, public signature or default changes.

## Acceptance

The existing Unicode/space tests and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.


## Realização com avaliação única

Disponibilizar internamente normalização para finalizar inserções após transformação de elemento/texto, sem mudar a política Unicode.


## Realização com avaliação única

Origem de espaço projeta carriers internos. Marcadores de proveniência sem visual não interrompem a busca de vizinhos.


## Realização com avaliação única

previous_visible ignora carriers sem informação visual, conservando a origem no grafo sem tratá-la como vizinho textual.


## Realização com avaliação única

next_visible ignora carriers sem informação visual, conservando semântica e fronteiras de parágrafo.


## Realização com avaliação única

Texto supportive projeta carriers internos após realização; a política de descarte de espaços continua a vigente.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

A delegação transparente já owned de ShowRecipe.body também aceita Source.body, sem executar origem separada, publicar carrier ou atribuir Location. Preservar classificação/preparo/serialização conforme a operação desta unidade.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Normalização emprestada com mudança explícita — 2026-10-10

Medição anterior à decisão: snapshot-normalization-before.json.gz em lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09 captura UTC/fontes; space.rs:66–114 reconstitui o primeiro espaço de todo grupo. space_with_origin:24–35 conserva Styled e a origem canônica Source, mas elimina Located/Realized e normaliza Source redundante/detached. Inferência: uma decisão compartilhada por grupo permite reusar sequências sem alteração sem duplicar a política Unicode.

Disponibilizar helper crate-private normalize_markup_spaces_if_changed sobre slice emprestado, retornando None sem alteração e Vec somente a partir da primeira mudança. Compartilhar scanner e aplicação com normalize_markup_spaces in-place; cada grupo consulta o estado já normalizado dos grupos anteriores. Grupo muda quando é descartado, colapsa mais de um espaço efetivo, agrega nova origem newline ou exige normalizar carrier. Empty intermediário não é mudança por si. Space/SpaceFromNewline e Styled recursivamente podem conservar sua forma; Source somente com span não-detached e corpo sem origem direta canônica. Located/Realized mantêm a transformação anterior. Nunca usar igualdade/hash/endereço de Content como decisão. Preservar todos os critérios Unicode, Linebreak destrutivo, vizinhos, slots Empty, estilos e proveniência; não alocar Vec no consumidor in-place.

Aceitação: RED→GREEN de reuso, grupos mistos, Unicode, carriers e vizinhos evolutivos; suite e qualificação do owner realize. Nenhum contrato público, flag ou fase muda.

### Refinamento após refutação de tempo

Medição antes da decisão: lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/measurements-normalization{-refuter,}-first.json.gz registra fontes/binários/UTC/HEAD/dirty/argv. O primeiro candidato foi consistentemente mais lento em no-fill-1600 e table-text-1600; normalize_markup_spaces passou também a certificar a forma canônica antes de aplicar cada grupo. Inferência refutável: restringir essa certificação ao consumidor emprestado reduz custo fora de normalize_insertions.

O scanner compartilhado determina todo grupo e sua política; o consumidor in-place aplica cada grupo, como antes. Somente o consumidor emprestado certifica a forma canônica e decide se materializa output. A política Unicode e o estado atualizado entre grupos continuam compartilhados. Repetir contagem e tempo nos controles que refutaram a primeira variante antes de aceitar.


## Exaustividade nominal e espaços transparentes — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Fontes/L0/configuração, hashes, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-v16-whitespace-2026-10-17/before.json.gz`. AST/spans em contexts.json; 11 V16 emitidos e um forwarding de identidade latente em classified_content. measurement-before.log mede cinco diferenças entre sete pares Source. measurement-family-before.log e source-family-before-analysis.json medem 28 pares, 20 diferentes: os cinco casos de quebra à esquerda/direita e grupo atravessando Empty falham igualmente sob Source, Located, Realized.visual e Styled. Parbreak e Text continuam fronteiras/apoios iguais. É medição do algoritmo real dentro do crate em workspace descartável, não uma cópia adaptada de helpers privados nem prova global de paridade. A primeira sonda externa falhou por E0624, foi preservada e não conta como RED funcional.

### Prescrição exclusiva

No scanner compartilhado de normalize_markup_spaces e normalize_markup_spaces_if_changed, classificar Empty intermediário e Linebreak destrutivo por uma projeção privada emprestada `space_boundary_content`: Source.body, Located.body, Realized.visual e Styled.body, recursivamente. Os outros Content param com complemento nominal. Não observar Realized.original, executar ShowRecipe ou mudar a política do agrupamento textual. Essa projeção só decide fronteiras; os nós originais, suas origens/estilos e posições continuam no grafo. Não consumir o marcador de quebra, não colapsar Parbreak/Texto/conteúdo matemático, não alargar para is_empty genérico. Conservar a política Unicode, newline agregado por OR, canonical_space e a transformação histórica de Located/Realized em ocorrências de espaço. ShowRecipe continua com seu papel específico atual em cada helper e não ganha travessia no scanner por esta alteração.

O novo helper privado tem quatro braços de projeção e complemento nominal dos outros 116 Content, retornando a própria referência. Sua nota V19 corresponde a parar a projeção, não a descartar conteúdo.

- **W02** — `01_core/src/compiler/eval/rules/textual/space.rs:19`, `space_origin`: Só Space/SpaceFromNewline e a projeção já existente de Source/Located/Realized.visual/Styled têm origem de espaço. Conservar None para os demais. Excluir do complemento somente `Realized, Source, Located, Space, SpaceFromNewline, Styled`; guardados preservados `(nenhum)`.

- **W03** — `01_core/src/compiler/eval/rules/textual/space.rs:32`, `space_with_origin`: Source/Styled preservam sua reconstrução; o complemento usa newline agregado para SpaceFromNewline ou Space. Manter o guard had_newline e o mesmo corpo de ambos os braços. Excluir do complemento somente `Source, Styled`; guardados preservados `(nenhum)`.

- **W04** — `01_core/src/compiler/eval/rules/textual/space.rs:33`, `space_with_origin`: Source/Styled preservam sua reconstrução; o complemento usa newline agregado para SpaceFromNewline ou Space. Manter o guard had_newline e o mesmo corpo de ambos os braços. Excluir do complemento somente `Source, Styled`; guardados preservados `(nenhum)`.

- **W05** — `01_core/src/compiler/eval/rules/textual/space.rs:61`, `supportive_text`: Somente os textos/decorações e carriers já discriminados informam a vizinhança Unicode. Não converter conteúdo não textual em texto auxiliar. Excluir do complemento somente `Realized, Located, Source, ShowRecipe, Text, Styled, Strong, Emph, Underline, Strike, Overline, SmallCaps`; guardados preservados `(nenhum)`.

- **W06** — `01_core/src/compiler/eval/rules/textual/space.rs:76`, `canonical_space`: Somente representações canônicas já discriminadas dispensam reconstrução. Located/Realized continuam exigindo a transformação histórica. Excluir do complemento somente `Space, SpaceFromNewline, Styled, Source`; guardados preservados `(nenhum)`.

### Aceitação

L0 antes de código; correção interna de invariantes existentes (ADR-0127 contínuo), sem campo/assinatura/trait/API, default, fase, dependência, flag ou compatibilidade nova. Complementos excluem somente braços Content incondicionais; conservar variantes guardadas no fallback e manter o guard had_newline. Grupos V19 têm a mesma decisão por caso, permanecem visíveis e não comprovam executar cada alternativa. Não introduzir allow/expect, @ ou um if-let que esconda a decisão. HTML mantém os braços Boundary absorventes existentes e fecha nominalmente as três células do complemento; não muda preformatted/raw/title/ruby, tabelas de tags ou SmartQuote.

Acrescentar regressões nos três módulos proprietários: ambos consumidores de normalização, Source único/aninhado e os quatro carriers em quebras/Empty, controles Parbreak/Text/MathIdent, preservação de span/estilo e Realized.visual separado de original; projeções matching positivas/negativas e matrizes 3×3 inline/top-level com Boundary/EmptyInline/Supportive. Não relaxar testes existentes. RED real antes da implementação, GREEN após. Sondas de variante futura Content/SpaceSupport com cfg(test) compilado: no core usar enum real; no infra desbloquear somente E0004 de dependências/unidades fora dos alvos, em cópia descartável, com braços panic sintéticos registrados e nunca executados. Nunca reparar o local selecionado na própria sonda. Isso distingue dependência que impede compilar o alvo de exaustividade efetiva do alvo. Registrar os 12 matches finais (13 braços), incluindo o helper novo; dois braços de space_with_origin compartilham um match e não contam como dois erros do compilador.

Gates: build e suíte workspace completa, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito, reversa, replay e diferencial SARIF completo. Esperado V16 152→141; V19 somente dos grupos prescritos; V20 sem crescimento. Recursos: MemoryMax=12 GiB, swap zero, um job Cargo e comandos pesados sequenciais. Sem commit/push/merge/release ou alteração de pin CI. O objetivo geral e math-stretch-composite permanecem abertos.
