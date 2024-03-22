const path = require('path')

// We were running into some funny business with relative paths which is why we use
// __dirname here.
const anyHtmlRustOrCssFileInTheWorkspace = path.resolve(
	__dirname,
	'..',
	'..',
	'**',
	'*.{html,rs,css}'
)
console.log(`Using ${anyHtmlRustOrCssFileInTheWorkspace} as the content path.`)

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [anyHtmlRustOrCssFileInTheWorkspace],
	theme: {
		extend: {
			fontFamily: {
				fugi: ['Fugi']
			}
		}
	},
	plugins: []
}
