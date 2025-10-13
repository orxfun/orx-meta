/// A strongly typed queue of arbitrary elements.
///
/// There exist three queue implementations:
/// * `Empty`: empty queue
/// * `Single<F>`: a queue with a single element of type `F`
/// * `Multi<F, B>`: a queue with multiple (>1) elements where the front element is of type `F`
///   and the remaining elements is a queue of type `B`.
pub trait Queue {
    /// Type of the queue obtained by pushing an element of type `Elem` to the back of the queue.
    type PushBack<Elem>: NonEmptyQueue;

    /// Type of the element at the front of the queue.
    type Front;

    /// Type of the queue that would be obtained by popping the `Front` element of the queue.
    type Back: Queue;

    type Raised: Queue;

    /// Pushes the element `x` to the back of the queue.
    ///
    /// Resulting queue implements [`NonEmptyQueue`].
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new();
    ///
    /// let queue = queue.push(42);
    /// assert_eq!(queue.len(), 1);
    ///
    /// let queue = queue.push(true).push('x');
    /// assert_eq!(queue.len(), 3);
    ///
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
    /// ```
    fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>;

    /// Number of elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new();
    /// assert_eq!(queue.len(), 0);
    ///
    /// let queue = queue.push(42);
    /// assert_eq!(queue.len(), 1);
    ///
    /// let queue = queue.push(true).push('x');
    /// assert_eq!(queue.len(), 3);
    ///
    /// let (num, queue) = queue.pop();
    /// assert_eq!(num, 42);
    /// assert_eq!(queue.len(), 2);
    ///
    /// let (flag, queue) = queue.pop();
    /// assert_eq!(flag, true);
    /// assert_eq!(queue.len(), 1);
    ///
    /// let (char, queue) = queue.pop();
    /// assert_eq!(char, 'x');
    /// assert_eq!(queue.len(), 0);
    /// ```
    fn len(&self) -> usize;

    /// Returns true if the queue is empty; equivalent to `queue.len() == 0`.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn raise(self) -> Self::Raised;

    fn from_raised(raised: Self::Raised) -> Self;
}

