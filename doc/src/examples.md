# Examples

Each page in this section demonstrates the examples compiled as WASM applications.

If you are running this book locally, you may compile all examples before visiting the pages using the following command:

```bash
for example in $(ls examples)
do wasm-pack build \
  --target web \
  --out-dir "../../doc/src/examples/pkg" \
  --release \
  "examples/${example}"
done
```
