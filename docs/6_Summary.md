# Summary

I have been working on the [orx-local-search](https://crates.io/crates/orx-local-search) crate. As operations research (OR) practitioners, we are moving towards solutions and algorithms that are flexible in handling various combinations of real-life constraints. Due to the huge number of possible combinations, the only way to achieve this is to properly define the composition. However, we cannot sacrifice performance while doing so since performance is always important in optimization algorithms.

This led to the observations and approach discussed in a talk about [composing zero cost abstractions in route optimization](https://orxfun.github.io/talk-composing-zero-cost-abstractions-in-route-optimization/).

However, the ideas discussed here are not specific to route optimization. The generalization led to the `queue` module of the [orx-meta](https://crates.io/crates/orx-meta) crate.

To summarize its use cases.

Having a queue of statically-typed heterogeneous elements has some advantages:

* Boxing is not necessary.
* We can use it as a kind-of ad-hoc struct. This gives us a generic builder that we can use for any struct.

These are available by the types defined in the queue module (`orx_meta::queue::*`).

On the other hand, having a queue of statically-typed heterogeneous elements having a shared behavior is particularly strong. We can use [`orx_meta::define_queue`](https://docs.rs/orx-meta/latest/orx_meta/macro.define_queue.html) macro to achieve this without boilerplate.

* We define the identity behavior (what is the common behavior in the absence of any element) and the composition (what is the common behavior when we have at least two elements).
* Then the queue allows us to exhibit a common behavior on a collection of heterogeneous elements.
* Usage of the queue has a dynamic feel; however, it is strongly typed in its elements, as if we have hand-written a special struct.
* Boxing is not necessary.
* No virtual calls and no recursive calls.
* Call on the queue can be completely inlined.

Notice that most of the features are related to performance, making the queue most useful for performance-critical programs as it is used for the local search.