/// A strongly typed [`Queue`] that is guaranteed to contain at least one element.
///
/// Among the three queue implementations, [`Single`] and [`Multi`] implements non-empty queue,
/// while [`Empty`] does not.
pub trait NonEmptyQueue: Queue {
    /// Consumes the queue and returns the front element.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_front(), 42);
    ///
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_front(), 42);
    /// ```
    fn into_front(self) -> Self::Front;

    /// Consumes the queue and returns the queue containing elements except for the front element.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// // front: 42; back: []
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_back(), Empty::new());
    ///
    /// // front: 42; back: [true]
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.into_back(), Single::new(true));
    ///
    /// // front: 42; back: [true, 'x']
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_back(), Single::new(true).push('x'));
    /// ```
    fn into_back(self) -> Self::Back;

    /// Consumes the queue and splits it into two pieces:
    /// * the element at the front of the queue, and
    /// * the queue containing elements except for the front element.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// // front: 42; back: []
    /// let queue = Empty::new().push(42);
    /// let (front, back) = queue.pop();
    /// assert_eq!(front, 42);
    /// assert_eq!(back, Empty::new());
    ///
    /// // front: 42; back: [true]
    /// let queue = Empty::new().push(42).push(true);
    /// let (front, back) = queue.pop();
    /// assert_eq!(front, 42);
    /// assert_eq!(back, Single::new(true));
    ///
    /// // front: 42; back: [true, 'x']
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// let (front, back) = queue.pop();
    /// assert_eq!(front, 42);
    /// assert_eq!(back, Single::new(true).push('x'));
    /// ```
    fn pop(self) -> (Self::Front, Self::Back);

    /// Returns a reference to the element at the front of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// // front: 42; back: []
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.front(), &42);
    ///
    /// // front: 42; back: [true]
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.front(), &42);
    ///
    /// // front: 42; back: [true, 'x']
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.front(), &42);
    /// ```
    fn front(&self) -> &Self::Front;

    /// Returns a reference to the queue containing elements of this queue except for the
    /// element at the front.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// // front: 42; back: []
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.back(), &Empty::new());
    ///
    /// // front: 42; back: [true]
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.back(), &Single::new(true));
    ///
    /// // front: 42; back: [true, 'x']
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.back(), &Single::new(true).push('x'));
    /// ```
    fn back(&self) -> &Self::Back;

    /// Returns a tuple of references to the front and back of the queue:
    /// * front: element at the front of the queue,
    /// * back: queue containing all elements except for the front element.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// // front: 42; back: []
    /// let queue = Empty::new().push(42);
    /// let (front, back) = queue.front_back();
    /// assert_eq!(front, &42);
    /// assert_eq!(back, &Empty::new());
    ///
    /// // front: 42; back: [true]
    /// let queue = Empty::new().push(42).push(true);
    /// let (front, back) = queue.front_back();
    /// assert_eq!(front, &42);
    /// assert_eq!(back, &Single::new(true));
    ///
    /// // front: 42; back: [true, 'x']
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// let (front, back) = queue.front_back();
    /// assert_eq!(front, &42);
    /// assert_eq!(back, &Single::new(true).push('x'));
    /// ```
    fn front_back(&self) -> (&Self::Front, &Self::Back);

    /// Returns mutable a reference to the element at the front of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// // front: 42; back: []
    /// let mut queue = Empty::new().push(42);
    /// *queue.front_mut() += 1;
    /// assert_eq!(queue.front(), &43);
    ///
    /// // front: 42; back: [true]
    /// let mut queue = Empty::new().push(42).push(true);
    /// *queue.front_mut() += 1;
    /// assert_eq!(queue.front(), &43);
    ///
    /// // front: 42; back: [true, 'x']
    /// let mut queue = Empty::new().push(42).push(true).push('x');
    /// *queue.front_mut() += 1;
    /// assert_eq!(queue.front(), &43);
    /// ```
    fn front_mut(&mut self) -> &mut Self::Front;

    /// Returns a mutable reference to the queue containing elements of this queue except for the
    /// element at the front.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// // front: 42; back: [true]
    /// let mut queue = Empty::new().push(42).push(true);
    /// *queue.back_mut().front_mut() = false;
    /// assert_eq!(queue.as_tuple(), (&42, &false));
    ///
    /// // front: 42; back: [true, 'x']
    /// let mut queue = Empty::new().push(42).push(true).push('x');
    /// *queue.back_mut().front_mut() = false;
    /// *queue.back_mut().back_mut().front_mut() = 'y';
    /// assert_eq!(queue.as_tuple(), (&42, &false, &'y'));
    /// ```
    fn back_mut(&mut self) -> &mut Self::Back;

    /// Returns a tuple of mutable references to the front and back of the queue:
    /// * front: element at the front of the queue,
    /// * back: queue containing all elements except for the front element.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// // front: 42; back: [true, 'x']
    /// let mut queue = Empty::new().push(42).push(true).push('x');
    /// let (front, back) = queue.front_back_mut();
    /// *front += 1;
    ///
    /// // recursively destruct the back
    /// let (front, back) = back.front_back_mut();
    /// *front = false;
    ///
    /// // recursively destruct the back
    /// let (front, _back_empty) = back.front_back_mut();
    /// *front = 'y';
    ///
    /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
    /// ```
    fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back);
}

/// An empty queue.
///
/// Implements [`Queue`] but not [`NonEmptyQueue`].
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// let queue = Empty::new();
/// assert!(queue.is_empty());
///
/// let queue = Single::new(42);
/// let (num, queue) = queue.pop();
/// assert_eq!(num, 42);
/// assert!(queue.is_empty());
///
/// let queue = Single::new(42).push(true);
/// let (num, queue) = queue.pop();
/// let (flag, queue) = queue.pop();
/// assert!(queue.is_empty());
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Empty {
    phantom: core::marker::PhantomData<()>,
}

impl Empty {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}
impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

impl Queue for Empty {
    type PushBack<Elem> = Single<Elem>;
    type Front = Empty;
    type Back = Self;
    type Raised = Self;
    fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem> {
        Single::new(x)
    }
    fn raise(self) -> Self::Raised {
        Default::default()
    }
    fn from_raised(raised: Self::Raised) -> Self {
        raised
    }
    fn len(&self) -> usize {
        0
    }
}

