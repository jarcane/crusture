# crusture

A simple "Hello World" app, with a Rust server at the backend, and ClojureScript+Reagent on the front end (with Figwheel support).

![crust in action!](https://pbs.twimg.com/media/CKhxltKWgAAP-mK.png:large)

## Why you do this?

Single language development rocks. At my internship this summer, I worked with Clojure+ClojureScript for a full stack user directory app, and it's great. 

But! What does kinda suck about it, is that it effectively means running at least two, possibly more, JVMs at once, and this is murder on system resources. That same simple user directory app consumes over 6.6 GB of RAM on my work machine, between server REPL, CLJS autobuilder, and MongoDB. I actually have had serious performance problems working with CLJ+CLJS from my home machine at all, because it's only got a puny 4GB of RAM and Windows 8 taking up half of that. It's even worse if I want to use Nightcode, that's a couple more JVM instances on top of everything ...

JVMs aren't cheap! And they're only as fast as your poor hardware can keep up with when you start spinning up a bunch of them.

Rust is cool though, and fast. It's my favorite new language ever. It has rad docs, and rad helpful people who work on it. You might say I'm a fan. So I thought "Why not just write the backend server in Rust, and save the overhead of one of those JVMs?"

It means no easy reloading on the backend, and Iron is maybe not as friendly a framework as Compojure either, but it's not so much worse either. Also, my Rust server consumes only a few megabytes of RAM, instead of 400-600 for another JVM to run on the back end. 

So that's a perk.

## Setup

### Step 1: Prereqs

You'll need Rust and Cargo. You'll also want Leiningen. 

You'll also need to make sure that you have the external prereqs for openssl-sys. Otherwise the Rust server will fail to build. You can get instructions on how to do that here: https://github.com/sfackler/rust-openssl

### Step 2: Building your ClojureScript

First, you need to build the ClojureScript side. After opening up a terminal, you have two options:

1) To get an interactive development environment run:

    lein figwheel

This will auto compile and send all changes to the browser without the
need to reload. After the compilation process is complete, you will
get a Browser Connected REPL. 

2) If you've no need for the live REPL, and don't mind reloading after changes, you can save some overhead and instead just do:

    lein cljsbuild dev
    
To clean all compiled files:

    lein clean
    
### Step 3: Build the server

This part's easy. Open a second terminal, and in your project folder run:

    cargo run
    
The server will start up, and host your files at [http://localhost:3000](http://localhost:3000).

### Step 4: Deployment! (optional)

To create a production build run:

    lein cljsbuild once min
    cargo build --release

## License

Copyright © 2014 John S. Berry III

Distributed under the Eclipse Public License either version 1.0 or (at your option) any later version.
