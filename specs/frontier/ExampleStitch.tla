---- MODULE ExampleStitch ----
\* README caso B. seam e coverage em centesimos: 0.42, 0.18, 0.07 e 0.71, 0.93.

EXTENDS AcvcRules, TLC

Schema == {
  [name |-> "seam", dir |-> "lower"],
  [name |-> "runtime_ms", dir |-> "lower"],
  [name |-> "coverage", dir |-> "higher"]
  }

Homography == [seam |-> 42, runtime_ms |-> 80, coverage |-> 71]
Mesh == [seam |-> 18, runtime_ms |-> 640, coverage |-> 93]
Blend == [seam |-> 7, runtime_ms |-> 710, coverage |-> 93]

Variants == {"homography", "mesh", "blend"}
Trials ==
  [ homography |-> <<Homography>>,
    mesh |-> <<Mesh>>,
    blend |-> <<Blend>> ]

St(v) == Status(Schema, Variants, Trials, {}, v)

Ok ==
  /\ ~Dominates(Schema, Blend, Mesh)
  /\ ~Dominates(Schema, Mesh, Blend)
  /\ ~Dominates(Schema, Homography, Mesh)
  /\ ~Dominates(Schema, Mesh, Homography)
  /\ St("homography") = "on_frontier"
  /\ St("mesh") = "on_frontier"
  /\ St("blend") = "on_frontier"

VARIABLE tick
Init == tick = 0
Next == tick' = tick
Spec == Init /\ [][Next]_tick
====
