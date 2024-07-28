# 25/07/2024

## main new things

- Basic type checker for Impl and Var, with context
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
### apply h
If the goal is `Goal(B)` and `h` is of type `A -> B`, transforms the goal into `App(h,Goal(A))`

## proven things today

- `a => (a => b) => b`
- `a => a`
- `a => b => a`

## TODO

- Add support for And and Couple

# 26/07/2024

## main new things

- support for and, fst, snd and couple
- support for not and pretty printing for not
- added intro and intros that doesnt take names
- added a cool text user interface and more precise goal

## commands added

### elim h (h is an and)
If `h` is of type `A ^ B` and `Goal(A)` then we transform the goal into `App(App(Goal(a->b->a),Fst(h1)),Snd(h1))`
### split
If `Goal(A ^ B)` then transforms it to `Couple(Goal(A), Goal(B))`
### absurd B
If `Goal(A)` then transforms it to `ExFalso(B, Goal(Bottom))`

# 27/07/2024

## main new things

- load and save of theorems
- mode emul
- or related commands

## commands added

### Assumption
If `Goal(A)` and `A` is in the hypotheses, then we complete the goal
### Left
If `Goal(A \/ B)` transforms it into a `Left(Goal(A), B)`
### Right
If `Goal(A \/ B)` transforms it into a `Left(Goal(B), A)`
### Elim h (h is an or)
If `h` is of type `A \/ B` and `Goal(C)` then we transform the goal into `match(h,Goal(A -> C),Goal(B -> C))`

## TODO

- make a real and better emul mode
- fix some theorem-related panics
- make an error system
- make a cancel command

# 28/07/2024

## main new things

- Archiving of the `intuitionist` branch, as i'm going for dependant type theory
- Removed everything
- Added back Lambdaterms : Var, Goal and Pi

## TODO

- remake already existing commands