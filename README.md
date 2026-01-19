# EasyPLS
An easy and readable positional logic system for Python. EasyPLS is not on PyPI at the moment, so you will need to clone the repo and build it using Maturin. Then `import easypls` into any Python project.

## Expressions
Expressions can be created in two ways: by building them from objects or by parsing them from a string.
The proposition` (a ∧ b) → c` can be expressed as either `Expr.If(Expr.And(Expr.Var("a"), Expr.Var("b")), Expr.Var("c"))` or as `Expr.parse("(a and b) -> c")`.
#### Exhaustive List:
* Conjunction: Expr.And, "and"
* Disjunction: Expr.Or, "or"
* Negation: Expr.Not, "not"
* Conditional: Expr.If, "->"
* Biconditional: Expr.Iff, "<->"
* Nand: Expr.Nand, "nand"
* Nor: Expr.Nor, "nor"
* Xor: Expr.Xor, "xor"
* True: Expr.T, "T"
* False: Expr.F, "F"
## Engine
The engine is used to store the truth-value of variables and evaluate expressions. To create an engine, run `engine = Engine()`. To define variables, use the method `define(self, name: str, value: bool)`. Conversely, to undefine a variable, use `undefine(self, name: str)`. Once you have defined all the variables in the expression, you can use the method `eval(expr: Expr)` to evaluate it. For example:
```
from easypls import Engine, Expr
engine = Engine()
some_proposition = Expr.parse("a and b")
engine.define("a", False)
engine.define("b", True)
engine.eval(some_proposition)			# Prints "False"
```
## SAT Solving
Many of EasyPLS's features are driven by its SAT-solving capabilities. SAT solving is determining if there is a satisfying assignment (one that makes the proposition true) for a proposition. This problem is at the core of system design, tautology checking, argument validity checking, and much more. To check if an expression is satisfiable, first turn it into its equisatisfiable conjunctive normal form (CNF) via the Tseitin transformation, then call `is_sat`. For example, we find that `Expr.parse("a or b").tseitin().is_sat()` evaluates to True, whereas `Expr.parse("a and not a").tseitin().is_sat()` evaluates to False.

### Extensions of SAT Solving
With SAT solving alone, you can do tautology/contradiction checking, logical equivalence checking, and argument verifying; however, EasyPLS provides built-in methods for all of these things.
```
from easypls import *

p = Expr.parse("a or not a")
print(p.is_tautology())						# True
print(p.is_contradiction())					# False

# De Morgan's Law
p = Expr.parse("not (a or b)")
q = Expr.parse("not a and not b")

print(p.is_logically_eq(q))					#True

# Fallacy of Affirming the Consequent
propositions = [Expr.parse(s) for s in [
	"a->b",
	"b",
]]

conclusion = Expr.Var("a")

is_valid_argument(propositions, conclusion)	# False
```
## Truth Tables
Truth tables display the value of a proposition given all possible assignments of its variables. Run it using `display_truth_table(s: str)` where s is the *string* representation of the proposition.

## Upcoming
The SAT solving code is very unoptimized at the moment, so optimizations are soon to come in the way of conflict-driven clause learning and proper backtracking. I also plan to add more methods for manually manipulating CNFs and expressions.
