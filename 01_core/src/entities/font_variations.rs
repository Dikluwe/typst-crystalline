//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/font_variations.md
//! @prompt-hash 40ca74d7
//! @layer L1
//! @updated 2026-07-22
//!
//! P836 (achado #21 de P831) — `FontVariations`: coordenadas de eixo
//! OpenType explícitas de `#text(variations:)` / `#set text(variations:)`.
//!
//! Contraponto de `text/font/variations.rs` do vanilla 0.15.0. A tag é
//! `[u8; 4]` com padding de espaço (paridade `Tag::from_bytes_lossy`,
//! `tag.rs:22-27`) — L1 não depende de `ttf_parser`. A validação
//! (`from_value`) replica verbatim as mensagens e hints do vanilla
//! (`variations.rs:217-236`, `tag.rs:85-117`), medidos em `temp/p836/`.

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::source_result::SourceDiagnostic;
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Lista de coordenadas de eixo de variação `(tag, valor)`, normalizada
/// (ordenada por tag, dedup "later wins" — paridade `normalized()`,
/// `variations.rs:196-208`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FontVariations(pub Vec<([u8; 4], f32)>);

impl FontVariations {
    /// `true` se não houver eixos.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Valida um `Value` vindo do eval (espera `Value::Dict`) e constrói
    /// a lista normalizada. Erros replicam o vanilla verbatim, com o hint
    /// final `occurred in tag at index {i} (`"{key}"`)` por entrada
    /// (`tag_hint_helper`, `variations.rs:233-235`).
    pub fn from_value(value: &Value, span: Span) -> Result<Self, Vec<SourceDiagnostic>> {
        let dict = match value {
            Value::Dict(d) => d,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected dictionary, found {}",
                        vanilla_type_name(other.type_name())
                    ),
                )]);
            }
        };

        let mut entries: Vec<([u8; 4], f32)> = Vec::with_capacity(dict.len());
        for (i, (key, val)) in dict.iter().enumerate() {
            let tag_hint = format!("occurred in tag at index {} (`\"{}\"`)", i, key);
            let tag = match validate_axis_tag(key) {
                Ok(t) => t,
                Err((msg, mut hints)) => {
                    hints.push(tag_hint);
                    let mut diag = SourceDiagnostic::error(span, msg);
                    for h in hints {
                        diag = diag.with_hint(h);
                    }
                    return Err(vec![diag]);
                }
            };
            let value = match val {
                Value::Int(n) => *n as f32,
                Value::Float(f) => *f as f32,
                other => {
                    let diag = SourceDiagnostic::error(
                        span,
                        format!(
                            "expected float, found {}",
                            vanilla_type_name(other.type_name())
                        ),
                    )
                    .with_hint(tag_hint);
                    return Err(vec![diag]);
                }
            };
            entries.push((tag, value));
        }

        Ok(Self(normalize(entries)))
    }

    /// Constrói a partir de um dict já validado no eval (caminho do
    /// resolver da style chain — nunca falha; entradas inesperadas são
    /// ignoradas).
    pub fn from_validated_dict(dict: &IndexMap<EcoString, Value, FxBuildHasher>) -> Self {
        let mut entries: Vec<([u8; 4], f32)> = Vec::with_capacity(dict.len());
        for (key, val) in dict.iter() {
            let value = match val {
                Value::Int(n) => *n as f32,
                Value::Float(f) => *f as f32,
                _ => continue, // neutro: N16[β] — entradas de dict com tag inválida ou valor não-numérico são ignoradas
            };
            if let Ok(tag) = validate_axis_tag(key) {
                entries.push((tag, value));
            }
        }
        Self(normalize(entries))
    }

    /// Fold da style chain (paridade `Fold for FontVariations`,
    /// `variations.rs:210-214`): `self` (interno) vence por tag; tags de
    /// `outer` ausentes em `self` sobrevivem.
    pub fn fold(&self, outer: &FontVariations) -> FontVariations {
        let mut entries = outer.0.clone();
        entries.extend(self.0.iter().copied());
        FontVariations(normalize(entries))
    }
}

/// Ordena por tag e remove duplicados "later wins" (a última ocorrência
/// vence — paridade `rdedup_by_key` após sort estável).
fn normalize(mut entries: Vec<([u8; 4], f32)>) -> Vec<([u8; 4], f32)> {
    entries.sort_by_key(|(tag, _)| *tag);
    let mut out: Vec<([u8; 4], f32)> = Vec::with_capacity(entries.len());
    for (tag, value) in entries {
        if let Some(last) = out.last_mut() {
            if last.0 == tag {
                last.1 = value;
                continue;
            }
        }
        out.push((tag, value));
    }
    out
}

/// Valida uma tag de eixo — paridade do cast de `Tag`
/// (`tag.rs:85-117`). Devolve a tag `[u8; 4]` com padding de espaço,
/// ou `(mensagem, hints)` com os textos verbatim do vanilla.
///
/// Ordem das verificações (como no vanilla): ASCII imprimível →
/// comprimento → espaços só como padding final.
///
/// Divergência registada: o vanilla itera grapheme clusters
/// (unicode-segmentation); o cristalino itera `chars()` (a crate não
/// está na whitelist L1). Difere apenas no hint `found invalid
/// cluster` para sequências combinantes (ex. `e` + U+0301).
fn validate_axis_tag(s: &str) -> Result<[u8; 4], (String, Vec<String>)> {
    // 1. Apenas ASCII imprimível (0x20..=0x7E).
    if let Some(cluster) = s
        .chars()
        .find(|c| !(*c as u32 >= 0x20 && *c as u32 <= 0x7E))
    {
        return Err((
            "tag may contain only printable ASCII characters".to_string(),
            vec![format!("found invalid cluster `\"{}\"`", cluster)],
        ));
    }

    // 2. Comprimento 1..=4 (bytes == chars aqui: ASCII garantido acima).
    if !(1..=4).contains(&s.len()) {
        return Err((
            "tag must be one to four characters in length".to_string(),
            vec![format!("found {} characters", s.len())],
        ));
    }

    // 3. Espaços apenas como padding final.
    let mut within_padding = false;
    for (i, &b) in s.as_bytes().iter().enumerate() {
        if (within_padding && b != b' ') || (i == 0 && b == b' ') {
            return Err((
                "spaces may only appear as padding following a tag".to_string(),
                Vec::new(),
            ));
        }
        within_padding |= b == b' ';
    }

    // Paridade `Tag::from_bytes_lossy` — padding com espaço.
    let mut tag = [b' '; 4];
    tag[..s.len()].copy_from_slice(s.as_bytes());
    Ok(tag)
}