/// A queue with a single element of type `Front`.
///
/// Implements both [`Queue`] and [`NonEmptyQueue`].
///
/// It can be created using [`Single::new`] or calling [`push`] on an [`Empty`] queue.
///
/// [`push`]: Queue::push
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// let queue: Single<u32> = Empty::new().push(42);
/// let queue: Single<u32> = Single::new(42);
///
/// let queue = Empty::new().push(42).push(true).push('x');
///
/// let (num, queue_pair) = queue.pop();
/// assert_eq!(num, 42);
///
/// let (flag, queue_single) = queue_pair.pop();
/// assert_eq!(flag, true);
///
/// assert_eq!(queue_single, Single::new('x'));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Single<Front> {
    phantom: core::marker::PhantomData<()>,
    empty: Empty,
    f: Front,
}

impl<F> Single<F> {
    /// Creates a with a single element `f` of type `F`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Single::new(42);
    ///
    /// // equivalent to
    /// let queue2 = Empty::new().push(42);
    ///
    /// assert_eq!(queue, queue2);
    /// ```
    pub fn new(f: F) -> Self {
        Self {
            phantom: Default::default(),
            empty: Empty::new(),
            f,
        }
    }
}

impl<F> Queue for Single<F> {
    type PushBack<Elem> = Multi<F, Single<Elem>>;
    type Front = F;
    type Back = Empty;
    type Raised = Single<Self>;
    fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem> {
        Multi::new(self.f, Single::new(x))
    }
    fn raise(self) -> Self::Raised {
        Single::new(self)
    }
    fn from_raised(raised: Self::Raised) -> Self {
        raised.f
    }
    fn len(&self) -> usize {
        1
    }
}
impl<F> NonEmptyQueue for Single<F> {
    fn into_front(self) -> Self::Front {
        self.f
    }
    fn into_back(self) -> Self::Back {
        self.empty
    }
    fn pop(self) -> (Self::Front, Self::Back) {
        (
            self.f,
            Empty {
                phantom: Default::default(),
            },
        )
    }
    fn front(&self) -> &Self::Front {
        &self.f
    }
    fn back(&self) -> &Self::Back {
        &self.empty
    }
    fn front_back(&self) -> (&Self::Front, &Self::Back) {
        (&self.f, &self.empty)
    }
    fn front_mut(&mut self) -> &mut Self::Front {
        &mut self.f
    }
    fn back_mut(&mut self) -> &mut Self::Back {
        &mut self.empty
    }
    fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back) {
        (&mut self.f, &mut self.empty)
    }
}

/// A queue with multiple, at least two, elements such that:
/// * type of the element at the front of the queue is `Front`, and
/// * remaining elements form a queue of type `Back`.
///
/// Implements both [`Queue`] and [`NonEmptyQueue`].
///
/// It can be created using [`Multi::new`] or calling [`push`] multiple times on an [`Empty`] queue.
///
/// Note that the back of the multiple elements queue can be any of the other queues including `Multi`.
/// This gives the recursive composition ability of the queues.
///
/// [`push`]: Queue::push
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// let pair = Multi::new(42, Single::new(true));
/// assert_eq!(pair.as_tuple(), (&42, &true));
///
/// let triple = Multi::new(42, Multi::new(true, Single::new('x')));
/// assert_eq!(triple.as_tuple(), (&42, &true, &'x'));
///
/// let quad = Multi::new(42, Multi::new(true, Multi::new('x', Single::new("foo"))));
/// assert_eq!(quad.as_tuple(), (&42, &true, &'x', &"foo"));
///
/// // more convenient to build using push though
///
/// let quad = Empty::new().push(42).push(true).push('x').push("foo");
/// assert_eq!(quad.as_tuple(), (&42, &true, &'x', &"foo"));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Multi<Front, Back>
where
    Back: Queue,
{
    phantom: core::marker::PhantomData<()>,
    f: Front,
    b: Back,
}

