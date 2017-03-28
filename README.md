# Chess library
This is a library that handles all the logic behind a chess game, written in Rust.
The full documentation can be found [here](https://kalkins.github.io/rust_chess/chess/)
An implementation of this library in an actual chess game can be found
[here](https://github.com/kalkins/rust_chess_cli).

## Automatic documentation
To automatically update the documentation in the docs/ folder to show it on Github Pages,
make a file `.git/hooks/post-commit` in the project folder and put the following content in it:

```sh
#!/bin/sh

cargo doc
cp -r target/doc/* docs/
git add docs
git commit -m "Updated docs."

exit 1
```

This will automatically generate and copy the documentation after each commit, as a seperate commit.
When the branch is merged with master and uploaded to Github, the documentation will be available on
Github Pages, if you've set it up for the project. Note that you have to put /chess/ at the end of the
Github Pages URL because of how rust docs are structured.
