# 高级中间表示(HIR, High-level Intermediate Representation)

[![badge](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com%2Fdefault%2Fsource_up_to_date%3Fowner%3Derg-lang%26repos%3Derg%26ref%3Dmain%26path%3Ddoc/EN/compiler/hir.md%26commit_hash%3D8673a0ce564fd282d0ca586642fa7f002e8a3c50)](https://gezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com/default/source_up_to_date?owner=erg-lang&repos=erg&ref=main&path=doc/EN/compiler/hir.md&commit_hash=8673a0ce564fd282d0ca586642fa7f002e8a3c50)

HIR 是 Erg 编译器从 AST 生成的结构
此结构包含源代码中每个表达式的完整类型信息，并且在语法上已脱糖
AST与源代码一一对应(纯文本)，但是HIR去掉了不必要的代码信息，添加了省略的类型信息，所以HIR可以转换为源代码很难恢复
让我们在下面的代码中查看 HIR 的示例

```python
v = ![]
for! 0..10, i =>
    v.push! i
log v.sum()
```

从此代码生成的 AST 如下所示: 

```python
AST(Module[
    VarDef{
        sig: VarSignature{
            pat: VarPattern::Ident(None, VarName("v")),
            spec_t: None,
        },
        op: "=",
        body: Block[
            UnaryOp{
                op: "!",
                expr: Array([]),
            },
        ],
    },
    Call{
        obj: Accessor::Local("for!"),
        args: [
            BinOp{
                op: "..",
                lhs: Literal(0),
                rhs: Literal(10),
            },
            Lambda{
                sig: LambdaSignature{
                    params: [
                        ParamSignature{
                            pat: ParamPattern::Name(VarName("i")),
                        },
                    ],
                    spec_ret_t: None,
                },
                body: Block[
                    Call{
                        obj: Accessor::Attr{"v", "push!"},
                        args: [
                            Accessor::Local("i"),
                        ],
                    },
                ],
            },
        ],
    },
    Call{
        obj: Accessor::Local("log"),
        args: [
            Call{
                obj: Accessor::Attr("v", "sum"),
                args: [],
            }
        ],
    }
])
```

从 AST 生成的 HIR 如下所示: 

```python
HIR(Module[
    VarDef{
        sig: VarSignature{
            pat: VarPattern::Ident(None, Name("v")),
            t: [0..10, _]!,
        },
        op: "=",
        body: Block[
            expr: UnaryOp{
                op: "!",
                expr: Array([]),
                t: [0..10, 0]!,
            },
        ],
    },
    Call{
        obj: Accessor::Local{
            name: "for!",
            t: (Range Nat, Nat => NoneType) => NoneType,
        },
        args: [
            BinOp{
                op: "..",
                lhs: Literal(0),
                rhs: Literal(10),
                t: 0..10,
            },
            Lambda{
                sig: LambdaSignature{
                    params: [
                        ParamSignature{
                            pat: ParamPattern::Name(Name("i")),
                            t: 0..10,
                        },
                    ],
                    t: 0..10 => NoneType,
                },
                body: Block[
                    Call{
                        obj: Accessor::Attr{
                            obj: Accessor::Local("v"),
                            field: "push!",
                            t: Ref!(Self![T ~> T, N ~> N+1]).(Nat) => NoneType,
                        },
                        args: [
                            Accessor::Local("i"),
                        ],
                    },
                ],
            },
        ],
    },
    Call{
        obj: Accessor::Local{
            name: "log",
            *t: Obj => NoneType,
        },
        args: [
            Call{
                obj: Accessor::Attr{
                    obj: Accessor::Local("v"),
                    field: "sum",
                    t: [0..10, !_] -> Nat
                },
                args: [],
                t: Nat
            }
        ],
    }
])
```

对象类型被推断为尽可能小。另一方面，子例程推断实现存在的类型
因此，实际参数的类型和形式参数的类型可能不匹配