impl<F, B> Multi<F, B>
where
    B: Queue,
{
    /// Creates a new queue with multiple elements such that:
    /// * the front of the queue is the element `f`, and
    /// * the remaining elements of the queue form the queue `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let pair = Multi::new(42, Single::new(true));
    /// assert_eq!(pair.front(), &42);
    /// assert_eq!(pair.back(), &Single::new(true));
    ///
    /// let triple = Multi::new(42, Multi::new(true, Single::new('x')));
    /// assert_eq!(triple.front(), &42);
    /// assert_eq!(triple.back(), &Multi::new(true, Single::new('x')));
    ///
    /// let quad = Multi::new(42, Multi::new(true, Multi::new('x', Single::new("foo"))));
    /// assert_eq!(quad.front(), &42);
    /// assert_eq!(quad.back(), &Multi::new(true, Multi::new('x', Single::new("foo"))));
    ///
    /// // more convenient to build using push though
    ///
    /// let quad = Empty::new().push(42).push(true).push('x').push("foo");
    /// assert_eq!(quad.front(), &42);
    /// assert_eq!(quad.back(), &Empty::new().push(true).push('x').push("foo"));
    /// ```
    pub fn new(f: F, b: B) -> Self {
        Self {
            phantom: Default::default(),
            f,
            b,
        }
    }
}

impl<F, B> Queue for Multi<F, B>
where
    B: Queue,
{
    type PushBack<Elem> = Multi<F, B::PushBack<Elem>>;
    type Front = F;
    type Back = B;
    type Raised = Multi<Single<F>, B::Raised>;
    fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem> {
        Multi::new(self.f, self.b.push(x))
    }
    fn raise(self) -> Self::Raised {
        Multi::new(Single::new(self.f), self.b.raise())
    }
    fn from_raised(raised: Self::Raised) -> Self {
        let f = raised.f.f;
        let b = B::from_raised(raised.b);
        Multi::new(f, b)
    }
    fn len(&self) -> usize {
        1 + self.b.len()
    }
}
impl<F, B> NonEmptyQueue for Multi<F, B>
where
    B: Queue,
{
    fn into_front(self) -> Self::Front {
        self.f
    }
    fn into_back(self) -> Self::Back {
        self.b
    }
    fn pop(self) -> (Self::Front, Self::Back) {
        (self.f, self.b)
    }
    fn front(&self) -> &Self::Front {
        &self.f
    }
    fn back(&self) -> &Self::Back {
        &self.b
    }
    fn front_back(&self) -> (&Self::Front, &Self::Back) {
        (&self.f, &self.b)
    }
    fn front_mut(&mut self) -> &mut Self::Front {
        &mut self.f
    }
    fn back_mut(&mut self) -> &mut Self::Back {
        &mut self.b
    }
    fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back) {
        (&mut self.f, &mut self.b)
    }
}

// builder

/// A type-safe builder for queues such that:
///
/// * `push` can only be called correct number of times with the correct types,
/// * `finish` can only be called after all elements are pushed.
///
/// It can also be used as a generic builder for any tuple type.
///
/// # Example
///
/// In the following example, we want to build a queue of four elements of types `u32`, `bool`, `char` and `&str` respectively.
///
/// For this, we can create a builder with `QueueBuilder::<MyQueue>::new()` where `MyQueue` is the target type to instantiate.
///
/// ```
/// use orx_meta::queue::*;
///
/// type MyQueue = Multi<u32, Multi<bool, Multi<char, Single<&'static str>>>>;
///
/// let instance = QueueBuilder::<MyQueue>::new()
///     .push(42)
///     .push(true)
///     .push('x')
///     .push("foo")
///     .finish();
/// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
/// ```
///
/// This provides a convenient way to build complex types without errors and with compiler support.
/// However, it is not easy to hand-write the type alias for the complex recursive queue type.
/// Therefore, this builder pattern is most useful when used together with the [`queue_of`] macro.
/// The above example could be re-written as follows with the `queue_of` macro.
///
/// ```
/// use orx_meta::queue::*;
/// use orx_meta::queue_of;
///
/// type MyQueue = queue_of!(u32, bool, char, &'static str);
///
/// let instance = QueueBuilder::<MyQueue>::new()
///     .push(42)
///     .push(true)
///     .push('x')
///     .push("foo")
///     .finish();
/// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
/// ```
///
/// ## Examples - Type Safety
///
/// Note that this builder pattern is type safe in the sense that neither of the following wrong implementations compiles.
///
/// Here the elements are pushed in the wrong order:
///
/// ```compile_fail
/// use orx_meta::queue::*;
/// use orx_meta::queue_of;
///
/// type MyQueue = queue_of!(u32, bool, char, &'static str);
///
/// let instance = QueueBuilder::<MyQueue>::new()
///     .push(true) // wrong order!
///     .push(42)
///     .push('x')
///     .push("foo")
///     .finish();
/// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
/// ```
///
/// And here, not all elements are pushed:
///
/// ```compile_fail
/// use orx_meta::queue::*;
/// use orx_meta::queue_of;
///
/// type MyQueue = queue_of!(u32, bool, char, &'static str);
///
/// let instance = QueueBuilder::<MyQueue>::new()
///     .push(42)
///     .push(true)
///     .push('x')
///     .finish(); // forgot to push &str
/// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
/// ```
pub struct QueueBuilder<Remaining, Current = Empty>
where
    Remaining: Queue,
    Current: Queue,
{
    cur: Current,
    rem: core::marker::PhantomData<Remaining>,
    phantom: core::marker::PhantomData<()>,
}

