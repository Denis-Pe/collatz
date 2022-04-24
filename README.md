# Collatz Conjecture in Rust

Here I have an implementation of the Collatz Conjecture in Rust.
Small but funny program if you want to see your CPU do some work yet seeing what is the highest number it can reach at least with a 16 byte integer.

## Considerations

I had initially thought about doing it with BigUint so that it has no limit, but I liked the fact that a fixed-size integer does not deal
with allocations and all kinds of stuff that slow the program down and also the simplicity of it.

I also thought about doing the conjecture itself in assembly to make it even faster. However, the main problem is that, from what I have heard at least, jumping between
higher level languages and assembly defeats the purpose since you lose on calling the function and stuff.
So, in theory, if you will use assembly, you should stay in assembly and you better be good at assembly also given
that compilers are very good. So all in all, I better be good at assembly if I will try to gain speed over the compiler
even counting the latency of calling the function and all that.

Probably some day I will make respective branches with assembly and BigUint, but for now I want to work on other things.
