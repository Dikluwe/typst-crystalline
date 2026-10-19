# L0 — builder/metadata
Hash do Código: c2338d85

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/metadata.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/metadata.rs:22` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Metadata and identification

Own Info, XMP, timestamps and document/instance ID generation, archival output
intents, tagged structure trees and PDF/UA viewer preferences. Do not serialize
xref/trailer or attachments; publish the existing stored references for those owners.

Emit Info except in A4/A4f/A4e. Encode Title/Author/Keywords as UTF-16BE hex with BOM
and retain the Typst Crystalline Creator branding. Select the present
CRYSTALLINE_PDF_FIXED_EPOCH value, otherwise SOURCE_DATE_EPOCH, before parsing.
A missing or invalid selected value falls back to current UTC; an invalid primary
value does not retry the secondary variable. Info PDF dates and XMP ISO-8601 dates use that policy.
Always emit a Type Metadata/Subtype XML stream and catalog reference. Escape XML
metacharacters, preserve user metadata presence, application/pdf format, proof
rendition class and selected page count. Generate separate standard-base64 instance
and document identifiers from 16 bytes; tests/fixed-epoch use the existing distinct
constants. An external document ID overrides only the document identifier. Persist
the pair for serialization; do not claim production bytes are globally deterministic.

Use archival identification exactly for the selected mode: A1/A2/A3 have part and
their A/B/U conformance as applicable; A4 has part 4 and rev 2020 without conformance;
A4f/A4e additionally use F/E. Emit the archival namespace only when applicable.
All archival modes receive the output intent and version-selected profile supplied
by the images ICC owner; preserve the GTS_PDFA1 subtype, profile reference and
existing descriptive fields. Do not infer full archival compliance from these fields.

For UA1, emit pdfuaid part 1 before the NPages field and DisplayDocTitle. With archival plus UA1 include
the minimal PDFUAIdentification extension schema, pdfuaid namespace/prefix and
internal Integer part property. Preserve document language in catalog and XMP;
use en only when the document language carrier is absent. Local styled language
does not retroactively replace the document language. Do not add viewer Direction.

## Tagged structure

Build structure only when PdfTags is Enabled. Bind to the page-local MCID table
owned by the stream façade rather than assigning a second numbering algorithm.
Emit per-page StructParents, ordered ParentTree associations, StructTreeRoot and
MarkInfo with Marked true/Suspects false. Disabled tags omit this structure.
Artifacts and descendants never become StructElem/MCID content. Formula alt is
nonvisual; preserve absent versus present-empty alt. Labels use Lbl.

Tables preserve Table/TR/TD, uniform automatic headers THead/TBody/TR/TH/TD,
and direct TR for explicitly mixed rows. Explicit Data in a header remains TD.
Summary belongs only to Table and preserves absent versus empty. TH Scope supports
Row/Column/Both; IDs and header associations follow the logical header carriers,
not a numeric PDF level attribute. Emit span attributes only above one. Repeated
headers across pages reuse the logical TH and do not duplicate IDs or relationships.

UA1 represents a physical heading as one H1-H6 unit and a paragraph as one P unit,
including multi-run content. Preserve Span children, Strong/Em role mapping,
Link/Figure alternative descriptions and language. Do not infer headings from
boldness or font size. Paragraph/line boundaries do not create empty nodes.
Link annotation OBJR and ParentTree relationships bind the references prepared by
navigation; no orphan keys. Keep meaningful content real, not disguised as Artifact.
Structural positives do not establish assistive-technology, reflow or full UA claims.

## Verification

Exercise metadata presence, fixed/external IDs, selected page counts, all archival
variants, UA-only/archival+UA, language, formulas, tables, repeated headers,
paragraph boundaries, links and tag-disabled omission.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/builder/metadata.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/core/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

TextSeparator pertence à mesma folha textual e idioma/estilo de Span no percurso UA1; não criar um algoritmo distinto de MCIDs ou novo heading/paragraph apenas pelo carrier.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/core/entities/layout_types.md`.

### Responsabilidade exclusiva

Walker estrutural desce no envelope sem criar estrutura/MCID próprio; estilos/idioma e MCIDs pertencem às folhas visuais.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T310: 03_infra/src/export/builder/metadata.rs:808; T312: 03_infra/src/export/builder/metadata.rs:835. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Unir braços Link em collect_top: no corpo decidir ua1 && !inside_artifact; ramo elegível conserva árvore Link/children e remoção de vazios, fallback conserva recursão direta no mesmo out. Retirar Link do braço Group/TextReplacement. Catchall decide elegibilidade no corpo; manter semantic_node, contadores MCID, Artifact, limites de PDF/UA e ordem.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F053: 03_infra/src/export/builder/metadata.rs:744 (ua1_leaf_node); F054: 03_infra/src/export/builder/metadata.rs:854 (collect_top). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F053 — `ua1_leaf_node`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `Semantic`, `Line`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F054 — `collect_top`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
