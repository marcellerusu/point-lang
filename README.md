# Point Lang

Message-oriented programming

# A Real Working Example

```
class Point
  def + Point{x; y;} ->
    Point{x: self :x. + x; y: self :y. + y;};
end

Point{x: 10; y: 10;} + Point{x: 10; y: 11;};
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

-- lists
[1; 2;];
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

Point{x: 1; y: 1;}; -- instance
```

```
class Point

  -- `def` defines a message handler, followed by argument patterns.
  def + Point{x; y;} ->
    Point{x: self :x. + x; y: self :y. + y;};

end

Point{x: 1; y: 1;} + Point{x: 2; y: 1;}.
  :log; -- prints "Point{ x: 3; y: 2; }"
```

## Map/Filter & Object Literals

We support map & filter, but we don't have traditional lambda's, the only notion that can respond to messages is an object so we can create an object literal.

```
[1; 2; 3;] :map
  object
    def x -> x + 1;
  end; -- [2; 3; 4;]

-- let's define a more interesting mapping

[1; 2; 3;] :map
  object
    def 1 -> 2;
    def 2 -> 0;
    def 3 -> 10;
  end; -- [2; 0; 10;]

-- similarly we have filter

[1; 2; 3;] :filter
  object
    def 2 -> true;
    def _ -> false;
  end; -- [2;]
```

## Booleans

Booleans are not a language construct, they are defined in the language.

Let's see the definition:

```
class TrueClass
  def && ^false -> false;
  def && ^true -> true;
  def || _ -> true;
end
class FalseClass
  def && _ -> false;
  def || ^true -> true;
  def || ^false -> false;
end

true := TrueClass{};
false := FalseClass{};

-- let's use them

true && false; -- false
true || false; -- true
false || true; -- true
false && true; -- false
```
