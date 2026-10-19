---- MODULE ExampleIntegral ----
\* README caso A. Residuos vezes 10^10, para o TLC comparar inteiros.
\* A ordem e a mesma de 1.4e-3, 2.1e-8 e 4.0e-10.

EXTENDS AcvcRules, TLC

Schema == {
  [name |-> "residual", dir |-> "lower"],
  [name |-> "evaluations", dir |-> "lower"]
  }

Trapezoid == [residual |-> 14000000, evaluations |-> 400]
Simpson == [residual |-> 210, evaluations |-> 80]
Series == [residual |-> 4, evaluations |-> 360]

Variants == {"trapezoid", "simpson", "series", "singular"}
Trials ==
  [ trapezoid |-> <<Trapezoid>>,
    simpson |-> <<Simpson>>,
    series |-> <<Series>>,
    singular |-> <<>> ]

St(rejected, v) == Status(Schema, Variants, Trials, rejected, v)

Ok ==
  /\ Dominates(Schema, Simpson, Trapezoid)
  /\ ~Dominates(Schema, Simpson, Series)
  /\ ~Dominates(Schema, Series, Simpson)
  /\ St({}, "trapezoid") = "superseded"
  /\ St({}, "simpson") = "on_frontier"
  /\ St({}, "series") = "on_frontier"
  /\ St({}, "singular") = "unmeasured"
  /\ St({"trapezoid"}, "trapezoid") = "rejected"
  /\ St({"trapezoid"}, "simpson") = "on_frontier"
  /\ St({"trapezoid"}, "series") = "on_frontier"

VARIABLE tick
Init == tick = 0
Next == tick' = tick
Spec == Init /\ [][Next]_tick
====
