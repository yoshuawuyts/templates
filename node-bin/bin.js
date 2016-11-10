#!/usr/bin/env node

const minimist = require('minimist')

const argv = minimist(process.argv.slice(2), {
  boolean: [
    'version',
    'help'
  ]
})

const usage = `
  Usage:
    $

  Commands:
    <default>

  Options:
    -h, --help      Print usage
    -v, --version   Print version

  Examples:
`

;(function main (argv) {
  if (argv.h) {
    return console.info(usage)
  } else if (argv.v) {
    return console.info('v' + require('./package.json').version)
  }
})(argv)
