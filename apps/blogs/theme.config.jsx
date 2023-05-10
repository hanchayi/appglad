export default {
  footer: <a href="https://beian.miit.gov.cn">苏ICP备2022043623号</a>,
  head: ({ title, meta }) => (
    <>
      {meta.description && (
        <meta name="description" content={meta.description} />
      )}
      {meta.tag && <meta name="keywords" content={meta.tag} />}
      {meta.author && <meta name="author" content={meta.author} />}
    </>
  ),
  readMore: 'Read More →',
  postFooter: null,
  darkMode: false,
  navs: [
    {
      url: 'https://github.com/shuding/nextra',
      name: 'Nextra'
    },
    {
      url: 'https://github.com/shuding/nextra',
      name: 'Nextra'
    }
  ]
}
