# MCP Logger - Manual مفصل

## 1. مقدمة
`mcp-logger` هو خادم MCP (بروتوكول سياق النموذج) مصمم للربط بين المساعدين الذكيين والبنية التحتية لـ Syslog. يتيح ذلك للذكاء الاصطناعي استخدام Syslog لأغراض التسجيل وتتبع الحالة.

## 2. التكوين

### Claude Desktop
أضف هذا إلى ملف `%APPDATA%\Claude\claude_desktop_config.json` (ويندوز) أو `~/Library/Application Support/Claude/claude_desktop_config.json` (ماك).

```json
{
  "mcpServers": {
    "mcp-logger": {
      "command": "/path/to/mcp-logger",
      "args": ["run-mcp"]
    }
  }
}
```

## 3. مرجع الأدوات

### `send-syslog`
يرسل رسالة إلى خادم Syslog.

| المعامل | النوع | الافتراضي | الوصف |
| :--- | :--- | :--- | :--- |
| `server` | string | (إلزامي) | عنوان IP أو اسم المضيف للخادم الوجهة. |
| `port` | number | 514 | رقم منفذ UDP. |
| `facility` | string | user | مرفق Syslog (kern، user، mail، إلخ) |
| `severity` | string | info | خطورة Syslog (emerg، alert، crit، err، warning، notice، info، debug) |
| `tag` | string | mcp-syslog | اسم البرنامج / الوسم. |
| `encoding` | string | utf-8 | ترميز الأحرف. |
| `message` | string | (إلزامي) | محتوى السجل. |

## 4. دليل الأحرف متعددة البايت (للغات غير اللاتينية مثل العربية)
إذا كنت تستخدم اللغة العربية أو لغات أخرى متعددة البايت، يرجى اتباع هذه الإرشادات:

### التوصية: استخدم UTF-8
تم تحسين `mcp-logger` لترميز UTF-8. يكتشف تلقائياً ما إذا كانت الرسالة تحتوي على أحرف غير لاتينية ويوفر **UTF-8 BOM (علامة ترتيب البايت)** إذا لزم الأمر.

### التوافق مع خوادم Syslog
يمكن للعديد من خوادم Syslog القائمة على ويندوز (مثل `vlt-syslogd`) أو إعدادات `rsyslog` الحديثة على لينكس التعامل مع UTF-8 إذا تم التعرف عليها بشكل صحيح. تضمن علامة BOM عدم قيام هذه الأدوات بإساءة تفسير السجلات على أنها ترميزات محلية قديمة.

## 5. استكشاف الأخطاء وإصلاحها
- **الذكاء الاصطناعي لا يتعرف على الأداة**: تأكد من أن المسار في ملف التكوين هو مسار مطلق (absolute path).
- **السجلات لا تصل**: تحقق مما إذا كان منفذ UDP 514 مفتوحاً في جدار الحماية الخاص بالخادم الوجهة.
- **خطأ في الأحرف**: تأكد من ضبط معامل `encoding` على `utf-8`.

## 6. الأدلة التكميلية
- [دليل الإعداد عبر المنصات](CROSS_PLATFORM_SETUP.ar.md)
- [دليل إعداد LM Studio](LM_STUDIO_SETUP.ar.md)
- [أمثلة بروتوكول MCP](MCP_PROTOCOL_EXAMPLES.ar.md)
- [المواصفات الفنية (SPEC)](SPEC.md)
