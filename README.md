# Pun Calculus
A variant of Typed Lambda Calculus with generalized variable punning (ad-hoc polymorphism).

## Motivation

Interactive theorem proving is too often a solo effort by either the programmer or the program. This type system aims to improve the interactivity.

## Types

Typechecking is a graph-coloring problem to ensure that Cannot Determine Color does not get applied to the result of Greedy Infer.

$$greedy \ infer \quad \frac{f:A \to B \quad f:B \to C}{f:A \to C}$$

$$infer \ argument \quad \frac{f:A \to B \quad f(x):B}{x:A}$$

$$cannot \ determine \ color \quad \frac{f:A \to B \quad f:B \to C \quad f:A \to C}{\bot}$$

$$always \ follows \quad \frac{f: A \to B \quad f:¬ A \to B}{f(x): B}$$

$$into \ DeMorgan \quad \frac{f:¬A \quad f:¬B}{f:¬(A \ + \ B)}$$

$$out \ of \ DeMorgan \quad \frac{f:¬(A \ + \ B)}{f:¬A \quad f:¬B}$$

$$application \quad \frac{f:A \to B \quad f:A \to C \quad x:A}{f(x): \ B \ + \ C}$$

$$absurd \quad \frac{f:A \quad f:¬A}{\bot}$$

## Notes

For simplicity, all bindings are modelled as morphisms.
An "object" A can be modelled as a simple morphism 1 -> A.
Similarly ¬A is shorthand for 0 -> A.

Greedy Infer is the gas. Determine Color is the brakes. The rules for graph coloring precedence can be customized.

The type system above is not strongly normalizing yet.

## Citations

[A very brisk introduction to Type Theory](https://ncatlab.org/nlab/show/type+theory)
