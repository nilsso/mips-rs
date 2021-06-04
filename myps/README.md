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
