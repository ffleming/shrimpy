Shrimpy
===

Shrimpy is an http server.  It is called 'Shrimpy' because the Rust community has an affinity for crustaceans and one of my son's favorite toys is a small stuffed shrimp.

Also, Shrimpy is very small and shrimpy.

Shrimpy exists solely for me to play around with Rust.  Because the point is to get familiar with the language, Shrimpy does not use any crates.

Shrimpy will probably never support verbs beyond GET and, if I'm feeling especially sassy, POST.

TODO
---
1. Maybe don't allow path traversal (fun fact: I couldn't trivially trigger this, but it has to be there.  It just has to.)
2. ~~Module & struct for server~~
3. Configurable root directory, serve files from it
4. Logging that isn't `println!()`
5. ~~Daemonization~~
6. Privilege dropping in child
7. ~~Argument parsing / help text~~
8. ~~Module & struct for argument parser~~
9. Help text generation for argument parser
10. Allow argument parser to specify required arguments
