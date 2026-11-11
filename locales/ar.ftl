app-name = Focus Launcher
lang-title = اختر لغتك
welcome-title = مرحبًا
welcome-body = كل شيء هنا يعمل بيد التحكم أو الفأرة أو لوحة المفاتيح. لا يتغير شيء في حاسوبك دون موافقتك.
welcome-autostart = الفتح عند تسجيل الدخول
welcome-sim = تبدأ الجلسات بالمحاكاة: ترى ما الذي سيتغير، ولا يُمَسّ أي شيء.
welcome-continue = متابعة

filter-all = الكل
filter-games = الألعاب
filter-software = البرامج
filter-social = التواصل

top-add = إضافة
top-stores = المتاجر
top-settings = الإعدادات
top-power = الطاقة

home-resume = استئناف
home-empty-title = لا شيء هنا بعد
home-empty-body = تظهر الألعاب المثبتة تلقائيًا. يمكنك أيضًا إضافة برنامج.
home-played = { $hours ->
    [zero] لم يُستخدم بعد
    [one] ساعة استخدام واحدة
    [two] ساعتا استخدام
    [few] { $hours } ساعات استخدام
   *[other] { $hours } ساعة استخدام
}
home-running = قيد التشغيل

detail-play = العب
detail-open = فتح
detail-favorite = إضافة إلى المفضلة
detail-unfavorite = إزالة من المفضلة
detail-remove = إزالة من المكتبة
detail-launches = { $count ->
    [one] تشغيل واحد
    [two] تشغيلان
    [few] { $count } مرات تشغيل
   *[other] { $count } مرة تشغيل
}
detail-source = المصدر: { $store }

prep-title = جارٍ التحضير
prep-simulation = محاكاة: لن يتغير أي شيء.
prep-real = سيتم التراجع عن هذه التغييرات عند الانتهاء.
prep-nothing = لا شيء لتغييره في هذه الجلسة.
prep-start = ابدأ
prep-cancel = إلغاء
prep-throttle = إبطاء في الخلفية: { $name }
prep-suspend = إيقاف مؤقت: { $name }
prep-close = إغلاق: { $name }
prep-service = إيقاف مؤقتًا: { $name }
prep-awake = إبقاء الحاسوب مستيقظًا
prep-power = خطة طاقة عالية الأداء
prep-ask = القرار لك: { $name }
prep-no-guardian = تم تخطي تغييرات الخدمات: الخدمة المساعدة غير مثبتة.

end-title = انتهت الجلسة
end-time = { $minutes ->
    [zero] أقل من دقيقة
    [one] دقيقة واحدة
    [two] دقيقتان
    [few] { $minutes } دقائق
   *[other] { $minutes } دقيقة
}
end-restored = تمت استعادة كل شيء.
end-problem = تعذرت استعادة بعض العناصر. التفاصيل في السجل.
end-back = العودة إلى المكتبة

panel-title = اللوحة السريعة
panel-return = العودة إليه
panel-close-entry = إغلاقه
panel-library = العودة إلى المكتبة
panel-none = لا شيء قيد التشغيل

close-title = إغلاق { $title }؟
close-warning = سيضيع التقدم غير المحفوظ.
close-social = قد تنقطع مكالمة أو بث.
close-no = لا
close-yes = نعم، أغلق
close-waiting = بانتظار الإغلاق. ربما يطلب منك الحفظ.
close-force = فرض الإغلاق
close-force-sure = قد يؤدي فرض الإغلاق إلى فقدان العمل. هل أنت متأكد؟

set-title = الإعدادات
set-language = اللغة
set-lang-status = حالة الترجمة: { $status }
status-reviewed = تمت مراجعتها
status-draft = بحاجة إلى مراجعة
set-autostart = الفتح عند تسجيل الدخول
set-real = تحسين فعلي
set-real-body = معطّل: محاكاة فقط. مفعّل: تُبطأ تطبيقات الخلفية أثناء الجلسة ثم تُستعاد.
set-real-confirm = هل تسمح بتغييرات فعلية على هذا الحاسوب أثناء الجلسات؟
set-keep-social = عدم المساس بتطبيقات التواصل
set-awake = إبقاء الحاسوب مستيقظًا أثناء الجلسة
set-high-perf = خطة طاقة عالية الأداء
set-big-picture = حظر وضع ملء الشاشة الآخر في Steam
set-sounds = الأصوات
set-motion = تقليل الحركة
set-theme = السمة حسب الوقت
set-glyphs = رموز الأزرار
glyph-auto = تلقائي
glyph-playStation = أشكال
glyph-xbox = حروف
set-detect = اكتشاف يد التحكم
set-guardian = الخدمة المساعدة
set-guardian-body = مطلوبة فقط لإيقاف خدمات Windows مؤقتًا. يطلب Windows الإذن مرة واحدة، وتحتاج تلك النافذة إلى الفأرة أو لوحة المفاتيح.
set-guardian-on = مثبتة
set-guardian-off = غير مثبتة
set-guardian-install = تثبيت
set-analysis = تحليل النظام
state-on = مفعّل
state-off = معطّل
yes = نعم
no = لا

