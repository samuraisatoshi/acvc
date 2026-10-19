---- MODULE AcvcRules ----
\* Operadores da fronteira. Um ensaio ausente usa Missing.
\* dir e "higher" ou "lower". O status nao recebe token nem pai.

EXTENDS Integers, Sequences, FiniteSets

Missing == -1

ToSet(seq) == { seq[i] : i \in DOMAIN seq }

Complete(schema, t) == \A spec \in schema : t[spec.name] # Missing

Better(dir, left, right) ==
  IF dir = "higher" THEN left > right ELSE left < right

Dominates(schema, L, R) ==
  /\ Complete(schema, L) /\ Complete(schema, R)
  /\ \A spec \in schema :
       L[spec.name] = R[spec.name] \/ Better(spec.dir, L[spec.name], R[spec.name])
  /\ \E spec \in schema : Better(spec.dir, L[spec.name], R[spec.name])

Submitted(schema, seq) ==
  { t \in ToSet(seq) :
      /\ Complete(schema, t)
      /\ ~\E u \in ToSet(seq) : Dominates(schema, u, t) }

Eligible(schema, trials, rejected, v) ==
  IF v \in rejected THEN {} ELSE Submitted(schema, trials[v])

OnFrontier(schema, variants, trials, rejected, v) ==
  /\ v \in variants
  /\ v \notin rejected
  /\ \E t \in Eligible(schema, trials, rejected, v) :
       \A w \in variants \ {v} :
         \A u \in Eligible(schema, trials, rejected, w) :
           ~Dominates(schema, u, t)

HasComplete(schema, trials, v) ==
  \E t \in ToSet(trials[v]) : Complete(schema, t)

Status(schema, variants, trials, rejected, v) ==
  IF v \notin variants THEN "absent"
  ELSE IF v \in rejected THEN "rejected"
  ELSE IF OnFrontier(schema, variants, trials, rejected, v) THEN "on_frontier"
  ELSE IF HasComplete(schema, trials, v) THEN "superseded"
  ELSE "unmeasured"

IsFrontierTrial(schema, variants, trials, rejected, v, t) ==
  /\ t \in Eligible(schema, trials, rejected, v)
  /\ \A w \in variants \ {v} :
       \A u \in Eligible(schema, trials, rejected, w) :
         ~Dominates(schema, u, t)
====
