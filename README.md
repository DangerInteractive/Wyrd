# Wyrd

**an entity component system written in Rust**

## Is this ready to use in my project?

No.

Make sure that you read `LICENSE` and understand the license terms and disclaimers of liability
before you make use of this library.

## What features does Wyrd provide right now?

- Generic component storage structs

## What features are planned for the future?

- `Domain` struct in which component storages are stored
- Ability to run systems on the components in a domain
- Archetype organization system so components likely to be used together have cache locality
- System scheduling and timing system
- Ability to use the ECS from a WebAssembly VM running on top of the rust code hosting it

## How is Wyrd pronounced?

It should be pronounced just like the modern English word **weird**.

## What does Wyrd mean?

Wyrd is an Old English word roughly translating to **fate**. There is some debate around the
exact significance of the concept to the ancient Anglo-Saxons, but it does not fully match our
modern meaning of fate.

As opposed to the **fates**, or the Old-Norse **nornir** or **urðr**, Wyrd is a concept
generally seen a force, rather than a deity or group of deities.

The Old-Norse equivalent and cognate **Urðr** is a goddess, one of the three Norns, and the one
representing the past.

For this reason, I consider Wyrd to represent fate in the past sense.
Unchangeable as the past is.

*"Wyrd bið ful aræd"* - A line from an Old English poem called "The Wanderer", translates to
"fate is wholly inexorable", and sums up the concept succinctly, but with many interpretations
that ought not to be explored in software readme.

Since an ECS handles storing the present state, the culmination of past events, and manages
change over time, **Wyrd** is a relevant concept, and that's why I chose the name.

## How to Use

This is a Rust library, available through crates.io.
To install in your project, add it to your Cargo.toml as a dependency:

```toml
[dependencies]
wyrd = "0.1.0"
```

Or run the following command from within your project directory:

```shell
cargo add wyrd
```
