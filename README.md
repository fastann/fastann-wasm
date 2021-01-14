# Getting Started

 Make sure [fastann-wasm](https://github.com/fastann/fastann-wasm) and [fastann](https://github.com/fastann/fastann) are under the same parent folder.

Then open shell, run the commands below:
```shell
cargo install wasm-pack

cd $THE_FOLDER_OF_FASTANN_WASM
wasm-pack build --target web

python -m SimpleHTTPServer # for Python 2
python3 -m http.server # for Python 3
```

Open the server website, click `sample.html`.
