from pathlib import Path
import subprocess,json
source='03_infra/src/integration_tests.rs'
subprocess.run(['python3','/tmp/tekt-native-spec.py',source,'A suíte completa em workspace.log falhou na fixture que exigia fallback Color sem size (linha 1686). Migrar o cenário válido para Color com size explícito, exigir ausência de erros e PatternType 1/scn além do retângulo e cor vermelha dentro da célula. Preservar o cenário original sem size como teste de diagnóstico causal exigido pelo preparador. Não aceitar saída inválida por simples ausência de cor nem remover cobertura de integração.'],check=True)
p=Path(source);s=p.read_text();a=s.index('    #[test]\n    fn rect_fill_tiling_cai_no_fallback_color_no_pdf()');b=s.index('    #[test]',a+10)
s=s[:a]+'''    #[test]
    fn rect_fill_tiling_emite_celula_pdf_sem_fallback() {
        let (world, _dir) = world_from_str(
            "#rect(width: 50pt, height: 30pt, fill: tiling(rgb(255,0,0), size: (4pt, 4pt)))",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let doc = layout(module.content().expect("deve ter content"));
        assert!(doc.layout_errors.is_empty(), "{:?}", doc.layout_errors);
        let pdf = export_pdf(&doc, StreamMode::Verbose);
        let pdf_str = String::from_utf8_lossy(&pdf);
        assert!(pdf_str.contains("/PatternType 1"));
        assert!(pdf_str.contains("/Pattern cs") && pdf_str.contains(" scn\\n"));
        assert!(pdf_str.contains(" re\\n") && pdf_str.contains("f\\n"));
        assert!(pdf_str.contains("1.000 0.000 0.000 rg\\n"), "célula mantém vermelho puro");
    }

    #[test]
    fn rect_fill_tiling_de_cor_sem_size_preserva_diagnostico() {
        let (world, _dir) = world_from_str(
            "#rect(width: 50pt, height: 30pt, fill: tiling(rgb(255,0,0)))",
        );
        let source = world.source(world.main()).unwrap();
        let module = do_eval(&world, &source).unwrap();
        let doc = layout(module.content().expect("deve ter content"));
        assert_eq!(doc.layout_errors.len(), 1);
        assert_eq!(doc.layout_errors[0].message.as_str(), "tile size must be explicit for a color or gradient body");
    }

'''+s[b:];p.write_text(s)
p=Path('00_nucleo/diagnosticos/tiling-nativo-2026-10-14/owned-sources.json');v=json.loads(p.read_text());v.append(source);p.write_text(json.dumps(sorted(set(v)),indent=2))
