To build:

```
$ cargo build
```

To run

```
$ cd examples
$ ../target/debug/spl-lib double-add
Listening on http://127.0.0.1:8000
```

In a separate terminal:

```
$ curl -d '{}' localhost:8000/double-add
{"result":600}
```

This runs the `double-add` LLSPL program.

