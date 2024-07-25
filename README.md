# ProofAssist
A proof assistant made in rust for a TIPE

# Commands to implement
## intro h
si le but est `Goal(A -> B)`, transforme en `Abs(h,A,Goal(B))`
## exact h
si le but est `Goal(T)` et que `h` est une variable de type `T`, transforme le goal en `Var(h)` (et done le termine !)
## cut A
si le but est `Goal(B)`, le transforme en `App(Goal(A -> B), Goal(A))` (en gros permet la création d'un lemme)
## apply h
si le but est `Goal(B)` et que `h` est de type `A -> B`, transforme le goal en `App(h,Goal(A))`