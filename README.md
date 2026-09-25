# RET Runtime

RET is a tiny cross-platform runtime for .ret programs.

The same hello.ret source can be run by the Windows ret.exe or the Linux ret binary.

## Run

Windows:
    ret.exe examples/hello.ret

Linux:
    ./ret examples/hello.ret

## Build locally

Install Rust, then:
    cargo build --release

The binaries are written to target/release/.

## Syntax

    print "Hello"
    pause

Lines beginning with # are comments.

## Downloads

GitHub Actions builds Windows x64 and Linux x64 binaries automatically on every push. Open the Actions run and download the ret-windows-x64 or ret-linux-x64 artifact.

The .ret files are portable source files. They are interpreted by the small RET runtime rather than executed directly by the operating system.
