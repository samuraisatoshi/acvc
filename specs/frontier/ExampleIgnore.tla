---- MODULE ExampleIgnore ----
\* assistant, exemplos: .acvcignore. Comentario e linha vazia nao sao regra.
\* A hipotese ja gravada nao muda quando o arquivo de ignore sai.

EXTENDS Integers, Sequences, FiniteSets, TLC

Lines == <<"# report", "lab/reports", "">>

IsRule(line) == line # "" /\ SubSeq(line, 1, 1) # "#"

RulesOf(lines) == { line \in { lines[i] : i \in DOMAIN lines } : IsRule(line) }

Ignored(rules, path) ==
  \E rule \in rules :
    \/ path = rule
    \/ /\ Len(path) > Len(rule)
       /\ SubSeq(path, 1, Len(rule)) = rule
       /\ SubSeq(path, Len(rule) + 1, Len(rule) + 1) = "/"

Paths == {"src/lib.rs", "lab/reports/out.txt", "lab/notes.txt"}
Rules == RulesOf(Lines)
Recorded == { path \in Paths : ~Ignored(Rules, path) }
AfterDelete == { path \in Paths : ~Ignored({}, path) }

Ok ==
  /\ Rules = {"lab/reports"}
  /\ "lab/reports/out.txt" \notin Recorded
  /\ "lab/notes.txt" \in Recorded
  /\ "src/lib.rs" \in Recorded
  /\ "lab/reports-extra" \notin { path \in {"lab/reports-extra"} : Ignored(Rules, path) }
  /\ AfterDelete = Paths
  /\ Recorded # AfterDelete

VARIABLE tick
Init == tick = 0
Next == tick' = tick
Spec == Init /\ [][Next]_tick
====
