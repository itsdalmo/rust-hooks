[![OSX/Linux](https://travis-ci.org/itsdalmo/rust-hooks.svg?branch=master)](https://travis-ci.org/itsdalmo/rust-hooks)
[![Windows](https://ci.appveyor.com/api/projects/status/ivr13cpisyc50mkb/branch/master?svg=true)](https://ci.appveyor.com/project/itsdalmo/rust-hooks/branch/master)

## Rust Git Hooks

Personal git hooks made in rust (to test rust).

#### Usage

Sym-link the binary to the .git directory in a project.

```bash
ln -s hooks .git/hooks/commit-msg
ln -s hooks .git/hooks/pre-push
```

