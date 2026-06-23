//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/loading.md
//! @prompt-hash 3d5d7f23
//! @layer L1
//! @updated 2026-06-21
//!
//! Módulo `loading` — carregamento de dados (`read`/`csv`/`json`/`yaml`/`toml`/
//! `cbor`/`xml`). Achado da Lista B do Passo 386; materializado no Passo 387.
//!
//! Arquitetura (ADR-0029 + ADR-0111): o **decode é L1 puro** — `decode_X(bytes,
//! opts) -> SourceResult<Value>`, zero I/O, testável com bytes literais. A
//! **leitura** reusa o L3 já existente `World::read_bytes`. As `native_X`
//! compõem os dois estratos. Paridade é com o `Value` de saída (ADR-0107), não
//! com a mecânica do parser.

use crate::entities::args::Args;
use crate::entities::bytes::Bytes;
use crate::entities::file_id::FileId;
use crate::entities::span::Span;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;
use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

type Dict = IndexMap<EcoString, Value, FxBuildHasher>;

fn err(msg: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(Span::detached(), msg.into())]
}

fn new_dict() -> Dict {
    IndexMap::with_hasher(FxBuildHasher)
}

// ─────────────────────────────────────────────────────────────────────────────
// JSON — serde_json (preserve_order: dict ordem-de-inserção)
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_json(bytes: &[u8]) -> SourceResult<Value> {
    let v: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|e| err(format!("json inválido: {e}")))?;
    Ok(json_to_value(v))
}

fn json_to_value(v: serde_json::Value) -> Value {
    use serde_json::Value as J;
    match v {
        J::Null => Value::None,
        J::Bool(b) => Value::Bool(b),
        J::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else {
                // f64 ou inteiro fora de i64 → Float (paridade: número grande vira float)
                Value::Float(n.as_f64().unwrap_or(f64::NAN))
            }
        }
        J::String(s) => Value::Str(s.into()),
        J::Array(a) => Value::Array(a.into_iter().map(json_to_value).collect()),
        J::Object(o) => {
            let mut d = new_dict();
            for (k, val) in o {
                d.insert(k.into(), json_to_value(val));
            }
            Value::Dict(d)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// YAML — saphyr (mantido; mapa Yaml→Value manual). early_parse=true em
// `load_from_str` → escalares chegam como `Yaml::Value(Scalar)`.
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_yaml(bytes: &[u8]) -> SourceResult<Value> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| err("yaml: ficheiro não é UTF-8 válido"))?;
    let docs = <saphyr::Yaml as saphyr::LoadableYamlNode>::load_from_str(s)
        .map_err(|e| err(format!("yaml inválido: {e}")))?;
    match docs.len() {
        0 => Ok(Value::None),
        // documento único → o valor; múltiplos docs (`---`) → Array de docs.
        1 => yaml_to_value(&docs[0]),
        _ => {
            let mut out = Vec::with_capacity(docs.len());
            for d in &docs {
                out.push(yaml_to_value(d)?);
            }
            Ok(Value::Array(out))
        }
    }
}

fn yaml_to_value(y: &saphyr::Yaml) -> SourceResult<Value> {
    use saphyr::{Scalar, Yaml};
    match y {
        Yaml::Value(scalar) => Ok(match scalar {
            Scalar::Null => Value::None,
            Scalar::Boolean(b) => Value::Bool(*b),
            Scalar::Integer(i) => Value::Int(*i),
            Scalar::FloatingPoint(f) => Value::Float(f.into_inner()),
            Scalar::String(s) => Value::Str(s.as_ref().into()),
        }),
        Yaml::Sequence(seq) => {
            let mut out = Vec::with_capacity(seq.len());
            for item in seq {
                out.push(yaml_to_value(item)?);
            }
            Ok(Value::Array(out))
        }
        Yaml::Mapping(map) => {
            let mut d = new_dict();
            for (k, v) in map {
                let key = match yaml_to_value(k)? {
                    Value::Str(s) => s,
                    _ => return Err(err("yaml: chave de mapa não-string sem suporte")),
                };
                d.insert(key, yaml_to_value(v)?);
            }
            Ok(Value::Dict(d))
        }
        // Tagged: ignora a tag (graded ADR-0054), decodifica o interior.
        Yaml::Tagged(_, inner) => yaml_to_value(inner),
        Yaml::Representation(..) => Err(err("yaml: escalar não-resolvido (interno)")),
        Yaml::Alias(_) => Err(err("yaml: alias/anchor sem suporte (graded)")),
        Yaml::BadValue => Err(err("yaml: valor inválido")),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TOML — toml (preserve_order). Requer UTF-8.
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_toml(bytes: &[u8]) -> SourceResult<Value> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| err("toml: ficheiro não é UTF-8 válido"))?;
    let v: toml::Value = toml::from_str(s)
        .map_err(|e| err(format!("toml inválido: {e}")))?;
    Ok(toml_to_value(v))
}

