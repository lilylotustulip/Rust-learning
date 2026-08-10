# OWNERSHIP
Ownership is a set of rules that manages how any rust programs is compiled because it manages its memory.
to understand how a value behaves you have to know whether its on the stack or the heap.
the stack and the heap are available memory spaces to use when running the code.
the stack is like short-term memory, it is fixed in size, therefore if you are unsure about the size of the data use the heap, the heap returns a pointer as the adress of the location it found after being requested to be used.

## RULES
Keep these rules about ownership in mind:
* **each value in Rust has an owner**
* **there can be only one owner at a time**
* **when the owner goes out of scope, the value will be dropped**