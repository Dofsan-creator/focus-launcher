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
