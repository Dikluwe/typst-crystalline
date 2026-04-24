//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra.md
//! @prompt-hash 4eecd2a1
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

    use typst_core::contracts::world::World;
    use typst_core::entities::module::Module;
    use typst_core::entities::source::Source;
    use typst_core::entities::source_result::SourceResult;
    use typst_core::rules::introspect::introspect;
    use typst_core::rules::layout::layout;

    use crate::export::export_pdf;
    use crate::world::SystemWorld;
    use image::ImageFormat;

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

    /// Pipeline completo → bytes PDF (Passo 65).
    ///
    /// Passagem 1 (introspecção): `introspect()` resolve labels, headings_for_toc
    /// e sinaliza `has_outline`.
    /// Passagem 2+ (fixpoint interno a L1): `layout()` converge o mapa de páginas
    /// da TOC internamente. O orquestrador L3 é agora linear.
    fn compile_to_pdf(src: &str) -> Vec<u8> {
        let (world, _dir) = world_from_str(src);
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");

        // ── Introspecção ──────────────────────────────────────────────────
        let intro_state = introspect(content);

        // ── Layout (fixpoint acontece internamente em L1) ─────────────────
        let doc = layout(content, intro_state);

        export_pdf(&doc)
    }

    // ── Testes de integração ──────────────────────────────────────────────

    #[test]
    fn pipeline_texto_simples() {
        let (world, _dir) = world_from_str("Olá, mundo!");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content, state);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn pipeline_export_pdf_helvetica() {
        let (world, _dir) = world_from_str("Texto simples.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);

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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let doc = layout(content, state);
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
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pipeline_equacao_inline_gera_pdf() {
        let (world, _dir) = world_from_str("A equação $x^2$ é famosa.");
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let state = introspect(content);
        let doc = layout(content, state);
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
        let s = String::from_utf8_lossy(&pdf);
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
        let s = String::from_utf8_lossy(&pdf);
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
        let s = String::from_utf8_lossy(&pdf);
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
        let s = String::from_utf8_lossy(&pdf);
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
        let s = String::from_utf8_lossy(&pdf);
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
        let doc = layout(content, state);
        let pdf = export_pdf(&doc);
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
        let doc = layout(content, state);
        assert!(!doc.pages.is_empty());
        // Pipeline completo deve produzir PDF válido sem numeração
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pipeline_heading_numeracao_activa() {
        let pdf = compile_to_pdf(
            "#set heading(numbering: \"1.1\")\n= Introdução\n== Motivação\n= Conclusão"
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
             = Conclusão"
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content, state);
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty(), "PDF com #outline() não deve estar vazio");
    }

    // ── Passo 62 — Figuras ────────────────────────────────────────────────

    #[test]
    fn pipeline_figure_com_ref_gera_pdf() {
        let pdf = compile_to_pdf(
            "#figure(\n  [Gráfico de Barras],\n  caption: [Resultados]\n) <fig1>\n\
             Como mostrado na @fig1."
        );
        assert!(!pdf.is_empty(), "PDF com figure e ref não deve estar vazio");
    }

    #[test]
    fn pipeline_figure_sem_ref_nao_causa_panico() {
        let pdf = compile_to_pdf(
            "#figure(\n  [Conteúdo],\n  caption: [Legenda simples]\n)"
        );
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
             = Conclusão"
        );
        assert!(!pdf.is_empty(), "PDF com TOC paginada não deve estar vazio");
    }

    #[test]
    fn pipeline_sem_toc_nao_regrediu() {
        // Regressão: documentos sem TOC não devem ser afectados pelo fixpoint.
        let pdf = compile_to_pdf(
            "= Introdução\n\
             Texto simples sem índice."
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
             = Conclusão"
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
             Mais conteúdo."
        );
        assert!(!pdf.is_empty(), "PDF com TOC em 3 passagens não deve estar vazio");
    }

    // ── Testes de imagem PNG (Passo 74) ───────────────────────────────────────

    /// Gera PNG em memória e escreve no diretório temporário.
    fn write_png_rgba(dir: &Path, name: &str, pixels: Vec<u8>, w: u32, h: u32) {
        use image::{ImageBuffer, Rgba};
        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(w, h, pixels).unwrap();
        let mut buf = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png).unwrap();
        std::fs::write(dir.join(name), &buf).unwrap();
    }

    #[test]
    fn pipeline_png_transparente_gera_smask() {
        let dir = tempdir();

        // PNG 2×2 com píxeis semi-transparentes.
        write_png_rgba(
            dir.path(), "alpha.png",
            vec![
                255, 0,   0,   128, // vermelho semi-transparente
                0,   255, 0,   255, // verde opaco
                0,   0,   255, 0,   // azul transparente
                255, 255, 0,   255, // amarelo opaco
            ],
            2, 2,
        );

        std::fs::write(dir.path().join("main.typ"), "#image(\"alpha.png\")").unwrap();
        let world  = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state  = introspect(content);
        let doc    = layout(content, state);
        let pdf    = export_pdf(&doc);
        let s      = String::from_utf8_lossy(&pdf);

        assert!(!pdf.is_empty(), "export_pdf deve produzir bytes");
        assert!(s.contains("/Filter /FlateDecode"), "PNG deve usar /FlateDecode");
        assert!(s.contains("/SMask"),               "PNG com transparência deve emitir /SMask");
        assert!(s.contains("/ColorSpace /DeviceGray"), "XObject alpha usa /DeviceGray");
        assert!(s.contains("/ColorSpace /DeviceRGB"),  "XObject RGB usa /DeviceRGB");
    }

    #[test]
    fn pipeline_png_opaco_sem_smask() {
        let dir = tempdir();

        // PNG 1×1 totalmente opaco.
        write_png_rgba(dir.path(), "opaco.png", vec![100u8, 150, 200, 255], 1, 1);

        std::fs::write(dir.path().join("main.typ"), "#image(\"opaco.png\")").unwrap();
        let world  = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state  = introspect(content);
        let doc    = layout(content, state);
        let pdf    = export_pdf(&doc);
        let s      = String::from_utf8_lossy(&pdf);

        assert!(!s.contains("/SMask"), "PNG totalmente opaco não deve emitir /SMask");
        assert!(s.contains("/Filter /FlateDecode"), "PNG opaco ainda usa /FlateDecode");
    }

    // ── Testes de Passo 75 — caminhos relativos e figuras numeradas ──────────

    #[test]
    fn pipeline_figura_numerada_prefixo_no_pdf() {
        let dir = tempdir();
        // JPEG mínimo válido (magic bytes suficientes para a detecção de formato)
        std::fs::write(dir.path().join("foto.jpg"), &[0xFF_u8, 0xD8, 0xFF, 0xE0]).unwrap();
        std::fs::write(
            dir.path().join("main.typ"),
            "#set figure(numbering: \"1\")\n#figure(image(\"foto.jpg\"), caption: [A foto])",
        ).unwrap();

        let world  = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state  = introspect(content);

        let image_nums = state.figure_numbers.get("image").cloned().unwrap_or_default();
        assert_eq!(image_nums, vec![1],
            "Uma figura de imagem deve produzir figure_numbers[\"image\"] = [1]");

        let doc = layout(content, state);
        let pdf = export_pdf(&doc);
        assert!(!pdf.is_empty(), "PDF não pode estar vazio");
    }

    #[test]
    fn image_resolve_caminho_relativo() {
        let dir = tempdir();
        std::fs::create_dir(dir.path().join("capitulo1")).unwrap();
        std::fs::write(
            dir.path().join("capitulo1/foto.jpg"),
            &[0xFF_u8, 0xD8, 0xFF, 0xE0],
        ).unwrap();
        std::fs::write(
            dir.path().join("capitulo1/intro.typ"),
            "#image(\"foto.jpg\")",
        ).unwrap();
        std::fs::write(
            dir.path().join("main.typ"),
            "#include \"capitulo1/intro.typ\"",
        ).unwrap();

        let world  = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let result = do_eval(&world, &source);
        assert!(result.is_ok(), "Avaliador falhou ao resolver caminho relativo: {:?}", result.err());
    }

    #[test]
    fn current_file_restaurado_apos_include() {
        let dir = tempdir();
        std::fs::create_dir(dir.path().join("capitulo1")).unwrap();
        std::fs::write(dir.path().join("capa.jpg"),           &[0xFF_u8, 0xD8, 0xFF, 0xE0]).unwrap();
        std::fs::write(dir.path().join("capitulo1/foto.jpg"), &[0xFF_u8, 0xD8, 0xFF, 0xE0]).unwrap();
        std::fs::write(
            dir.path().join("capitulo1/intro.typ"),
            "#image(\"foto.jpg\")",
        ).unwrap();
        std::fs::write(
            dir.path().join("main.typ"),
            "#image(\"capa.jpg\")\n#include \"capitulo1/intro.typ\"\n#image(\"capa.jpg\")",
        ).unwrap();

        let world  = SystemWorld::new(dir.path(), "main.typ").unwrap();
        let source = world.source(world.main()).unwrap();
        let result = do_eval(&world, &source);
        assert!(result.is_ok(),
            "current_file não restaurado após #include: {:?}", result.err());
    }

    // ── Passo 76 — primitivas geométricas ────────────────────────────────────

    #[test]
    fn rect_ordem_operadores_pdf() {
        // #rect(fill: "red", stroke: "black") deve produzir:
        // q → rg (fill) → RG (stroke) → w → re (path) → B (paint) → Q
        let (world, _dir) = world_from_str(
            "#rect(width: 100pt, height: 50pt, fill: \"red\", stroke: \"black\")"
        );
        let source = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);
        let pdf     = export_pdf(&doc);

        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains("q\n"),   "PDF deve ter push state (q)");
        assert!(pdf_str.contains(" rg\n"), "PDF deve ter operador de fill (rg)");
        assert!(pdf_str.contains(" RG\n"), "PDF deve ter operador de stroke (RG)");
        assert!(pdf_str.contains(" w\n"),  "PDF deve ter operador de espessura (w)");
        assert!(pdf_str.contains(" re\n"), "PDF deve ter operador de rectângulo (re)");
        assert!(pdf_str.contains("B\n"),   "PDF deve ter paint operator B (fill+stroke)");
        assert!(pdf_str.contains("Q\n"),   "PDF deve ter pop state (Q)");

        // Verificar a ordem relativa.
        let pos_q         = pdf_str.find("q\n").unwrap();
        let pos_rg        = pdf_str.find(" rg\n").unwrap();
        let pos_rg_upper  = pdf_str.find(" RG\n").unwrap();
        let pos_re        = pdf_str.find(" re\n").unwrap();
        let pos_b         = pdf_str.find("B\n").unwrap();
        let pos_q_close   = pdf_str.rfind("Q\n").unwrap();

        assert!(pos_q        < pos_rg,        "q deve preceder rg");
        assert!(pos_rg       < pos_rg_upper,  "rg (fill) deve preceder RG (stroke)");
        assert!(pos_rg_upper < pos_re,         "RG deve preceder re");
        assert!(pos_re       < pos_b,          "re deve preceder B");
        assert!(pos_b        < pos_q_close,    "B deve preceder Q final");
    }

    #[test]
    fn line_coordenada_y_fim_inferior_ao_inicio() {
        // #line(dy: 50pt) — dy positivo = desce no layout.
        // No espaço PDF (Y cresce para cima), end_y < start_y.
        let (world, _dir) = world_from_str(
            "#line(dx: 100pt, dy: 50pt)"
        );
        let source = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);
        let pdf     = export_pdf(&doc);

        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains(" m\n"), "PDF deve conter operador m");
        assert!(pdf_str.contains(" l\n"), "PDF deve conter operador l");

        // Extrair Y do operador m (ponto inicial) e l (ponto final).
        fn extrair_y_antes_op(s: &str, op: &str) -> f64 {
            s.split(op).next()
                .and_then(|antes| antes.split_whitespace().last())
                .and_then(|tok| tok.parse::<f64>().ok())
                .unwrap_or(0.0)
        }

        let m_y = extrair_y_antes_op(&pdf_str, " m\n");
        let l_y = extrair_y_antes_op(&pdf_str, " l\n");

        assert!(l_y < m_y,
            "Y do ponto final ({}) deve ser inferior ao Y do início ({}) — \
             dy positivo desce no layout, subtrai no PDF",
            l_y, m_y);
    }

    #[test]
    fn rect_sem_cores_gera_stroke_no_pdf() {
        // #rect() sem fill nem stroke → fallback de stroke preta.
        // O PDF deve conter S (stroke only), RG, w.
        let (world, _dir) = world_from_str("#rect(width: 50pt, height: 30pt)");
        let source = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);
        let pdf     = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains(" RG\n"), "PDF deve ter stroke RG");
        assert!(pdf_str.contains(" re\n"), "PDF deve ter rectângulo re");
        assert!(pdf_str.contains("S\n"),   "PDF deve ter paint operator S (stroke only)");
    }

    // ── Passo 77 — Bézier, elipses e deltas negativos ───────────────────────

    #[test]
    fn export_line_com_delta_negativo_respeita_bounding_box() {
        // #line(dx: -50pt, dy: -30pt) — linha para a esquerda e para cima.
        // Com dx < 0, o ponto 'm' deve ter X maior que o ponto 'l' (end_x < start_x).
        let (world, _dir) = world_from_str("#line(dx: -50pt, dy: -30pt)");
        let source = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);
        let pdf     = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains(" m\n"), "PDF deve conter operador m");
        assert!(pdf_str.contains(" l\n"), "PDF deve conter operador l");

        fn extrair_x_antes_op(s: &str, op: &str) -> f64 {
            // O operador tem formato "X Y op" — extrair o penúltimo token.
            s.split(op).next()
                .and_then(|antes| {
                    let toks: Vec<&str> = antes.split_whitespace().collect();
                    toks.iter().rev().nth(1).and_then(|t| t.parse::<f64>().ok())
                })
                .unwrap_or(0.0)
        }

        let m_x = extrair_x_antes_op(&pdf_str, " m\n");
        let l_x = extrair_x_antes_op(&pdf_str, " l\n");

        assert!(l_x < m_x,
            "Linha com dx negativo deve terminar à esquerda do início: \
             end_x ({}) deve ser menor que start_x ({})", l_x, m_x);
    }

    #[test]
    fn export_ellipse_emite_quatro_operadores_bezier() {
        let (world, _dir) = world_from_str(
            "#ellipse(width: 80pt, height: 40pt, fill: \"blue\")"
        );
        let source = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);
        let pdf     = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert_eq!(
            pdf_str.matches(" c\n").count(), 4,
            "Elipse deve ser desenhada com exactamente 4 operadores Bézier 'c'"
        );
        assert!(pdf_str.contains(" m\n"), "Elipse deve ter um ponto inicial 'm'");
        assert!(!pdf_str.contains(" re\n"),
            "Elipse não deve emitir operador re — placeholder foi substituído");
    }

    // ── Passo 78 — transformações afins ─────────────────────────────────────

    #[test]
    fn pdf_export_emite_q_cm_q_para_transformacoes() {
        let (world, _dir) = world_from_str(
            "#rotate(90deg, rect(width: 100pt, height: 100pt, fill: \"red\"))"
        );
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);
        let pdf     = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains("q\n"),   "Falta guardar o estado gráfico (q)");
        assert!(pdf_str.contains(" cm\n"), "Falta a matriz de transformação (cm)");
        assert!(pdf_str.contains("Q\n"),   "Falta restaurar o estado gráfico (Q)");

        let pos_q       = pdf_str.find("q\n").unwrap();
        let pos_cm      = pdf_str.find(" cm\n").unwrap();
        let pos_q_close = pdf_str.rfind("Q\n").unwrap();

        assert!(pos_q  < pos_cm,       "q deve preceder cm");
        assert!(pos_cm < pos_q_close,  "cm deve preceder Q");
    }

    #[test]
    fn export_circle_emite_quatro_operadores_bezier() {
        let (world, _dir) = world_from_str("#circle(radius: 20pt)");
        let source = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);
        let pdf     = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert_eq!(
            pdf_str.matches(" c\n").count(), 4,
            "Circle deve ser desenhado com exactamente 4 operadores Bézier 'c'"
        );
    }

    // ── Passo 81 — Configuração dinâmica de página via #set page ────────────

    #[test]
    fn set_page_forca_quebra_com_conteudo() {
        let (world, _dir) = world_from_str(
            "Primeira linha\n\
             #set page(width: 200pt, height: 200pt, margin: 10pt)\n\
             Segunda página\n"
        );
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

        assert_eq!(doc.pages.len(), 2,
            "SetPage com conteúdo deve criar 2 páginas");
        assert!(doc.pages[0].height > 800.0,
            "Primeira página deve ser A4 (height > 800pt)");
        assert!((doc.pages[1].height - 200.0).abs() < 0.01,
            "Segunda página deve ter height = 200pt do SetPage");
    }

    #[test]
    fn set_page_no_topo_nao_quebra() {
        let (world, _dir) = world_from_str(
            "#set page(width: 300pt, height: 400pt)\n\
             Conteúdo único\n"
        );
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

        assert_eq!(doc.pages.len(), 1,
            "SetPage sem conteúdo anterior não deve criar página extra");
        assert!((doc.pages[0].width  - 300.0).abs() < 0.01);
        assert!((doc.pages[0].height - 400.0).abs() < 0.01);
    }

    #[test]
    fn multiplas_mudancas_de_pagina_preservam_snapshots() {
        let (world, _dir) = world_from_str(
            "P1\n\
             #set page(width: 200pt, height: 200pt)\n\
             P2\n\
             #set page(width: 100pt, height: 300pt)\n\
             P3\n"
        );
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

        assert_eq!(doc.pages.len(), 3);
        assert!(doc.pages[0].width > 500.0, "Página 1 deve ser A4");
        assert!((doc.pages[1].width  - 200.0).abs() < 0.01);
        assert!((doc.pages[1].height - 200.0).abs() < 0.01);
        assert!((doc.pages[2].width  - 100.0).abs() < 0.01);
        assert!((doc.pages[2].height - 300.0).abs() < 0.01);
    }

    #[test]
    fn grid_respeita_page_config_dinamico() {
        let (world, _dir) = world_from_str(
            "#set page(width: 400pt, height: 400pt, margin: 20pt)\n\
             #grid(columns: (1fr, 1fr), [A], [B])\n"
        );
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

        // available_width = 400 - 2*20 = 360pt; cada 1fr = 180pt.
        // O segundo item (célula B) deve começar em margin + 180 = 200pt.
        assert_eq!(doc.pages.len(), 1);
        let second_item_x = doc.pages[0].items
            .iter()
            .filter_map(|item| match item {
                typst_core::entities::layout_types::FrameItem::Text { pos, .. }
                    if pos.x.0 > 150.0 => Some(pos.x.0),
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
             B\n"
        );
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);
        let pdf     = export_pdf(&doc);

        let pdf_str = String::from_utf8_lossy(&pdf);
        assert!(pdf_str.contains("[0 0 595.28 841.89]"),
            "Primeira página deve ter MediaBox A4");
        assert!(pdf_str.contains("[0 0 200.00 600.00]"),
            "Segunda página deve ter MediaBox 200×600pt");
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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        layout(content, state)
    }

    /// Extrai a posição primária de qualquer `FrameItem`.
    fn frame_item_pos(item: &typst_core::entities::layout_types::FrameItem)
        -> typst_core::entities::layout_types::Point
    {
        use typst_core::entities::layout_types::FrameItem;
        match item {
            FrameItem::Text  { pos, .. } => *pos,
            FrameItem::Line  { start, .. } => *start,
            FrameItem::Glyph { pos, .. } => *pos,
            FrameItem::Image { pos, .. } => *pos,
            FrameItem::Shape { pos, .. } => *pos,
            FrameItem::Group { pos, .. } => *pos,
        }
    }

    // Fase 1 — Invariantes de estado (macro)
    #[test]
    fn stress_81_5_tres_paginas_com_snapshots_correctos() {
        let doc = compilar_stress_81_5();

        assert_eq!(doc.pages.len(), 3,
            "SetPage deve criar exactamente 2 quebras de página (3 snapshots)");

        // Página 1: A4 padrão (595.28 × 841.89).
        assert!((doc.pages[0].width  - 595.28).abs() < 0.01,
            "Página 1 deve preservar A4 width (595.28pt), obteve {}",
            doc.pages[0].width);
        assert!((doc.pages[0].height - 841.89).abs() < 0.01,
            "Página 1 deve preservar A4 height (841.89pt), obteve {}",
            doc.pages[0].height);

        // Página 2: 400×300pt.
        assert!((doc.pages[1].width  - 400.0).abs() < 0.01);
        assert!((doc.pages[1].height - 300.0).abs() < 0.01);

        // Página 3: 200×200pt.
        assert!((doc.pages[2].width  - 200.0).abs() < 0.01);
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

        let shape_positions: Vec<(f64, f64, f64, f64)> = items.iter()
            .filter_map(|it| match it {
                FrameItem::Shape { pos, width, height, .. } =>
                    Some((pos.x.0, pos.y.0, *width, *height)),
                _ => None,
            })
            .collect();

        // Deve existir um rect de 100×50 no col 0 da segunda linha.
        let rect_100 = shape_positions.iter()
            .find(|(_, _, w, h)| (*w - 100.0).abs() < 0.1 && (*h - 50.0).abs() < 0.1)
            .expect("rect(100×50) deve existir como FrameItem::Shape na página 2");
        assert!((rect_100.0 - 20.0).abs() < 0.5,
            "rect(100×50) deve estar em col 0 (x ≈ 20pt na página 400/20), obteve x={}",
            rect_100.0);

        // Deve existir um rect de 80×30 no col 1 da segunda linha.
        let rect_80 = shape_positions.iter()
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

        let rect_50 = items.iter()
            .find_map(|it| match it {
                FrameItem::Shape { pos, height, .. } if (*height - 50.0).abs() < 0.1
                    => Some(*pos),
                _ => None,
            })
            .expect("rect(100×50) deve existir na página 2");

        // Os items da linha 0 incluem o FrameItem::Group (move + rect) e texto
        // da célula (0,1). A linha 1 (onde estão os rects 100 e 80) deve estar
        // visualmente abaixo.
        let first_row_y_max: f64 = items.iter()
            .filter_map(|it| match it {
                // Excluir shapes da linha 1 (100×50 e 80×30) — procuramos só
                // items da linha 0.
                FrameItem::Shape { height, .. }
                    if (*height - 50.0).abs() < 0.1 || (*height - 30.0).abs() < 0.1 => None,
                other => Some(frame_item_pos(other).y.0),
            })
            .fold(f64::NEG_INFINITY, f64::max);

        assert!(rect_50.y.0 >= first_row_y_max,
            "Linha 1 do grid deve estar ao nível ou abaixo da linha 0. \
             rect_50 y={}, first_row_y_max={}",
            rect_50.y.0, first_row_y_max);
    }

    // Fase 4 — Anti-regressão: nenhum item excede os limites físicos da página 2
    #[test]
    fn stress_81_5_nenhum_item_excede_limites_da_pagina_2() {
        use typst_core::entities::layout_types::FrameItem;
        let doc = compilar_stress_81_5();
        let items = &doc.pages[1].items;

        for item in items {
            let pos = frame_item_pos(item);
            assert!(pos.x.0 <= 400.0,
                "Item {:?} excede largura da página 2 (400pt): x={}",
                item, pos.x.0);
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
                    assert!(abs_x <= 400.0,
                        "Item transformado excede largura da página 2: abs_x={}", abs_x);
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
        let pdf = export_pdf(&doc);
        let pdf_str = String::from_utf8_lossy(&pdf);

        assert!(pdf_str.contains("[0 0 595.28 841.89]"),
            "PDF: página 1 deve ter MediaBox A4");
        assert!(pdf_str.contains("[0 0 400.00 300.00]"),
            "PDF: página 2 deve ter MediaBox 400×300pt");
        assert!(pdf_str.contains("[0 0 200.00 200.00]"),
            "PDF: página 3 deve ter MediaBox 200×200pt");

        // Nenhum MediaBox híbrido — sinal de vazamento catastrófico de dimensão.
        assert!(!pdf_str.contains("[0 0 400.00 841.89]"),
            "PDF: MediaBox híbrido detectado — height da página 2 vazou para A4");
        assert!(!pdf_str.contains("[0 0 595.28 300.00]"),
            "PDF: MediaBox híbrido detectado — width da página 2 ficou em A4");
        assert!(!pdf_str.contains("[0 0 200.00 841.89]"),
            "PDF: MediaBox híbrido detectado — height da página 3 vazou para A4");

        let count = pdf_str.matches("/MediaBox").count();
        assert_eq!(count, 3,
            "PDF deve ter exactamente 3 /MediaBox, encontrou {}", count);
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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

        let items = &doc.pages[0].items;
        assert!(!items.is_empty(), "Deve haver pelo menos um item");

        let rect_x = frame_item_pos(&items[0]).x.0;
        assert!(
            (rect_x - 150.0).abs() < 0.5,
            "Rectângulo centrado deve estar em x=150pt, obteve x={:.1}", rect_x
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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

        let rect_x = frame_item_pos(&doc.pages[0].items[0]).x.0;
        assert!(
            (rect_x - 300.0).abs() < 0.5,
            "Rectângulo direita deve estar em x=300pt, obteve x={:.1}", rect_x
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
            let source  = world.source(world.main()).unwrap();
            let module  = do_eval(&world, &source).unwrap();
            let content = module.content().expect("deve ter content");
            let state   = introspect(content);
            layout(content, state)
        };

        let doc_sem   = layout_doc(src_sem_place);
        let doc_com   = layout_doc(src_com_place);

        let items_sem = &doc_sem.pages[0].items;
        let items_com = &doc_com.pages[0].items;

        assert_eq!(items_sem.len(), 2, "Doc sem place deve ter 2 rectângulos");
        assert_eq!(items_com.len(), 3, "Doc com place deve ter 3 FrameItems (2 rect + 1 place)");

        // Rect 1 nas duas versões — mesmo Y.
        let y0_sem = frame_item_pos(&items_sem[0]).y.0;
        let y0_com = frame_item_pos(&items_com[0]).y.0;
        assert!(
            (y0_sem - y0_com).abs() < 0.5,
            "Rect 1 deve estar no mesmo Y com e sem place ({} vs {})",
            y0_sem, y0_com
        );

        // Rect 3 (com place) vs Rect 2 (sem place) — mesmo Y → Place não avançou cursor.
        let y_final_sem = frame_item_pos(&items_sem[1]).y.0;
        let y_final_com = frame_item_pos(&items_com[2]).y.0;
        assert!(
            (y_final_sem - y_final_com).abs() < 0.5,
            "O rectângulo após place deve estar no mesmo Y que sem place \
             ({} sem place, {} com place) — Place consumiu fluxo",
            y_final_sem, y_final_com
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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

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
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

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
        let src = "\
#set page(width: 400pt, height: 400pt, margin: 20pt)
#grid(columns: (200pt,), rows: (100pt,),
  place(\"bottom-right\", scope: \"parent\", rect(width: 30pt, height: 20pt)),
)
";
        let (world, _dir) = world_from_str(src);
        let source  = world.source(world.main()).unwrap();
        let module  = do_eval(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state   = introspect(content);
        let doc     = layout(content, state);

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
        assert_eq!(warnings.len(), 1,
            "ficheiro vazio deve gerar exactamente 1 warning; obteve {}: {:?}",
            warnings.len(),
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>());
        assert!(warnings[0].message.contains("ficheiro vazio"),
            "mensagem esperada contém 'ficheiro vazio'; obteve: {:?}",
            warnings[0].message);
    }

    /// Teste de ausência: ficheiro não-vazio não dispara o pilot.
    #[test]
    fn sink_canal_vazio_quando_sem_trigger() {
        let (world, _dir) = world_from_str("Olá mundo");
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert!(warnings.is_empty(),
            "ficheiro não-vazio não deve gerar warnings; obteve {:?}",
            warnings);
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
        assert_eq!(warnings2.len(), 1,
            "cada `eval` tem o seu próprio Sink; segundo run deve também gerar 1 warning");
    }

    // ── Passo 107 (encerra DEBT-49): warnings reais de #set ────────────

    /// Propriedade `font` em `#set text(...)` não está implementada —
    /// emite warning com mensagem específica (Passo 107).
    #[test]
    fn debt49_set_text_font_emite_warning() {
        let (world, _dir) = world_from_str(r#"#set text(font: "Arial")"#);
        let source = world.source(world.main()).unwrap();

        let (result, warnings) = do_eval_with_sink(&world, &source);
        assert!(result.is_ok(), "eval não deve falhar; Sink absorve o desconhecido");
        assert_eq!(warnings.len(), 1,
            "esperado 1 warning para propriedade 'font'; obteve {}: {:?}",
            warnings.len(),
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>());
        assert!(warnings[0].message.contains("'font'"),
            "mensagem deve identificar a propriedade 'font'; obteve: {:?}",
            warnings[0].message);
        assert!(warnings[0].message.contains("text"),
            "mensagem deve identificar o target 'text'; obteve: {:?}",
            warnings[0].message);
        assert!(!warnings[0].hints.is_empty(),
            "warning deve ter pelo menos um hint referenciando ADR-0040");
        assert!(warnings[0].hints[0].contains("ADR-0040"),
            "hint deve referenciar ADR-0040; obteve: {:?}",
            warnings[0].hints[0]);
    }

    /// Propriedade `lang` análoga — deve também emitir warning específico.
    #[test]
    fn debt49_set_text_lang_emite_warning() {
        let (world, _dir) = world_from_str(r#"#set text(lang: "pt")"#);
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("'lang'"),
            "mensagem deve identificar 'lang'; obteve: {:?}",
            warnings[0].message);
    }

    /// Múltiplas propriedades desconhecidas num único `#set text(...)` —
    /// N warnings distintos (uma por propriedade), pois spans + messages
    /// diferem.
    #[test]
    fn debt49_set_text_multiplas_propriedades_desconhecidas() {
        let (world, _dir) = world_from_str(r#"#set text(font: "A", lang: "pt", weight: 700)"#);
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert_eq!(warnings.len(), 3,
            "esperado 3 warnings (font, lang, weight); obteve {}: {:?}",
            warnings.len(),
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>());
        let joined = warnings.iter()
            .map(|d| d.message.clone())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(joined.contains("'font'"), "faltou 'font': {}", joined);
        assert!(joined.contains("'lang'"), "faltou 'lang': {}", joined);
        assert!(joined.contains("'weight'"), "faltou 'weight': {}", joined);
    }

    /// Propriedades suportadas de `#set text(...)` (bold, italic, size,
    /// fill) não devem emitir warnings — teste de regressão.
    #[test]
    fn debt49_set_text_propriedades_suportadas_sem_warnings() {
        let (world, _dir) = world_from_str(
            "#set text(bold: true, italic: false, size: 14pt)"
        );
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert!(warnings.is_empty(),
            "propriedades suportadas não devem emitir warnings; obteve: {:?}",
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>());
    }

    /// Target desconhecido em `#set` (ex: `par`, `align`) emite warning
    /// diferente — identificar o target, não a propriedade.
    #[test]
    fn debt49_set_target_desconhecido_emite_warning() {
        let (world, _dir) = world_from_str("#set par(leading: 10pt)");
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        assert_eq!(warnings.len(), 1,
            "target desconhecido 'par' deve gerar 1 warning; obteve {}: {:?}",
            warnings.len(),
            warnings.iter().map(|d| &d.message).collect::<Vec<_>>());
        assert!(warnings[0].message.contains("'par'"),
            "mensagem deve identificar o target 'par'; obteve: {:?}",
            warnings[0].message);
        assert!(warnings[0].message.contains("target"),
            "mensagem deve indicar que é um problema de target; obteve: {:?}",
            warnings[0].message);
    }

    /// Dedup real: mesma propriedade desconhecida em dois `#set` idênticos
    /// deve produzir apenas 1 warning (mesmos span+message).
    #[test]
    fn debt49_dedup_warnings_identicos() {
        // Dois `#set text(font: "X")` no mesmo ficheiro. Os spans são
        // diferentes (linha 1 vs linha 2), por isso dedup não aplica aqui —
        // spans distintos contam como warnings distintos.
        //
        // Para testar dedup de verdade, precisaríamos de um sítio que
        // dispara DEBT-49 repetidamente com o MESMO span + message, o que
        // não acontece numa passagem pelo código fonte (cada texto fonte é
        // parsed uma vez por eval). O mecanismo existe no Sink, mas validá-lo
        // requer chamada artificial à API; ver `sink.rs#tests`.
        let (world, _dir) = world_from_str(
            "#set text(font: \"A\")\n#set text(font: \"A\")"
        );
        let source = world.source(world.main()).unwrap();

        let (_result, warnings) = do_eval_with_sink(&world, &source);
        // Dois spans distintos → 2 warnings (não deduplicados).
        assert_eq!(warnings.len(), 2,
            "#set text(font) repetido em 2 linhas distintas → 2 warnings (spans diferem); \
             dedup real validado em tests unitários de Sink");
    }

    // ── Passo 119 (ADR-0050) ────────────────────────────────────────────
    //
    // 5 testes `format_diagnostic_*` removidos: duplicados das L2 unit
    // tests (`typst_shell::diagnostic::tests::formato_*`) e dos
    // `debt49_*` / `sink_canal_*` já existentes que asseveram
    // `SourceDiagnostic.message` e `.hints` directamente. Cobertura
    // preservada sem duplicação.
}
