# Type checking (型解析)

Ergの型解析は、フェーズとしてはASTをHIRに変換するlowering(低位化)フェーズの一部である。HIRはASTよりも若干コンパクトな構文木(中間表現)であり、全ての式に対し型が明示されている。
loweringを行うのは`ASTLowerer`であり、型解析を実行するのは`Context`という構造体である。

Ergの型解析は型検査と型推論の２つの側面を持つ。両者は同時に実行される。
型検査では、型指定と型環境を参照して項が規則通りに使用されているか検査する。型推論では、型指定がなされていない箇所に対して型変数を発行し、型環境を参照して単一化する。