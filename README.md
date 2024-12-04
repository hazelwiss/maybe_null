# MaybeNull

The `MaybeNull` type is used for hinting that a pointer may be null, and still prevent accidental null pointer dereference.

```rs
use maybe_null::MaybeNull;

let ptr = MaybeNull::<u32>::null();

let Some(ptr) = ptr.get() {
  // We know the pointer is non-null here.

  // Whoops! this will never happen because our `ptr` was initialized as null!
  ptr.write(0);
}
``` 

# AtomicMaybeNull

The `AtomicMaybeNull` works like `MaybeNull` but allows for atomic access of the pointer.

