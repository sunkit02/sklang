# SkLang (subject to change)

Toy language project to learn about interpreters and compilers.

## Features

Those marked with \* are essential features and those marked with ^ are eventual goals.

- [ ] Math arithmetic\*
- [ ] Variable declaration\*
- [ ] Control flow\*
  - [ ] If statements\*
  - [ ] Switch statements
  - [ ] Match statements^
- [ ] Loops\*
  - [ ] For
  - [ ] While\*
- [ ] Functions\*
  - [ ] Regular free hanging functions
  - [ ] Closures^
- [ ] Structs and methods (including the dot "." operator)
  - [ ] Structs that hold data\*
  - [ ] Methods
  - [ ] Enums and algebraic types^
- [ ] Traits/interfaces^
- [ ] Static types\*
  - [ ] Primitives
    - [ ] int
    - [ ] uint
    - [ ] float
    - [ ] char
    - [ ] boolean
  - [ ] Built-in
    - [ ] arrays (including the index "[]" operator)
    - [ ] strings
- [ ] Line terminators
  - [ ] Semicolons (initial stage)
  - [ ] Newline (final goal)

## Grammar

Comments

```
// Comment
```

Math arithmetic

```
1 + ((2 * 3) / 4) % 5
```

Variable declaration (type declaration is optional)

```
var x = 1;
var y: int = 2;
```

If statements

```
if (1 < 10) {
    ...
} else if (1 > 10) {
    ...
} else {
    ...
}
```

Switch statements

```
switch (x) {
    1 -> {},
    'a' -> {},
    y   -> {},
    default -> {},
}
```

Loops

```
for (var i = 0; i < 10; i += 1) {
    ...
}

while (1 < 0) {
    ...
}

```

Function declaration and evocation

```
fn function(param1 type1, param type2, ...) -> return_type {
    ...
    return return_type;
}

function(param1, param2)
```

Closures

```
(param1: type1, param2: type2) -> return_type? { ... }
```

Struct and method declaration (fields must come before methods)

```
struct Type {
    field: type,
    field: type,

    fn static_method(param1 type1, param2 type2) -> return_type { ... }

    fn method(self, param1 type1, param2 type2) -> return_type { ... }
}

var object = Type { field1: value1, field2: value2 };
var result = object.method(argument1, argument2);

enum Enum {
    Variant1,
    Variant2,
}
```
