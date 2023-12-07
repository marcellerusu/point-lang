# Pnt Lang

A perverse misunderstanding of object oriented programming

# FizzBuzz

Let's start with a sane simple question, fizz buzz. If a number is divisible by 3 print fizz, 5 print buzz, 15 print fizzbuzz. How would we go about this in pnt?

```
1..=100. :map
  Classify{fizz: _ % 3, buzz: _ % 5}
  object
    def {fizz: 0, buzz: 0} -> "fizzbuzz".
    def {fizz: 0} -> "fizz".
    def {buzz: 0} -> "buzz".
  .
```

# FAQ

Q: I tried to run that fizzbuzz example, it didn't compile..
A: Yes, I lied. One day it might work that way, but right now it's fairly limited.
