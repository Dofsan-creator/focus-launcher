app-name = Focus Launcher
lang-title = 言語を選んでください
welcome-title = ようこそ
welcome-body = ここではすべてをコントローラー、マウス、キーボードで操作できます。あなたの許可なしに PC は変更されません。
welcome-autostart = サインイン時に開く
welcome-sim = セッションはシミュレーションで始まります。変更内容を確認でき、何も変更されません。
welcome-continue = 続ける

filter-all = すべて
filter-games = ゲーム
filter-software = ソフトウェア
filter-social = ソーシャル

top-add = 追加
top-stores = ストア
top-settings = 設定
top-power = 電源

home-resume = 再開
home-empty-title = まだ何もありません
home-empty-body = インストール済みのゲームは自動で表示されます。プログラムを追加することもできます。
home-played = { $hours ->
   *[other] 使用時間 { $hours } 時間
}
home-running = 実行中

detail-play = プレイ
detail-open = 開く
detail-favorite = お気に入りに追加
detail-unfavorite = お気に入りから削除
detail-remove = ライブラリから削除
detail-launches = { $count ->
   *[other] 起動 { $count } 回
}
detail-source = 入手元: { $store }

prep-title = 準備中
prep-simulation = シミュレーション: 何も変更されません。
prep-real = これらの変更は終了時に元に戻されます。
prep-nothing = このセッションで変更するものはありません。
prep-start = 開始
prep-cancel = キャンセル
prep-throttle = バックグラウンドで減速: { $name }
prep-suspend = 一時停止: { $name }
prep-close = 閉じる: { $name }
prep-service = 一時的に停止: { $name }
prep-awake = PC をスリープさせない
prep-power = 高パフォーマンスの電源プラン
prep-ask = あなたが選択: { $name }
prep-no-guardian = 補助サービスが未インストールのため、サービスの変更は省略されます。

end-title = セッション終了
end-time = { $minutes ->
   *[other] { $minutes } 分
}
end-restored = すべて元に戻しました。
end-problem = 一部の項目を元に戻せませんでした。詳細はログにあります。
end-back = ライブラリに戻る

panel-title = クイックパネル
panel-return = 戻る
panel-close-entry = 閉じる
panel-library = ライブラリに戻る
panel-none = 実行中のものはありません

close-title = { $title } を閉じますか?
close-warning = 保存していない進行状況は失われます。
close-social = 通話や配信が止まる可能性があります。
close-no = いいえ
close-yes = はい、閉じる
close-waiting = 終了を待っています。保存を求められているかもしれません。
close-force = 強制終了
close-force-sure = 強制終了すると作業が失われる可能性があります。よろしいですか?

set-title = 設定
set-language = 言語
set-lang-status = 翻訳の状態: { $status }
status-reviewed = 確認済み
status-draft = 確認待ち
set-autostart = サインイン時に開く
set-real = 実際の最適化
set-real-body = オフ: シミュレーションのみ。オン: セッション中はバックグラウンドアプリを減速し、終了後に元に戻します。
set-real-confirm = セッション中にこの PC を実際に変更することを許可しますか?
set-keep-social = ソーシャルアプリには触れない
set-awake = セッション中は PC をスリープさせない
set-high-perf = 高パフォーマンスの電源プラン
set-big-picture = Steam のもう一つの全画面モードをブロック
set-sounds = サウンド
set-motion = アニメーションを減らす
set-theme = 時間帯に合わせたテーマ
set-glyphs = ボタンのアイコン
glyph-auto = 自動
glyph-playStation = 図形
glyph-xbox = 文字
set-detect = コントローラーを検出
set-guardian = 補助サービス
set-guardian-body = Windows のサービスを一時停止する場合にのみ必要です。Windows が一度だけ許可を求め、その画面はマウスかキーボードで操作します。
set-guardian-on = インストール済み
set-guardian-off = 未インストール
set-guardian-install = インストール
set-analysis = システム分析
state-on = オン
state-off = オフ
yes = はい
no = いいえ

ctl-family = コントローラー: { $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = 未検出
ctl-mode = モード: { $mode }
mode-native = ネイティブ
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = なし
ctl-connected = コントローラーを接続しました: { $family }

an-title = システム分析
an-processes = プロセス: { $count }
an-services = サービス: { $count }
an-memory = メモリ: { $gb } GB
an-cpu = プロセッサ: { $name }
an-top = メモリ使用量が多いもの
an-loading = システムを読み取っています。何も変更しません。

add-title = プログラムを追加
add-body = ファイルを選んでください。読み取るだけで、実行はしません。
add-up = 上のフォルダー
add-suggest = このフォルダーから提案
add-suggested = 可能性が高いファイル
add-name = 名前
add-kind = 種類
add-save = ライブラリに追加
add-script = これはスクリプトです。追加する前にフルパスを確認してください。
kind-game = ゲーム
kind-software = ソフトウェア
kind-social = ソーシャル
kind-other = その他

store-title = ストアとアプリ
store-install = インストール
store-page = 公式ページ
store-confirm = { $publisher } の { $name } をインストールしますか?
store-source = 入手元: Windows パッケージ マネージャー
store-uac = Windows が許可を求める場合があります。その画面はマウスかキーボードで操作します。
store-progress = { $name } をインストール中
store-done = インストールが完了しました。
store-failed = インストールに失敗しました。公式ページを利用できます。

power-title = 電源
power-quit = デスクトップに戻る
power-restart = 再起動
power-shutdown = シャットダウン
power-sure = よろしいですか?

kbd-done = 完了
kbd-space = スペース
kbd-delete = 削除

hint-select = 選択
hint-back = 戻る
hint-panel = クイックパネル

err-generic = 問題が発生しました。詳細はログにあります。
autostart-disabled = 起動に 2 回失敗したため、サインイン時に開く設定をオフにしました。
