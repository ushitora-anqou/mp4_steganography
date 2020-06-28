# mp4\_steganography

Hide your lovely message in a MP4 file without corruption.

```
$ cargo run input.mp4 output.mp4 "What you want to say"
```

The message will be stored in a MP4's UUID box, that is, user extension area.
