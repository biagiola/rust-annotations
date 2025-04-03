// The vector capacity: the number of elements that the vector can contain

// when we reach a max, rust is going to find a new largar location on the heap that has increased capacity.
// It's going to move all of the previous elements over there and then add the new one as well.
// The old memory will be deallocated because it no longer holde the data it use to.

// Behind the scenes, that's why rust also avoid to have multiple mutable ref, because one of them can add
// so much values, that makes to move the heap position and the other no longer knows the new location.