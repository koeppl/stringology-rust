#!/bin/zsh
function die {
	echo "$1" >&2
	exit 1
}

[[ $# -eq 1 ]] || die "Usage: $0 [output-folder]"
folder="$1"
mkdir -p "$folder"
[[ -d "$folder" ]] || die "Folder $folder not accessible!"

set -x
set -e
for k in $(seq -f "%02.f" 0 20); do 
	for name in tribonacci vtm fibonacci kolakoski thue-morse period-doubling paper-folding quaternary-paper-folding binary-de-brujin power2; do
		cargo run --bin word -- -n "$name" -k "$k" > "$folder/$name.$k"
	done
done
# for k in $(seq -f "%02.f" 26 40); do 
# 	for name in fibonacci; do
# 		cargo run --bin word -- -n "$name" -k "$k" > "$folder/$name.$k"
# 	done
# done