fn toml_to_value(v: toml::Value) -> Value {
    use toml::Value as T;
    match v {
        T::String(s) => Value::Str(s.into()),
        T::Integer(i) => Value::Int(i),
        T::Float(f) => Value::Float(f),
        T::Boolean(b) => Value::Bool(b),
        // graded (ADR-0054): mapa rico → Datetime deferido; RFC 3339 como Str.
        T::Datetime(dt) => Value::Str(dt.to_string().into()),
        T::Array(a) => Value::Array(a.into_iter().map(toml_to_value).collect()),
        T::Table(t) => {
            let mut d = new_dict();
            for (k, val) in t {
                d.insert(k.into(), toml_to_value(val));
            }
            Value::Dict(d)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CBOR — ciborium. byte-strings → graded Err (Value::Bytes ausente, DEBT-62).
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_cbor(bytes: &[u8]) -> SourceResult<Value> {
    let v: ciborium::value::Value = ciborium::from_reader(bytes)
        .map_err(|e| err(format!("cbor inválido: {e}")))?;
    cbor_to_value(v)
}

fn cbor_to_value(v: ciborium::value::Value) -> SourceResult<Value> {
    use ciborium::value::Value as C;
    match v {
        C::Null => Ok(Value::None),
        C::Bool(b) => Ok(Value::Bool(b)),
        C::Integer(i) => {
            let n: i128 = i.into();
            i64::try_from(n)
                .map(Value::Int)
                .map_err(|_| err("cbor: inteiro fora de i64"))
        }
        C::Float(f) => Ok(Value::Float(f)),
        C::Text(s) => Ok(Value::Str(s.into())),
        // P398: byte-strings CBOR → Value::Bytes (fecha DEBT-62).
        C::Bytes(b) => Ok(Value::Bytes(Bytes::new(b))),

        C::Tag(_, inner) => cbor_to_value(*inner),
        C::Array(a) => {
            let mut out = Vec::with_capacity(a.len());
            for item in a {
                out.push(cbor_to_value(item)?);
            }
            Ok(Value::Array(out))
        }
        C::Map(m) => {
            let mut d = new_dict();
            for (k, val) in m {
                let key = match cbor_to_value(k)? {
                    Value::Str(s) => s,
                    _ => return Err(err("cbor: chave de mapa não-string sem suporte")),
                };
                d.insert(key, cbor_to_value(val)?);
            }
            Ok(Value::Dict(d))
        }
        _ => Err(err("cbor: variante sem suporte")),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// XML — roxmltree (read-only). Cada elemento → Dict{tag, attrs, children}.
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_xml(bytes: &[u8]) -> SourceResult<Value> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| err("xml: ficheiro não é UTF-8 válido"))?;
    let doc = roxmltree::Document::parse(s)
        .map_err(|e| err(format!("xml inválido: {e}")))?;
    // Topo: o elemento-raiz como nó único, num Array (paridade `convert_xml`).
    Ok(Value::Array(vec![xml_node_to_value(doc.root_element())]))
}

fn xml_node_to_value(node: roxmltree::Node) -> Value {
    let mut d = new_dict();
    d.insert("tag".into(), Value::Str(node.tag_name().name().into()));

    let mut attrs = new_dict();
    for a in node.attributes() {
        attrs.insert(a.name().into(), Value::Str(a.value().into()));
    }
    d.insert("attrs".into(), Value::Dict(attrs));

    let mut children = Vec::new();
    for child in node.children() {
        if child.is_element() {
            children.push(xml_node_to_value(child));
        } else if child.is_text() {
            if let Some(t) = child.text() {
                children.push(Value::Str(t.into()));
            }
        }
    }
    d.insert("children".into(), Value::Array(children));
    Value::Dict(d)
}

// ─────────────────────────────────────────────────────────────────────────────
// CSV — csv. Array 2D; row-type array (default) ou dictionary.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
pub enum RowType {
    Array,
    Dictionary,
}

pub fn decode_csv(bytes: &[u8], delimiter: u8, row_type: RowType) -> SourceResult<Value> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .flexible(true)
        .from_reader(bytes);

    let mut records = reader.records();

    // row-type dictionary: a 1ª linha são as chaves (paridade vanilla).
    let header: Option<Vec<EcoString>> = if row_type == RowType::Dictionary {
        match records.next() {
            Some(r) => {
                let r = r.map_err(|e| err(format!("csv inválido: {e}")))?;
                Some(r.iter().map(EcoString::from).collect())
            }
            None => None,
        }
    } else {
        None
    };

    let mut rows = Vec::new();
    for rec in records {
        let rec = rec.map_err(|e| err(format!("csv inválido: {e}")))?;
        match &header {
            None => {
                rows.push(Value::Array(rec.iter().map(|c| Value::Str(c.into())).collect()));
            }
            Some(keys) => {
                let mut d = new_dict();
                for (i, cell) in rec.iter().enumerate() {
                    let key = keys.get(i).cloned().unwrap_or_else(|| EcoString::from(i.to_string()));
                    d.insert(key, Value::Str(cell.into()));
                }
                rows.push(Value::Dict(d));
            }
        }
    }
    Ok(Value::Array(rows))
}

// ─────────────────────────────────────────────────────────────────────────────
// Funções nativas — compõem L3 (read_bytes) + L1 (decode). ABI per ADR-0044:
// (ctx, args, world, current_file). Ver `figure_image::native_image`.
// ─────────────────────────────────────────────────────────────────────────────

fn arg_path(args: &Args, fname: &str) -> SourceResult<String> {
    match args.items.first() {
        Some(Value::Str(s)) => Ok(s.to_string()),
        Some(other) => Err(err(format!(
            "{fname}() requer string com o caminho, recebeu {}",
            other.type_name()
        ))),
        None => Err(err(format!("{fname}() requer 1 argumento posicional (caminho)"))),
    }
}

fn reject_named(args: &Args, fname: &str) -> SourceResult<()> {
    if let Some(k) = args.named.keys().next() {
        return Err(err(format!("argumento nomeado inesperado em {fname}(): '{k}'")));
    }
    Ok(())
}

fn read_bytes(
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
    path: &str,
    fname: &str,
) -> SourceResult<std::sync::Arc<Vec<u8>>> {
    world
        .read_bytes(current_file, path)
        .map_err(|msg| err(format!("{fname}(): não foi possível ler '{path}': {msg}")))
}

/// `read(path)` → `Str` (utf8) ou `Bytes` (binário).
/// Heurística vanilla: tenta UTF-8; se falhar, retorna bytes opacos.
pub fn native_read(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    reject_named(args, "read")?;
    let path = arg_path(args, "read")?;
    let data = read_bytes(world, current_file, &path, "read")?;
    match String::from_utf8(data.to_vec()) {
        Ok(text) => Ok(Value::Str(text.into())),
        Err(_) => Ok(Value::Bytes(Bytes::new(data.to_vec()))),
    }
}

macro_rules! native_loader {
    ($fn_name:ident, $name:literal, $decode:expr) => {
        pub fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            world: &dyn crate::contracts::world::World,
            current_file: FileId,
        ) -> SourceResult<Value> {
            reject_named(args, $name)?;
            let path = arg_path(args, $name)?;
            let data = read_bytes(world, current_file, &path, $name)?;
            $decode(&data[..])
        }
    };
}

native_loader!(native_json, "json", decode_json);
native_loader!(native_yaml, "yaml", decode_yaml);
native_loader!(native_toml, "toml", decode_toml);
native_loader!(native_cbor, "cbor", decode_cbor);
native_loader!(native_xml, "xml", decode_xml);

/// `csv(path, delimiter: ",", row-type: "array")`. Aceita named `delimiter` e
/// `row-type` (paridade vanilla: subset graded — sem `escape`/`encoding` cosméticos).
pub fn native_csv(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    for k in args.named.keys() {
        if k.as_str() != "delimiter" && k.as_str() != "row-type" {
            return Err(err(format!("argumento nomeado inesperado em csv(): '{k}'")));
        }
    }
    let path = arg_path(args, "csv")?;

    let delimiter = match args.named.get("delimiter") {
        None => b',',
        Some(Value::Str(s)) => {
            let bytes = s.as_bytes();
            if bytes.len() != 1 {
                return Err(err("csv(): delimiter deve ser um único carácter"));
            }
            bytes[0]
        }
        Some(other) => {
            return Err(err(format!("csv(): delimiter deve ser string, recebeu {}", other.type_name())))
        }
    };

    let row_type = match args.named.get("row-type") {
        None => RowType::Array,
        Some(Value::Str(s)) => match s.as_str() {
            "array" => RowType::Array,
            "dictionary" => RowType::Dictionary,
            other => return Err(err(format!("csv(): row-type inválido '{other}' (array|dictionary)"))),
        },
        Some(other) => {
            return Err(err(format!("csv(): row-type deve ser string, recebeu {}", other.type_name())))
        }
    };

    let data = read_bytes(world, current_file, &path, "csv")?;
    decode_csv(&data[..], delimiter, row_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dict_of(pairs: Vec<(&str, Value)>) -> Value {
        let mut d = new_dict();
        for (k, v) in pairs {
            d.insert(k.into(), v);
        }
        Value::Dict(d)
    }

    // ── JSON ────────────────────────────────────────────────────────────────
    #[test]
    fn json_objeto_array_primitivos() {
        let v = decode_json(br#"{"a":1,"b":[true,null,"x"],"c":3.5}"#).unwrap();
        assert_eq!(
            v,
            dict_of(vec![
                ("a", Value::Int(1)),
                ("b", Value::Array(vec![Value::Bool(true), Value::None, Value::Str("x".into())])),
                ("c", Value::Float(3.5)),
            ])
        );
    }

    #[test]
    fn json_escalar_e_erro() {
        assert_eq!(decode_json(b"3.5").unwrap(), Value::Float(3.5));
        assert_eq!(decode_json(br#""x""#).unwrap(), Value::Str("x".into()));
        assert!(decode_json(b"{bad").is_err());
    }

    #[test]
    fn json_dict_preserva_ordem_de_insercao() {
        let v = decode_json(br#"{"z":1,"a":2,"m":3}"#).unwrap();
        if let Value::Dict(d) = v {
            let keys: Vec<&str> = d.keys().map(|k| k.as_str()).collect();
            assert_eq!(keys, vec!["z", "a", "m"], "ordem de inserção, não alfabética");
        } else {
            panic!("esperava Dict");
        }
    }

    // ── YAML ────────────────────────────────────────────────────────────────
    #[test]
    fn yaml_mapping_e_sequence() {
        let v = decode_yaml(b"a: 1\nb: hello\n").unwrap();
        assert_eq!(v, dict_of(vec![("a", Value::Int(1)), ("b", Value::Str("hello".into()))]));
        let v2 = decode_yaml(b"- 1\n- 2\n").unwrap();
        assert_eq!(v2, Value::Array(vec![Value::Int(1), Value::Int(2)]));
    }

    #[test]
    fn yaml_tipos_e_null() {
        let v = decode_yaml(b"x: true\ny: 3.5\nz: null\n").unwrap();
        assert_eq!(
            v,
            dict_of(vec![
                ("x", Value::Bool(true)),
                ("y", Value::Float(3.5)),
                ("z", Value::None),
            ])
        );
    }

    // ── TOML ────────────────────────────────────────────────────────────────
    #[test]
    fn toml_table_tipos() {
        let v = decode_toml(b"x = 1\ny = \"s\"\nf = 1.5\n").unwrap();
        assert_eq!(
            v,
            dict_of(vec![
                ("x", Value::Int(1)),
                ("y", Value::Str("s".into())),
                ("f", Value::Float(1.5)),
            ])
        );
    }

    #[test]
    fn toml_datetime_graded_str() {
        // graded (ADR-0054 / DEBT-62 contexto): datetime → Str RFC 3339.
        let v = decode_toml(b"d = 1979-05-27\n").unwrap();
        if let Value::Dict(d) = v {
            assert_eq!(d.get("d"), Some(&Value::Str("1979-05-27".into())));
        } else {
            panic!("esperava Dict");
        }
    }

    // ── CBOR ────────────────────────────────────────────────────────────────
    fn cbor_bytes(v: &ciborium::value::Value) -> Vec<u8> {
        let mut out = Vec::new();
        ciborium::into_writer(v, &mut out).unwrap();
        out
    }

    #[test]
    fn cbor_tree() {
        use ciborium::value::Value as C;
        let doc = C::Map(vec![(C::Text("a".into()), C::Integer(1.into()))]);
        let v = decode_cbor(&cbor_bytes(&doc)).unwrap();
        assert_eq!(v, dict_of(vec![("a", Value::Int(1))]));
    }

    #[test]
    fn cbor_byte_string_returns_bytes() {
        use ciborium::value::Value as C;
        let doc = C::Bytes(vec![1, 2, 3]);
        let v = decode_cbor(&cbor_bytes(&doc)).unwrap();
        assert_eq!(v, Value::Bytes(Bytes::new(vec![1, 2, 3])));
    }

    // ── CSV ─────────────────────────────────────────────────────────────────
    #[test]
    fn csv_array_default() {
        let v = decode_csv(b"a,b\n1,2", b',', RowType::Array).unwrap();
        assert_eq!(
            v,
            Value::Array(vec![
                Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
                Value::Array(vec![Value::Str("1".into()), Value::Str("2".into())]),
            ])
        );
    }

    #[test]
    fn csv_dictionary_e_delimiter() {
        let v = decode_csv(b"a;b\n1;2", b';', RowType::Dictionary).unwrap();
        assert_eq!(
            v,
            Value::Array(vec![dict_of(vec![
                ("a", Value::Str("1".into())),
                ("b", Value::Str("2".into())),
            ])])
        );
    }

    // ── XML ─────────────────────────────────────────────────────────────────
    #[test]
    fn xml_nodes() {
        let v = decode_xml(b"<r><c>t</c></r>").unwrap();
        let inner_c = dict_of(vec![
            ("tag", Value::Str("c".into())),
            ("attrs", Value::Dict(new_dict())),
            ("children", Value::Array(vec![Value::Str("t".into())])),
        ]);
        let root = dict_of(vec![
            ("tag", Value::Str("r".into())),
            ("attrs", Value::Dict(new_dict())),
            ("children", Value::Array(vec![inner_c])),
        ]);
        assert_eq!(v, Value::Array(vec![root]));
    }

    #[test]
    fn xml_atributos() {
        let v = decode_xml(br#"<a href="x"/>"#).unwrap();
        if let Value::Array(nodes) = v {
            if let Value::Dict(d) = &nodes[0] {
                assert_eq!(
                    d.get("attrs"),
                    Some(&dict_of(vec![("href", Value::Str("x".into()))]))
                );
            } else {
                panic!("esperava Dict");
            }
        } else {
            panic!("esperava Array");
        }
    }

    // ── read() texto vs binário — P398 ───────────────────────────────────────

    use std::num::NonZeroU16;
    use std::sync::Arc;

    struct MockWorld {
        files: std::collections::HashMap<String, Arc<Vec<u8>>>,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl Default for MockWorld {
        fn default() -> Self {
            Self {
                files: std::collections::HashMap::new(),
                library: crate::entities::world_types::Library::default(),
                book: crate::entities::font_book::FontBook::default(),
            }
        }
    }
    impl crate::contracts::world::World for MockWorld {
        fn library(&self) -> &crate::entities::world_types::Library { &self.library }
        fn book(&self) -> &crate::entities::font_book::FontBook { &self.book }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> crate::entities::world_types::FileResult<crate::entities::source::Source> {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(&self, _: FileId) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes> {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> { None }
        fn today(&self, _: Option<i64>) -> Option<crate::entities::world_types::Datetime> { None }
        fn read_bytes(&self, _current_file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String> {
            self.files.get(path).cloned().ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
    }

    fn mock_args(path: &str) -> Args {
        Args::positional(vec![Value::Str(path.into())])
    }

    #[test]
    fn read_texto_utf8() {
        let mut world = MockWorld::default();
        world.files.insert("texto.txt".into(), Arc::new(b"hello".to_vec()));
        let v = native_read(&mut EvalContext::new(), &mock_args("texto.txt"), &world, FileId::from_raw(NonZeroU16::new(1).unwrap())).unwrap();
        assert_eq!(v, Value::Str("hello".into()));
    }

    #[test]
    fn read_binario_nao_utf8() {
        let mut world = MockWorld::default();
        world.files.insert("logo.png".into(), Arc::new(vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]));
        let v = native_read(&mut EvalContext::new(), &mock_args("logo.png"), &world, FileId::from_raw(NonZeroU16::new(1).unwrap())).unwrap();
        assert_eq!(v, Value::Bytes(Bytes::new(vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])));
    }

    #[test]
    fn read_vazio() {
        let mut world = MockWorld::default();
        world.files.insert("empty".into(), Arc::new(Vec::new()));
        let v = native_read(&mut EvalContext::new(), &mock_args("empty"), &world, FileId::from_raw(NonZeroU16::new(1).unwrap())).unwrap();
        assert_eq!(v, Value::Str("".into()));
    }
}
