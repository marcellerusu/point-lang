# Pnt Lang

Message-oriented programming

# A Real Working Example

```
class Point
  def + Point{x; y;} ->
    Point{x: self :x. + x; y: self :y. + y;};
end

Point{x: 10, y: 10} + Point{x: 10, y: 11};
```

# Syntax

## Literals

`;` ends an expression

```
-- this is a comment

-- this is a keyword literal
:hello;

-- integer literal
12;

-- instance literal
Point{x: 1; y: 2;};

-- bool literal
true;

-- operators are data literals just like true & false
+;
```

## Assignment

```
one := 1;
```

## Method Calls

`.` ends a method call
`;` ends an expression, it can also be used to end a method call

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

  -- `def` defines a message handler, followed by argument patterns.
  def + Point{x, y} ->
    Point{x: self :x. + x, y: self :y. + y};

end

Point{x: 1; y: 1;} + Point{x: -1; y: 1;}.
  :log; -- prints "Point{ x: 0; y: 2; }"
```

# FizzBuzz

Let's start with a sane simple question, fizz buzz. If a number is divisible by 3 print fizz, 5 print buzz, 15 print fizzbuzz. How would we go about this in pnt?

```
1..=100. :map
  Classify{fizz: _ % 3; buzz: _ % 5;}
  object
    def {fizz: 0; buzz: 0;} -> "fizzbuzz";
    def {fizz: 0;} -> "fizz";
    def {buzz: 0;} -> "buzz";
  end;
```

# FAQ

Q: I tried to run that fizzbuzz example, it didn't compile..

A: Yes, I lied. One day it might work that way, but right now it's fairly limited.

Q: How do you pronounce pnt?

A: "point"
