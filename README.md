# Chapter4 [Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

- Ownership is Rust's most unique feature, ownership give rust a safer memory management, you don't need a GC or allocate and free memory by yourself.
- Known size variable has no ownership, in other words the variables declared on the stack don't have ownership.
- The variables with unknow size or changeable size while runtime or declared on the heap have ownership.
- The varibale will be dropped when it's own out of scope.
- When assigning a variable to another or use it to call a function will give it's ownership, you can't use it in the scope you introduced it anymore.
- We can use reference to borrow the value instead of giving the ownership.
- Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
- The default reference is immutable, you can't change the value of a varaible using immutable reference.
- Decalare a mutable reference using "&mut", we can only have one mutable reference of a variable in a scope because we need to avoid data races or racing condition.
- Slice"&s[...]" let you reference a contiguous sequence of elements in a collection rather than the whole collection.
- &str is more flexable than &String.
- Dangling reference is a reference points to some freed memory.
