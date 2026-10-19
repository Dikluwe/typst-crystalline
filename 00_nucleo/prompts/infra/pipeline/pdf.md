# L0 — infra/pipeline/pdf
Hash do Código: 3ff43fc0

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/pdf.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
```

## Current evidence

- `03_infra/src/pipeline/pdf.rs:180` — `resolve_pdf_pages_and_tags`.
- `03_infra/src/pipeline/pdf.rs:243` — `resolve_pdf_pages_tags_and_standards`.
- `03_infra/src/pipeline/pdf.rs:288` — `require_ua1_document_title`.
- `03_infra/src/pipeline/pdf.rs:308` — `require_ua1_image_alt`.
- `03_infra/src/pipeline/pdf.rs:351` — `compile_to_pdf_bytes_impl`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/pdf.rs`.

## Owned contract

- Own PDF compilation facades, resolved document options, PDF-only validations and font-aware export dispatch. Obtain a complete paged document from its owner; preserve raw warnings on both success and failure.
- Preserve every compatibility wrapper's explicit/default choices, DocumentID transport, oracle path, timings and public import paths. Some DocumentID fixes the document id, not the instance id; None leaves existing random behavior. Empty executable-only content preserves the existing empty-output path.
- Resolve pages from caller then document then all pages; resolve standards from caller then document then PdfStandards default. Resolve pretty from explicit caller then document. StreamMode (verbose production default versus explicit compact), serialization, tagging, features and DocumentID are independent axes.
- Without page selection, explicit tagged decisions win; documentary Auto enables tagging. With pages, effective Explicit(true) fails with `cannot enable tagged PDF and export a page range`; Auto disables tagging and emits `exporting a page range disables PDF tagging` with hints `the resulting PDF will be inaccessible` and ``set `pdf(tagged: false)` to silence this warning``; explicit false stays silent.
- CLI-only rejections and the separate CLI pages warning belong to `00_nucleo/prompts/wiring/compile.md`, not this module. Do not collapse caller absence early or duplicate those messages.
- After pages/tagging resolution, an accessible archival profile (A1a/A2a/A3a) or Ua1 with Disabled tags fails with ``cannot set `pdf(tagged: false)` when exporting a {name} document``. Add `using `pdf(pages: ...)` implies `pdf(tagged: false)` only when pages caused disabling and false was not explicitly selected.
- Pass resolved pages/tags/standards/serialization unchanged to all font-aware exporter branches. Standard-name/version resolution is not proof of full PDF/A or PDF/UA structural conformity; do not add a conformity engine or version-specific internal rewrites.
- Before export validate attachments in final order: duplicate derived virtual paths fail with `attempted to attach file {path} twice`. Direct infallible exporters require a validated document; they do not replace this language error channel.
- Only under Ua1, absent document title fails with `PDF/UA-1 error: missing document title` and hint ``set the title with `set document(title: [...])```. This checks None versus Some, not a new empty-title policy or combined-validator prefix.
- Only under Ua1, real images without alt outside `pdf.artifact` fail with `PDF/UA-1 error: missing alt text` and hint `make sure your images and equations have alt text`. Some empty text is still supplied; keep available origin spans and existing detached-span limitation.
- Resolve/instantiate ordered complete font keys through the resource owner, passing the World without requesting its catalog in advance. Dispatch no resolved fonts to fallback export, one to single-font export and many to multifont export without dropping explicit variations. Preserve ordinary and oracle behavior, not total PDF-byte identity as a language criterion.

The resource owner's private matching-axis predicate governs the early instancer dependency check. Unknown-axis-only requests must not produce a missing-tool diagnostic; retain dependency errors for actual font axes and preserve export dispatch/options.

Quando a dependência do instanciador faltar para um eixo real, delegar o gate
e diagnóstico ao helper privado de `infra/pipeline/resources.md`, preservando
mensagem, span e precedência. A medição causal está no L0 daquele owner; esta
correção interna não altera o despacho de zero/uma/várias fontes.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Validação PDF partilhada com bundle — 2026-09-30

### Medição anterior à decisão

`lab/results/construction/diagnostics/auditoria-composicao-construcao-2026-09-29/probe.json` mede o mesmo corpo com `format.pdf(standard: "ua-1")`: PDF isolado falha com `PDF/UA-1 error: missing document title`, mas PDF dentro de bundle compila e declara `pdfuaid:part=1` sem título. `03_infra/src/pipeline/pdf.rs:404-426` verifica título, alt e duplicação de anexos antes da exportação; `03_infra/src/pipeline/bundle.rs:463-513` não executa essas verificações. A inferência é que o target PDF perdeu invariantes ao ser composto no bundle; seria refutada se o mesmo body não passasse pelas duas rotas com o mesmo standard efetivo. A sonda documental confirmou o standard efetivo no XMP; não se presume conformidade PDF/UA completa nem paridade byte a byte.

### Contrato do owner

Concentrar num helper privado ao pipeline, pertencente a este owner PDF, as validações pré-exportação já existentes de documento paginado: primeiro título UA-1, depois alt UA-1, depois caminho de anexo duplicado na ordem original. Preservar exatamente mensagens, hints, spans, condições e precedência. PDF isolado e bundle chamam esse mesmo helper depois de resolver standards e antes de escolher fontes ou exportar. Não criar API pública, novo modo, fase ou regra de conformidade; a delegação de bundle não transfere ownership do contrato PDF.

### Aceitação

Sob UA-1, ausência de título falha em ambas as rotas com o mesmo diagnóstico; título fornecido continua a compilar. Alt ausente em imagem real e anexo duplicado usam os diagnósticos vigentes quando a rota os alcança. Fora de UA-1, o título continua opcional. A correção é de paridade interna do target PDF e de invariantes já especificadas, em fluxo contínuo pela ADR-0127 (sem assinatura pública ou mudança de fase), com teste RED→GREEN e revalidação.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## Transferência de buffers de fontes já completos — 2026-10-10

Medição anterior à decisão: `03_infra/src/pipeline/pdf.rs:436-437`
resolve uma tabela owned e entrega uma referência temporária a bind_document.
`03_infra/src/export/font_binding.rs:127-148` retorna Cow::Borrowed quando
nenhuma face precisa ser anexada. O into_owned seguinte clona a tabela inteira,
incluindo Vec<u8> por fonte. A regra de binding é semântica; essa cópia é mecânica.
O diagnóstico font-primary-reuse-2026-10-10 registra o pico de variantes e
sua proveniência; atribuir parte desse pico à cópia ainda é inferência a
refutar por RSS e alocações.

Decisão interna: reter a tabela owned em variável local durante o binding e,
quando o resultado for Borrowed, transferir a tabela original sem clonar.
Quando for Owned, usar a tabela completa produzida pelo binder e soltar a
original. Helper privado pode tornar essa escolha testável. Preservar a
ordem, todas as chaves/bytes, faces anexadas, fallback, validações e exportadores
normal/oráculo. Não alterar APIs, fase, instanciamento, fonte ou diagnósticos.
Aceitação RED→GREEN: teste de transferência do buffer original e teste de
face faltante; regressão de linguagem/geometria/raster ausente; medir
alocações/bytes solicitados e pico RSS separadamente. Não prometer redução
da memória residente antes de medi-la. Fluxo contínuo ADR-0127.


### Limite de transferência para tabelas plurais — 2026-10-10

Medição anterior à decisão: measurements.json.gz desta rodada (arquivado
como all-tables-measurements.json.gz em pdf-font-buffer-reuse-2026-10-10)
mede, no release 0ef5e759, RSS local 41.122→42.356 KiB (+2,84% pareado),
apesar de poupar apenas um buffer de 337.547 bytes solicitados e três
alocações. A tabela de 800 variantes poupa 240,6 MiB solicitados e reduz
RSS 525.072→292.954 KiB e tempo 15,16%. Estado/UTC completos estão no receipt.
A origem da amplificação no caso de uma fonte não foi atribuída; fragmentação
é hipótese. Não atribuir exclusivamente à captura de estilos já revertida.

Decisão interna: conservar o caminho anterior de binding+into_owned para
tabelas vazias ou unitárias; transferir a tabela original completa somente
quando supplied.len()>1. O limiar distingue o caminho single-font já existente
no dispatch; não depende de fixture, texto, família, peso ou tamanho. No
caso plural Owned, conservar a tabela completa do binder. Isto é política
privada de alocação, não comportamento de linguagem. Preservar bytes/ordem
em todos os casos. Confirmar por controle de RSS local, contagens de
alocações unitárias iguais à base e redução na tabela plural; repetir a
comparação de saída no release final. Não congelar endereços como requisito
da linguagem.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C101`: `03_infra/src/pipeline/pdf.rs:121`, `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_serialization`, 8 argumentos contados pelo Clippy (limiar 7).
- `C102`: `03_infra/src/pipeline/pdf.rs:376`, `compile_to_pdf_bytes_impl`, 12 argumentos contados pelo Clippy (limiar 7).
- `C103`: `03_infra/src/pipeline/pdf.rs:548`, `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override`, 9 argumentos contados pelo Clippy (limiar 7).
- `C104`: `03_infra/src/pipeline/pdf.rs:578`, `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override`, 10 argumentos contados pelo Clippy (limiar 7).
- `C105`: `03_infra/src/pipeline/pdf.rs:614`, `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override`, 8 argumentos contados pelo Clippy (limiar 7).
- `C106`: `03_infra/src/pipeline/pdf.rs:641`, `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override`, 9 argumentos contados pelo Clippy (limiar 7).
- `C107`: `03_infra/src/pipeline/pdf.rs:669`, `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override`, 10 argumentos contados pelo Clippy (limiar 7).
- `C108`: `03_infra/src/pipeline/pdf.rs:702`, `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override`, 8 argumentos contados pelo Clippy (limiar 7).
- `C109`: `03_infra/src/pipeline/pdf.rs:733`, `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_override`, 9 argumentos contados pelo Clippy (limiar 7).
- `C110`: `03_infra/src/pipeline/pdf.rs:760`, `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_and_standard_override`, 10 argumentos contados pelo Clippy (limiar 7).
- `C111`: `03_infra/src/pipeline/pdf.rs:793`, `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_override`, 8 argumentos contados pelo Clippy (limiar 7).
- `C112`: `03_infra/src/pipeline/pdf.rs:878`, `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_serialization`, 8 argumentos contados pelo Clippy (limiar 7).
- `C113`: `03_infra/src/pipeline/pdf.rs:945`, `compile_to_pdf_bytes_oracle_with_features_and_serialization`, 8 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C101` — `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_serialization`: Exceção pontual justificada

Wrapper público converte PdfTags/serialization explícitos em overrides Some, preservando features, ID, modo e retorno de tempos. Essa distinção de presença é normativa no L0.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C101: Wrapper público converte PdfTags/serialization explícitos em overrides Some, preservando features, ID, modo e retorno de tempos")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C102` — `compile_to_pdf_bytes_impl`: Correção interna prescrita

