# My _Rust book_ Notes ✨

### Central commands 👑

- 🆕 Create a new project: `cargo new`
- 🔨 Build the project: `cargo build`
- 🚀 Build & run in one step: `cargo run`
- 👀 Check for errors (no binary): `cargo check`
- 📦 Build output goes to: `target/debug/`, not the code folder

❗ _NB: When you initialize a project using `cargo new`, a Git repository and `.gitignore` are auto-created unless the command is run inside an existing Git repo._

---

### 📁 Project Files & Artifacts

#### 🔒 `Cargo.lock`

`Cargo.lock` is how Cargo keeps your builds loyal — no flaky updates, no version drama 💅  
It locks in exact crate versions so your project doesn’t randomly break on someone else’s machine.

✨ `cargo clean` won’t delete it — this diva doesn’t wipe easily.
❗ Never add `Cargo.lock` to `.gitignore` (for binary projects).

When you want to update a crate &rarr; `cargo update`.

#### 🏗️ `target/`

- All compiled output lives here (`target/debug/`, `target/release/`)
- You can delete it with `cargo clean`
- It’s **big**, temporary, and **should be `.gitignore`d`** (and Cargo does this by default)

---

### Docs 📄

- Stadard libraries set / _Prelude_: https://doc.rust-lang.org/stable/std/prelude/index.html

> ❗ **Tip:** You won’t magically know which traits or methods to use in a crate — but that’s what docs are for!
> Run cargo doc --open to open local docs for all your dependencies in the browser.
> Wanna explore rand? Just run the command and click rand in the sidebar 💙
