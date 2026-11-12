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
set-real-body = Focus を開いている間、ゲームに不要なものは脇に置かれます。プログラムの動作を抑え、Windows の裏方の作業を一時停止し、最大パワーにします。Focus を終了するとすべて元に戻ります。
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
mode-auto = 自動
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

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = 検索
hint-options = オプション
home-searching = 「{ $text }」の検索結果
detail-remove-confirm = { $title } をライブラリから削除しますか？
set-section-console = コンソールモード
set-console = Focus を開いている間、Windows をコンソールモードにする
set-console-body = デスクトップ、タスクバー、通知が消えます。Focus を終了すると、いつもの Windows に戻ります。
set-hide-taskbar = タスクバーを隠す
set-silence = 通知をミュートする
set-start-minimized = サインイン時に最小化して起動する
set-section-opt = パフォーマンス
set-section-sound = サウンドと表示
set-music = BGM
set-vibration = コントローラーの振動
opt-off = オフ
opt-off-body = シミュレーション：変更内容を確認するだけで、何も変更しません。
opt-balanced = バランス
opt-balanced-body = セッション中はバックグラウンドアプリを抑え、終了後に元へ戻します。
opt-max = 最大
opt-max-body = 許可されたすべての手段：アプリを抑え、更新を一時停止し、高パフォーマンスにします。終了後にすべて元に戻します。
power-quit-body = Windows が通常の状態に戻ります。
power-restart-body = PC を再起動します。
power-shutdown-body = PC の電源を切ります。
con-title = 接続
con-wifi = Wi-Fi
con-bt = Bluetooth
con-scan = もう一度検索
con-searching = 検索中…
con-connected = 接続済み
con-connected-to = { $name } に接続済み
con-connect = 接続
con-disconnected = 切断しました。
con-password = パスワード
con-none = ネットワークが見つかりません。
con-ethernet = 有線接続
con-offline = 未接続
con-result-success = 接続しました。
con-result-badPassword = パスワードが正しくありません。
con-result-timeout = ネットワークから応答がありません。
con-result-failed = 接続できませんでした。
con-result-notFound = ネットワークが見つかりません。もう一度検索してください。
con-result-noAdapter = Wi-Fi アダプターが見つかりません。
bt-paired = お使いのデバイス
bt-search = デバイスを探す
bt-pair = ペアリング
bt-pair-hint = デバイスをペアリングモードにしてから検索してください。
bt-forget-confirm = { $name } を削除しますか？
bt-forgotten = デバイスを削除しました。
bt-result-success = ペアリングしました。
bt-result-failed = ペアリングできませんでした。
setup-step = ステップ { $current }
setup-next = 次へ
setup-back = 戻る
setup-start = はじめる
setup-console-title = PC がコンソールに変わります
setup-console-body = Focus を開いている間、Windows は表に出ません。デスクトップもタスクバーも通知もありません。Focus を終了すれば元どおりです。
setup-pad-title = コントローラー
setup-pad-body = ボタンの表示方法を選んでください。Focus はマウスとキーボードでも使えます。
setup-net-title = いつでも接続
setup-net-body = Wi-Fi と Bluetooth は、ゲーム中でもボタンひとつで開けます。
setup-lib-title = ライブラリ
setup-lib-body = { $count ->
   *[other] この PC で { $count } 件のゲームとアプリが見つかりました。
}
setup-opt-title = セッション中のパフォーマンス
setup-opt-body = Focus はプレイ中に PC の負荷を減らし、終了後にすべて元へ戻せます。あとから変更することもできます。
setup-comfort-title = サウンドと快適さ
setup-comfort-body = 雰囲気を調整しましょう。サインイン時に Focus を開くと、PC がコンソールのように起動します。
setup-done-title = 準備完了
setup-done-body = ライブラリの準備ができました。「はじめる」を押して入りましょう。
setup-open-wifi = Wi-Fi を設定
setup-open-bt = コントローラーをペアリング
store-installed = インストール済み
filter-stores = ストア
launch-starting = 起動中…
store-open = 開く
panel-screenshot = スクリーンショット
shot-saved = スクリーンショットをピクチャ\Focus に保存しました。
detail-update = アップデートがあります
detail-update-open = アップデート
power-sleep = スリープ
power-sleep-body = コンソールのレストモードのように PC をスリープします。
end-auto = ライブラリに戻ります…
con-ethernet-on = 有線接続 (Ethernet)
ctl-native-note = Steam 以外では、Focus Input が PlayStation コントローラーを、すべてのゲームが認識する Xbox コントローラーに置き換えます。入力の重複を防ぐには HidHide（Focus のストアのドライバー）が必要です。プレイ中は本物のコントローラーをゲームから隠し、終了後に戻します。HidHide がない場合、「自動」はコントローラーをそのままにします。
steam-guide-on = Steam では、コントローラーの中央ボタンでまだゲームの上に Steam が開きます。Steam の 設定 > コントローラー > 詳細設定 で、ガイドボタンで Steam を前面に出すオプションと、ガイドボタンのコード設定をオフにしてください。
steam-guide-off = Steam は中央ボタンを Focus に任せています。
steam-guide-open = Steam のコントローラー設定を開く

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = PC を準備中…
profile-on = コンソールモード
profile-off = 通常モード
unit-gb = GB
profile-flash = PC がコンソールモードになりました
profile-shell = Windows のデスクトップを非表示にしました。Focus を終了すると戻ります
profile-freed = ゲームのために { $gb } GB のメモリを空けました
profile-power = 最大パワーに切り替えました
profile-slowed = { $count ->
   *[other] 不要なプログラム { $count } 個の動作を抑えました
}
profile-closed = { $count ->
   *[other] プログラム { $count } 個を閉じました
}
profile-paused = { $count ->
   *[other] Windows の裏方の作業 { $count } 個を一時停止しました
}
profile-title = Focus がしてくれること
profile-lead = Focus を開いている間、PC はゲーム機のように動きます。ゲームに必要のないものは脇に置かれ、Focus を終了するとすべて元に戻ります。
profile-lead-off = デスクトップは非表示ですが、PC はまだゲーム向けに軽くなっていません。設定でパフォーマンスを有効にしてください。
profile-none = PC は通常の状態です。設定でコンソールモードを有効にすると、ゲーム機のように動きます。
profile-memory = ゲームに使える空きメモリ
profile-power-label = パワー
power-plan-balanced = バランス
power-plan-high = 最大
power-plan-saver = 省電力
power-plan-other = カスタム
apps-title = { $count ->
   *[other] アプリが { $count } 個まだ開いています
}
apps-body = 閉じると PC をゲームに集中させられます。保存を求めるアプリは開いたままになるので、何も失われません。
apps-closing = 閉じています…
apps-close = 閉じる
apps-keep = 開いたままにする
set-stop-shell = Windows のデスクトップを完全に隠す