Núcleo privado recebe nove opções de execução/exportação mais World/Source e Timings. As mesmas sequências são montadas por wrappers ordinários e oracle; o pedido privado deve nomear os eixos preservando None até a precedência documental.

Pedido privado PdfCompileRequest nomeia full_error, document_id, stream_mode, pdf_tagged_override, features, pretty_override, pages_override, standard_override e oracle; World, Source e Timings permanecem argumentos separados. Todas as fachadas públicas ficam intactas.

Aceitação: Preservar None/Some, precedência caller/documento/default, canais de erro/warning, timings e rotas zero/uma/várias fontes e oracle; não ampliar o suporte do oracle.

#### `C103` — `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override`: Exceção pontual justificada

Wrapper público com tempos conserva pages e fornece standard None ao mais completo. A API e a diferença entre ausência e override explícito são de compatibilidade.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C103: Wrapper público com tempos conserva pages e fornece standard None ao mais completo")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C104` — `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override`: Exceção pontual justificada

Wrapper público completo com tempos fornece as opções ordinárias e oracle=false ao núcleo. Preservar assinatura/retorno e construir o futuro pedido interno com campos nomeados.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C104: Wrapper público completo com tempos fornece as opções ordinárias e oracle=false ao núcleo")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C105` — `compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override`: Exceção pontual justificada

