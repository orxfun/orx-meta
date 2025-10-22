# Summary

I have been working on the [orx-local-search](https://github.com/orxfun/orx-local-search/) library. As operations research (OR) practitioners, we are moving towards solutions and algorithms that are flexible in handling various combinations of real-life constraints. Due to the huge number of possible combinations, the only way to achieve this is to conveniently define the composition. However, as performance is always important in optimization algorithms, we cannot sacrifice performance while doing so.

This led to the observations and approach discussed in a talk about [composing zero cost abstractions in route optimization](https://orxfun.github.io/talk-composing-zero-cost-abstractions-in-route-optimization/) and the [zero-cost composition article](https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15).

Generalized idea is published in the `queue` module of the [orx-meta](https://crates.io/crates/orx-meta) crate.

To summarize its use cases.

Having a queue of statically-typed heterogeneous elements has some advantages:

* Boxing is not necessary.
* No dynamic dispatch, no runtime branching and no recursion.
* We can use it as an incremental ad-hoc struct. This gives us a generic builder that we can use for any struct.

On the other hand, being as statically-typed queue, it has the important limitation that types of all of its elements must be known in compile-time.

[<|](https://github.com/orxfun/orx-meta/blob/main/docs/5_solution_with_macros.md)======
