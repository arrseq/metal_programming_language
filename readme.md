# Line Ending are LF `\n` not CLRF `\r`
The parser will not work if it reaches `\r`

# Expression Syntax
> With rust counterpart

**Basic byte reference**
```
&integer1     // &i8
&integer4     // &i32

// Parse steps
// - found '&', run Node::parse recursive.
// - found '&', run Node::parse again.
// - found non-keyworded, type is Reference(Reference(integer4)
&&integer4 // &i32

#integer4  // &mut i32
```