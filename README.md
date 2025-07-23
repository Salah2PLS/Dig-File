# Ftext: a Simple `grep` coded in Rust

### Example usage: (Note: its colored, but you cant see cuz of bad MarkDown)
```bash
~ $ ./ftxt -h
----------- FText Help -----------
ftxt *args
   -h: Help,   print this message.
   -v: Ver.,   print the Version.
   -i: Ignore, ignore char case.
   -r: Regex,  search using Regex instead of normal String (todo!)
----------- FText Help -----------
~ $ ./ftxt -v
FText v0.1
Licensed under Apache v2.0 License, ©2025 Salah Al-Refaai
~ $ ./ftxt -i "int" code.c
grepping "code.c"
1: int main() {
2:     int x = 5;
3:     int y = 10; // int
4:     int sum = x + y;
7:         printf("Sum is %d\n", sum);

...Done!
~ $ ./ftxt -r

thread 'main' panicked at src/main.rs:46:17:
not yet implemented: cannot use regex right now, but its comming soon (if this project continued)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
~ $
```
