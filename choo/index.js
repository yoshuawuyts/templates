const html = require('choo/html')
const sf = require('sheetify')
const choo = require('choo')

sf('tachyons')

const app = choo()
app.router(['/', mainView])

const tree = app.start()
document.body.appendChild(tree)

function mainView (state, prev, send) {
  return html`<main>hello world</main>`
}