impl<Remaining> QueueBuilder<Remaining, Empty>
where
    Remaining: Queue,
{
    /// Creates a new builder for a target queue.
    ///
    /// The type parameter used when constructing the builder defines the target
    /// type to be constructed.
    ///
    /// The builder makes sure that `finish` can be called only after all elements
    /// required by the target type are pushed.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    /// use orx_meta::queue_of;
    ///
    /// let builder = QueueBuilder::<Empty>::new();
    /// let instance = builder.finish();
    /// assert!(instance.is_empty());
    ///
    /// let builder = QueueBuilder::<Single<u32>>::new(); // or
    /// let builder = QueueBuilder::<queue_of!(u32)>::new();
    /// let instance = builder.push(42).finish();
    /// assert_eq!(instance.as_tuple(), (&42));
    ///
    /// let builder = QueueBuilder::<Multi<u32, Single<bool>>>::new(); // or
    /// let builder = QueueBuilder::<queue_of!(u32, bool)>::new();
    /// let instance = builder.push(42).push(true).finish();
    /// assert_eq!(instance.as_tuple(), (&42, &true));
    ///
    /// let builder = QueueBuilder::<Multi<u32, Multi<bool, Multi<char, Single<&'static str>>>>>::new(); // or
    /// let builder = QueueBuilder::<queue_of!(u32, bool, char, &'static str)>::new();
    /// let instance = builder.push(42).push(true).push('x').push("foo").finish();
    /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn new() -> Self {
        Self {
            cur: Empty::new(),
            rem: Default::default(),
            phantom: Default::default(),
        }
    }
}
impl<Remaining> Default for QueueBuilder<Remaining, Empty>
where
    Remaining: Queue,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<Remaining, Current> QueueBuilder<Remaining, Current>
