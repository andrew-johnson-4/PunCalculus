# [Pun Calculus](https://github.com/andrew-johnson-4/PunCalculus/wiki)
A variant of Typed Lambda Calculus with generalized variable punning (ad-hoc polymorphism).

## Contribution

Ad-Hoc Polymorphism is introduced to the Simply Typed Lambda Calculus by pluralizing lambda abstractions.
Terms such as `λx:X. y` are represented instead as `λ⟨x:X. y⟩`.
Plural abstractions are represented with more braces: `λ⟨a:A. b⟩⟨x:X. y⟩`.
The type system is also extended slightly to support plural types: `A + B`.

## Motivation

In the LSTS proof assistant it became apparent that plural abstractions are valuable.
This was observed in simple function applications `f(x)` where the function candidates could prove multiple properties depending on properties of the input.
This was the immediate origin of the concept of "plural" arrows that can carry multiple properties.
PunC is an attempt to generalize this idea before upgrading the LSTS framework.

## Types

$$abstraction \quad \frac{\Gamma \vdash a:A \quad \Gamma \vdash b:B \quad \Gamma \vdash x:X \quad \Gamma \vdash y:Y \quad λ⟨a.b⟩⟨x.y⟩}{\Gamma \vdash λ⟨a.b⟩⟨x.y⟩:(A \to B) + (X \to Y)}$$

$$application \quad \frac{\Gamma \vdash f:(A \to B) + (C \to D) + (X \to Y) \quad \Gamma \vdash x:A + X \quad f(x)}{\Gamma \vdash f(x):B + Y}$$

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

Banning plurals still permits ad-hoc polymorphism in the *either-or* cases.

## Notes

"Plural Types" are similar to product types plus the implicit subtyping relations that `A + B ⇒ A` and `A + B ⇒ B`.
Types are either singular or plural, never both.
If you want to turn `A + B` into a singular type, then you could write it as its corresponding product: `(A,B)`.

## Direct Citations

[A very brisk introduction to Type Theory](https://ncatlab.org/nlab/show/type+theory)

## Similar Work

[The Kernel of Ad Hoc Polymorphism](https://dspace.mit.edu/bitstream/handle/1721.1/106072/965197677-MIT.pdf)

## Possible Extensions

* Explore interaction with other rich type systems such as Haskell or ML
  * Since those languages have a monomorphic Term System, their interactions with PunC inference should still be sound, though inference may become correspondingly weaker
* string rewriting at the type level: `a -> T<a>` apply `B` tries a [Relog](https://github.com/andrew-johnson-4/InPlace) unification `a=B;T<a>`
* $$terminal \ absurd \quad \frac{\Gamma \vdash f:A + ¬A}{\Gamma \vdash \bot}$$
* $$intermediate \ greedy \ infer \quad \frac{f:A \to B \quad f:B \to C}{f:A \to C}$$
* $$intermediate \ always \ follows \quad \frac{f: A \to B \quad f:¬ A \to B}{f(x): B}$$
* Subtyping
* Dependent Types
