# LEARNING ROADMAP

## Vision

Use Rust to build a small Git implementation.

The project focuses on understanding Git internals through real implementation rather than reproducing every Git feature.

---

# Stage 0 — Repository Basics

Implement:

```bash
rust-git init
```

Learn:

- Cargo project structure
- filesystem operations
- `Path` / `PathBuf`
- `Result`
- basic Git repository structure

Understand:

```text
.git/
├── HEAD
├── objects/
└── refs/
    └── heads/
```

Milestone:

Create a valid basic Git repository structure.

---

# Stage 1 — Git Object Database

Implement:

```bash
rust-git hash-object
rust-git cat-file
```

Learn:

- Git objects
- blob
- SHA hashing
- binary data
- zlib compression
- file I/O

Understand:

```text
blob <size>\0<content>
        ↓
      SHA-1
        ↓
.git/objects/xx/yyyy...
```

Milestone:

Store and read Git blob objects.

---

# Stage 2 — Trees and Staging

Implement:

```bash
rust-git add
rust-git ls-files
rust-git write-tree
```

Learn:

- Git index
- staging area
- tree objects
- binary formats
- serialization

Understand:

```text
Working Tree
     ↓
   Index
     ↓
Tree Object
```

Milestone:

Stage files and generate a Git tree.

---

# Stage 3 — Commits

Implement:

```bash
rust-git commit
rust-git log
```

Learn:

- commit objects
- parent commits
- timestamps
- repository history

Understand:

```text
Commit
 ├── Tree
 ├── Parent
 ├── Author
 └── Message
```

Milestone:

Create commits and traverse history.

---

# Stage 4 — Status and Diff

Implement:

```bash
rust-git status
rust-git diff
```

Learn how Git compares:

```text
Working Tree
Index
HEAD
```

Understand:

- untracked files
- modified files
- staged files

Milestone:

Detect repository changes correctly.

---

# Stage 5 — Branches and HEAD

Implement:

```bash
rust-git branch
rust-git switch
```

Learn:

- refs
- symbolic refs
- HEAD
- branch pointers

Understand:

```text
HEAD
 ↓
refs/heads/main
 ↓
commit
```

Milestone:

Create and switch branches.

---

# Stage 6 — Checkout / Restore

Implement:

```bash
rust-git checkout
```

Learn:

- restoring trees
- updating working tree
- updating index
- filesystem conflicts

Milestone:

Restore a commit into the working directory.

At this point the project already forms a usable local version control system.

---

# Stage 7 — Merge Basics

Implement a simple:

```bash
rust-git merge
```

Start with:

- fast-forward merge

Then optionally learn:

- common ancestor
- three-way merge
- merge conflicts

Milestone:

Understand how Git combines history.

---

# Stage 8 — Remote Git

Learn:

- remotes
- packfiles
- Git transport
- HTTP Git protocol

Target commands:

```bash
rust-git clone
rust-git push
```

Do not implement remote functionality until the local object model, refs, commits, and history are fully understood.

Milestone:

Communicate with a real Git remote.

---

# Final Goal

The final project should support a useful subset such as:

```bash
rust-git init
rust-git add
rust-git commit
rust-git status
rust-git log
rust-git branch
rust-git switch
rust-git checkout
rust-git push
```

The most important outcome is not command count.

The developer should understand the complete path:

```text
File
 ↓
Blob
 ↓
Index
 ↓
Tree
 ↓
Commit
 ↓
Branch / HEAD
 ↓
History
 ↓
Remote
```

# Learning Principle

Only learn Rust concepts when the current Git feature requires them.

Prefer:

```text
Understand Git concept
        ↓
Design minimal representation
        ↓
Learn required Rust
        ↓
Implement
        ↓
Test
        ↓
Explain what happened
```

over studying large amounts of Rust or Git theory before writing code.
