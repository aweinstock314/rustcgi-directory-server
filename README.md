# rustcgi-directory-server
# Dependencies

- `rustcgi-directory-server` assumes that `cargo-script` is installed, which can be done with `cargo install cargo-script`.
- The `fswatch` C library is a dependency. This repo's `Makefile` should fetch it as a submodule and compile it.

# Discussion of the design considerations
```
13:56:20 < mst> Obormot\Arcturus: but also, for anything except PHP, how would you do "just unzip this"
13:57:04 < Obormot\Arcturus> mst: I dunno? I'm hardly the right person to ask - I was gonna ask you the same thing tbh! I mean, why can't python work like php? Why can't I put an index.py into a folder somewhere or upload it somewhere and have it run when I make 
                             web requests?
13:57:11 < Obormot\Arcturus> What's the reason? Is that reason fixable?
13:57:26 < Obormot\Arcturus> Or perl?
13:57:48 < Obormot\Arcturus> Is it a web server issue? Do we need a mod_python? Why doesn't someone write one for nginx or something?
13:57:48 <@saturn2> the main reason is that python is too slow. perl can do it
13:57:48 < mst> I mean on dreamhost I scp up a perl file and it generally just runs
13:58:13 < Obormot\Arcturus> saturn2: So python is slower than php, is what we're saying here?
13:58:19 < MarkOtaris> yes, of course it's a web server issue
13:58:32 <@saturn2> Obormot\Arcturus: yes
13:58:34 < MarkOtaris> nothing to do with python being slow—you don't do this with other faster programming languages either
13:59:10 < mst> PHP has really fast startup in return for runtime inefficiencies
13:59:13 < MarkOtaris> using apache, etc. is slowly falling out of favor, so people do not want to develop new... modules for other programming languages
13:59:32 < Obormot\Arcturus> saturn2: gosh maybe the people criticizing PHP and evangelizing Python might want to ever mention that or consider it! wow
13:59:42 <@saturn2> there aren't many languages that are fast enough - php, perl, bash... that's all i can think of
13:59:44 < Obormot\Arcturus> Like maybe speed matters??? Is that even possible??
14:00:26 < Khoth> I don't think speed is the reason
14:00:35 < Khoth> If it was, we wouldn't have modern websites
14:00:49 < Obormot\Arcturus> Really, it's amazing that people can go "lol PHP is bad, why does anyone use it, only idiots use it, lol" and yet if you actually try to instead use something else, well...
14:00:51 < aweinstock> if there were a nginx/apache plugin that watched a directory for Rust file changes and cached the binaries and used them to serve requests, what fraction the sort of people who use PHP for webdev would be likely to switch?
14:01:07 <@saturn2> none lol
14:01:16 < aweinstock> (or Haskell, or OCaml, or Standard ML, etc)
14:01:25 < Obormot\Arcturus> I mean, I dunno about *Rust* exactly
14:01:42 < aweinstock> one of the languages with a good static type system
14:01:50 < Obormot\Arcturus> aweinstock: Would it literally be as easy as "put a Rust file into a folder and it runs when you do web requests"?
14:02:07 < Obormot\Arcturus> I'd at least try it
14:02:14 < Obormot\Arcturus> This would get me to try learning Rust
14:02:41 < aweinstock> I think so, that's doable with some inotify-style directory watcher and I think cargo-script already does the caching
14:02:47 < Obormot\Arcturus> I mean, going back to my original point: when it comes to web dev, I am totally an amateur
14:03:18 < Obormot\Arcturus> I don't want to learn some complicated fucking ecosystem and build process and invest massive amounts of time into some arcane shit
14:03:38 < Obormot\Arcturus> Learning a new programming language? I can do that
14:04:11 < Obormot\Arcturus> Let me get right to writing code and experimenting, such that I can immediately start trying to build what I want to build, and learn as I go, and I'm in
14:04:18 < aweinstock> err, do you use linux webservers, and if not, do you know offhand what the MacOS equivalent to inotify (for watching a directory for file changes without actively polling) is?
14:04:22 < Obormot\Arcturus> But in web dev, that *literally just means php*
14:04:34 < Obormot\Arcturus> aweinstock: I believe NFSN runs some manner of BSD, actually
14:04:37 < Obormot\Arcturus> Not 100% sure
14:04:44 <@saturn2> freebsd
14:04:48 < Obormot\Arcturus> There we go
14:05:33 < Obormot\Arcturus> aweinstock: The Mac's version of inotify is FSEvents, fwiw
14:05:41 < Obormot\Arcturus> https://en.wikipedia.org/wiki/FSEvents
14:05:41 < Robomot> FSEvents - Wikipedia (The FSEvents API in macOS allows applications to register for notifications of changes to a given directory tree.[1] Whenever the filesystem is changed, the kernel passes notifications via the special device file 
                    /dev/fsevents to a userspace process called fseventsd. …)
14:06:13 < aweinstock> would you be developing locally, or would you copying files over to the freebsd box
14:06:15 < Obormot\Arcturus> https://github.com/emcrisostomo/fswatch exists, which lets you write cross-platform stuff
14:06:16 < Robomot> GitHub - emcrisostomo/fswatch: A cross-platform file change monitor with multiple backends: Apple OS X File System Events, *BSD kqueue, Solaris/Illumos File Events Notification, Linux inotify, Microsoft Windows and a stat()-based backend.
14:06:23 < aweinstock> ah
14:06:48 < Obormot\Arcturus> aweinstock: I mean, either. Both. Why, do you have something in mind?
14:07:14 < aweinstock> neat, fswatch has a C library I could bind to
14:08:05 < aweinstock> I'm half-considering writing this tool (a thing to watch a directory for Rust files and serve them CGI-style)
14:09:58 < Obormot\Arcturus> aweinstock: If you do, let me know!
14:10:07 < aweinstock> the design I'm thinking of wouldn't include a templating engine, so it wouldn't be a drop in replacement for PHP (in the sense that you'd still need to write `fn main()` and emit HTML on stdout manually), would you still try it?
14:10:27 < Obormot\Arcturus> aweinstock: yes
14:11:07 < Obormot\Arcturus> Don't get me wrong, templating is a great PHP feature, but emitting HTML on stdout would be huge
14:11:20 < Obormot\Arcturus> I even know exactly what my first project would be:
14:11:25 < aweinstock> apparently there's a bunch of library options: https://crates.io/categories/template-engine
14:11:25 < Obormot\Arcturus> Rewriting my font server in Rust
14:11:35 < Obormot\Arcturus> Which doesn't need templating
14:13:12 < aweinstock> also, if you'd want to use rust for client-side stuff as a JS replacement, the web_sys ecosystem is decent
14:13:46 < Obormot\Arcturus> One step at a time... :p
14:14:33 < aweinstock> several of the types in https://github.com/aweinstock314/wasm-snake/blob/master/src/client.rs (e.g. CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent) are actually the JS objects directly
14:14:34 < Robomot> wasm-snake/client.rs at master · aweinstock314/wasm-snake · GitHub (Contribute to aweinstock314/wasm-snake development by creating an account on GitHub.)
14:14:50 < aweinstock> fair :)
14:16:11 < aweinstock> I'll let you know when I have a draft of the tool
14:18:09 < Obormot\Arcturus> aweinstock: k!
```

# Choice of notification API
- `fswatch` seems to be the most cross-platform
- `notify` only explicitly has Windows/MacOS/Linux support, it would poll on FreeBSD
- There are pre-existing `fswatch` and `fswatch-sys` crates, but they were last updated years ago (less recently than the C version of `fswatch`)
- Best solution seems to use `bindgen` to make my own `fswatch-sys` and use that.
