orientation horizontal

_: minecraft:stone
I: minecraft:glass_pane
P: minecraft:planks
V: minecraft:log[0]
H: minecraft:log[4]
D: minecraft:log[8]

layer front
  H H H
V P P P V
V P   P V
V P   P V
_ _ _ _ _
==

layer middle
  D D D
D P P P D
P       P
P       P
_ _ _ _ _
==

layer window
  D D D
D P P P D
I       I
P       P
_ _ _ _ _
==

repeat window
repeat middle

layer back
  H H H
V P P P V
V P P P V
V P P P V
_ _ _ _ _
==
