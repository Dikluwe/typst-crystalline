from pathlib import Path
import subprocess,json
out=Path('00_nucleo/diagnosticos/tekt-enums-2026-10-16');record=dict(before={},after={})
def add(path,anchor,test):
 p=Path('01_core/src/entities/'+path+'.rs');old=p.read_text();assert old.count(anchor)==1;p.write_text(old.replace(anchor,test+anchor));record['before'][str(p)]=old
add('color','    // ── sRGB ──', '''    #[test]
    fn v16_color_equality_keeps_spaces_bits_and_spot_identity() {
        let colors = [
            Color::srgb_f32(0.5, 0.5, 0.5, 1.0), Color::luma(0.5),
            Color::linear_rgb(0.5, 0.5, 0.5, 1.0), Color::oklab(0.5, 0.0, 0.0, 1.0),
            Color::oklch(0.5, 0.0, 0.0, 1.0), Color::hsl(0.0, 0.0, 0.5, 1.0),
            Color::hsv(0.0, 0.0, 0.5, 1.0), Color::cmyk(0.0, 0.0, 0.0, 0.5),
            Color::Spot(SpotColor {
                colorant: Arc::new(SpotColorant {
                    name: Some("ink".into()), fallback: Color::rgb(0, 0, 0),
                }), tint: Ratio(0.5),
            }),
        ];
        for (i, left) in colors.iter().enumerate() {
            for (j, right) in colors.iter().enumerate() {
                assert_eq!(left == right, i == j, "spaces {i}/{j}");
            }
            assert_eq!(left, &left.clone());
        }
        assert_ne!(Color::srgb_f32(0.0, 0.0, 0.0, 1.0), Color::srgb_f32(-0.0, 0.0, 0.0, 1.0));
        let nan = Color::srgb_f32(f32::NAN, 0.0, 0.0, 1.0);
        assert_eq!(nan, nan.clone());
    }

''')
add('math_style','    #[test]\n    fn kind_is_size_variant()', '''    #[test]
    fn v16_math_style_examples_and_size_variants_keep_morphology() {
        assert_eq!(map_glyph('x', MathStyleKind::SansSerif, false, false), '\\u{1D5D1}');
        assert_eq!(map_glyph('k', MathStyleKind::Fraktur, false, false), '\\u{1D528}');
        assert_eq!(map_glyph('x', MathStyleKind::SansSerif, true, true), '\\u{1D66D}');
        for kind in [MathStyleKind::Script, MathStyleKind::SScript, MathStyleKind::Display, MathStyleKind::Inline] {
            for (bold, italic) in [(false,false), (false,true), (true,false), (true,true)] {
                assert_eq!(map_glyph('A', kind, bold, italic), 'A');
                assert_eq!(map_glyph('5', kind, bold, italic), '5');
                assert_eq!(map_glyph_vs('A', kind), None);
            }
        }
    }

''')
add('state_update','    #[test]\n    fn set_round_trip()', '''    #[test]
    fn v16_state_update_function_identity_remains_stricter_than_func_names() {
        let first = Func::native("same", |_, _, _, _| Ok(Value::None));
        let second = Func::native("same", |_, _, _, _| Ok(Value::None));
        assert_eq!(first, second);
        let first_update = StateUpdate::Func(first.clone());
        let alias = StateUpdate::Func(first);
        let second_update = StateUpdate::Func(second);
        assert_eq!(first_update, alias);
        assert_ne!(first_update, second_update);
        assert_ne!(second_update, first_update);
        let set = StateUpdate::Set(Box::new(Value::None));
        assert_ne!(set, first_update);
        assert_ne!(first_update, set);
    }

''')
add('func','    #[test]\n    fn func_debug_nao_panicar()', '''    #[test]
    fn v16_func_representations_keep_identity_hash_and_metadata_without_calls() {
        use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
        use crate::entities::bytes::Bytes;
        use crate::entities::content::Content;
        use crate::entities::plugin_func::PluginFunc;
        struct UncalledHost;
        impl PluginHost for UncalledHost {
            fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> { panic!("metadata called plugin host") }
            fn exports(&self, _: PluginModuleId) -> Result<Vec<ecow::EcoString>, PluginError> { panic!("metadata called plugin host") }
            fn call(&self, _: PluginModuleId, _: &str, _: &[Bytes]) -> Result<Bytes, PluginError> { panic!("metadata called plugin host") }
            fn transition(&self, _: PluginModuleId, _: &str, _: &[Bytes]) -> Result<PluginModuleId, PluginError> { panic!("metadata called plugin host") }
        }
        fn fingerprint(func: &Func) -> u64 {
            let mut state = std::collections::hash_map::DefaultHasher::new();
            func.hash(&mut state); state.finish()
        }
        let mut namespace = Scope::new();
        namespace.define("member", Value::Int(1));
        let namespace = Arc::new(namespace);
        let native = Func::native_with_namespace("same", |_, _, _, _| panic!("metadata called native"), namespace.clone());
        let engine = Func::native_with_engine_and_namespace("same", |_, _, _, _, _, _| panic!("metadata called engine native"), namespace);
        let functions = [
            native.clone(), engine.clone(), make_closure(),
            Func::element("same", Arc::new(|_| -> SourceResult<Content> { panic!("metadata called constructor") })),
            Func::plugin(PluginFunc { host: Arc::new(UncalledHost), module: PluginModuleId(0), name: "same".into() }),
            native.clone().with(Args::positional(vec![])),
        ];
        for (i, left) in functions.iter().enumerate() {
            let alias = left.clone();
            assert_eq!(left, &alias);
            assert_eq!(fingerprint(left), fingerprint(&alias));
            for (j, right) in functions.iter().enumerate() {
                assert_eq!(left == right, i == j, "representations {i}/{j}");
            }
            assert_eq!(left.element_name(), (i == 3).then_some("same"));
            assert_eq!(left.namespace().and_then(|s| s.get("member")).cloned(),
                matches!(i, 0 | 1 | 5).then_some(Value::Int(1)));
        }
        let native_alias = Func::native("same", |_, _, _, _| Ok(Value::None));
        assert_eq!(native, native_alias);
        assert_eq!(fingerprint(&native), fingerprint(&native_alias));
        let engine_alias = Func::native_with_engine("same", |_, _, _, _, _, _| Ok(Value::None));
        assert_eq!(engine, engine_alias);
        assert_eq!(fingerprint(&engine), fingerprint(&engine_alias));
        assert_ne!(functions[5], native.with(Args::positional(vec![])));
    }

''')
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*record['before']],check=True)
record['after']={p:Path(p).read_text() for p in record['before']};(out/'tests-materialization.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print({'new_tests':4,'files':len(record['before'])})
