# セットアップ
## 環境設定ファイル
`cp .env.example .env`  
Postgresqlの値を.envに設定する  
## データベース
`diesel setup`  
migrations以下に定義されたテーブルが作られる  
## サーバー
`cargo build`  
`cargo run`  
localhost:3000/index.htmlにアクセスできるようになる