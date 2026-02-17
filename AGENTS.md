**readability** is a Rust port of [mozilla/readability](https://github.com/mozilla/readability).

### Overview

Look into `submodules/readability/Readability.js` for the JavaScript code we are
porting. This should be your primary reference guide.

The goal is to have a modular implementation that is implemented by running
'stages' on the document, and aggregating data into a shared context structure.

A 'stage' is a way to organize mutations or information gathering.

For instance, we have a `RemoveDisallowedNodes` stage:

```rust
use super::*;

pub struct RemoveDisallowedNodes;

impl Stage for RemoveDisallowedNodes {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.document.select("script, style, noscript").remove();

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  test! {
    name: removes_script_tags,
    stage: RemoveDisallowedNodes,
    content: "<html><body><script>alert('hi');</script><p>Content</p></body></html>",
    expected: "<html><head></head><body><p>Content</p></body></html>",
  }

  test! {
    name: removes_style_tags,
    stage: RemoveDisallowedNodes,
    content: "<html><head><style>body { color: red; }</style></head><body><p>Content</p></body></html>",
    expected: "<html><head></head><body><p>Content</p></body></html>",
  }

  test! {
    name: removes_noscript_tags,
    stage: RemoveDisallowedNodes,
    content: "<html><body><noscript>Enable JS</noscript><p>Content</p></body></html>",
    expected: "<html><head></head><body><p>Content</p></body></html>",
  }
}
```

This stage removed `script`, `style` and `noscript` elements from the document.
Later stages act on the document without these elements.

All logic should be encapsulated into its own stage, and we prefer smaller, more
testable stages over larger ones.

### Testing

We have set up an integration test suite over at `tests/integration.rs`. Because
we aim to have full compatibility with the JavaScript implementation, we've set
up the test suite in a way that makes it easy to compare against output from the
original implementation.

Tests are split up between full tests, output tests, and metadata tests.

The goal is to have one single `test!(...)` call that passes per directory entry
located at `/Users/liam/src/readability/submodules/readability/test/test-pages`.
For instance:

```rust
test!("001");
test!("002");
test!("003-metadata-preferred");
test!("004-metadata-space-separated-properties");
test!("005-unescape-html-entities");
test!("aclu");
...
```
