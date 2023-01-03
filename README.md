We're sending a rover to Mars and we have to control it from the earth.

The rover can be deployed from the rocket that took it there at a specific coordinate (x, y) and facing one direction (N, S, E, W).



1 - Write a rover implementation that receives the coordinates and direction at start and reports those after landing. ie "Landed at (1, 4) EAST"



2 - Allow the rover to move around and report its location, to do that send a sequence of commands (F -> forward, B -> backward, L -> turn left, R -> turn right) to the rover. ie "FFFLFFRBRF".
If the rover started at position (0, 0) NORTH then it would report after consuming the sequence of commands (-1, 2) EAST
You don't have to write a command line interpreter for this, you can use tests to drive the implementation.

