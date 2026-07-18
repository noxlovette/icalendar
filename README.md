A tree-parser of an RFC spec into Rust and vice versa

```
bytes
↓
Lexer
↓
Parser
↓
Generic AST
(Component, Property)
↓
Semantic Analyzer
↓
Typed Components
(VCalendar, VEvent, VTodo...)
↓
Typed Properties
(DtStart, Organizer, Attendee...)
```
