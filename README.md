# Pnt Lang

Message-oriented programming

# A Real Working Example

```
class Point
  def + Point{x, y} ->
    Point{x: self :x. + x, y: self :y. + y}.
.

Point{x: 10, y: 10} + Point{x: 10, y: 11}.
```

# Syntax

## Literals

`;` ends an expression

```
-- comment
:hello; -- keyword
12; -- int
{a: 10}; -- object literal
Point{x: 1, y: 2}; -- instance literal
true; -- bool literal
+; -- operators are data literals just like true & false
```

## Assignment

```
one := 1;
```

## Method Calls

`.` ends a method call
`;` ends a method call, and doesn't allow chaining to continue

```
-- "pass :log to 10"
10 :log;
-- "pass the arguments + and 3 to 10"
10 + 3;

-- "pass the arguments + and 3 to 10, then pass :log to the result of that"
10 + 3.
  :log;
```

## Classes

```
class Point; -- definition

Point{x: 1, y: 1}; -- instance
```

```
class Point
  -- `def` defines a method, followed by argument patterns.
  def + Point{x, y} ->
    Point{x: self :x. + x, y: self :y. + y};
end
-- `end` or `;` can finish a class definition

Point{x: 1, y: 1} + Point{x: -1, y: 1}.
  :log; -- prints "Point{x: 0, y: 2}"
```

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

Q: How do you pronounce pnt?

A: "point"
