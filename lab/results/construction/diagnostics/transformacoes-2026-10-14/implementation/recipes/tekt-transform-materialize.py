from pathlib import Path
import re,json
root=Path.cwd()
def edit(p,f):
 p=Path(p);s=p.read_text();t=f(s);assert t!=s,p;p.write_text(t)
def replace(p,a,b):
 def f(s): assert a in s,(p,a);return s.replace(a,b)
 edit(p,f)
p='00_nucleo/prompts/entities/content/traversal.md'
edit(p,lambda s:s+'''\n## Transformações tipadas — preservação aprovada\n\nMedição: `01_core/src/entities/content/traversal.rs:906` reconstrói matrix/body;\ncorpus transformacoes-2026-10-14 mostra perda de identidade e presença.\nManifesto aprovado 960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d.\nReconstrução deve copiar operation integralmente e transformar apenas body,\nsem normalizar parâmetros, assar defaults ou reduzir variantes à matriz.\nTestar mapas com zero/false/auto e valores dependentes do estilo.\n''')
# Entity: exact approved carrier, existing Element hooks only.
p=Path('01_core/src/entities/elements/transform.rs');s=p.read_text();s=s.replace('use crate::entities::layout_types::TransformMatrix;','use crate::entities::layout_types::{Align2D, Angle, Length, Ratio, TransformMatrix};\nuse crate::entities::rel::Rel;\nuse crate::entities::value::Value;')
s=s.replace('/// Aplica `matrix` (transformação afim 2D) ao `body`.','/// Operação autoral; parâmetros dependentes do contexto são resolvidos no layout.')
s=s.replace('pub matrix: TransformMatrix,','pub operation: TransformOperation,')
at=s.index('// `Hash` manual')
s=s[:at]+'''#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformScale {
    Auto,
    Ratio(Ratio),
    Length(Length),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformOperation {
    Matrix(TransformMatrix),
    Move { dx: Option<Rel<Length>>, dy: Option<Rel<Length>> },
    Rotate { angle: Option<Angle>, origin: Option<Align2D>, reflow: Option<bool> },
    Scale { x: Option<TransformScale>, y: Option<TransformScale>, origin: Option<Align2D>, reflow: Option<bool> },
    Skew { ax: Option<Angle>, ay: Option<Angle>, origin: Option<Align2D>, reflow: Option<bool> },
}

fn scale_value(value: TransformScale) -> Value {
    match value {
        TransformScale::Auto => Value::Auto,
        TransformScale::Ratio(r) => Value::Ratio(r),
        TransformScale::Length(l) => Value::Length(l),
    }
}

'''+s[at:]
s=s.replace('matrix: self.matrix,','operation: self.operation,').replace('matrix: TransformMatrix::identity(),','operation: TransformOperation::Matrix(TransformMatrix::identity()),').replace('assert_eq!(e.matrix, TransformMatrix::identity());','assert_eq!(e.operation, TransformOperation::Matrix(TransformMatrix::identity()));')
pos=s.index('    fn plain_text(&self)')
s=s[:pos]+'''    fn get_field(&self, field: &str) -> Option<Value> {
        use TransformOperation::*;
        match (self.operation, field) {
            (Matrix(_), _) => None,
            (_, "body") => Some(Value::Content(self.body.clone())),
            (Move { dx, .. }, "dx") => dx.map(Value::Relative),
            (Move { dy, .. }, "dy") => dy.map(Value::Relative),
            (Rotate { angle, .. }, "angle") => angle.map(Value::Angle),
            (Scale { x, .. }, "x") => x.map(scale_value),
            (Scale { y, .. }, "y") => y.map(scale_value),
            (Skew { ax, .. }, "ax") => ax.map(Value::Angle),
            (Skew { ay, .. }, "ay") => ay.map(Value::Angle),
            (Rotate { origin, .. } | Scale { origin, .. } | Skew { origin, .. }, "origin") => origin.map(Value::Align),
            (Rotate { reflow, .. } | Scale { reflow, .. } | Skew { reflow, .. }, "reflow") => reflow.map(Value::Bool),
            _ => None,
        }
    }

'''+s[pos:];p.write_text(s)
replace('01_core/src/entities/content/construction/layout.rs','TransformElem { matrix, body }','TransformElem { operation: crate::entities::elements::transform::TransformOperation::Matrix(matrix), body }')
replace('01_core/src/entities/content/traversal.rs','matrix: e.matrix,','operation: e.operation,')
replace('01_core/src/compiler/introspect/materialize_time.rs','Content::transform(e.matrix, materialize_time(&e.body, intr, location))','Content::Transform(std::sync::Arc::new(crate::entities::elements::transform::TransformElem { operation: e.operation, body: materialize_time(&e.body, intr, location) }))')
replace('01_core/src/entities/content/fields.rs','(Content::Raw(e), f) => e.get_field(f),','(Content::Raw(e), f) => e.get_field(f),\n            (Content::Transform(e), f) => e.get_field(f),')
replace('01_core/src/entities/content/identity.rs','Content::Transform(_) => "transform",','''Content::Transform(e) => match e.operation {
                crate::entities::elements::transform::TransformOperation::Matrix(_) => "transform",
                crate::entities::elements::transform::TransformOperation::Move { .. } => "move",
                crate::entities::elements::transform::TransformOperation::Rotate { .. } => "rotate",
                crate::entities::elements::transform::TransformOperation::Scale { .. } => "scale",
                crate::entities::elements::transform::TransformOperation::Skew { .. } => "skew",
            },''')
