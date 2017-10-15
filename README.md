# templates
Template files used to generate things. Uses the `{{varname}}` syntax for
interpolation. Each project has a `manifest.json` file the metadata for that
template.

## Usage
This templates directory is meant to be sourced through a shell script. It's
a neat little way of multiplexing arbitrary initialization commands. Here's a
basic version you can place in your `/usr/bin` directory (or equivalent):
```sh
#!/bin/sh

if [ ! -z "$TEMPLATES" ]; then
  printf 'bin/ew: TEMPLATES variable not set\n'
  exit 1
fi

if [ $# = "0" ]; then
  printf 'usage: ew <template_name>\n'
  exit 1
fi

template="$TEMPLATES/$1/main"
if [ ! -x "$template" ]; then
  printf 'bin/ew: template %s does not exist\n' "$1"
  exit 1
fi

shift
"$template" "$@"
```
You can now run commands using `ew`, for example:
```sh
$ ew shell my_cool_shell_script
```

## See Also
- [yoshuawuyts/dotfiles](https://github.com/yoshuawuyts/dotfiles)
- [yoshuawuyts/infrastructure](https://github.com/yoshuawuyts/infrastructure)
- [yoshuawuyts/knowledge](https://github.com/yoshuawuyts/knowledge)
- [yoshuawuyts/writing](https://github.com/yoshuawuyts/writing)

## License
[Apache-2.0](https://tldrlegal.com/license/mit-license)
