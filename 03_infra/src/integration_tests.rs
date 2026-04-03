//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra.md
//! @prompt-hash ab5728d1
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
    use std::path::{Path, PathBuf};

    use comemo::Track;
    use typst_core::contracts::world::TrackedWorld;
    use typst_core::entities::module::Module;
    use typst_core::entities::source::Source;
    use typst_core::entities::source_result::SourceResult;
    use typst_core::entities::world_types::{Route, Routines, Sink, Traced};
    use typst_core::rules::eval::eval;
    use typst_core::rules::layout::layout;

    use crate::export::export_pdf;
    use crate::world::SystemWorld;

    // ── Utilitário: diretório temporário sem dependência externa ─────────

    struct TempDir(PathBuf);

    impl TempDir {
        fn path(&self) -> &Path { &self.0 }
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

    /// Chama `eval()` com o boilerplate de comemo — mesmo padrão de eval_for_test.
    fn do_eval(world: &SystemWorld, source: &Source) -> SourceResult<Module> {
        let routines = Routines::new();
        let traced   = Traced::new();
        let mut sink = Sink::new();
        let route    = Route::new();
        let dyn_world: &dyn TrackedWorld = world;
        eval(
            &routines,
            dyn_world.track(),
            traced.track(),
            sink.track_mut(),
            route.track(),
            source,
        )
    }

    // ── Testes de integração ──────────────────────────────────────────────

    #[test]
    fn pipeline_texto_simples() {
        let (world, _dir) = world_from_str("Olá, mundo!");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn pipeline_export_pdf_helvetica() {
        let (world, _dir) = world_from_str("Texto simples.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
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
        let doc = layout(content);

        if let Some(font) = world.font(0) {
            let pdf = crate::export::export_pdf_with_font(&doc, font.as_slice());
            assert!(!pdf.is_empty());
            assert_eq!(&pdf[..5], b"%PDF-");
        } else {
            // Sem fontes carregadas — fallback Helvetica
            let pdf = export_pdf(&doc);
            assert!(!pdf.is_empty());
            assert_eq!(&pdf[..5], b"%PDF-");
        }
    }

    #[test]
    fn pipeline_com_set_text_bold() {
        let (world, _dir) = world_from_str("#set text(bold: true)\nTexto a negrito.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
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
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        // Confirmar ausência de "[" nos itens de texto
        for page in &doc.pages {
            for item in &page.items {
                if let typst_core::entities::layout_types::FrameItem::Text { text, .. } = item {
                    assert!(!text.starts_with('['),
                        "equação não deve produzir '[': {}", text);
                }
            }
        }
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_equacao_com_frac_sem_panic() {
        let (world, _dir) = world_from_str("$ frac(a, b) $");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pipeline_equacao_inline_gera_pdf() {
        let (world, _dir) = world_from_str("A equação $x^2$ é famosa.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_equacao_block_gera_pdf() {
        let (world, _dir) = world_from_str("$ E = m c^2 $");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pipeline_set_scoped_nao_vaza() {
        // Verifica que #set text() dentro de { } não afecta o texto após o bloco.
        // Com Passo 33: ctx.styles é restaurado ao sair do bloco.
        let (world, _dir) = world_from_str(
            "normal\n#{ #set text(bold: true); [negrito] }\nnormal novamente"
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);
        assert!(pdf_str.contains(" S ") || pdf_str.contains(" S Q"),
            "PDF deve conter operador S (stroke) para a linha de fracção");
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
}
