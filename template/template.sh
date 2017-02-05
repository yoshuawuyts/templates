__dirname=$(dirname "$(readlink -f "$0")")
. "$(readlink -f "$__dirname/../shared.sh")"

# define PROJECTNAME
if [ -z "$1" ]; then
  printf 'What is the name of the project?\n❯ '
  read -r PROJECTNAME
  if [ "$PROJECTNAME" = "" ]; then
    printf 'no name provided, exiting\n'
    exit 1
  fi
else
  PROJECTNAME="$1"
fi