# Téléchargements, aide, mise à jour, signalement (à relire)
dl-title = ダウンロード
dl-empty = 現在ダウンロード中のものはありません。
dl-steam = Steam のゲーム
dl-waiting = 待機中
dl-focus = Focus から始めたインストール
dl-state-running = インストール中…
dl-state-done = インストール済み
dl-state-failed = インストールに失敗しました
dl-open-steam = Steam で見る
dl-open-epic = Epic Games で見る
dl-other = ほかのストアは、それぞれのウィンドウでダウンロードを表示します。
help-title = ヘルプとアップデート
help-version = Focus { $version }
upd-check = アップデートを確認
upd-checking = 確認中…
upd-none = Focus は最新です。
upd-available = Focus { $version } が利用できます
upd-install = アップデートをインストール
upd-downloading = アップデートをダウンロード中: { $percent } %
upd-restart = Focus はいったん閉じ、最新の状態で開き直します。
upd-failed = アップデートできませんでした。あとでもう一度お試しください。
rep-title = 問題を報告
rep-body = 何が起きたかを書いてください。そのあとブラウザーが入力済みのフォームを開きます。送信には無料の GitHub アカウントが必要です。名前と個人フォルダーはレポートから取り除かれます。
rep-describe = 問題を説明する
rep-send = フォームを開く
rep-opened = フォームをブラウザーで開きました。

# Effet de profil et tutoriel du bouton central (à relire)
fx-profile = パフォーマンスプロファイル
guide-title = コントローラーの中央ボタンがまだ Steam を開きます
guide-step-1 = 下のボタンで Steam の設定を開き、「コントローラー」を選びます。
guide-step-2 = 「ガイドボタンで Steam にフォーカス」をオフにします。
guide-step-3 = 「コントローラーのガイドボタンコード設定を有効にする」をオフにして、ここに戻ります。
guide-check = 完了、確認する
guide-later = あとで
guide-show = 直し方を見る
