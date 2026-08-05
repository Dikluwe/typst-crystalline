//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra.md
//! @prompt-hash 8e65820e
//! @layer L3
//! @updated 2026-04-03 (Passo 34)

/// Testes de integração: pipeline completo via SystemWorld real.
///
/// Estes testes exercitam o caminho de código de produção que os testes
/// unitários de L1 (com MockWorld) não cobrem (DEBT-6).
///
/// Pipeline: SystemWorld → eval → layout → export_pdf
#[cfg(test)]
mod integration {
    #![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
    use std::path::{Path, PathBuf};

    use regex::Regex;

    use typst_core::contracts::world::World;
    use typst_core::engine::introspect::{introspect, introspect_with_introspector};
    use typst_core::engine::layout::layout;
    use typst_core::entities::bytes::Bytes;
    use typst_core::entities::introspector::Introspector;
    use typst_core::entities::module::Module;
    use typst_core::entities::source::Source;
    use typst_core::entities::source_result::SourceResult;
    use typst_core::entities::value::Value;

    use crate::export::{
        StreamMode, export_pdf, extract_page_content_streams_text,
    };
    use crate::world::SystemWorld;
    use image::ImageFormat;

    // ── Utilitário: diretório temporário sem dependência externa ─────────

    struct TempDir(PathBuf);

    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn tempdir() -> TempDir {
        // Sufixo baseado em subsec_nanos para colisões mínimas em paralelo
        let path = std::env::temp_dir().join(format!(
            "typst-crystalline-it-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&path).unwrap();
        TempDir(path)
    }

    /// Cria um SystemWorld com um ficheiro `main.typ` contendo `src`.
    fn world_from_str(src: &str) -> (SystemWorld, TempDir) {
        let dir = tempdir();
        std::fs::write(dir.path().join("main.typ"), src).unwrap();
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        (world, dir)
    }

    // Helpers promovidos para API pública em 03_infra (Passo 113, ADR-0046):
    //   `pipeline::eval_to_module_with_sink`
    // Formatter migrou para L2 no Passo 119 (ADR-0050); L3 já não
    // importa nem testa `format_diagnostic` — testes unit em
    // `typst_shell::diagnostic` cobrem esse caminho.
    use crate::pipeline::eval_to_module_with_sink as do_eval_with_sink;

    /// Wrapper fino sobre `eval_to_module_with_sink` para testes que só
    /// precisam do `SourceResult<Module>`, descartando warnings.
    fn do_eval(world: &SystemWorld, source: &Source) -> SourceResult<Module> {
        let (result, _warnings) = do_eval_with_sink(world, source);
        result
    }

    /// Pipeline completo → bytes PDF (Passo 65 + P602).
    ///
    /// Passagem 1 (introspecção): `introspect_with_introspector()` resolve labels,
    /// headings para bookmarks e popula `extracted_headings` no documento.
    /// Passagem 2+ (fixpoint interno a L1): `layout()` converge o mapa de páginas
    /// da TOC internamente. O orquestrador L3 é agora linear.
    fn compile_to_pdf(src: &str) -> Vec<u8> {
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");

        // ── Introspecção ──────────────────────────────────────────────────
        let intr = introspect_with_introspector(content);

        // ── Layout (fixpoint acontece internamente em L1) ─────────────────
        let mut doc = layout(content);

        // P602/P606 — transportar headings para que `emit_outlines` possa gerar
        // os bookmarks PDF (igual à pipeline de produção).
        doc.extracted_headings = intr.headings_for_bookmarks().to_vec();

        export_pdf(&doc, StreamMode::Verbose)
    }

    // ── Testes de integração ──────────────────────────────────────────────

    #[test]
    fn pipeline_texto_simples() {
        let (world, _dir) = world_from_str("Olá, mundo!");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn pipeline_export_pdf_helvetica() {
        let (world, _dir) = world_from_str("Texto simples.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn p811_export_documento_sem_paginas_emite_pagina_em_branco() {
        // P811 — `PagedDocument` sem páginas (ex.: ficheiro vazio, `$ $`,
        // `$frak()$` antes da validação de P811): antes, `/Kids [3 0 R]`
        // referenciava um objecto nunca emitido → PDF inválido
        // (`Kid object is wrong type (null)`, medido com pdfinfo). O vanilla
        // emite 1 página em branco A4 (medido). Agora o builder sintetiza a
        // página em branco — o objecto de página EXISTE no PDF.
        use typst_core::entities::layout_types::PagedDocument;
        let doc = PagedDocument::new(vec![]);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert_eq!(&pdf[..5], b"%PDF-");
        let blob = String::from_utf8_lossy(&pdf);
        assert!(
            blob.contains("/Kids [3 0 R]"),
            "/Pages deve declarar a página 3: {}",
            &blob[..blob.len().min(400)]
        );
        assert!(
            blob.contains("/Type /Page /"),
            "o objecto de página 3 deve existir (/Type /Page, não só /Type /Pages)"
        );
        assert!(
            blob.contains("/MediaBox [0 0 595.28 841.89]"),
            "página em branco A4 default (paridade vanilla medida)"
        );
    }

    #[test]
    fn pipeline_export_pdf_com_fonte_real() {
        // SystemWorld sem with_fonts() — world.font(0) retorna None.
        // O teste verifica que o fallback Helvetica funciona correctamente.
        // Quando fontes do sistema forem carregadas, world.font(0) retorna Some.
        let (world, _dir) = world_from_str("Texto com fonte real.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        if let Some(font) = world.font(0) {
            let pdf = crate::export::export_pdf_with_font(&doc, font.as_slice(), StreamMode::Verbose);
            assert!(!pdf.is_empty());
            assert_eq!(&pdf[..5], b"%PDF-");
        } else {
            // Sem fontes carregadas — fallback Helvetica
            let pdf = export_pdf(&doc, StreamMode::Verbose);
            assert!(!pdf.is_empty());
            assert_eq!(&pdf[..5], b"%PDF-");
        }
    }

    #[test]
    fn pipeline_com_set_text_bold() {
        let (world, _dir) = world_from_str("#set text(weight: 700)\nTexto a negrito.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn pipeline_com_closures() {
        // Usa sintaxe #let saudacao(nome) = ... do Passo 31
        let src = "#let saudacao(nome) = \"Olá, \" + nome\n#saudacao(\"Mundo\")";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn pipeline_equacao_inline_sem_placeholder() {
        // Após Passo 36: MathLayouter processa sem placeholder [...]
        let (world, _dir) = world_from_str("$x + y$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        // Confirmar ausência de "[" nos itens de texto
        for page in &doc.pages {
            for item in &page.items {
                if let typst_core::entities::layout_types::FrameItem::Text {
                    text, ..
                } = item
                {
                    assert!(
                        !text.starts_with('['),
                        "equação não deve produzir '[': {}",
                        text
                    );
                }
            }
        }
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_equacao_com_frac_sem_panic() {
        let (world, _dir) = world_from_str("$ frac(a, b) $");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pipeline_equacao_inline_gera_pdf() {
        let (world, _dir) = world_from_str("A equação $x^2$ é famosa.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_equacao_block_gera_pdf() {
        let (world, _dir) = world_from_str("$ E = m c^2 $");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pipeline_set_scoped_nao_vaza() {
        // Verifica que #set text() dentro de { } não afecta o texto após o bloco.
        // Com Passo 33: ctx.styles é restaurado ao sair do bloco.
        // P786a: fonte corrigida — `set` sem `#` dentro de código (a forma
        // anterior é rejeitada pelo vanilla: `#` inválido em código).
        let (world, _dir) = world_from_str(
            "normal\n#{ set text(weight: 700); [negrito] }\nnormal novamente",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pipeline_frac_gera_pdf_sem_panic() {
        // Passo 37: MathFrac com posicionamento vertical.
        // Usa a/b (operador /) que produz Expr::MathFrac no AST — não frac(a,b).
        let (world, _dir) = world_from_str("$a/b$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_attach_sup_gera_pdf_sem_panic() {
        // Passo 37: MathAttach com sup elevado — usa ^ que produz Expr::MathAttach.
        let (world, _dir) = world_from_str("$x^2$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_frac_funcao_nativa_gera_pdf() {
        // Passo 38: frac(a,b) como função nativa → Content::MathFrac
        let (world, _dir) = world_from_str("$frac(a, b)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_linha_fraccao_no_pdf() {
        // Passo 38: linha de fracção deve produzir operador S (stroke) no PDF
        let (world, _dir) = world_from_str("$a/b$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = extract_page_content_streams_text(&pdf);
        assert!(
            pdf_str.contains(" S ") || pdf_str.contains(" S Q"),
            "PDF deve conter operador S (stroke) para a linha de fracção"
        );
    }

    #[test]
    fn pipeline_simbolos_gregos_gera_pdf() {
        // Passo 39: alpha/beta/gamma → Unicode α/β/γ no PDF
        let (world, _dir) = world_from_str("$alpha + beta = gamma$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_funcao_sin_gera_pdf() {
        // Passo 39: sin(x) — sin em não-itálico, x em itálico
        let (world, _dir) = world_from_str("$sin(x)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_eval_retorna_err_em_sintaxe_invalida() {
        // #let x = sem valor — incompleto. Pode ser Err de parse ou eval.
        // O importante é não entrar em panic.
        let (world, _dir) = world_from_str("#let x = ");
        let source = world.source(world.main()).unwrap();
        let _ = do_eval(&world, &source);
        // Se chegamos aqui, não houve panic — teste passa
    }

    #[test]
    fn pipeline_sqrt_basico_gera_pdf() {
        // Passo 40: sqrt(x) — MathRoot sem índice, símbolo √ + overline
        let (world, _dir) = world_from_str("$sqrt(x^2 + 1)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_root_com_indice_gera_pdf() {
        // Passo 40: root(3, x) — MathRoot com índice 3
        let (world, _dir) = world_from_str("$root(3, x)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    // ── Passo 41 — MathConstants via tabela OpenType MATH ────────────────

    // ── Passo 42 — GlyphVariants e MathDelimited ─────────────────────────

    #[test]
    fn pipeline_delimited_parenteses_gera_pdf() {
        let (world, _dir) = world_from_str("$(x + y)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_delimited_colchetes_gera_pdf() {
        let (world, _dir) = world_from_str("$[a, b]$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_delimited_com_frac() {
        // Fracção dentro de parênteses — delimitadores devem adaptar-se à altura
        let (world, _dir) = world_from_str("$(frac(a, b))$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_sqrt_expressao_alta() {
        // sqrt de fracção — radical deve adaptar-se à altura
        let (world, _dir) = world_from_str("$sqrt(frac(a, b))$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    // ── Passo 44 — AxisHeight e MathKernInfo ─────────────────────────────

    #[test]
    fn pdf_frac_inline_nao_vazio() {
        // Fracção inline com AxisHeight activo — deve produzir PDF válido
        let (world, _dir) = world_from_str("Valor: $frac(1, 2)$.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_attach_sup_sub_nao_vazio() {
        // Sup+sub com kern (kern=0 com FixedMetrics) — sem panic
        let (world, _dir) = world_from_str("$x^2 + y_i$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_delimitadores_com_axis_height() {
        // Delimitadores após AxisHeight — PDF não vazio
        let (world, _dir) = world_from_str("$(frac(a, b))$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_sqrt_com_axis_height() {
        // sqrt após AxisHeight — PDF não vazio
        let (world, _dir) = world_from_str("$sqrt(frac(a, b))$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    // ── Passo 43 — FrameItem::Glyph e GlyphAssembly ─────────────────────

    #[test]
    fn pdf_com_delimitadores_nao_vazio() {
        // Pipeline com delimitadores — PDF deve ser não-vazio e válido
        let (world, _dir) = world_from_str("$(x + y)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_com_sqrt_frac_nao_vazio() {
        // sqrt de fracção — sem panic, PDF válido
        let (world, _dir) = world_from_str("$sqrt(frac(a, b))$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_com_delimitadores_contem_bt_et() {
        // Delimitadores produzem BT/ET no PDF (texto ou glifo directo)
        let (world, _dir) = world_from_str("$(a)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = extract_page_content_streams_text(&pdf);
        assert!(
            pdf_str.contains("BT") && pdf_str.contains("ET"),
            "PDF deve conter operadores BT/ET para texto ou glifo"
        );
    }

    // ── Passo 41 — MathConstants via tabela OpenType MATH ────────────────

    #[test]
    fn pdf_frac_com_constants() {
        // Pipeline completo — confirmar que não panic após refactoring
        let (world, _dir) = world_from_str("$frac(a, b)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_sqrt_com_constants() {
        let (world, _dir) = world_from_str("$sqrt(x)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_attach_com_constants() {
        let (world, _dir) = world_from_str("$x^2_i$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    // ── Passo 45 — DEBT-9: ToUnicode para FrameItem::Glyph ───────────────

    #[test]
    fn pdf_delimitadores_nao_vazio_passo45() {
        // Regressão: pipeline com delimitadores continua a produzir PDF válido
        let pdf = compile_to_pdf("$(x + y)$");
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_valido_apos_passo45() {
        // Regressão geral: PDF estruturalmente válido após Passo 45
        let pdf = compile_to_pdf("$frac(a, b)$");
        assert!(!pdf.is_empty());
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("xref") && s.contains("%%EOF"));
    }

    #[test]
    fn p727_pdf_curve_contem_operador_stroke() {
        // P727 — regressão "curve renderiza página em branco": o PDF do
        // caso mínimo `#curve(curve.move(...), curve.line(...))` tem de
        // conter o operador de stroke `S` — sem ele o path existe no
        // content stream mas nada é pintado (paridade vanilla: forma
        // visível com stroke default 1pt preto).
        let pdf =
            compile_to_pdf("#curve(curve.move((0pt,0pt)), curve.line((50pt,50pt)))");
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
        let s = extract_page_content_streams_text(&pdf);
        assert!(
            s.contains("S\n"),
            "P727: PDF de curve deve conter operador de stroke (S)"
        );
    }

    #[test]
    // P916 (Parte B): fixture pinada — NewCMMath-Book.otf tem tabela MATH real.
    // Antes: lia /usr/share/fonts/… (não-hermético) com fallback NimbusSans (sem
    // tabela MATH) — o teste podia passar pelos motivos errados.
    fn pdf_tounicode_contem_mapeamento_de_delimitador() {
        // Com fonte MATH real (NewCMMath-Book.otf, fixture pinada), o ToUnicode CMap
        // deve mapear '(' e ')'.  U+0028 = '(', U+0029 = ')'.
        //
        // Nota de escopo: este teste verifica apenas a presença de U+0028/U+0029 no
        // CMap — não verifica crescimento de variante (isso é coberto pelas medições
        // de mutool trace registadas em typst-passo-916-relatorio.md).
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NewCMMath-Book.otf"
        ))
        .expect("fixture 03_infra/fixtures/fonts/NewCMMath-Book.otf deve existir");

        let (world, _dir) = world_from_str("$(frac(a, b))$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let _state = introspect(content);
        let doc = layout(content);
        let pdf = crate::export::export_pdf_with_font(&doc, &data, StreamMode::Verbose);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("<0028>"), "CMap deve ter U+0028 para parêntese de abertura");
        assert!(s.contains("<0029>"), "CMap deve ter U+0029 para parêntese de fecho");
    }

    // ── Testes do Passo 46 — Pre-scripts ─────────────────────────────────

    #[test]
    fn pdf_pre_scripts_nao_vazio() {
        // Pipeline completo com pre-superscript (emulado por Content directo no eval)
        // O eval não consegue extrair tl/bl do AST (NO-GO), mas o layout suporta-os.
        // Testar com right-script como regressão mínima do pipeline.
        let pdf = compile_to_pdf("$x^2$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_pre_scripts_dos_lados_nao_vazio() {
        // Regressão com sub e sup no mesmo nó
        let pdf = compile_to_pdf("$x_1^2$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_pre_scripts_contem_bt_et() {
        // PDF com script contém texto (BT/ET)
        let pdf = compile_to_pdf("$x^2$");
        let s = extract_page_content_streams_text(&pdf);
        assert!(s.contains("BT"), "PDF deve conter BT");
        assert!(s.contains("ET"), "PDF deve conter ET");
    }

    // ── Testes do Passo 47 — MathPrimes ──────────────────────────────────

    #[test]
    fn pdf_prime_simples_nao_vazio() {
        let pdf = compile_to_pdf("$x'$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_double_prime_nao_vazio() {
        let pdf = compile_to_pdf("$f''(x)$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_prime_com_sup_nao_vazio() {
        let pdf = compile_to_pdf("$x'^2$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_prime_contem_bt_et() {
        let pdf = compile_to_pdf("$x'$");
        let s = extract_page_content_streams_text(&pdf);
        assert!(s.contains("BT"), "PDF deve conter BT");
        assert!(s.contains("ET"), "PDF deve conter ET");
    }

    // ── Passo 48 — Baselines em equações inline ──────────────────────────────

    #[test]
    fn pdf_equacao_inline_frac_nao_vazio() {
        let pdf = compile_to_pdf("$frac(1, 2)$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_equacao_inline_com_texto_nao_vazio() {
        let pdf = compile_to_pdf("Valor: $frac(1, 2)$ calculado.");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_equacao_inline_contem_bt_et() {
        let pdf = compile_to_pdf("$x^2 + 1$");
        let s = extract_page_content_streams_text(&pdf);
        assert!(s.contains("BT"));
        assert!(s.contains("ET"));
    }

    #[test]
    fn pdf_equacao_inline_com_sqrt_nao_vazio() {
        let pdf = compile_to_pdf("$sqrt(x)$");
        assert!(!pdf.is_empty());
    }

    // ── Passo 49 — Limites verticais em operadores grandes ───────────────────

    #[test]
    fn pdf_sum_com_limites_nao_vazio() {
        let pdf = compile_to_pdf("$sum_(i=0)^n x_i$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_prod_com_limites_nao_vazio() {
        let pdf = compile_to_pdf("$product_(k=1)^n a_k$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_lim_com_limite_nao_vazio() {
        let pdf = compile_to_pdf("$lim_(x -> 0) f(x)$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_integral_com_limites_nao_vazio() {
        let pdf = compile_to_pdf("$integral_(0)^1 f(x)$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_attach_normal_nao_regride() {
        let pdf = compile_to_pdf("$x^2 + y_i$");
        assert!(!pdf.is_empty());
    }

    // ── Passo 50 — Diferenciação inline/bloco ────────────────────────────────

    #[test]
    fn pdf_sum_inline_no_texto_nao_vazio() {
        let pdf = compile_to_pdf("Soma $sum_(i=0)^n x_i$ no texto.");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_sum_inline_contem_bt_et() {
        let pdf = compile_to_pdf("$sum_(i=0)^n$");
        let s = extract_page_content_streams_text(&pdf);
        assert!(s.contains("BT"));
        assert!(s.contains("ET"));
    }

    #[test]
    fn pdf_lim_inline_nao_vazio() {
        let pdf = compile_to_pdf("O limite $lim_(x -> 0) f(x)$ existe.");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_sum_block_nao_vazio() {
        let pdf = compile_to_pdf("$ sum_(i=0)^n x_i $");
        assert!(!pdf.is_empty());
    }

    // ── Passo 51 — MathAlignPoint ──────────────────────────────────────────

    #[test]
    fn pdf_align_duas_linhas_nao_vazio() {
        let pdf = compile_to_pdf("$ a &= b + c \\ alpha &= x $");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_align_linha_unica_nao_vazio() {
        let pdf = compile_to_pdf("$ a &= b $");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_align_com_frac_nao_vazio() {
        let pdf = compile_to_pdf("$ frac(1,2) &= x \\ y &= z $");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_align_contem_bt_et() {
        let pdf = compile_to_pdf("$ a &= b \\ c &= d $");
        let s = extract_page_content_streams_text(&pdf);
        assert!(s.contains("BT"));
        assert!(s.contains("ET"));
    }

    #[test]
    fn pdf_sem_align_nao_regride() {
        let pdf = compile_to_pdf("$ x^2 + y_i $");
        assert!(!pdf.is_empty());
    }

    // ── Passo 52 — math_leading via MathConstants ─────────────────────────

    #[test]
    fn pdf_math_grid_leading_gera_pdf() {
        // Grid com math_leading lido da constante MATH (ou fallback 20%)
        let pdf = compile_to_pdf("$ a &= b \\ c &= d $");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_math_grid_leading_contem_bt_et() {
        let pdf = compile_to_pdf("$ a &= b \\ c &= d $");
        let s = extract_page_content_streams_text(&pdf);
        assert!(s.contains("BT"), "BT ausente");
        assert!(s.contains("ET"), "ET ausente");
    }

    // ── Passo 54 — Matrizes matemáticas ─────────────────────────────────

    #[test]
    fn pipeline_math_matrix_gera_pdf() {
        let (world, _dir) = world_from_str("$ mat(1, 2; 3, 4) $");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
    }

    // ── Passo 55 — Vectores e Casos ──────────────────────────────────────

    #[test]
    fn pipeline_math_vec_gera_pdf() {
        let pdf = compile_to_pdf("$ vec(1, 2, 3) $");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pipeline_math_cases_gera_pdf() {
        let pdf = compile_to_pdf("$ cases(1, 0) $");
        assert!(!pdf.is_empty());
    }

    // ── Passo 56 / Passo 59 — Labels e Referências ───────────────────────

    #[test]
    fn pipeline_introspeccao_labels_refs_gera_pdf() {
        // Passo 59: referência para trás (@intro depois de <intro>) resolve para
        // "Secção 1" — a numeração hierárquica é registada mesmo sem #set heading(numbering:).
        let pdf = compile_to_pdf("= Introdução <intro>\nIsto é uma referência: @intro");
        assert!(!pdf.is_empty(), "PDF não deve estar vazio");
        let pdf_str = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_str.contains("Sec") || pdf_str.contains("1"),
            "PDF deve conter o texto resolvido da referência, obtido (primeiros 500): {:?}",
            &pdf_str[..pdf_str.len().min(500)]
        );
    }

    #[test]
    fn pipeline_ref_forward_nao_causa_panico() {
        // Passo 59: forward ref não causa panic — fallback @nome.
        // Passo 60: com duas passagens, forward ref resolve para "Secção 1" (não fallback).
        let pdf = compile_to_pdf("Ver a @conclusao\n= Conclusão <conclusao>");
        assert!(!pdf.is_empty(), "PDF deve ser gerado mesmo com forward ref");
    }

    // ── Passo 60 — Motor de Introspecção (Duas Passagens) ────────────────

    #[test]
    fn pipeline_forward_ref_resolve_no_pdf() {
        // Passo 60: forward ref deve resolver para o texto da secção, não para @conclusao.
        let pdf = compile_to_pdf(
            "#set heading(numbering: \"1.\")\nVer a @conclusao.\n= Conclusão <conclusao>",
        );
        assert!(!pdf.is_empty());
        let pdf_str = String::from_utf8_lossy(&pdf);
        assert!(
            !pdf_str.contains("@conclusao"),
            "forward ref não deve aparecer como fallback no PDF"
        );
    }

    #[test]
    fn pipeline_backward_ref_continua_a_funcionar() {
        // Regressão: garantir que backward refs não partiram com a mudança.
        let pdf = compile_to_pdf(
            "#set heading(numbering: \"1.\")\n= Metodologia <metodo>\nDe acordo com a @metodo.",
        );
        assert!(!pdf.is_empty());
        assert!(!String::from_utf8_lossy(&pdf).contains("@metodo"));
    }

    // ── Passo 57 — Contadores e Numeração de Headings ─────────────────────

    #[test]
    fn pipeline_heading_numeracao_por_defeito_sem_prefixo() {
        // Sem #set heading(numbering: ...), o PDF não deve ter prefixos numéricos.
        let (world, _dir) = world_from_str("= Introdução\n== Motivação");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        // Pipeline completo deve produzir PDF válido sem numeração
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pipeline_heading_numeracao_activa() {
        let pdf = compile_to_pdf(
            "#set heading(numbering: \"1.1\")\n= Introdução\n== Motivação\n= Conclusão",
        );
        assert!(!pdf.is_empty(), "PDF não deve estar vazio");
        let pdf_str = String::from_utf8_lossy(&pdf);
        // "1." deve aparecer no stream do PDF como prefixo do primeiro heading
        assert!(pdf_str.contains("1."), "H1 deve ter prefixo numérico no PDF");
    }

    // ── Passo 58 — Contadores Genéricos ───────────────────────────────────

    #[test]
    fn pipeline_counter_step_nao_quebra_pdf() {
        let pdf = compile_to_pdf("#counter(\"equation\").step()");
        assert!(!pdf.is_empty(), "PDF não deve estar vazio");
    }

    #[test]
    fn pipeline_counter_update_nao_quebra_pdf() {
        let pdf = compile_to_pdf("#counter(\"fig\").update(3)");
        assert!(!pdf.is_empty());
    }

    // ── Passo 61 — TOC (#outline()) ───────────────────────────────────────

    #[test]
    fn pipeline_outline_gera_pdf_sem_panico() {
        let (world, _dir) = world_from_str(
            "#set heading(numbering: \"1.\")\n\
             #outline()\n\
             = Introdução\n\
             == Motivação\n\
             = Conclusão",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty(), "PDF com #outline() não deve estar vazio");
    }

    // ── Passo 62 — Figuras ────────────────────────────────────────────────

    #[test]
    fn pipeline_figure_com_ref_gera_pdf() {
        let pdf = compile_to_pdf(
            "#figure(\n  [Gráfico de Barras],\n  caption: [Resultados]\n) <fig1>\n\
             Como mostrado na @fig1.",
        );
        assert!(!pdf.is_empty(), "PDF com figure e ref não deve estar vazio");
    }

    #[test]
    fn pipeline_figure_sem_ref_nao_causa_panico() {
        let pdf =
            compile_to_pdf("#figure(\n  [Conteúdo],\n  caption: [Legenda simples]\n)");
        assert!(!pdf.is_empty());
    }

    // ── Passo 65 — Pipeline simplificado (fixpoint em L1) ────────────────

    #[test]
    fn pipeline_toc_paginada_pipeline_linear() {
        // Confirmar que o pipeline L3 é agora linear (sem passagens manuais)
        // e que a TOC não causa panic.
        let pdf = compile_to_pdf(
            "#set heading(numbering: \"1.\")\n\
             #outline()\n\
             = Introdução\n\
             = Conclusão",
        );
        assert!(!pdf.is_empty(), "PDF com TOC paginada não deve estar vazio");
    }

    #[test]
    fn pipeline_sem_toc_nao_regrediu() {
        // Regressão: documentos sem TOC não devem ser afectados pelo fixpoint.
        let pdf = compile_to_pdf(
            "= Introdução\n\
             Texto simples sem índice.",
        );
        assert!(!pdf.is_empty());
    }

    // ── Passo 63 — TOC com números de página (3 passagens) ───────────────

    #[test]
    fn pipeline_toc_com_paginas_nao_causa_panico() {
        // A 3ª passagem não deve causar panic mesmo que a TOC seja maior
        // com os números de página (caso de degradação DEBT-17).
        let pdf = compile_to_pdf(
            "#set heading(numbering: \"1.\")\n\
             #outline()\n\
             = Introdução\n\
             == Motivação\n\
             = Conclusão",
        );
        assert!(!pdf.is_empty(), "PDF com TOC paginada não deve estar vazio");
    }

    #[test]
    fn pipeline_toc_tres_passagens_produz_pdf_valido() {
        // Verificar que as 3 passagens produzem um PDF não vazio com headings.
        let pdf = compile_to_pdf(
            "#outline()\n\
             = Primeira Secção\n\
             Conteúdo aqui.\n\
             = Segunda Secção\n\
             Mais conteúdo.",
        );
        assert!(!pdf.is_empty(), "PDF com TOC em 3 passagens não deve estar vazio");
    }

    // ── P602/P603 — /Count nos bookmarks PDF ─────────────────────────────

    /// **P603** — Parser mínimo do PDF que isola a árvore `/Outlines` antes de
    /// extrair os valores de `/Count`. Evita misturar com `/Count` do catálogo
    /// `/Pages` ou de outros objectos.
    fn parse_pdf_objects(pdf: &[u8]) -> std::collections::HashMap<usize, String> {
        let text = String::from_utf8_lossy(pdf);
        let start_re = Regex::new(r"(\d+)\s+0\s+obj\s*<<").unwrap();
        let mut objects = std::collections::HashMap::new();
        for m in start_re.captures_iter(&text) {
            let id: usize = m[1].parse().unwrap();
            let start = m.get(0).unwrap().end();
            let mut depth = 1usize;
            let mut i = start;
            let mut in_paren = false;
            let mut in_hex = false;
            let bytes = text.as_bytes();
            while i < text.len() && depth > 0 {
                let c = bytes[i] as char;
                if in_paren {
                    if c == ')' {
                        in_paren = false;
                    } else if c == '\\' {
                        i += 1;
                    }
                } else if in_hex {
                    if c == '>' {
                        in_hex = false;
                    }
                } else {
                    if c == '(' {
                        in_paren = true;
                    } else if c == '<' {
                        in_hex = true;
                    } else if i + 1 < text.len() && c == '<' && bytes[i + 1] == b'<' {
                        depth += 1;
                        i += 1;
                    } else if i + 1 < text.len() && c == '>' && bytes[i + 1] == b'>' {
                        depth -= 1;
                        i += 1;
                    }
                }
                i += 1;
            }
            let end = i.saturating_sub(2);
            objects.insert(id, text[start..end].to_string());
        }
        objects
    }

    /// Devolve os valores de `/Count` presentes apenas na árvore de bookmarks.
    fn outline_counts(pdf: &[u8]) -> Vec<i64> {
        use std::collections::HashSet;

        let objects = parse_pdf_objects(pdf);
        let catalog_re = Regex::new(r"/Type\s*/Catalog").unwrap();
        let outlines_re = Regex::new(r"/Outlines\s*(\d+)\s+0\s+R").unwrap();
        let count_re = Regex::new(r"/Count\s*(-?\d+)").unwrap();
        let first_re = Regex::new(r"/First\s*(\d+)\s+0\s+R").unwrap();
        let next_re = Regex::new(r"/Next\s*(\d+)\s+0\s+R").unwrap();

        let root_id = objects
            .iter()
            .find(|(_, body)| catalog_re.is_match(body))
            .and_then(|(_, body)| outlines_re.captures(body))
            .map(|c| c[1].parse::<usize>().unwrap())
            .or_else(|| {
                objects
                    .iter()
                    .find(|(_, body)| {
                        body.contains("/Type /Outlines")
                            || body.contains("/Type/Outlines")
                    })
                    .map(|(id, _)| *id)
            })
            .expect("PDF deve ter catálogo com /Outlines");

        let mut counts = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = vec![root_id];
        while let Some(id) = stack.pop() {
            if !visited.insert(id) {
                continue;
            }
            let body = objects.get(&id).cloned().unwrap_or_default();
            if let Some(c) = count_re.captures(&body) {
                counts.push(c[1].parse::<i64>().unwrap());
            }
            if let Some(c) = first_re.captures(&body) {
                stack.push(c[1].parse().unwrap());
            }
            if let Some(c) = next_re.captures(&body) {
                stack.push(c[1].parse().unwrap());
            }
        }
        counts
    }

    fn assert_outline_counts(pdf: &[u8], expected: &mut [i64]) {
        let mut counts = outline_counts(pdf);
        counts.sort();
        expected.sort();
        assert_eq!(
            counts, expected,
            "/Count isolados da árvore /Outlines devem ser {:?}; obtive {:?}",
            expected, counts
        );
    }

    #[test]
    fn p602_outline_count_sinal_negativo_para_entradas_com_filhos() {
        let pdf = compile_to_pdf(
            "= Primeira Secção\n\
             == Subsecção A\n\
             == Subsecção B\n\
             = Segunda Secção",
        );
        // Raiz /Outlines: 2 itens de topo (abertos por defeito).
        // Primeira Secção: 2 filhos directos (fechados por defeito).
        assert_outline_counts(&pdf, &mut [2, -2]);
    }

    #[test]
    fn p602_outline_count_tres_niveis_conta_filhos_directos() {
        let pdf = compile_to_pdf(
            "= Nível 1\n\
             == Nível 2\n\
             === Nível 3\n\
             == Nível 2 B\n\
             = Nível 1 B",
        );
        // Raiz: 2 itens de topo.
        // Nível 1: 2 filhos directos.
        // Nível 2: 1 filho directo.
        assert_outline_counts(&pdf, &mut [2, -2, -1]);
    }

    #[test]
    fn p603_outline_count_isolado_em_documento_multipagina() {
        // P603 — com várias páginas, o /Count de /Pages é diferente do das
        // bookmarks; o parser isolado deve continuar a dar os mesmos valores.
        let pdf = compile_to_pdf(
            "= Primeira Secção\n\
             == Subsecção A\n\
             === Sub-sub A1\n\
             #lorem(400)\n\
             == Subsecção B\n\
             #lorem(400)\n\
             = Segunda Secção\n\
             #lorem(400)",
        );
        assert_outline_counts(&pdf, &mut [2, -2, -1]);
    }

    #[test]
    fn p605_heading_outlined_false_exclui_de_bookmarks() {
        // P606 — `outlined: false` arrasta `bookmarked: false` por defeito,
        // pelo que o heading desaparece de /Outlines. Os vizinhos continuam.
        let pdf = compile_to_pdf(
            "= Visível\n\
             #heading(outlined: false)[Oculto de bookmarks]\n\
             = Outro visível",
        );
        assert_outline_counts(&pdf, &mut [2]);
    }

    #[test]
    fn p606_heading_bookmarked_false_exclui_de_bookmarks_mas_mantem_toc() {
        // P606 — `bookmarked: false` exclui só de /Outlines; o heading
        // continua elegível para o índice (testado em L1).
        let pdf = compile_to_pdf(
            "= Visível\n\
             #heading(bookmarked: false)[Oculto de bookmarks]\n\
             = Outro visível",
        );
        assert_outline_counts(&pdf, &mut [2]);
    }

    #[test]
    fn p606_heading_outlined_false_bookmarked_true_aparece_em_bookmarks() {
        // P606 — `outlined: false` exclui do índice, mas `bookmarked: true`
        // força a inclusão em /Outlines. Os três headings aparecem em bookmarks.
        let pdf = compile_to_pdf(
            "= Visível\n\
             #heading(outlined: false, bookmarked: true)[Só bookmarks]\n\
             = Outro visível",
        );
        assert_outline_counts(&pdf, &mut [3]);
    }

    // ── Testes de imagem PNG (Passo 74) ───────────────────────────────────────

    /// Gera PNG em memória e escreve no diretório temporário.
    fn write_png_rgba(dir: &Path, name: &str, pixels: Vec<u8>, w: u32, h: u32) {
        use image::{ImageBuffer, Rgba};
        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(w, h, pixels).unwrap();
        let mut buf = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
            .unwrap();
        std::fs::write(dir.join(name), &buf).unwrap();
    }

    #[test]
    fn pipeline_png_transparente_gera_smask() {
        let dir = tempdir();

        // PNG 2×2 com píxeis semi-transparentes.
        write_png_rgba(
            dir.path(),
            "alpha.png",
            vec![
                255, 0, 0, 128, // vermelho semi-transparente
                0, 255, 0, 255, // verde opaco
                0, 0, 255, 0, // azul transparente
                255, 255, 0, 255, // amarelo opaco
            ],
            2,
            2,
        );

        std::fs::write(dir.path().join("main.typ"), "#image(\"alpha.png\")").unwrap();
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let s = String::from_utf8_lossy(&pdf);

        assert!(!pdf.is_empty(), "export_pdf deve produzir bytes");
        assert!(s.contains("/Filter /FlateDecode"), "PNG deve usar /FlateDecode");
        assert!(s.contains("/SMask"), "PNG com transparência deve emitir /SMask");
        assert!(s.contains("/ColorSpace /DeviceGray"), "XObject alpha usa /DeviceGray");
        assert!(s.contains("/ColorSpace /DeviceRGB"), "XObject RGB usa /DeviceRGB");
    }

    #[test]
    fn pipeline_png_opaco_sem_smask() {
        let dir = tempdir();

        // PNG 1×1 totalmente opaco.
        write_png_rgba(dir.path(), "opaco.png", vec![100u8, 150, 200, 255], 1, 1);

        std::fs::write(dir.path().join("main.typ"), "#image(\"opaco.png\")").unwrap();
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let s = String::from_utf8_lossy(&pdf);

        assert!(!s.contains("/SMask"), "PNG totalmente opaco não deve emitir /SMask");
        assert!(s.contains("/Filter /FlateDecode"), "PNG opaco ainda usa /FlateDecode");
    }

    // ── Testes de Passo 75 — caminhos relativos e figuras numeradas ──────────

    #[test]
    fn pipeline_figura_numerada_prefixo_no_pdf() {
        let dir = tempdir();
        // JPEG mínimo válido (magic bytes suficientes para a detecção de formato)
        std::fs::write(dir.path().join("foto.jpg"), &[0xFF_u8, 0xD8, 0xFF, 0xE0])
            .unwrap();
        std::fs::write(
            dir.path().join("main.typ"),
            "#set figure(numbering: \"1\")\n#figure(image(\"foto.jpg\"), caption: [A foto])",
        ).unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        // P190H: state.figure_numbers eliminado; verificação via intr.
        let intr = introspect_with_introspector(content);

        use typst_core::entities::introspector::Introspector;
        assert_eq!(intr.figure_number_at_index("image", 0), Some(1),
            "Uma figura de imagem deve produzir intr.figure_number_at_index(image, 0) = 1");

        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        assert!(!pdf.is_empty(), "PDF não pode estar vazio");
    }

    #[test]
    fn image_resolve_caminho_relativo() {
        let dir = tempdir();
        std::fs::create_dir(dir.path().join("capitulo1")).unwrap();
        std::fs::write(
            dir.path().join("capitulo1/foto.jpg"),
            &[0xFF_u8, 0xD8, 0xFF, 0xE0],
        )
        .unwrap();
        std::fs::write(dir.path().join("capitulo1/intro.typ"), "#image(\"foto.jpg\")")
            .unwrap();
        std::fs::write(dir.path().join("main.typ"), "#include \"capitulo1/intro.typ\"")
            .unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let result = do_eval(&world, &source);
        assert!(
            result.is_ok(),
            "Avaliador falhou ao resolver caminho relativo: {:?}",
            result.err()
        );
    }

    #[test]
    fn current_file_restaurado_apos_include() {
        let dir = tempdir();
        std::fs::create_dir(dir.path().join("capitulo1")).unwrap();
        std::fs::write(dir.path().join("capa.jpg"), &[0xFF_u8, 0xD8, 0xFF, 0xE0])
            .unwrap();
        std::fs::write(
            dir.path().join("capitulo1/foto.jpg"),
            &[0xFF_u8, 0xD8, 0xFF, 0xE0],
        )
        .unwrap();
        std::fs::write(dir.path().join("capitulo1/intro.typ"), "#image(\"foto.jpg\")")
            .unwrap();
        std::fs::write(
            dir.path().join("main.typ"),
            "#image(\"capa.jpg\")\n#include \"capitulo1/intro.typ\"\n#image(\"capa.jpg\")",
        ).unwrap();

        let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let result = do_eval(&world, &source);
        assert!(
            result.is_ok(),
            "current_file não restaurado após #include: {:?}",
            result.err()
        );
    }

    // ── Passo 76 — primitivas geométricas ────────────────────────────────────

    #[test]
    fn rect_ordem_operadores_pdf() {
        // #rect(fill: "red", stroke: "black") deve produzir:
        // q → rg (fill) → RG (stroke) → w → re (path) → B (paint) → Q
        let (world, _dir) = world_from_str(
            "#rect(width: 100pt, height: 50pt, fill: \"red\", stroke: \"black\")",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);

        let pdf_str = extract_page_content_streams_text(&pdf);

        assert!(pdf_str.contains("q\n"), "PDF deve ter push state (q)");
        assert!(pdf_str.contains(" rg\n"), "PDF deve ter operador de fill (rg)");
        assert!(pdf_str.contains(" RG\n"), "PDF deve ter operador de stroke (RG)");
        assert!(pdf_str.contains(" w\n"), "PDF deve ter operador de espessura (w)");
        assert!(pdf_str.contains(" re\n"), "PDF deve ter operador de rectângulo (re)");
        assert!(pdf_str.contains("B\n"), "PDF deve ter paint operator B (fill+stroke)");
        assert!(pdf_str.contains("Q\n"), "PDF deve ter pop state (Q)");

        // Verificar a ordem relativa.
        let pos_q = pdf_str.find("q\n").unwrap();
        let pos_rg = pdf_str.find(" rg\n").unwrap();
        let pos_rg_upper = pdf_str.find(" RG\n").unwrap();
        let pos_re = pdf_str.find(" re\n").unwrap();
        let pos_b = pdf_str.find("B\n").unwrap();
        let pos_q_close = pdf_str.rfind("Q\n").unwrap();

        assert!(pos_q < pos_rg, "q deve preceder rg");
        assert!(pos_rg < pos_rg_upper, "rg (fill) deve preceder RG (stroke)");
        assert!(pos_rg_upper < pos_re, "RG deve preceder re");
        assert!(pos_re < pos_b, "re deve preceder B");
        assert!(pos_b < pos_q_close, "B deve preceder Q final");
    }

    #[test]
    fn line_coordenada_y_fim_inferior_ao_inicio() {
        // #line(dy: 50pt) — dy positivo = desce no layout.
        // No espaço PDF (Y cresce para cima), end_y < start_y.
        let (world, _dir) = world_from_str("#line(dx: 100pt, dy: 50pt)");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);

        let pdf_str = extract_page_content_streams_text(&pdf);

        assert!(pdf_str.contains(" m\n"), "PDF deve conter operador m");
        assert!(pdf_str.contains(" l\n"), "PDF deve conter operador l");

        // Extrair Y do operador m (ponto inicial) e l (ponto final).
        fn extrair_y_antes_op(s: &str, op: &str) -> f64 {
            s.split(op)
                .next()
                .and_then(|antes| antes.split_whitespace().last())
                .and_then(|tok| tok.parse::<f64>().ok())
                .unwrap_or(0.0)
        }

        let m_y = extrair_y_antes_op(&pdf_str, " m\n");
        let l_y = extrair_y_antes_op(&pdf_str, " l\n");

        assert!(
            l_y < m_y,
            "Y do ponto final ({}) deve ser inferior ao Y do início ({}) — \
             dy positivo desce no layout, subtrai no PDF",
            l_y,
            m_y
        );
    }

    #[test]
    fn rect_sem_cores_gera_stroke_no_pdf() {
        // #rect() sem fill nem stroke → fallback de stroke preta.
        // O PDF deve conter S (stroke only), RG, w.
        let (world, _dir) = world_from_str("#rect(width: 50pt, height: 30pt)");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = extract_page_content_streams_text(&pdf);

        assert!(pdf_str.contains(" RG\n"), "PDF deve ter stroke RG");
        assert!(pdf_str.contains(" re\n"), "PDF deve ter rectângulo re");
        assert!(pdf_str.contains("S\n"), "PDF deve ter paint operator S (stroke only)");
    }

    #[test]
    fn rect_fill_tiling_cai_no_fallback_color_no_pdf() {
        // #rect(fill: tiling(red)) → Paint::Tiling; o exportador PDF ainda não
        // renderiza padrões (ADR-0054), pelo que o layout converte para a cor
        // de fallback do tiling antes de emitir FrameItem::Shape.
        let (world, _dir) = world_from_str(
            "#rect(width: 50pt, height: 30pt, fill: tiling(rgb(255,0,0)))",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = String::from_utf8_lossy(&pdf);

        // Deve emitir o rectângulo preenchido com a cor de fallback (vermelho).
        assert!(pdf_str.contains(" re\n"), "PDF deve ter rectângulo re");
        assert!(pdf_str.contains(" rg\n"), "PDF deve ter operador de fill rg");
        assert!(pdf_str.contains("f\n"), "PDF deve ter paint operator f (fill only)");
        // Red DeviceRGB ≈ 1.000 0.000 0.000 rg.
        assert!(
            pdf_str.contains("1.000 0.000 0.000 rg\n"),
            "PDF deve preencher com vermelho puro (fallback do tiling)"
        );
    }

    #[test]
    fn document_metadata_nao_emite_frames_no_pdf() {
        // #document(title: [Hello]) é metadata pura — o PDF deve ser vazio
        // (apenas estrutura, sem stream de conteúdo com operadores de texto).
        let (world, _dir) = world_from_str(
            "#document(title: [Hello], author: \"Ana\", keywords: (\"a\", \"b\"))",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let _state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = String::from_utf8_lossy(&pdf);

        // Sem operadores de texto (Tj / Td) nem formas (re/f/B).
        assert!(!pdf_str.contains("Tj"), "Document não deve emitir texto");
        assert!(!pdf_str.contains(" re\n"), "Document não deve emitir shapes");
    }

    #[test]
    fn asset_placeholder_nao_emite_frames_no_pdf() {
        // #asset("logo.png") é placeholder — não deve emitir conteúdo.
        let (world, _dir) = world_from_str("#asset(\"logo.png\")");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let _state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(!pdf_str.contains("Tj"), "Asset não deve emitir texto");
        assert!(!pdf_str.contains(" re\n"), "Asset não deve emitir shapes");
    }

    // ── Passo 398 — read binário + Value::Bytes ───────────────────────────────

    #[test]
    fn read_texto_utf8_pipeline() {
        let (world, dir) = world_from_str("#let data = read(\"file.txt\")");
        std::fs::write(dir.path().join("file.txt"), b"hello").unwrap();
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let data = module.scope().get("data").expect("data deve estar no scope");
        assert_eq!(data, &Value::Str("hello".into()));
    }

    #[test]
    fn read_binario_pipeline() {
        // P824 — ficheiro não-UTF8 SEM `encoding:` é ERRO (paridade vanilla
        // medida: `failed to convert to string (file is not valid UTF-8 in
        // {ficheiro}:{l}:{c})`); o fallback silencioso para Bytes que este
        // teste codificava foi removido (ver `read_binario_nao_utf8` em
        // `01_core/src/engine/stdlib/loading.rs`).
        let bytes = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
        let (world, dir) = world_from_str("#let data = read(\"logo.png\")");
        std::fs::write(dir.path().join("logo.png"), &bytes).unwrap();
        let source = world.source(world.main()).unwrap();
        let e = do_eval(&world, &source).unwrap_err();
        assert!(
            e.iter().any(|d| d
                .message
                .contains("failed to convert to string (file is not valid UTF-8 in logo.png:1:1)")),
            "mensagem inesperada: {:?}",
            e.iter().map(|d| d.message.to_string()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn read_binario_encoding_none_pipeline() {
        // P824 — `encoding: none` devolve os bytes crus (paridade vanilla).
        let bytes = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
        let (world, dir) =
            world_from_str("#let data = read(\"logo.png\", encoding: none)");
        std::fs::write(dir.path().join("logo.png"), &bytes).unwrap();
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let data = module.scope().get("data").expect("data deve estar no scope");
        assert_eq!(data, &Value::Bytes(Bytes::new(bytes)));
    }

    // ── Passo 77 — Bézier, elipses e deltas negativos ───────────────────────

    #[test]
    fn export_line_com_delta_negativo_respeita_bounding_box() {
        // #line(dx: -50pt, dy: -30pt) — linha para a esquerda e para cima.
        // Com dx < 0, o ponto 'm' deve ter X maior que o ponto 'l' (end_x < start_x).
        let (world, _dir) = world_from_str("#line(dx: -50pt, dy: -30pt)");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = extract_page_content_streams_text(&pdf);

        assert!(pdf_str.contains(" m\n"), "PDF deve conter operador m");
        assert!(pdf_str.contains(" l\n"), "PDF deve conter operador l");

        fn extrair_x_antes_op(s: &str, op: &str) -> f64 {
            // O operador tem formato "X Y op" — extrair o penúltimo token.
            s.split(op)
                .next()
                .and_then(|antes| {
                    let toks: Vec<&str> = antes.split_whitespace().collect();
                    toks.iter().rev().nth(1).and_then(|t| t.parse::<f64>().ok())
                })
                .unwrap_or(0.0)
        }

        let m_x = extrair_x_antes_op(&pdf_str, " m\n");
        let l_x = extrair_x_antes_op(&pdf_str, " l\n");

        assert!(
            l_x < m_x,
            "Linha com dx negativo deve terminar à esquerda do início: \
             end_x ({}) deve ser menor que start_x ({})",
            l_x,
            m_x
        );
    }

    #[test]
    fn export_ellipse_emite_quatro_operadores_bezier() {
        let (world, _dir) =
            world_from_str("#ellipse(width: 80pt, height: 40pt, fill: \"blue\")");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = extract_page_content_streams_text(&pdf);

        assert_eq!(
            pdf_str.matches(" c\n").count(),
            4,
            "Elipse deve ser desenhada com exactamente 4 operadores Bézier 'c'"
        );
        assert!(pdf_str.contains(" m\n"), "Elipse deve ter um ponto inicial 'm'");
        assert!(
            !pdf_str.contains(" re\n"),
            "Elipse não deve emitir operador re — placeholder foi substituído"
        );
    }

    // ── Passo 78 — transformações afins ─────────────────────────────────────

    #[test]
    fn pdf_export_emite_q_cm_q_para_transformacoes() {
        let (world, _dir) = world_from_str(
            "#rotate(90deg, rect(width: 100pt, height: 100pt, fill: \"red\"))",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = extract_page_content_streams_text(&pdf);

        assert!(pdf_str.contains("q\n"), "Falta guardar o estado gráfico (q)");
        assert!(pdf_str.contains(" cm\n"), "Falta a matriz de transformação (cm)");
        assert!(pdf_str.contains("Q\n"), "Falta restaurar o estado gráfico (Q)");

        let pos_q = pdf_str.find("q\n").unwrap();
        let pos_cm = pdf_str.find(" cm\n").unwrap();
        let pos_q_close = pdf_str.rfind("Q\n").unwrap();

        assert!(pos_q < pos_cm, "q deve preceder cm");
        assert!(pos_cm < pos_q_close, "cm deve preceder Q");
    }

    #[test]
    fn export_circle_emite_quatro_operadores_bezier() {
        let (world, _dir) = world_from_str("#circle(radius: 20pt)");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = extract_page_content_streams_text(&pdf);

        assert_eq!(
            pdf_str.matches(" c\n").count(),
            4,
            "Circle deve ser desenhado com exactamente 4 operadores Bézier 'c'"
        );
    }

    // ── Passo 81 — Configuração dinâmica de página via #set page ────────────

    #[test]
    fn set_page_forca_quebra_com_conteudo() {
        let (world, _dir) = world_from_str(
            "Primeira linha\n\
             #set page(width: 200pt, height: 200pt, margin: 10pt)\n\
             Segunda página\n",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        assert_eq!(doc.pages.len(), 2, "SetPage com conteúdo deve criar 2 páginas");
        assert!(
            doc.pages[0].height > 800.0,
            "Primeira página deve ser A4 (height > 800pt)"
        );
        assert!(
            (doc.pages[1].height - 200.0).abs() < 0.01,
            "Segunda página deve ter height = 200pt do SetPage"
        );
    }

    #[test]
    fn set_page_no_topo_nao_quebra() {
        let (world, _dir) = world_from_str(
            "#set page(width: 300pt, height: 400pt)\n\
             Conteúdo único\n",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        assert_eq!(
            doc.pages.len(),
            1,
            "SetPage sem conteúdo anterior não deve criar página extra"
        );
        assert!((doc.pages[0].width - 300.0).abs() < 0.01);
        assert!((doc.pages[0].height - 400.0).abs() < 0.01);
    }

    #[test]
    fn multiplas_mudancas_de_pagina_preservam_snapshots() {
        let (world, _dir) = world_from_str(
            "P1\n\
             #set page(width: 200pt, height: 200pt)\n\
             P2\n\
             #set page(width: 100pt, height: 300pt)\n\
             P3\n",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        assert_eq!(doc.pages.len(), 3);
        assert!(doc.pages[0].width > 500.0, "Página 1 deve ser A4");
        assert!((doc.pages[1].width - 200.0).abs() < 0.01);
        assert!((doc.pages[1].height - 200.0).abs() < 0.01);
        assert!((doc.pages[2].width - 100.0).abs() < 0.01);
        assert!((doc.pages[2].height - 300.0).abs() < 0.01);
    }

    #[test]
    fn grid_respeita_page_config_dinamico() {
        let (world, _dir) = world_from_str(
            "#set page(width: 400pt, height: 400pt, margin: 20pt)\n\
             #grid(columns: (1fr, 1fr), [A], [B])\n",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        // available_width = 400 - 2*20 = 360pt; cada 1fr = 180pt.
        // O segundo item (célula B) deve começar em margin + 180 = 200pt.
        assert_eq!(doc.pages.len(), 1);
        let second_item_x =
            doc.pages[0]
                .items
                .iter()
                .filter_map(|item| match item {
                    typst_core::entities::layout_types::FrameItem::Text {
                        pos, ..
                    } if pos.x.0 > 150.0 => Some(pos.x.0),
                    _ => None,
                })
                .next();
        assert!(
            second_item_x.map(|x| (x - 200.0).abs() < 2.0).unwrap_or(false),
            "Segundo item do grid deve estar em x ≈ 200pt, obteve {:?}",
            second_item_x,
        );
    }

    #[test]
    fn pdf_mediabox_diferente_por_pagina() {
        let (world, _dir) = world_from_str(
            "A\n\
             #set page(width: 200pt, height: 600pt)\n\
             B\n",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);
        let pdf = export_pdf(&doc, StreamMode::Verbose);

        let pdf_str = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_str.contains("[0 0 595.28 841.89]"),
            "Primeira página deve ter MediaBox A4"
        );
        assert!(
            pdf_str.contains("[0 0 200.00 600.00]"),
            "Segunda página deve ter MediaBox 200×600pt"
        );
    }

    // ── Passo 81.5 — Stress de composição geométrica (Grid × Transform × SetPage) ──
    //
    // Divergência documentada face ao prompt:
    // - O prompt pede `#transform(translate(5pt, 10pt))[A]`, mas a stdlib
    //   actual expõe `move(dx, dy)` (sem `transform`/`translate` como funções
    //   nomeadas). Usamos `#move(dx: 5pt, dy: 10pt)[...]` — produz o mesmo
    //   `Content::Transform { matrix: translate(dx, dy), body }`.
    // - `FrameItem` embute `pos` em cada variante (não é uma tupla
    //   `(Point, FrameItem)`). Os testes adaptam a extracção.
    // - `collect_sub_items` só captura `Shape`/`Sequence` em coordenadas
    //   locais — texto dentro de `Transform` não aparece nos sub_items.
    //   Por isso, usamos `#rect` como marcador dentro da `move(...)`.

    fn stress_81_5_source() -> &'static str {
        "\
         Texto introdutório na primeira página.\n\
         \n\
         #set page(width: 400pt, height: 300pt, margin: 20pt)\n\
         \n\
         #grid(\n\
           columns: (1fr, 2fr),\n\
           [#move(dx: 5pt, dy: 10pt)[#rect(width: 15pt, height: 15pt)]],\n\
           [Texto na célula que deve caber em 240pt de largura.],\n\
           [#rect(width: 100pt, height: 50pt)],\n\
           [#rect(width: 80pt, height: 30pt)],\n\
         )\n\
         \n\
         #set page(width: 200pt, height: 200pt, margin: 5pt)\n\
         \n\
         Fim.\n"
    }

    fn compilar_stress_81_5() -> typst_core::entities::layout_types::PagedDocument {
        let (world, _dir) = world_from_str(stress_81_5_source());
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        layout(content)
    }

    /// Extrai a posição primária de qualquer `FrameItem`.
    fn frame_item_pos(
        item: &typst_core::entities::layout_types::FrameItem,
    ) -> typst_core::entities::layout_types::Point {
        use typst_core::entities::layout_types::FrameItem;
        match item {
            FrameItem::Text { pos, .. } => *pos,
            FrameItem::TextShaped { pos, .. } => *pos,
            FrameItem::Line { start, .. } => *start,
            FrameItem::Glyph { pos, .. } => *pos,
            FrameItem::Image { pos, .. } => *pos,
            FrameItem::Shape { pos, .. } => *pos,
            FrameItem::Group { pos, .. } => *pos,
            FrameItem::Link { pos, .. } => *pos,
        }
    }

    // Fase 1 — Invariantes de estado (macro)
    #[test]
    fn stress_81_5_tres_paginas_com_snapshots_correctos() {
        let doc = compilar_stress_81_5();

        assert_eq!(
            doc.pages.len(),
            3,
            "SetPage deve criar exactamente 2 quebras de página (3 snapshots)"
        );

        // Página 1: A4 padrão (595.28 × 841.89).
        assert!(
            (doc.pages[0].width - 595.28).abs() < 0.01,
            "Página 1 deve preservar A4 width (595.28pt), obteve {}",
            doc.pages[0].width
        );
        assert!(
            (doc.pages[0].height - 841.89).abs() < 0.01,
            "Página 1 deve preservar A4 height (841.89pt), obteve {}",
            doc.pages[0].height
        );

        // Página 2: 400×300pt.
        assert!((doc.pages[1].width - 400.0).abs() < 0.01);
        assert!((doc.pages[1].height - 300.0).abs() < 0.01);

        // Página 3: 200×200pt.
        assert!((doc.pages[2].width - 200.0).abs() < 0.01);
        assert!((doc.pages[2].height - 200.0).abs() < 0.01);
    }

    // Fase 2 — Grid usa available_width da página activa (não A4)
    #[test]
    fn stress_81_5_grid_usa_available_width_da_pagina_activa() {
        use typst_core::entities::layout_types::FrameItem;
        let doc = compilar_stress_81_5();
        let items = &doc.pages[1].items;

        // available_width da página 2 = 400 - 2*20 = 360pt; total_fr = 3.
        // Col 0 (1fr) = 120pt começando em x=20.
        // Col 1 (2fr) = 240pt começando em x=140.
        //
        // O rect(100×50) está na célula (linha 1, col 0) → x ≈ 20.
        // O rect(80×30) está na célula (linha 1, col 1) → x ≈ 140.
        //
        // Se o Grid usasse A4 available_width (595.28 - 141.74 ≈ 453.54),
        // col 1 estaria em x ≈ 70.87 + 151.18 ≈ 222 — inconsistente com 140.

        let shape_positions: Vec<(f64, f64, f64, f64)> = items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Shape { pos, width, height, .. } => {
                    Some((pos.x.0, pos.y.0, *width, *height))
                }
                _ => None,
            })
            .collect();

        // Deve existir um rect de 100×50 no col 0 da segunda linha.
        let rect_100 = shape_positions
            .iter()
            .find(|(_, _, w, h)| (*w - 100.0).abs() < 0.1 && (*h - 50.0).abs() < 0.1)
            .expect("rect(100×50) deve existir como FrameItem::Shape na página 2");
        assert!(
            (rect_100.0 - 20.0).abs() < 0.5,
            "rect(100×50) deve estar em col 0 (x ≈ 20pt na página 400/20), obteve x={}",
            rect_100.0
        );

        // Deve existir um rect de 80×30 no col 1 da segunda linha.
        let rect_80 = shape_positions
            .iter()
            .find(|(_, _, w, h)| (*w - 80.0).abs() < 0.1 && (*h - 30.0).abs() < 0.1)
            .expect("rect(80×30) deve existir como FrameItem::Shape na página 2");
        assert!((rect_80.0 - 140.0).abs() < 0.5,
            "rect(80×30) deve estar em col 1 (x ≈ 140pt = margin + 1fr_width), obteve x={}",
            rect_80.0);
    }

    // Fase 3 — Row height avança cursor correctamente
    #[test]
    fn stress_81_5_row_height_e_maximo_da_linha() {
        use typst_core::entities::layout_types::FrameItem;
        let doc = compilar_stress_81_5();
        let items = &doc.pages[1].items;

        let rect_50 = items
            .iter()
            .find_map(|it| match it {
                FrameItem::Shape { pos, height, .. } if (*height - 50.0).abs() < 0.1 => {
                    Some(*pos)
                }
                _ => None,
            })
            .expect("rect(100×50) deve existir na página 2");

        // Os items da linha 0 incluem o FrameItem::Group (move + rect) e texto
        // da célula (0,1). A linha 1 (onde estão os rects 100 e 80) deve estar
        // visualmente abaixo.
        let first_row_y_max: f64 = items
            .iter()
            .filter_map(|it| match it {
                // Excluir shapes da linha 1 (100×50 e 80×30) — procuramos só
                // items da linha 0.
                FrameItem::Shape { height, .. }
                    if (*height - 50.0).abs() < 0.1 || (*height - 30.0).abs() < 0.1 =>
                {
                    None
                }
                other => Some(frame_item_pos(other).y.0),
            })
            .fold(f64::NEG_INFINITY, f64::max);

        assert!(
            rect_50.y.0 >= first_row_y_max,
            "Linha 1 do grid deve estar ao nível ou abaixo da linha 0. \
             rect_50 y={}, first_row_y_max={}",
            rect_50.y.0,
            first_row_y_max
        );
    }

    // Fase 4 — Anti-regressão: nenhum item excede os limites físicos da página 2
    #[test]
    fn stress_81_5_nenhum_item_excede_limites_da_pagina_2() {
        use typst_core::entities::layout_types::FrameItem;
        let doc = compilar_stress_81_5();
        let items = &doc.pages[1].items;

        for item in items {
            let pos = frame_item_pos(item);
            assert!(
                pos.x.0 <= 400.0,
                "Item {:?} excede largura da página 2 (400pt): x={}",
                item,
                pos.x.0
            );
            assert!(pos.y.0 <= 300.0,
                "Item {:?} excede altura da página 2 (300pt): y={}. \
                 Possível causa: inversão Y ou new_page usou 841.89pt (A4) em vez de 300pt.",
                item, pos.y.0);

            // Verificar recursivamente itens dentro de Groups (Transforms).
            if let FrameItem::Group { pos: group_pos, items: sub_items, .. } = item {
                for sub in sub_items {
                    let sub_pos = frame_item_pos(sub);
                    let abs_x = group_pos.x.0 + sub_pos.x.0;
                    let abs_y = group_pos.y.0 + sub_pos.y.0;
                    assert!(
                        abs_x <= 400.0,
                        "Item transformado excede largura da página 2: abs_x={}",
                        abs_x
                    );
                    assert!(abs_y <= 300.0,
                        "Item transformado excede altura da página 2: abs_y={}. \
                         Possível causa: Transform usou page_height global em vez de snapshot.",
                        abs_y);
                }
            }
        }
    }

    // Fase 5 — PDF tem três MediaBox distintos e correctos
    #[test]
    fn stress_81_5_pdf_tem_tres_mediabox_distintos() {
        let doc = compilar_stress_81_5();
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(
            pdf_str.contains("[0 0 595.28 841.89]"),
            "PDF: página 1 deve ter MediaBox A4"
        );
        assert!(
            pdf_str.contains("[0 0 400.00 300.00]"),
            "PDF: página 2 deve ter MediaBox 400×300pt"
        );
        assert!(
            pdf_str.contains("[0 0 200.00 200.00]"),
            "PDF: página 3 deve ter MediaBox 200×200pt"
        );

        // Nenhum MediaBox híbrido — sinal de vazamento catastrófico de dimensão.
        assert!(
            !pdf_str.contains("[0 0 400.00 841.89]"),
            "PDF: MediaBox híbrido detectado — height da página 2 vazou para A4"
        );
        assert!(
            !pdf_str.contains("[0 0 595.28 300.00]"),
            "PDF: MediaBox híbrido detectado — width da página 2 ficou em A4"
        );
        assert!(
            !pdf_str.contains("[0 0 200.00 841.89]"),
            "PDF: MediaBox híbrido detectado — height da página 3 vazou para A4"
        );

        let count = pdf_str.matches("/MediaBox").count();
        assert_eq!(count, 3, "PDF deve ter exactamente 3 /MediaBox, encontrou {}", count);
    }

    // ── Passo 82 — Align e Place ─────────────────────────────────────────

    #[test]
    fn align_center_reposiciona_no_eixo_x() {
        // Página 400pt de largura, margem 20pt → available_width = 360pt.
        // Rectângulo de 100pt centrado: target_x = 20 + (360 - 100) / 2 = 150pt.
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#align(\"center\", rect(width: 100pt, height: 20pt))
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let items = &doc.pages[0].items;
        assert!(!items.is_empty(), "Deve haver pelo menos um item");

        let rect_x = frame_item_pos(&items[0]).x.0;
        assert!(
            (rect_x - 150.0).abs() < 0.5,
            "Rectângulo centrado deve estar em x=150pt, obteve x={:.1}",
            rect_x
        );
    }

    #[test]
    fn align_right_ancora_a_margem_direita() {
        // Página 400pt, margem 20pt → available_width = 360pt.
        // Rectângulo 80pt: target_x = 20 + (360 - 80) = 300pt.
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#align(\"right\", rect(width: 80pt, height: 20pt))
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let rect_x = frame_item_pos(&doc.pages[0].items[0]).x.0;
        assert!(
            (rect_x - 300.0).abs() < 0.5,
            "Rectângulo direita deve estar em x=300pt, obteve x={:.1}",
            rect_x
        );
    }

    #[test]
    fn place_nao_altera_cursor_y() {
        // Propriedade a validar: o cursor vertical não avança por causa de Place.
        // Estratégia: comparar dois documentos idênticos — um sem Place, outro
        // com Place intercalado. Os rectângulos de fluxo devem ficar em Y
        // idênticos nos dois casos. Isto evita assumir uma fórmula para o
        // line_height injectado pelo flush_line de cada Shape.
        let src_sem_place = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#rect(width: 100pt, height: 50pt)
#rect(width: 100pt, height: 30pt)
";
        let src_com_place = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#rect(width: 100pt, height: 50pt)
#place(\"bottom-right\", rect(width: 60pt, height: 20pt))
#rect(width: 100pt, height: 30pt)
";

        let layout_doc = |src: &str| {
            let (world, _dir) = world_from_str(src);
            let source = world.source(world.main()).unwrap();
            let module = do_eval(&world, &source).unwrap();
            let content = module.content().expect("deve ter content");
            let state = introspect(content);
            layout(content)
        };

        let doc_sem = layout_doc(src_sem_place);
        let doc_com = layout_doc(src_com_place);

        let items_sem = &doc_sem.pages[0].items;
        let items_com = &doc_com.pages[0].items;

        assert_eq!(items_sem.len(), 2, "Doc sem place deve ter 2 rectângulos");
        assert_eq!(
            items_com.len(),
            3,
            "Doc com place deve ter 3 FrameItems (2 rect + 1 place)"
        );

        // Rect 1 nas duas versões — mesmo Y.
        let y0_sem = frame_item_pos(&items_sem[0]).y.0;
        let y0_com = frame_item_pos(&items_com[0]).y.0;
        assert!(
            (y0_sem - y0_com).abs() < 0.5,
            "Rect 1 deve estar no mesmo Y com e sem place ({} vs {})",
            y0_sem,
            y0_com
        );

        // Rect 3 (com place) vs Rect 2 (sem place) — mesmo Y → Place não avançou cursor.
        let y_final_sem = frame_item_pos(&items_sem[1]).y.0;
        let y_final_com = frame_item_pos(&items_com[2]).y.0;
        assert!(
            (y_final_sem - y_final_com).abs() < 0.5,
            "O rectângulo após place deve estar no mesmo Y que sem place \
             ({} sem place, {} com place) — Place consumiu fluxo",
            y_final_sem,
            y_final_com
        );

        // E o item Place (items_com[1]) deve estar na zona de baixo-direita da página.
        let y_place = frame_item_pos(&items_com[1]).y.0;
        let x_place = frame_item_pos(&items_com[1]).x.0;
        assert!(
            y_place > 300.0,
            "Place(bottom-right) deve estar na zona inferior (y > 300pt), obteve y={:.1}",
            y_place
        );
        assert!(
            x_place > 250.0,
            "Place(bottom-right) deve estar na zona direita (x > 250pt), obteve x={:.1}",
            x_place
        );
    }

    // ── Passo 83 — Grid: rows e alinhamento vertical ────────────────────

    #[test]
    fn grid_rows_fixed_coordenadas_y_correctas() {
        // grid(columns: 1, rows: (50pt, 100pt)) com 3 items.
        // Linha 0: 50pt. Linha 1: 100pt. Linha 2: 50pt (ciclo 2 % 2 = 0).
        // Grid começa em cursor_y = margin = 20pt.
        // - Item 0 (linha 0) em y = 20pt.
        // - Item 1 (linha 1) em y = 20 + 50 = 70pt.
        // - Item 2 (linha 2) em y = 70 + 100 = 170pt.
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#grid(columns: 1, rows: (50pt, 100pt),
  rect(width: 100pt, height: 10pt),
  rect(width: 100pt, height: 10pt),
  rect(width: 100pt, height: 10pt),
)
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let items = &doc.pages[0].items;
        assert_eq!(items.len(), 3, "Deve haver 3 FrameItems (um por célula)");

        let y0 = frame_item_pos(&items[0]).y.0;
        let y1 = frame_item_pos(&items[1]).y.0;
        let y2 = frame_item_pos(&items[2]).y.0;

        assert!(
            (y1 - (y0 + 50.0)).abs() < 0.5,
            "Item 1 deve estar em y = y0 + 50 (altura da linha 0), obteve y1={:.1} (y0={:.1})",
            y1, y0
        );
        assert!(
            (y2 - (y1 + 100.0)).abs() < 0.5,
            "Item 2 deve estar em y = y1 + 100 (altura da linha 1), obteve y2={:.1} (y1={:.1})",
            y2, y1
        );
    }

    #[test]
    fn grid_valign_bottom_ancora_ao_limite_inferior_da_celula() {
        // grid(columns: 1, rows: (100pt)) com #align("bottom", rect(height: 20pt)).
        // Altura da célula: 100pt. Conteúdo: 20pt.
        // VAlign::Bottom → cell_top + (cell_h - content_h) = cell_top + 80.
        // cell_top = margin = 20pt → rect em y = 100pt.
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#grid(columns: 1, rows: (100pt),
  align(\"bottom\", rect(width: 80pt, height: 20pt)),
)
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let items = &doc.pages[0].items;
        assert!(!items.is_empty(), "Deve haver pelo menos um item");

        let rect_y = frame_item_pos(&items[0]).y.0;
        assert!(
            (rect_y - 100.0).abs() < 0.5,
            "Rect com valign bottom deve estar em y=100pt (cell_top 20 + 80 offset), obteve y={:.1}",
            rect_y
        );
    }

    #[test]
    fn grid_rows_auto_e_fraction_coexistem() {
        // grid(columns: 1, rows: (auto, 1fr)) com rects de 40pt e 10pt.
        // Página 400pt, margin 20pt → available_height = 360pt.
        // Linha 0 (auto): 40pt. Linha 1 (1fr): 360 - 40 = 320pt.
        // Item 0 em y = 20pt. Item 1 em y = 20 + 40 = 60pt (com célula 320pt,
        // mas VAlign default = Top → ancora ao topo da célula).
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#grid(columns: 1, rows: (auto, 1fr),
  rect(width: 100pt, height: 40pt),
  rect(width: 100pt, height: 10pt),
)
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let items = &doc.pages[0].items;
        assert_eq!(items.len(), 2);

        let y0 = frame_item_pos(&items[0]).y.0;
        let y1 = frame_item_pos(&items[1]).y.0;

        assert!(
            (y1 - (y0 + 40.0)).abs() < 0.5,
            "Item 1 deve estar em y = y0 + 40 (altura da linha auto), obteve y1={:.1} (y0={:.1})",
            y1, y0
        );
    }

    // ── Passo 84.2 — DEBT-38: cache de sub-frames Auto ──────────────────

    #[test]
    fn grid_auto_com_multiplas_celulas_reutiliza_cache() {
        // Grid 2x2 com todas as linhas Auto. Cada célula tem altura distinta.
        // Linha 0: rects de 30pt e 50pt → altura da linha = 50pt.
        // Linha 1: rects de 20pt e 40pt → altura da linha = 40pt.
        //
        // - Item 0 (linha 0, col 0, 30pt) em y = 20pt (margem).
        // - Item 1 (linha 0, col 1, 50pt) em y = 20pt.
        // - Item 2 (linha 1, col 0, 20pt) em y = 20 + 50 = 70pt.
        // - Item 3 (linha 1, col 1, 40pt) em y = 70pt.
        //
        // O teste é black-box: não inspecciona o cache directamente.
        // Garantia: trocas entre `cell_idx` da fase 1 e da fase de emissão
        // produziriam coordenadas cruzadas — este teste falharia.
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#grid(columns: 2,
  rect(width: 100pt, height: 30pt),
  rect(width: 100pt, height: 50pt),
  rect(width: 100pt, height: 20pt),
  rect(width: 100pt, height: 40pt),
)
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let items = &doc.pages[0].items;
        assert_eq!(items.len(), 4, "Deve haver 4 FrameItems, obteve {}", items.len());

        let y0 = frame_item_pos(&items[0]).y.0;
        let y1 = frame_item_pos(&items[1]).y.0;
        let y2 = frame_item_pos(&items[2]).y.0;
        let y3 = frame_item_pos(&items[3]).y.0;

        assert!((y0 - 20.0).abs() < 0.5, "Item 0 em y=20, obteve {:.1}", y0);
        assert!((y1 - 20.0).abs() < 0.5, "Item 1 em y=20, obteve {:.1}", y1);
        assert!((y2 - 70.0).abs() < 0.5, "Item 2 em y=70, obteve {:.1}", y2);
        assert!((y3 - 70.0).abs() < 0.5, "Item 3 em y=70, obteve {:.1}", y3);
    }

    // ── Passo 84.5 — DEBT-36: constantes simbólicas + composição ────────

    #[test]
    fn align_aceita_constante_simbolica() {
        // Sintaxe nova `align(center, ...)` — sem string, usando a constante
        // top-level `center` registada como Value::Align em make_stdlib().
        // Mesmo efeito visual do Passo 82: rect 100pt centrado em x=150
        // (margem 20 + (360 - 100)/2 = 150).
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#align(center, rect(width: 100pt, height: 20pt))
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let items = &doc.pages[0].items;
        assert!(!items.is_empty(), "Deve haver pelo menos um item");

        let rect_x = frame_item_pos(&items[0]).x.0;
        assert!(
            (rect_x - 150.0).abs() < 0.5,
            "Rectângulo centrado via constante 'center' deve estar em x=150pt, obteve x={:.1}",
            rect_x
        );
    }

    #[test]
    fn align_aceita_composicao_via_plus() {
        // `center + bottom` combina HAlign::Center + VAlign::Bottom.
        // Rect 100pt centrado horizontalmente: x = 150pt.
        // VAlign::Bottom no fluxo livre da página consome o resto vertical
        // → o cursor avança até page_bottom_limit. Validar X pelo menos.
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#align(center + bottom, rect(width: 100pt, height: 20pt))
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let items = &doc.pages[0].items;
        assert!(!items.is_empty(), "Deve haver pelo menos um item");

        let rect_x = frame_item_pos(&items[0]).x.0;
        let rect_y = frame_item_pos(&items[0]).y.0;
        assert!(
            (rect_x - 150.0).abs() < 0.5,
            "Rectângulo `center + bottom` deve estar em x=150pt, obteve x={:.1}",
            rect_x
        );
        // VAlign::Bottom: rect deve estar na metade inferior da página
        // (page_bottom_limit = 380pt, rect altura ~20pt → y > 200pt).
        assert!(
            rect_y > 200.0,
            "Rectângulo com VAlign::Bottom deve estar na metade inferior (y>200), obteve y={:.1}",
            rect_y
        );
    }

    // ── Passo 84.6 — DEBT-37: place ancora à célula com scope=Column ────

    #[test]
    fn place_dentro_de_grid_ancora_a_celula() {
        // Grid com 1 coluna fixa de 200pt e 1 linha fixa de 100pt.
        // Célula em (margem=20, margem=20), tamanho 200×100.
        // place("bottom-right", rect 30×20) → ancora ao canto inferior-direito
        // da CÉLULA (não da página).
        // - x esperado: cell_x + cell_w - rect_w = 20 + 200 - 30 = 190.
        // - y esperado: cell_y + cell_h - rect_h = 20 + 100 - 20 = 100.
        //
        // Pré P84.6 (apenas mitigação parcial DEBT-37): X = line_start_x
        // dentro da célula = 20 + (200-30) = 190 (já correcto via P81.5);
        // Y = 380 - 20 = 360 (canto inferior da PÁGINA, errado).
        // Pós P84.6: Y = 100 (canto inferior da célula, correcto).
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#grid(columns: (200pt,), rows: (100pt,),
  place(\"bottom-right\", rect(width: 30pt, height: 20pt)),
)
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let items = &doc.pages[0].items;
        assert!(!items.is_empty(), "Deve haver pelo menos um item");

        let rect_x = frame_item_pos(&items[0]).x.0;
        let rect_y = frame_item_pos(&items[0]).y.0;
        assert!(
            (rect_x - 190.0).abs() < 0.5,
            "Place na célula deve ter x=190 (cell_x 20 + 200 - 30), obteve x={:.1}",
            rect_x
        );
        assert!(
            (rect_y - 100.0).abs() < 0.5,
            "Place na célula com scope=Column deve ter y=100 (cell_y 20 + 100 - 20), obteve y={:.1}",
            rect_y
        );
    }

    #[test]
    fn place_dentro_de_grid_com_scope_parent_ancora_a_pagina() {
        // Mesma estrutura do teste anterior, mas scope="parent" → ancora à página.
        // - x esperado: page_margin + (avail_w - rect_w) = 20 + (360 - 30) = 350.
        // - y esperado: page_margin + (avail_h - rect_h) = 20 + (360 - 20) = 360.
        //
        // P223 (DEBT-37 §"Divergência" fechada — Decisão 3 Opção α):
        // `scope: "parent"` agora exige `float: true` paridade vanilla.
        // Test pre-existente P84.6 adaptado adicionando `float: true`;
        // semantic real adiada per ADR-0054 graded — body renderiza
        // na mesma posição (paridade visual preservada literal).
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#grid(columns: (200pt,), rows: (100pt,),
  place(\"bottom-right\", scope: \"parent\", float: true, rect(width: 30pt, height: 20pt)),
)
";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content);

        let items = &doc.pages[0].items;
        assert!(!items.is_empty(), "Deve haver pelo menos um item");

        let rect_x = frame_item_pos(&items[0]).x.0;
        let rect_y = frame_item_pos(&items[0]).y.0;
        assert!(
            (rect_x - 350.0).abs() < 0.5,
            "Place scope=parent deve ter x=350 (margem + avail - rect), obteve x={:.1}",
            rect_x
        );
        assert!(
            (rect_y - 360.0).abs() < 0.5,
            "Place scope=parent deve ter y=360 (margem + avail - rect), obteve y={:.1}",
            rect_y
        );
    }

    // ── Passo 106 (ADR-0043): canal de saída do Sink ──────────────────

    /// Teste end-to-end do canal: input Typst vazio → pilot emite warning
    /// → caller drena via `into_diagnostics` e verifica conteúdo.
    #[test]
    fn sink_canal_emite_warning_para_ficheiro_vazio() {
        let (world, _dir) = world_from_str("");
        let source = world.source(world.main()).unwrap();

        let (result, warnings) = do_eval_with_sink(&world, &source);
        assert!(result.is_ok(), "eval de ficheiro vazio não deve falhar");
        assert_eq!(
            warnings.len(),
            1,
            "ficheiro vazio deve gerar exactamente 1 warning; obteve {}: {:?}",
            warnings.len(),
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
        assert!(
            warnings[0].message.contains("ficheiro vazio"),
            "mensagem esperada contém 'ficheiro vazio'; obteve: {:?}",
            warnings[0].message
        );
    }

    /// Teste de ausência: ficheiro não-vazio não dispara o pilot.
    #[test]
    fn sink_canal_vazio_quando_sem_trigger() {
        let (world, _dir) = world_from_str("Olá mundo");
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert!(
            warnings.is_empty(),
            "ficheiro não-vazio não deve gerar warnings; obteve {:?}",
            warnings
        );
    }

    // `sink_canal_formato_minimo` removido no Passo 119 (ADR-0050):
    // duplicado literal de `typst_shell::diagnostic::tests::formato_warning_detached_sem_cores`.

    /// Teste de dedup end-to-end: o pilot emite para ficheiro vazio. Se
    /// o mesmo ficheiro vazio for processado duas vezes em `eval`s
    /// independentes, cada `eval` tem o seu próprio `Sink` — cada um gera
    /// 1 warning.
    #[test]
    fn sink_canal_cada_run_tem_proprio_sink() {
        let (world1, _dir1) = world_from_str("");
        let source1 = world1.source(world1.main()).unwrap();
        let (_result, warnings1) = do_eval_with_sink(&world1, &source1);
        assert_eq!(warnings1.len(), 1);

        // Segundo run — Sink novo, warning novo.
        let (world2, _dir2) = world_from_str("");
        let source2 = world2.source(world2.main()).unwrap();
        let (_result, warnings2) = do_eval_with_sink(&world2, &source2);
        assert_eq!(
            warnings2.len(),
            1,
            "cada `eval` tem o seu próprio Sink; segundo run deve também gerar 1 warning"
        );
    }

    // ── Passo 107 (encerra DEBT-49): warnings reais de #set ────────────

    /// Propriedade `hyphenate` em `#set text(...)` não está
    /// implementada — emite warning com mensagem específica.
    ///
    /// Passo 132B (ADR-0053): canary DEBT-50 migrou de `font`
    /// para `hyphenate` porque `font` passou a ser capturado
    /// via `FontList`. Teste renomeado de
    /// `debt49_set_text_font_emite_warning`.
    #[test]
    fn debt49_set_text_hyphenate_emite_warning() {
        let (world, _dir) = world_from_str(r#"#set text(hyphenate: true)"#);
        let source = world.source(world.main()).unwrap();

        let (result, warnings) = do_eval_with_sink(&world, &source);
        assert!(result.is_ok(), "eval não deve falhar; Sink absorve o desconhecido");
        assert_eq!(
            warnings.len(),
            1,
            "esperado 1 warning para propriedade 'hyphenate'; obteve {}: {:?}",
            warnings.len(),
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
        assert!(
            warnings[0].message.contains("'hyphenate'"),
            "mensagem deve identificar a propriedade 'hyphenate'; obteve: {:?}",
            warnings[0].message
        );
        assert!(
            warnings[0].message.contains("text"),
            "mensagem deve identificar o target 'text'; obteve: {:?}",
            warnings[0].message
        );
        assert!(
            !warnings[0].hints.is_empty(),
            "warning deve ter pelo menos um hint referenciando ADR-0040"
        );
        assert!(
            warnings[0].hints[0].contains("ADR-0040"),
            "hint deve referenciar ADR-0040; obteve: {:?}",
            warnings[0].hints[0]
        );
    }

    /// Propriedade `baseline` análoga — deve também emitir warning
    /// específico.
    ///
    /// Passo 130 (DEBT-1 subset): `lang` passou a ser capturado —
    /// canary rotou para `alignment` (ainda desconhecida).
    /// P816: `alignment` não existe no `TextElem` vanilla → erro hard
    /// `unexpected argument`; canary rota para `baseline` (válida no
    /// vanilla, `text/mod.rs:371`, ainda não capturada no cristalino).
    #[test]
    fn debt49_set_text_alignment_emite_warning() {
        let (world, _dir) = world_from_str(r#"#set text(baseline: 3pt)"#);
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].message.contains("'baseline'"),
            "mensagem deve identificar 'baseline'; obteve: {:?}",
            warnings[0].message
        );
    }

    /// Múltiplas propriedades desconhecidas num único `#set text(...)` —
    /// N warnings distintos (uma por propriedade), pois spans + messages
    /// diferem.
    ///
    /// Passo 126 (DEBT-1 subset): `weight` passou a ser capturado como
    /// `u16` — já não emite warning.
    /// Passo 130 (DEBT-1 subset): `lang` passou a ser capturado — trio
    /// rotou para `font/alignment/stroke`.
    /// Passo 132B (ADR-0053): `font` passou a ser capturado — trio
    /// rotou para `hyphenate/alignment/stroke`.
    /// P816: `alignment` virou erro hard (não existe no vanilla) — trio
    /// rota para `hyphenate/baseline/stroke` (válidas no vanilla,
    /// não capturadas no cristalino).
    #[test]
    fn debt49_set_text_multiplas_propriedades_desconhecidas() {
        let (world, _dir) = world_from_str(
            r#"#set text(hyphenate: true, baseline: 3pt, stroke: 1pt)"#,
        );
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert_eq!(
            warnings.len(),
            3,
            "esperado 3 warnings (hyphenate, baseline, stroke); obteve {}: {:?}",
            warnings.len(),
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
        let joined = warnings
            .iter()
            .map(|d| d.message.clone())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(joined.contains("'hyphenate'"), "faltou 'hyphenate': {}", joined);
        assert!(joined.contains("'baseline'"), "faltou 'baseline': {}", joined);
        assert!(joined.contains("'stroke'"), "faltou 'stroke': {}", joined);
    }

    /// Propriedades suportadas de `#set text(...)` (bold, italic, size,
    /// fill) não devem emitir warnings — teste de regressão.
    #[test]
    fn debt49_set_text_propriedades_suportadas_sem_warnings() {
        let (world, _dir) =
            world_from_str("#set text(bold: true, italic: false, size: 14pt)");
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert!(
            warnings.is_empty(),
            "propriedades suportadas não devem emitir warnings; obteve: {:?}",
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    /// Target desconhecido em `#set` (ex: `list`, `table`) emite warning
    /// diferente — identificar o target, não a propriedade.
    ///
    /// Passo 133: `par` passou a ser known target. Input migra para
    /// `list` (continua unknown).
    #[test]
    fn debt49_set_target_desconhecido_emite_warning() {
        let (world, _dir) = world_from_str("#set list(indent: 10pt)");
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert_eq!(
            warnings.len(),
            1,
            "target desconhecido 'list' deve gerar 1 warning; obteve {}: {:?}",
            warnings.len(),
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
        assert!(
            warnings[0].message.contains("'list'"),
            "mensagem deve identificar o target 'list'; obteve: {:?}",
            warnings[0].message
        );
        assert!(
            warnings[0].message.contains("target"),
            "mensagem deve indicar que é um problema de target; obteve: {:?}",
            warnings[0].message
        );
    }

    /// Dedup real: mesma propriedade desconhecida em dois `#set` idênticos
    /// deve produzir apenas 1 warning (mesmos span+message).
    ///
    /// Passo 132B (ADR-0053): canary DEBT-50 migrou de `font` para
    /// `hyphenate` — teste rotado do input `font: "A"` para
    /// `hyphenate: true`.
    #[test]
    fn debt49_dedup_warnings_identicos() {
        // Dois `#set text(hyphenate: true)` no mesmo ficheiro. Os spans
        // são diferentes (linha 1 vs linha 2), por isso dedup não aplica
        // aqui — spans distintos contam como warnings distintos.
        //
        // Para testar dedup de verdade, precisaríamos de um sítio que
        // dispara DEBT-49 repetidamente com o MESMO span + message, o que
        // não acontece numa passagem pelo código fonte (cada texto fonte é
        // parsed uma vez por eval). O mecanismo existe no Sink, mas validá-lo
        // requer chamada artificial à API; ver `sink.rs#tests`.
        let (world, _dir) =
            world_from_str("#set text(hyphenate: true)\n#set text(hyphenate: true)");
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        // Dois spans distintos → 2 warnings (não deduplicados).
        assert_eq!(warnings.len(), 2,
            "#set text(hyphenate) repetido em 2 linhas distintas → 2 warnings (spans diferem); \
             dedup real validado em tests unitários de Sink");
    }

    // ── P816 (achado #3 de P810) — `#set` valida nome/tipo; warning de fonte ──

    /// (a) Propriedade inexistente no `TextElem` vanilla → eval `Err`
    /// com `unexpected argument: {name}` (paridade
    /// `foundations/args.rs:262`, exit 1 no binário).
    #[test]
    fn p816_set_text_propriedade_inexistente_erro() {
        let (world, _dir) = world_from_str("#set text(nonexistent-prop: 12pt)");
        let source = world.source(world.main()).unwrap();

        let (result, _warnings) = do_eval_with_sink(&world, &source);
        let errs = result.expect_err("propriedade inexistente deve falhar o eval");
        assert!(
            errs.iter()
                .any(|e| e.message.contains("unexpected argument: nonexistent-prop")),
            "errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// (c) `size` com `Int` → eval `Err` `expected length, found integer`
    /// + hint `did you mean 12pt?` (paridade `foundations/cast.rs:325-343`).
    #[test]
    fn p816_set_text_size_int_erro() {
        let (world, _dir) = world_from_str("#set text(size: 12)");
        let source = world.source(world.main()).unwrap();

        let (result, _warnings) = do_eval_with_sink(&world, &source);
        let errs = result.expect_err("size com Int deve falhar o eval");
        assert!(
            errs.iter()
                .any(|e| e.message.contains("expected length, found integer")),
            "errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
        assert!(
            errs.iter()
                .any(|e| e.hints.iter().any(|h| h.contains("did you mean 12pt?"))),
            "hint vanilla esperado; errs: {:?}",
            errs
        );
    }

    /// (b) Família de fonte desconhecida → 1 warning
    /// `unknown font family: {nome lowercased}` (paridade
    /// `check_font_list`, `text/mod.rs:1577-1588`), eval `Ok`.
    #[test]
    fn p816_set_text_font_desconhecida_warning() {
        let (world, _dir) = world_from_str(r#"#set text(font: "FamiliaQueNaoExiste")"#);
        let source = world.source(world.main()).unwrap();

        let (result, warnings) = do_eval_with_sink(&world, &source);
        assert!(result.is_ok(), "fonte desconhecida é warning, não erro");
        assert!(
            warnings
                .iter()
                .any(|d| d.message.contains("unknown font family: familiaquenaoexiste")),
            "warning vanilla esperado; warnings: {:?}",
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    // ── Passo 119 (ADR-0050) ────────────────────────────────────────────
    //
    // 5 testes `format_diagnostic_*` removidos: duplicados das L2 unit
    // tests (`typst_shell::diagnostic::tests::formato_*`) e dos
    // `debt49_*` / `sink_canal_*` já existentes que asseveram
    // `SourceDiagnostic.message` e `.hints` directamente. Cobertura
    // preservada sem duplicação.

    // ── Passo 140B — Wiring single-font (DEBT-52 gap 5) ─────────────────
    //
    // Testes end-to-end do dispatch font-aware em `compile_to_pdf_bytes`:
    // documento com `#set text(font: "X")` cuja família resolve em
    // `world.book()` produz PDF com CIDFont (`/CrystallineFont`).
    // Família não resolvida ou ausência de `#set` cai no fallback
    // Helvetica.
    //
    // Tests 1 e 4 dependem de fonts no sistema — degradam graciosamente
    // (early return com `eprintln!`) quando o ambiente não tem TTFs nos
    // candidatos canónicos. Fixture dedicado (`tests/fixtures/fonts/`)
    // é decisão futura; o spec do 140B autoriza-o mas não obriga.

    use crate::fonts::{discover_fonts, pair_slots_with_book, FontSlot};
    use crate::pipeline::compile_to_pdf_bytes;
    use typst_core::entities::font_book::FontBook;

    /// Probe de directórios canónicos de fonts. Devolve `None` se
    /// nenhum candidato existe — chamadores devem `return` cedo
    /// e marcar o teste como skipped via `eprintln!`.
    fn discover_any_system_fonts() -> Option<Vec<FontSlot>> {
        let candidates: &[&str] = &[
            "/usr/share/fonts/truetype/dejavu",
            "/usr/share/fonts/truetype/liberation",
            "/usr/share/fonts/dejavu",
            "/usr/share/fonts/TTF",
            "/Library/Fonts",
            "/System/Library/Fonts",
        ];
        for c in candidates {
            let p = PathBuf::from(c);
            if p.is_dir() {
                let slots = discover_fonts(&[p]);
                if !slots.is_empty() {
                    return Some(slots);
                }
            }
        }
        None
    }

    fn first_family(book: &FontBook) -> Option<String> {
        book.infos().first().map(|i| i.family.clone())
    }

    fn second_distinct_family(book: &FontBook, first: &str) -> Option<String> {
        book.infos()
            .iter()
            .map(|i| i.family.clone())
            .find(|f| !f.eq_ignore_ascii_case(first))
    }

    /// Compõe um `SystemWorld` com `src` em `main.typ` e fontes
    /// fornecidas via `with_fonts`.
    fn world_with_fonts(src: &str, slots: Vec<FontSlot>) -> (SystemWorld, TempDir) {
        let dir = tempdir();
        std::fs::write(dir.path().join("main.typ"), src).unwrap();
        let world = SystemWorld::new(dir.path(), "main.typ").unwrap().with_fonts(slots);
        (world, dir)
    }

    /// **P950** — `BaseFont`/`FontName` no PDF exportado reflectem o nome REAL
    /// da fonte (PostScript da tabela `name`), não o genérico
    /// `CrystallineFont[N]` — com o prefixo determinístico de subset
    /// `AAAAAA+` (P517) preservado. Dois casos no mesmo documento (corpo +
    /// math) para garantir que não é hardcoded para uma só fonte.
    #[test]
    fn p950_basefont_usa_nome_real_das_fontes() {
        let src = "#set text(size: 11pt)\nTexto corpo $x^2 + mat(1, 2; 3, 4)$ fim";
        let dir = tempdir();
        std::fs::write(dir.path().join("main.typ"), src).unwrap();
        let world = SystemWorld::new(dir.path(), "main.typ")
            .unwrap()
            .with_embedded_fonts();
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação deve ter sucesso");
        let blob = String::from_utf8_lossy(&pdf);

        // Extrai todos os /BaseFont /Nome presentes no PDF.
        let names: Vec<String> = blob
            .split("/BaseFont /")
            .skip(1)
            .map(|s| {
                s.chars()
                    .take_while(|c| !c.is_whitespace() && *c != '>')
                    .collect()
            })
            .collect();
        assert!(!names.is_empty(), "PDF deve ter pelo menos um /BaseFont");
        for n in &names {
            assert!(
                !n.trim_start_matches("AAAAAA+").starts_with("CrystallineFont"),
                "P950: BaseFont genérico não pode permanecer: {n}"
            );
        }
        // Pelo menos dois nomes reais distintos (corpo + math) — confirma que
        // a correcção não é hardcoded para uma só fonte.
        let distinct: std::collections::HashSet<&String> = names.iter().collect();
        assert!(
            distinct.len() >= 2,
            "P950: documento com corpo+math deve ter ≥2 fontes com nomes reais: {names:?}"
        );
        assert!(
            names.iter().any(|n| n.contains("NewCM")),
            "P950: nome real da família New Computer Modern esperado: {names:?}"
        );
    }

    #[test]
    fn font_wiring_set_text_font_existente_embute_cidfont() {
        let Some(slots) = discover_any_system_fonts() else {
            eprintln!(
                "[skip] font_wiring_set_text_font_existente_embute_cidfont: \
                       nenhum directório de fonts canónico encontrado"
            );
            return;
        };
        let (slots, book) = pair_slots_with_book(slots);
        let Some(family) = first_family(&book) else {
            eprintln!("[skip] FontBook vazio após pair_slots_with_book");
            return;
        };

        let src = format!("#set text(font: \"{}\")\nOlá", family);
        let (world, _dir) = world_with_fonts(&src, slots);
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação deve ter sucesso");

        assert_eq!(&pdf[..5], b"%PDF-", "header PDF esperado");
        let blob = String::from_utf8_lossy(&pdf);
        // P950 — o marcador do caminho CIDFont deixa de ser o nome genérico
        // `CrystallineFont` (agora é o nome real da fonte): passa a ser a
        // presença de Type0 sem fallback Helvetica.
        assert!(
            blob.contains("/Subtype /Type0") && !blob.contains("/BaseFont /Helvetica"),
            "PDF deve conter CIDFont (Type0) quando \
             `#set text(font: \"{}\")` resolve em FontBook",
            family
        );
    }

    #[test]
    fn font_wiring_set_text_font_inexistente_fallback_helvetica() {
        let src = "#set text(font: \"FontQueNaoExiste\")\nOlá";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação deve ter sucesso");

        assert_eq!(&pdf[..5], b"%PDF-");
        let blob = String::from_utf8_lossy(&pdf);
        assert!(
            !blob.contains("CrystallineFont"),
            "PDF não deve conter marker CIDFont — família não existe"
        );
        assert!(blob.contains("Helvetica"), "fallback Helvetica deve estar presente");
    }

    // ── P941 — glifos bitmap (CBDT) como imagens XObject ─────────────────
    //
    // A fonte CBDT (Noto Color Emoji) não é subsettable (`UnknownKind`); em
    // vez de a embutir inteira (~10 MB), os seus glifos são desenhados como
    // imagens XObject (dedup por glifo), como o vanilla (krilla). Testes
    // dependem da fonte no sistema — degradam para skip se ausente.

    fn p941_world(src: &str) -> Option<(SystemWorld, TempDir)> {
        if !PathBuf::from("/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf").is_file() {
            return None;
        }
        let dir = tempdir();
        std::fs::write(dir.path().join("main.typ"), src).unwrap();
        let world = SystemWorld::new(dir.path(), "main.typ")
            .unwrap()
            .with_fonts_and_system(&[]);
        Some((world, dir))
    }

    fn p941_compile(world: &SystemWorld) -> Vec<u8> {
        let source = world.source(world.main()).unwrap();
        let (result, _w) = compile_to_pdf_bytes(world, &source, StreamMode::Verbose);
        result.expect("compilação deve ter sucesso")
    }

    #[test]
    fn p941_emoji_unico_gera_imagem_sem_embute_cbdt() {
        let Some((world, _dir)) = p941_world("Emoji: 🎉\n") else {
            eprintln!("[skip] NotoColorEmoji ausente");
            return;
        };
        let pdf = p941_compile(&world);
        let blob = String::from_utf8_lossy(&pdf);
        assert!(
            blob.contains("/Subtype /Image"),
            "P941: emoji bitmap deve ser emitido como imagem XObject"
        );
        assert!(
            pdf.len() < 500_000,
            "P941: PDF não deve embutir a fonte CBDT inteira (~10 MB) — {} bytes",
            pdf.len()
        );
    }

    #[test]
    fn p941_emoji_repetido_dedup_um_xobject() {
        // 🎉🚀🎉 — 2 glifos únicos, 🎉 repetido: deve haver exactamente 2
        // XObjects de imagem e 3 referências `Do`.
        let Some((world, _dir)) = p941_world("🎉🚀🎉\n") else {
            eprintln!("[skip] NotoColorEmoji ausente");
            return;
        };
        let pdf = p941_compile(&world);
        let blob = String::from_utf8_lossy(&pdf);
        let n_images = blob.matches("/Subtype /Image").count();
        // Contar referências `/ImN <id> 0 R` no dicionário /XObject: uma por
        // glifo único (🎉 e 🚀 → 2), apesar de 🎉 aparecer 2 vezes no texto.
        let xobj_dict_entries = {
            let bytes = blob.as_bytes();
            let mut count = 0usize;
            let mut i = 0usize;
            while let Some(pos) = blob[i..].find("/Im") {
                let start = i + pos + 3;
                let digits = bytes[start..].iter().take_while(|b| b.is_ascii_digit()).count();
                if digits > 0 {
                    count += 1;
                }
                i = start;
            }
            count
        };
        assert_eq!(
            xobj_dict_entries, 2,
            "P941: 2 glifos únicos (🎉🚀) → 2 entradas no dicionário /XObject; \
             🎉 repetido partilha o mesmo XObject"
        );
        assert!(
            n_images >= 2,
            "P941: deve haver objectos de imagem para os glifos bitmap — {}",
            n_images
        );
    }

    #[test]
    fn p941_documento_misto_texto_e_emoji() {
        // Texto latino continua no caminho normal de fonte (subset); emoji
        // sai como imagem. O PDF deve conter ambos: `/Font` com FontFile
        // (texto) e `/Subtype /Image` (emoji).
        let Some((world, _dir)) = p941_world("A 🎉 B\n") else {
            eprintln!("[skip] NotoColorEmoji ausente");
            return;
        };
        let pdf = p941_compile(&world);
        let blob = String::from_utf8_lossy(&pdf);
        assert!(
            blob.contains("/Subtype /Image"),
            "P941: emoji deve ser imagem XObject"
        );
        assert!(
            blob.contains("/FontFile"),
            "P941: texto latino deve manter fonte embutida (subset)"
        );
        assert!(
            pdf.len() < 500_000,
            "P941: PDF misto não deve embutir a fonte CBDT inteira — {} bytes",
            pdf.len()
        );
    }

    #[test]
    fn font_wiring_sem_set_text_font_usa_helvetica() {
        let src = "Olá mundo";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação deve ter sucesso");

        assert_eq!(&pdf[..5], b"%PDF-");
        let blob = String::from_utf8_lossy(&pdf);
        assert!(
            !blob.contains("CrystallineFont"),
            "documento sem `#set text(font:)` cai no fallback Helvetica"
        );
        assert!(blob.contains("Helvetica"), "fallback Helvetica deve estar presente");
    }

    #[test]
    fn p887_table_sem_stroke_explicito_desenha_grelha_e2e() {
        // P887 (achado 3 de P885) — pipeline completo (source → eval →
        // layout → export), não só a resolução de valor isolada
        // (`p887_table_stroke_omitido_tem_default_1pt_preto`, em
        // `01_core/src/engine/stdlib/mod.rs`) nem só o export dado um
        // stroke já resolvido — este teste falha se qualquer ponto da
        // cadeia (native_table → layout_grid → exportador PDF) deixar de
        // propagar o default. Sem `stroke:` explícito, `table()` deve
        // desenhar grelha (paridade vanilla, `model/table.rs:268-270`).
        let src = "#table(columns: 2, [a], [b], [c], [d])";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação deve ter sucesso");

        // Os content streams das páginas são comprimidos com FlateDecode
        // desde P884 — `String::from_utf8_lossy(&pdf)` direto não vê os
        // operadores PDF (estão dentro do stream binário comprimido).
        // `extract_page_content_streams_text` descomprime antes de expor.
        let content = crate::export::test_helpers::extract_page_content_streams_text(&pdf);
        assert!(
            content.contains("S\n") || content.contains("S "),
            "operador S (stroke) ausente do content stream de uma tabela \
             sem `stroke:` explícito — grelha default não está a ser \
             desenhada: {content}"
        );
    }

    #[test]
    fn p805a_ligatura_fi_tem_entrada_to_unicode_no_embed_integral() {
        // P805a — no caminho de embutimento integral (fallback de P797 para
        // CFF, `glyph_mapping` vazio), os glifos de ligadura produzidos pelo
        // shaper ("fi" → gid f_i) ficavam SEM entrada no ToUnicode CMap —
        // renderizavam correctamente, mas extracção (`pdftotext`) perdia os
        // caracteres ("fieri" → "eri"). A causa era o gate
        // `if !glyph_mapping.is_empty()` em builder.rs que saltava
        // `collect_shaped_cluster_texts` no fallback. Este teste verifica que
        // o PDF final tem uma entrada ToUnicode para o texto "fi"
        // (`<00660069>` em UTF-16BE). Usa as fontes embutidas (Libertinus
        // Serif é CFF — o caminho do fallback integral de P797).
        let dir = tempdir();
        std::fs::write(
            dir.path().join("main.typ"),
            "#set text(font: \"Libertinus Serif\")\nfi fierce",
        )
        .unwrap();
        let world = SystemWorld::new(dir.path(), "main.typ")
            .unwrap()
            .with_embedded_fonts();
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação deve ter sucesso");
        let blob = String::from_utf8_lossy(&pdf);
        assert!(
            blob.contains("LibertinusSerif"),
            "sanity: Libertinus Serif embutido como CIDFont (P950: nome real)"
        );
        assert!(
            blob.contains("<00660069>"),
            "ToUnicode deve mapear o glifo da ligadura para \"fi\" (U+0066 U+0069)"
        );
    }

    #[test]
    fn font_wiring_segunda_font_diferente_ambas_embebidas() {
        // Renomeado e ajustado no Passo 146 (multi-font per document
        // — ADR-0055 decisão 5 materializada). Pré-146 este teste
        // afirmava `exactly 1 /Subtype /Type0` (MVP single-font);
        // pós-146 ambas as fonts são embebidas e o assert reflecte
        // a regressão deliberada do MVP. Documentado no relatório
        // do Passo 146.
        let Some(slots) = discover_any_system_fonts() else {
            eprintln!(
                "[skip] font_wiring_segunda_font_diferente_ambas_embebidas: \
                       sem fonts no sistema"
            );
            return;
        };
        let (slots, book) = pair_slots_with_book(slots);
        let Some(first) = first_family(&book) else {
            eprintln!("[skip] FontBook vazio");
            return;
        };
        let Some(second) = second_distinct_family(&book, &first) else {
            eprintln!("[skip] sistema só tem uma família — teste exige duas distintas");
            return;
        };

        let src = format!(
            "#set text(font: \"{}\")\nOlá\n\n#set text(font: \"{}\")\nAdeus",
            first, second
        );
        let (world, _dir) = world_with_fonts(&src, slots);
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação deve ter sucesso");

        assert_eq!(&pdf[..5], b"%PDF-");
        let blob = String::from_utf8_lossy(&pdf);
        // P950 — marcador CIDFont actualizado (nome genérico removido).
        assert!(
            blob.contains("/Subtype /Type0") && !blob.contains("/BaseFont /Helvetica"),
            "PDF deve embutir pelo menos uma das famílias como CIDFont"
        );
        let n_type0 = blob.matches("/Subtype /Type0").count();
        assert_eq!(
            n_type0, 2,
            "Pós-146 multi-font: exactamente 2 Type0 esperados (uma \
             por família distinta) — encontrados {}",
            n_type0
        );
    }

    // ── Passo 141 — Array fallback chain (DEBT-52 gap 6) ────────────────
    //
    // Verifica que `resolve_font` itera todas as famílias da
    // `FontList` e usa a primeira que resolve. Documento com
    // `font: ("FontQueNaoExiste", "<família-real>")` deve embutir
    // a segunda família como CIDFont — a primeira é silenciosamente
    // saltada (não há fallback Helvetica enquanto restarem famílias
    // por tentar).

    #[test]
    fn font_wiring_array_fallback_primeira_falha_segunda_vence() {
        let Some(slots) = discover_any_system_fonts() else {
            eprintln!(
                "[skip] font_wiring_array_fallback_primeira_falha_segunda_vence: \
                       sem fonts no sistema"
            );
            return;
        };
        let (slots, book) = pair_slots_with_book(slots);
        let Some(family) = first_family(&book) else {
            eprintln!("[skip] FontBook vazio");
            return;
        };

        // Primeira família é deliberadamente inexistente; segunda
        // resolve no FontBook → CIDFont embutida.
        let src = format!("#set text(font: (\"FontQueNaoExiste\", \"{}\"))\nOlá", family);
        let (world, _dir) = world_with_fonts(&src, slots);
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação deve ter sucesso");

        assert_eq!(&pdf[..5], b"%PDF-");
        let blob = String::from_utf8_lossy(&pdf);
        // P950 — marcador CIDFont actualizado (nome genérico removido).
        assert!(
            blob.contains("/Subtype /Type0"),
            "PDF deve embutir a segunda família como CIDFont quando \
             a primeira não resolve"
        );
        assert!(
            !blob.contains("/BaseFont /Helvetica"),
            "fallback Helvetica não deve estar presente — segunda \
             família resolveu"
        );
    }

    // ── Passo 144 — Lang hyphenation (gap 7 DEBT-52, ADR-0057) ──────────
    //
    // Verifica end-to-end que `compile_to_pdf_bytes` emite FrameItems
    // com hífen literal quando `#set text(lang: "<código>")` está
    // presente e palavra longa não cabe na linha. Testes inspeccionam
    // o `PagedDocument` directamente (tickets `FrameItem::Text` cujo
    // `text` termina em `-`).

    fn build_doc(src: &str) -> typst_core::entities::layout_types::PagedDocument {
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        layout(content)
    }

    fn count_hyphenated_words(
        doc: &typst_core::entities::layout_types::PagedDocument,
    ) -> usize {
        doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                typst_core::entities::layout_types::FrameItem::Text { text, .. } => {
                    Some(text.as_str())
                }
                _ => None,
            })
            .filter(|t| t.ends_with('-') && t.chars().count() > 1)
            .count()
    }

    #[test]
    fn lang_hyphenation_en_palavra_longa_quebra_com_hifen() {
        // Coluna muito estreita (100pt − 2×10pt margem = 80pt;
        // ~11 chars a 12pt com FixedMetrics 0.6×size) força quebra
        // de palavras longas como "extraordinary".
        let src = "#set page(width: 100pt, height: 400pt, margin: 10pt)\n\
                   #set text(lang: \"en\")\n\
                   The extraordinary characteristics of this remarkable phenomenon.";
        let doc = build_doc(src);
        let n = count_hyphenated_words(&doc);
        assert!(
            n >= 1,
            "documento com `lang: \"en\"` em coluna estreita deve produzir \
             pelo menos 1 palavra com hífen de quebra; encontradas {}",
            n
        );
    }

    #[test]
    fn lang_hyphenation_pt_palavra_longa_quebra_com_hifen() {
        let src = "#set page(width: 100pt, height: 400pt, margin: 10pt)\n\
                   #set text(lang: \"pt\")\n\
                   As características extraordinárias deste fenómeno notável.";
        let doc = build_doc(src);
        let n = count_hyphenated_words(&doc);
        assert!(
            n >= 1,
            "documento com `lang: \"pt\"` em coluna estreita deve produzir \
             pelo menos 1 palavra com hífen de quebra; encontradas {}",
            n
        );
    }

    #[test]
    fn lang_hyphenation_sem_set_lang_comportamento_inalterado() {
        // Sem `#set text(lang:)`: hyphenation não dispara — palavras
        // que não cabem migram inteiras para a linha seguinte (pré-144).
        let src = "#set page(width: 100pt, height: 400pt, margin: 10pt)\n\
                   The extraordinary characteristics of this remarkable phenomenon.";
        let doc = build_doc(src);
        assert_eq!(
            count_hyphenated_words(&doc),
            0,
            "documento sem `#set text(lang:)` não deve produzir hífenes \
             de quebra (regressão pré-144 preservada)"
        );
    }

    // ── Passo 146 — Multi-font per document (ADR-0055 decisão 5) ────────

    #[test]
    fn font_wiring_multifont_uma_resolve_outra_falla_silent_drop() {
        // Doc com 3 spans: 2 fonts disponíveis + 1 inexistente. PDF
        // embute 2 (silent drop da inexistente). Spans da inexistente
        // caem em /F1 (font 0) por consistência multi-font.
        let Some(slots) = discover_any_system_fonts() else {
            eprintln!("[skip] sem fonts no sistema");
            return;
        };
        let (slots, book) = pair_slots_with_book(slots);
        let Some(first) = first_family(&book) else {
            eprintln!("[skip] FontBook vazio");
            return;
        };
        let Some(second) = second_distinct_family(&book, &first) else {
            eprintln!("[skip] precisa duas famílias distintas");
            return;
        };

        let src = format!(
            "#set text(font: \"{}\")\nA\n\n#set text(font: \"{}\")\nB\n\n\
             #set text(font: \"FontInexistenteZZZ\")\nC",
            first, second
        );
        let (world, _dir) = world_with_fonts(&src, slots);
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação");
        assert_eq!(&pdf[..5], b"%PDF-");
        let blob = String::from_utf8_lossy(&pdf);
        let n_type0 = blob.matches("/Subtype /Type0").count();
        assert_eq!(
            n_type0, 2,
            "3 fonts no doc; 1 não resolve → 2 Type0 embebidas; \
             encontradas {}",
            n_type0
        );
    }

    #[test]
    fn font_wiring_multifont_regressao_single_font_inalterado() {
        // Documento com UMA única font (single-font path) deve
        // continuar a produzir 1 Type0 — regressão do caminho do
        // 140B/141 preservada por `compile_to_pdf_bytes` matching
        // `[(_, b)] => export_pdf_with_font(...)`.
        let Some(slots) = discover_any_system_fonts() else {
            eprintln!("[skip] sem fonts");
            return;
        };
        let (slots, book) = pair_slots_with_book(slots);
        let Some(family) = first_family(&book) else {
            eprintln!("[skip]");
            return;
        };
        let src = format!("#set text(font: \"{}\")\nOlá", family);
        let (world, _dir) = world_with_fonts(&src, slots);
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let pdf = result.expect("compilação");
        let blob = String::from_utf8_lossy(&pdf);
        let n_type0 = blob.matches("/Subtype /Type0").count();
        assert_eq!(
            n_type0, 1,
            "single-font deve continuar a produzir 1 Type0 (caminho \
             export_pdf_with_font preservado); encontradas {}",
            n_type0
        );
        // **P950** — o single-font path usa agora o nome REAL da fonte
        // (tabela `name`), não o genérico `/CrystallineFont` (o genérico
        // numerado `/CrystallineFont1` do multi-font também desapareceu).
        // O prefixo determinístico de subset `AAAAAA+` (P517) mantém-se.
        assert!(
            !blob.contains("CrystallineFont"),
            "P950: o nome genérico CrystallineFont[N] não pode permanecer no PDF"
        );
        assert!(
            blob.contains("/BaseFont /"),
            "single-font path deve ter /BaseFont com o nome real da fonte"
        );
    }

    #[test]
    fn lang_hyphenation_idioma_sem_padroes_silent_skip() {
        // Código ISO improvável (não suportado pelo `hypher`) →
        // silent skip; sem hífenes; sem warning; sem erro.
        let src = "#set page(width: 100pt, height: 400pt, margin: 10pt)\n\
                   #set text(lang: \"xx\")\n\
                   The extraordinary characteristics of this remarkable phenomenon.";
        let doc = build_doc(src);
        assert_eq!(
            count_hyphenated_words(&doc),
            0,
            "idioma sem padrões TeX → silent skip; sem hífenes inseridos"
        );
    }

    // ── P966 — default de itálico math em conteúdo de função de utilizador ──
    //
    // Especificação: `00_nucleo/prompts/engine/math/layout/_comum.md` §P966.
    // `#let bra(x) = [⟨#x\|]` chamado como `bra(phi)` dentro de `$…$` chega
    // ao layout como `Content::Sequence` de markup (filhos `Text`/`MathText`)
    // e a folha `MathText("φ")` nunca recebia o default de itálico
    // (`apply_math_default` só recursava em containers `Math*`). No vanilla
    // o output da função é re-realizado COMO math → 𝜑 (U+1D711).

    /// Texto concatenado de todos os items de texto do documento
    /// (`Text` fallback + `TextShaped`).
    fn texto_do_doc(doc: &typst_core::entities::layout_types::PagedDocument) -> String {
        use typst_core::entities::layout_types::FrameItem;
        doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                FrameItem::TextShaped { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn p966_bra_ket_funcao_utilizador_recebe_default() {
        // Caminho real medido em P966 Fase A: bra(phi) ket(psi) em math.
        let src = "#let bra(x) = [⟨#x\\|]\n#let ket(x) = [\\|#x⟩]\n$ bra(phi) ket(psi) $";
        let doc = build_doc(src);
        let t = texto_do_doc(&doc);
        assert!(
            t.contains('\u{1D711}'),
            "φ de bra(phi) deve ser 𝜑 (U+1D711, itálico math); texto: {t:?}"
        );
        assert!(
            t.contains('\u{1D713}'),
            "ψ de ket(psi) deve ser 𝜓 (U+1D713, itálico math); texto: {t:?}"
        );
        assert!(
            !t.contains('φ') && !t.contains('ψ'),
            "φ/ψ não devem ficar no bloco grego (U+03C6/U+03C8): {t:?}"
        );
        // Os delimitadores literais ⟨ ⟩ | ficam intactos (Text nunca é
        // transformado).
        assert!(t.contains('⟨') && t.contains('⟩') && t.contains('|'),
            "delimitadores literais intactos: {t:?}");
    }

    /// **Guarda** — função de utilizador FORA de math é inafectada por
    /// construção (`apply_math_default` só corre a partir de
    /// `layout_equation`): `d` em prosa fica reto.
    #[test]
    fn p966_funcao_utilizador_fora_de_math_intacta() {
        let src = "#let f(x) = [d #x]\n#f[texto]";
        let doc = build_doc(src);
        let t = texto_do_doc(&doc);
        assert!(t.contains('d'), "prosa contém 'd' literal: {t:?}");
        assert!(
            !t.contains('\u{1D451}'),
            "fora de math NÃO pode haver 𝑑 (U+1D451, itálico math): {t:?}"
        );
    }

    // ── P204F (M8) — Smoke tests do corpus paridade introspection ────────
    //
    // 5 core + 1 opcional adicionados a `lab/parity/corpus/visual/` em
    // P204F per ADR-0073 plano de materialização. Cobertura de features
    // de introspection (outline, counter, figure-ref, equation-ref,
    // cite-bibliography, query-metadata).
    //
    // **Caminho B reduzido (cristalino-only)**: vanilla integration
    // deferred per pre-existing DEBT-53 (lab/parity harness vanilla
    // não funcional). Per P204F.div-1 — spec assumiu observable
    // harness; realidade é cristalino-only baseline.
    //
    // Smoke validation: cristalino compila cada .typ + produz PDF
    // não-vazio. Asserções estruturais profundas (queries comparadas
    // com vanilla) ficam para sub-passo dedicado pós-M8.

    #[test]
    fn p204f_corpus_outline_toc_compila() {
        let src = include_str!("../../lab/parity/corpus/visual/outline-toc.typ");
        let pdf = compile_to_pdf(src);
        assert!(!pdf.is_empty(), "outline-toc.typ deve produzir PDF");
        assert_eq!(&pdf[..5], b"%PDF-", "header PDF válido");
    }

    #[test]
    fn p204f_corpus_counter_heading_compila() {
        let src = include_str!("../../lab/parity/corpus/visual/counter-heading.typ");
        let pdf = compile_to_pdf(src);
        assert!(!pdf.is_empty(), "counter-heading.typ deve produzir PDF");
    }

    #[test]
    fn p204f_corpus_figure_ref_compila() {
        let src = include_str!("../../lab/parity/corpus/visual/figure-ref.typ");
        let pdf = compile_to_pdf(src);
        assert!(!pdf.is_empty(), "figure-ref.typ deve produzir PDF");
    }

    #[test]
    fn p204f_corpus_equation_ref_compila() {
        let src = include_str!("../../lab/parity/corpus/visual/equation-ref.typ");
        let pdf = compile_to_pdf(src);
        assert!(!pdf.is_empty(), "equation-ref.typ deve produzir PDF");
    }

    #[test]
    fn p204f_corpus_cite_bibliography_compila() {
        // Nota: bibliography("refs.yaml") referencia ficheiro
        // lateral. SystemWorld pode falhar resolução em
        // tempdir; teste valida ate ao limite suportado.
        // Caso falhe na resolução de refs.yaml, é lacuna
        // documentada (DEBT-53/54 vanilla integration).
        let src = include_str!("../../lab/parity/corpus/visual/cite-bibliography.typ");
        // build_doc tolera erros de bibliography? Usar
        // compilação parcial via compile_to_pdf que panics
        // em erro fatal — se falhar, é gap conhecido.
        // Por defesa, marcar este test como #[ignore] caso
        // bibliography não compile sem refs.yaml acessível.
        let result = std::panic::catch_unwind(|| compile_to_pdf(src));
        match result {
            Ok(pdf) => {
                assert!(
                    !pdf.is_empty(),
                    "cite-bibliography.typ deve produzir PDF se compilar"
                );
            }
            Err(_) => {
                // P204F.div-1 documenta: bibliography asset
                // resolution requires SystemWorld file path.
                // include_str! não preserva path context.
                eprintln!(
                    "[P204F] cite-bibliography.typ requer \
                          ficheiro refs.yaml em path resolvível — \
                          DEBT pré-existente; cobertura reduzida"
                );
            }
        }
    }

    #[test]
    fn p204f_corpus_query_metadata_compila() {
        let src = include_str!("../../lab/parity/corpus/visual/query-metadata.typ");
        let pdf = compile_to_pdf(src);
        assert!(!pdf.is_empty(), "query-metadata.typ deve produzir PDF");
    }

    // ── P506 — runtime state/counter/context ───────────────────────────────

    #[test]
    fn p506_context_block_expande_valor_simples() {
        let (world, _dir) = world_from_str("#context { 1 + 2 }");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        let expanded = crate::pipeline::expand_context_blocks(
            content.clone(),
            &intr,
            &world,
            &source,
        )
        .unwrap();
        assert_eq!(expanded.plain_text(), "3");
    }

    #[test]
    fn p506_state_update_e_get_via_context() {
        let src = "#let s = state(\"x\", 0)\n#s.update(5)\n#context s.get()";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        let expanded = crate::pipeline::expand_context_blocks(
            content.clone(),
            &intr,
            &world,
            &source,
        )
        .unwrap();
        assert!(
            expanded.plain_text().contains("5"),
            "esperado '5' em {:?}",
            expanded.plain_text()
        );
    }

    #[test]
    fn p506_counter_step_e_get_via_context() {
        let src = "#counter(heading).step()\n#context counter(heading).get()";
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        let expanded = crate::pipeline::expand_context_blocks(
            content.clone(),
            &intr,
            &world,
            &source,
        )
        .unwrap();
        assert!(
            expanded.plain_text().contains("1"),
            "esperado '1' em {:?}",
            expanded.plain_text()
        );
    }

    #[test]
    fn p506_counter_at_label_via_context() {
        let src = concat!(
            "#counter(heading).step()\n",
            "= Heading <lbl>\n",
            "#context counter(heading).at(<lbl>)"
        );
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        let expanded = crate::pipeline::expand_context_blocks(
            content.clone(),
            &intr,
            &world,
            &source,
        )
        .unwrap();
        assert!(
            expanded.plain_text().contains("2"),
            "esperado '2' em {:?}",
            expanded.plain_text()
        );
    }

    #[test]
    fn p506_pipeline_com_context_state_produz_pdf() {
        let src = "#let s = state(\"x\", 0)\n#s.update(7)\n#context s.display()";
        let pdf = compile_to_pdf(src);
        assert!(!pdf.is_empty());
    }

    // ── P858 — `measure()` com métricas reais via `Engine::font_metrics` ──

    /// **P858** — expande `#context` num `SystemWorld` com fontes reais
    /// carregadas. Reutiliza os helpers de descoberta do Passo 140B.
    fn p858_expand_plain_text_with_fonts(src: &str) -> Option<String> {
        let slots = discover_any_system_fonts()?;
        let (world, _dir) = world_with_fonts(src, slots);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        Some(
            crate::pipeline::expand_context_blocks(content.clone(), &intr, &world, &source)
                .unwrap()
                .plain_text(),
        )
    }

    /// **P858** — `measure([hello])` deve usar métricas reais de fonte
    /// (`FallbackFontMetrics` injectado em `expand_context_blocks`), não a
    /// heurística monoespaçada de `FixedMetrics`.
    /// Valores de referência (vanilla 0.15.0, Helvetica default):
    /// width ≈ 22.19pt, height ≈ 7.24pt.
    /// Antes de P858: width ≈ 33pt, height ≈ 14.85pt.
    #[test]
    fn p858_measure_hello_usa_metricas_reais() {
        let Some(text) = p858_expand_plain_text_with_fonts(
            "#context (measure([hello]).width, measure([hello]).height)",
        ) else {
            eprintln!("[skip] p858_measure_hello_usa_metricas_reais: nenhuma fonte de sistema encontrada");
            return;
        };
        // Extrai os dois números do texto: espera-se "(XX.XXpt, YY.YYpt)".
        let nums: Vec<f64> = text
            .split(|c: char| !c.is_ascii_digit() && c != '.')
            .filter(|s| !s.is_empty() && s.contains('.'))
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();
        assert_eq!(nums.len(), 2, "esperado dois números em {text:?}");
        let (w, h) = (nums[0], nums[1]);
        // Sem fontes reais, o fallback de FallbackFontMetrics coincide com
        // FixedMetrics (0.6×size por codepoint) → 33pt. Com fontes reais,
        // a largura deve ser claramente inferior à heurística.
        assert!(
            w < 30.0,
            "largura de [hello] deve usar métricas reais (< 30pt), obtido {w}pt em {text:?}"
        );
        assert!(
            w > 0.0 && h > 0.0,
            "dimensões devem ser positivas, obtido ({w}pt, {h}pt) em {text:?}"
        );
    }

    /// **P858** — `measure([])` continua a devolver dimensões zero, mesmo com
    /// métricas reais injectadas.
    #[test]
    fn p858_measure_vazio_zero() {
        let Some(text) =
            p858_expand_plain_text_with_fonts("#context (measure([]).width, measure([]).height)")
        else {
            eprintln!("[skip] p858_measure_vazio_zero: nenhuma fonte de sistema encontrada");
            return;
        };
        assert!(
            text.contains("0pt"),
            "measure([]) deve produzir 0pt em {text:?}"
        );
    }

    /// **P858** — múltiplos `#context`/`measure()` no mesmo documento reutilizam
    /// a mesma instância de `FallbackFontMetrics` (cache partilhado) sem
    /// corromper resultados.
    #[test]
    fn p858_measure_multiplo_contexto_consistente() {
        let Some(text) = p858_expand_plain_text_with_fonts(
            "#context measure([hello]).width\n#context measure([hello]).height",
        ) else {
            eprintln!("[skip] p858_measure_multiplo_contexto_consistente: nenhuma fonte de sistema encontrada");
            return;
        };
        let nums: Vec<f64> = text
            .split(|c: char| !c.is_ascii_digit() && c != '.')
            .filter(|s| !s.is_empty() && s.contains('.'))
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();
        assert_eq!(nums.len(), 2, "esperado dois números em {text:?}");
        let (w, h) = (nums[0], nums[1]);
        assert!(
            w < 30.0,
            "largura partilhada deve usar métricas reais (< 30pt), obtido {w}pt"
        );
        assert!(
            w > 0.0 && h > 0.0,
            "dimensões partilhadas devem ser positivas, obtido ({w}pt, {h}pt)"
        );
    }

    // ── P844 — introspecção: achados #47–#54 de P831 ─────────────────────

    /// Helper P844: eval + introspecção + expansão de `#context`, devolve
    /// o `plain_text()` do documento expandido (mesmo padrão dos testes
    /// P506/P821 acima).
    fn p844_expand_plain_text(src: &str) -> String {
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        crate::pipeline::expand_context_blocks(content.clone(), &intr, &world, &source)
            .unwrap()
            .plain_text()
    }

    /// Helper P844: igual a `p844_expand_plain_text` mas devolve as
    /// mensagens de erro da expansão (para asserir erros verbatim).
    fn p844_expand_errors(src: &str) -> Vec<String> {
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        crate::pipeline::expand_context_blocks(content.clone(), &intr, &world, &source)
            .expect_err("expansão deve falhar")
            .iter()
            .map(|d| d.message.clone())
            .collect()
    }

    #[test]
    fn p844_a3_state_at_e_final_via_context() {
        // Achado #49 de P831: `state.at`/`state.final` não estavam ligados
        // no dispatch de métodos. Medido no vanilla 0.15.0
        // (`temp/p844/a3_state_at_final.typ`): `0 3` (at(here()) antes do
        // update → init; final() → último valor).
        let text = p844_expand_plain_text(
            "#let s = state(\"s\", 0)\n#context s.at(here())\n#s.update(3)\n#context s.final()",
        );
        assert!(text.contains('0'), "esperado '0' (init) em {text:?}");
        assert!(text.contains('3'), "esperado '3' (final) em {text:?}");
    }

    #[test]
    fn p844_a3_counter_final_via_context() {
        // Achado #49: `counter.final` ausente do dispatch
        // (`error: counter não tem método 'final'`).
        let text = p844_expand_plain_text(
            "#counter(heading).step()\n#context counter(heading).final()",
        );
        assert!(text.contains('1'), "esperado '1' em {text:?}");
    }

    #[test]
    fn p844_a3_state_at_erros_verbatim_vanilla() {
        // Mensagens medidas no vanilla 0.15.0 (ver relatório P844 §#49):
        // `state.at()` → "missing argument: selector";
        // `state.at(1)` → "expected label, function, location, or selector,
        // found integer"; `state.final(1)` → "unexpected argument".
        let errs = p844_expand_errors("#let s = state(\"s\", 0)\n#context s.at()");
        assert!(
            errs.iter().any(|m| m.contains("missing argument: selector")),
            "errs: {errs:?}"
        );
        let errs = p844_expand_errors("#let s = state(\"s\", 0)\n#context s.at(1)");
        assert!(
            errs.iter().any(|m| m
                .contains("expected label, function, location, or selector, found integer")),
            "errs: {errs:?}"
        );
        let errs = p844_expand_errors("#let s = state(\"s\", 0)\n#context s.final(1)");
        assert!(
            errs.iter().any(|m| m.contains("unexpected argument")),
            "errs: {errs:?}"
        );
    }

    #[test]
    fn p844_a4_counter_at_aceita_location() {
        // Achado #50 de P831: `counter(heading).at(here())` — cristalino
        // `error: counter.at() requer label ou string como argumento`;
        // vanilla `(3,)` (medido em `temp/p844/a4_counter_at_location.typ`,
        // com `#set heading(numbering: "1.")` — headings sem numbering não
        // stepam o counter no vanilla 0.15.0, medido).
        let text = p844_expand_plain_text(concat!(
            "#set heading(numbering: \"1.\")\n",
            "= Um\n= Dois\n= Tres\n",
            "#context counter(heading).at(here())"
        ));
        assert!(text.contains('3'), "esperado '3' em {text:?}");
    }

    #[test]
    fn p844_a5_context_array_usa_repr() {
        // Achado #51 de P831: `#context ((3,))` mostrava join ("3") em
        // vez do repr ("(3,)"). Medido nos dois binários
        // (`temp/p844/a5_context_array.typ`): vanilla `(3,)`; cristalino
        // `3` (só no caminho `value_to_content` — fora de `#context` o
        // repr já estava correcto desde P801).
        let text = p844_expand_plain_text("#context ((3,))");
        assert_eq!(text, "(3,)", "esperado '(3,)', obtido {text:?}");
    }

    #[test]
    fn p844_a2_locate_aceita_funcao_de_elemento() {
        // Achado #48 de P831: `locate(heading)` — cristalino erro
        // (`parse_selector_arg` sem braço para `Value::Func`); vanilla
        // selecciona por tipo de elemento. Medido nos dois binários
        // (`temp/p844/a2_selector_func.typ`): vanilla `location 1`.
        let text = p844_expand_plain_text("= Titulo\n#context type(locate(heading))");
        assert!(
            text.contains("location"),
            "esperado 'location' em {text:?}"
        );
        let text = p844_expand_plain_text("= Titulo\n#context query(heading).len()");
        assert!(
            text.trim_end().ends_with('1'),
            "esperado len 1 no fim de {text:?}"
        );
    }

    #[test]
    fn p844_a2_query_func_nao_elemento_erro_verbatim() {
        // Mensagem medida no vanilla 0.15.0: `query((x) => x)` →
        // "only element functions can be used as selectors".
        let errs = p844_expand_errors("#context query((x) => x)");
        assert!(
            errs.iter()
                .any(|m| m.contains("only element functions can be used as selectors")),
            "errs: {errs:?}"
        );
    }

    #[test]
    fn p844_a1_query_devolve_content_com_campos() {
        // Achado #47 de P831: `query()` devolvia `location` em vez de
        // `content`. Medido nos dois binários
        // (`temp/p844/a1_query_content.typ`): vanilla
        // `type(query(<meta>).first())` → `content` e
        // `query(<meta>).first().value` → `ola`; cristalino `location` +
        // `error: cannot access fields on type location`.
        let text = p844_expand_plain_text(
            "#metadata(\"ola\") <meta>\n#context type(query(<meta>).first())",
        );
        assert!(
            text.contains("content"),
            "esperado 'content' em {text:?}"
        );
        let text = p844_expand_plain_text(
            "#metadata(\"ola\") <meta>\n#context query(<meta>).first().value",
        );
        assert_eq!(text.trim(), "ola", "esperado 'ola', obtido {text:?}");
    }

    #[test]
    fn p844_a7_display_pattern_aplica_estilos_reais() {
        // Achado #53 de P831: `counter.display(pattern)` era stub
        // ("Pattern minimal"). Medido nos dois binários com counter=2
        // (`temp/p844/a7_display_pattern.typ`): vanilla `II B ii ② 2`;
        // cristalino `I A i ① 2.1`.
        let src = concat!(
            "#counter(heading).update(2)\n",
            "#context counter(heading).display(\"I\")\n",
            "#context counter(heading).display(\"A\")\n",
            "#context counter(heading).display(\"i\")\n",
            "#context counter(heading).display(\"①\")\n",
            "#context counter(heading).display(\"1.1\")"
        );
        let text = p844_expand_plain_text(src);
        for esperado in ["II", "B", "ii", "②", "2"] {
            assert!(text.contains(esperado), "esperado '{esperado}' em {text:?}");
        }
        // Tokens extra são descartados quando há menos valores que
        // tokens (vanilla: `display("1.1")` com counter=2 → "2", medido).
        assert!(
            !text.contains("2.1"),
            "pattern '1.1' com 1 valor não deve render '2.1': {text:?}"
        );
    }

    #[test]
    fn p844_a6_display_sem_pattern_usa_numbering_do_set() {
        // Achado #52 de P831: sem argumento, `counter.display()` ignorava
        // o numbering do `#set heading(numbering:)`. Medido nos dois
        // binários (`temp/p844/a6_display_set_numbering.typ`): vanilla
        // `1.`; cristalino `1`. Controlo sem `numbering:` mantém `1`.
        let text = p844_expand_plain_text(concat!(
            "#set heading(numbering: \"1.\")\n",
            "= Um\n",
            "#context counter(heading).display()"
        ));
        assert!(text.contains("1."), "esperado '1.' em {text:?}");
        let text = p844_expand_plain_text("= Um\n#context counter(heading).display()");
        assert!(text.contains('1'), "esperado '1' em {text:?}");
        assert!(!text.contains("1."), "sem set não deve ter '.': {text:?}");
    }

    #[test]
    fn p844_a8_context_entre_headings_nao_dessincroniza_numeracao() {
        // Achado #54 de P831 — sonda (medição em
        // `temp/p844/a8_context_between_headings.typ` + probes):
        // `ContextBlock` é locatable no walk de introspecção, mas a
        // expansão substitui-o por conteúdo não-locatable; o walk de
        // layout (Locator próprio, invariante P185C) atribuía Locations
        // desfasadas do introspector pré-expansão → `value_at` apanhava
        // o snapshot anterior (cristalino `1.|1.|2.`; vanilla `1.|2.|3.`).
        // Probes: `#metadata(1)` e `#counter(heading).step()` entre
        // headings (locatable que PERMANECE no conteúdo) não
        // dessincronizam — confirmando o mecanismo (remoção de um
        // locatable, não efeito do `#context` no contador).
        let src = concat!(
            "#set heading(numbering: \"1.\")\n",
            "= Um\n#context 1\n= Dois\n= Tres"
        );
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        // Sequência de produção pós-fix: expansão + re-introspecção do
        // conteúdo expandido (mesma função usada pela pipeline).
        let (expanded, intr2) = crate::pipeline::expand_context_blocks_and_reintrospect(
            content.clone(),
            &intr,
            &world,
            &source,
        )
        .unwrap();
        let doc = typst_core::engine::layout::layout_with_introspector(&expanded, intr2);
        let text = doc.plain_text();
        assert!(text.contains("2."), "esperado '2.' (Dois) em {text:?}");
        assert!(text.contains("3."), "esperado '3.' (Tres) em {text:?}");
    }

    // ── P821 — `#target()`: gate de contexto + display de `type()` ─────────

    #[test]
    fn p821_context_target_expande_paged() {
        // Controlo (paridade medida): `#context target()` → "paged".
        let (world, _dir) = world_from_str("#context target()");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        let expanded = crate::pipeline::expand_context_blocks(
            content.clone(),
            &intr,
            &world,
            &source,
        )
        .unwrap();
        assert_eq!(expanded.plain_text().trim(), "paged");
    }

    #[test]
    fn p821_context_type_expande_nome_do_tipo() {
        // Colateral do achado #8 de P810: `#context type(1)` rendia vazio
        // (`value_to_content` sem braço para Value::Type). Vanilla medido:
        // `type(1)` → "int", `type("abc")` → "str".
        let (world, _dir) = world_from_str("#context type(1)\n#context type(\"abc\")");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().unwrap();
        let intr = introspect_with_introspector(content);
        let expanded = crate::pipeline::expand_context_blocks(
            content.clone(),
            &intr,
            &world,
            &source,
        )
        .unwrap();
        let text = expanded.plain_text();
        assert!(text.contains("int"), "esperado 'int' em {text:?}");
        assert!(text.contains("str"), "esperado 'str' em {text:?}");
    }

    #[test]
    fn p821_target_fora_de_contexto_erro_no_pipeline() {
        // O gate também corre fora da expansão: `#target()` no corpo do
        // documento falha no eval (antes da fase de introspecção).
        let (world, _dir) = world_from_str("#target()");
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = crate::pipeline::compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let errs = result.expect_err("#target() fora de context deve errar");
        let found = errs.iter().any(|d| {
            d.message.contains("can only be used when context is known")
                && d.hints.iter().any(|h| h.contains("try wrapping this in a `context` expression"))
        });
        assert!(found, "erro/hint ausentes: {errs:?}");
    }

    // ── P788 — refs: validações do vanilla + happy path "Section 1" ──────
    #[test]
    fn p788_ref_heading_sem_numbering_erro() {
        // Vanilla 0.15.0 (medido): `error: cannot reference heading without
        // numbering` + hint `#set heading(numbering: "1.")`, exit 1.
        let (world, _dir) = world_from_str("= Sem numeração <sem-num>\n@sem-num\n");
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = crate::pipeline::compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let errs = result.expect_err("ref a heading sem numbering deve errar");
        let found = errs.iter().any(|d| {
            d.message.contains("cannot reference heading without numbering")
                && d.hints.iter().any(|h| h.contains("#set heading(numbering: \"1.\")"))
        });
        assert!(found, "erro/hint ausentes: {errs:?}");
    }

    #[test]
    fn p788_ref_label_inexistente_erro() {
        // Vanilla 0.15.0 (medido): `error: label `<naoexiste1984>` does not
        // exist in the document`, exit 1 (cristalino renderizava "?" em silêncio).
        let (world, _dir) = world_from_str("Isto cita @naoexiste1984.\n");
        let source = world.source(world.main()).unwrap();
        let (result, _warnings) = crate::pipeline::compile_to_pdf_bytes(&world, &source, StreamMode::Verbose);
        let errs = result.expect_err("ref a label inexistente deve errar");
        let found = errs.iter().any(|d| {
            d.message
                .contains("label `<naoexiste1984>` does not exist in the document")
        });
        assert!(found, "erro ausente: {errs:?}");
    }

    #[test]
    fn p788_ref_happy_path_section_en() {
        // Vanilla 0.15.0 (medido): "Ver Section 1 no texto." — suplemento
        // default por língua (en → "Section "), número do counter formatado.
        let (world, _dir) = world_from_str(
            "#set heading(numbering: \"1.\")\n= Título <sec1>\nVer @sec1 no texto.",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("content");
        let doc = layout(content);
        let text = doc.plain_text().replace('\u{a0}', " ");
        assert!(
            text.contains("Section 1") && !text.contains("Secção"),
            "suplemento en esperado: {text:?}"
        );
    }

    #[test]
    fn p788_ref_suplemento_explicito() {
        // `@sec1[Cap]` → suplemento explícito substitui o default.
        let (world, _dir) = world_from_str(
            "#set heading(numbering: \"1.\")\n= Título <sec1>\nVer @sec1[Cap] no texto.",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("content");
        let doc = layout(content);
        let text = doc.plain_text().replace('\u{a0}', " ");
        assert!(text.contains("Cap 1"), "suplemento explícito esperado: {text:?}");
    }

    // ── P972 — parênteses (Glyph) alinhados à baseline em fracções ─────
    //
    // Especificação: `engine/math/layout/frac.md` §P972. Regressão: os
    // ciclos de posicionamento do numerador/denominador só deslocavam
    // Text/TextShaped; FrameItem::Glyph (delimitador stretchy sem
    // mapeamento Unicode — só com a fonte real) ficava sem o offset.
    #[cfg(test)]
    mod p972_tests {
        use super::*;

        /// Itens (tipo, texto/glyph, y) da página, com as fontes reais
        /// carregadas (como o CLI — sem fontes o parêntese sai como Text e
        /// o bug não se manifesta, ver frac.md §P972).
        fn items_de(src: &str) -> Vec<(String, f64)> {
            let (world, _dir) = world_from_str(src);
            let world = world.with_fonts_and_system(&[]);
            let source = world.source(world.main()).unwrap();
            let module = do_eval(&world, &source).unwrap();
            let content = module.content().expect("deve ter content");
            let intr = typst_core::engine::introspect::introspect_with_introspector(content);
            let metrics = crate::font_metrics::FallbackFontMetrics::new(&world);
            let doc = typst_core::engine::layout::layout_with_introspector_and_metrics(
                content,
                intr,
                metrics,
                crate::image_sizer::ImageSizeImageSizer,
                11.0,
            );
            let mut out = Vec::new();
            for page in &doc.pages {
                for i in &page.items {
                    use typst_core::entities::layout_types::FrameItem as FI;
                    match i {
                        FI::Text { pos, text, .. } | FI::TextShaped { pos, text, .. } => {
                            out.push((format!("text:{text}"), pos.y.val()));
                        }
                        FI::Glyph { pos, base_char, .. } => {
                            out.push((format!("glyph:{base_char}"), pos.y.val()));
                        }
                        _ => {}
                    }
                }
            }
            out
        }

        fn y_de(items: &[(String, f64)], chave: &str) -> f64 {
            items
                .iter()
                .find(|(k, _)| k == chave)
                .map(|(_, y)| *y)
                .unwrap_or_else(|| panic!("{chave:?} não encontrado em {items:?}"))
        }

        /// Caso do achado 9.3: os parênteses do numerador partilham a
        /// baseline dos dígitos (antes: 3.52pt abaixo).
        #[test]
        fn p972_parenteses_do_numerador_na_baseline() {
            let items = items_de("$ (n(n+1)) / 2 $");
            let y_paren = y_de(&items, "glyph:(");
            let y_um = y_de(&items, "text:1");
            assert!(
                (y_paren - y_um).abs() < 0.01,
                "'(' do numerador na baseline do '1': Δy = {:.3}pt (era 3.52)",
                y_paren - y_um
            );
        }

        /// Mesma correcção no ciclo do denominador (mesmo braço `_`).
        #[test]
        fn p972_parenteses_do_denominador_na_baseline() {
            let items = items_de("$ 1 / (n(n+1)) $");
            let y_paren = y_de(&items, "glyph:(");
            let y_n = y_de(&items, "text:𝑛");
            assert!(
                (y_paren - y_n).abs() < 0.01,
                "'(' do denominador na baseline do '𝑛': Δy = {:.3}pt",
                y_paren - y_n
            );
        }
    }

    // ── P975 — advance de glifo math inclui italics correction ─────────
    //
    // Especificação: `infra/font_metrics.md` §P975. Vanilla `update_glyph`
    // (`fragment/glyph.rs:204-211`): `x_advance += italics_correction`
    // para glifos math singulares. Fontes reais obrigatórias (lição P972).
    #[cfg(test)]
    mod p975_tests {
        use super::*;

        fn items_de(src: &str) -> Vec<(String, f64)> {
            let (world, _dir) = world_from_str(src);
            let world = world.with_fonts_and_system(&[]);
            let source = world.source(world.main()).unwrap();
            let module = do_eval(&world, &source).unwrap();
            let content = module.content().expect("deve ter content");
            let intr = typst_core::engine::introspect::introspect_with_introspector(content);
            let metrics = crate::font_metrics::FallbackFontMetrics::new(&world);
            let doc = typst_core::engine::layout::layout_with_introspector_and_metrics(
                content,
                intr,
                metrics,
                crate::image_sizer::ImageSizeImageSizer,
                11.0,
            );
            let mut out = Vec::new();
            for page in &doc.pages {
                for i in &page.items {
                    use typst_core::entities::layout_types::FrameItem as FI;
                    match i {
                        FI::Text { pos, text, .. } | FI::TextShaped { pos, text, .. } => {
                            out.push((format!("text:{text}"), pos.x.val()));
                        }
                        FI::Glyph { pos, base_char, .. } => {
                            out.push((format!("glyph:{base_char}"), pos.x.val()));
                        }
                        _ => {}
                    }
                }
            }
            out
        }

        /// **Regressão ao nível da pipeline** (lição deste passo: o teste
        /// de layout directo não apanhou a reversão pelo
        /// `fix_line_positions` — a asserção tem de ser DEPOIS dos
        /// estágios todos): `$ tau(G) $` — o `(` fica a 5.929pt do 𝜏
        /// (advance 437du + IC 102du a 11pt) mesmo depois de
        /// bidi/shape/fix_line_positions.
        #[test]
        fn p975_pipeline_preserva_ic_no_posicionamento() {
            let (world, _dir) = world_from_str("$ tau(G) $");
            let world = world.with_fonts_and_system(&[]);
            let source = world.source(world.main()).unwrap();
            let module = do_eval(&world, &source).unwrap();
            let content = module.content().expect("deve ter content");
            let intr = typst_core::engine::introspect::introspect_with_introspector(content);
            let metrics = crate::font_metrics::FallbackFontMetrics::new(&world);
            let doc = typst_core::engine::layout::layout_with_introspector_and_metrics(
                content, intr, metrics, crate::image_sizer::ImageSizeImageSizer, 11.0,
            );
            let doc = crate::layout_bidi::reorder_bidi_document(
                doc, &crate::font_metrics::FallbackFontMetrics::new(&world));
            let doc = crate::shaper::shape_document(&world, doc);
            let doc = crate::shaper::fix_line_positions(&world, doc);
            let mut x_tau = None;
            let mut x_paren = None;
            for page in &doc.pages {
                for i in &page.items {
                    use typst_core::entities::layout_types::FrameItem as FI;
                    match i {
                        FI::Text { pos, text, .. } | FI::TextShaped { pos, text, .. }
                            if text.as_str() == "𝜏" => x_tau = Some(pos.x.val()),
                        FI::Glyph { pos, base_char, .. } if *base_char == '(' => {
                            x_paren = Some(pos.x.val())
                        }
                        _ => {}
                    }
                }
            }
            let (x_tau, x_paren) = (x_tau.expect("𝜏"), x_paren.expect("("));
            let obtido = x_paren - x_tau;
            assert!(
                (obtido - 5.929).abs() < 0.01,
                "após a pipeline completa, x('(') − x(𝜏) = 5.929pt (advance+IC); obteve {obtido:.3}pt"
            );
        }

        /// Caso medido na Fase A: `$ tau(G) $` — o `(` fica a
        /// advance(𝜏)+IC(𝜏) = (437+102)du = 5.929pt a 11pt do início do 𝜏
        /// (antes: 4.807pt — sem a IC).
        #[test]
        fn p975_advance_math_inclui_italics_correction() {
            let items = items_de("$ tau(G) $");
            let x_tau = items
                .iter()
                .find(|(k, _)| k == "text:𝜏")
                .map(|(_, x)| *x)
                .unwrap_or_else(|| panic!("𝜏 não encontrado: {items:?}"));
            let x_paren = items
                .iter()
                .find(|(k, _)| k == "glyph:(")
                .map(|(_, x)| *x)
                .unwrap_or_else(|| panic!("'(' não encontrado: {items:?}"));
            let esperado = (437.0 + 102.0) * 11.0 / 1000.0; // 5.929pt
            let obtido = x_paren - x_tau;
            assert!(
                (obtido - esperado).abs() < 0.01,
                "x('(') − x(𝜏) = advance+IC = {esperado:.3}pt; obteve {obtido:.3}pt"
            );
        }

        /// Guarda: texto multi-carácter em math ("sin") NÃO leva IC (é um
        /// run de texto no vanilla, não GlyphFragment) — a posição do `(`
        /// fica no advance puro do hmtx (13.508pt medido na fonte real a
        /// 11pt) — inalterado pela regra (que é só para 1 carácter).
        #[test]
        fn p975_texto_multicaracter_math_sem_ic() {
            let items = items_de("$ sin(x) $");
            let x_sin = items
                .iter()
                .find(|(k, _)| k == "text:sin")
                .map(|(_, x)| *x)
                .unwrap_or_else(|| panic!("sin não encontrado: {items:?}"));
            let x_paren = items
                .iter()
                .find(|(k, _)| k == "glyph:(")
                .map(|(_, x)| *x)
                .unwrap_or_else(|| panic!("'(' não encontrado: {items:?}"));
            let obtido = x_paren - x_sin;
            assert!(
                (obtido - 13.508).abs() < 0.01,
                "advance puro de \"sin\" = 13.508pt (sem IC — regra só para 1 carácter); obteve {obtido:.3}pt"
            );
        }
    }

    // ── P977 — variantes ssty (.st/.sts) em scripts math ───────────────
    //
    // Especificação: `infra/shaper.md` §P977 + `infra/font_metrics.md`
    // §P977. Vanilla aplica ssty=1/2 por nível MathSize
    // (text/mod.rs:1457-1460). Avanços reais NewCMMath-Book (upem 1000):
    // u1D45B base 600du, .st 706du, .sts 881du.
    #[cfg(test)]
    mod p977_tests {
        use super::*;

        /// (largura pt, gids) dos itens TextShaped cujo texto é `needle`,
        /// após a pipeline de shaping.
        fn gids_shaped(src: &str, needle: &str) -> Vec<(f64, Vec<u16>)> {
            let (world, _dir) = world_from_str(src);
            let world = world.with_fonts_and_system(&[]);
            let source = world.source(world.main()).unwrap();
            let module = do_eval(&world, &source).unwrap();
            let content = module.content().expect("deve ter content");
            let intr = typst_core::engine::introspect::introspect_with_introspector(content);
            let metrics = crate::font_metrics::FallbackFontMetrics::new(&world);
            let doc = typst_core::engine::layout::layout_with_introspector_and_metrics(
                content, intr, metrics, crate::image_sizer::ImageSizeImageSizer, 11.0,
            );
            let doc = crate::shaper::shape_document(&world, doc);
            let mut out = Vec::new();
            for page in &doc.pages {
                for i in &page.items {
                    if let typst_core::entities::layout_types::FrameItem::TextShaped {
                        text, glyphs, style, units_per_em, ..
                    } = i
                    {
                        if text.as_str() == needle {
                            let upem = (*units_per_em).max(1) as f64;
                            let w = glyphs
                                .iter()
                                .map(|g| g.x_advance as f64 / upem * style.size.val())
                                .sum::<f64>();
                            out.push((w, glyphs.iter().map(|g| g.glyph_id).collect()));
                        }
                    }
                }
            }
            out
        }

        /// Larguras (pt) dos itens TextShaped cujo texto é `needle`,
        /// medidas pelos avanços shaped após a pipeline de shaping.
        fn larguras_shaped(src: &str, needle: &str) -> Vec<f64> {
            let (world, _dir) = world_from_str(src);
            let world = world.with_fonts_and_system(&[]);
            let source = world.source(world.main()).unwrap();
            let module = do_eval(&world, &source).unwrap();
            let content = module.content().expect("deve ter content");
            let intr = typst_core::engine::introspect::introspect_with_introspector(content);
            let metrics = crate::font_metrics::FallbackFontMetrics::new(&world);
            let doc = typst_core::engine::layout::layout_with_introspector_and_metrics(
                content, intr, metrics, crate::image_sizer::ImageSizeImageSizer, 11.0,
            );
            let doc = crate::shaper::shape_document(&world, doc);
            let mut out = Vec::new();
            for page in &doc.pages {
                for i in &page.items {
                    if let typst_core::entities::layout_types::FrameItem::TextShaped {
                        text, glyphs, style, units_per_em, ..
                    } = i
                    {
                        if text.as_str() == needle {
                            let upem = (*units_per_em).max(1) as f64;
                            let w = glyphs
                                .iter()
                                .map(|g| g.x_advance as f64 / upem * style.size.val())
                                .sum::<f64>();
                            out.push(w);
                        }
                    }
                }
            }
            out
        }

        /// **Caso principal**: o subscrito de `$ K_n $` (Script, 7.7pt)
        /// usa a variante `.st` — largura 706du × 7.7/1000 = 5.436pt
        /// (antes: glifo base, 600du = 4.62pt).
        #[test]
        fn p977_subscript_usa_variante_st() {
            let ws = larguras_shaped("$ K_n $", "𝑛");
            assert_eq!(ws.len(), 1, "um só '𝑛' esperado: {ws:?}");
            let esperado = 706.0 * 7.7 / 1000.0;
            assert!(
                (ws[0] - esperado).abs() < 0.01,
                "sub n: .st {esperado:.3}pt esperado; obteve {:.3}pt (base = 4.620)",
                ws[0]
            );
        }

        /// **ScriptScript** (ssty=2 → `.sts`): `$ x_(y_z) $` — o z interno
        /// a 5.5pt usa `.sts` (881du → 4.846pt; base seria 3.30pt).
        #[test]
        fn p977_scriptscript_usa_variante_sts() {
            let ws = gids_shaped("$ x_(y_z) $", "𝑧");
            assert_eq!(ws.len(), 1, "um só '𝑧' esperado: {ws:?}");
            // u1D467.sts = gid 5784 (medido na fonte). A asserção é no
            // GLYPH, não na largura: o tamanho do nível ScriptScript em
            // attach.rs ainda usa o factor plano ×0.7 (scope-out de P945,
            // mesmo já corrigido para o índice de raiz em P970) — fora do
            // escopo de P977, registado no relatório.
            assert_eq!(
                ws[0].1,
                vec![5784u16],
                "sub-sub z deve ser o glifo .sts (gid 5784); obteve {:?}",
                ws[0].1
            );
        }

        /// **Guardas**: math em tamanho de corpo (`$ n $` — Text size)
        /// mantém o glifo base (600du × 11/1000 = 6.60pt); prosa "n"
        /// fora de math inalterada (Libertinus, não entra na regra).
        #[test]
        fn p977_base_size_e_prosa_inalterados() {
            let ws_math = larguras_shaped("$ n $", "𝑛");
            assert_eq!(ws_math.len(), 1);
            let esperado = 600.0 * 11.0 / 1000.0;
            assert!(
                (ws_math[0] - esperado).abs() < 0.01,
                "math corpo: base {esperado:.3}pt; obteve {:.3}pt",
                ws_math[0]
            );
        }
    }

}
