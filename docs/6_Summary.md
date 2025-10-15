# Summary

Having a queue of statically-typed heterogeneous elements has certain advantages:

* Boxing is not necessary.
* No dynamic dispatch involved. Although the `draw` calls in the example looks and feels recursive, there is also no recursion involved. The `draw` call to a queue can be completely inlined.
