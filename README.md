# blathekite

_blatherskite, but the 'rs' is in the code._ Expanding on the project
[gibberish], which demonstrates using `jmp` instructions to jump through an 
x86 execution flow, `blathekite` does this by trusting the compiler to compile 
to the instructions we want and in the order we want.

[gibberish]: https://github.com/phoreverpheebs/gibberish

## Interesting things

### 'lea' instead of 'add' optimisation

Since ABIs for x86 architectures usually define _rdi_, _rsi_, ... as
registers for arguments to be passed in and _rax_ as the register for
the return value, the compiler will optimise an add function to use
a `lea` instruction first, as the `SIB-byte` allows us to perform quick
addition, multiplication and subtraction, whilst also writing the result
to a seperate register.

Therefore, we may notice that in the `deadname` function, we assign the 
addition of variables _di_ and _si_ to an accumulator variable. This eliminates
the initial use of a `lea` instruction for further `add` operations.

## why is the source code so girlypop?

Because it's adorable!!!
