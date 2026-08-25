//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/int.md
//! @prompt-hash 8b27689f
//! @layer L1
//! @updated 2026-08-24

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::bytes::Bytes;
use crate::entities::file_id::FileId;
use crate::entities::func::{Func, FuncRepr};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

type NativeResult = SourceResult<Value>;

pub fn int_type_field(field: &str) -> Option<Value> {
    Some(match field {
        "min" => Value::Int(i64::MIN),
        "max" => Value::Int(i64::MAX),
        "signum" => Value::Func(Func::native("signum", native_int_signum)),
        "bit-not" => Value::Func(Func::native("bit-not", native_int_bit_not)),
        "bit-and" => Value::Func(Func::native("bit-and", native_int_bit_and)),
        "bit-or" => Value::Func(Func::native("bit-or", native_int_bit_or)),
        "bit-xor" => Value::Func(Func::native("bit-xor", native_int_bit_xor)),
        "bit-lshift" => Value::Func(Func::native("bit-lshift", native_int_bit_lshift)),
        "bit-rshift" => Value::Func(Func::native("bit-rshift", native_int_bit_rshift)),
        "from-bytes" => Value::Func(Func::native("from-bytes", native_int_from_bytes)),
        "to-bytes" => Value::Func(Func::native("to-bytes", native_int_to_bytes)),
        _ => return None,
    })
}

pub(crate) fn is_int_instance_method(name: &str) -> bool {
    matches!(
        name,
        "signum"
            | "bit-not"
            | "bit-and"
            | "bit-or"
            | "bit-xor"
            | "bit-lshift"
            | "bit-rshift"
            | "to-bytes"
    )
}

pub(crate) fn dispatch_int_method(
    receiver: i64,
    name: &str,
    mut args: Args,
    ctx: &mut EvalContext,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> NativeResult {
    args.items.insert(0, Value::Int(receiver));
    let Some(Value::Func(func)) = int_type_field(name) else { unreachable!() };
    let FuncRepr::Native(native) = func.repr() else { unreachable!() };
    (native.call)(ctx, &args, world, current_file)
}

fn error(args: &Args, message: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(args.span, message.into())]
}

fn no_named(args: &Args) -> Result<(), Vec<SourceDiagnostic>> {
    if args.named.is_empty() {
        Ok(())
    } else {
        Err(error(args, "unexpected argument"))
    }
}

fn unary(args: &Args) -> Result<i64, Vec<SourceDiagnostic>> {
    no_named(args)?;
    match args.items.as_slice() {
        [Value::Int(value)] => Ok(*value),
        [] => Err(error(args, "missing argument: self")),
        [other] => {
            Err(error(args, format!("expected integer, found {}", other.type_name())))
        }
        _ => Err(error(args, "unexpected argument")),
    }
}

fn binary(args: &Args) -> Result<(i64, i64), Vec<SourceDiagnostic>> {
    no_named(args)?;
    match args.items.as_slice() {
        [Value::Int(lhs), Value::Int(rhs)] => Ok((*lhs, *rhs)),
        [] => Err(error(args, "missing argument: self")),
        [Value::Int(_)] => Err(error(args, "missing argument: rhs")),
        [other, ..] if !matches!(other, Value::Int(_)) => {
            Err(error(args, format!("expected integer, found {}", other.type_name())))
        }
        [Value::Int(_), other] => {
            Err(error(args, format!("expected integer, found {}", other.type_name())))
        }
        _ => Err(error(args, "unexpected argument")),
    }
}

macro_rules! native {
    ($name:ident, $body:expr) => {
        fn $name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: FileId,
        ) -> NativeResult {
            $body(args)
        }
    };
}

native!(native_int_signum, |args: &Args| Ok(Value::Int(unary(args)?.signum())));
native!(native_int_bit_not, |args: &Args| Ok(Value::Int(!unary(args)?)));
native!(native_int_bit_and, |args: &Args| {
    let (a, b) = binary(args)?;
    Ok(Value::Int(a & b))
});
native!(native_int_bit_or, |args: &Args| {
    let (a, b) = binary(args)?;
    Ok(Value::Int(a | b))
});
native!(native_int_bit_xor, |args: &Args| {
    let (a, b) = binary(args)?;
    Ok(Value::Int(a ^ b))
});

