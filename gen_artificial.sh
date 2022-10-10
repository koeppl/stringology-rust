#!/bin/zsh
function die {
	echo "$1" >&2
	exit 1
}

[[ $# -eq 1 ]] || die "Usage: $0 [output-folder]"
folder="$1"
[[ -d "$folder" ]] || die "Folder $folder not accessible!"

for i in $(seq -f "%02.f" 0 20); do 
	cargo run --bin fibonacci --  "$i" >! "$folder/fibonacci.$i"
	cargo run --bin paperfolding -- -n "$i" >! "$folder/paperfold.$i"
	cargo run --bin paperfolding -q -- -n "$i" >! "$folder/paperfold.q.$i"
	cargo run --bin thuemorse -- "$i" >! "$folder/thuemorse.$i"
	cargo run --bin perioddoubling --  "$i" >! "$folder/perioddoubling.$i"
done
