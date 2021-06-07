MYPS Lexer
==========

## Notes and todo

* Move ALL parser AST stuff into the lexer. The lexer will be responsible for constructing the AST
    while at the same time validating the pairs.
* Probably should move all mathematic reductions from the parser to the lexer,
    where constant replacements can be done at the same time.
* Essentially all branches need to check whether or not a return line is necessary
    (that is, to distinguish between `j/jr` and `jal`)
* Branching statements need to cover:
    * `bdns` - Branch if device not set
    * `bdnsal` - Branch if 
    * `bdse`
    * `bdseal`
    * `brdns`
    * `brdse`
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