ctl-family = يد التحكم: { $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = غير مكتشفة
ctl-mode = الوضع: { $mode }
mode-auto = تلقائي
mode-native = أصلي
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = بلا
ctl-connected = تم توصيل يد التحكم: { $family }

an-title = تحليل النظام
an-processes = العمليات: { $count }
an-services = الخدمات: { $count }
an-memory = الذاكرة: { $gb } غيغابايت
an-cpu = المعالج: { $name }
an-top = الأكثر استهلاكًا للذاكرة
an-loading = جارٍ قراءة نظامك. لا يتغير شيء.

add-title = إضافة برنامج
add-body = اختر ملفًا. تتم قراءته فقط، ولا يُشغَّل أبدًا.
add-up = المجلد الأعلى
add-suggest = اقتراح من هذا المجلد
add-suggested = الملفات الأكثر احتمالًا
add-name = الاسم
add-kind = النوع
add-save = إضافة إلى المكتبة
add-script = هذا نص برمجي. تحقق من مساره الكامل قبل إضافته.
kind-game = لعبة
kind-software = برنامج
kind-social = تواصل
kind-other = أخرى

store-title = المتاجر والتطبيقات
store-install = تثبيت
store-page = الصفحة الرسمية
store-confirm = تثبيت { $name } من { $publisher }؟
store-source = المصدر: مدير حزم Windows
store-uac = قد يطلب Windows الإذن. تحتاج تلك النافذة إلى الفأرة أو لوحة المفاتيح.
store-progress = جارٍ تثبيت { $name }
store-done = اكتمل التثبيت.
store-failed = فشل التثبيت. يمكنك استخدام الصفحة الرسمية.

power-title = الطاقة
power-quit = الخروج إلى سطح المكتب
power-restart = إعادة التشغيل
power-shutdown = إيقاف التشغيل
power-sure = هل أنت متأكد؟

kbd-done = تم
kbd-space = مسافة
kbd-delete = حذف

hint-select = اختيار
hint-back = رجوع
hint-panel = اللوحة السريعة

err-generic = حدث خطأ ما. التفاصيل في السجل.
autostart-disabled = فشل المشغّل في البدء مرتين، لذا تم إيقاف الفتح عند تسجيل الدخول.

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = بحث
hint-options = خيارات
home-searching = نتائج البحث عن «{ $text }»
detail-remove-confirm = إزالة { $title } من المكتبة؟
set-section-console = وضع المنصة
set-console = تحويل Windows إلى وضع المنصة ما دام Focus مفتوحًا
set-console-body = يختفي شريط المهام وتصمت الإشعارات. اخرج من Focus لتستعيد Windows المعتاد.
set-hide-taskbar = إخفاء شريط المهام
set-silence = كتم الإشعارات
set-start-minimized = البدء مصغرًا عند تسجيل الدخول
set-section-opt = الأداء
set-section-sound = الصوت والعرض
set-music = موسيقى الخلفية
set-vibration = اهتزاز وحدة التحكم
opt-off = متوقف
opt-off-body = محاكاة: ترى ما سيتغير ولا يُمس أي شيء.
opt-balanced = متوازن
opt-balanced-body = تُبطَّأ التطبيقات في الخلفية أثناء الجلسة ثم تُستعاد.
opt-max = أقصى
opt-max-body = كل الوسائل المسموح بها: إبطاء التطبيقات وإيقاف التحديثات مؤقتًا والأداء العالي. ثم يُستعاد كل شيء.
power-quit-body = يعود Windows إلى وضعه الطبيعي.
power-restart-body = يعيد تشغيل الحاسوب.
power-shutdown-body = يطفئ الحاسوب.
con-title = الاتصالات
con-wifi = Wi-Fi
con-bt = Bluetooth
con-scan = البحث مجددًا
con-searching = جارٍ البحث…
con-connected = متصل
con-connected-to = متصل بـ { $name }
con-connect = اتصال
con-disconnected = تم قطع الاتصال.
con-password = كلمة المرور
con-none = لم يُعثر على أي شبكة.
con-ethernet = اتصال سلكي
con-offline = غير متصل
con-result-success = تم الاتصال.
con-result-badPassword = كلمة المرور غير صحيحة.
con-result-timeout = لم تستجب الشبكة.
con-result-failed = تعذر الاتصال.
con-result-notFound = لم يُعثر على الشبكة. أعد البحث.
con-result-noAdapter = لم يُعثر على محول Wi-Fi.
bt-paired = أجهزتك
bt-search = البحث عن أجهزة
bt-pair = إقران
bt-pair-hint = ضع الجهاز في وضع الإقران ثم ابدأ البحث.
bt-forget-confirm = نسيان { $name }؟
bt-forgotten = تم نسيان الجهاز.
bt-result-success = تم إقران الجهاز.
bt-result-failed = تعذر الإقران.
setup-step = الخطوة { $current }
setup-next = التالي
setup-back = رجوع
setup-start = ابدأ
setup-console-title = حاسوبك يصبح منصة ألعاب
setup-console-body = ما دام Focus مفتوحًا يتنحى Windows جانبًا: لا شريط مهام ولا إشعارات. اخرج من Focus فيعود كل شيء كما كان.
setup-pad-title = وحدة التحكم
setup-pad-body = اختر طريقة عرض الأزرار. يعمل Focus أيضًا بالفأرة ولوحة المفاتيح.
setup-net-title = ابقَ متصلًا
setup-net-body = Wi-Fi وBluetooth على بُعد زر واحد دائمًا، حتى أثناء اللعب.
setup-lib-title = مكتبتك
setup-lib-body = { $count ->
    [zero] لم يُعثر على أي لعبة أو تطبيق على هذا الحاسوب.
    [one] عُثر على لعبة أو تطبيق واحد على هذا الحاسوب.
    [two] عُثر على لعبتين أو تطبيقين على هذا الحاسوب.
    [few] عُثر على { $count } ألعاب وتطبيقات على هذا الحاسوب.
    [many] عُثر على { $count } لعبة وتطبيقًا على هذا الحاسوب.
   *[other] عُثر على { $count } لعبة وتطبيق على هذا الحاسوب.
}
setup-opt-title = الأداء أثناء الجلسات
setup-opt-body = يستطيع Focus تخفيف الحمل عن حاسوبك أثناء اللعب ثم إعادة كل شيء كما كان. القرار لك ويمكنك تغييره لاحقًا.
setup-comfort-title = الصوت والراحة
setup-comfort-body = اضبط الأجواء. عند فتح Focus مع تسجيل الدخول يبدأ حاسوبك كمنصة ألعاب.
setup-done-title = كل شيء جاهز
setup-done-body = مكتبتك جاهزة. اضغط «ابدأ» للدخول.
setup-open-wifi = إعداد Wi-Fi
setup-open-bt = إقران وحدة تحكم
store-installed = مثبّت
filter-stores = المتاجر
launch-starting = جارٍ التشغيل…
store-open = فتح
panel-screenshot = لقطة شاشة
shot-saved = حُفظت اللقطة في الصور\Focus.
detail-update = يتوفر تحديث
detail-update-open = تحديث
power-sleep = سكون
power-sleep-body = يدخل الحاسوب في السكون مثل وضع راحة المنصة.
end-auto = العودة إلى المكتبة…
con-ethernet-on = متصل بكابل (Ethernet)
ctl-native-note = خارج Steam، يستبدل Focus Input ذراع PlayStation بذراع Xbox تتعرف عليه كل الألعاب. لتجنب تكرار الأوامر يلزم HidHide (في متجر Focus، قسم برامج التشغيل): يخفي Focus الذراع الحقيقي عن اللعبة أثناء اللعب ثم يعيده. بدون HidHide يترك «تلقائي» الذراع كما هو.
steam-guide-on = في Steam، لا يزال الزر الأوسط في الذراع يفتح Steam فوق ألعابك. في Steam: الإعدادات، وحدة التحكم، الإعدادات المتقدمة، أوقف خيار زر الدليل الذي يضع Steam في المقدمة وإعداد تركيبات زر الدليل.
steam-guide-off = يترك Steam الزر الأوسط لـ Focus.
steam-guide-open = فتح إعدادات وحدة التحكم في Steam

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = جارٍ تجهيز الحاسوب…
profile-on = وضع المنصة
profile-off = الوضع العادي
unit-gb = غ.ب
profile-flash = حاسوبك الآن في وضع المنصة
profile-shell = سطح مكتب Windows مخفي: يعود عند الخروج من Focus
profile-freed = تم تحرير { $gb } غ.ب من الذاكرة لألعابك
profile-power = تم تفعيل القدرة القصوى
profile-slowed = { $count ->
    [one] تم إبطاء برنامج واحد غير ضروري
    [two] تم إبطاء برنامجين غير ضروريين
    [few] تم إبطاء { $count } برامج غير ضرورية
   *[other] تم إبطاء { $count } برنامجًا غير ضروري
}
profile-closed = { $count ->
    [one] تم إغلاق برنامج واحد
    [two] تم إغلاق برنامجين
    [few] تم إغلاق { $count } برامج
   *[other] تم إغلاق { $count } برنامجًا
}
profile-paused = { $count ->
    [one] تم إيقاف مهمة واحدة في خلفية Windows مؤقتًا
    [two] تم إيقاف مهمتين في خلفية Windows مؤقتًا
    [few] تم إيقاف { $count } مهام في خلفية Windows مؤقتًا
   *[other] تم إيقاف { $count } مهمة في خلفية Windows مؤقتًا
}
profile-title = ما يفعله Focus من أجلك
profile-lead = ما دام Focus مفتوحًا يتصرف حاسوبك كمنصة ألعاب: كل ما لا تحتاجه اللعبة يوضع جانبًا. ويعود كل شيء عند الخروج من Focus.
profile-lead-off = سطح المكتب مخفي، لكن حاسوبك لم يُخفَّف للّعب بعد. فعّل الأداء من الإعدادات.
profile-none = حاسوبك في حالته العادية. فعّل وضع المنصة من الإعدادات ليتصرف كمنصة ألعاب.
profile-memory = الذاكرة المتاحة لألعابك
profile-power-label = القدرة
power-plan-balanced = متوازنة
power-plan-high = قصوى
power-plan-saver = توفير الطاقة
power-plan-other = مخصصة
apps-title = { $count ->
    [one] ما زال تطبيق واحد مفتوحًا
    [two] ما زال تطبيقان مفتوحين
    [few] ما زالت { $count } تطبيقات مفتوحة
   *[other] ما زال { $count } تطبيقًا مفتوحًا
}
apps-body = إغلاقها يفرّغ حاسوبك للّعبة. إذا طلب أحدها الحفظ فسيبقى مفتوحًا: لن يضيع شيء.
apps-closing = جارٍ الإغلاق…
apps-close = إغلاقها
apps-keep = إبقاؤها
set-stop-shell = إخفاء سطح مكتب Windows تمامًا

# Téléchargements, aide, mise à jour, signalement (à relire)
dl-title = التنزيلات
dl-empty = لا يوجد شيء قيد التنزيل الآن.
dl-steam = ألعاب Steam
dl-waiting = في الانتظار
dl-focus = تثبيتات بدأت من Focus
dl-state-running = جارٍ التثبيت…
dl-state-done = تم التثبيت
dl-state-failed = فشل التثبيت
dl-open-steam = العرض في Steam
dl-open-epic = العرض في Epic Games
dl-other = المتاجر الأخرى تعرض تنزيلاتها في نافذتها الخاصة.
help-title = المساعدة والتحديث
help-version = Focus { $version }
upd-check = البحث عن تحديث
upd-checking = جارٍ البحث…
upd-none = Focus محدَّث.
upd-available = Focus { $version } متاح
upd-install = تثبيت التحديث
upd-downloading = جارٍ تنزيل التحديث: { $percent } %
upd-restart = سيُغلق Focus ثم يُفتح من جديد محدَّثًا.
upd-failed = تعذّر التحديث. حاول لاحقًا.
rep-title = الإبلاغ عن مشكلة
rep-body = صِف ما حدث. بعدها يفتح متصفحك نموذجًا مملوءًا مسبقًا: تحتاج إلى حساب GitHub مجاني لإرساله. يُحذف اسمك ومجلدك الشخصي من التقرير.
rep-describe = وصف المشكلة
rep-send = فتح النموذج
rep-opened = النموذج مفتوح في متصفحك.
