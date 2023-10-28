Circe: Prototyping for 2D CAD drawn with Iced canvas

Soliciting experienced help and/or advice with architecture and gui

### Preview:
Dcop simulation\
![dcop](https://github.com/ua-kxie/circe/assets/56177821/299fc8ec-ba04-4618-b9f5-94dac25142cb)

Ac simulation\
![ac](https://github.com/ua-kxie/circe/assets/56177821/2488e5b8-d226-4820-91ae-603ab274efda)

Simple op-amp with generic devices\
![simple opa](https://github.com/ua-kxie/circe/assets/56177821/c196f999-93ed-4a55-af42-5cdd73d2c706)

### Setup:
Clone the repo, followed by `cargo run`.

To run the binary executable on windows, place a copy of `ngspice.dll` in the directory root (next to `circe.exe`).

### Controls: 
* left click wires or device to select  
* mouse wheel to zoom and pan  
* F key to fit viewport to geometry
* right click drag to zoom to area  
* left click drag to select area
* left click drag on selected device to drag selected
* select single device to edit parameter
  
#### Hotkeys:

##### Schematic Controls (circuit schematic and symbol designer):

Ctrl-C/left-click - copy/paste

Shift-C - cycle tentative selection

Del - delete selected

M - move selected

X/Y - flip horizontal/vertical during move
##### Circuit Schematic:

Shift-L - net label (has no effect on net connections atm, is just a comment)

W - draw wire

R - resistor (during move/copy, rotates selected, ctrl-R to counter rotate)

L - inductor

C - capacitor

G - ground

V - voltage source

N - nmos device

P - pmos device

Space - run dc op simulation

Ctrl-space - run ac simulation

Shift-T - run transient simulation

##### Symbol Designer
-for now, intended for dev use only-

W - draw a line

A - draw an arc/circle

P - place port

B - define device boundary

##### Plot/chart view
(shift) X - horizontal zoom

(shift) Y - vertical zoom 

### Goals
Target application is EDA schematic capture

#### Currently Working On:
* improved wiring pathfinding (petgraph)
* device/wire drag/grab keeping net connections
* connect overlapping ports with wiring
* net labels

### Contribute:
Consider using `cargo fmt` & `cargo fix`.

Looking for experienced help with architecture and GUI.
