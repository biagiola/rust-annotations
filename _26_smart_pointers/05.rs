- Smart Pointer and borrowring rules
So next let's review the concept of borrowing.

When we borrow something, we use it without taking ownership away from the owner.

So when we create a reference with the borrow operator the ampersand symbol, we borrow data.

References do not take ownership away.

Interestingly enough, smart pointers will often own the data they point to.

This can be a bit confusing, so let's reiterate.

The smart pointer is a wrapper container that will store a pointer or a reference and manage that deallocation

of the data the reference points to as well.

The concept of ownership revolves around who is responsible for deallocating data.

Regular references are not responsible for deallocation.

The original owner cleans up its data.

Smart pointers, in comparison, are types that not only point to some data but also own it.

That can be very confusing when you hear it, but the good news is that complexity of both owning and

borrowing the same data is usually abstracted away or hidden away by the smart pointer.

We don't have to worry about it.

The whole benefit of smart pointers is that they hide away the complexity.

Smart pointers behave like regular pointers, like regular references, and they likely manage some

kind of memory in their internal design.

But we can treat the smart pointer like any owned data type within our program.

That separation, that distance from the raw pointer, or the reference within the smart pointer makes

the program easier to reason about.

So I will assume that all of this terminology sounds really confusing, and the value of smart pointers

is not quite clear.

So here's an example that hopefully brings it all together.

We've actually worked with smart pointers many times throughout the course.

