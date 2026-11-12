app-name = Focus Launcher
lang-title = 选择语言
welcome-title = 欢迎
welcome-body = 这里的一切都可以用手柄、鼠标或键盘操作。未经你同意，电脑不会有任何改动。
welcome-autostart = 登录时打开
welcome-sim = 会话以模拟方式开始：你会看到将会改变什么，但不会动任何东西。
welcome-continue = 继续

filter-all = 全部
filter-games = 游戏
filter-software = 软件
filter-social = 社交

top-add = 添加
top-stores = 商店
top-settings = 设置
top-power = 电源

home-resume = 继续
home-empty-title = 这里还没有内容
home-empty-body = 已安装的游戏会自动出现。你也可以添加程序。
home-played = { $hours ->
   *[other] 已使用 { $hours } 小时
}
home-running = 运行中

detail-play = 开始游戏
detail-open = 打开
detail-favorite = 加入收藏
detail-unfavorite = 取消收藏
detail-remove = 从库中移除
detail-launches = { $count ->
   *[other] 启动 { $count } 次
}
detail-source = 来源：{ $store }

prep-title = 正在准备
prep-simulation = 模拟：不会做任何改动。
prep-real = 结束后这些改动会被撤销。
prep-nothing = 本次会话无需改动。
prep-start = 开始
prep-cancel = 取消
prep-throttle = 在后台降速：{ $name }
prep-suspend = 暂停：{ $name }
prep-close = 关闭：{ $name }
prep-service = 暂时停止：{ $name }
prep-awake = 保持电脑唤醒
prep-power = 高性能电源计划
prep-ask = 由你决定：{ $name }
prep-no-guardian = 未安装辅助服务，已跳过服务改动。

end-title = 会话结束
end-time = { $minutes ->
   *[other] { $minutes } 分钟
}
end-restored = 一切已恢复。
end-problem = 部分项目未能恢复。详情见日志。
end-back = 返回游戏库

panel-title = 快捷面板
panel-return = 返回
panel-close-entry = 关闭
panel-library = 返回游戏库
panel-none = 没有正在运行的内容

close-title = 关闭 { $title }？
close-warning = 未保存的进度将会丢失。
close-social = 通话或直播可能会中断。
close-no = 否
close-yes = 是，关闭
close-waiting = 正在等待关闭。它可能在请你保存。
close-force = 强制关闭
close-force-sure = 强制关闭可能会丢失工作。确定吗？

set-title = 设置
set-language = 语言
set-lang-status = 翻译状态：{ $status }
status-reviewed = 已审校
status-draft = 待审校
set-autostart = 登录时打开
set-real = 实际优化
set-real-body = 关闭：仅模拟。开启：会话期间降低后台应用的速度，结束后恢复。
set-real-confirm = 允许在会话期间对这台电脑进行实际改动吗？
set-keep-social = 不动社交应用
set-awake = 会话期间保持电脑唤醒
set-high-perf = 高性能电源计划
set-big-picture = 阻止 Steam 的另一种全屏模式
set-sounds = 声音
set-motion = 减少动画
set-theme = 主题随时间变化
set-glyphs = 按键图标
glyph-auto = 自动
glyph-playStation = 图形
glyph-xbox = 字母
set-detect = 检测手柄
set-guardian = 辅助服务
set-guardian-body = 仅在暂停 Windows 服务时需要。Windows 会请求一次权限，该窗口需要用鼠标或键盘操作。
set-guardian-on = 已安装
set-guardian-off = 未安装
set-guardian-install = 安装
set-analysis = 系统分析
state-on = 开
state-off = 关
yes = 是
no = 否

