/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-expect-error
import markwondItContainer from 'markdown-it-container'
import type { RenderRule } from 'markdown-it/lib/renderer.mjs'
import type MarkdownIt from 'markdown-it'

type ContainerArgs = [typeof markwondItContainer, string, { render: RenderRule }]

function createContainer(klass: string, defaultTitle: string, md: MarkdownIt): ContainerArgs {
    return [
        markwondItContainer,
        klass,
        {
            render(tokens: any, idx: any, _options, env: { references?: any }) {
                const token = tokens[idx]
                const info = token.info.trim().slice(klass.length).trim()
                const attrs = md.renderer.renderAttrs(token)
                if (token.nesting === 1) {
                    const title = md.renderInline(info || defaultTitle, {
                        references: env.references,
                    })
                    if (klass === 'details')
                        return `<details class="${klass} custom-block"${attrs}><summary>${title}</summary>\n`
                    return `<div class="${klass} custom-block"${attrs}><p class="custom-block-title">${title}</p>\n`
                } else return klass === 'details' ? `</details>\n` : `</div>\n`
            },
        },
    ]
}

const containerPlugin = (md: MarkdownIt) => {
    md.use(...createContainer('tip', 'TIP', md))
        .use(...createContainer('info', 'INFO', md))
        .use(...createContainer('warning', 'WARNING', md))
        .use(...createContainer('danger', 'DANGER', md))
        .use(...createContainer('details', 'Details', md))
}

export default containerPlugin
export { containerPlugin }
