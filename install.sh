#!/bin/sh

echo ""
cargo build --release

echo ""
CHOICE=$(gum choose --header="What os are you on ?" "linux" "mac" "windows")

if [ $CHOICE = "linux" ]; then
    gum spin -s dot --title '* 🐧 Installing for linux' sleep 1
    sudo cp target/release/kettle /usr/bin
    sudo cp kettlecli /usr/bin

elif [ $CHOICE = "mac" ]; then
    gum spin -s dot --title '* 🍎 Installing for mac' sleep 1
    sudo cp target/release/kettle /usr/local/bin
    sudo cp kettlecli /usr/local/bin
fi

echo " * 🫖 Kettle was successfully installed !"