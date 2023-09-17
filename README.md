# PunCalculus
A variant of Typed Lambda Calculus with generalized variable punning (ad-hoc polymorphism)

## EBNF

```PLC
rhs        := [a-z][_a-zA-Z0-9]*   //Variable
            | ( rhs* )             //Function Application
            | λ rhs* . rhs*        //Lambda Function
            | [^ ]+                //Literal Value

binding    := [a-zA-Z0-9]+ [:] [=] rhs \n

program    := binding*
```
