# jsonmap

Transform data from `stdin` using javascript.

## How does it work?

- data in `json` format needs to be piped to `jsonmap`
- write a closure in `js` syntax as argument for jsonmap (tip: use `'` single ticks in your shell for this
- `node` will execute your lambda on the closure and the return value of the closure will be printed (as `json`) to stdout

Fully functional!