fn shift_args(
    args: &Args,
    logical_allowed: bool,
) -> Result<(i64, u32, bool), Vec<SourceDiagnostic>> {
    if args.named.keys().any(|key| key.as_str() != "logical")
        || (!logical_allowed && !args.named.is_empty())
    {
        return Err(error(args, "unexpected argument"));
    }
    let logical = match args.named.get("logical") {
        None => false,
        Some(Value::Bool(value)) => *value,
        Some(other) => {
            return Err(error(
                args,
                format!("expected boolean, found {}", other.type_name()),
            ))
        }
    };
    let positional = Args {
        items: args.items.clone(),
        named: Default::default(),
        span: args.span,
    };
    let (value, shift) = binary(&positional)?;
    let shift =
        u32::try_from(shift).map_err(|_| error(args, "number must be at least zero"))?;
    Ok((value, shift, logical))
}

native!(native_int_bit_lshift, |args: &Args| {
    let (value, shift, _) = shift_args(args, false)?;
    value
        .checked_shl(shift)
        .map(Value::Int)
        .ok_or_else(|| error(args, "the result is too large"))
});
native!(native_int_bit_rshift, |args: &Args| {
    let (value, shift, logical) = shift_args(args, true)?;
    let result = if logical {
        if shift >= 64 {
            0
        } else {
            ((value as u64) >> shift) as i64
        }
    } else {
        value >> shift.min(63)
    };
    Ok(Value::Int(result))
});

#[derive(Clone, Copy)]
enum Endian {
    Little,
    Big,
}

fn endian(args: &Args) -> Result<Endian, Vec<SourceDiagnostic>> {
    match args.named.get("endian") {
        None => Ok(Endian::Little),
        Some(Value::Str(value)) if value.as_str() == "little" => Ok(Endian::Little),
        Some(Value::Str(value)) if value.as_str() == "big" => Ok(Endian::Big),
        Some(Value::Str(_)) => Err(error(args, "unknown variant")),
        Some(other) => {
            Err(error(args, format!("expected string, found {}", other.type_name())))
        }
    }
}

native!(native_int_from_bytes, |args: &Args| {
    if args
        .named
        .keys()
        .any(|key| !matches!(key.as_str(), "endian" | "signed"))
    {
        return Err(error(args, "unexpected argument"));
    }
    let bytes = match args.items.as_slice() {
        [Value::Bytes(bytes)] => bytes.as_slice(),
        [] => return Err(error(args, "missing argument: bytes")),
        [other] => {
            return Err(error(
                args,
                format!("expected bytes, found {}", other.type_name()),
            ))
        }
        _ => return Err(error(args, "unexpected argument")),
    };
    if bytes.len() > 8 {
        return Err(error(args, "too many bytes to convert to a 64 bit number"));
    }
    if bytes.is_empty() {
        return Ok(Value::Int(0));
    }
    let signed = match args.named.get("signed") {
        None => true,
        Some(Value::Bool(value)) => *value,
        Some(other) => {
            return Err(error(
                args,
                format!("expected boolean, found {}", other.type_name()),
            ))
        }
    };
    let order = endian(args)?;
    let mut buf = [0_u8; 8];
    match order {
        Endian::Big => buf[8 - bytes.len()..].copy_from_slice(bytes),
        Endian::Little => buf[..bytes.len()].copy_from_slice(bytes),
    }
    let high = match order {
        Endian::Big => bytes[0],
        Endian::Little => bytes[bytes.len() - 1],
    };
    if signed && high & 0x80 != 0 {
        match order {
            Endian::Big => buf[..8 - bytes.len()].fill(0xff),
            Endian::Little => buf[bytes.len()..].fill(0xff),
        }
    }
    Ok(Value::Int(match order {
        Endian::Big => i64::from_be_bytes(buf),
        Endian::Little => i64::from_le_bytes(buf),
    }))
});

native!(native_int_to_bytes, |args: &Args| {
    if args
        .named
        .keys()
        .any(|key| !matches!(key.as_str(), "endian" | "size"))
    {
        return Err(error(args, "unexpected argument"));
    }
    let positional = Args {
        items: args.items.clone(),
        named: Default::default(),
        span: args.span,
    };
    let value = unary(&positional)?;
    let size = match args.named.get("size") {
        None => 8,
        Some(Value::Int(value)) => usize::try_from(*value)
            .map_err(|_| error(args, "number must be at least zero"))?,
        Some(other) => {
            return Err(error(
                args,
                format!("expected integer, found {}", other.type_name()),
            ))
        }
    };
    let order = endian(args)?;
    let array = match order {
        Endian::Big => value.to_be_bytes(),
        Endian::Little => value.to_le_bytes(),
    };
    let mut output = vec![0; size];
    match order {
        Endian::Big => {
            let out = size.saturating_sub(8);
            let input = 8_usize.saturating_sub(size);
            output[out..].copy_from_slice(&array[input..]);
        }
        Endian::Little => {
            let end = size.min(8);
            output[..end].copy_from_slice(&array[..end]);
        }
    }
    Ok(Value::Bytes(Bytes::new(output)))
});
