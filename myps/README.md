MYPS Lexer
==========

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

## Specifications?

For example, a simple solar panel control program:
```
alias sensor d0
alias x r0
define SolarPanelHash -2045627372
define H0 0 # some multiple of 90
start:
yield
horizontal:
l x sensor Horizontal
sub x H0 x
sb SolarPanelHash Horizontal x
vertical:
l x sensor Vertical
sub x 75 x
div x x 1.5
sb SolarPanelHash Vertical x
j start
```
Which in Myps we might write as:
```
sensor = d0
H0     = 0
panels = -2045627372.all

loop:
    panels.Horizontal = H0 - sensor.Horizontal
    panels.Vertical = (sensor.Vertical - 75) / 1.5
```
Which compiles to:
```
l r0 d0 Horizontal
sub r0 0 r0
sb -2045627372 Horizontal r0
l r0 d0 Vertical
sub r0 75 r0
div r0 r0 1.5
sb -2045627372 Vertical r0
j 0
```
