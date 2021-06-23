MYPS Lexer
==========

## Usage

```
cargo r <file.mpys>
```

## Notes and todo

* Fix `functions` to be a vector instead of a hash (to fix undefined aliases in later defined
    function, deterministically)
* Come up with a different way for the Lexer to construct the items and blocks. It's real messy at
  the moment, which deconstructing and reconstructing blocks.
* Refactor lexer into a struct. Need to add called function names to the list;
    too many mutable references floating around in the lexer function.
* Tighten up error variant names (i.e. UndefinedAlias versus FuncUndefined)
* Add back in MYPS functions `pow`, `ln` and `log` and have these expand to MIPS `exp` and `log`
    composite expressions.
* Re-implement functions
* Add function definition checking at the end of the lexer. For each call, add the function name
    to a set, then at the end of lexing check if every name in the set was defined.
* Consider undo-ing the passing down of comments to the translator.
* Add function definitions and calls to grammar
    * `yield()`, `hcf()`, `sleep(n)`
    * Starting consider how to do something like:
        ```
        def dump(pump, analyzer):
            while analyzer.TotalMoles > 0:
                pump.On = True
                yield()
            pump.On = False
        ```

        Like `UnitArg`?
