import { readFile } from 'fs'
import { promisify } from 'util'
import fm from 'front-matter'
import marked from 'marked'

export async function get(req, res, next) {
    let pageSource
    try {
        console.log(process.cwd())
        pageSource = await promisify(readFile)('_pages/portfolio.md', 'utf-8')
    } catch (e) {
        res.statusCode = 500
        res.end('Error loading portfolio source file. \n' + e.toString())
        return
    }
    
    const parsed = fm(pageSource)
    console.log(parsed)
    const response = {
        title: parsed.attributes.title,
        content: marked(parsed.attributes.content),
    }

    res.setHeader('Content-Type', 'application/json')
    res.end(JSON.stringify(response))
}