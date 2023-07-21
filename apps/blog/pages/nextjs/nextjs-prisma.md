---
title: 如何使用 Next.js、Prisma 和 Vercel Postgres 构建全栈应用程序
description:
---

[Prisma](https://prisma.io/)是下一代 ORM，可用于访问 Node.js 和 TypeScript 应用程序中的数据库。
在本指南中，您将了解如何使用以下技术实现示例全栈博客应用程序：

- [Next.js](https://nextjs.org/)作为 React 框架
- [Next.js API](https://nextjs.org/docs/pages/building-your-application/routing/api-routes) 路由用于作为后端的服务器端 API 路由
- [Prisma](https://prisma.io/)作为迁移和数据库访问的 ORM
- [Vercel Postgres](https://vercel.com/storage/postgres)作为数据库
- Next Auth实现 GitHub (OAuth) 进行身份验证
- Typescript作为编程语言
- Vercel作为部署

## 先决条件

要成功完成本指南，您需要：

- Node.js
- Vercel 帐户（用于设置免费的 Postgres 数据库并部署应用程序）
- GitHub 帐户（用于创建 OAuth 应用程序）

## 第 1 步：设置您的 Next.js 入门项目

到您选择的目录并在终端中运行以下命令，以使用页面路由器设置新的 Next.js 项目：

``` bash
npx create-next-app --example https://github.com/prisma/blogr-nextjs-prisma/tree/main blogr-nextjs-prisma
```

您现在可以进入目录并启动应用程序：

``` bash
cd blogr-nextjs-prisma && npm run dev
```

目前的情况是这样的：
![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F2urWoZq7yHkEhi93PPJXD4%2Fec4c90b45389261b215a499f24caaeb4%2F1.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

`getStaticProps` 该应用程序当前显示从文件中 返回的硬编码数据 `index.tsx` 。在接下来的几节中，您将更改此设置，以便从实际数据库返回数据。


## 第 2 步：设置 Vercel Postgres 数据库

出于本指南的目的，我们将使用 Vercel 上托管的免费 Postgres 数据库。首先，将步骤 1 中克隆的存储库推送到我们自己的 GitHub 并将其部署到 Vercel 以创建 Vercel 项目。

拥有 Vercel 项目后，选择 “存储” 选项卡，然后选择 “连接数据库” 按钮。在 “新建”选项 卡下，选择 “Postgres”  ，然后 选择“继续” 按钮。

要创建新数据库，请在打开的对话框中执行以下操作：

1. 在Store Name下输入sample_postgres_db（或您想要的任何其他名称）。名称只能包含字母数字字母、“_”和“-”，且不能超过 32 个字符。
2. 选择一个区域。我们建议选择地理位置靠近您的功能区域的区域（默认为美国东部）以减少延迟。
3. 单击创建。

我们的空数据库是在指定的区域中创建的。由于您在项目中创建了 Postgres 数据库，因此我们自动为您创建了以下环境变量并将其添加到项目中。

运行`npm i -g vercel@latest`安装 Vercel CLI 后，下拉最新的环境变量以使本地项目能够使用 `Postgres` 数据库。

``` bash
vercel link
vercel env pull .env
```
我们现在拥有一个功能齐全的 Vercel Postgres 数据库，并拥有在本地和 Vercel 上运行它的所有环境变量。

## 第 3 步：设置 Prisma 并创建数据库架构

接下来，您将设置 Prisma 并将其连接到 PostgreSQL 数据库。首先通过 npm 安装 Prisma CLI：

``` bash
npm install prisma --save-dev
```
现在，您将使用 Prisma CLI 在数据库中创建表。

为此，请创建一个 prisma 文件夹并添加一个名schema.prisma,为主 Prisma 配置文件的文件，其中将包含您的数据库架构。

将以下模型定义添加到您的模型中， `schema.prisma` 使其看起来像这样：

``` prisma
// schema.prisma

generator client {
  provider = "prisma-client-js"
  previewFeatures = ["jsonProtocol"]
}

datasource db {
  provider = "postgresql"
  url = env("POSTGRES_PRISMA_URL") // uses connection pooling
  directUrl = env("POSTGRES_URL_NON_POOLING") // uses a direct connection
  shadowDatabaseUrl = env("POSTGRES_URL_NON_POOLING") // used for migrations
}

model Post {
  id        String     @default(cuid()) @id
  title     String
  content   String?
  published Boolean @default(false)
  author    User?   @relation(fields: [authorId], references: [id])
  authorId  String?
}

model User {
  id            String       @default(cuid()) @id
  name          String?
  email         String?   @unique
  createdAt     DateTime  @default(now()) @map(name: "created_at")
  updatedAt     DateTime  @updatedAt @map(name: "updated_at")
  posts         Post[]
  @@map(name: "users")
}
```

此 `Prisma Schema`定义了两个模型，每个模型都将映射到 底层数据库中的 一个表`User`和 `Post`。请注意，两个模型之间也存在关系（一对多），通过`Post`表字段 `author` 和 `User`表的`posts` 字段。

要在数据库中实际创建表，您现在可以使用 `Prisma CLI` 的以下命令：

``` bash
npx prisma db push
```

您应该看到以下输出：

``` bash
Environment variables loaded from .env
Prisma schema loaded from schema.prisma
Datasource "db": PostgreSQL database "verceldb", schema "public" at "ep-fragrant-moon-310453.us-east-1.postgres.vercel-storage.com"

🚀  Your database is now in sync with your Prisma schema. Done in 10.69s

✔ Generated Prisma Client (4.16.2 | library) to ./node_modules/@prisma/client in 57ms
```

恭喜，表已创建！继续使用 `Prisma Studio` 添加一些初始虚拟数据。运行以下命令：

``` bash
npx prisma studio
```

使用 `Prisma Studio` 的界面创建新 `User` 记录 `Post` 并通过关系字段连接它们。

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F4Xv0gxUZulUcEbkx44meID%2F35c49dbd5d8e10e3796e920e1270df7f%2F2.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F4gg9hkQjCaRfZuGPNrD7Ao%2Ff35ec469a65f168970d190f46dd20872%2F3.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

## 步骤4.安装并生成Prisma客户端

在使用 `Prisma` 从 `Next.js` 访问数据库之前，您首先需要在应用程序中安装 `Prisma` 客户端。您可以通过 `npm` 安装它，如下所示：

``` bash
npm install @prisma/client
```

由于 `Prisma Client`是根据您的schema定制的，因此每次 `Prisma Schema`文件发生更改时，您都需要通过运行以下命令来更新它：

``` bash
npx prisma generate
```
您将使用`prisma.ts`创建一个 `PrismaClient` 实例，可以将其导入到任何需要的文件中使用该实例。
将在 `lib/`目录中创建文件，继续创建缺少的目录和文件：
``` bash
mkdir lib && touch lib/prisma.ts
```

现在，将以下代码添加到该文件中：
```typescript
import { PrismaClient } from '@prisma/client';

let prisma: PrismaClient;

if (process.env.NODE_ENV === 'production') {
  prisma = new PrismaClient();
} else {
  if (!global.prisma) {
    global.prisma = new PrismaClient();
  }
  prisma = global.prisma;
}

export default prisma;
```

现在，每当您需要访问数据库时，您都可以将 prisma 实例导入到需要的文件中。

## 步骤 5. 更新现有视图以从数据库加载数据

`pages/index.tsx `中实现的博客文章提要和文章详细信息视图 `pages/p/[id].tsx` 当前正在返回硬编码数据。在此步骤中，您将调整实现以使用 Prisma 客户端从数据库返回数据。

打开 `pages/index.tsx` 并在现有声明的正下方添加以下代码 `import ：`


```typescript
import prisma from '../lib/prisma';
```

当您想要在数据库中读取和写入数据时，您的`prisma` 实例将成为数据库的接口。例如，您可以通过调用`prisma.user.create()`创建` User `新记录或使用`prisma.post.findMany()`检索数据库中`Post`的所有记录 。有关完整 `Prisma API` 客户端 的概述，请访问[Prisma](https://www.prisma.io/docs/concepts/components/prisma-client/crud) 文档。

现在您可以修改`index.tsx`通过对数据库的正确调用来替换`getStaticProps`内部硬编码的`feed`对象:

``` typescript
export const getStaticProps: GetStaticProps = async () => {
  const feed = await prisma.post.findMany({
    where: { published: true },
    include: {
      author: {
        select: { name: true },
      },
    },
  });
  return {
    props: { feed },
    revalidate: 10,
  };
};
```
`Prisma Client` 查询需要注意的两点：

- 指定过滤where器仅包含以下Post记录：`published:true`
- 记录name的也会被查询并将包含在返回的对象author中Post

在运行应用程序之前，请前往 `/pages/p/[id].tsx`并调整那里的实现，以 Post 从数据库中读取正确的记录。

此页面使用 `getServerSideProps` (SSR) 而不是 `getStaticProps` (SSG)。这是因为数据是 `动态的`，它取决于  `URL` 中请求的`id` 数据 。Post例如，路线视图 /p/42 显示  所在 Post 位置 。id42

和之前一样，首先需要在页面导入Prisma Client：
``` bash
import prisma from '../../lib/prisma';
```

现在您可以更新 `getServerSideProps`的实现, 以从数据库中检索正确的帖子，并通过组件将其提供给您的前端 props：

```typescript
export const getServerSideProps: GetServerSideProps = async ({ params }) => {
  const post = await prisma.post.findUnique({
    where: {
      id: String(params?.id),
    },
    include: {
      author: {
        select: { name: true },
      },
    },
  });
  return {
    props: post,
  };
};
```

就是这样！如果您的应用程序不再运行，您可以使用以下命令重新启动它：

```
npm run dev
```

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F11i7qnImXEGxOifqJgTbSJ%2Fc4a13562edb18c54a3c39b28898c8459%2F4.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)


## 步骤 6. 使用 NextAuth 设置 GitHub 身份验证

在此步骤中，您将向应用程序添加 GitHub 身份验证。该功能可用后，您将向应用程序添加更多功能，以便经过身份验证的用户可以通过 UI 创建、发布和删除帖子。

第一步，继续在您的应用程序中安装 NextAuth.js 库：

``` bash
npm install next-auth@4 @next-auth/prisma-adapter next@^12.2.5
```

接下来，您需要按`NextAuth`更改数据库Schema以添加缺少的表

要更改数据库`Schema`，您可以手动更改 `Prisma Schema`，然后 `npx prisma db push` 再次运行该命令。
打开 `schema.prisma` 并调整其中的模型，如下所示：

> 执行`npx prisma db push`时候，先清理下User和Post表的数据，然后后台`npx prisma studio` 要重新运行

``` prisma
// schema.prisma

model Post {
  id        String  @id @default(cuid())
  title     String
  content   String?
  published Boolean @default(false)
  author    User?@relation(fields:[authorId], references:[id])
  authorId  String?}

model Account {
  id                 String  @id @default(cuid())
  userId             String  @map("user_id")
  type               String
  provider           String
  providerAccountId  String  @map("provider_account_id")
  refresh_token      String?
  access_token       String?
  expires_at         Int?
  token_type         String?
  scope              String?
  id_token           String?
  session_state      String?
  oauth_token_secret String?
  oauth_token        String?

  user User @relation(fields:[userId], references:[id], onDelete: Cascade)

  @@unique([provider, providerAccountId])}

model Session {
  id           String   @id @default(cuid())
  sessionToken String   @unique@map("session_token")
  userId       String   @map("user_id")
  expires      DateTime
  user         User     @relation(fields:[userId], references:[id], onDelete: Cascade)}

model User {
  id            String    @id @default(cuid())
  name          String?
  email         String?@unique
  emailVerified DateTime?
  image         String?
  posts         Post[]
  accounts      Account[]
  sessions      Session[]}

model VerificationToken {
  id         Int      @id @default(autoincrement())
  identifier String
  token      String   @unique
  expires    DateTime

  @@unique([identifier, token])}
}
```

由于您使用的是 GitHub 身份验证，因此还需要创建一个新的 [GitHub 上的 OAuth 应用程序](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps)
- 首先，登录您的`GitHub`帐户。
- 然后，导航至`Settings`
- 然后打开`Developer Settings`
- 然后切换到`OAuth Apps`

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F42WGp7BsBEk2we6u3nx9O7%2F1bea8b695cf0e40081b7f2187244791b%2F5.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)


单击 `Register a new application`  （或 `New OAuth application`）按钮会将您重定向到注册表以填写应用程序的一些信息。`Authorization callback URL` 应为 Next.js `/api/auth/callback/github` 路由：  http://localhost:3000/api/auth/callback/github。

这里需要注意的重要一点是， 授权回调 URL 字段仅支持单个 URL，与 Auth0 不同，Auth0 允许您添加以逗号分隔的其他回调 URL。这意味着如果您想稍后使用生产 URL 部署应用程序，则需要设置新的 GitHub OAuth 应用程序。

点击 注册应用程序 按钮，然后您将能够找到新生成的 `Client ID` 和 `Client Secret`。将此信息复制并粘贴到 .env 根目录中的文件中，作为 `GITHUB_ID` 和 `GITHUB_SECRET` env var。还将其设置 为 与您在 GitHub 上配置的 授权回调 `URLNEXTAUTH_URL` 相同的值 ：http://localhost:3000/api/auth

您还需要在整个应用程序中保留用户的身份验证状态。快速更改应用程序的根文件 ，并使用`next-auth/react`包中的`SessionProvider` 包装当前的根组件  `_app.tsx`。打开该文件并将其当前内容替换为以下代码：

``` typescript
import { SessionProvider } from "next-auth/react"
export default function App({
  Component,
  pageProps: { session, ...pageProps },
}) {
  return (
    <SessionProvider session={session}>
      <Component {...pageProps} />
    </SessionProvider>
  )
}
```

## 步骤 7. 添加登录功能

登录按钮和其他一些 UI 组件将添加到该 `Header.tsx` 文件中。打开该文件并将以下代码粘贴到其中：

```typescript
import React from 'react';
import Link from 'next/link';
import { useRouter } from 'next/router';
import { signOut, useSession } from 'next-auth/react';

const Header: React.FC = () => {
  const router = useRouter();
  const isActive: (pathname: string) => boolean = (pathname) =>
    router.pathname === pathname;

  const { data: session, status } = useSession();

  let left = (
    <div className="left">
      <Link href="/">
        <a className="bold" data-active={isActive('/')}>
          Feed
        </a>
      </Link>
      <style jsx>{`
        .bold {
          font-weight: bold;
        }

        a {
          text-decoration: none;
          color: var(--geist-foreground);
          display: inline-block;
        }

        .left a[data-active='true'] {
          color: gray;
        }

        a + a {
          margin-left: 1rem;
        }
      `}</style>
    </div>
  );

  let right = null;

  if (status === 'loading') {
    left = (
      <div className="left">
        <Link href="/">
          <a className="bold" data-active={isActive('/')}>
            Feed
          </a>
        </Link>
        <style jsx>{`
          .bold {
            font-weight: bold;
          }

          a {
            text-decoration: none;
            color: var(--geist-foreground);
            display: inline-block;
          }

          .left a[data-active='true'] {
            color: gray;
          }

          a + a {
            margin-left: 1rem;
          }
        `}</style>
      </div>
    );
    right = (
      <div className="right">
        <p>Validating session ...</p>
        <style jsx>{`
          .right {
            margin-left: auto;
          }
        `}</style>
      </div>
    );
  }

  if (!session) {
    right = (
      <div className="right">
        <Link href="/api/auth/signin">
          <a data-active={isActive('/signup')}>Log in</a>
        </Link>
        <style jsx>{`
          a {
            text-decoration: none;
            color: var(--geist-foreground);
            display: inline-block;
          }

          a + a {
            margin-left: 1rem;
          }

          .right {
            margin-left: auto;
          }

          .right a {
            border: 1px solid var(--geist-foreground);
            padding: 0.5rem 1rem;
            border-radius: 3px;
          }
        `}</style>
      </div>
    );
  }

  if (session) {
    left = (
      <div className="left">
        <Link href="/">
          <a className="bold" data-active={isActive('/')}>
            Feed
          </a>
        </Link>
        <Link href="/drafts">
          <a data-active={isActive('/drafts')}>My drafts</a>
        </Link>
        <style jsx>{`
          .bold {
            font-weight: bold;
          }

          a {
            text-decoration: none;
            color: var(--geist-foreground);
            display: inline-block;
          }

          .left a[data-active='true'] {
            color: gray;
          }

          a + a {
            margin-left: 1rem;
          }
        `}</style>
      </div>
    );
    right = (
      <div className="right">
        <p>
          {session.user.name} ({session.user.email})
        </p>
        <Link href="/create">
          <button>
            <a>New post</a>
          </button>
        </Link>
        <button onClick={() => signOut()}>
          <a>Log out</a>
        </button>
        <style jsx>{`
          a {
            text-decoration: none;
            color: var(--geist-foreground);
            display: inline-block;
          }

          p {
            display: inline-block;
            font-size: 13px;
            padding-right: 1rem;
          }

          a + a {
            margin-left: 1rem;
          }

          .right {
            margin-left: auto;
          }

          .right a {
            border: 1px solid var(--geist-foreground);
            padding: 0.5rem 1rem;
            border-radius: 3px;
          }

          button {
            border: none;
          }
        `}</style>
      </div>
    );
  }

  return (
    <nav>
      {left}
      {right}
      <style jsx>{`
        nav {
          display: flex;
          padding: 2rem;
          align-items: center;
        }
      `}</style>
    </nav>
  );
};

export default Header;
```

以下是标题如何呈现的概述：

- 如果没有用户经过身份验证，将显示登录按钮。
- 如果用户通过身份验证，将显示我的草稿、新帖子和注销按钮。

您已经可以通过`npm run dev`运行该应用程序来验证其是否有效 ，您会发现  现在显示了“Log In”按钮。但是，如果您单击它，它确实会将您导航到， http://localhost:3000/api/auth/signin 但 Next.js 将为您呈现一个 404 页面。

那是因为 [NextAuth.js](https://next-auth.js.org/configuration/pages) 要求您设置特定的身份验证路由。接下来你会这样做


创建一个新目录并在该 `pages/api` 目录中创建一个新文件：

``` bash
mkdir -p pages/api/auth && touch pages/api/auth/[...nextauth].ts
```
> 如果touch失败请手动创建`[...nextauth].ts`

在这个新 pages/api/auth/[...nextauth].ts 文件中，您现在需要添加以下样板，以使用 GitHub OAuth 凭据和[Prisma 适配器](https://authjs.dev/reference/adapters#prisma-adapter)

```typescript
import { NextApiHandler } from 'next';
import NextAuth from 'next-auth';
import { PrismaAdapter } from '@next-auth/prisma-adapter';
import GitHubProvider from 'next-auth/providers/github';
import prisma from '../../../lib/prisma';

export const authOptions = {
    providers: [
      GitHubProvider({
        clientId: process.env.GITHUB_ID,
        clientSecret: process.env.GITHUB_SECRET,
      }),
    ],
    adapter: PrismaAdapter(prisma),
    secret: process.env.SECRET,
  };
const authHandler: NextApiHandler = (req, res) => {
    return NextAuth(req, res, authOptions);
}

export default authHandler;
```

添加代码后，您可以 `http://localhost:3000/api/auth/signin` 再次导航到。
这次，  将显示“使用 GitHub 登录”按钮。

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F4kQ9bV1IGBC43DwzvuT1p0%2F2725db72dd59860dc3ace3ee35331e43%2F7.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

如果单击它，您将被转发到 GitHub，您可以在其中使用 GitHub 凭据进行身份验证。身份验证完成后，您将被重定向回应用程序。

> 注意： 如果您看到错误且无法进行身份验证，请停止该应用程序并使用 重新运行它 `npm run dev`。 如果无法跳转并且链接协议变成https尝试将.env中的`VERCEL="1"`改成`VERCEL=""`

标题布局现已更改为显示经过身份验证的用户的按钮。

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F42h50fNeDIOpA6YEH5MVUg%2Fe6f7f16ddbe32520e24bb3dcc107f760%2F8.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

## 步骤 8. 添加新的帖子功能

在此步骤中，您将实现一种让用户创建新帖子的方法。用户  通过身份验证后，可以通过单击“新帖子”按钮来使用此功能。

该按钮已经转发到该 /create 路由，但是，这当前会导致 404，因为该路由尚未实现。

要解决此问题，请在页面目录中创建一个名为的新文件 create.tsx：

``` bash
touch pages/create.tsx
```

现在，将以下代码添加到新创建的文件中：

```typescript
import React, { useState } from 'react';
import Layout from '../components/Layout';
import Router from 'next/router';

const Draft: React.FC = () => {
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');

  const submitData = async (e: React.SyntheticEvent) => {
    e.preventDefault();
    // TODO
    // You will implement this next ...
  };

  return (
    <Layout>
      <div>
        <form onSubmit={submitData}>
          <h1>New Draft</h1>
          <input
            autoFocus
            onChange={(e) => setTitle(e.target.value)}
            placeholder="Title"
            type="text"
            value={title}
          />
          <textarea
            cols={50}
            onChange={(e) => setContent(e.target.value)}
            placeholder="Content"
            rows={8}
            value={content}
          />
          <input disabled={!content || !title} type="submit" value="Create" />
          <a className="back" href="#" onClick={() => Router.push('/')}>
            or Cancel
          </a>
        </form>
      </div>
      <style jsx>{`
        .page {
          background: var(--geist-background);
          padding: 3rem;
          display: flex;
          justify-content: center;
          align-items: center;
        }

        input[type='text'],
        textarea {
          width: 100%;
          padding: 0.5rem;
          margin: 0.5rem 0;
          border-radius: 0.25rem;
          border: 0.125rem solid rgba(0, 0, 0, 0.2);
        }

        input[type='submit'] {
          background: #ececec;
          border: 0;
          padding: 1rem 2rem;
        }

        .back {
          margin-left: 1rem;
        }
      `}</style>
    </Layout>
  );
};

export default Draft;
```

该页面由组件包装 `Layout` ，因此它仍然包含 `Header` 和任何其他通用 `UI` 组件。

它呈现一个带有多个输入字段的表单。提交后，（现在为空） `submitData` 函数被调用。在该函数中，您需要将数据从 `React` 组件传递到 `API` 路由，然后该路由可以处理数据库中新发布数据的实​​际存储。

以下是实现该功能的方法：

```typescript
const submitData = async (e: React.SyntheticEvent) => {
  e.preventDefault();
  try {
    const body = { title, content };
    await fetch('/api/post', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    await Router.push('/drafts');
  } catch (error) {
    console.error(error);
  }
};
```

在此代码中，您使用  从组件状态中提取的 `title`和 属性 ，并通过 `HTTP POST` 请求将它们提交到  `API` 路由。`content``useState``api/post`

然后，您将用户重定向到该 `/drafts` 页面，以便他们可以立即看到新创建的草稿。如果运行该应用程序，该 `/create` 路由将呈现以下 UI：

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F282p7LTOlQRXTV4CJzzfxa%2F15b5b92f4ebfb31d791ea5f651503dc0%2F9.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

但请注意，该实现尚未完全正常工作，因为  到目前为止，该路由api/post 和 路由都不存在。/drafts接下来您将实现这些。

首先，我们确保您的后端可以处理用户提交的 POST 请求。
创建一个名为 `post`的新目录  ，并添加一个名为`index.ts` 的新文件 ：

``` bash
mkdir -p pages/api/post && touch pages/api/post/index.ts
```

现在，将以下代码添加到 `pages/api/post/index.ts`：

```typescript
import { getServerSession } from 'next-auth/next';
import prisma from '../../../lib/prisma';
import { authOptions } from '../auth/[...nextauth]';

// POST /api/post
// Required fields in body: title
// Optional fields in body: content
export default async function handle(req, res) {
  const { title, content } = req.body;

  const session = await getServerSession(req, res, authOptions);
  const result = await prisma.post.create({
    data: {
      title: title,
      content: content,
      author: { connect: { email: session?.user?.email } },
    },
  });
  res.json(result);
}
```

此代码  为路由中传入的任何请求 实现处理程序/api/post/函数 。该实现执行以下操作：首先，它 从传入的 `HTTP POST` 请求的正文中提取`title` 和  `content`之后，它使用 `NextAuth.js` 的帮助函数检查请求是否来自经过身份验证的用户 `getServerSession` 。最后，它使用 Prisma 客户端在数据库中创建`Post`新记录。

您现在可以通过打开应用程序来测试此功能，确保您已通过身份验证并创建包含标题和内容的新帖子：

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F2Z41gYOddxIlo6KwqswDPd%2F99eca6c7deaf23d437c71c8646b7444a%2F10.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

单击 “创建”后，该 `Post` 记录将被添加到数据库中。请注意， `/drafts` 创建后立即重定向到的路由仍然会呈现 `404`，这将很快得到修复。但是，如果您使用`npx prisma studio`再次运行 `Prisma Studio`  ，您将看到新 `Post`记录已添加到数据库中。


## 步骤 9. 添加草稿功能

在此步骤中，您将向应用程序添加一个新页面，允许经过身份验证的用户查看其当前 `草稿`。

此页面无法静态呈现，因为它取决于经过身份验证的用户。像这样  根据经过身份验证的用户 动态getServerSideProps获取数据的页面是通过.

首先，在目录 `pages` 中创建一个新文件并命名为 `drafts.tsx`：

``` bash
touch pages/drafts.tsx
```

接下来，将以下代码添加到该文件中：

```typescript
import React from 'react';
import { GetServerSideProps } from 'next';
import { useSession } from 'next-auth/react';
import Layout from '../components/Layout';
import Post, { PostProps } from '../components/Post';
import prisma from '../lib/prisma';
import { getServerSession } from 'next-auth';
import { authOptions } from './api/auth/[...nextauth]';

export const getServerSideProps: GetServerSideProps = async ({ req, res }) => {
  const session = await getServerSession(req, res, authOptions);
  if (!session) {
    res.statusCode = 403;
    return { props: { drafts: [] } };
  }

  const drafts = await prisma.post.findMany({
    where: {
      author: { email: session.user.email },
      published: false,
    },
    include: {
      author: {
        select: { name: true },
      },
    },
  });
  return {
    props: { drafts },
  };
};

type Props = {
  drafts: PostProps[];
};

const Drafts: React.FC<Props> = (props) => {
  const { data: session } = useSession();

  if (!session) {
    return (
      <Layout>
        <h1>My Drafts</h1>
        <div>You need to be authenticated to view this page.</div>
      </Layout>
    );
  }

  return (
    <Layout>
      <div className="page">
        <h1>My Drafts</h1>
        <main>
          {props.drafts.map((post) => (
            <div key={post.id} className="post">
              <Post post={post} />
            </div>
          ))}
        </main>
      </div>
      <style jsx>{`
        .post {
          background: var(--geist-background);
          transition: box-shadow 0.1s ease-in;
        }

        .post:hover {
          box-shadow: 1px 1px 3px #aaa;
        }

        .post + .post {
          margin-top: 2rem;
        }
      `}</style>
    </Layout>
  );
};

export default Drafts;
```

在此 `React` 组件中，您将呈现经过身份验证的用户的“草稿”列表。草稿是在服务器端渲染期间从数据库中检索的，因为使用 `Prisma` 客户端的数据库查询是在 `getServerSideProps`。 然后，React 组件可以通过`props`访问。

如果您现在导航到 `应用程序的“我的草稿”` 部分，您将看到之前创建的未发布的帖子：

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F221Ve9cFpuAu9zBKZVjaWJ%2Fcd6fe2068b187c30220a8f868b767c35%2F11.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)


## 步骤 10. 添加发布功能

要将草稿“移动”到公共提要视图，您需要能够“发布”它，即将 `设置Post` 记录`published` `字段` 为 `true`。此功能将在当前位于的帖子详细信息视图中实现 pages/p/[id].tsx。

该功能将通过 `HTTP PUT` 请求来实现，该请求将发送到 api/publish “Next.js 后端”中的路由。继续并首先实施该路线。

在名为`pages/api`的目录中创建一个新目录 `publish.[id].ts` 然后在新目录中创建一个名为的新文件 ：

``` bash
mkdir -p pages/api/publish && touch pages/api/publish/[id].ts
```

现在，将以下代码添加到新创建的文件中：

``` bash
import prisma from '../../../lib/prisma';

// PUT /api/publish/:id
export default async function handle(req, res) {
  const postId = req.query.id;
  const post = await prisma.post.update({
    where: { id: postId },
    data: { published: true },
  });
  res.json(post);
}
```

这是 `API` 路由处理程序的实现，它从 `URL` 检索 `Post`的 `ID`   ，然后使用 `Prisma` 客户端的 `update`方法将 `Post表对应记录`  的`published`字段设置为`true`。

接下来，您将在文件中的前端实现该功能 `pages/p/[id].tsx` 。打开该文件并将其内容替换为以下内容：

``` typescript
import React from 'react';
import { GetServerSideProps } from 'next';
import ReactMarkdown from 'react-markdown';
import Router from 'next/router';
import Layout from '../../components/Layout';
import { PostProps } from '../../components/Post';
import { useSession } from 'next-auth/react';
import prisma from '../../lib/prisma';

export const getServerSideProps: GetServerSideProps = async ({ params }) => {
  const post = await prisma.post.findUnique({
    where: {
      id: String(params?.id),
    },
    include: {
      author: {
        select: { name: true, email: true },
      },
    },
  });
  return {
    props: post,
  };
};

async function publishPost(id: string): Promise<void> {
  await fetch(`/api/publish/${id}`, {
    method: 'PUT',
  });
  await Router.push('/');
}

const Post: React.FC<PostProps> = (props) => {
  const { data: session, status } = useSession();
  if (status === 'loading') {
    return <div>Authenticating ...</div>;
  }
  const userHasValidSession = Boolean(session);
  const postBelongsToUser = session?.user?.email === props.author?.email;
  let title = props.title;
  if (!props.published) {
    title = `${title} (Draft)`;
  }

  return (
    <Layout>
      <div>
        <h2>{title}</h2>
        <p>By {props?.author?.name || 'Unknown author'}</p>
        <ReactMarkdown children={props.content} />
        {!props.published && userHasValidSession && postBelongsToUser && (
          <button onClick={() => publishPost(props.id)}>Publish</button>
        )}
      </div>
      <style jsx>{`
        .page {
          background: var(--geist-background);
          padding: 2rem;
        }

        .actions {
          margin-top: 2rem;
        }

        button {
          background: #ececec;
          border: 0;
          border-radius: 0.125rem;
          padding: 1rem 2rem;
        }

        button + button {
          margin-left: 1rem;
        }
      `}</style>
    </Layout>
  );
};

export default Post;
```

此代码将 `publishPost` 函数添加到 `React` 组件，该组件负责将 `HTTP PUT` 请求发送到您刚刚实现的 `API` 路由。该 `render` 组件的功能也进行了调整，以检查用户是否经过身份验证，如果是这样，它也会在帖子详细信息视图中显示“发布”按钮：

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F1Ohdg5zNejFfXxwWAe6hpR%2F1d7a7059cb51bf3d6f29a66caafd884f%2F12.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

如果您单击该按钮，您将被重定向到公共源，并且该帖子将显示在那里！

> 注意： 应用程序部署到生产环境后，当收到请求时，提要最多每 10 秒更新一次。这是因为您正在使用静态站点生成 (SSG) via getStaticProps 来检索此视图的数据


## 步骤 11. 添加删除功能

您将在本指南中实现的最后一项功能是使用户能够删除现有 `Post` 记录。您将遵循与“发布”功能类似的方法，首先在后端实现 `API` 路由处理程序，然后调整您的前端以使用新路由！

在目录中创建一个新文件 `pages/api/post` 并命名为 `[id].ts`：

``` bash
touch pages/api/post/[id].ts
```

现在，添加以下代码：
``` typescript
import prisma from '../../../lib/prisma';

// DELETE /api/post/:id
export default async function handle(req, res) {
  const postId = req.query.id;
  if (req.method === 'DELETE') {
    const post = await prisma.post.delete({
      where: { id: postId },
    });
    res.json(post);
  } else {
    throw new Error(
      `The HTTP ${req.method} method is not supported at this route.`,
    );
  }
}
```

此代码处理 `DELETE` `/api/post/:id`的`HTTP`请求 。然后路由处理程序从 `URL` 中拿到`id`，检索记录并使用 `Prisma` 客户端根据`id`删除数据库`Post`表中的该记录 。

要在前端使用此功能，您再次需要调整帖子详细信息视图。打开 `pages/p/[id].tsx` 以下函数并将其插入到该 publishPost 函数的正下方：

``` typescript
async function deletePost(id: string): Promise<void> {
  await fetch(`/api/post/${id}`, {
    method: 'DELETE',
  });
  Router.push('/');
}
```
现在，您可以对“删除” 按钮采用与 “发布”按钮类似的方法  ，并且仅在用户经过身份验证时才呈现它。为此，您可以直接将此代码添加到 组件中 呈现“发布”return按钮 的 Post 正下方 部分：

```typescript
// pages/p/[id].tsx
{
  !props.published && userHasValidSession && postBelongsToUser && (
    <button onClick={() => publishPost(props.id)}>Publish</button>
  );
}
{
  userHasValidSession && postBelongsToUser && (
    <button onClick={() => deletePost(props.id)}>Delete</button>
  );
}
```

您现在可以通过创建新草稿、导航到其详细视图，然后单击新出现的“删除”按钮来尝试新功能  ：

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F5yf3vrKOb7i56gMZY7zJQA%2F94975799afd8fa6cbe460e9301bc389d%2F13.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

## 步骤 12. 部署到 Vercel

在最后一步中，您将把应用程序从 GitHub 存储库部署到 Vercel。

> 如果遇到build typescript error 可以吧`typescript`升级到"5.1.3"版本

在部署之前，您需要：

- 在 GitHub 上创建另一个 OAuth 应用程序
- 创建一个新的 GitHub 存储库并将您的项目推送到其中
要开始使用 OAuth 应用程序，请返回步骤“步骤 5.使用 NextAuth 设置 GitHub 身份验证”，然后按照步骤通过 GitHub UI 创建另一个 OAuth 应用程序。

这次， 授权回调 URL 需要与未来 Vercel 部署的域相匹配，该域将基于 Vercel 项目名称。作为 Vercel 项目名称，您将选择 blogr-nextjs-prisma 在您的名字和姓氏前面添加：  FIRSTNAME-LASTNAME-blogr-nextjs-prisma。例如，如果您的名字为“Jane Doe”，则您的项目名称应为 jane-doe-blogr-nextjs-prisma。

因此，授权 回调 URL 必须设置为 https://FIRSTNAME-LASTNAME-blogr-nextjs-prisma.vercel.app/api/auth。创建应用程序后，调整 .env 文件并将 客户端 ID设置 为 GITHUB_ID 环境变量，并将 客户端密钥设置 为 GITHUB_SECRET 环境变量。环境 NEXTAUTH_URL 变量需要设置为  与 GitHub 上的 授权回调 URLhttps://FIRSTNAME-LASTNAME-blogr-nextjs-prisma.vercel.app/api/auth相同的值：

接下来，创建一个具有相同名称的新 GitHub 存储库，例如 jane-doe-blogr-nextjs-prisma. 现在，从底部复制三个终端命令，显示 ...或从命令行推送现有存储库，它应该类似于以下内容：

``` bash
git remote add origin git@github.com:janedoe/jane-doe-blogr-nextjs-prisma.git
git branch -M main
git push -u origin main
```

您现在应该已准备好新存储库 https://github.com/GITHUB_USERNAME/FIRSTNAME-LASTNAME-blogr-nextjs-prisma，例如 https://github.com/janedoe/jane-doe-blogr-nextjs-prisma。

GitHub 存储库就位后，您现在可以将其导入 Vercel 以便部署应用程序：
![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F3B7J88ffcf3qs9Bha2dXzM%2F0cb1d6a68e7aeacb2d4e0b4cad8abe8d%2F15.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

单击 继续。下一个屏幕要求您为生产部署设置环境变量：
![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2F5sGJJuC4IwP36gk1bKUpqP%2Fa6ca4c68a304336a11a3961918f31639%2F16.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

以下是您需要提供的内容：

- GITHUB_ID：将其设置为您刚刚创建的 GitHub OAuth 应用程序的客户端 ID
- GITHUB_SECRET：将其设置为您刚刚创建的 GitHub OAuth 应用程序的客户端密钥
- NEXTAUTH_URL：将其设置为您刚刚创建的 GitHub OAuth 应用程序的授权回调 URL
- SECRET：将此设置为您自己的强大秘密。开发中不需要这一点，因为如果未提供，NextAuth.js 将生成一个。但是，您需要提供自己的生产值，否则您将收到错误。
您还需要将 Vercel postgres 数据库链接到此 Vercel 项目，以便自动添加所有数据库环境变量。设置所有环境变量后，点击 Deploy。您的应用程序现已部署到 Vercel。准备就绪后，Vercel 将向您显示以下成功屏幕：

![image](https://vercel.com/_next/image?url=https%3A%2F%2Fimages.ctfassets.net%2Fe5382hct74si%2Fyrw2Jce5AvD3uaa1zZWUl%2Fb28ccffc1601fd441cee1f21a504fc1c%2F17.png&w=3840&q=75&dpl=dpl_2z3u8qHghW7NbJ58cNyTG46GGME7)

您可以单击 “访问” 按钮来查看全栈应用程序的部署版本🎉


## 原文链接

- [nextjs-prisma-postgres](https://vercel.com/guides/nextjs-prisma-postgres)

> 原文中有些代码跑不了已在本文章修改，比如`getServerSession`替换原文中的`getSession`
