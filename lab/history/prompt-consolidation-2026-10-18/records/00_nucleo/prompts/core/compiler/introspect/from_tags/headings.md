# L0 — compiler/introspect/from_tags/headings
Hash do Código: 1850810e

Camada L1. Consumer produtivo único:
`01_core/src/compiler/introspect/from_tags/headings.rs`.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24


## A1 — callbacks de suplemento de Heading, 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c e working tree/UTC/hash
registrados em lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
before-pdf-probe.json.gz: heading-supplement-panic termina em sucesso no Tekt
e em A1_HEADING no vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-supplement-valid aceita ambos, mas só o vanilla usa Custom na referência.
Fonte upstream model/heading.rs:249-289 sintetiza supplement por ocorrência e
model/reference.rs:393-415 aplica a função e display. Set.rs:286-320 ignora
supplement e limpa numbering; snapshot_fields heading.rs:55-68 fixa tradução.
Isto é callback/valor observável, não exigência de mecânica upstream.

### Contrato vigente

A1-H2 abaixo substitui o transporte provisório em Styled. O snapshot canônico
sempre guarda Heading; styles causais pendentes ficam no cabeçalho privado.
A finalização aceita Content/Str/None/Auto e callback de argumento único Heading
com fields/Location, captura lexical e styles causais. Converter o resultado
em conteúdo de exibição; atualizar supplement e texto legado de resolved_labels
com numbering/pattern existente. Propagar erros, restaurar styles/contexto.
Sem callback em walk puro ou encoder, sem igualdade de árvore/corpo/label como
chave, sem nova lógica de contadores. O owner lang continua dono da tradução
do default.

## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

Finalizar Heading pelo índice causal existente de Heading, somente se o snapshot
possui styles pendentes. Guardar temporariamente apenas Locations pendentes;
não alocar para todos os defaults. O consumo da pendência impede repetir a
callback se o índice repetir Location ou a fachada for chamada novamente.
Base é entry.content() Heading, não wrapper nem corpo/hash/label como identidade.
Usar styles capturados para callback, sem inventar here/context, restaurando
Engine.styles no erro. Publicar nova entrada com supplement Content e sem styles
pendentes; clones anteriores não mudam. Preservar resolved_labels, numbering e
demais campos/label. Manter responsabilidade exclusiva do callback neste filho;
entrypoints só delegam, sem duplicar lógica. Chamadas runtime usam a mesma função.

### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.

O contexto do caller da fachada pública não autoriza context/here na callback:
salvar in_context/current_location, limpar durante apply_func e restaurar no
sucesso e no erro, junto com styles. H7 testa caller já contextual sem vazamento;
a Location do argumento introspectado continua preservada separadamente.
