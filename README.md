Shrimpy
===

Shrimpy is an http server.  It is called 'Shrimpy' because the Rust community
has an affinity for crustaceans and one of my son's favorite toys is a small
stuffed shrimp.

Also, shrimpy is very small and, well, shrimpy.

Shrimpy exists solely for me to play around with Rust.  Its aim is to get a
functioning server that is capable of serving statically rendered web pages.

Shrimpy will probably never support verbs beyond GET and, if I'm feeling
especially sassy, POST.

Shrimpy does not use any crates, with the possible exception of libc so I can
use `fork()`.  If I'm being realistic, though, Shrimpy probably won't make it
that far.
