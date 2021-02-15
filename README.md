# Getting Started

Make sure:

1.  [fastann-wasm](https://github.com/fastann/fastann-wasm) and [fastann](https://github.com/fastann/fastann) are under the same parent folder
2.  fastann is already built.

Then open shell, run the commands below:

```shell
cargo install wasm-pack

cd $THE_FOLDER_OF_FASTANN_WASM
wasm-pack build --target web

python3 -m http.server # Start a local server. Here is an example for Python 3. More solutions in https://gist.github.com/willurd/5720255
```

Open the server website, click `sample.html`.
