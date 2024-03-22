const path = require('path')

/** @typedef {import('tailwindcss/types/config').ResolvableTo<import('tailwindcss/types/config').KeyValuePair>} ThemeExtension*/

/**@type {ThemeExtension} */
const spacing = {}
/**@type {ThemeExtension} */
const fontSize = {}

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

/** @type {import('tailwindcss').Config}*/
const config = {
  content: [anyHtmlRustOrCssFileInTheWorkspace],

  theme: {
    extend: {
      fontSize,
      spacing,
      colors: {
        cerulean: 'rgba(169, 232, 252, 1)', // Sky blue.
        cloud: 'rgba(245, 245, 245, 0)', // Fully transparent white.
        twilight: 'rgba(96,76,126,1)' // A dusky purple.
        // midnight: 'rgba(38,38,38,0)' // Fully transparent black.
      }
    }
  },

  plugins: []
}

module.exports = config
