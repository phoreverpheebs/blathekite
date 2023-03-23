# blathekite

_blatherskite, but the 'rs' is in the code._ Expanding on the project
[gibberish], which demonstrates using `jmp` instructions to jump through an 
x86 execution flow, `blathekite` does this by trusting the compiler to compile 
to the instructions we want and in the order we want.

As of now, this branch defines the printed string in plaintext similar to the
[defined-string] branch in `gibberish`. Similar methods as the main branch of 
[gibberish], which scatters printable bytes across the binary, are being 
actively worked on!

[gibberish]: https://github.com/phoreverpheebs/gibberish
[defined-string]: https://github.com/phoreverpheebs/gibberish/tree/defined-string
