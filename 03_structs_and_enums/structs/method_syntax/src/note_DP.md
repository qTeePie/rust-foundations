# 🦀 Rust Design Patterns: Factory, Builder, and Decorator

---

## 🏭 Factory Pattern

**Purpose:**
Encapsulates _object creation logic_ so you don’t manually construct structs everywhere.
You just call the factory, and it returns the right object depending on parameters or context.

**When to use:**

- When multiple structs implement the same trait and you want to choose which one to instantiate dynamically.
- When object creation involves conditional logic or configuration.

**Example:**

```rust
trait Shape {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Shape for Circle {
    fn draw(&self) { println!("Drawing a circle 🟢"); }
}
impl Shape for Square {
    fn draw(&self) { println!("Drawing a square 🟦"); }
}

struct ShapeFactory;

impl ShapeFactory {
    fn make(shape_type: &str) -> Box<dyn Shape> {
        match shape_type {
            "circle" => Box::new(Circle),
            "square" => Box::new(Square),
            _ => panic!("Unknown shape type!"),
        }
    }
}

fn main() {
    let shape = ShapeFactory::make("circle");
    shape.draw();
}
```

💡 _Use when:_ you want to abstract away the `new()` calls or the actual struct names.

---

## 🧱 Builder Pattern

**Purpose:**
Simplifies construction of complex structs step by step.
Instead of a giant constructor with too many arguments, you call _fluent-style methods_ to configure values and then `.build()` the final object.

**When to use:**

- When struct initialization has many optional fields.
- When readability and immutability matter.

**Example:**

```rust
struct Pizza {
    cheese: bool,
    olives: bool,
    pineapple: bool,
}

struct PizzaBuilder {
    cheese: bool,
    olives: bool,
    pineapple: bool,
}

impl PizzaBuilder {
    fn new() -> Self {
        Self { cheese: false, olives: false, pineapple: false }
    }

    fn cheese(mut self, val: bool) -> Self {
        self.cheese = val; self
    }

    fn olives(mut self, val: bool) -> Self {
        self.olives = val; self
    }

    fn pineapple(mut self, val: bool) -> Self {
        self.pineapple = val; self
    }

    fn build(self) -> Pizza {
        Pizza {
            cheese: self.cheese,
            olives: self.olives,
            pineapple: self.pineapple,
        }
    }
}

fn main() {
    let pizza = PizzaBuilder::new()
        .cheese(true)
        .olives(true)
        .pineapple(false)
        .build();

    println!("Pizza ready 🍕: cheese={}, olives={}, pineapple={}",
             pizza.cheese, pizza.olives, pizza.pineapple);
}
```

💡 _Use when:_ you want expressive configuration and immutability before finalization.

---

## 🎁 Decorator Pattern

**Purpose:**
Adds new behavior to an existing object _without changing its type_.
You wrap it in another struct that implements the same trait and optionally extends or overrides functionality.

**When to use:**

- To “layer” functionality (like logging, caching, metrics) around a core component.
- To dynamically add responsibilities.

**Example:**

```rust
use std::io::{self, Write};

struct LoggingWriter<W: Write> {
    inner: W,
}

impl<W: Write> Write for LoggingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        println!("📝 Logging write: {:?}", std::str::from_utf8(buf).unwrap());
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

fn main() {
    let stdout = io::stdout();
    let handle = stdout.lock();

    let mut logger = LoggingWriter { inner: handle };
    let _ = logger.write(b"Hello world!");
}
```

💡 _Use when:_ you want to modify or extend behavior at runtime, not compile time.

---

## ⚡ Quick Summary Table

| Pattern      | Core Idea                                     | Typical Use Case                                             |
| ------------ | --------------------------------------------- | ------------------------------------------------------------ |
| 🏭 Factory   | A single entry point to construct objects     | Hide complex creation logic / choose between implementations |
| 🧱 Builder   | Step-by-step configuration of complex objects | Many optional params or fluent APIs                          |
| 🎁 Decorator | Wrap an object to extend its behavior         | Add runtime features like logging, caching, or validation    |

---

✨ **Tip:**
All three patterns can combine beautifully in Rust:

- Use a **Factory** to choose _what builder_ to use.
- Use a **Builder** to configure the object.
- Wrap the result in a **Decorator** for dynamic behavior.

---
