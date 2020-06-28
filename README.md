# mp4\_steganography

Hide your lovely message in a MP4 file without corruption.

```
$ cargo run input.mp4 output.mp4 "What you want to say"
$ od -tx1z -Ax -v output.mp4 | tail
005590 6d 65 74 61 00 00 00 00 00 00 00 21 68 64 6c 72  >meta.......!hdlr<
0055a0 00 00 00 00 00 00 00 00 6d 64 69 72 61 70 70 6c  >........mdirappl<
0055b0 00 00 00 00 00 00 00 00 00 00 00 00 2d 69 6c 73  >............-ils<
0055c0 74 00 00 00 25 a9 74 6f 6f 00 00 00 1d 64 61 74  >t...%.too....dat<
0055d0 61 00 00 00 01 00 00 00 00 4c 61 76 66 35 38 2e  >a........Lavf58.<
0055e0 32 30 2e 31 30 30 00 00 00 2c 75 75 69 64 43 62  >20.100...,uuidCb<
0055f0 15 4c ac 63 43 cc 83 76 e3 00 e8 72 31 19 57 68  >.L.cC..v...r1.Wh<
005600 61 74 20 79 6f 75 20 77 61 6e 74 20 74 6f 20 73  >at you want to s<
005610 61 79                                            >ay<
005612
```

The message will be stored in a MP4's UUID box, that is, user extension area.
