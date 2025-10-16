# Summary

I have been working on the [orx-local-search](https://github.com/orxfun/orx-local-search/) library. As operations research (OR) practitioners, we are moving towards solutions and algorithms that are flexible in handling various combinations of real-life constraints. Due to the huge number of possible combinations, the only way to achieve this is to conveniently define the composition. However, as performance is always important in optimization algorithms, we cannot sacrifice performance while doing so.

This led to the observations and approach discussed in a talk about [composing zero cost abstractions in route optimization](https://orxfun.github.io/talk-composing-zero-cost-abstractions-in-route-optimization/).

And the generalization of the idea led to the `queue` module of the [orx-meta](https://crates.io/crates/orx-meta) crate.

To summarize its use cases.

Having a queue of statically-typed heterogeneous elements has some advantages:

* Boxing is not necessary.
* We can use it as an incremental ad-hoc struct. This gives us a generic builder that we can use for any struct.

These are available by the types defined in the queue module (`orx_meta::queue::*`).

On the other hand, having a queue of statically-typed heterogeneous elements having a shared behavior is particularly strong. We can use [`orx_meta::define_queue`](https://docs.rs/orx-meta/latest/orx_meta/macro.define_queue.html) macro to achieve this without boilerplate.

* We define the identity behavior (what is the common behavior in the absence of any element) and the composition (what is the common behavior when we have at least two elements).
* Then the queue allows us to exhibit a common behavior on a collection of heterogeneous elements.
* Using the queue has the convenience of dynamic counterparts; however, it is strongly typed.
* It has certain performance advantages,
  * Boxing is not necessary.
  * No virtual calls and no recursive calls.

On the other hand, type of the statically typed queue is of course much more complex than using another collection, such as a vec of trait objects.

[<|](https://github.com/orxfun/orx-meta/blob/main/docs/5_solution_with_macros.md)======
