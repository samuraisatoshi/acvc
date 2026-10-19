---- MODULE ExampleTwoTrials ----
\* assistant, exemplos: dois ensaios incomparaveis ficam; o terceiro sai.

EXTENDS AcvcRules, TLC

Schema == {
  [name |-> "tests_passed", dir |-> "higher"],
  [name |-> "latency_ms", dir |-> "lower"]
  }

First == [tests_passed |-> 10, latency_ms |-> 10]
Second == [tests_passed |-> 1, latency_ms |-> 1]
Third == [tests_passed |-> 0, latency_ms |-> 20]

Variants == {"index"}
Trials == [index |-> <<First, Second, Third>>]
None == {}

Ok ==
  /\ ~Dominates(Schema, First, Second)
  /\ ~Dominates(Schema, Second, First)
  /\ Dominates(Schema, First, Third)
  /\ Dominates(Schema, Second, Third)
  /\ IsFrontierTrial(Schema, Variants, Trials, None, "index", First)
  /\ IsFrontierTrial(Schema, Variants, Trials, None, "index", Second)
  /\ ~IsFrontierTrial(Schema, Variants, Trials, None, "index", Third)
  /\ Status(Schema, Variants, Trials, None, "index") = "on_frontier"

VARIABLE tick
Init == tick = 0
Next == tick' = tick
Spec == Init /\ [][Next]_tick
====
