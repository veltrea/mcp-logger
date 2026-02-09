# الإعداد عبر المنصات (باستخدام Google Antigravity)

يوضح هذا المستند كيفية إعداد بيئة الأدوات وخوادم MCP في بيئات مختلفة (ويندوز، ماك، لينكس)، مع التركيز على الخصوصية والأمان.

## 1. سياسة الخصوصية والمسارات
للحفاظ على مستوى عالٍ من الأمان، يفصل هذا المشروع أسماء المستخدمين العامة عن حسابات المستخدمين المحلية على الكمبيوتر.

> [!IMPORTANT]
> في المستندات والمحتوى الذي ينشئه الذكاء الاصطناعي، سيتم استبدال المسارات العامة بالرموز التالية:
> - **اسم المستخدم المحلي**: `[LOCAL_USER]`
> - **مسار المشروع**: `/Users/[LOCAL_USER]/DEV/rust-logger`

## 2. إعداد macOS
بيئة macOS هي البيئة الأساسية لتشغيل Claude Desktop وGoogle Antigravity.

- **التثبيت**: استخدم `brew` لإدارة التبعيات.
- **مسار تكوين MCP**: `~/Library/Application Support/Claude/claude_desktop_config.json`

## 3. إعداد Windows (Remote-Server / Local-Server)
يستخدم لتشغيل خوادم Syslog والتحقق من مهام ويندوز الخاصة.

- **Syslog**: استخدم `vlt-syslogd` كخدمة أساسية.
- **الترميز**: استخدم UTF-8 بشكل افتراضي.
- **مسار تكوين MCP**: `%APPDATA%\Claude\claude_desktop_config.json`

## 4. إعداد Linux (Ubuntu / Debian / AlmaLinux)
يستخدم لاختبار تكامل النظام وتسجيل Syslog عن بعد.

- **WOL (Wake-On-LAN)**: استخدم أمر `wakeonlan [MAC_ADDRESS]` لتشغيل الأجهزة عن بُعد.
- **دليل العمل**: `~/DEV`

## 5. الإشعارات الصوتية (Speak-MCP)
للمهام الطويلة، يمكنك استخدام `speak-mcp` للإخطار عند الانتهاء.
- **مثال**: `speak "تمت عملية التثبيت بنجاح"`

---
العودة إلى [ README.ar.md](../../README.ar.md) | [MANUAL.ar.md](MANUAL.ar.md)
