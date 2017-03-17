var expose = require('choo-expose')
var html = require('choo/html')
var log = require('choo-log')
var css = require('sheetify')
var choo = require('choo')

css('tachyons')

var app = choo()
app.use(log())
app.use(expose())
app.route('/', mainView)
app.mount('body')

function mainView (state, prev, send) {
  return html`
    <body>
      <header></header>
      <main>
        <h1 class="f-6">
          choo choo
        </h1>
      </main>
      <footer></footer>
    </body>
  `
}
