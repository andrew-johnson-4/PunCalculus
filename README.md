# Pun Calculus
A variant of Typed Lambda Calculus with generalized variable punning (ad-hoc polymorphism).

## EBNF

```PLC
rhs        := [a-z][_a-zA-Z0-9]*   //Variable
            | ( rhs* )             //Function Application
            | λ rhs* . rhs*        //Lambda Function
            | [^ ]+                //Literal Value

binding    := [a-zA-Z0-9]+ [:] [=] rhs \n

program    := binding*
```

## Types

Typechecking is a graph-coloring problem to ensure that Cannot Determine Color does not get applied to the result of Greedy Infer.

$$greedy \ infer \quad \frac{f:A \to B \quad f:B \to C}{f:A \to C}$$

$$cannot \ determine \ color \quad \frac{f:A \to B \quad f:B \to C \quad f:A \to C}{\bot}$$

## Notes

For simplicity, all bindings are modelled as morphisms.
An "object" A can be modelled as a simple morphism 0 -> A.
