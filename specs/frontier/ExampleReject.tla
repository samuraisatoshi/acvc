---- MODULE ExampleReject ----
\* assistant, exemplos: fechar de proposito. O gasto de 15000 tokens nao muda o status.

EXTENDS AcvcRules, TLC

Schema == {
  [name |-> "tests_passed", dir |-> "higher"],
  [name |-> "latency_ms", dir |-> "lower"]
  }

Tree == [tests_passed |-> 16, latency_ms |-> 4]
Variants == {"cache", "tree"}
Trials == [cache |-> <<>>, tree |-> <<Tree>>]

VARIABLES rejected, tokens
vars == <<rejected, tokens>>

St(v) == Status(Schema, Variants, Trials, rejected, v)

Init == rejected = {} /\ tokens = 0

Reject ==
  /\ rejected = {}
  /\ rejected' = {"tree"}
  /\ UNCHANGED tokens

Spend ==
  /\ tokens = 0
  /\ tokens' = 15000
  /\ UNCHANGED rejected

Next == Reject \/ Spend
Spec == Init /\ [][Next]_vars

Ok ==
  /\ tokens \in {0, 15000}
  /\ rejected \subseteq {"tree"}
  /\ rejected = {} => St("tree") = "on_frontier"
  /\ "tree" \in rejected => St("tree") = "rejected"
  /\ St("cache") = "unmeasured"
====
