# Ownership in Rust

Rust has a concept of ownership that is unique among programming languages. It is a key feature of the language that allows it to be both safe and fast. In this article, we will explore what ownership is, how it works, and why it is important.

## What is ownership?

Ownership is a concept that is unique to Rust. It is a way of managing memory that allows the language to be both safe and fast. In Rust, every value has a single owner, and that owner is responsible for cleaning up the value when it is no longer needed. This means that there are no garbage collectors or reference counting in Rust, which allows it to be fast and predictable.

In this lesson, you’ll learn ownership by working through some examples that focus on a very common data structure: strings.

## The Stack and the Heap

The stack operates on a last-in, first-out basis, where data is added ("pushed") and removed ("popped") in reverse order. It's efficient but limited to data with a known, fixed size at compile time.

The heap, on the other hand, is more flexible but less organized. When you need to store data with an unknown size at compile time or variable size, it goes on the heap. This involves requesting space and receiving a pointer to the allocated space. The pointer, being of a fixed size, can be stored on the stack. Accessing heap data requires following this pointer, which is slower than stack access due to the additional step and potential for memory jumps.

Stack operations are generally faster than heap operations. The stack doesn't require searching for space or complex bookkeeping, unlike the heap, which needs space allocation and management.

When a function is called, its arguments and local variables are pushed onto the stack. After the function completes, these are popped off.

Ownership in Rust deals with managing heap data, ensuring efficient use of memory, avoiding duplication, and cleaning up unused data to prevent running out of space. Understanding ownership helps manage heap data effectively, though with a good grasp of ownership, you won't need to focus much on stack and heap specifics.

## Ownership Rules

Rust has a few ownership rules:

1. Each value in Rust has a variable that's its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

Let's see an example.

### Example

```rust
fn main() {
{                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

In the example above, `s` is the owner of the string "hello". When `s` goes out of scope, the string will be dropped.

## The String type

In a previous lesson, we discussed `data types` in Rust. Those types are all simple and fixed in size. Now we'll discuss the `String` type, a compound type that is allocated on the heap. We will also see the `String` type in detail in an upcoming lesson.

We've already seen string literals, which are immutable and hardcoded into the program. The `String` type, however, is mutable and can be changed. It is also allocated on the heap, which means it can store an amount of text that is unknown to us at compile time.

```rust
let s = String::from("hello");
```

This kind of string can be mutated:

```rust
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
```

So, what’s the difference here? Why can String be mutated but literals cannot? The difference is in how these two types deal with memory.

```rust
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
```

There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.




Invalidated reference:

```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
```

## Variables and Data Interacting with Clone

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone. We’ll discuss method syntax in Chapter 5, but because methods are a common feature in many programming languages, you’ve probably seen them before.

```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

## Stack-Only Data: Copy

There’s another wrinkle we haven’t talked about yet. 

This code using integers is valid:

```rust
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```

The reason is that integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack. That means there’s no reason we would want x to invalidate y when x goes out of scope. In other words, there’s no difference between deep and shallow copying here, so it’s safe for Rust to assume it’s okay to just copy the bits for x into y.

Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are.

Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error

So, what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

## Ownership and Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. 

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error. These static checks protect us from mistakes. Try adding code to main that uses s and x to see where you can use them and where the ownership rules prevent you from doing so.


## Return Values and Scope

Returning values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```


The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

It’s possible to return multiple values using a tuple, as shown in the following example:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}
```

Rust has also introduced a new concept called references, which allow you to refer to some value without taking ownership of it. We’ll discuss references in the next lesson.