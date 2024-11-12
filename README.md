# konpatch

brainfuck interpreter

# quick start

build project

```shell
cargo build
```

write brainfuck code & save `test.bf`

```
+++++++++[->++++++++>+++++++++++>+++++<<<]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.
```

run code

```shell
target/debug/konpatch interpreter test.bf
```

```
Hello World!
```
