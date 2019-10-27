# Rustbik

Rustbik is Rubik's cube implementation in Rust, using no reference materials (other than the rust language book).
The cube is manipulated from a single perspective, for brevity of implementation, and features six total moves;
- Front Clockwise Turn
- Left Counterclockwise Turn
- Right Clockwise Turn
- Top Clockwise Turn
- Bottom Clockwise Turn
- Back Clockwise Turn

From these 6 moves, all movements can theoretically be accomplished (I think?). This project was inspired by Dan and 
Hardik at my job, and I'm very thankful for the seed in my brain.

You can download and compile source on Windows 64 bit with the MSVC toolchain, or Unix with GCC. I don't know if anything
else works.

The project was built using CLion 2019.2.4

## TODO
The feature list thus far;
- [X] Implement all 6 moves
  - [X] Implement tests and benchmarks for all 6 moves
- [X] Implement equality checker for cube
- [X] Implement visual representation of cube in console
- [X] Implement cube shuffler
- [ ] Vectorize and use AVX for cube movements
- [ ] Move out of console project and into library
- [ ] Expose cube to command line to allow for solving by hand.
- [ ] **Programmatically solve the cube**

___

# Solution Log #1
Shuffling random moves didn't make it happen in any reasonable amount of time. It turns out that, given the insane
total permutations of a rubiks cube (something like 43 quintillion), it wasn't good enough to solve over an 8 hour period.

I need to turn to something smarter.... 
