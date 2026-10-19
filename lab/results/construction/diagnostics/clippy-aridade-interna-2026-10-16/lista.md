# Seis aridades internas tratadas

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, snapshot 2026-10-16T15:46:35.161001+00:00; fontes/L0/hashes e diffstat em [before.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16/before.json.gz). Linhas correspondem ao estado anterior.

| ID | Função | Fonte e linha anterior | Intervenção |
|---|---|---|---|
| C045 | `walk` | `03_infra/src/export/gradients/mod.rs:320` | Collector local de descoberta possui ptr_to_idx, refs, grad_objs, next_id e counter. A recursão recebe o collector mais items/referencial/altura/placement; não trocar a travessia nem dedup. |
| C047 | `render_text_shaped` | `03_infra/src/export/render.rs:349` | Remover somente _text do helper privado raster e de seus chamadores. Após a redução de nove para oito argumentos, autorizar expect local com motivo dos oito inputs reais de pintura. |
| C062 | `cidfont` | `03_infra/src/export/stream.rs:222` | Compor PageContext com FontScenario já construído e os recursos de página/modo, em construtor comum interno. Migrar cidfont e seus chamadores; manter variantes e dados por empréstimo. |
| C064 | `multifont` | `03_infra/src/export/stream.rs:253` | Usar o mesmo construtor interno que recebe FontScenario, mantendo as tabelas multifont existentes e LookupCache default. Não redesenhar o enum nem expor tipo público novo. |
| C066 | `render_text_shaped` | `03_infra/src/export/svg/glyphs.rs:286` | Remover somente _text do leaf SVG de contornos e seu argumento no dispatcher. Após passar de nove para oito inputs, autorizar expect local para os oito inputs efetivos. |
| C102 | `compile_to_pdf_bytes_impl` | `03_infra/src/pipeline/pdf.rs:380` | Pedido privado PdfCompileRequest nomeia full_error, document_id, stream_mode, pdf_tagged_override, features, pretty_override, pages_override, standard_override e oracle; World, Source e Timings permanecem argumentos separados. Todas as fachadas públicas ficam intactas. |