where
    Remaining: Queue,
    Current: Queue,
{
    /// Pushes the next element to the builder, returns the builder for the remaining elements
    /// to reach the target type.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    /// use orx_meta::queue_of;
    ///
    /// type MyQueue = queue_of!(u32, bool, char, &'static str);
    ///
    /// // remaining:[u32, bool, char, &'static str]
    /// // current: []
    /// let builder = QueueBuilder::<MyQueue>::new();
    ///
    /// // remaining:[bool, char, &'static str]
    /// // current: [u32]
    /// let builder = builder.push(42);
    ///
    /// // remaining:[char, &'static str]
    /// // current: [u32, bool]
    /// let builder = builder.push(true);
    ///
    /// // remaining:[&'static str]
    /// // current: [u32, bool, char]
    /// let builder = builder.push('x');
    ///
    /// // remaining:[] -> we can now call finish
    /// // current: [u32, bool, char, &'static str]
    /// let builder = builder.push("foo");
    ///
    /// let instance = builder.finish();
    /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
    ///
    /// // it is often more convenient to chain the push calls
    /// let instance = QueueBuilder::<MyQueue>::new()
    ///     .push(42)
    ///     .push(true)
    ///     .push('x')
    ///     .push("foo")
    ///     .finish();
    /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn push(
        self,
        x: Remaining::Front,
    ) -> QueueBuilder<Remaining::Back, Current::PushBack<Remaining::Front>> {
        QueueBuilder {
            cur: self.cur.push(x),
            rem: Default::default(),
            phantom: Default::default(),
        }
    }

    /// Consumes the builder and returns the built instance of the target type.
    ///
    /// Note that `finish` can only be called after pushing all required elements of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    /// use orx_meta::queue_of;
    ///
    /// type MyQueue = queue_of!(u32, bool, char, &'static str);
    ///
    /// // remaining:[u32, bool, char, &'static str]
    /// // current: []
    /// let builder = QueueBuilder::<MyQueue>::new();
    ///
    /// // remaining:[bool, char, &'static str]
    /// // current: [u32]
    /// let builder = builder.push(42);
    ///
    /// // remaining:[char, &'static str]
    /// // current: [u32, bool]
    /// let builder = builder.push(true);
    ///
    /// // remaining:[&'static str]
    /// // current: [u32, bool, char]
    /// let builder = builder.push('x');
    ///
    /// // remaining:[] -> we can now call finish
    /// // current: [u32, bool, char, &'static str]
    /// let builder = builder.push("foo");
    ///
    /// let instance = builder.finish();
    /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
    ///
    /// // it is often more convenient to chain the push calls
    /// let instance = QueueBuilder::<MyQueue>::new()
    ///     .push(42)
    ///     .push(true)
    ///     .push('x')
    ///     .push("foo")
    ///     .finish();
    /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn finish(self) -> Current
    where
        Remaining: Queue<Back = Remaining>,
    {
        self.cur
    }
}

// tuple transformations

impl<X1> Single<X1> {
    /// Consumes the queue and returns the corresponding tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_tuple(), (42));
    ///
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> X1 {
        self.f
    }
    pub fn as_tuple(&self) -> &X1 {
        &self.f
    }
    pub fn as_tuple_mut(&mut self) -> &mut X1 {
        &mut self.f
    }
}
impl<X1> From<X1> for Single<X1> {
    fn from(x: X1) -> Self {
        Single::new(x)
    }
}

impl<X1, X2> Multi<X1, Single<X2>> {
    /// Consumes the queue and returns the corresponding tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_tuple(), (42));
    ///
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2) {
        (self.f, self.b.f)
    }
    pub fn as_tuple(&self) -> (&X1, &X2) {
        (&self.f, &self.b.f)
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2) {
        (&mut self.f, &mut self.b.f)
    }
}
impl<X1, X2> From<(X1, X2)> for Multi<X1, Single<X2>> {
    fn from(x: (X1, X2)) -> Self {
        Multi::new(x.0, Single::new(x.1))
    }
}

impl<X1, X2, X3> Multi<X1, Multi<X2, Single<X3>>> {
    /// Consumes the queue and returns the corresponding tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_tuple(), (42));
    ///
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3) {
        (self.f, self.b.f, self.b.b.f)
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3) {
        (&self.f, &self.b.f, &self.b.b.f)
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3) {
        (&mut self.f, &mut self.b.f, &mut self.b.b.f)
    }
}
impl<X1, X2, X3> From<(X1, X2, X3)> for Multi<X1, Multi<X2, Single<X3>>> {
    fn from(x: (X1, X2, X3)) -> Self {
        Multi::new(x.0, Multi::new(x.1, Single::new(x.2)))
    }
}

impl<X1, X2, X3, X4> Multi<X1, Multi<X2, Multi<X3, Single<X4>>>> {
    /// Consumes the queue and returns the corresponding tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_tuple(), (42));
    ///
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4) {
        (self.f, self.b.f, self.b.b.f, self.b.b.b.f)
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4) {
        (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f)
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4) {
        (
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.f,
        )
    }
}
impl<X1, X2, X3, X4> From<(X1, X2, X3, X4)> for Multi<X1, Multi<X2, Multi<X3, Single<X4>>>> {
    fn from(x: (X1, X2, X3, X4)) -> Self {
        Multi::new(x.0, Multi::new(x.1, Multi::new(x.2, Single::new(x.3))))
    }
}

