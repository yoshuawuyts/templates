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
  echo "[copy] $tmpfile $outfile"
  cat "$tmpfile" > "$outfile"
}

to_camel_case () {
  sed 's/[_|-]\([a-z]\)/\U\1/g;s/^\([a-z]\)/\1/g'
}
