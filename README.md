# 304 PACMAN

The goal of this project is to implement a parsing algorithm to help a ghost to reach pacman in the eponimous game.

## How to build
```sh
$ make re
```

## Examples
```sh
$ ./304pacman file c1 c2
file file describing the board, using the following characters:
                ‘0’ for an empty square,
                ‘1’ for a wall,
                ‘F’ for the ghost’s position,
                ‘P’ for Pacman’s position.
    c1      character to display for a wall
    c2      character to display for an empty space.
```
```sh
$ cat map1
1111111111
0000000000
0000000000
0000000000
00000F0000
0000000000
00000P0000
0000000000
0000000000
1111111111
```
```sh
$ ./304pacman map1 ‘+’ ‘ ’
++++++++++
2 212
1F12 12
P
++++++++++
```
```sh
cat map2
111111111111111
100000010000001
101011010110101
100P00010000001
101010111010101
101010010000001
101011010110111
111010000010111
100000000010F01
101010000010111
111010111110111
100000110000001
101110110101101
100100010001001
110001110101011
100100010000001
101110110111101
101010000000001
111111111111111
```
```sh
./304pacman map2 ‘@’ ‘ ’
@@@@@@@@@@@@@@@
@ @109890@
@ @ @@8@0@@7@9@
@  P767@987678@
@ @ @5@@@7@5@7@
@ @8@43@765456@
@ @7@@2@8@@3@@@
@@@6@21090@2@@@
@765432101@1F1@
@8@6@43212@2@@@
@@@7@5@@@@@3@@@
@  876@@765456@
@ @@@7@@8@6@@7@
@  @ 8 @987@98@
@@   @@@0@8@0@@
@  @   @109012@
@ @@@7@@2@@@@3@
@ @ @654345654@
@@@@@@@@@@@@@@@
```