impl<X1, X2, X3, X4, X5> Multi<X1, Multi<X2, Multi<X3, Multi<X4, Single<X5>>>>> {
    /// Consumes the queue and returns the corresponding tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_tuple(), (42));
    ///
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
        (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f)
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5) {
        (
            &self.f,
            &self.b.f,
            &self.b.b.f,
            &self.b.b.b.f,
            &self.b.b.b.b.f,
        )
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5) {
        (
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.f,
            &mut self.b.b.b.b.f,
        )
    }
}
impl<X1, X2, X3, X4, X5> From<(X1, X2, X3, X4, X5)>
    for Multi<X1, Multi<X2, Multi<X3, Multi<X4, Single<X5>>>>>
{
    fn from(x: (X1, X2, X3, X4, X5)) -> Self {
        Multi::new(
            x.0,
            Multi::new(x.1, Multi::new(x.2, Multi::new(x.3, Single::new(x.4)))),
        )
    }
}

impl<X1, X2, X3, X4, X5, X6> Multi<X1, Multi<X2, Multi<X3, Multi<X4, Multi<X5, Single<X6>>>>>> {
    /// Consumes the queue and returns the corresponding tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_tuple(), (42));
    ///
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
        (
            self.f,
            self.b.f,
            self.b.b.f,
            self.b.b.b.f,
            self.b.b.b.b.f,
            self.b.b.b.b.b.f,
        )
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6) {
        (
            &self.f,
            &self.b.f,
            &self.b.b.f,
            &self.b.b.b.f,
            &self.b.b.b.b.f,
            &self.b.b.b.b.b.f,
        )
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6) {
        (
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.f,
            &mut self.b.b.b.b.f,
            &mut self.b.b.b.b.b.f,
        )
    }
}
impl<X1, X2, X3, X4, X5, X6> From<(X1, X2, X3, X4, X5, X6)>
    for Multi<X1, Multi<X2, Multi<X3, Multi<X4, Multi<X5, Single<X6>>>>>>
{
    fn from(x: (X1, X2, X3, X4, X5, X6)) -> Self {
        Multi::new(
            x.0,
            Multi::new(
                x.1,
                Multi::new(x.2, Multi::new(x.3, Multi::new(x.4, Single::new(x.5)))),
            ),
        )
    }
}

impl<X1, X2, X3, X4, X5, X6, X7>
    Multi<X1, Multi<X2, Multi<X3, Multi<X4, Multi<X5, Multi<X6, Single<X7>>>>>>>
{
    /// Consumes the queue and returns the corresponding tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_tuple(), (42));
    ///
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
        (
            self.f,
            self.b.f,
            self.b.b.f,
            self.b.b.b.f,
            self.b.b.b.b.f,
            self.b.b.b.b.b.f,
            self.b.b.b.b.b.b.f,
        )
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7) {
        (
            &self.f,
            &self.b.f,
            &self.b.b.f,
            &self.b.b.b.f,
            &self.b.b.b.b.f,
            &self.b.b.b.b.b.f,
            &self.b.b.b.b.b.b.f,
        )
    }
    pub fn as_tuple_mut(
        &mut self,
    ) -> (
        &mut X1,
        &mut X2,
        &mut X3,
        &mut X4,
        &mut X5,
        &mut X6,
        &mut X7,
    ) {
        (
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.f,
            &mut self.b.b.b.b.f,
            &mut self.b.b.b.b.b.f,
            &mut self.b.b.b.b.b.b.f,
        )
    }
}
impl<X1, X2, X3, X4, X5, X6, X7> From<(X1, X2, X3, X4, X5, X6, X7)>
    for Multi<X1, Multi<X2, Multi<X3, Multi<X4, Multi<X5, Multi<X6, Single<X7>>>>>>>
{
    fn from(x: (X1, X2, X3, X4, X5, X6, X7)) -> Self {
        Multi::new(
            x.0,
            Multi::new(
                x.1,
                Multi::new(
                    x.2,
                    Multi::new(x.3, Multi::new(x.4, Multi::new(x.5, Single::new(x.6)))),
                ),
            ),
        )
    }
}