Wrapper público com tempos e pretty sem pages delega pages None. A assinatura legada é explícita no transporte e deve continuar disponível.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C105: Wrapper público com tempos e pretty sem pages delega pages None")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C106` — `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override`: Exceção pontual justificada

Wrapper público sem tempos com pages delega standard None. Preservar warnings/resultado e não resolver o default do standard antes do documento.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C106: Wrapper público sem tempos com pages delega standard None")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C107` — `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override`: Exceção pontual justificada

Wrapper público ordinário completo sem tempos cria Timings local e passa oracle=false. A reorganização do núcleo privado não muda sua API.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C107: Wrapper público ordinário completo sem tempos cria Timings local e passa oracle=false")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C108` — `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_override`: Exceção pontual justificada

Wrapper público sem tempos e sem pages fornece ausência de seleção. O eixo tagged/pretty não equivale a pages; preservar o adapter.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C108: Wrapper público sem tempos e sem pages fornece ausência de seleção")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C109` — `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_override`: Exceção pontual justificada

Wrapper público oracle com pages fornece standard None; a rota diagnóstica é independente da comum. Preservar os limites atuais do oracle em vez de expandi-los para eliminar o aviso.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C109: Wrapper público oracle com pages fornece standard None; a rota diagnóstica é independente da comum")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C110` — `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_and_standard_override`: Exceção pontual justificada

