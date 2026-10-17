from pathlib import Path
import re,json
root=Path.cwd()
def edit(p,f):
 p=Path(p);s=p.read_text();t=f(s);assert t!=s,p;p.write_text(t)
def replace(p,a,b):
 def f(s): assert a in s,(p,a);return s.replace(a,b)
 edit(p,f)
replace('01_core/src/entities/content/identity.rs','Self::Transform(_) => "transform",','''Self::Transform(e) => match e.operation {
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
