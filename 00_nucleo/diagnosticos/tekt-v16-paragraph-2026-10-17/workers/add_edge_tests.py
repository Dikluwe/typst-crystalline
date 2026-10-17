from pathlib import Path
import json,hashlib,datetime
out=Path(__file__).resolve().parents[1];p=Path('01_core/src/compiler/layout/paragraph.rs');s=p.read_text();assert s.endswith('}\n');s=s[:-2]+'''
    #[test]
    fn v16_paragraph_empty_text_uses_local_font_size() {
        for (size, baseline) in [(8, 28.4), (18, 35.4)] {
            let doc = language(&format!(r#"#par[#text(size:{size}pt, "")]C"#));
            assert!((positions(&doc)[0].2 - baseline).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_empty_text_uses_local_top_edge() {
        for (edge, baseline) in [(0, 22.8), (-5, 22.8), (20, 42.8)] {
            let doc = language(&format!(r#"#par[#text(top-edge:{edge}pt, "")]C"#));
            assert!((positions(&doc)[0].2 - baseline).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_empty_text_keeps_local_descent() {
        let doc = language(r#"#par[#text(bottom-edge:-10pt, "")]C"#);
        assert!((positions(&doc)[0].2 - 41.2).abs() < 1e-8);
        let only = language(r#"#par[#text(bottom-edge:-10pt, "")]"#);
        assert!((only.pages[0].height - 18.4).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_empty_text_combines_edges_from_siblings() {
        let doc = language(r#"#par[#text(top-edge:20pt, "")#text(bottom-edge:-10pt, "")]C"#);
        assert!((positions(&doc)[0].2 - 52.8).abs() < 1e-8);
    }
}
''';p.write_text(s);r=json.load(open(out/'margin-tests-before-code.json'));r['files'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest();r['edge_tests_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(out/'margin-tests-before-code.json').write_text(json.dumps(r,indent=2)+'\n')
