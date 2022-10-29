use std::mem;

use erg_common::enum_unwrap;

use crate::context::Context;
use crate::ty::constructors::{and, mono};
use crate::ty::value::{EvalValueResult, GenTypeObj, TypeObj, ValueObj};
use crate::ty::ValueArgs;
use erg_common::astr::AtomicStr;
use erg_common::error::{ErrorCore, ErrorKind, Location};
use erg_common::style::{RED, RESET, YELLOW};

/// Requirement: Type, Impl := Type -> ClassType
pub fn class_func(mut args: ValueArgs, ctx: &Context) -> EvalValueResult<ValueObj> {
    let require = args.remove_left_or_key("Requirement").ok_or_else(|| {
        ErrorCore::new(
            line!() as usize,
            ErrorKind::TypeError,
            Location::Unknown,
            AtomicStr::from(format!("{RED}Requirement{RESET} is not passed")),
            None,
        )
    })?;
    let Some(require) = require.as_type() else {
        return Err(ErrorCore::new(
            line!() as usize,
            ErrorKind::TypeError,
            Location::Unknown,
            AtomicStr::from(format!(
                "non-type object {RED}{require}{RESET} is passed to {YELLOW}Requirement{RESET}",
            )),
            None,
        ).into());
    };
    let impls = args.remove_left_or_key("Impl");
    let impls = impls.map(|v| v.as_type().unwrap());
    let t = mono(ctx.name.clone());
    Ok(ValueObj::gen_t(GenTypeObj::class(t, require, impls)))
}

/// Super: ClassType, Impl := Type, Additional := Type -> ClassType
pub fn inherit_func(mut args: ValueArgs, ctx: &Context) -> EvalValueResult<ValueObj> {
    let sup = args.remove_left_or_key("Super").ok_or_else(|| {
        ErrorCore::new(
            line!() as usize,
            ErrorKind::KeyError,
            Location::Unknown,
            AtomicStr::from(format!("{RED}Super{RESET} is not passed")),
            None,
        )
    })?;
    let Some(sup) = sup.as_type() else {
        return Err(ErrorCore::new(
            line!() as usize,
            ErrorKind::TypeError,
            Location::Unknown,
            AtomicStr::from(format!(
                "non-class object {RED}{sup}{RESET} is passed to {YELLOW}Super{RESET}",
            )),
            None,
        ).into());
    };
    let impls = args.remove_left_or_key("Impl");
    let impls = impls.map(|v| v.as_type().unwrap());
    let additional = args.remove_left_or_key("Additional");
    let additional = additional.map(|v| v.as_type().unwrap());
    let t = mono(ctx.name.clone());
    Ok(ValueObj::gen_t(GenTypeObj::inherited(
        t, sup, impls, additional,
    )))
}

/// Class: ClassType -> ClassType (with `InheritableType`)
/// This function is used by the compiler to mark a class as inheritable and does nothing in terms of actual operation.
pub fn inheritable_func(mut args: ValueArgs, _ctx: &Context) -> EvalValueResult<ValueObj> {
    let class = args.remove_left_or_key("Class").ok_or_else(|| {
        ErrorCore::new(
            line!() as usize,
            ErrorKind::KeyError,
            Location::Unknown,
            AtomicStr::from(format!("{RED}Class{RESET} is not passed")),
            None,
        )
    })?;
    match class {
        ValueObj::Type(TypeObj::Generated(mut gen)) => {
            if let Some(typ) = gen.impls_mut() {
                match typ.as_mut().map(|x| x.as_mut()) {
                    Some(TypeObj::Generated(gen)) => {
                        *gen.typ_mut() = and(mem::take(gen.typ_mut()), mono("InheritableType"));
                    }
                    Some(TypeObj::Builtin(t)) => {
                        *t = and(mem::take(t), mono("InheritableType"));
                    }
                    _ => {
                        *typ = Some(Box::new(TypeObj::Builtin(mono("InheritableType"))));
                    }
                }
            }
            Ok(ValueObj::Type(TypeObj::Generated(gen)))
        }
        _ => todo!(),
    }
}

