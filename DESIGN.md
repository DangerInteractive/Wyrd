# Design of Wyrd

## Problems to Solve

### Entity Component System

Game engines and similar systems need to store and manage a large collection of "entities" that
exist in a shared "domain", and apply a variety of transformations on these entities over time
depending on their characteristics.

All "entities" in a "domain" have the following things in common:
- They exist in the same space/time/reality (this is what we mean by "domain").
- They MAY have some "components" (chunks of data representing a quantifiable physical
  attribute) from within the set of components that are valid for the domain.
- They MUST have a live entity ID, which is an index that can be used to retrieve its components.

To transform entities in the domain, one must do so through "systems". Systems are defined as
the following:
- They take a set of components (from a single entity at a time) as arguments.
  - QUESTION: how would a collision-check system check for collisions between TWO entities if it
    can only operate on the components of a single entity?
- They return a set of "effects" (chunks of data much like a component that represent a
  change rather than a state)

Components define how they would be modified given a collection of effects yielded by systems:
- Question: does it make sense to split this behavior out from components as a separate function?
- They define an effect type, which describes a change in the component.
- They take in a collection/iterator of effects.
- They return a new component modified by the effects

### Static Typing

Because this system is aiming for maximum performance and the capability to process millions of
entities multiple times per second, making heavy use of polymorphism is simply not an option,
and any polymorphism that we do use will need to be very carefully considered.

Any time you access a type through a polymorphic interface rather than a known-at-compile-time
static type, you incur a pointer dereference to a vtable for potentially every method call on
that object. This vtable lookup will almost inevitably cause a new cache line to be loaded,
which is very slow and wastes a lot of time the CPU could be using the process systems.

### Cache Locality

When a system iterates through entities, it would be ideal for the components of that entity to
be stored in adjacent memory as much as possible in order to maximize the performance boost of
the CPU cache.

The CPU cache is different from CPU to CPU, but for many common cases of modern desktop/laptop
CPUs, whenever an instruction references a memory address that is not already in cache, it will
actually load 64 bytes (512 bits) of adjacent memory into the cache.

This memory will remain in the CPU cache for as long as the CPU deems it necessary to do so
(presumably pushing memory that hasn't been accessed in a while to bigger, slower caches, and
then eventually out of the cache).

If the memory is in the CPU cache, accessing that memory is much faster. In the L1 cache it'd be
nearly as fast as accessing a register.

On processors with Simultaneous Multithreading capability (sometimes called Hyperthreading by
Intel), while waiting for a memory fetch, the CPU can work on another logical thread in a
separate set of registers.

### Vectorization

A lot of the operations performed by systems will involve operations on a parallel set of
integers/floats, which could potentially be vectorized using SIMD.

This will probably wait until portable SIMD is stable, as right now it's a nightly-only feature.

Additionally, there may be limitations in how effectively we can vectorize systems if we can't
guarantee that a component storage will contain adjacent components that need to be processed,
which is likely.

However, for particularly expensive computations that can be vectorized, it may be
efficient to iterate through the components, copy some of them into an array on the stack, and
then once the collection is filled, perform the vectorized operations. However, for less
expensive systems, this memory copying would probably be slower than just doing the math on a
single number at a time.

### Parallelism & Concurrency

To maximize our use of multi-core CPUs for performance, we need the system to be designed around
concurrent execution. However, we also want to support running the system in contexts without
multiple cores, or at least without building it entirely around Rust's thread implementation.

In other words, it should be built in a concurrency-friendly way, but allow the user to control
the way concurrency is done (and provide a batteries-included implementation for the majority of
users who really don't need to customize it)

### WebAssembly Interoperability

Since Wyrd will be the core data structure of any game that makes use of it, the data contained
within it will need to be accessible to any addons/mods that are attached to the game, and as
designed with the TimberWolf game engine, these mods would be run in a webassembly VM.

In addition to accessing components, mods will also need to be able to run their own systems and
possibly even attach their own data structures.

However, not every use case will require WebAssembly interop, so it shouldn't be a requirement
either, and the performance of the ECS shouldn't be stepped on just to make it possible.
