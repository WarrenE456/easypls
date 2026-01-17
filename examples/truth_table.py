from easypls import *
from typing import List

def gen_truth_table(vars: List[str], prop: str):
    engine = Engine()
    expr = Expr.parse(prop)

    def helper(i: int):
        if i >= len(vars):
            for var in vars:
                print(f"{var}={engine.eval(Expr.Var(var))}, ", end="")
            print(engine.eval(expr))
        else:
            engine.define(vars[i], True)
            helper(i+1)
            engine.define(vars[i], False)
            helper(i+1)

    print(prop + ":")
    helper(0)

gen_truth_table(["a", "b"], "a -> b")
