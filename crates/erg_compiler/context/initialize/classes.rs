#[allow(unused_imports)]
use erg_common::log;
use erg_common::vis::Visibility;
use erg_common::Str as StrStruct;

use crate::ty::constructors::*;
use crate::ty::typaram::TyParam;
use crate::ty::value::ValueObj;
use crate::ty::Type;
use ParamSpec as PS;
use Type::*;

use crate::context::initialize::*;
use crate::context::{Context, ParamSpec};
use crate::varinfo::Mutability;
use Mutability::*;
use Visibility::*;

impl Context {
    pub(super) fn init_builtin_classes(&mut self) {
        let vis = if cfg!(feature = "py_compatible") {
            Public
        } else {
            Private
        };
        let T = mono_q(TY_T, instanceof(Type));
        let U = mono_q(TY_U, instanceof(Type));
        let L = mono_q(TY_L, instanceof(Type));
        let R = mono_q(TY_R, instanceof(Type));
        let N = mono_q_tp(TY_N, instanceof(Nat));
        let M = mono_q_tp(TY_M, instanceof(Nat));
        let never = Self::builtin_mono_class(NEVER, 1);
        /* Obj */
        let mut obj = Self::builtin_mono_class(OBJ, 2);
        let Slf = mono_q(SELF, subtypeof(Obj));
        let t = fn0_met(Slf.clone(), Slf).quantify();
        obj.register_builtin_erg_impl(FUNC_CLONE, t, Const, Public);
        obj.register_builtin_erg_impl(FUNDAMENTAL_MODULE, Str, Const, Public);
        obj.register_builtin_erg_impl(FUNDAMENTAL_SIZEOF, fn0_met(Obj, Nat), Const, Public);
        obj.register_builtin_erg_impl(FUNDAMENTAL_REPR, fn0_met(Obj, Str), Immutable, Public);
        obj.register_builtin_erg_impl(FUNDAMENTAL_STR, fn0_met(Obj, Str), Immutable, Public);
        obj.register_builtin_erg_impl(
            FUNDAMENTAL_DICT,
            fn0_met(Obj, dict! {Str => Obj}.into()),
            Immutable,
            Public,
        );
        obj.register_builtin_erg_impl(
            FUNDAMENTAL_BYTES,
            fn0_met(Obj, mono(BYTES)),
            Immutable,
            Public,
        );
        let mut obj_in = Self::builtin_methods(Some(poly(IN, vec![ty_tp(Type)])), 2);
        obj_in.register_builtin_erg_impl(OP_IN, fn1_met(Obj, Type, Bool), Const, Public);
        obj.register_trait(Obj, obj_in);
        let mut obj_mutizable = Self::builtin_methods(Some(mono(MUTIZABLE)), 1);
        obj_mutizable.register_builtin_const(
            MUTABLE_MUT_TYPE,
            Public,
            ValueObj::builtin_t(mono(MUTABLE_OBJ)),
        );
        obj.register_trait(Obj, obj_mutizable);
        // Obj does not implement Eq

        /* Float */
        let mut float = Self::builtin_mono_class(FLOAT, 2);
        float.register_superclass(Obj, &obj);
        // TODO: support multi platform
        float.register_builtin_const(EPSILON, Public, ValueObj::Float(2.220446049250313e-16));
        float.register_builtin_py_impl(REAL, Float, Const, Public, Some(FUNC_REAL));
        float.register_builtin_py_impl(IMAG, Float, Const, Public, Some(FUNC_IMAG));
        float.register_builtin_py_impl(
            FUNC_CONJUGATE,
            fn0_met(Float, Float),
            Const,
            Public,
            Some(FUNC_CONJUGATE),
        );
        float.register_py_builtin(FUNC_HEX, fn0_met(Float, Str), Some(FUNC_HEX), 24);
        float.register_py_builtin(
            FUNC_IS_INTEGER,
            fn0_met(Float, Bool),
            Some(FUNC_IS_INTEGER),
            32,
        );
        float.register_builtin_py_impl(
            FUNC_FROMHEX,
            nd_func(vec![kw(KW_S, Str)], None, Float),
            Const,
            Public,
            Some(FUNC_FROMHEX),
        );
        float.register_marker_trait(mono(NUM));
        float.register_marker_trait(mono(ORD));
        let mut float_ord = Self::builtin_methods(Some(mono(ORD)), 2);
        float_ord.register_builtin_erg_impl(
            OP_CMP,
            fn1_met(Float, Float, mono(ORDERING)),
            Const,
            Public,
        );
        float.register_trait(Float, float_ord);
        // Float doesn't have an `Eq` implementation
        let op_t = fn1_met(Float, Float, Float);
        let mut float_add = Self::builtin_methods(Some(poly(ADD, vec![ty_tp(Float)])), 2);
        float_add.register_builtin_erg_impl(OP_ADD, op_t.clone(), Const, Public);
        float_add.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Float));
        float.register_trait(Float, float_add);
        let mut float_sub = Self::builtin_methods(Some(poly(SUB, vec![ty_tp(Float)])), 2);
        float_sub.register_builtin_erg_impl(OP_SUB, op_t.clone(), Const, Public);
        float_sub.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Float));
        float.register_trait(Float, float_sub);
        let mut float_mul = Self::builtin_methods(Some(poly(MUL, vec![ty_tp(Float)])), 2);
        float_mul.register_builtin_erg_impl(OP_MUL, op_t.clone(), Const, Public);
        float_mul.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Float));
        float_mul.register_builtin_const(POW_OUTPUT, Public, ValueObj::builtin_t(Float));
        float.register_trait(Float, float_mul);
        let mut float_div = Self::builtin_methods(Some(poly(DIV, vec![ty_tp(Float)])), 2);
        float_div.register_builtin_erg_impl(OP_DIV, op_t.clone(), Const, Public);
        float_div.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Float));
        float_div.register_builtin_const(MOD_OUTPUT, Public, ValueObj::builtin_t(Float));
        float.register_trait(Float, float_div);
        let mut float_floordiv =
            Self::builtin_methods(Some(poly(FLOOR_DIV, vec![ty_tp(Float)])), 2);
        float_floordiv.register_builtin_erg_impl(OP_FLOOR_DIV, op_t, Const, Public);
        float_floordiv.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Float));
        float.register_trait(Float, float_floordiv);
        let mut float_mutizable = Self::builtin_methods(Some(mono(MUTIZABLE)), 2);
        float_mutizable.register_builtin_const(
            MUTABLE_MUT_TYPE,
            Public,
            ValueObj::builtin_t(mono(MUT_FLOAT)),
        );
        float.register_trait(Float, float_mutizable);
        let mut float_show = Self::builtin_methods(Some(mono(SHOW)), 1);
        let t = fn0_met(Float, Str);
        float_show.register_builtin_py_impl(TO_STR, t, Immutable, Public, Some(FUNDAMENTAL_STR));
        float.register_trait(Float, float_show);

        /* Ratio */
        // TODO: Int, Nat, Boolの継承元をRatioにする(今はFloat)
        let mut ratio = Self::builtin_mono_class(RATIO, 2);
        ratio.register_superclass(Obj, &obj);
        ratio.register_builtin_py_impl(REAL, Ratio, Const, Public, Some(FUNC_REAL));
        ratio.register_builtin_py_impl(IMAG, Ratio, Const, Public, Some(FUNC_IMAG));
        ratio.register_marker_trait(mono(NUM));
        ratio.register_marker_trait(mono(ORD));
        let mut ratio_ord = Self::builtin_methods(Some(mono(ORD)), 2);
        ratio_ord.register_builtin_erg_impl(
            OP_CMP,
            fn1_met(Ratio, Ratio, mono(ORDERING)),
            Const,
            Public,
        );
        ratio.register_trait(Ratio, ratio_ord);
        let mut ratio_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        ratio_eq.register_builtin_erg_impl(OP_EQ, fn1_met(Ratio, Ratio, Bool), Const, Public);
        ratio.register_trait(Ratio, ratio_eq);
        let op_t = fn1_met(Ratio, Ratio, Ratio);
        let mut ratio_add = Self::builtin_methods(Some(poly(ADD, vec![ty_tp(Ratio)])), 2);
        ratio_add.register_builtin_erg_impl(OP_ADD, op_t.clone(), Const, Public);
        ratio_add.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Ratio));
        ratio.register_trait(Ratio, ratio_add);
        let mut ratio_sub = Self::builtin_methods(Some(poly(SUB, vec![ty_tp(Ratio)])), 2);
        ratio_sub.register_builtin_erg_impl(OP_SUB, op_t.clone(), Const, Public);
        ratio_sub.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Ratio));
        ratio.register_trait(Ratio, ratio_sub);
        let mut ratio_mul = Self::builtin_methods(Some(poly(MUL, vec![ty_tp(Ratio)])), 2);
        ratio_mul.register_builtin_erg_impl(OP_MUL, op_t.clone(), Const, Public);
        ratio_mul.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Ratio));
        ratio_mul.register_builtin_const(POW_OUTPUT, Public, ValueObj::builtin_t(Ratio));
        ratio.register_trait(Ratio, ratio_mul);
        let mut ratio_div = Self::builtin_methods(Some(poly(DIV, vec![ty_tp(Ratio)])), 2);
        ratio_div.register_builtin_erg_impl(OP_DIV, op_t.clone(), Const, Public);
        ratio_div.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Ratio));
        ratio_div.register_builtin_const(MOD_OUTPUT, Public, ValueObj::builtin_t(Ratio));
        ratio.register_trait(Ratio, ratio_div);
        let mut ratio_floordiv =
            Self::builtin_methods(Some(poly(FLOOR_DIV, vec![ty_tp(Ratio)])), 2);
        ratio_floordiv.register_builtin_erg_impl(OP_FLOOR_DIV, op_t, Const, Public);
        ratio_floordiv.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Ratio));
        ratio.register_trait(Ratio, ratio_floordiv);
        let mut ratio_mutizable = Self::builtin_methods(Some(mono(MUTIZABLE)), 2);
        ratio_mutizable.register_builtin_const(
            MUTABLE_MUT_TYPE,
            Public,
            ValueObj::builtin_t(mono(MUT_RATIO)),
        );
        ratio.register_trait(Ratio, ratio_mutizable);
        let mut ratio_show = Self::builtin_methods(Some(mono(SHOW)), 1);
        let t = fn0_met(Ratio, Str);
        ratio_show.register_builtin_erg_impl(TO_STR, t, Immutable, Public);
        ratio.register_trait(Ratio, ratio_show);

        /* Int */
        let mut int = Self::builtin_mono_class(INT, 2);
        int.register_superclass(Float, &float); // TODO: Float -> Ratio
        int.register_marker_trait(mono(NUM));
        // class("Rational"),
        // class("Integral"),
        int.register_builtin_py_impl(FUNC_ABS, fn0_met(Int, Nat), Immutable, Public, Some(OP_ABS));
        int.register_builtin_py_impl(
            FUNC_SUCC,
            fn0_met(Int, Int),
            Immutable,
            Public,
            Some(FUNC_SUCC),
        );
        int.register_builtin_py_impl(
            FUNC_PRED,
            fn0_met(Int, Int),
            Immutable,
            Public,
            Some(FUNC_PRED),
        );
        int.register_py_builtin(
            FUNC_BIT_LENGTH,
            fn0_met(Int, Nat),
            Some(FUNC_BIT_LENGTH),
            28,
        );
        int.register_py_builtin(FUNC_BIT_COUNT, fn0_met(Int, Nat), Some(FUNC_BIT_COUNT), 17);
        let t_from_bytes = func(
            vec![kw(
                BYTES,
                or(
                    mono(BYTES),
                    array_t(Type::from(value(0)..=value(255)), TyParam::erased(Nat)),
                ),
            )],
            None,
            vec![kw(
                FUNC_BYTEORDER,
                v_enum(
                    set! {ValueObj::Str(TOKEN_BIG_ENDIAN.into()), ValueObj::Str(TOKEN_LITTLE_ENDIAN.into())},
                ),
            )],
            Int,
        );
        int.register_builtin_py_impl(
            FUNC_FROM_BYTES,
            t_from_bytes,
            Const,
            Public,
            Some(FUNC_FROM_BYTES),
        );
        let t_to_bytes = func(
            vec![kw(KW_SELF, Int)],
            None,
            vec![
                kw(KW_LENGTH, Nat),
                kw(
                    FUNC_BYTEORDER,
                    v_enum(
                        set! {ValueObj::Str(TOKEN_BIG_ENDIAN.into()), ValueObj::Str(TOKEN_LITTLE_ENDIAN.into())},
                    ),
                ),
            ],
            mono(BYTES),
        );
        int.register_builtin_py_impl(
            FUNC_TO_BYTES,
            t_to_bytes,
            Immutable,
            Public,
            Some(FUNC_TO_BYTES),
        );
        let mut int_ord = Self::builtin_methods(Some(mono(ORD)), 2);
        int_ord.register_builtin_erg_impl(
            OP_PARTIAL_CMP,
            fn1_met(Int, Int, or(mono(ORDERING), NoneType)),
            Const,
            Public,
        );
        int.register_trait(Int, int_ord);
        let mut int_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        int_eq.register_builtin_erg_impl(OP_EQ, fn1_met(Int, Int, Bool), Const, Public);
        int.register_trait(Int, int_eq);
        // __div__ is not included in Int (cast to Ratio)
        let op_t = fn1_met(Int, Int, Int);
        let mut int_add = Self::builtin_methods(Some(poly(ADD, vec![ty_tp(Int)])), 2);
        int_add.register_builtin_erg_impl(OP_ADD, op_t.clone(), Const, Public);
        int_add.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Int));
        int.register_trait(Int, int_add);
        let mut int_sub = Self::builtin_methods(Some(poly(SUB, vec![ty_tp(Int)])), 2);
        int_sub.register_builtin_erg_impl(OP_SUB, op_t.clone(), Const, Public);
        int_sub.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Int));
        int.register_trait(Int, int_sub);
        let mut int_mul = Self::builtin_methods(Some(poly(MUL, vec![ty_tp(Int)])), 2);
        int_mul.register_builtin_erg_impl(OP_MUL, op_t.clone(), Const, Public);
        int_mul.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Int));
        int_mul.register_builtin_const(POW_OUTPUT, Public, ValueObj::builtin_t(Nat));
        int.register_trait(Int, int_mul);
        let mut int_floordiv = Self::builtin_methods(Some(poly(FLOOR_DIV, vec![ty_tp(Int)])), 2);
        int_floordiv.register_builtin_erg_impl(OP_FLOOR_DIV, op_t, Const, Public);
        int_floordiv.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Int));
        int.register_trait(Int, int_floordiv);
        let mut int_mutizable = Self::builtin_methods(Some(mono(MUTIZABLE)), 2);
        int_mutizable.register_builtin_const(
            MUTABLE_MUT_TYPE,
            Public,
            ValueObj::builtin_t(mono(MUT_INT)),
        );
        int.register_trait(Int, int_mutizable);
        let mut int_show = Self::builtin_methods(Some(mono(SHOW)), 1);
        let t = fn0_met(Int, Str);
        int_show.register_builtin_py_impl(TO_STR, t, Immutable, Public, Some(FUNDAMENTAL_STR));
        int.register_trait(Int, int_show);
        int.register_builtin_py_impl(REAL, Int, Const, Public, Some(FUNC_REAL));
        int.register_builtin_py_impl(IMAG, Int, Const, Public, Some(FUNC_IMAG));

        /* Nat */
        let mut nat = Self::builtin_mono_class(NAT, 10);
        nat.register_superclass(Int, &int);
        // class("Rational"),
        // class("Integral"),
        nat.register_builtin_py_impl(
            PROC_TIMES,
            pr_met(
                Nat,
                vec![kw(KW_PROC, nd_proc(vec![], None, NoneType))],
                None,
                vec![],
                NoneType,
            ),
            Immutable,
            Public,
            Some(FUNC_TIMES),
        );
        nat.register_marker_trait(mono(NUM));
        let mut nat_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        nat_eq.register_builtin_erg_impl(OP_EQ, fn1_met(Nat, Nat, Bool), Const, Public);
        nat.register_trait(Nat, nat_eq);
        let mut nat_ord = Self::builtin_methods(Some(mono(ORD)), 2);
        nat_ord.register_builtin_erg_impl(OP_CMP, fn1_met(Nat, Nat, mono(ORDERING)), Const, Public);
        nat.register_trait(Nat, nat_ord);
        // __sub__, __div__ is not included in Nat (cast to Int/ Ratio)
        let op_t = fn1_met(Nat, Nat, Nat);
        let mut nat_add = Self::builtin_methods(Some(poly(ADD, vec![ty_tp(Nat)])), 2);
        nat_add.register_builtin_erg_impl(OP_ADD, op_t.clone(), Const, Public);
        nat_add.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Nat));
        nat.register_trait(Nat, nat_add);
        let mut nat_mul = Self::builtin_methods(Some(poly(MUL, vec![ty_tp(Nat)])), 2);
        nat_mul.register_builtin_erg_impl(OP_MUL, op_t.clone(), Const, Public);
        nat_mul.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Nat));
        nat.register_trait(Nat, nat_mul);
        let mut nat_floordiv = Self::builtin_methods(Some(poly(FLOOR_DIV, vec![ty_tp(Nat)])), 2);
        nat_floordiv.register_builtin_erg_impl(OP_FLOOR_DIV, op_t, Const, Public);
        nat_floordiv.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Nat));
        nat.register_trait(Nat, nat_floordiv);
        let mut nat_mutizable = Self::builtin_methods(Some(mono(MUTIZABLE)), 2);
        nat_mutizable.register_builtin_const(
            MUTABLE_MUT_TYPE,
            Public,
            ValueObj::builtin_t(mono(MUT_NAT)),
        );
        nat.register_trait(Nat, nat_mutizable);
        nat.register_builtin_erg_impl(REAL, Nat, Const, Public);
        nat.register_builtin_erg_impl(IMAG, Nat, Const, Public);

        /* Bool */
        let mut bool_ = Self::builtin_mono_class(BOOL, 10);
        bool_.register_superclass(Nat, &nat);
        // class("Rational"),
        // class("Integral"),
        bool_.register_builtin_erg_impl(OP_AND, fn1_met(Bool, Bool, Bool), Const, Public);
        bool_.register_builtin_erg_impl(OP_OR, fn1_met(Bool, Bool, Bool), Const, Public);
        bool_.register_marker_trait(mono(NUM));
        let mut bool_ord = Self::builtin_methods(Some(mono(ORD)), 2);
        bool_ord.register_builtin_erg_impl(
            OP_CMP,
            fn1_met(Bool, Bool, mono(ORDERING)),
            Const,
            Public,
        );
        bool_.register_trait(Bool, bool_ord);
        let mut bool_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        bool_eq.register_builtin_erg_impl(OP_EQ, fn1_met(Bool, Bool, Bool), Const, Public);
        bool_.register_trait(Bool, bool_eq);
        let mut bool_mutizable = Self::builtin_methods(Some(mono(MUTIZABLE)), 2);
        bool_mutizable.register_builtin_const(
            MUTABLE_MUT_TYPE,
            Public,
            ValueObj::builtin_t(mono(MUT_BOOL)),
        );
        bool_.register_trait(Bool, bool_mutizable);
        let mut bool_show = Self::builtin_methods(Some(mono(SHOW)), 1);
        bool_show.register_builtin_erg_impl(TO_STR, fn0_met(Bool, Str), Immutable, Public);
        bool_.register_trait(Bool, bool_show);
        let t = fn0_met(Bool, Bool);
        bool_.register_builtin_py_impl(FUNC_INVERT, t, Immutable, Public, Some(FUNC_INVERT));
        /* Str */
        let mut str_ = Self::builtin_mono_class(STR, 10);
        str_.register_superclass(Obj, &obj);
        str_.register_marker_trait(mono(ORD));
        str_.register_marker_trait(mono(PATH_LIKE));
        str_.register_builtin_erg_impl(
            FUNC_REPLACE,
            fn_met(
                Str,
                vec![kw(KW_PAT, Str), kw(KW_INTO, Str)],
                None,
                vec![],
                Str,
            ),
            Immutable,
            Public,
        );
        str_.register_builtin_erg_impl(
            FUNC_ENCODE,
            fn_met(
                Str,
                vec![],
                None,
                vec![kw(KW_ENCODING, Str), kw(KW_ERRORS, Str)],
                mono(BYTES),
            ),
            Immutable,
            Public,
        );
        str_.register_builtin_erg_impl(
            FUNC_FORMAT,
            fn_met(Str, vec![], Some(kw(KW_ARGS, Obj)), vec![], Str),
            Immutable,
            Public,
        );
        str_.register_builtin_erg_impl(
            FUNC_LOWER,
            fn_met(Str, vec![], None, vec![], Str),
            Immutable,
            Public,
        );
        str_.register_builtin_erg_impl(
            FUNC_UPPER,
            fn_met(Str, vec![], None, vec![], Str),
            Immutable,
            Public,
        );
        str_.register_builtin_erg_impl(
            FUNC_TO_INT,
            fn_met(Str, vec![], None, vec![], or(Int, NoneType)),
            Immutable,
            Public,
        );
        str_.register_builtin_py_impl(
            FUNC_STARTSWITH,
            fn1_met(Str, Str, Bool),
            Immutable,
            Public,
            Some(FUNC_STARTSWITH),
        );
        str_.register_builtin_py_impl(
            FUNC_ENDSWITH,
            fn1_met(Str, Str, Bool),
            Immutable,
            Public,
            Some(FUNC_ENDSWITH),
        );
        str_.register_builtin_py_impl(
            FUNC_SPLIT,
            fn_met(
                Str,
                vec![kw(KW_SEP, Str)],
                None,
                vec![kw(KW_MAXSPLIT, Nat)],
                unknown_len_array_t(Str),
            ),
            Immutable,
            Public,
            Some(FUNC_SPLIT),
        );
        str_.register_builtin_py_impl(
            FUNC_SPLITLINES,
            fn_met(
                Str,
                vec![],
                None,
                vec![kw(KW_KEEPENDS, Bool)],
                unknown_len_array_t(Str),
            ),
            Immutable,
            Public,
            Some(FUNC_SPLITLINES),
        );
        str_.register_builtin_py_impl(
            FUNC_JOIN,
            fn1_met(unknown_len_array_t(Str), Str, Str),
            Immutable,
            Public,
            Some(FUNC_JOIN),
        );
        str_.register_builtin_py_impl(
            FUNC_INDEX,
            fn_met(
                Str,
                vec![kw(KW_SUB, Str)],
                None,
                vec![kw(KW_START, Nat), kw(KW_END, Nat)],
                or(Nat, Never),
            ),
            Immutable,
            Public,
            Some(FUNC_INDEX),
        );
        str_.register_builtin_py_impl(
            FUNC_RINDEX,
            fn_met(
                Str,
                vec![kw(KW_SUB, Str)],
                None,
                vec![kw(KW_START, Nat), kw(KW_END, Nat)],
                or(Nat, Never),
            ),
            Immutable,
            Public,
            Some(FUNC_RINDEX),
        );
        str_.register_builtin_py_impl(
            FUNC_FIND,
            fn_met(
                Str,
                vec![kw(KW_SUB, Str)],
                None,
                vec![kw(KW_START, Nat), kw(KW_END, Nat)],
                or(Nat, v_enum(set! {(-1).into()})),
            ),
            Immutable,
            Public,
            Some(FUNC_FIND),
        );
        str_.register_builtin_py_impl(
            FUNC_RFIND,
            fn_met(
                Str,
                vec![kw(KW_SUB, Str)],
                None,
                vec![kw(KW_START, Nat), kw(KW_END, Nat)],
                or(Nat, v_enum(set! {(-1).into()})),
            ),
            Immutable,
            Public,
            Some(FUNC_RFIND),
        );
        str_.register_builtin_py_impl(
            FUNC_COUNT,
            fn_met(
                Str,
                vec![kw(KW_SUB, Str)],
                None,
                vec![kw(KW_START, Nat), kw(KW_END, Nat)],
                Nat,
            ),
            Immutable,
            Public,
            Some(FUNC_COUNT),
        );
        str_.register_py_builtin(
            FUNC_CAPITALIZE,
            fn0_met(Str, Str),
            Some(FUNC_CAPITALIZE),
            13,
        );
        str_.register_builtin_erg_impl(FUNC_CONTAINS, fn1_met(Str, Str, Bool), Immutable, Public);
        let str_getitem_t = fn1_kw_met(Str, kw(KW_IDX, Nat), Str);
        str_.register_builtin_erg_impl(FUNDAMENTAL_GETITEM, str_getitem_t, Immutable, Public);
        let mut str_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        str_eq.register_builtin_erg_impl(OP_EQ, fn1_met(Str, Str, Bool), Const, Public);
        str_.register_trait(Str, str_eq);
        let mut str_seq = Self::builtin_methods(Some(poly(SEQ, vec![ty_tp(Str)])), 2);
        str_seq.register_builtin_erg_impl(FUNC_LEN, fn0_met(Str, Nat), Const, Public);
        str_seq.register_builtin_erg_impl(FUNC_GET, fn1_met(Str, Nat, Str), Const, Public);
        str_.register_trait(Str, str_seq);
        let mut str_add = Self::builtin_methods(Some(poly(ADD, vec![ty_tp(Str)])), 2);
        str_add.register_builtin_erg_impl(OP_ADD, fn1_met(Str, Str, Str), Const, Public);
        str_add.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Str));
        str_.register_trait(Str, str_add);
        let mut str_mul = Self::builtin_methods(Some(poly(MUL, vec![ty_tp(Nat)])), 2);
        str_mul.register_builtin_erg_impl(OP_MUL, fn1_met(Str, Nat, Str), Const, Public);
        str_mul.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(Str));
        str_.register_trait(Str, str_mul);
        let mut str_mutizable = Self::builtin_methods(Some(mono(MUTIZABLE)), 2);
        str_mutizable.register_builtin_const(
            MUTABLE_MUT_TYPE,
            Public,
            ValueObj::builtin_t(mono(MUT_STR)),
        );
        str_.register_trait(Str, str_mutizable);
        let mut str_show = Self::builtin_methods(Some(mono(SHOW)), 1);
        str_show.register_builtin_erg_impl(TO_STR, fn0_met(Str, Str), Immutable, Public);
        str_.register_trait(Str, str_show);
        let mut str_iterable = Self::builtin_methods(Some(poly(ITERABLE, vec![ty_tp(Str)])), 2);
        str_iterable.register_builtin_py_impl(
            FUNC_ITER,
            fn0_met(Str, mono(STR_ITERATOR)),
            Immutable,
            Public,
            Some(FUNDAMENTAL_ITER),
        );
        str_iterable.register_builtin_const(ITERATOR, vis, ValueObj::builtin_t(mono(STR_ITERATOR)));
        str_.register_trait(Str, str_iterable);
        /* NoneType */
        let mut nonetype = Self::builtin_mono_class(NONE_TYPE, 10);
        nonetype.register_superclass(Obj, &obj);
        let mut nonetype_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        nonetype_eq.register_builtin_erg_impl(
            OP_EQ,
            fn1_met(NoneType, NoneType, Bool),
            Const,
            Public,
        );
        nonetype.register_trait(NoneType, nonetype_eq);
        let mut nonetype_show = Self::builtin_methods(Some(mono(SHOW)), 1);
        nonetype_show.register_builtin_erg_impl(TO_STR, fn0_met(NoneType, Str), Immutable, Public);
        nonetype.register_trait(NoneType, nonetype_show);
        /* Type */
        let mut type_ = Self::builtin_mono_class(TYPE, 2);
        type_.register_superclass(Obj, &obj);
        type_.register_builtin_erg_impl(
            FUNC_MRO,
            array_t(Type, TyParam::erased(Nat)),
            Immutable,
            Public,
        );
        type_.register_marker_trait(mono(NAMED));
        let mut type_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        type_eq.register_builtin_erg_impl(OP_EQ, fn1_met(Type, Type, Bool), Const, Public);
        type_.register_trait(Type, type_eq);
        let mut class_type = Self::builtin_mono_class(CLASS_TYPE, 2);
        class_type.register_superclass(Type, &type_);
        class_type.register_marker_trait(mono(NAMED));
        let mut class_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        class_eq.register_builtin_erg_impl(
            OP_EQ,
            fn1_met(ClassType, ClassType, Bool),
            Const,
            Public,
        );
        class_type.register_trait(ClassType, class_eq);
        let mut trait_type = Self::builtin_mono_class(TRAIT_TYPE, 2);
        trait_type.register_superclass(Type, &type_);
        trait_type.register_marker_trait(mono(NAMED));
        let mut trait_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        trait_eq.register_builtin_erg_impl(
            OP_EQ,
            fn1_met(TraitType, TraitType, Bool),
            Const,
            Public,
        );
        trait_type.register_trait(TraitType, trait_eq);
        let mut code = Self::builtin_mono_class(CODE, 10);
        code.register_superclass(Obj, &obj);
        code.register_builtin_erg_impl(FUNC_CO_ARGCOUNT, Nat, Immutable, Public);
        code.register_builtin_erg_impl(
            FUNC_CO_VARNAMES,
            array_t(Str, TyParam::erased(Nat)),
            Immutable,
            Public,
        );
        code.register_builtin_erg_impl(
            FUNC_CO_CONSTS,
            array_t(Obj, TyParam::erased(Nat)),
            Immutable,
            Public,
        );
        code.register_builtin_erg_impl(
            FUNC_CO_NAMES,
            array_t(Str, TyParam::erased(Nat)),
            Immutable,
            Public,
        );
        code.register_builtin_erg_impl(
            FUNC_CO_FREEVARS,
            array_t(Str, TyParam::erased(Nat)),
            Immutable,
            Public,
        );
        code.register_builtin_erg_impl(
            FUNC_CO_CELLVARS,
            array_t(Str, TyParam::erased(Nat)),
            Immutable,
            Public,
        );
        code.register_builtin_erg_impl(FUNC_CO_FILENAME, Str, Immutable, Public);
        code.register_builtin_erg_impl(FUNC_CO_NAME, Str, Immutable, Public);
        code.register_builtin_erg_impl(FUNC_CO_FIRSTLINENO, Nat, Immutable, Public);
        code.register_builtin_erg_impl(FUNC_CO_STACKSIZE, Nat, Immutable, Public);
        code.register_builtin_erg_impl(FUNC_CO_FLAGS, Nat, Immutable, Public);
        code.register_builtin_erg_impl(FUNC_CO_CODE, mono(BYTES), Immutable, Public);
        code.register_builtin_erg_impl(FUNC_CO_LNOTAB, mono(BYTES), Immutable, Public);
        code.register_builtin_erg_impl(FUNC_CO_NLOCALS, Nat, Immutable, Public);
        code.register_builtin_erg_impl(FUNC_CO_KWONLYARGCOUNT, Nat, Immutable, Public);
        code.register_builtin_erg_impl(FUNC_CO_POSONLYARGCOUNT, Nat, Immutable, Public);
        let mut code_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        code_eq.register_builtin_erg_impl(OP_EQ, fn1_met(Code, Code, Bool), Const, Public);
        code.register_trait(Code, code_eq);
        let g_module_t = mono(GENERIC_MODULE);
        let mut generic_module = Self::builtin_mono_class(GENERIC_MODULE, 2);
        generic_module.register_superclass(Obj, &obj);
        generic_module.register_marker_trait(mono(NAMED));
        let mut generic_module_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        generic_module_eq.register_builtin_erg_impl(
            OP_EQ,
            fn1_met(g_module_t.clone(), g_module_t.clone(), Bool),
            Const,
            Public,
        );
        generic_module.register_trait(g_module_t.clone(), generic_module_eq);
        let Path = mono_q_tp(PATH, instanceof(Str));
        let module_t = module(Path.clone());
        let py_module_t = py_module(Path);
        let mut module = Self::builtin_poly_class(MODULE, vec![PS::named_nd(PATH, Str)], 2);
        module.register_superclass(g_module_t.clone(), &generic_module);
        let mut py_module = Self::builtin_poly_class(PY_MODULE, vec![PS::named_nd(PATH, Str)], 2);
        if !cfg!(feature = "py_compatible") {
            py_module.register_superclass(g_module_t.clone(), &generic_module);
        }
        /* Array */
        let mut array_ =
            Self::builtin_poly_class(ARRAY, vec![PS::t_nd(TY_T), PS::named_nd(TY_N, Nat)], 10);
        array_.register_superclass(Obj, &obj);
        array_.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        let arr_t = array_t(T.clone(), N.clone());
        let t = fn_met(
            arr_t.clone(),
            vec![kw(KW_RHS, array_t(T.clone(), M.clone()))],
            None,
            vec![],
            array_t(T.clone(), N.clone() + M.clone()),
        )
        .quantify();
        array_.register_builtin_py_impl(FUNC_CONCAT, t.clone(), Immutable, Public, Some(OP_ADD));
        let t_count =
            fn_met(arr_t.clone(), vec![kw(KW_X, T.clone())], None, vec![], Nat).quantify();
        array_.register_builtin_py_impl(FUNC_COUNT, t_count, Immutable, Public, Some(FUNC_COUNT));
        // Array(T, N)|<: Add(Array(T, M))|.
        //     Output = Array(T, N + M)
        //     __add__: (self: Array(T, N), other: Array(T, M)) -> Array(T, N + M) = Array.concat
        let mut array_add = Self::builtin_methods(
            Some(poly(ADD, vec![ty_tp(array_t(T.clone(), M.clone()))])),
            2,
        );
        array_add.register_builtin_erg_impl(OP_ADD, t, Immutable, Public);
        let out_t = array_t(T.clone(), N.clone() + M.clone());
        array_add.register_builtin_const(OUTPUT, Public, ValueObj::builtin_t(out_t));
        array_.register_trait(arr_t.clone(), array_add);
        let t = fn_met(
            arr_t.clone(),
            vec![kw(KW_ELEM, T.clone())],
            None,
            vec![],
            array_t(T.clone(), N.clone() + value(1usize)),
        )
        .quantify();
        array_.register_builtin_erg_impl(FUNC_PUSH, t, Immutable, Public);
        // [T; N].MutType! = [T; !N] (neither [T!; N] nor [T; N]!)
        let mut_type = ValueObj::builtin_t(poly(
            MUT_ARRAY,
            vec![TyParam::t(T.clone()), N.clone().mutate()],
        ));
        array_.register_builtin_const(MUTABLE_MUT_TYPE, Public, mut_type);
        let var = StrStruct::from(fresh_varname());
        let input = refinement(
            var.clone(),
            Nat,
            set! { Predicate::le(var, N.clone() - value(1usize)) },
        );
        // __getitem__: |T, N|(self: [T; N], _: {I: Nat | I <= N}) -> T
        let array_getitem_t =
            fn1_kw_met(array_t(T.clone(), N.clone()), anon(input), T.clone()).quantify();
        let get_item = ValueObj::Subr(ConstSubr::Builtin(BuiltinConstSubr::new(
            FUNDAMENTAL_GETITEM,
            __array_getitem__,
            array_getitem_t,
            None,
        )));
        array_.register_builtin_const(FUNDAMENTAL_GETITEM, Public, get_item);
        let mut array_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        array_eq.register_builtin_erg_impl(
            OP_EQ,
            fn1_met(arr_t.clone(), arr_t.clone(), Bool).quantify(),
            Const,
            Public,
        );
        array_.register_trait(arr_t.clone(), array_eq);
        array_.register_marker_trait(mono(MUTIZABLE));
        array_.register_marker_trait(poly(SEQ, vec![ty_tp(T.clone())]));
        let mut array_show = Self::builtin_methods(Some(mono(SHOW)), 1);
        array_show.register_builtin_py_impl(
            TO_STR,
            fn0_met(arr_t.clone(), Str).quantify(),
            Immutable,
            Public,
            Some(FUNDAMENTAL_STR),
        );
        array_.register_trait(arr_t.clone(), array_show);
        let mut array_iterable =
            Self::builtin_methods(Some(poly(ITERABLE, vec![ty_tp(T.clone())])), 2);
        let array_iter = poly(ARRAY_ITERATOR, vec![ty_tp(T.clone())]);
        let t = fn0_met(array_t(T.clone(), TyParam::erased(Nat)), array_iter.clone()).quantify();
        array_iterable.register_builtin_py_impl(
            FUNC_ITER,
            t,
            Immutable,
            Public,
            Some(FUNDAMENTAL_ITER),
        );
        array_iterable.register_builtin_const(ITERATOR, vis, ValueObj::builtin_t(array_iter));
        array_.register_trait(arr_t.clone(), array_iterable);
        let t = fn1_met(
            array_t(T.clone(), TyParam::erased(Nat)),
            func1(T.clone(), Bool),
            tuple_t(vec![
                array_t(T.clone(), TyParam::erased(Nat)),
                array_t(T.clone(), TyParam::erased(Nat)),
            ]),
        );
        array_.register_builtin_erg_impl(FUNC_PARTITION, t.quantify(), Immutable, Public);
        let t = fn_met(
            array_t(T.clone(), TyParam::erased(Nat)),
            vec![],
            None,
            vec![kw("f", or(func1(T.clone(), Bool), NoneType))],
            array_t(T.clone(), TyParam::erased(Nat)),
        );
        array_.register_builtin_erg_impl(FUNC_DEDUP, t.quantify(), Immutable, Public);
        /* Set */
        let mut set_ =
            Self::builtin_poly_class(SET, vec![PS::t_nd(TY_T), PS::named_nd(TY_N, Nat)], 10);
        let set_t = set_t(T.clone(), N.clone());
        set_.register_superclass(Obj, &obj);
        set_.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        let t = fn_met(
            set_t.clone(),
            vec![kw(KW_RHS, array_t(T.clone(), M.clone()))],
            None,
            vec![],
            array_t(T.clone(), N.clone() + M),
        )
        .quantify();
        set_.register_builtin_erg_impl(FUNC_CONCAT, t, Immutable, Public);
        let mut_type = ValueObj::builtin_t(poly(
            MUT_SET,
            vec![TyParam::t(T.clone()), N.clone().mutate()],
        ));
        set_.register_builtin_const(MUTABLE_MUT_TYPE, Public, mut_type);
        let mut set_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        set_eq.register_builtin_erg_impl(
            OP_EQ,
            fn1_met(set_t.clone(), set_t.clone(), Bool).quantify(),
            Const,
            Public,
        );
        set_.register_trait(set_t.clone(), set_eq);
        set_.register_marker_trait(mono(MUTIZABLE));
        set_.register_marker_trait(poly(SEQ, vec![ty_tp(T.clone())]));
        let mut set_show = Self::builtin_methods(Some(mono(SHOW)), 1);
        set_show.register_builtin_erg_impl(
            TO_STR,
            fn0_met(set_t.clone(), Str).quantify(),
            Immutable,
            Public,
        );
        set_.register_trait(set_t.clone(), set_show);
        let g_dict_t = mono(GENERIC_DICT);
        let mut generic_dict = Self::builtin_mono_class(GENERIC_DICT, 2);
        generic_dict.register_superclass(Obj, &obj);
        let mut generic_dict_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        generic_dict_eq.register_builtin_erg_impl(
            OP_EQ,
            fn1_met(g_dict_t.clone(), g_dict_t.clone(), Bool).quantify(),
            Const,
            Public,
        );
        generic_dict.register_trait(g_dict_t.clone(), generic_dict_eq);
        let D = mono_q_tp(TY_D, instanceof(mono(GENERIC_DICT)));
        // .get: _: T -> T or None
        let dict_get_t = fn1_met(g_dict_t.clone(), T.clone(), or(T.clone(), NoneType)).quantify();
        generic_dict.register_builtin_erg_impl(FUNC_GET, dict_get_t, Immutable, Public);
        let dict_t = poly(DICT, vec![D.clone()]);
        let mut dict_ =
            // TODO: D <: GenericDict
            Self::builtin_poly_class(DICT, vec![PS::named_nd(TY_D, mono(GENERIC_DICT))], 10);
        dict_.register_superclass(g_dict_t.clone(), &generic_dict);
        dict_.register_marker_trait(poly(OUTPUT, vec![D.clone()]));
        // __getitem__: _: T -> D[T]
        let dict_getitem_t = fn1_met(
            dict_t.clone(),
            T.clone(),
            proj_call(D, FUNDAMENTAL_GETITEM, vec![ty_tp(T.clone())]),
        )
        .quantify();
        let get_item = ValueObj::Subr(ConstSubr::Builtin(BuiltinConstSubr::new(
            FUNDAMENTAL_GETITEM,
            __dict_getitem__,
            dict_getitem_t,
            None,
        )));
        dict_.register_builtin_const(FUNDAMENTAL_GETITEM, Public, get_item);
        /* Bytes */
        let mut bytes = Self::builtin_mono_class(BYTES, 2);
        bytes.register_superclass(Obj, &obj);
        let decode_t = pr_met(
            mono(BYTES),
            vec![],
            None,
            vec![kw(KW_ENCODING, Str), kw(KW_ERRORS, Str)],
            Str,
        );
        bytes.register_builtin_erg_impl(FUNC_DECODE, decode_t, Immutable, Public);
        /* GenericTuple */
        let mut generic_tuple = Self::builtin_mono_class(GENERIC_TUPLE, 1);
        generic_tuple.register_superclass(Obj, &obj);
        let mut tuple_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        tuple_eq.register_builtin_erg_impl(
            OP_EQ,
            fn1_met(mono(GENERIC_TUPLE), mono(GENERIC_TUPLE), Bool),
            Const,
            Public,
        );
        generic_tuple.register_trait(mono(GENERIC_TUPLE), tuple_eq);
        let Ts = mono_q_tp(TY_TS, instanceof(array_t(Type, N.clone())));
        // Ts <: GenericArray
        let _tuple_t = poly(TUPLE, vec![Ts.clone()]);
        let mut tuple_ = Self::builtin_poly_class(
            TUPLE,
            vec![PS::named_nd(TY_TS, array_t(Type, N.clone()))],
            2,
        );
        tuple_.register_superclass(mono(GENERIC_TUPLE), &generic_tuple);
        tuple_.register_marker_trait(poly(OUTPUT, vec![Ts.clone()]));
        // __Tuple_getitem__: (self: Tuple(Ts), _: {N}) -> Ts[N]
        let return_t = proj_call(Ts, FUNDAMENTAL_GETITEM, vec![N.clone()]);
        let tuple_getitem_t =
            fn1_met(_tuple_t.clone(), tp_enum(Nat, set! {N}), return_t).quantify();
        tuple_.register_builtin_py_impl(
            FUNDAMENTAL_TUPLE_GETITEM,
            tuple_getitem_t.clone(),
            Const,
            Public,
            Some(FUNDAMENTAL_GETITEM),
        );
        // `__Tuple_getitem__` and `__getitem__` are the same thing
        // but `x.0` => `x__Tuple_getitem__(0)` determines that `x` is a tuple, which is better for type inference.
        tuple_.register_builtin_py_impl(
            FUNDAMENTAL_GETITEM,
            tuple_getitem_t,
            Const,
            Public,
            Some(FUNDAMENTAL_GETITEM),
        );
        /* record */
        let mut record = Self::builtin_mono_class(RECORD, 2);
        record.register_superclass(Obj, &obj);
        /* Or (true or type) */
        let or_t = poly(OR, vec![ty_tp(L), ty_tp(R)]);
        let mut or = Self::builtin_poly_class(OR, vec![PS::t_nd(TY_L), PS::t_nd(TY_R)], 2);
        or.register_superclass(Obj, &obj);
        /* Iterators */
        let mut str_iterator = Self::builtin_mono_class(STR_ITERATOR, 1);
        str_iterator.register_superclass(Obj, &obj);
        str_iterator.register_marker_trait(poly(ITERABLE, vec![ty_tp(Str)]));
        str_iterator.register_marker_trait(poly(OUTPUT, vec![ty_tp(Str)]));
        let mut array_iterator = Self::builtin_poly_class(ARRAY_ITERATOR, vec![PS::t_nd(TY_T)], 1);
        array_iterator.register_superclass(Obj, &obj);
        array_iterator.register_marker_trait(poly(ITERABLE, vec![ty_tp(T.clone())]));
        array_iterator.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        let mut range_iterator = Self::builtin_poly_class(RANGE_ITERATOR, vec![PS::t_nd(TY_T)], 1);
        range_iterator.register_superclass(Obj, &obj);
        range_iterator.register_marker_trait(poly(ITERABLE, vec![ty_tp(T.clone())]));
        range_iterator.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        /* Enumerate */
        let mut enumerate = Self::builtin_poly_class(ENUMERATE, vec![PS::t_nd(TY_T)], 2);
        enumerate.register_superclass(Obj, &obj);
        enumerate.register_marker_trait(poly(ITERABLE, vec![ty_tp(T.clone())]));
        enumerate.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        /* Filter */
        let mut filter = Self::builtin_poly_class(FILTER, vec![PS::t_nd(TY_T)], 2);
        filter.register_superclass(Obj, &obj);
        filter.register_marker_trait(poly(ITERABLE, vec![ty_tp(T.clone())]));
        filter.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        /* Map */
        let mut map = Self::builtin_poly_class(MAP, vec![PS::t_nd(TY_T)], 2);
        map.register_superclass(Obj, &obj);
        map.register_marker_trait(poly(ITERABLE, vec![ty_tp(T.clone())]));
        map.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        /* Reversed */
        let mut reversed = Self::builtin_poly_class(REVERSED, vec![PS::t_nd(TY_T)], 2);
        reversed.register_superclass(Obj, &obj);
        reversed.register_marker_trait(poly(ITERABLE, vec![ty_tp(T.clone())]));
        reversed.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        /* Zip */
        let mut zip = Self::builtin_poly_class(ZIP, vec![PS::t_nd(TY_T), PS::t_nd(TY_U)], 2);
        zip.register_superclass(Obj, &obj);
        zip.register_marker_trait(poly(
            ITERABLE,
            vec![ty_tp(tuple_t(vec![T.clone(), U.clone()]))],
        ));
        zip.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        zip.register_marker_trait(poly(OUTPUT, vec![ty_tp(U.clone())]));
        let mut obj_mut = Self::builtin_mono_class(MUTABLE_OBJ, 2);
        obj_mut.register_superclass(Obj, &obj);
        let mut obj_mut_mutable = Self::builtin_methods(Some(mono(MUTABLE)), 2);
        obj_mut_mutable.register_builtin_const(IMMUT_TYPE, Public, ValueObj::builtin_t(Obj));
        let f_t = kw(KW_FUNC, func(vec![kw(KW_OLD, Int)], None, vec![], Int));
        let t = pr_met(
            ref_mut(mono(MUTABLE_OBJ), None),
            vec![f_t],
            None,
            vec![],
            NoneType,
        );
        obj_mut_mutable.register_builtin_erg_impl(PROC_UPDATE, t, Immutable, Public);
        obj_mut.register_trait(mono(MUTABLE_OBJ), obj_mut_mutable);
        /* Float! */
        let mut float_mut = Self::builtin_mono_class(MUT_FLOAT, 2);
        float_mut.register_superclass(Float, &float);
        let mut float_mut_mutable = Self::builtin_methods(Some(mono(MUTABLE)), 2);
        float_mut_mutable.register_builtin_const(IMMUT_TYPE, Public, ValueObj::builtin_t(Float));
        let f_t = kw(KW_FUNC, func(vec![kw(KW_OLD, Float)], None, vec![], Float));
        let t = pr_met(
            ref_mut(mono(MUT_FLOAT), None),
            vec![f_t],
            None,
            vec![],
            NoneType,
        );
        float_mut_mutable.register_builtin_erg_impl(PROC_UPDATE, t, Immutable, Public);
        float_mut.register_trait(mono(MUT_FLOAT), float_mut_mutable);
        /* Ratio! */
        let mut ratio_mut = Self::builtin_mono_class(MUT_RATIO, 2);
        ratio_mut.register_superclass(Ratio, &ratio);
        let mut ratio_mut_mutable = Self::builtin_methods(Some(mono(MUTABLE)), 2);
        ratio_mut_mutable.register_builtin_const(IMMUT_TYPE, Public, ValueObj::builtin_t(Ratio));
        let f_t = kw(KW_FUNC, func(vec![kw(KW_OLD, Ratio)], None, vec![], Ratio));
        let t = pr_met(
            ref_mut(mono(MUT_RATIO), None),
            vec![f_t],
            None,
            vec![],
            NoneType,
        );
        ratio_mut_mutable.register_builtin_erg_impl(PROC_UPDATE, t, Immutable, Public);
        ratio_mut.register_trait(mono(MUT_RATIO), ratio_mut_mutable);
        /* Int! */
        let mut int_mut = Self::builtin_mono_class(MUT_INT, 2);
        int_mut.register_superclass(Int, &int);
        int_mut.register_superclass(mono(MUT_FLOAT), &float_mut);
        let t = pr_met(mono(MUT_INT), vec![], None, vec![kw("i", Int)], NoneType);
        int_mut.register_builtin_py_impl(PROC_INC, t.clone(), Immutable, Public, Some(FUNC_INC));
        int_mut.register_builtin_py_impl(PROC_DEC, t, Immutable, Public, Some(FUNC_DEC));
        let mut int_mut_mutable = Self::builtin_methods(Some(mono(MUTABLE)), 2);
        int_mut_mutable.register_builtin_const(IMMUT_TYPE, Public, ValueObj::builtin_t(Int));
        let f_t = kw(KW_FUNC, func(vec![kw(KW_OLD, Int)], None, vec![], Int));
        let t = pr_met(
            ref_mut(mono(MUT_INT), None),
            vec![f_t],
            None,
            vec![],
            NoneType,
        );
        int_mut_mutable.register_builtin_erg_impl(PROC_UPDATE, t, Immutable, Public);
        int_mut.register_trait(mono(MUT_INT), int_mut_mutable);
        let mut nat_mut = Self::builtin_mono_class(MUT_NAT, 2);
        nat_mut.register_superclass(Nat, &nat);
        nat_mut.register_superclass(mono(MUT_INT), &int_mut);
        /* Nat! */
        let mut nat_mut_mutable = Self::builtin_methods(Some(mono(MUTABLE)), 2);
        nat_mut_mutable.register_builtin_const(IMMUT_TYPE, Public, ValueObj::builtin_t(Nat));
        let f_t = kw(KW_FUNC, func(vec![kw(KW_OLD, Nat)], None, vec![], Nat));
        let t = pr_met(
            ref_mut(mono(MUT_NAT), None),
            vec![f_t],
            None,
            vec![],
            NoneType,
        );
        nat_mut_mutable.register_builtin_erg_impl(PROC_UPDATE, t, Immutable, Public);
        nat_mut.register_trait(mono(MUT_NAT), nat_mut_mutable);
        /* Bool! */
        let mut bool_mut = Self::builtin_mono_class(MUT_BOOL, 2);
        bool_mut.register_superclass(Bool, &bool_);
        bool_mut.register_superclass(mono(MUT_NAT), &nat_mut);
        let mut bool_mut_mutable = Self::builtin_methods(Some(mono(MUTABLE)), 2);
        bool_mut_mutable.register_builtin_const(IMMUT_TYPE, Public, ValueObj::builtin_t(Bool));
        let f_t = kw(KW_FUNC, func(vec![kw(KW_OLD, Bool)], None, vec![], Bool));
        let t = pr_met(
            ref_mut(mono(MUT_BOOL), None),
            vec![f_t],
            None,
            vec![],
            NoneType,
        );
        bool_mut_mutable.register_builtin_erg_impl(PROC_UPDATE, t, Immutable, Public);
        bool_mut.register_trait(mono(MUT_BOOL), bool_mut_mutable);
        let t = pr0_met(mono(MUT_BOOL), NoneType);
        bool_mut.register_builtin_py_impl(PROC_INVERT, t, Immutable, Public, Some(FUNC_INVERT));
        /* Str! */
        let mut str_mut = Self::builtin_mono_class(MUT_STR, 2);
        str_mut.register_superclass(Str, &nonetype);
        let mut str_mut_mutable = Self::builtin_methods(Some(mono(MUTABLE)), 2);
        str_mut_mutable.register_builtin_const(IMMUT_TYPE, Public, ValueObj::builtin_t(Str));
        let f_t = kw(KW_FUNC, func(vec![kw(KW_OLD, Str)], None, vec![], Str));
        let t = pr_met(
            ref_mut(mono(MUT_STR), None),
            vec![f_t],
            None,
            vec![],
            NoneType,
        );
        str_mut_mutable.register_builtin_erg_impl(PROC_UPDATE, t, Immutable, Public);
        str_mut.register_trait(mono(MUT_STR), str_mut_mutable);
        let t = pr_met(
            ref_mut(mono(MUT_STR), None),
            vec![kw("s", Str)],
            None,
            vec![],
            NoneType,
        );
        str_mut.register_builtin_py_impl(PROC_PUSH, t, Immutable, Public, Some(FUNC_PUSH));
        let t = pr0_met(ref_mut(mono(MUT_STR), None), Str);
        str_mut.register_builtin_py_impl(PROC_POP, t, Immutable, Public, Some(FUNC_POP));
        let t = pr0_met(ref_mut(mono(MUT_STR), None), NoneType);
        str_mut.register_builtin_py_impl(PROC_CLEAR, t, Immutable, Public, Some(FUNC_CLEAR));
        let t = pr_met(
            ref_mut(mono(MUT_STR), None),
            vec![kw("idx", Nat), kw("s", Str)],
            None,
            vec![],
            NoneType,
        );
        str_mut.register_builtin_py_impl(PROC_INSERT, t, Immutable, Public, Some(FUNC_INSERT));
        let t = pr_met(
            ref_mut(mono(MUT_STR), None),
            vec![kw("idx", Nat)],
            None,
            vec![],
            Str,
        );
        str_mut.register_builtin_py_impl(PROC_REMOVE, t, Immutable, Public, Some(FUNC_REMOVE));
        /* File! */
        let mut file_mut = Self::builtin_mono_class(MUT_FILE, 2);
        let mut file_mut_readable = Self::builtin_methods(Some(mono(MUT_READABLE)), 1);
        file_mut_readable.register_builtin_py_impl(
            PROC_READ,
            pr_met(
                ref_mut(mono(MUT_FILE), None),
                vec![],
                None,
                vec![kw(KW_N, Int)],
                Str,
            ),
            Immutable,
            Public,
            Some(FUNC_READ),
        );
        file_mut.register_trait(mono(MUT_FILE), file_mut_readable);
        let mut file_mut_writable = Self::builtin_methods(Some(mono(MUT_WRITABLE)), 1);
        file_mut_writable.register_builtin_py_impl(
            PROC_WRITE,
            pr1_kw_met(ref_mut(mono(MUT_FILE), None), kw(KW_S, Str), Nat),
            Immutable,
            Public,
            Some(FUNC_WRITE),
        );
        file_mut.register_trait(mono(MUT_FILE), file_mut_writable);
        file_mut.register_marker_trait(mono(FILE_LIKE));
        file_mut.register_marker_trait(mono(MUT_FILE_LIKE));
        /* Array! */
        let N_MUT = mono_q_tp(TY_N, instanceof(mono(MUT_NAT)));
        let array_mut_t = poly(MUT_ARRAY, vec![ty_tp(T.clone()), N_MUT.clone()]);
        let mut array_mut_ = Self::builtin_poly_class(
            MUT_ARRAY,
            vec![PS::t_nd(TY_T), PS::named_nd(TY_N, mono(MUT_NAT))],
            2,
        );
        array_mut_.register_superclass(arr_t.clone(), &array_);
        let t = pr_met(
            ref_mut(
                array_mut_t.clone(),
                Some(poly(
                    MUT_ARRAY,
                    vec![ty_tp(T.clone()), N_MUT.clone() + value(1usize)],
                )),
            ),
            vec![kw(KW_ELEM, T.clone())],
            None,
            vec![],
            NoneType,
        )
        .quantify();
        array_mut_.register_builtin_py_impl(PROC_PUSH, t, Immutable, Public, Some(FUNC_APPEND));
        let t_extend = pr_met(
            ref_mut(
                array_mut_t.clone(),
                Some(poly(
                    MUT_ARRAY,
                    vec![ty_tp(T.clone()), TyParam::erased(mono(MUT_NAT))],
                )),
            ),
            vec![kw(KW_ITERABLE, poly(ITERABLE, vec![ty_tp(T.clone())]))],
            None,
            vec![],
            NoneType,
        )
        .quantify();
        array_mut_.register_builtin_py_impl(
            PROC_EXTEND,
            t_extend,
            Immutable,
            Public,
            Some(FUNC_EXTEND),
        );
        let t_insert = pr_met(
            ref_mut(
                array_mut_t.clone(),
                Some(poly(
                    MUT_ARRAY,
                    vec![ty_tp(T.clone()), N_MUT.clone() + value(1usize)],
                )),
            ),
            vec![kw(KW_INDEX, Nat), kw(KW_ELEM, T.clone())],
            None,
            vec![],
            NoneType,
        )
        .quantify();
        array_mut_.register_builtin_py_impl(
            PROC_INSERT,
            t_insert,
            Immutable,
            Public,
            Some(FUNC_INSERT),
        );
        let t_remove = pr_met(
            ref_mut(
                array_mut_t.clone(),
                Some(poly(
                    MUT_ARRAY,
                    vec![ty_tp(T.clone()), N_MUT.clone() - value(1usize)],
                )),
            ),
            vec![kw(KW_X, T.clone())],
            None,
            vec![],
            NoneType,
        )
        .quantify();
        array_mut_.register_builtin_py_impl(
            PROC_REMOVE,
            t_remove,
            Immutable,
            Public,
            Some(FUNC_REMOVE),
        );
        let t_pop = pr_met(
            ref_mut(
                array_mut_t.clone(),
                Some(poly(
                    MUT_ARRAY,
                    vec![ty_tp(T.clone()), N_MUT.clone() - value(1usize)],
                )),
            ),
            vec![],
            None,
            vec![kw(KW_INDEX, Nat)],
            T.clone(),
        )
        .quantify();
        array_mut_.register_builtin_py_impl(PROC_POP, t_pop, Immutable, Public, Some(FUNC_POP));
        let t_clear = pr0_met(
            ref_mut(
                array_mut_t.clone(),
                Some(poly(MUT_ARRAY, vec![ty_tp(T.clone()), value(0usize)])),
            ),
            NoneType,
        )
        .quantify();
        array_mut_.register_builtin_py_impl(
            PROC_CLEAR,
            t_clear,
            Immutable,
            Public,
            Some(FUNC_CLEAR),
        );
        let t_sort = pr_met(
            ref_mut(array_mut_t.clone(), None),
            vec![],
            None,
            vec![kw(
                KW_KEY,
                func(vec![kw(KW_X, T.clone())], None, vec![], mono(ORD)),
            )],
            NoneType,
        )
        .quantify();
        array_mut_.register_builtin_py_impl(PROC_SORT, t_sort, Immutable, Public, Some(FUNC_SORT));
        let t_reverse = pr0_met(ref_mut(array_mut_t.clone(), None), NoneType).quantify();
        array_mut_.register_builtin_py_impl(
            PROC_REVERSE,
            t_reverse,
            Immutable,
            Public,
            Some(FUNC_REVERSE),
        );
        let t = pr_met(
            array_mut_t.clone(),
            vec![kw(KW_FUNC, nd_func(vec![anon(T.clone())], None, T.clone()))],
            None,
            vec![],
            NoneType,
        )
        .quantify();
        array_mut_.register_builtin_erg_impl(PROC_STRICT_MAP, t, Immutable, Public);
        let f_t = kw(
            KW_FUNC,
            func(vec![kw(KW_OLD, arr_t.clone())], None, vec![], arr_t.clone()),
        );
        let t = pr_met(
            ref_mut(array_mut_t.clone(), None),
            vec![f_t],
            None,
            vec![],
            NoneType,
        )
        .quantify();
        let mut array_mut_mutable = Self::builtin_methods(Some(mono(MUTABLE)), 2);
        array_mut_mutable.register_builtin_erg_impl(PROC_UPDATE, t, Immutable, Public);
        array_mut_.register_trait(array_mut_t.clone(), array_mut_mutable);
        /* Set! */
        let set_mut_t = poly(MUT_SET, vec![ty_tp(T.clone()), N_MUT]);
        let mut set_mut_ = Self::builtin_poly_class(
            MUT_SET,
            vec![PS::t_nd(TY_T), PS::named_nd(TY_N, mono(MUT_NAT))],
            2,
        );
        set_mut_.register_superclass(set_t.clone(), &set_);
        // `add!` will erase N
        let t = pr_met(
            ref_mut(
                set_mut_t.clone(),
                Some(poly(
                    MUT_SET,
                    vec![ty_tp(T.clone()), TyParam::erased(mono(MUT_NAT))],
                )),
            ),
            vec![kw(KW_ELEM, T.clone())],
            None,
            vec![],
            NoneType,
        )
        .quantify();
        set_mut_.register_builtin_py_impl(PROC_ADD, t, Immutable, Public, Some(FUNC_ADD));
        let t = pr_met(
            set_mut_t.clone(),
            vec![kw(KW_FUNC, nd_func(vec![anon(T.clone())], None, T.clone()))],
            None,
            vec![],
            NoneType,
        )
        .quantify();
        set_mut_.register_builtin_erg_impl(PROC_STRICT_MAP, t, Immutable, Public);
        let f_t = kw(
            KW_FUNC,
            func(vec![kw(KW_OLD, set_t.clone())], None, vec![], set_t.clone()),
        );
        let t = pr_met(
            ref_mut(set_mut_t.clone(), None),
            vec![f_t],
            None,
            vec![],
            NoneType,
        )
        .quantify();
        let mut set_mut_mutable = Self::builtin_methods(Some(mono(MUTABLE)), 2);
        set_mut_mutable.register_builtin_erg_impl(PROC_UPDATE, t, Immutable, Public);
        set_mut_.register_trait(set_mut_t.clone(), set_mut_mutable);
        /* Range */
        let range_t = poly(RANGE, vec![TyParam::t(T.clone())]);
        let mut range = Self::builtin_poly_class(RANGE, vec![PS::t_nd(TY_T)], 2);
        // range.register_superclass(Obj, &obj);
        range.register_superclass(Type, &type_);
        range.register_marker_trait(poly(OUTPUT, vec![ty_tp(T.clone())]));
        range.register_marker_trait(poly(SEQ, vec![ty_tp(T.clone())]));
        let mut range_eq = Self::builtin_methods(Some(mono(EQ)), 2);
        range_eq.register_builtin_erg_impl(
            OP_EQ,
            fn1_met(range_t.clone(), range_t.clone(), Bool).quantify(),
            Const,
            Public,
        );
        range.register_trait(range_t.clone(), range_eq);
        let mut range_iterable =
            Self::builtin_methods(Some(poly(ITERABLE, vec![ty_tp(T.clone())])), 2);
        let range_iter = poly(RANGE_ITERATOR, vec![ty_tp(T.clone())]);
        range_iterable.register_builtin_py_impl(
            FUNC_ITER,
            fn0_met(range_t.clone(), range_iter.clone()).quantify(),
            Immutable,
            Public,
            Some(FUNDAMENTAL_ITER),
        );
        range_iterable.register_builtin_const(ITERATOR, vis, ValueObj::builtin_t(range_iter));
        range.register_trait(range_t.clone(), range_iterable);
        let range_getitem_t = fn1_kw_met(range_t.clone(), anon(T.clone()), T.clone()).quantify();
        let get_item = ValueObj::Subr(ConstSubr::Builtin(BuiltinConstSubr::new(
            FUNDAMENTAL_GETITEM,
            __range_getitem__,
            range_getitem_t,
            None,
        )));
        range.register_builtin_const(FUNDAMENTAL_GETITEM, Public, get_item);
        let mut g_callable = Self::builtin_mono_class(GENERIC_CALLABLE, 2);
        g_callable.register_superclass(Obj, &obj);
        let t_return = fn1_met(mono(GENERIC_CALLABLE), Obj, Never).quantify();
        g_callable.register_builtin_erg_impl(FUNC_RETURN, t_return, Immutable, Public);
        let mut g_generator = Self::builtin_mono_class(GENERIC_GENERATOR, 2);
        g_generator.register_superclass(mono(GENERIC_CALLABLE), &g_callable);
        let t_yield = fn1_met(mono(GENERIC_GENERATOR), Obj, Never).quantify();
        g_generator.register_builtin_erg_impl(FUNC_YIELD, t_yield, Immutable, Public);
        /* Proc */
        let mut proc = Self::builtin_mono_class(PROC, 2);
        proc.register_superclass(mono(GENERIC_CALLABLE), &g_callable);
        let mut named_proc = Self::builtin_mono_class(NAMED_PROC, 2);
        named_proc.register_superclass(mono(PROC), &proc);
        named_proc.register_marker_trait(mono(NAMED));
        /* Func */
        let mut func = Self::builtin_mono_class(FUNC, 2);
        func.register_superclass(mono(PROC), &proc);
        let mut named_func = Self::builtin_mono_class(NAMED_FUNC, 2);
        named_func.register_superclass(mono(FUNC), &func);
        named_func.register_marker_trait(mono(NAMED));
        let mut quant = Self::builtin_mono_class(QUANTIFIED, 2);
        quant.register_superclass(mono(PROC), &proc);
        let mut qfunc = Self::builtin_mono_class(QUANTIFIED_FUNC, 2);
        qfunc.register_superclass(mono(FUNC), &func);
        self.register_builtin_type(Never, never, vis, Const, Some(NEVER));
        self.register_builtin_type(Obj, obj, vis, Const, Some(FUNC_OBJECT));
        // self.register_type(mono(RECORD), vec![], record, Private, Const);
        self.register_builtin_type(Int, int, vis, Const, Some(FUNC_INT));
        self.register_builtin_type(Nat, nat, vis, Const, Some(NAT));
        self.register_builtin_type(Float, float, vis, Const, Some(FUNC_FLOAT));
        self.register_builtin_type(Ratio, ratio, vis, Const, Some(RATIO));
        let name = if cfg!(feature = "py_compatible") {
            FUNC_BOOL
        } else {
            BOOL
        };
        self.register_builtin_type(Bool, bool_, vis, Const, Some(name));
        let name = if cfg!(feature = "py_compatible") {
            FUNC_STR
        } else {
            STR
        };
        self.register_builtin_type(Str, str_, vis, Const, Some(name));
        self.register_builtin_type(NoneType, nonetype, vis, Const, Some(NONE_TYPE));
        self.register_builtin_type(Type, type_, vis, Const, Some(FUNC_TYPE));
        self.register_builtin_type(ClassType, class_type, vis, Const, Some(CLASS_TYPE));
        self.register_builtin_type(TraitType, trait_type, vis, Const, Some(TRAIT_TYPE));
        self.register_builtin_type(Code, code, vis, Const, Some(CODE_TYPE));
        self.register_builtin_type(
            g_module_t,
            generic_module,
            Private,
            Const,
            Some(MODULE_TYPE),
        );
        self.register_builtin_type(py_module_t, py_module, vis, Const, Some(MODULE_TYPE));
        self.register_builtin_type(arr_t, array_, vis, Const, Some(FUNC_LIST));
        self.register_builtin_type(set_t, set_, vis, Const, Some(FUNC_SET));
        self.register_builtin_type(g_dict_t, generic_dict, vis, Const, Some(FUNC_DICT));
        self.register_builtin_type(dict_t, dict_, vis, Const, Some(FUNC_DICT));
        self.register_builtin_type(mono(BYTES), bytes, vis, Const, Some(BYTES));
        self.register_builtin_type(
            mono(GENERIC_TUPLE),
            generic_tuple,
            Private,
            Const,
            Some(FUNC_TUPLE),
        );
        self.register_builtin_type(_tuple_t, tuple_, vis, Const, Some(FUNC_TUPLE));
        self.register_builtin_type(mono(RECORD), record, vis, Const, Some(RECORD));
        self.register_builtin_type(or_t, or, vis, Const, Some(UNION));
        self.register_builtin_type(
            mono(STR_ITERATOR),
            str_iterator,
            Private,
            Const,
            Some(FUNC_STR_ITERATOR),
        );
        self.register_builtin_type(
            poly(ARRAY_ITERATOR, vec![ty_tp(T.clone())]),
            array_iterator,
            Private,
            Const,
            Some(FUNC_ARRAY_ITERATOR),
        );
        self.register_builtin_type(
            poly(RANGE_ITERATOR, vec![ty_tp(T.clone())]),
            range_iterator,
            Private,
            Const,
            Some(RANGE_ITERATOR),
        );
        self.register_builtin_type(
            poly(ENUMERATE, vec![ty_tp(T.clone())]),
            enumerate,
            Private,
            Const,
            Some(FUNC_ENUMERATE),
        );
        self.register_builtin_type(
            poly(FILTER, vec![ty_tp(T.clone())]),
            filter,
            Private,
            Const,
            Some(FUNC_FILTER),
        );
        self.register_builtin_type(
            poly(MAP, vec![ty_tp(T.clone())]),
            map,
            Private,
            Const,
            Some(FUNC_MAP),
        );
        self.register_builtin_type(
            poly(REVERSED, vec![ty_tp(T.clone())]),
            reversed,
            Private,
            Const,
            Some(FUNC_REVERSED),
        );
        self.register_builtin_type(
            poly(ZIP, vec![ty_tp(T), ty_tp(U)]),
            zip,
            Private,
            Const,
            Some(FUNC_ZIP),
        );
        self.register_builtin_type(mono(MUT_FILE), file_mut, vis, Const, Some(FILE));
        self.register_builtin_type(array_mut_t, array_mut_, vis, Const, Some(FUNC_LIST));
        self.register_builtin_type(set_mut_t, set_mut_, vis, Const, Some(FUNC_SET));
        self.register_builtin_type(
            mono(GENERIC_CALLABLE),
            g_callable,
            vis,
            Const,
            Some(CALLABLE),
        );
        self.register_builtin_type(
            mono(GENERIC_GENERATOR),
            g_generator,
            vis,
            Const,
            Some(GENERATOR),
        );
        self.register_builtin_type(mono(PROC), proc, vis, Const, Some(PROC));
        self.register_builtin_type(mono(FUNC), func, vis, Const, Some(FUNC));
        self.register_builtin_type(range_t, range, vis, Const, Some(FUNC_RANGE));
        if !cfg!(feature = "py_compatible") {
            self.register_builtin_type(module_t, module, vis, Const, Some(MODULE_TYPE));
            self.register_builtin_type(mono(MUTABLE_OBJ), obj_mut, vis, Const, Some(FUNC_OBJECT));
            self.register_builtin_type(mono(MUT_INT), int_mut, vis, Const, Some(FUNC_INT));
            self.register_builtin_type(mono(MUT_NAT), nat_mut, vis, Const, Some(NAT));
            self.register_builtin_type(mono(MUT_FLOAT), float_mut, vis, Const, Some(FUNC_FLOAT));
            self.register_builtin_type(mono(MUT_RATIO), ratio_mut, vis, Const, Some(RATIO));
            self.register_builtin_type(mono(MUT_BOOL), bool_mut, vis, Const, Some(BOOL));
            self.register_builtin_type(mono(MUT_STR), str_mut, vis, Const, Some(STR));
            self.register_builtin_type(
                mono(NAMED_PROC),
                named_proc,
                Private,
                Const,
                Some(NAMED_PROC),
            );
            self.register_builtin_type(
                mono(NAMED_FUNC),
                named_func,
                Private,
                Const,
                Some(NAMED_FUNC),
            );
            self.register_builtin_type(mono(QUANTIFIED), quant, Private, Const, Some(QUANTIFIED));
            self.register_builtin_type(
                mono(QUANTIFIED_FUNC),
                qfunc,
                Private,
                Const,
                Some(QUANTIFIED_FUNC),
            );
        }
    }
}
