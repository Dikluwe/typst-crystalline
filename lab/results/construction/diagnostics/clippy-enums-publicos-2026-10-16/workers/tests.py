from pathlib import Path
import json,datetime
out=Path('00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16');record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before={},after={})
tag=r'''
    #[test]
    fn public_enum_budget_tag_tracks_end_payload() {
        assert!(std::mem::size_of::<Tag>() <= 3 * std::mem::size_of::<Location>());
    }

    #[test]
    fn boxed_tag_clone_keeps_payload_ownership_independent() {
        use crate::entities::label::Label;
        use std::hash::{Hash, Hasher};
        fn hash(tag: &Tag) -> u64 {
            let mut state = std::collections::hash_map::DefaultHasher::new();
            tag.hash(&mut state);
            state.finish()
        }
        let original = Tag::Start(loc(7), Box::new(info()));
        let independent = Tag::Start(loc(7), Box::new(info()));
        assert_eq!(original, independent);
        assert_eq!(hash(&original), hash(&independent));
        let mut copied = original.clone();
        let Tag::Start(location, payload) = &mut copied else { unreachable!() };
        assert_eq!(*location, loc(7));
        payload.label = Some(Label("copied".into()));
        let Tag::Start(_, payload) = &original else { unreachable!() };
        assert_eq!(payload.label, None);
        assert_ne!(original, copied);
    }
'''
show=r'''
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::entities::style::Style;

    #[test]
    fn public_enum_budget_transformation_tracks_content() {
        assert!(std::mem::size_of::<Transformation>()
            <= std::mem::size_of::<Content>() + std::mem::align_of::<Transformation>());
    }

    #[test]
    fn shared_show_styles_clone_and_copy_on_write_preserve_values() {
        let styles = Styles::from_iter([Style::Weight(700)])
            .push_custom("probe", Value::Int(1));
        let original = Transformation::Style(Arc::new(styles));
        let mut copied = original.clone();
        let (Transformation::Style(original_styles), Transformation::Style(copied_styles))
            = (&original, &mut copied) else { unreachable!() };
        assert!(Arc::ptr_eq(original_styles, copied_styles));
        Arc::make_mut(copied_styles).push(Style::Weight(400));
        let changed = Arc::make_mut(copied_styles).clone()
            .push_custom("probe", Value::Int(2));
        *copied_styles = Arc::new(changed);
        assert!(!Arc::ptr_eq(original_styles, copied_styles));
        assert_eq!(original_styles.delta().weight, Some(700));
        assert_eq!(copied_styles.delta().weight, Some(400));
        assert_eq!(original_styles.delta().custom[0].1, Value::Int(1));
        assert_eq!(copied_styles.delta().custom.last().unwrap().1, Value::Int(2));
    }
}
'''
for name,test in [('tag',tag),('show',show)]:
 p=Path(f'01_core/src/entities/{name}.rs');t=p.read_text();record['before'][str(p)]=t
 if name=='tag':assert t.count('mod tests {')==1;t=t.replace('mod tests {','mod tests {'+test,1)
 else:assert '#[cfg(test)]' not in t;t+='\n'+test
 p.write_text(t);record['after'][str(p)]=t
(out/'tests-materialization.json').write_text(json.dumps(record,indent=2)+'\n')
