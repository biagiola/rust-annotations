Believe it or not, a heap string is an example of a smart pointer.

A string is a smart pointer because it stores a reference to some heap memory where the text data is

allocated, and it also stores additional metadata like the length and capacity of that heap location.

Recall how a string lives up to the definition of a smart pointer.

It's a type that behaves like a pointer that does reference some data that acts as a wrapper around

that reference, but can be treated just like an owned regular type.

The string type literally stores a reference to heap data, but we never have to worry about that reference.

The string hides that complexity away from us in our program.

In our code, we treat the string as a regular own type.

The string handles the deallocation of that heap memory and the reference.

We never have to work with the strings internal raw pointer, and that is the primary benefit of smart

pointers.

We can treat a smart pointer like a regular own type, and that's a lot less complex to reason about

than references or raw pointers, or all of the things associated with references like lifetimes.

Think about how easy a string is to work with compared to something like raw pointers in the previous

lesson.

Imagine that we had to manually manage the text data on the heap ourselves, but also have a nice type

in our program that we can call methods on and interact with that behaves just like a regular own type,

right?

We don't have to implement that from scratch, because the string gives us that power.

And the string is an example of a smart pointer.

And other smart pointers are going to exist to offer the exact same benefit as the strength smart pointer

right there.

Going to help us manage some kind of references or pointers to some data elsewhere, but through much

more simple owned data types that we can interact with directly in our program.

And of course, the smart pointers are also going to be much safer.

They might package up raw pointers under the hood, but we don't have to worry about that code.

That code is written by other professional rust developers that has been battle tested.

It's safe in the sense that it has a lot more rigor in the analysis and evaluation of the code to do

what it's supposed to do.

So smart pointers are going to give us a lot of the benefits of raw pointers and enable us to do operations

that we couldn't with plain references, while also giving us additional safety checks and concerns

that a raw pointer could not.

So it really is the best of both worlds, and the best way to see this is when we actually start to

build something that will require us to use a smart pointer.

We're going to start talking about that a few lessons from now with that.

That's all there is to cover in this lesson, so I will see you in the next one.


