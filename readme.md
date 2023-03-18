# Day 1

## Strings
Rust has TWO types of string!
* `&str` is an immutable buffer of characters in memory. 
  * You usually use this for literals, such as "Herbert". 
  * You can refer to any String as an &str by borrowing it - with &my_string.
* `String` is an all-singing, all dancing buffered string designed for modification. 
  * Internally, String is a buffer of characters with the length stored. 
  * Changing a String updates or replaces the buffer.