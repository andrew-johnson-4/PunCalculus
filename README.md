# Pun Calculus
A variant of Typed Lambda Calculus with generalized variable punning (ad-hoc polymorphism).

## Contribution

Ad-Hoc Polymorphism is introduced to the Simply Typed Lambda Calculus by pluralizing lambda abstractions.
Terms such as `λx:X. y` are represented instead as `λ⟨x:X. y⟩`.
Plural abstractions are represented with more braces: `λ⟨a:A. b⟩⟨x:X. y⟩`.
The type system is then extended to provide a surprisingly rich set of logical primitives.

## Motivation

In the LSTS proof assistant, it became apparent that plural abstractions are valuable.
This was observed in simple function applications `f(x)` where the function candidates could prove multiple properties depending on properties of the input.
This was the immediate origin of the concept of "plural" arrows that can carry multiple properties.
PunC is an attempt to generalize this practice before upgrading the LSTS framework.

## Types

$$intermediate \ greedy \ infer \quad \frac{f:A \to B \quad f:B \to C}{f:A \to C}$$

$$intermediate \ always \ follows \quad \frac{f: A \to B \quad f:¬ A \to B}{f(x): B}$$

$$intermediate \ into \ DeMorgan \quad \frac{f:¬A \quad f:¬B}{f:¬(A \ + \ B)}$$

$$intermediate \ out \ of \ DeMorgan \quad \frac{f:¬(A \ + \ B)}{f:¬A \quad f:¬B}$$

TODO: migrate intermediate results across the turnstile

$$terminal \ absurd \quad \frac{\Gamma \vdash f:A + ¬A}{\Gamma \vdash \bot}$$

$$terminal \ infer \ argument \quad \frac{\Gamma \vdash f:A \to B \quad \Gamma \vdash f(x):B}{\Gamma \vdash x:A}$$

$$terminal \ abstraction \quad \frac{\Gamma \vdash a:A \quad \Gamma \vdash b:B \quad \Gamma \vdash x:X \quad \Gamma \vdash y:Y \quad λ⟨a.b⟩⟨x.y⟩}{\Gamma \vdash λ⟨a.b⟩⟨x.y⟩:(A \to B) + (X \to Y)}$$

$$terminal \ application \quad \frac{\Gamma \vdash f:(A \to B) + (C \to D) + (X \to Y) \quad \Gamma \vdash x:A + X \quad f(x)}{\Gamma \vdash f(x):B + Y}$$

## Evaluation

Rules for evaluation are mostly the same as lambda calculus with the exception of plural arrows that may *carry* multiple values at a time. This feature leads to the possibility of plural values which may diverge in new ways.

Example split (singular value yields plural):
```punc
λ⟨a:Int.True⟩⟨x:Int.x⟩ 3
---------------------------------
⟨True⟩⟨3⟩
```

Example merge (plural value yields singular):
```punc
λ⟨a:Bool.2⟩⟨x:Int.2⟩ (⟨False⟩⟨5⟩)
---------------------------------
⟨2⟩
```

Example carry (plural value yields plural):
```punc
λ⟨a:Bool.not a⟩⟨x:Int.- x 2⟩ (⟨False⟩⟨5⟩)
---------------------------------
⟨True⟩⟨3⟩
```

## Optional Constraints

It may often be desirable to entirely prevent plural values.
This would require the type system to show that no splits will happen, which are always the root cause of plural values.
Notice that plural types always have plural values.

$$ban \ plurals \quad \frac{\Gamma \vdash f:(A \to B)+(A \to C) \quad \Gamma \vdash x:A \quad \Gamma \vdash f(x)}{\Gamma \vdash \bot}$$

## Notes

For simplicity, all bindings are modelled as morphisms.
An "object" A can be modelled as a simple morphism 1 → A.
Similarly ¬A is shorthand for 0 → A.

"Plural Types" are similar to product types plus the implicit subtyping relations that `A + B ⇒ A` and `A + B ⇒ B`.

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

* Subtyping
* [Strongly Normalizing subsets of Lambda Calculus](https://cstheory.stackexchange.com/questions/20364/how-to-make-the-lambda-calculus-strong-normalizing-without-a-type-system)
