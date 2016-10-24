const persist = require('choo-persist')
const bulk = require('bulk-require')
const mount = require('choo/mount')
const html = require('choo/html')
const log = require('choo-log')
const sf = require('sheetify')
const choo = require('choo')

// css
sf('tachyons')

// init
const opts = {}
const app = choo()
app.router(['/', mainView])

// logic
const logic = bulk(__dirname, [ 'models/*', 'actions/*' ])
Object.keys(logic).forEach((key) => {
  Object.keys(logic[key]).forEach((key) => {
    app.model(logic.actions[key](opts))
  })
})

// start
if (process.env.NODE_ENV === 'development') {
  app.use(log())
  mount('body', app.start())
} else {
  persist((persist) => {
    app.use(persist)
    mount('body', app.start())
  })
}

function mainView (state, prev, send) {
  return html`
    <body>
      <main>
        <h1 class="f-6">choo choo</h1>
      </main>
    </body>
  `
}
