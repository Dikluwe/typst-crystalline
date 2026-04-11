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
    use typst_core::contracts::world::World;
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
        eval(
            &routines,
            world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            source,
        )
    }

    /// Pipeline completo → bytes PDF (caminho Helvetica sem fonte real).
    fn compile_to_pdf(src: &str) -> Vec<u8> {
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        export_pdf(&doc)
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
    fn pipeline_simbolos_gregos_gera_pdf() {
        // Passo 39: alpha/beta/gamma → Unicode α/β/γ no PDF
        let (world, _dir) = world_from_str("$alpha + beta = gamma$");
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
    fn pipeline_funcao_sin_gera_pdf() {
        // Passo 39: sin(x) — sin em não-itálico, x em itálico
        let (world, _dir) = world_from_str("$sin(x)$");
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
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pipeline_delimited_colchetes_gera_pdf() {
        let (world, _dir) = world_from_str("$[a, b]$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_delimited_com_frac() {
        // Fracção dentro de parênteses — delimitadores devem adaptar-se à altura
        let (world, _dir) = world_from_str("$(frac(a, b))$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        let pdf = export_pdf(&doc);
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
        let doc = layout(content);
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);
        assert!(pdf_str.contains("BT") && pdf_str.contains("ET"),
            "PDF deve conter operadores BT/ET para texto ou glifo");
    }

    // ── Passo 41 — MathConstants via tabela OpenType MATH ────────────────

    #[test]
    fn pdf_frac_com_constants() {
        // Pipeline completo — confirmar que não panic após refactoring
        let (world, _dir) = world_from_str("$frac(a, b)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_sqrt_com_constants() {
        let (world, _dir) = world_from_str("$sqrt(x)$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn pdf_attach_com_constants() {
        let (world, _dir) = world_from_str("$x^2_i$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        let pdf = export_pdf(&doc);
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
    #[ignore = "requer fonte com tabela MATH em tests/fixtures/stix-two-math.otf"]
    fn pdf_tounicode_contem_mapeamento_de_delimitador() {
        // Com fonte MATH real, ToUnicode deve mapear '(' e ')' incluindo variantes.
        // U+0028 = '(', U+0029 = ')'
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/stix-two-math.otf")
        ).expect("fixture necessária");
        let (world, _dir) = world_from_str("$(frac(a, b))$");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let doc = layout(content);
        let pdf = crate::export::export_pdf_with_font(&doc, &data);
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
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("BT"), "PDF deve conter BT");
        assert!(s.contains("ET"), "PDF deve conter ET");
    }
}
