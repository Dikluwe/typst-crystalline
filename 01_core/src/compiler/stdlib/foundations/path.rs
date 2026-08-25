//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval.md
//! @prompt-hash 6325ac31
//! @layer L1
//! @updated 2026-08-24

use crate::compiler::eval::EvalContext;
use crate::compiler::stdlib::{err, expect_no_named};
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

pub fn resolve_path_value(
    value: &Value,
    world: &dyn World,
    current_file: FileId,
) -> Result<crate::entities::path::RootedPath, String> {
    match value {
        Value::Path(path) => Ok(path.clone()),
        Value::Str(path) => world.resolve_path(current_file, path),
        other => Err(format!("expected path or string, found {}", other.type_name())),
    }
}

pub fn read_path_value(
    value: &Value,
    world: &dyn World,
    current_file: FileId,
) -> Result<(crate::entities::path::RootedPath, std::sync::Arc<Vec<u8>>), String> {
    let path = resolve_path_value(value, world, current_file)?;
    let bytes = world.read_path(&path)?;
    Ok((path, bytes))
}

pub fn native_path(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let value = match args.items.as_slice() {
        [value] => value,
        _ => {
            return err(format!(
                "path() requer 1 argumento, recebeu {}",
                args.items.len()
            ))
        }
    };
    match value {
        Value::Path(path) => Ok(Value::Path(path.clone())),
        Value::Str(path) => world
            .resolve_path(current_file, path)
            .map(Value::Path)
            .map_err(|message| err(message).unwrap_err()),
        other => err(format!("expected path or string, found {}", other.type_name())),
    }
}
