---- MODULE ExampleSearch ----
\* assistant, exemplos: abrir a campanha e a hipotese seguinte.
\* O seed nao tem ensaio. O record tem tests_passed=16 e latency_ms=4.

EXTENDS AcvcRules, TLC

Schema == {
  [name |-> "tests_passed", dir |-> "higher"],
  [name |-> "latency_ms", dir |-> "lower"]
  }

Tree == [tests_passed |-> 16, latency_ms |-> 4]
Variants == {"cache", "tree"}
Trials == [cache |-> <<>>, tree |-> <<Tree>>]

Ok ==
  /\ Status(Schema, Variants, Trials, {}, "cache") = "unmeasured"
  /\ Status(Schema, Variants, Trials, {}, "tree") = "on_frontier"

VARIABLE tick
Init == tick = 0
Next == tick' = tick
Spec == Init /\ [][Next]_tick
====
