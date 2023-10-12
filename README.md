# Pun Calculus
A variant of Typed Lambda Calculus with generalized variable punning (ad-hoc polymorphism).

## Motivation

Interactive theorem proving is too often a solo effort by either the programmer or the program. This type system aims to improve the interactivity.

## Types

Typechecking is a graph-coloring problem to ensure that Cannot Determine Color does not get applied to the result of Greedy Infer.

$$greedy \ infer \quad \frac{f:A \to B \quad f:B \to C}{f:A \to C}$$

$$always \ follows \quad \frac{f: A \to B \quad f:¬ A \to B}{f(x): B}$$

$$into \ DeMorgan \quad \frac{f:¬A \quad f:¬B}{f:¬(A \ + \ B)}$$

$$out \ of \ DeMorgan \quad \frac{f:¬(A \ + \ B)}{f:¬A \quad f:¬B}$$

$$application \quad \frac{f:A \to B \quad f:A \to C \quad x:A}{f(x): \ B \ + \ C}$$

$$terminal \ cannot \ determine \ color \quad \frac{f:A \to B \quad f:B \to C \quad f:A \to C \quad x:A \quad x:B}{f(x) \vdash \bot}$$

$$terminal \ absurd \quad \frac{f:A \quad f:¬A}{f \vdash \bot}$$

$$terminal \ infer \ argument \quad \frac{f \vdash A \to B \quad f(x) \vdash B}{x \vdash A}$$

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

Greedy Infer is the gas. Determine Color is the brakes. The rules for graph coloring precedence can be customized. Greedy Infer may be strange to work with because making an Integer term 0 may ascribe a Type with a lot of extra information such as "1123 is prime." Obviously this comes with a steep computational cost, which makes the coloring precedence important.

The type system above is not strongly normalizing yet.

## Citations

[A very brisk introduction to Type Theory](https://ncatlab.org/nlab/show/type+theory)