/// Nomes de tipo do vanilla nos erros de cast — o `type_name()`
/// cristalino usa formas curtas (`int`, `str`, `bool`).
fn vanilla_type_name(name: &str) -> &str {
    match name {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dict(entries: &[(&str, Value)]) -> Value {
        let mut d = IndexMap::with_hasher(FxBuildHasher::default());
        for (k, v) in entries {
            d.insert(EcoString::from(*k), v.clone());
        }
        Value::Dict(d)
    }

    #[test]
    fn p836_from_value_valido_normaliza_e_ordena() {
        let v = dict(&[("wght", Value::Int(250)), ("ital", Value::Float(1.0))]);
        let fv = FontVariations::from_value(&v, Span::detached()).unwrap();
        // Ordenado por tag: "ital" < "wght".
        assert_eq!(fv.0, vec![(*b"ital", 1.0), (*b"wght", 250.0)]);
    }

    #[test]
    fn p836_from_value_tag_curta_padding_espaco() {
        let v = dict(&[("w", Value::Int(2))]);
        let fv = FontVariations::from_value(&v, Span::detached()).unwrap();
        assert_eq!(fv.0, vec![([b'w', b' ', b' ', b' '], 2.0)]);
    }

    #[test]
    fn p836_tag5_erro_verbatim() {
        let v = dict(&[("wgght", Value::Int(1))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "tag must be one to four characters in length");
        assert_eq!(
            err[0].hints,
            vec![
                "found 5 characters".to_string(),
                "occurred in tag at index 0 (`\"wgght\"`)".to_string(),
            ]
        );
    }

    #[test]
    fn p836_tag_vazia_erro_verbatim() {
        let v = dict(&[("", Value::Int(1))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "tag must be one to four characters in length");
        assert_eq!(
            err[0].hints,
            vec![
                "found 0 characters".to_string(),
                "occurred in tag at index 0 (`\"\"`)".to_string(),
            ]
        );
    }

    #[test]
    fn p836_tag_nao_ascii_erro_verbatim() {
        let v = dict(&[("wg€t", Value::Int(1))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "tag may contain only printable ASCII characters");
        assert_eq!(
            err[0].hints,
            vec![
                "found invalid cluster `\"€\"`".to_string(),
                "occurred in tag at index 0 (`\"wg€t\"`)".to_string(),
            ]
        );
    }

    #[test]
    fn p836_tag_espaco_interior_erro_verbatim() {
        let v = dict(&[("w g", Value::Int(1))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "spaces may only appear as padding following a tag");
        assert_eq!(
            err[0].hints,
            vec!["occurred in tag at index 0 (`\"w g\"`)".to_string()]
        );
    }

    #[test]
    fn p836_tag_espaco_padding_final_aceite() {
        let v = dict(&[("w  ", Value::Int(3))]);
        let fv = FontVariations::from_value(&v, Span::detached()).unwrap();
        assert_eq!(fv.0, vec![([b'w', b' ', b' ', b' '], 3.0)]);
    }

    #[test]
    fn p836_valor_string_erro_verbatim() {
        let v = dict(&[("wght", Value::Str(EcoString::from("bold")))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "expected float, found string");
        assert_eq!(
            err[0].hints,
            vec!["occurred in tag at index 0 (`\"wght\"`)".to_string()]
        );
    }

    #[test]
    fn p836_nao_dict_erro_verbatim() {
        let err = FontVariations::from_value(&Value::Int(5), Span::detached()).unwrap_err();
        assert_eq!(err[0].message, "expected dictionary, found integer");
        assert!(err[0].hints.is_empty());
    }

    #[test]
    fn p836_indice_no_hint_segue_posicao_do_dict() {
        let v = dict(&[("wght", Value::Int(1)), ("xxxxx", Value::Int(2))]);
        let err = FontVariations::from_value(&v, Span::detached()).unwrap_err();
        assert_eq!(
            err[0].hints,
            vec![
                "found 5 characters".to_string(),
                "occurred in tag at index 1 (`\"xxxxx\"`)".to_string(),
            ]
        );
    }

    #[test]
    fn p836_fold_interno_vence_por_tag_externo_sobrevive() {
        let inner = FontVariations(vec![(*b"wght", 250.0)]);
        let outer = FontVariations(vec![(*b"ital", 1.0), (*b"wght", 700.0)]);
        let folded = inner.fold(&outer);
        assert_eq!(folded.0, vec![(*b"ital", 1.0), (*b"wght", 250.0)]);
    }

    #[test]
    fn p836_from_validated_dict_leniente() {
        let Value::Dict(d) = dict(&[("wght", Value::Int(300))]) else {
            panic!("dict esperado")
        };
        let fv = FontVariations::from_validated_dict(&d);
        assert_eq!(fv.0, vec![(*b"wght", 300.0)]);
    }
}
