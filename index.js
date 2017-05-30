const fs = require('fs');
const mkdirp = require('mkdirp');
const md = require('markdown-it');
const hbs = require('handlebars');
const sass = require('node-sass');

const mdToHtml = md({breaks: true, typographer: true, xhtmlOut: true});

const limericks = fs.readFileSync('./limericks-20170503.md')
                      .toString('utf-8')
                      .split('\n---\n')
                      .map(text => ({content: mdToHtml.render(text.trim())}));

console.log(`Read ${limericks.length} limericks.`);

const data = {
  title: 'Rust Limericks',
  author: 'Andre \'lloqig\' Bogus',
  limericks,
};

const template =
    hbs.compile(fs.readFileSync('./template.hbs').toString('utf-8'));
const htmlContent = template(data);

mkdirp.sync('dist');
fs.writeFileSync('./dist/index.html', htmlContent);
console.log(`Wrote HTML file.`);

fs.writeFileSync(
    './dist/style.css', sass.renderSync({file: './style.scss'}).css);
console.log(`Wrote SCSS file.`);

// No emojis

// const htmlToPdf = require('wkhtmltopdf');
// htmlToPdf(htmlContent, {output: './dist/index.pdf', pageSize: 'A5'}, () => {
//   console.log(`Rendered PDF file.`);
// });



// Creates empty pdf file

// const execFileSync = require('child_process').execFileSync;

// function htmlToPdf(url) {
//   // Assuming MacOSx.
//   const CHROME =
//       '/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome';
//   execFileSync(CHROME, ['--headless', '--disable-gpu', '--print-to-pdf', url]);
// }

// const Server = require('static-server');
// const server = new Server({rootPath: './dist', host: '127.0.0.1', port: 32323});
// server.start(() => {
//   htmlToPdf(`https://localhost:${server.port}`)
//   console.log(`Rendered PDF file.`);
// });