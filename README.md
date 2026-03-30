# simple_ref_count

A simple reference counting implementation. Includes a non-atomic version `RefCount` and an atomic version `AtomicRefCount`.

## API

- `new() -> Self` – Creates a new reference counter (initial count = 1)
- `is(&self, other: &Self) -> bool` – Returns `true` if both handles point to the same counter
- `count(&self) -> usize` – Returns the current reference count
- `clone(&self) -> Self` – Clone the reference (count += 1)
- `drop(&mut self)` – Drop the reference (count -= 1 and free counter if last one dropped)

## Usage

```rust
let rc1 = simple_ref_count::RefCount::new(); // or AtomicRefCount::new();
let rc2 = rc1.clone();

assert!(rc1.is(&rc2));
assert_eq!(rc1.count(), 2);

drop(rc1);
assert_eq!(rc2.count(), 1);
```

## Safety

- `RefCount` is **not thread-safe** – use only in single-threaded contexts.
- `AtomicRefCount` is thread-safe