Sometimes it is useful to wrap up a function and free variables for better clarity and reuse. The free variables that can be used come from the enclosing scope and are ‘closed over’ when used in the function. From this, we get the name ‘closures’ and Rust provides a really great implementation of them, as we’ll see.

Enclosing (or nonlocal) scope is a special scope that only exists for nested functions. If the local scope is an inner or nested function, then the enclosing scope is the scope of the outer or enclosing function. This scope contains the names that you define in the enclosing function.
The environment for a closure can include bindings from its enclosing scope in addition to parameters and local bindings. It looks like this:

