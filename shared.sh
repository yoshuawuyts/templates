readf () {
  file="$(readlink -f "$1")"
  cat "$file"
}

replace () {
  sed "s/{{$1}}/$2/g"
}

copy () {
  infile="$1"
  outfile="$2"
  tmpfile="$(readlink -f "$infile")"
  cat "$tmpfile" > "$outfile"
}
