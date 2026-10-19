---- MODULE AcvcFrontier ----
\* Regras da fronteira e do status. Duas hipoteses, duas metricas, no maximo
\* dois ensaios cada. m1 quanto maior melhor. m2 quanto menor melhor.

EXTENDS Integers, Sequences, FiniteSets, TLC

CONSTANTS MaxTrials
ASSUME MaxTrials \in Nat /\ MaxTrials >= 1

VARIABLES present, origin, trials, rejected

vars == <<present, origin, trials, rejected>>

Variants == {"a", "b"}
Metrics == {"m1", "m2"}
Missing == -1
TrialSpace == [Metrics -> {0, 1, Missing}]

ToSet(seq) == {seq[i] : i \in DOMAIN seq}

Complete(t) == \A m \in Metrics : t[m] # Missing

Better(m, left, right) ==
  IF m = "m1" THEN left > right ELSE left < right

Dominates(L, R) ==
  /\ Complete(L) /\ Complete(R)
  /\ \A m \in Metrics : L[m] = R[m] \/ Better(m, L[m], R[m])
  /\ \E m \in Metrics : Better(m, L[m], R[m])

Submitted(v) ==
  { t \in ToSet(trials[v]) :
      /\ Complete(t)
      /\ ~\E u \in ToSet(trials[v]) : Dominates(u, t) }

Eligible(v) == IF v \in rejected THEN {} ELSE Submitted(v)

OnFrontier(v) ==
  /\ v \in present
  /\ v \notin rejected
  /\ \E t \in Eligible(v) :
       \A w \in present \ {v} :
         \A u \in Eligible(w) : ~Dominates(u, t)

HasComplete(v) == \E t \in ToSet(trials[v]) : Complete(t)

Status(v) ==
  IF v \notin present THEN "absent"
  ELSE IF v \in rejected THEN "rejected"
  ELSE IF OnFrontier(v) THEN "on_frontier"
  ELSE IF HasComplete(v) THEN "superseded"
  ELSE "unmeasured"

Init ==
  /\ present = {}
  /\ origin = "none"
  /\ trials = [v \in Variants |-> <<>>]
  /\ rejected = {}

Seed ==
  /\ present = {}
  /\ \E v \in Variants :
       /\ present' = {v}
       /\ origin' = v
       /\ UNCHANGED <<trials, rejected>>

Record ==
  /\ present # {}
  /\ present # Variants
  /\ \E v \in Variants \ present : present' = present \union {v}
  /\ UNCHANGED <<origin, trials, rejected>>

Score ==
  /\ \E v \in present :
       /\ Len(trials[v]) < MaxTrials
       /\ \E t \in TrialSpace :
            trials' = [trials EXCEPT ![v] = Append(@, t)]
  /\ UNCHANGED <<present, origin, rejected>>

Reject ==
  /\ \E v \in present \ rejected : rejected' = rejected \union {v}
  /\ UNCHANGED <<present, origin, trials>>

Next == Seed \/ Record \/ Score \/ Reject
Spec == Init /\ [][Next]_vars

TypeOK ==
  /\ present \subseteq Variants
  /\ origin \in (Variants \union {"none"})
  /\ \A v \in Variants : trials[v] \in Seq(TrialSpace) /\ Len(trials[v]) <= MaxTrials
  /\ rejected \subseteq present
  /\ (present = {}) <=> (origin = "none")
  /\ origin # "none" => origin \in present

InvRejectedOff == \A v \in rejected : ~OnFrontier(v)

InvIncompleteOff ==
  \A v \in present : \A t \in Submitted(v) : Complete(t)

InvSameVariantDropsWorse ==
  \A v \in present :
    \A t \in Submitted(v) :
      ~\E u \in ToSet(trials[v]) : Dominates(u, t)

InvNoSingleWinner ==
  \A v, w \in present :
    ( /\ v # w
      /\ v \notin rejected /\ w \notin rejected
      /\ Cardinality(Submitted(v)) = 1
      /\ Cardinality(Submitted(w)) = 1
      /\ LET tv == CHOOSE t \in Submitted(v) : TRUE
             tw == CHOOSE t \in Submitted(w) : TRUE
         IN ~Dominates(tv, tw) /\ ~Dominates(tw, tv)
    ) => (Status(v) = "on_frontier" /\ Status(w) = "on_frontier")

InvDominatedIsSuperseded ==
  \A v, w \in present :
    ( /\ v # w
      /\ v \notin rejected /\ w \notin rejected
      /\ Cardinality(Submitted(v)) = 1
      /\ Cardinality(Submitted(w)) = 1
      /\ LET tv == CHOOSE t \in Submitted(v) : TRUE
             tw == CHOOSE t \in Submitted(w) : TRUE
         IN Dominates(tw, tv)
    ) => (Status(v) = "superseded" /\ Status(w) = "on_frontier")

InvRejectedDoesNotDominate ==
  \A v \in present :
    ( /\ v \notin rejected
      /\ Submitted(v) # {}
      /\ \A w \in present \ {v} : w \in rejected \/ Submitted(w) = {}
    ) => Status(v) = "on_frontier"

InvStatus ==
  \A v \in present :
    /\ (Status(v) = "rejected") <=> (v \in rejected)
    /\ Status(v) = "superseded" => HasComplete(v) /\ ~OnFrontier(v) /\ v \notin rejected
    /\ Status(v) = "unmeasured" => ~HasComplete(v) /\ v \notin rejected
    /\ Status(v) = "on_frontier" => OnFrontier(v) /\ v \notin rejected

AppendOnly ==
  [][ \A v \in Variants :
        /\ Len(trials[v]) <= Len(trials'[v])
        /\ \A i \in 1..Len(trials[v]) : trials'[v][i] = trials[v][i]
   ]_trials

OriginOnce ==
  [][ \/ origin' = origin
     \/ (origin = "none" /\ origin' \in Variants)
   ]_origin

PresentGrows == [][present \subseteq present']_present
RejectGrows == [][rejected \subseteq rejected']_rejected
====
