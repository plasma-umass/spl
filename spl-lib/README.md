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

# LLSPL APIs

This documents the JSON interface of LLSPL primitives. LLSPL actions such as
`pure`, `split`, `if`, and `project` consume JSON values as input and produce
JSON values as output, but different actions make assumptions and manipulate
JSON in certain ways.

- `pure "actionName": json -> "actionName" -> json` 

  JSON is passed as input to `actionName`, and the output JSON is
  `actionName`'s result.

- `split (expr): { "x": <input>, "y": <state> } -> expr -> { "x": <output>, "y": <state> }`

  The input JSON is expected to be a JSON object with two fields, `"x"` and
  `"y"`. The value of the `"x"` field is passed as input the `expr`, which
  produces some JSON `output`. The output of the entire `split` action is a
  JSON object with two fields, `"x"` and `"y"`, where the field `"x"` holds
  the `output` value of `expr`, and the field `"y"` retains its value from
  the original input.

- `if (expr) { expr } else { expr }: json -> expr -> expr -> json`

  `if` statements take arbitrary JSON as input, and produce JSON as output.
  the input `json` is first passed to the guard-`expr`. If the output of the
  guard-`expr` is the JSON value `true`, the input `json` is passed to the
  then-`expr`. If the output of the guard-`expr` is the JSON value `false`,
  the input `json` is passed to the else-`expr`. If the output of the
  guard-`expr` is neither a `true` or `false` JSON value, the statement
  aborts execution. The output `json` of the entire `if` statement is the
  JSON value output by whichever branch is taken.

- `project transformer: json -> transformer -> json`

  `project` actions take JSON as input, run the `json_transformer` evaluator
  on this `json`, and produce a JSON value as output. The `transformer`
  expression is defined in the `json_transformer` DSL.