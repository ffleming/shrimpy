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

TODO
---
1. Maybe don't allow path traversal (fun fact: I couldn't trivially trigger
   this, but it has to be there.  It just has to.)
2. Struct for server
3. Configurable root directory, serve files from it
4. Logging that isn't `println!()`
5. Privilege dropping in child