Wrapper público oracle completo fornece oracle=true e Timings local. Preservar a API e a distinção entre validações de opções e os limites da emissão oracle.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C110: Wrapper público oracle completo fornece oracle=true e Timings local")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C111` — `compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_override`: Exceção pontual justificada

Wrapper público oracle sem pages fornece pages None. A compatibilidade dos defaults é verificável pelo adapter existente e prescrita no L0.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C111: Wrapper público oracle sem pages fornece pages None")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C112` — `compile_to_pdf_bytes_full_error_and_document_id_with_features_and_serialization`: Exceção pontual justificada

Wrapper público sem tempos transforma tags/serialization explícitos em Some para o núcleo com precedência documental. Manter a conversão e a assinatura publicada.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C112: Wrapper público sem tempos transforma tags/serialization explícitos em Some para o núcleo com precedência documental")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C113` — `compile_to_pdf_bytes_oracle_with_features_and_serialization`: Exceção pontual justificada

Wrapper público oracle com serialization conserva as conversões explícitas e a seleção da rota diagnóstica, sem antecipar resolução documental.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C113: Wrapper público oracle com serialization conserva as conversões explícitas e a seleção da rota diagnóstica, sem antecipar resolução documental")], somente nesta função.

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

O inventário validado em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C101, C103, C104, C105, C106, C107, C108, C109, C110, C111, C112, C113. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.


## Aridade interna — materialização de seis casos, 2026-10-16

### Medição anterior à decisão

As fontes, chamadores, cláusulas vigentes, spans e inventário de 28 avisos foram conferidos antes da decisão. `lab/results/construction/diagnostics/clippy-aridade-interna-2026-10-16/before.json.gz` guarda fontes/L0 completos, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A auditoria anterior classifica estes casos como correção; não são exceções diretas. O RED deve observar as aridades atuais antes de alterações; controles funcionais existentes serão corridos antes/depois. É refatoração de representação privada, sem nova paridade ou ganho de performance medido.

### Responsabilidade exclusiva e aceitação

C102: criar PdfCompileRequest estritamente privado, sem derive Default, contendo full_error, document_id, stream_mode, pdf_tagged_override, features, pretty_override, pages_override, standard_override e oracle nos mesmos tipos atuais. compile_to_pdf_bytes_impl recebe World, Source, request e &mut Timings; desestruturar request uma vez no início e conservar literalmente o restante do corpo. As três fachadas diretas montam os mesmos valores em campos nomeados, conservando oracle false/false/true, None/Some até resolução documental e ordem das expressões puras. Nenhuma fachada pública muda.

Fluxo contínuo ADR-0127: nenhuma API externamente pública, default do produto, fase, dependência ou semântica é alterada. Tipos/constructors novos ficam internos; contratos públicos existentes permanecem. Este adendo concretiza somente os seis casos nominais; substitui proibições genéricas de supressão apenas para as duas expectativas residuais condicionadas C047/C066 após remover _text. As outras cinco aridades a corrigir continuam visíveis. Não elevar limiar ou silenciar no módulo/impl/crate. L0 precede código e resselo restrito. Confirmar preservação das assinaturas externamente públicas, campos/valores transportados, controles PNG/SVG/PDF, precedência/diagnósticos/recursos/MCIDs e ausência de mutações globais. Usar os testes existentes sem relaxar fixtures/esperados; não escrever testes que espelhem os carriers privados. Revalidar suíte workspace completa, build, Clippy all-targets e unfulfilled_lint_expectations, Rustdoc, fmt e linhagem estrita/reversa, mantendo 12 GiB, swap zero e um job.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F065: 03_infra/src/pipeline/pdf.rs:330 (missing). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F065 — `missing`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Text`, `TextShaped`, `Line`, `Glyph`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: Semantic: kind: typst_core::entities::layout_types::SemanticKind::Artifact(_).

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
