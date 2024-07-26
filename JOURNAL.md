# 25/07/2024

- Basic type checker for Impl and Var, with context

## main new things

- context-dependant hypothesis for the 4 commands added below
- type checker at the end of the proof
- proof verifier at the end
- shell to interactively input commands
- tests

## commands added

### intro h
If the Goal is of the form `Goal(A -> B)`, make it `Abs(h,A,Goal(B))`
### exact h
If the goal is `Goal(T)` and `h` is a variable of type `T` (in the context), transformes the goal in `Var(h)` (and finishes !)
### cut A
If the goal is `Goal(B)`, transforms it in `App(Goal(A -> B), Goal(A))` (creates a lemma)
## apply h
If the goal is `Goal(B)` and `h` is of type `A -> B`, transforms the goal into `App(h,Goal(A))`

## proven things today

- `a => (a => b) => b`
- `a => a`
- `a => b => a`
