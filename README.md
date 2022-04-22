# Primitive for aliasing mutability in Rust
In rust, `&mut T` is not normally allowed to refer to aliasing memory. But when writing self
referential structs, one needs aliasing mutable references. This crate provides the
[`UnsafeAliasCell<T>`] primitive type. It works similar to the [`UnsafeCell<T>`] from the
[stdlib]:
> `UnsafeCell<T>` opts-out of the immutability guarantee for `&T`: a shared reference
> `&UnsafeCell<T>` may point to data that is being mutated.


[`UnsafeAliasCell<T>`] opts-out of the uniqueness guarantee for `&mut T`: a unique mutable reference  
`&mut UnsafeAliasCell<T>` may point to data that is being mutated.

## Using [`UnsafeAliasCell<T>`]
One needs to be careful, when using [`UnsafeAliasCell<T>`], because wrong usage leads to undefined
behavior.

Even when using [`UnsafeAliasCell<T>`] it **is considered undefined behavior** to create multiple
aliasing `&mut T`. But you are allowed to create multiple aliasing `*mut T`/`*const T`.

### Example
Use [`UnsafeAliasCell<T>`] on the part that you intend to alias:
```rust
# use unsafe_alias_cell::UnsafeAliasCell;
pub struct SelfReferential {
	item: UnsafeAliasCell<i32>,
	ptr: *const i32,
}
```
Now you are allowed to call [`.get()`] on `item` and store that pointer in `ptr`. For as long as
that `SelfReferential` stays [pinned](https://doc.rust-lang.org/std/pin/index.html), you can use
`ptr` to read the item.

## Undefined behavior
Implementing [`Unpin`] for any type containing a [`UnsafeAliasCell<T>`] is UB.

It is UB to cast the pointer returned by [`.get()`] to 
- `&mut T`, when there exists another pointer (`&T`, `*const T` or `*mut T`) pointing to the inner
of the cell.
- `&T`, when there exists another mutable pointer (`*mut T`) pointing to the inner of the cell.

Similar to [`UnsafeCell<T>`] you need to ensure the aliasing rules for any reference you create
(taken from the [stdlib]):

- If you create a safe reference with lifetime `'a` (either a `&T` or `&mut T` reference) that is
accessible by safe code (for example, because you returned it), then you must not access the data
in any way that contradicts that reference for the remainder of `'a`. For example, this means that
if you take the `*mut T` from an [`UnsafeAliasCell<T>`] and cast it to an `&T`, then the data in T
must remain immutable (modulo any [`UnsafeCell<U>`]/[`UnsafeAliasCell<U>`] data found within T, of
course) until that reference’s lifetime expires. Similarly, if you create a `&mut T` reference that
is released to safe code, then you must not access the data within the [`UnsafeAliasCell<T>`] until
that reference expires.
- At all times, you must avoid data races. If multiple threads have access to the same
[`UnsafeAliasCell<T>`], then any writes must have a proper happens-before relation to all other
accesses (or use atomics).

## How does it work?
Under the current rules, all types that are [`!Unpin`] do not emit `noalias` for `&T` and `&mut T`
in [LLVM] and are thus able to alias. For [`UnsafeAliasCell<T>`] to be sound, it is therefore
required to be contained in only [`!Unpin`] types.

[stdlib]: https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html
[`.get()`]: UnsafeAliasCell::get
[`!Unpin`]: Unpin
[LLVM]: https://llvm.org/
[`UnsafeAliasCell<T>`]: https://docs.rs/unsafe-alias-cell/latest/unsafe_alias_cell/struct.UnsafeAliasCell.html
[`UnsafeCell<T>`]: https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html
[`UnsafeAliasCell<U>`]: https://docs.rs/unsafe-alias-cell/latest/unsafe_alias_cell/struct.UnsafeAliasCell.html
[`UnsafeCell<U>`]: https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html
