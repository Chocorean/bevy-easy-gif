#!/bin/bash
#
# usage: bin/wasm
#
# note that it needs an additional index.html file

EXAMPLES=$(ls examples | cut -d. -f1)
DEFAULT="basic"

if [ -n "$1" ]
then
    grep "$1" <(echo "$EXAMPLES") > /dev/null
    if [ $? -eq 0 ]
    then
        TARGET="$1"
    else
        echo "error: example '$1' not found" 2> /dev/null
        exit 1
    fi
else
    TARGET="$DEFAULT"
fi

echo "Running '$TARGET' example in browser..."

rm -rf ./out
cargo build --target wasm32-unknown-unknown --example "$TARGET";
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/debug/examples/"$TARGET".wasm;
cp assets/index.html out/
sed -i 's/'"$DEFAULT"'/'"$TARGET"'/g' out/index.html
ln -s ../assets ./out/assets
python3 -m http.server -d ./out