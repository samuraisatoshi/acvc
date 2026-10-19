---- MODULE ExampleOrders ----
\* README caso C. Os tres numeros ja sao inteiros. O pai nao entra no status.

EXTENDS AcvcRules, TLC

Schema == {
  [name |-> "p95_ms", dir |-> "lower"],
  [name |-> "lost_writes", dir |-> "lower"],
  [name |-> "monthly_usd", dir |-> "lower"]
  }

Postgres == [p95_ms |-> 48, lost_writes |-> 0, monthly_usd |-> 220]
Document == [p95_ms |-> 19, lost_writes |-> 3, monthly_usd |-> 90]

Variants == {"api", "postgres", "document"}
Trials ==
  [ api |-> <<>>,
    postgres |-> <<Postgres>>,
    document |-> <<Document>> ]

St(rejected, v) == Status(Schema, Variants, Trials, rejected, v)

Ok ==
  /\ ~Dominates(Schema, Postgres, Document)
  /\ ~Dominates(Schema, Document, Postgres)
  /\ St({}, "api") = "unmeasured"
  /\ St({}, "postgres") = "on_frontier"
  /\ St({}, "document") = "on_frontier"
  /\ St({"document"}, "document") = "rejected"
  /\ St({"document"}, "postgres") = "on_frontier"
  /\ St({"document"}, "api") = "unmeasured"

VARIABLE tick
Init == tick = 0
Next == tick' = tick
Spec == Init /\ [][Next]_tick
====