replace('01_core/src/compiler/eval/observation/elements.rs','''let Self { matrix: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.matrix.observation_relation(&other.matrix))''','''let Self { operation: _, body: _ } = self;
        // All operation payloads are pure scalars. NaN remains conservative.
        (if self.operation == other.operation { ObservationRelation::Same } else { ObservationRelation::Different })''')
p='01_core/src/compiler/eval/bindings/field_access/content_methods.rs'
replace(p,'        Content::Raw(_) => match field {','''        Content::Transform(e) => {
            use crate::entities::elements::transform::TransformOperation;
            let declared = match e.operation {
                TransformOperation::Matrix(_) => false,
                TransformOperation::Move { .. } => matches!(field, "dx" | "dy" | "body"),
                TransformOperation::Rotate { .. } => matches!(field, "angle" | "origin" | "reflow" | "body"),
                TransformOperation::Scale { .. } => matches!(field, "x" | "y" | "origin" | "reflow" | "body"),
                TransformOperation::Skew { .. } => matches!(field, "ax" | "ay" | "origin" | "reflow" | "body"),
            };
            if declared { c.get_field(field).map(ContentField::Set).unwrap_or(ContentField::Unset) }
            else { ContentField::Undeclared }
        }
        Content::Raw(_) => match field {''')
replace(p,'        Content::Raw(_) => &["text", "block", "lang"],','''        Content::Raw(_) => &["text", "block", "lang"],
        Content::Transform(e) => match e.operation {
            crate::entities::elements::transform::TransformOperation::Matrix(_) => &[],
            crate::entities::elements::transform::TransformOperation::Move { .. } => &["dx", "dy", "body"],
            crate::entities::elements::transform::TransformOperation::Rotate { .. } => &["angle", "origin", "reflow", "body"],
            crate::entities::elements::transform::TransformOperation::Scale { .. } => &["x", "y", "origin", "reflow", "body"],
            crate::entities::elements::transform::TransformOperation::Skew { .. } => &["ax", "ay", "origin", "reflow", "body"],
        },''')
replace(p,'        Content::Raw(_) => crate::compiler::stdlib::native_raw,','''        Content::Raw(_) => crate::compiler::stdlib::native_raw,
        Content::Transform(e) => match e.operation {
            crate::entities::elements::transform::TransformOperation::Matrix(_) => content_func_not_callable,
            crate::entities::elements::transform::TransformOperation::Move { .. } => crate::compiler::stdlib::native_move,
            crate::entities::elements::transform::TransformOperation::Rotate { .. } => crate::compiler::stdlib::native_rotate,
            crate::entities::elements::transform::TransformOperation::Scale { .. } => crate::compiler::stdlib::native_scale,
            crate::entities::elements::transform::TransformOperation::Skew { .. } => crate::compiler::stdlib::native_skew,
        },''')
p='01_core/src/compiler/eval/bindings/field_access/fields.rs'
edit(p,lambda s:s.replace('is_smartquote_content','is_presence_content').replace('        Content::SmartQuote(_) => true,','''        Content::SmartQuote(_) => true,
        Content::Transform(e) => !matches!(e.operation, crate::entities::elements::transform::TransformOperation::Matrix(_)),''').replace('        | Content::Transform(..)\n',''))
replace('01_core/src/compiler/eval/repr/content.rs','        Content::Transform(t) => format!("transform[{}]", repr_content(&t.body)),','''        Content::Transform(t) => {
            if matches!(t.operation, crate::entities::elements::transform::TransformOperation::Matrix(_)) {
                format!("transform[{}]", repr_content(&t.body))
            } else {
                let fields = crate::compiler::eval::bindings::field_access::content_methods::content_fields(c)
                    .into_iter().map(|(name, value)| format!("{name}: {}", repr_value(&value))).collect::<Vec<_>>();
                format!("{}{}", c.elem_name(), pretty_array_like(&fields, false))
            }
        },''')
# Repr helper is private: use existing re-export if available; will inspect before compiling.
p='01_core/src/compiler/eval/selector_matching.rs'
replace(p,'        Content::Shape(_) | Content::Transform(_) => false,','''        Content::Shape(_) => false,
        Content::Transform(e) => match e.operation {
            crate::entities::elements::transform::TransformOperation::Matrix(_) => false,
            crate::entities::elements::transform::TransformOperation::Move { .. } => native!(native_move),
            crate::entities::elements::transform::TransformOperation::Rotate { .. } => native!(native_rotate),
            crate::entities::elements::transform::TransformOperation::Scale { .. } => native!(native_scale),
            crate::entities::elements::transform::TransformOperation::Skew { .. } => native!(native_skew),
        },''')
print('carrier and dependent language consumers migrated')
