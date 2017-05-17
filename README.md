# Utility string data structures for Rust

Currently contains (more welcome):

* Rope - an unbalanced rope - efficient append, insertion, and removal of substrings;
* SrcRope - an unbalanced rope which allows for insertion etc. based on the original positions in the string as well as the current postitions;
* StringBuffer - a String/linked list hybrid which is efficient for appending strings and iteration over characters from the start of the string.

All data structures are unicode (UTF8) based and interact as closely as possible with Rust strings.

TODO document char iterators


TODO
----

share code between rope and src_rope
balanced rope
