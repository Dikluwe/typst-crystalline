# L0 — Font slots, metadata and inventory
Hash do Código: 0f713c1c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fonts.rs`

## Current evidence

- `03_infra/src/fonts.rs:23`, `:101`, `:113`, `:138`, `:153` — `FontSlot` and lazy loading.
- `03_infra/src/fonts.rs:202` — collection-face reconstruction.
- `03_infra/src/fonts.rs:271`, `:339`, `:404`, `:893` — discovery, metadata, coverage and paired insertion.
- `03_infra/src/fonts.rs:52` — `inventory_fonts`.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own `FontSlot`, explicit-path discovery, metadata extraction, collection-face reconstruction, the stable coverage facade/delegation and the inventory DTO. Font selection/scoring belongs to FontBook and its infrastructure adapters, not to this loader. External parser types do not cross into L1 contracts.

`FontSlot` retains public `path` and `index` and private embedded bytes, lazy mmap and final-font `OnceLock<Option<Font>>`. `new_embedded` uses index zero and treats the path as display information only. Source bytes prefer embedded bytes to a shared lazy file mapping. Loading/validation is idempotent per slot, including failure. Invalid or unreadable bytes and invalid face indices return `None`; validate the requested index against the original source before reconstructing a collection or parsing the normalized index-zero result, including embedded slots whose public index has been changed; mapping failures are not silently converted to a new eager-read policy. Simple fonts retain opaque mmap storage; reconstructed collection faces use independently owned bytes.

For a supported multi-face collection, rebuild the selected sfnt header/table directory, copy tables from collection-absolute offsets, rewrite offsets with alignment and recompute `head.checkSumAdjustment`. Preserve the existing no-extraction branch for noncollections and collections with at most one face; this document does not broaden collection support.

`discover_fonts` processes supplied files/directories recursively, recognizing lowercase `.ttf`, `.otf`, `.ttc` and `.otc`. Discovery creates one slot per discovered face, using one as the face-count fallback. Discovery does not require final-font validity; metadata pairing discards invalid slots. Preserve directory iteration behavior rather than promise an additional global sort or deduplication.

`font_info_from_bytes(data, index)` validates the requested face and returns neutral metadata or `None`. PostScript nameID 6 exceptions override each available family/style/weight/stretch field. Otherwise family comes from nameID 1 with repeated case-insensitive style-suffix trimming; nameID 16 is deliberately ignored. Decode Macintosh Roman names where applicable. Preserve the residual first-decodable-name fallback for a missing ID1, without claiming upstream equivalence for it. Infer Italic before Oblique using face style bits and full-name spelling; retain weight/stretch extraction, monospace flags and OS/2 PANOSE serif detection `[2, 2..=10, ..]`. Preserve the existing exception entries, including documented New Computer Modern names and weight/style corrections.

Metadata extraction leaves `coverage` empty. `extract_coverage` retains its crate-private signature and delegates exact extraction to the private child `03_infra/src/fonts/coverage.rs`, exclusively specified by `00_nucleo/prompts/infra/fonts/coverage.md`; `pair_slots_with_book` inserts metadata and slots together and preserves input order. `00_nucleo/prompts/infra/fontdb.md` owns eager system coverage; `00_nucleo/prompts/infra/world/fonts.md` owns lazy coverage for entries that lack it, cache lifetime and source preloading. Do not state that every loader eagerly extracts coverage or that all system coverage must remain lazy.

`inventory_fonts(font_paths, include_system)` composes existing embedded/system/project loaders into `FontInventoryEntry { family, path, index, style, weight, stretch, embedded }`. Discard invalid pairs, sort by lowercase family, style, weight, stretch, path and index, then remove identical entries. Do not format stdout in L3; `00_nucleo/prompts/wiring/fonts.md` delegates presentation to L2.

## Verification boundary

Existing inline tests cover invalid slots, pairing, synthetic collection reconstruction, Macintosh names, suffixes/style inference, exceptions, exact extraction and mmap loading. Repository fixtures include `03_infra/fixtures/fonts/p839-*.ttf`, `p840-fandolhei-bold.ttf`, `NotoSans-Regular.ttf` and the licensed OTF fixtures. Host CJK/font tests must report environmental absence without treating it as positive coverage evidence.


## Coverage ranges — gate package2026-10-07

Measurement before decision: lab/results/construction/diagnostics/cmap-intervalos-2026-10-05/measurement.json.gz records sources/HEAD/dirty tree/UTC/argv/fonts and53.19→10.85ms isolated extraction/canonicalization; no CLI speedup measured. lab/results/construction/diagnostics/coverage-ranges-preparacao-2026-10-07/audit.json verifies this consumer remains identical to the measured HEAD source, and the pure font_book consumer remains byte-identical to the experiment snapshot. ttf-parser0.25.1 format4.rs:103–115,format12.rs:68–75,format13.rs:41–48 retain declared bounds/glyph-zero; its cmap/mod.rs iterator stops at first invalid record.

Decision after package ratification: declare private child coverage and delegate only existing extract_coverage. Font slots, collection reconstruction, metadata, pairing, discovery timing/eager-vs-lazy policy, cache lifetime and API remain unchanged. Parent must not duplicate range parsing, normalization or callback fallback; child owns these algorithms. Invalid face and missing cmap behavior remain. The public Coverage constructor is owned exclusively by 00_nucleo/prompts/core/entities/font_book.md. No code before human ratification of the public constructor in this package.

Acceptance: existing inline tests remain, child tests exercise syntax/native/fallback and independent membership; full build/lint lineage and exact per-face/fallback/PDF controls, then same-state paired CLI/cost measurement. No performance promise from isolated historical data.


Ratificação: dono confirmou o pacote coverage-ranges-2026-10-07 com `confirmo` em2026-10-07; manifesto f8f7805c6d2ca08d1d14296ed5c93767c416b536fa9c17532eaa8310ca93a549.

## Compartilhamento de bytes embutidos — medição 2026-10-08

Estado congelado: `lab/results/construction/diagnostics/embedded-fonts-memoria2-2026-10-08/before.json.gz`. `03_infra/src/fonts.rs:32,119,158-163` retém um Vec no slot e clona seus bytes no primeiro get, mantendo os dois buffers vivos. `01_core/src/entities/world_types.rs:40-49,57-61` já oferece Font::Vec com Arc<Vec<u8>>, clone de armazenamento e slice; nenhum novo contrato L1 é necessário.

Classificação: duplicação mecânica interna, não obrigação da linguagem. Inferência a validar: retirar cópia diminui memória quando fonte embutida simples é carregada; se RSS pareado não diminuir de maneira repetida, não atribuir ganho do processo a esta mudança.

Privadamente, reter os bytes embutidos em Arc<Vec<u8>>, criado a partir do Vec transferido pelo construtor. Isso não valida nem carrega a face antecipadamente. source_bytes empresta o slice original. get valida primeiro o índice solicitado nos bytes originais; se reconstruir coleção suportada, valida a face extraída no índice zero e cria Font próprio. Sem reconstrução, conserva a validação legada de índice zero e constrói a variante Font::Vec já existente clonando somente o Arc compartilhado. OnceLock continua a guardar sucesso/falha apenas na primeira chamada. Assinaturas e campos públicos permanecem intactos; index continua aplicável à validação dos embutidos. Não criar storage estático, cache global, construtor ou variante pública.

Aceitação: RED→GREEN de não duplicação do buffer para fonte simples, bytes/metadados/cobertura idênticos e validade após descarte do slot; coleção mantém bytes fonte originais e resultado independente válido, índices inválidos/falhas/mmap continuam cobertos. Teste de identidade física é testemunha de custo interno, não critério de paridade da linguagem. Exportações e CLI pareados, builds/lint com resselo somente dos owners alterados.

Refinamento medido antes da decisão: a primeira representação privada
Option<Font> (`attempt-font-container/candidate.json.gz`, SHA de binário
a4d93e4410e329ad7dd34fcef61f9114c863b3a4a96aba98f178b63bcc6e7953)
mostrou −1,54% RSS na tabela local nas duas baterias, mas a repetição
indicou +0,91/+1,00/+2,01% de tempo em context/simples/denso
(`attempt-font-container/repeat.json.gz`). Usar Option<Arc<Vec<u8>>>
privado remove o despacho do container Font na leitura de bytes originais.
Isso é hipótese de custo, não causalidade comprovada; nova bateria pareada
exige validar tempo/RSS antes de conservar a implementação. Nenhuma nova
variante ou assinatura Font é introduzida.

## Aquisição estática seletiva — 2026-10-08

### Medição anterior à decisão

`lab/results/construction/diagnostics/custo-fixo-fontes-item2-2026-10-08/probe-results.json` mede, em três processos por modo, 17 assets embutidos com 9.682.300 bytes. O catálogo embutido retém ~9,24 MiB de alocações Rust requisitadas e sobe ~9,3 MiB em mapping anônimo e ~9,3 MiB em páginas residentes do executável. `embedded_fonts.rs:92-94` copia cada slice estático para Vec; `fonts.rs:115-123` retém todos os Vecs mesmo sem `get`. `typst-kit/src/fonts.rs:136-141` no vanilla pinado fornece `Bytes::new(data)`, e `typst-library/src/foundations/bytes.rs:48-62` documenta que o slice estático é o backing direto, sem Vec. Isto mede mecânica de armazenamento, não intenção nem paridade de linguagem. O ganho de RSS é hipótese refutável por bateria pareada do CLI; o tamanho dos buffers é prova direta da duplicação.

### Decisão

Supersede apenas a proibição anterior de storage estático para fontes embutidas do catálogo; o `new_embedded(path, Vec<u8>)` público continua com storage Arc<Vec<u8>> e compartilhamento source/get. Adicionar internamente uma origem `&'static [u8]` que `source_bytes` empresta sem copiar. A construção do catálogo usa essa origem somente depois de validar metadados; `get` valida o índice solicitado, reconstrói coleção quando aplicável e materializa um Font::Vec próprio somente para face estática solicitada, no OnceLock. Sucesso/falha, estabilidade após descarte do slot, caminhos disco/mmap e contrato Font L1 ficam iguais. Não há nova flag, default, fase ou assinatura pública; correção interna ADR-0127.

Aceitação RED→GREEN: todos os slots de `load_embedded_fonts` referenciam exatamente os slices de typst_assets antes do primeiro `get`; conteúdo/metadados/cobertura, seleção, PDF e inventário conservam observáveis. Medir novamente RSS e tempo do CLI completo; não atribuir ao produto a diferença de uma sonda isolada. O ownership 1:1 e as linhagens devem ser ressellados.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/fonts.rs` nas linhas 367, 368, 369, 370 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Corrigir a separação/continuação de parágrafos e listas dos comentários Rustdoc. Preservar o conteúdo técnico e todas as asserções/casos de teste.

Separar P840 da lista anterior; encerrar a frase incompleta sobre is_italic sem inventar a justificativa que falta.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.
