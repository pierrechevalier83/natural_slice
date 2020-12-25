# Natural slice encoding

Encoding some properties of small slices of data as natural numbers.

These are the most compact representations of specific pieces of information about slices.

One use for this library is in encoding the state of a Rubiks Cube in a space efficient way to allow for fast Rubiks cube solving with move tables.

Credit goes to Herbert Kociemba for all the algorithms implemented here and the idea to use them for fast Rubik's cube manipulation.

## Permutation

Encode the permutation of elements of a slice as an integer between `0` and `n! - 1` where `n` is the number of elements in the slice.

See "corner permutation coordinate" and "edge permutation coordinate" [here](http://kociemba.org/math/coordlevel.htm) for a description of this encoding in the context of a Rubik's Cube.

## Property

Encode a property of elements of a slice which maps to a digit per element in a certain base as an integer between `0` and `base^(n - 1) - 1` where `n` is the number of elements in the slice.

See "corner orientation coordinate" and "edge orientation coordinate" [here](http://kociemba.org/math/coordlevel.htm) for a description of this encoding in the context of a Rubik's Cube.

## Position

Encode the position of "interesting" elements in a slice relative to other elements in the same slice. The relative order of "interesting" and "uninteresting" elements is ignored.

See "UD Slice coordinate" [here](http://kociemba.org/math/UDSliceCoord.htm) for a description of this encoding in the context of a Rubik's Cube.

