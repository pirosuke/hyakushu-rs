# hyakushu-rs

百人一首特訓Webアプリ

## ビルドに必要なツール

* PostgreSQL9.5 or later
* Node.js
* Rust (rustup)

## データベースのセットアップ

下記のSQLを順番に実行します。

* sql/create_databases.sql
* sql/create_tables.sql
* sql/insert_wakas.sql

## 設定ファイルを環境に合わせて変更

app_config.json の内容を更新します。

```json
{
    "public_dir_path": "hyakushu-web/public",
    "server_host": "127.0.0.1:8080", -- Webアプリを起動するポート
    "db": {
        "host": "localhost", -- データベースホスト
        "port": "5432", -- データベースポート
        "db_name": "hyakushu", -- データベース名
        "user_name": "dbuser", -- データベースユーザー名
        "password": "dbpassword" -- データベースパスワード
    }
}
```

## フロントエンドのセットアップとビルド

```
cd hyaushu-web/src_vue
npm install
npm run build
```

## Webアプリのビルドと起動

```
cd REPOSITORY_ROOT
cargo run -p hyakushu-web -- -c app_config.json
```