impl<X1, X2, X3, X4, X5, X6, X7, X8>
    Multi<X1, Multi<X2, Multi<X3, Multi<X4, Multi<X5, Multi<X6, Multi<X7, Single<X8>>>>>>>>
{
    /// Consumes the queue and returns the corresponding tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Empty::new().push(42);
    /// assert_eq!(queue.into_tuple(), (42));
    ///
    /// let queue = Empty::new().push(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x');
    /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
    ///
    /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
        (
            self.f,
            self.b.f,
            self.b.b.f,
            self.b.b.b.f,
            self.b.b.b.b.f,
            self.b.b.b.b.b.f,
            self.b.b.b.b.b.b.f,
            self.b.b.b.b.b.b.b.f,
        )
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7, &X8) {
        (
            &self.f,
            &self.b.f,
            &self.b.b.f,
            &self.b.b.b.f,
            &self.b.b.b.b.f,
            &self.b.b.b.b.b.f,
            &self.b.b.b.b.b.b.f,
            &self.b.b.b.b.b.b.b.f,
        )
    }
    pub fn as_tuple_mut(
        &mut self,
    ) -> (
        &mut X1,
        &mut X2,
        &mut X3,
        &mut X4,
        &mut X5,
        &mut X6,
        &mut X7,
        &mut X8,
    ) {
        (
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.f,
            &mut self.b.b.b.b.f,
            &mut self.b.b.b.b.b.f,
            &mut self.b.b.b.b.b.b.f,
            &mut self.b.b.b.b.b.b.b.f,
        )
    }
}
impl<X1, X2, X3, X4, X5, X6, X7, X8> From<(X1, X2, X3, X4, X5, X6, X7, X8)>
    for Multi<X1, Multi<X2, Multi<X3, Multi<X4, Multi<X5, Multi<X6, Multi<X7, Single<X8>>>>>>>>
{
    fn from(x: (X1, X2, X3, X4, X5, X6, X7, X8)) -> Self {
        Multi::new(
            x.0,
            Multi::new(
                x.1,
                Multi::new(
                    x.2,
                    Multi::new(
                        x.3,
                        Multi::new(x.4, Multi::new(x.5, Multi::new(x.6, Single::new(x.7)))),
                    ),
                ),
            ),
        )
    }
}

// queue of

#[macro_export]
macro_rules! queue_of {
    () => {
        Empty
    };

    ($t1:ty) => {
        Single<$t1>
    };

    ($t1:ty, $t2:ty) => {
        Multi<$t1, Single<$t2>>
    };

    ($t1:ty, $t2:ty, $t3:ty) => {
        Multi<$t1, Multi<$t2, Single<$t3>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Single<$t4>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Single<$t5>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Single<$t6>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Single<$t7>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Multi<$t7, Single<$t8>>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Multi<$t7, Multi<$t8, Single<$t9>>>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Multi<$t7, Multi<$t8,
            Multi<$t9, Single<$t10>>
        >>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Multi<$t7, Multi<$t8,
            Multi<$t9, Multi<$t10, Single<$t11>>>
        >>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Multi<$t7, Multi<$t8,
            Multi<$t9, Multi<$t10, Multi<$t11, Single<$t12>>>>
        >>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Multi<$t7, Multi<$t8,
            Multi<$t9, Multi<$t10, Multi<$t11, Multi<$t12, Single<$t13>>>>>
        >>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Multi<$t7, Multi<$t8,
            Multi<$t9, Multi<$t10, Multi<$t11, Multi<$t12, Multi<$t13, Single<$t14>>>>>>
        >>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Multi<$t7, Multi<$t8,
            Multi<$t9, Multi<$t10, Multi<$t11, Multi<$t12, Multi<$t13, Multi<$t14, Single<$t15>>>>>>>
        >>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty) => {
        Multi<$t1, Multi<$t2, Multi<$t3, Multi<$t4, Multi<$t5, Multi<$t6, Multi<$t7, Multi<$t8,
            Multi<$t9, Multi<$t10, Multi<$t11, Multi<$t12, Multi<$t13, Multi<$t14, Multi<$t15, Single<$t16>>>>>>>>
        >>>>>>>>
    };
}
