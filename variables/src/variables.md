1. variables are IMMUTABLE while can marked to be mutable by adding the keyword `mut`
2. Constants and Variables are both values bound to a name and are not allowed to change.

Differences between constants and variables:

1. `const` not allowed to use `mut`.
2. the type of `const` must be annotated.
3. `const` can be used in the **global scope**, and `let` can only be used in a function
4. `const` can only be set to a constant expression instead of the result of a value computed at runtime. (【Q】what is 'the result of a value computed at runtime')
5. Rust’s naming convention for `const` is to use all _UPPER_CASE_ with underscores between words

shadow variables

shadowing is different from marking a variable

1. the use of `let` in shadowing can not be neglected. Variables will still be immutable when out of scope.
2. shadowing can change the type of variables while `mut` can not.
