dirname=$(dirname "$(readlink -f "$0")")

readf () {
  file="$(readlink -f "$dirname/$1")"
  cat "$file"
}

replace () {
  sed "s/{{$1}}/$2/g"
}

copy () {
  infile="$1"
  outfile="$2"
  tmpfile="$(readlink -f "$dirname/$infile")"
  cat "$tmpfile" > "$outfile"
}

if [ $# = "0" ]; then
  printf 'usage: ew NAME\n'
  exit 1
fi
