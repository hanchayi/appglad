import { remarkMermaid } from 'remark-mermaid-nextra';
import nextra from 'nextra'

const withNextra = nextra({
  theme: 'nextra-theme-blog',
  latex: true,
  themeConfig: './theme.config.jsx',
  mdxOptions: {
    remarkPlugins: [remarkMermaid],
}
})

export default withNextra()