ctl-family = 手柄：{ $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = 未检测到
ctl-mode = 模式：{ $mode }
mode-auto = 自动
mode-native = 原生
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = 无
ctl-connected = 手柄已连接：{ $family }

an-title = 系统分析
an-processes = 进程：{ $count }
an-services = 服务：{ $count }
an-memory = 内存：{ $gb } GB
an-cpu = 处理器：{ $name }
an-top = 内存占用最多
an-loading = 正在读取系统信息，不会做任何改动。

add-title = 添加程序
add-body = 选择一个文件。只读取，绝不运行。
add-up = 上一级文件夹
add-suggest = 从此文件夹推荐
add-suggested = 最可能的文件
add-name = 名称
add-kind = 类型
add-save = 添加到库
add-script = 这是一个脚本。添加前请检查完整路径。
kind-game = 游戏
kind-software = 软件
kind-social = 社交
kind-other = 其他

store-title = 商店和应用
store-install = 安装
store-page = 官方页面
store-confirm = 安装 { $publisher } 的 { $name }？
store-source = 来源：Windows 程序包管理器
store-uac = Windows 可能会请求权限。该窗口需要用鼠标或键盘操作。
store-progress = 正在安装 { $name }
store-done = 安装完成。
store-failed = 安装失败。你可以使用官方页面。

power-title = 电源
power-quit = 退出到桌面
power-restart = 重启
power-shutdown = 关机
power-sure = 确定吗？

kbd-done = 完成
kbd-space = 空格
kbd-delete = 删除

hint-select = 选择
hint-back = 返回
hint-panel = 快捷面板

err-generic = 出现问题。详情见日志。
autostart-disabled = 启动器连续两次启动失败，已关闭登录时打开。

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = 搜索
hint-options = 选项
home-searching = “{ $text }”的搜索结果
detail-remove-confirm = 要从库中移除 { $title } 吗？
set-section-console = 主机模式
set-console = Focus 打开时将 Windows 切换为主机模式
set-console-body = 任务栏会隐藏，通知会静音。退出 Focus 即可回到平常的 Windows。
set-hide-taskbar = 隐藏任务栏
set-silence = 通知静音
set-start-minimized = 登录时最小化启动
set-section-opt = 性能
set-section-sound = 声音与显示
set-music = 背景音乐
set-vibration = 手柄振动
opt-off = 关闭
opt-off-body = 模拟：只显示会改变什么，不做任何改动。
opt-balanced = 均衡
opt-balanced-body = 会话期间降低后台应用的速度，结束后恢复。
opt-max = 最大
opt-max-body = 所有允许的手段：降低应用速度、暂停更新、高性能。结束后全部恢复。
power-quit-body = Windows 将恢复正常状态。
power-restart-body = 重启电脑。
power-shutdown-body = 关闭电脑。
con-title = 连接
con-wifi = Wi-Fi
con-bt = 蓝牙
con-scan = 重新搜索
con-searching = 正在搜索…
con-connected = 已连接
con-connected-to = 已连接到 { $name }
con-connect = 连接
con-disconnected = 已断开连接。
con-password = 密码
con-none = 未找到网络。
con-ethernet = 有线连接
con-offline = 未连接
con-result-success = 已连接。
con-result-badPassword = 密码错误。
con-result-timeout = 网络没有响应。
con-result-failed = 连接失败。
con-result-notFound = 找不到该网络，请重新搜索。
con-result-noAdapter = 未找到 Wi-Fi 适配器。
bt-paired = 你的设备
bt-search = 查找设备
bt-pair = 配对
bt-pair-hint = 请将设备设为配对模式，然后搜索。
bt-forget-confirm = 要删除 { $name } 吗？
bt-forgotten = 已删除设备。
bt-result-success = 设备已配对。
bt-result-failed = 配对失败。
setup-step = 第 { $current } 步
setup-next = 下一步
setup-back = 返回
setup-start = 开始
setup-console-title = 你的电脑变成了主机
setup-console-body = Focus 打开时，Windows 会退到幕后：没有任务栏，没有通知。退出 Focus，一切恢复原样。
setup-pad-title = 你的手柄
setup-pad-body = 选择按键的显示方式。Focus 也支持鼠标和键盘。
setup-net-title = 保持连接
setup-net-body = 即使在游戏中，Wi-Fi 和蓝牙也只需一个按键。
setup-lib-title = 你的库
setup-lib-body = { $count ->
   *[other] 在这台电脑上找到 { $count } 个游戏和应用。
}
setup-opt-title = 会话期间的性能
setup-opt-body = Focus 可以在你游戏时为电脑减负，结束后一切复原。由你决定，之后也可以更改。
setup-comfort-title = 声音与舒适
setup-comfort-body = 调整氛围。登录时打开 Focus，电脑就会像主机一样启动。
setup-done-title = 一切就绪
setup-done-body = 你的库已准备好。按“开始”进入。
setup-open-wifi = 设置 Wi-Fi
setup-open-bt = 配对手柄
store-installed = 已安装
filter-stores = 商店
launch-starting = 正在启动…
store-open = 打开
panel-screenshot = 截图
shot-saved = 截图已保存到 图片\Focus。
detail-update = 有可用更新
detail-update-open = 更新
power-sleep = 睡眠
power-sleep-body = 像主机的休息模式一样让电脑睡眠。
end-auto = 正在返回库…
con-ethernet-on = 已通过网线连接（以太网）
ctl-native-note = 在 Steam 之外，Focus Input 会用所有游戏都能识别的 Xbox 手柄替代 PlayStation 手柄。为避免重复输入，需要 HidHide（在 Focus 商店的驱动程序中）：游戏期间 Focus 会对游戏隐藏真实手柄，结束后再恢复。没有 HidHide 时，“自动”会保持手柄原样。
steam-guide-on = 在 Steam 中，手柄中央按钮仍会把 Steam 调到游戏上方。请在 Steam 的 设置 > 控制器 > 高级设置 中关闭让指南按钮将 Steam 置于前台的选项，以及指南按钮组合键配置。
steam-guide-off = Steam 已把中央按钮留给 Focus。
steam-guide-open = 打开 Steam 控制器设置

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = 正在准备电脑…
profile-on = 主机模式
profile-off = 普通模式
unit-gb = GB
profile-flash = 你的电脑已进入主机模式
profile-shell = Windows 桌面已隐藏：退出 Focus 后恢复
profile-freed = 已为游戏腾出 { $gb } GB 内存
profile-power = 已开启最大性能
profile-slowed = { $count ->
   *[other] 已让 { $count } 个用不到的程序慢下来
}
profile-closed = { $count ->
   *[other] 已关闭 { $count } 个程序
}
profile-paused = { $count ->
   *[other] 已暂停 { $count } 个 Windows 后台任务
}
profile-title = Focus 为你做了什么
profile-lead = Focus 打开时，你的电脑就像一台游戏主机：游戏用不到的东西都先放到一边。退出 Focus 后全部恢复。
profile-lead-off = 桌面已隐藏，但电脑还没有为游戏减负。请在设置中开启性能优化。
profile-none = 你的电脑处于普通状态。在设置中开启主机模式，它就会像游戏主机一样工作。
profile-memory = 可供游戏使用的空闲内存
profile-power-label = 性能
power-plan-balanced = 平衡
power-plan-high = 最大
power-plan-saver = 节能
power-plan-other = 自定义
apps-title = { $count ->
   *[other] 还有 { $count } 个应用开着
}
apps-body = 关闭它们可以让电脑专心运行游戏。如果某个应用提示保存，它会保持打开：不会丢失任何内容。
apps-closing = 正在关闭…
apps-close = 关闭它们
apps-keep = 保留
set-stop-shell = 完全隐藏 Windows 桌面

# Téléchargements, aide, mise à jour, signalement (à relire)
dl-title = 下载
dl-empty = 现在没有正在下载的内容。
dl-steam = Steam 游戏
dl-waiting = 等待中
dl-focus = 从 Focus 开始的安装
dl-state-running = 正在安装…
dl-state-done = 已安装
dl-state-failed = 安装失败
dl-open-steam = 在 Steam 中查看
dl-open-epic = 在 Epic Games 中查看
dl-other = 其他商店在各自的窗口中显示下载内容。
help-title = 帮助与更新
help-version = Focus { $version }
upd-check = 检查更新
upd-checking = 正在检查…
upd-none = Focus 已是最新版本。
upd-available = Focus { $version } 可以更新了
upd-install = 安装更新
upd-downloading = 正在下载更新：{ $percent } %
upd-restart = Focus 会关闭，然后以最新版本重新打开。
upd-failed = 更新没有成功，请稍后再试。
rep-title = 报告问题
rep-body = 请描述发生了什么。随后浏览器会打开一张已填好的表单：发送需要一个免费的 GitHub 账号。你的名字和个人文件夹会从报告中去掉。
rep-describe = 描述问题
rep-send = 打开表单
rep-opened = 表单已在浏览器中打开。

# Effet de profil et tutoriel du bouton central (à relire)
fx-profile = 性能模式
guide-title = 手柄的中间按键仍然会打开 Steam
guide-step-1 = 用下面的按钮打开 Steam 设置，然后选择“控制器”。
guide-step-2 = 关闭“导航键使 Steam 获得焦点”。
guide-step-3 = 关闭“为控制器启用导航键组合键配置”，然后回到这里。
guide-check = 已完成，检查
guide-later = 稍后
guide-show = 查看解决方法
