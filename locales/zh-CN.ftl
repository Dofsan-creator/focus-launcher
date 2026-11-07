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
