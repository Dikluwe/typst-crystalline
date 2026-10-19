//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/state/resolve.md
//! @prompt-hash 58bfe389
//! @layer L1
//! @updated 2026-10-17

use super::*;
use crate::entities::state_registry::RecordedStateUpdate;
use std::borrow::Cow;

pub(super) fn read(
    actions: Vec<(u128, RecordedStateUpdate)>,
    init: &Value,
    until: Option<u128>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let selected_at = until
        .map_or(actions.len(), |end| actions.partition_point(|(order, _)| *order <= end));
    let in_context = ctx.in_context;
    let location = ctx.current_location.take();
    ctx.in_context = false;
    let result = (|| {
        let mut scopes = Scopes::new(None);
        let mut value = Cow::Borrowed(init);
        let mut selected = if selected_at == 0 && !actions.is_empty() {
            Some(init.clone())
        } else {
            None
        };
        for (index, (_, update)) in actions.iter().enumerate() {
            value = match update {
                RecordedStateUpdate::Set(next) => Cow::Borrowed(next.as_ref()),
                RecordedStateUpdate::Func(callback) => Cow::Owned(apply_func(
                    callback.clone(),
                    Args::positional(vec![value.into_owned()]),
                    &mut scopes,
                    ctx,
                    engine,
                )?),
            };
            if index + 1 == selected_at && selected_at < actions.len() {
                selected = Some(value.as_ref().clone());
            }
        }
        Ok(selected.unwrap_or_else(|| value.into_owned()))
    })();
    ctx.in_context = in_context;
    ctx.current_location = location;
    result
}

pub(super) struct LegacyRead {
    actions: Vec<(u128, RecordedStateUpdate)>,
    init: std::sync::Arc<Value>,
    until: Option<u128>,
}

impl LegacyRead {
    pub(super) fn capture(
        registry: &crate::entities::state_registry::StateRegistry,
        key: &str,
        location: Option<crate::entities::location::Location>,
    ) -> Self {
        let actions = registry.actions_for(key);
        let init = if actions.is_empty() {
            let value = match location {
                Some(location) => registry.value_at(key, location),
                None => registry.final_value(key),
            };
            std::sync::Arc::new(value.cloned().unwrap_or(Value::None))
        } else {
            registry
                .legacy_init(key, location)
                .unwrap_or_else(|| std::sync::Arc::new(Value::None))
        };
        Self {
            actions,
            init,
            until: location.map(|location| registry.order_key(location)),
        }
    }

    pub(super) fn read(
        self,
        ctx: &mut EvalContext,
        engine: &mut Engine<'_>,
    ) -> SourceResult<Value> {
        read(self.actions, self.init.as_ref(), self.until, ctx, engine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::func::Func;
    use crate::entities::location::Location;
    use crate::entities::state_registry::StateRegistry;
    use std::sync::Arc;

    #[test]
    fn a1_legacy_read_plan_keeps_causal_actions_and_shares_payloads() {
        let first = Location::from_raw(100);
        let later = Location::from_raw(2);
        let mut registry = StateRegistry::empty();
        registry.observe_location(first);
        registry.observe_location(later);
        registry.record_action(
            "key",
            &StateUpdate::Set(Box::new(Value::Array(vec![Value::Int(7); 32]))),
            first,
        );
        registry.record_action(
            "key",
            &StateUpdate::Func(Func::native("future", |_, _, _, _| {
                panic!("capturing a plan must not execute the callback")
            })),
            later,
        );
        registry.record_action(
            "other",
            &StateUpdate::Set(Box::new(Value::Int(9))),
            later,
        );
        let plan = LegacyRead::capture(&registry, "key", Some(first));
        assert_eq!(plan.until, Some(0));
        assert_eq!(plan.actions.len(), 2);
        assert_eq!(plan.actions[0].0, 0);
        assert_eq!(plan.actions[1].0, 1);
        let RecordedStateUpdate::Set(shared) = &plan.actions[0].1 else {
            panic!("first action must be Set");
        };
        let actions = registry.actions_for("key");
        let RecordedStateUpdate::Set(original) = &actions[0].1 else {
            panic!("first action must be Set");
        };
        assert!(Arc::ptr_eq(shared, original));
        assert!(Arc::ptr_eq(shared, &plan.init));
        assert!(matches!(plan.actions[1].1, RecordedStateUpdate::Func(_)));

        registry.init("legacy".into(), Value::Int(1), first);
        registry.update("legacy".into(), Value::Int(2), later);
        let prefix = LegacyRead::capture(&registry, "legacy", Some(first));
        let final_value = LegacyRead::capture(&registry, "legacy", None);
        assert!(prefix.actions.is_empty());
        assert_eq!(*prefix.init, Value::Int(1));
        assert_eq!(*final_value.init, Value::Int(2));
        let missing = LegacyRead::capture(&registry, "missing", None);
        assert!(missing.actions.is_empty());
        assert_eq!(*missing.init, Value::None);
    }
}
