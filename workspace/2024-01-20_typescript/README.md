# TypeScript で Hello World
## プロジェクト作成方法
[参考資料](https://qiita.com/notakaos/items/3bbd2293e2ff286d9f49)

## バージョン確認
```
$ node -v
v18.18.0
$ npm -v
9.8.1
```

## プロジェクト作成と TypeScript インストール
```
npm init -y
npm install -D typescript @types/node@18
npx tsc --init
```

## gitignore 生成
 [`giho`](https://github.com/simonwhitaker/gibo) を使う。
```
gibo dump macos linux windows node > .gitignore
```

## tsconfig.json 修正
* target については [こちら](https://github.com/microsoft/TypeScript/wiki/Node-Target-Mapping) を参照。
* オプションについては [こちら](https://zenn.dev/chida/articles/bdbcd59c90e2e1)を参照。
* 以下、コメントアウトをはずした部分を記載。

```
{
  "compilerOptions": {
    "target": "ES2022",                                  /* Set the JavaScript language version for emitted JavaScript and include compatible library declarations. */
    "lib": ["ES2022"],                                        /* Specify a set of bundled library declaration files that describe the target runtime environment. */
    "module": "commonjs",                                /* Specify what module code is generated. */
    "rootDir": "./src",                                  /* Specify the root folder within your source files. */
    "outDir": "./dist",                                   /* Specify an output folder for all emitted files. */
    "esModuleInterop": true,                             /* Emit additional JavaScript to ease support for importing CommonJS modules. This enables 'allowSyntheticDefaultImports' for type compatibility. */
    "forceConsistentCasingInFileNames": true,            /* Ensure that casing is correct in imports. */
    "strict": true,                                      /* Enable all strict type-checking options. */
    "skipLibCheck": true                                 /* Skip type checking all .d.ts files. */
  },
  "include": [
    "src/**/*"
  ],
}
```

## コンパイルと実行
ツールの導入：

```
npm install -D ts-node ts-node-dev
```

コンパイルと実行：
```
npx ts-node src/index.ts
```

↑は以下のコマンドを実行しているのと同じ：
```
npx tsc
node dist/index.js
```

変更を自動的に検知するためには以下のようにする：
```
npx ts-node-dev --respawn src/index.ts
```