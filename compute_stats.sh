#!/bin/zsh
function die {
	echo "$1" >&2
	exit 1
}

[[ $# -eq 1 ]] || die "Usage: $0 filename"
kFilename="$1"
[[ -r "$kFilename" ]] || die "cannot read $kFilename"

result_line=$(cargo run --bin=count_sigma -- --file "$kFilename" | grep '^RESULT')

echo "dataset & \$n\$ & \$\\sigma\$ & \$z\$ & \$r\$ & \$H_0\$ & \$H_1\$ & \$H_2\$ & \$H_3\$ & \$H_4\$ \\\\\\"

echo -n $(basename "$kFilename")
echo -n " & "
echo -n $result_line | sed 's@.* length=\([0-9]\+\) .*@\1@'
echo -n " & "
echo -n $result_line | sed 's@.* sigma=\([0-9]\+\) .*@\1@'

bwtruns=$(cargo run --bin=count_r -- -d --file "$kFilename" | grep '^RESULT' | sed 's@.* bwt_runs=\([0-9]\+\) .*@\1@')
echo -n " & $bwtruns " 
lz77factors=$(cargo run --bin=count_z -- --file "$kFilename" | grep '^RESULT' | sed 's@.* factors=\([0-9]\+\).*@\1@')
echo -n " & $lz77factors " 

for k in $(seq 0 4); do
	entropy=$(cargo run --bin=entropy -- --order "$k" --file "$kFilename" | grep '^RESULT' | sed 's@.* entropy=\([0-9\.]\+\) .*@\1@')
	echo -n " & $entropy "
done

echo "\\\\\\"

# echo -n "$length $sigma $bwtruns $lz77factors"
#
# RESULT algo=bwt time_ms=0 length=512 bwt_runs=16 file=/scratch/data/perioddoubling10.txt use_dollar=false
#
#
# count_sigma
# count_z
# entropy
