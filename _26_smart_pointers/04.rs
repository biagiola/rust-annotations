- Introduction to Smart Pointers
In the previous lesson, we explored raw pointers and compared them to regular references,

and we saw that references had clear benefits over raw pointers in terms of safety.

Unfortunately, references have their own limitations and weaknesses as well.

There are certain things that we cannot build or build well using regular references.

And of course, turning to raw pointers is fine, but that also carries risk.

So for that reason, the language gives us access to smart pointers.

A smart pointer is a type that behaves like a pointer.

You can think of it almost like a raw pointer that is enhanced with extra functionality and metadata

and even safety.

So it's a data type that's still fundamentally contains a memory address, but it also stores additional

pieces of metadata alongside that address.

It's like a premium version of a regular reference or a premium version of a raw pointer.

The rust book defines a smart pointer as a data structure that acts like a pointer, but also has additional
capabilities.

That metadata that we're talking about is just additional pieces of state, right?

So coming back to the real world analogy, a reference is like a piece of paper with an address to a

house. Now imagine that that paper also holds additional data about the house, like the property

tax or the quality of the local schools.

Maybe it holds a more specific set of GPS instructions on how to get to the house. The point is,

it's more than just the address by itself. It's the address plus extra data.

So a smart pointer is a wrapper.

It's a container type that encapsulates or bundles together a reference or a pointer, and some extra

information or functionality on top of that base reference.

Under the hood, most smart pointers are actually going to be implemented using structs, because

struck types have the capacity to store multiple pieces of data through the use of fields.

So you can think of a struct that's storing some reference or some raw pointer to some data somewhere,

plus additional functionality that allows us to elegantly interact with that raw pointer or just general

functionality on top of that core reference.

The smart pointer is storing.