/// Requirement: Type, Impl := Type -> TraitType
pub fn trait_func(mut args: ValueArgs, ctx: &Context) -> EvalValueResult<ValueObj> {
    let require = args.remove_left_or_key("Requirement").ok_or_else(|| {
        ErrorCore::new(
            line!() as usize,
            ErrorKind::KeyError,
            Location::Unknown,
            AtomicStr::from(format!("{RED}Requirement{RESET} is not passed")),
            None,
        )
    })?;
    let Some(require) = require.as_type() else {
        return Err(ErrorCore::new(
            line!() as usize,
            ErrorKind::TypeError,
            Location::Unknown,
            AtomicStr::from(format!(
                "non-type object {RED}{require}{RESET} is passed to {YELLOW}Requirement{RESET}",
            )),
            None,
        ).into());
    };
    let impls = args.remove_left_or_key("Impl");
    let impls = impls.map(|v| v.as_type().unwrap());
    let t = mono(ctx.name.clone());
    Ok(ValueObj::gen_t(GenTypeObj::trait_(t, require, impls)))
}

/// Super: TraitType, Impl := Type, Additional := Type -> TraitType
pub fn subsume_func(mut args: ValueArgs, ctx: &Context) -> EvalValueResult<ValueObj> {
    let sup = args.remove_left_or_key("Super").ok_or_else(|| {
        ErrorCore::new(
            line!() as usize,
            ErrorKind::KeyError,
            Location::Unknown,
            AtomicStr::from(format!("{RED}Super{RESET} is not passed")),
            None,
        )
    })?;
    let Some(sup) = sup.as_type() else {
        return Err(ErrorCore::new(
            line!() as usize,
            ErrorKind::TypeError,
            Location::Unknown,
            AtomicStr::from(format!(
                "non-trait object {RED}{sup}{RESET} is passed to {YELLOW}Super{RESET}",
            )),
            None,
        ).into());
    };
    let impls = args.remove_left_or_key("Impl");
    let impls = impls.map(|v| v.as_type().unwrap());
    let additional = args.remove_left_or_key("Additional");
    let additional = additional.map(|v| v.as_type().unwrap());
    let t = mono(ctx.name.clone());
    Ok(ValueObj::gen_t(GenTypeObj::subsumed(
        t, sup, impls, additional,
    )))
}

pub fn __array_getitem__(mut args: ValueArgs, ctx: &Context) -> EvalValueResult<ValueObj> {
    let slf = ctx
        .convert_value_into_array(args.remove_left_or_key("Self").unwrap())
        .unwrap();
    let index = enum_unwrap!(args.remove_left_or_key("Index").unwrap(), ValueObj::Nat);
    if let Some(v) = slf.get(index as usize) {
        Ok(v.clone())
    } else {
        Err(ErrorCore::new(
            line!() as usize,
            ErrorKind::IndexError,
            Location::Unknown,
            AtomicStr::from(format!(
                "[{}] has {} elements, but accessed {}th element",
                erg_common::fmt_vec(&slf),
                slf.len(),
                index
            )),
            None,
        )
        .into())
    }
}

pub fn __dict_getitem__(mut args: ValueArgs, ctx: &Context) -> EvalValueResult<ValueObj> {
    let slf = args.remove_left_or_key("Self").unwrap();
    let slf = enum_unwrap!(slf, ValueObj::Dict);
    let index = args.remove_left_or_key("Index").unwrap();
    if let Some(v) = slf.get(&index).or_else(|| {
        for (k, v) in slf.iter() {
            match (&index, k) {
                (ValueObj::Type(idx), ValueObj::Type(kt)) => {
                    if ctx.subtype_of(idx.typ(), kt.typ()) {
                        return Some(v);
                    }
                }
                (idx, k) => {
                    if idx == k {
                        return Some(v);
                    }
                }
            }
        }
        None
    }) {
        Ok(v.clone())
    } else {
        Err(ErrorCore::new(
            line!() as usize,
            ErrorKind::IndexError,
            Location::Unknown,
            AtomicStr::from(format!("{slf} has no key {index}",)),
            None,
        )
        .into())
    }
}

pub fn __range_getitem__(mut args: ValueArgs, _ctx: &Context) -> EvalValueResult<ValueObj> {
    let (_name, fields) = enum_unwrap!(
        args.remove_left_or_key("Self").unwrap(),
        ValueObj::DataClass { name, fields }
    );
    let index = enum_unwrap!(args.remove_left_or_key("Index").unwrap(), ValueObj::Nat);
    let start = fields.get("start").unwrap();
    let start = *enum_unwrap!(start, ValueObj::Nat);
    let end = fields.get("end").unwrap();
    let end = *enum_unwrap!(end, ValueObj::Nat);
    // FIXME <= if inclusive
    if start + index < end {
        Ok(ValueObj::Nat(start + index))
    } else {
        Err(ErrorCore::new(
            line!() as usize,
            ErrorKind::IndexError,
            Location::Unknown,
            AtomicStr::from(format!("Index out of range: {}", index)),
            None,
        )
        .into())
    }
}
