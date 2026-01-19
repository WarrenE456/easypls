from easypls import *

premises = [Expr.parse(s) for s in [
    "A -> (B and C)",
    "B -> (D or E)",
    "C -> F",
    "D -> G",
    "E -> G",
    "F -> H",
    "(G and H) -> I",
    "A",
]]

conclusion = Expr.Var("I")
valid = Expr.is_valid_argument(premises, conclusion)
print(f"Is argument 1 valid? {"yes" if valid else "no"}")

premises = [Expr.parse(s) for s in [
    "A -> (B and C)",
    "B -> D",
    "C -> E",
    "D or E",
    "F -> G",
    "G -> H",
    "A",
]]

conclusion = Expr.Var("H")
valid = Expr.is_valid_argument(premises, conclusion)
print(f"Is argument 2 valid? {"yes" if valid else "no"}")
