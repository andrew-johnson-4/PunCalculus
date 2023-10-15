# Pun Calculus
A variant of Typed Lambda Calculus with generalized variable punning (ad-hoc polymorphism).

## Contribution

Ad-Hoc Polymorphism is introduced to the Simply Typed Lambda Calculus by pluralizing lambda abstractions.
Terms such as `λx:X. y` are represented instead as `λ⟨x:X. y⟩`.
Plural abstractions are represented with more braces: `λ⟨a:A. b⟩⟨x:X. y⟩`.
The type system is then extended to provide a suprisingly rich set of logical primitives.

## Types

Typechecking is a graph-coloring problem to ensure that Cannot Determine Color does not get applied to the result of Greedy Infer.

$$intermediate \ greedy \ infer \quad \frac{f:A \to B \quad f:B \to C}{f:A \to C}$$

$$intermediate \ always \ follows \quad \frac{f: A \to B \quad f:¬ A \to B}{f(x): B}$$

$$intermediate \ into \ DeMorgan \quad \frac{f:¬A \quad f:¬B}{f:¬(A \ + \ B)}$$

$$intermediate \ out \ of \ DeMorgan \quad \frac{f:¬(A \ + \ B)}{f:¬A \quad f:¬B}$$

TODO: migrate intermediate results across the turnstile

$$terminal \ application \quad \frac{\Gamma \vdash f:(A \to B) + (A \to C) \quad \Gamma \vdash x:A \quad f(x)}{\Gamma \vdash f(x):B \ + \ C}$$

$$terminal \ cannot \ determine \ color \quad \frac{\Gamma \vdash f:(A \to B) + (B \to C) + (A \to C) \quad \Gamma \vdash x:A + B \quad f(x)}{f(x) \vdash \bot}$$

$$terminal \ absurd \quad \frac{\Gamma \vdash f:A + ¬A}{\Gamma \vdash \bot}$$

$$terminal \ infer \ argument \quad \frac{\Gamma \vdash f:A \to B \quad \Gamma \vdash f(x):B}{\Gamma \vdash x:A}$$

$$terminal \ abstraction \quad \frac{a \vdash A \quad b \vdash B \quad x \vdash X \quad y \vdash Y}{λ⟨a.b⟩⟨x.y⟩ \vdash (A \to B) + (X \to Y)}$$

$$terminal \ application \quad \frac{f \vdash (A \to B) + (C \to D) + (X \to Y) \quad x \vdash A + X}{f(x) \vdash B + Y}$$

$$terminal \ argument \quad \frac{f \vdash A \to B \quad f(x) \vdash B}{x \vdash A}$$

## Notes

_Concrete_ types are never ambiguous, they get collapsed into at most a plural type.
For this reason, the Cannot Determine Color rule fails when it is incapable of determining the concrete type.
Concrete types are types on the right of the turnstile, which every term needs and is the terminal state of typechecking.

_Plural_ types are similar to product types plus an implicit subtyping rule: A + B ⇒ A and A + B ⇒ B.

For simplicity, all bindings are modelled as morphisms.
An "object" A can be modelled as a simple morphism 1 → A.
Similarly ¬A is shorthand for 0 → A.

Greedy Infer is the gas. Determine Color is the brakes.
The rules for graph coloring precedence can be customized.
Greedy Infer may be strange to work with because making an Integer term 0 may ascribe a Type with a lot of extra information such as "1123 is prime."
Obviously this comes with a steep computational cost, which makes the coloring precedence important.

Some rules are termed "intermediate" because they do not immediately assign a concrete type to any term.
Intermediate rules are subject to coloring precedence.
All rules that do assign a concrete type are termed "terminal."

Traditionally strong normalization is proved by showing that all rules assign a concrete type,
thereby limiting inference to a linear number of steps.
Intermediate rules don't assign a concrete type, so strong normalization should be guaranteed
either by demonstrating forward progress or adding some sort of arbitrary limit.

Types are either singular or plural, never both.
If you want to turn A + B into a singular type, then you could write it as AB.

## Direct Citations

[A very brisk introduction to Type Theory](https://ncatlab.org/nlab/show/type+theory)

## Similar Work

[The Kernel of Ad Hoc Polymorphism](https://dspace.mit.edu/bitstream/handle/1721.1/106072/965197677-MIT.pdf)

## Possible Extensions

[Strongly Normalizing subsets of Lambda Calculus](https://cstheory.stackexchange.com/questions/20364/how-to-make-the-lambda-calculus-strong-normalizing-without-a-type-system)
