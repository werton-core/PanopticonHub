; NSIS script (UTF-8) NSIS-3 Unicode
; Install

Unicode true
SetCompressor zlib

; --------------------
; HEADER SIZE: 164460
; START HEADER SIZE: 300
; MAX STRING LENGTH: 8192
; STRING CHARS: 51154

OutFile [NSIS].exe
!include WinMessages.nsh

ShowInstDetails nevershow


; --------------------
; LANG TABLES: 26
; LANG STRINGS: 77

Name "Shinri Hub"
BrandingText "Shinri Hub 26.11.23196"

; LANG: 1066
LangString LSTR_0 1066 "Shinri Hub 26.11.23196"
LangString LSTR_1 1066 "Cài đặt $(LSTR_2)"
LangString LSTR_2 1066 "Shinri Hub"
LangString LSTR_3 1066 "Dung lượng hiện có: "
LangString LSTR_4 1066 "Dung lượng yêu cầu: "
LangString LSTR_5 1066 "Không thể ghi: "
LangString LSTR_6 1066 "Sao chép bị thất bại"
LangString LSTR_7 1066 "Sao chép vào "
LangString LSTR_8 1066 "Không thể tìm thấy biểu tượng: "
LangString LSTR_9 1066 "Không thể nạp: "
LangString LSTR_10 1066 "Tạo thư mục: "
LangString LSTR_11 1066 "Tạo lối tắt: "
LangString LSTR_13 1066 "Xóa tập tin: "
LangString LSTR_14 1066 "Xóa khi khởi động lại: "
LangString LSTR_15 1066 "Lỗi khi tạo lối tắt: "
LangString LSTR_17 1066 "Lỗi khi giải nén dữ liệu! Có thể trình cài đặt đã bị hỏng?"
LangString LSTR_20 1066 "Thực thi: "
LangString LSTR_21 1066 "Trích xuất: "
LangString LSTR_22 1066 "Trích xuất: lôi khi ghi vào tập tin "
LangString LSTR_23 1066 "Trình cài đặt bị hỏng: mã tác vụ bất hợp lệ"
LangString LSTR_24 1066 "Không có OLE cho: "
LangString LSTR_25 1066 "Thư mục đầu ra: "
LangString LSTR_26 1066 "Xóa thư mục: "
LangString LSTR_27 1066 "Đổi tên khi khởi động lại: "
LangString LSTR_28 1066 "Đổi tên: "
LangString LSTR_29 1066 "Đã bỏ qua: "
LangString LSTR_30 1066 "Sao chép Chi tiết vào Clipboard"
LangString LSTR_32 1066 B
LangString LSTR_33 1066 " K"
LangString LSTR_34 1066 " M"
LangString LSTR_35 1066 " G"
LangString LSTR_36 1066 "Lỗi khi mở tập tin để ghi: $\r$\n$\r$\n$0$\r$\n$\r$\nNhấn “Hủy bỏ” để ngừng việc cài đặt,$\r$\n“Thử lại” để thử lần nữa, hoặc$\r$\n“Bỏ qua” để bỏ qua tập tin này."
LangString LSTR_37 1066 "Chọn thư mục cài đặt"
LangString LSTR_38 1066 "Chọn thư mục để cài đặt $(LSTR_76)."
LangString LSTR_39 1066 "Cài đặt"
LangString LSTR_40 1066 "Vui lòng đợi trong khi $(LSTR_76) đang được cài đặt."
LangString LSTR_41 1066 "Cài đặt hoàn tất"
LangString LSTR_42 1066 "Việc cài đặt đã hoàn tất thành công."
LangString LSTR_43 1066 "Cài đặt bị hủy"
LangString LSTR_44 1066 "Việc cài đặt không hoàn tất thành công."
LangString LSTR_45 1066 "&Hoàn thành"
LangString LSTR_46 1066 0
LangString LSTR_47 1066 "Hoàn tất Cài đặt $(LSTR_76)"
LangString LSTR_48 1066 "MS Shell Dlg"
LangString LSTR_49 1066 "Máy tính của bạn phải được khởi động lại để hoàn tất việc cài đặt $(LSTR_76). Bạn có muốn khởi động lại ngay không?"
LangString LSTR_50 1066 "Khởi động lại ngay"
LangString LSTR_51 1066 "Tôi muốn khởi động lại sau"
LangString LSTR_52 1066 "$(LSTR_76) đã được cài đặt vào máy tính của bạn.$\r$\n$\r$\nNhấn “Hoàn thành” để đóng Trình trợ lí."
LangString LSTR_53 1066 "&Chạy $(LSTR_76)"
LangString LSTR_54 1066 8
LangString LSTR_55 1066 "Windows 7 and above is required"
LangString LSTR_56 1066 "64-bit Windows is required"
LangString LSTR_57 1066 "Failed to uninstall old application files. Please try running the installer again."
LangString LSTR_58 1066 "Shinri Hub không thể đóng $\r$\nVui lòng đóng nó theo cách thủ công và nhấp vào Thử lại để tiếp tục."
LangString LSTR_59 1066 "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
LangString LSTR_60 1066 "Closing running Shinri Hub..."
LangString LSTR_61 1066 "Tùy biến"
LangString LSTR_62 1066 "Hủy bỏ"
LangString LSTR_63 1066 "Trình trợ lí sẽ cài đặt $(LSTR_76) vào thư mục sau. Để cài đặt vào một thư mục khác, hãy nhấn Duyệt để chọn thư mục. $_CLICK"
LangString LSTR_64 1066 "Thư mục đích"
LangString LSTR_65 1066 Du&yệt...
LangString LSTR_66 1066 "Chọn thư mục để cài đặt $(LSTR_76):"
LangString LSTR_67 1066 "< &Lùi"
LangString LSTR_68 1066 "&Cài đặt"
LangString LSTR_69 1066 "Nhấn “Cài đặt” để bắt đầu việc cài đặt."
LangString LSTR_70 1066 "&Hiện chi tiết"
LangString LSTR_71 1066 "Hoàn thành"
LangString LSTR_72 1066 "&Tiến >"
LangString LSTR_73 1066 "Nhấn Tiến để tiếp tục."
LangString LSTR_74 1066 " "
LangString LSTR_75 1066 Đón&g
LangString LSTR_76 1066 "Shinri Hub"


; LANG: 1054
LangString LSTR_0 1054 "Shinri Hub 26.11.23196"
LangString LSTR_1 1054 "$(LSTR_2) ติดตั้ง"
LangString LSTR_2 1054 "Shinri Hub"
LangString LSTR_3 1054 "มีพื้นที่เหลือ: "
LangString LSTR_4 1054 "ต้องการพื้นที่: "
LangString LSTR_5 1054 "ไม่สามารถเขียน: "
LangString LSTR_6 1054 คัดลอกผิดพลาด
LangString LSTR_7 1054 "คัดลอกไปยัง "
LangString LSTR_8 1054 "ไม่สามารถหาสัญลักษณ์ได้: "
LangString LSTR_9 1054 "ไม่สามารถโหลดได้: "
LangString LSTR_10 1054 "สร้างแฟ้ม: "
LangString LSTR_11 1054 "สร้างชอร์ตคัท: "
LangString LSTR_13 1054 "ลบไฟล์: "
LangString LSTR_14 1054 "ลบตอนรีบูท: "
LangString LSTR_15 1054 "มีปัญหาสร้างไฟล์ชอร์ตคัท: "
LangString LSTR_17 1054 "มีปัญหาในการคลายข้อมูล! เกิดข้อผิดพลาดจากตัวติดตั้ง?"
LangString LSTR_20 1054 "รันไฟล์: "
LangString LSTR_21 1054 "แตกไฟล์: "
LangString LSTR_22 1054 "แตกไฟล์: เกิดปัญหาในการเขียนไฟล์"
LangString LSTR_23 1054 "ตัวติดตั้งมีปัญหา: รหัส opcode ผิดพลาด"
LangString LSTR_24 1054 "ไม่มี OLE สำหรับ: "
LangString LSTR_25 1054 "แฟ้มทีติดตั้ง: "
LangString LSTR_26 1054 "ลบแฟ้ม: "
LangString LSTR_27 1054 "เปลี่ยนชื่อตอนรีบูท: "
LangString LSTR_28 1054 "เปลี่ยนชื่อ: "
LangString LSTR_29 1054 "ข้าม: "
LangString LSTR_30 1054 คัดลอกรายละเอียดลงคลิปบอร์ด
LangString LSTR_32 1054 B
LangString LSTR_33 1054 " K"
LangString LSTR_34 1054 " M"
LangString LSTR_35 1054 " G"
LangString LSTR_36 1054 "ไม่สามารถเปิดไฟล์สำหรับเขียนได้: $\r$\n$\r$\n$0$\r$\n$\r$\nกด ยกเลิก เพื่อหยุดการติดตั้ง,$\r$\nลองอีกครั้ง เพื่อลองอีกครั้ง, หรือ$\r$\nเพิกเฉย เพื่อข้ามไฟล์นี้."
LangString LSTR_37 1054 เลือกที่ที่ต้องการติดตั้ง
LangString LSTR_38 1054 "เลือกแผ้มที่ต้องการติดตั้ง $(LSTR_76)."
LangString LSTR_39 1054 กำลังติดตั้ง
LangString LSTR_40 1054 "โปรดรอในขณะที่ $(LSTR_76) กำลังถูกติดตั้ง"
LangString LSTR_41 1054 การติดตั้งเสร็จสิ้น
LangString LSTR_42 1054 การติดตั้งเสร็จสมบูรณ์
LangString LSTR_43 1054 การติดตั้งถูกยกเลิก
LangString LSTR_44 1054 การติดตั้งไม่เสร็จสมบูรณ์
LangString LSTR_45 1054 &เสร็จสิ้น
LangString LSTR_46 1054 0
LangString LSTR_47 1054 "การติดตั้งอัติโนมัติของ  $(LSTR_76) กำลังเสร็จสิ้น"
LangString LSTR_48 1054 "MS Shell Dlg"
LangString LSTR_49 1054 "เครื่องคอมพิวเตอร์ของคุณจำเป็นต้องรีสตารท์เพื่อการติดตั้งของ $(LSTR_76) จะเรียบร้อย, คุณต้องการจะ รีบูท เดี๋ยวนี้ไหม?"
LangString LSTR_50 1054 "รีบูท เดี๋ยวนี้"
LangString LSTR_51 1054 "ฉันต้องการ รีบูทด้วยตนเอง ทีหลัง"
LangString LSTR_52 1054 "$(LSTR_76) ได้ถูกติดตั้งลงในเครื่องคอมพิวเตอร์ของคุณแล้ว$\r$\n$\r$\nกด เสร็จสิ้นเพื่อปิดตัวติดตั้งอัติโนมัติ"
LangString LSTR_53 1054 "&รัน $(LSTR_76)"
LangString LSTR_54 1054 8
LangString LSTR_55 1054 "Windows 7 and above is required"
LangString LSTR_56 1054 "64-bit Windows is required"
LangString LSTR_57 1054 "Failed to uninstall old application files. Please try running the installer again."
LangString LSTR_58 1054 "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
LangString LSTR_59 1054 "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
LangString LSTR_60 1054 "Closing running Shinri Hub..."
LangString LSTR_61 1054 กำหนดเอง
LangString LSTR_62 1054 ยกเลิก
LangString LSTR_63 1054 "ตัวติดตั้งจะทำการติดตั้ง $(LSTR_76) ลงในแฟ้มดังต่อไปนี้, ถ้าต้องการติดตั้งลงในแฟ้มอื่น, กด เรียกดูและเลือกแฟ้มอื่น $_CLICK"
LangString LSTR_64 1054 แฟ้มที่ต้องการติดตั้ง
LangString LSTR_65 1054 เ&รียกดู...
LangString LSTR_66 1054 "เลือกแฟ้มที่ต้องการติดตั้ง $(LSTR_76) ใน:"
LangString LSTR_67 1054 "< &กลับไป"
LangString LSTR_68 1054 &ติดตั้ง
LangString LSTR_69 1054 "กด  ติดตั้ง เพื่อทำการติดตั้ง"
LangString LSTR_70 1054 &รายละเอียด
LangString LSTR_71 1054 เสร็จสิ้นแล้ว
LangString LSTR_72 1054 "&ต่อไป >"
LangString LSTR_73 1054 "กด ต่อไป เพื่อเริ่มระบบอัติโนมัติ"
LangString LSTR_74 1054 " "
LangString LSTR_75 1054 &ปิด
LangString LSTR_76 1054 "Shinri Hub"


; LANG: 1025
LangString LSTR_0 1025 "Shinri Hub 26.11.23196"
LangString LSTR_1 1025 "تنصيب $(LSTR_2)"
LangString LSTR_2 1025 "Shinri Hub"
LangString LSTR_3 1025 "المساحة المتوفرة: "
LangString LSTR_4 1025 "المساحة المطلوبة: "
LangString LSTR_5 1025 "لا يستطيع الكتابة: "
LangString LSTR_6 1025 "فشل النسخ"
LangString LSTR_7 1025 "نسخ إلى"
LangString LSTR_8 1025 "لم يتمكن من إيجاد الرمز :"
LangString LSTR_9 1025 "لم يتمكن من تحميل :"
LangString LSTR_10 1025 "إنشاء مجلد"
LangString LSTR_11 1025 "إنشاء إختصار: "
LangString LSTR_13 1025 "حذف ملف: "
LangString LSTR_14 1025 "حذف بعد إعادة التشغيل: "
LangString LSTR_15 1025 "خلل أثناء إنشاء إختصار: "
LangString LSTR_17 1025 "خلل أثناء فتح البيانات المضغوطة! منصب تالف؟"
LangString LSTR_20 1025 "تنفيذ: "
LangString LSTR_21 1025 "إستخراج: "
LangString LSTR_22 1025 "إستخراج: خلل أثناء الكتابة إلى ملف "
LangString LSTR_23 1025 "المنصّب تالف: شفرة غير صالحة"
LangString LSTR_24 1025 "لا توجد OLE لـِ: "
LangString LSTR_25 1025 "مجلد الإخراج: "
LangString LSTR_26 1025 "إزالة مجلد: "
LangString LSTR_27 1025 "إعادة تسمية بعد إعادة التشغيل: "
LangString LSTR_28 1025 "إعادة تسمية: "
LangString LSTR_29 1025 "تخطى: "
LangString LSTR_30 1025 "نسخ التفاصيل إلى الذاكرة"
LangString LSTR_32 1025 بايت
LangString LSTR_33 1025 " كيلو"
LangString LSTR_34 1025 " ميغا"
LangString LSTR_35 1025 " جيغا"
LangString LSTR_36 1025 "حدث خلل أثناء فتح ملف للكتابة: $\r$\n$\t\$\"$0\$\"$\r$\nإضغط إلغاء لإلغاء التنصيب،$\r$\nمحاولة لإعادة محاولة كتابة الملف،$\r$\n تجاهل لتخطي الملف"
LangString LSTR_37 1025 "إختر موقع التنصيب"
LangString LSTR_38 1025 "إختر المجلد المراد تنصيب $(LSTR_76) فيه."
LangString LSTR_39 1025 تنصيب
LangString LSTR_40 1025 "الرجاء الإنتظار أثناء تنصيب $(LSTR_76)."
LangString LSTR_41 1025 "إنتهى التنصيب"
LangString LSTR_42 1025 "إنتهت عملية التنصيب بنجاح."
LangString LSTR_43 1025 "إلغاء التنصيب"
LangString LSTR_44 1025 "لم ينتهي التنصيب بنجاح."
LangString LSTR_45 1025 &إنهاء
LangString LSTR_46 1025 1
LangString LSTR_47 1025 "إنهاء مرشد إعداد $(LSTR_76)"
LangString LSTR_48 1025 "MS Shell Dlg"
LangString LSTR_49 1025 "يجب إعادة تشغيل الجهاز لإنهاء تنصيب $(LSTR_76). هل تريد إعادة التشغيل الآن؟"
LangString LSTR_50 1025 "أعد التشغيل الآن"
LangString LSTR_51 1025 "أرغب في إعادة تشغيل الجهاز في وقت لاحق"
LangString LSTR_52 1025 "لقد تم تنصيب $(LSTR_76) على الجهاز$\r$\n$\r$\nإضغط إنهاء لإغلاق مرشد الإعداد."
LangString LSTR_53 1025 "&شغل $(LSTR_76)"
LangString LSTR_54 1025 8
LangString LSTR_55 1025 "Windows 7 and above is required"
LangString LSTR_56 1025 "64-bit Windows is required"
LangString LSTR_57 1025 "‫لقد فشل إلغاء التثبيت ملفات التطبيق. يُرجى محاولة تشغيل أداة التثبيت مرة أخرى."
LangString LSTR_58 1025 "‫لم يتمكن من إغلاق Shinri Hub $\r$\nيُرجى إغلاقه يدويا ثم الضغط مرة أخرى على إعادة المحاولة للاستمرار.‏"
LangString LSTR_59 1025 "‫إن Shinri Hub يعمل حاليا.$\r$\n‫يُرجى الضغط على « موافق » لغلقه.$\r$\n‫إذا لم يُغلَق، يُرجى محاولة إغلاقه يدويا."
LangString LSTR_60 1025 "Closing running Shinri Hub..."
LangString LSTR_61 1025 إختياري
LangString LSTR_62 1025 إلغاء
LangString LSTR_63 1025 "سيتم تنصيب $(LSTR_76) في المجلد التالي. للتنصيب في مجلد آخر، إضغط عرض وإختر مجلد آخر. $_CLICK"
LangString LSTR_64 1025 "مجلد الهدف"
LangString LSTR_65 1025 &عرض...
LangString LSTR_66 1025 "إختر المجلد لتنصيب $(LSTR_76) فيه:"
LangString LSTR_67 1025 "< ال&سابق"
LangString LSTR_68 1025 &تنصيب
LangString LSTR_69 1025 "إضغط على تنصيب لتشغيل التنصيب."
LangString LSTR_70 1025 "إ&ظهار التفاصيل"
LangString LSTR_71 1025 إنتهى
LangString LSTR_72 1025 "ال&تالي >"
LangString LSTR_73 1025 "إضغط على التالي للمتابعة."
LangString LSTR_74 1025 " "
LangString LSTR_75 1025 إ&غلاق
LangString LSTR_76 1025 "Shinri Hub"


; LANG: 1058
LangString LSTR_0 1058 "Shinri Hub 26.11.23196"
LangString LSTR_1 1058 "$(LSTR_2) Встановлення"
LangString LSTR_2 1058 "Shinri Hub"
LangString LSTR_3 1058 "Доступно місця: "
LangString LSTR_4 1058 "Необхідно місця: "
LangString LSTR_5 1058 "Неможливо записати: "
LangString LSTR_6 1058 "Не вдалося копіювати"
LangString LSTR_7 1058 "Копіювання в "
LangString LSTR_8 1058 "Неможливо знайти символ: "
LangString LSTR_9 1058 "Неможливо завантажити: "
LangString LSTR_10 1058 "Створення теки: "
LangString LSTR_11 1058 "Створення ярлика: "
LangString LSTR_13 1058 "Видалення файлу: "
LangString LSTR_14 1058 "Видалення після перезавантаження: "
LangString LSTR_15 1058 "Помилка створення ярлика: "
LangString LSTR_17 1058 "Неможливо витягнути дані. Можливо пошкоджений дистрибутив."
LangString LSTR_20 1058 "Виконання: "
LangString LSTR_21 1058 "Витягнення: "
LangString LSTR_22 1058 "Витягнення: помилка запису файла"
LangString LSTR_23 1058 "Дистрибутив пошкоджений: помилковий код відповіді"
LangString LSTR_24 1058 "Немає OLE для: "
LangString LSTR_25 1058 "Тека призначення: "
LangString LSTR_26 1058 "Видалення теки: "
LangString LSTR_27 1058 "Переіменування після перезаванаження: "
LangString LSTR_28 1058 "Перейменування: "
LangString LSTR_29 1058 "Пропущено: "
LangString LSTR_30 1058 "Копіювати деталі у буфер обміну"
LangString LSTR_32 1058 б
LangString LSTR_33 1058 " К"
LangString LSTR_34 1058 " М"
LangString LSTR_35 1058 " Г"
LangString LSTR_36 1058 "Неможливо відкрити файл для запису: $\r$\n$\t$\"$0$\"$\r$\nНатисніть Перервати для того, щоб перервати встановлення,$\r$\nПовторити для того, щоб повторити спробу запису у файл, чи$\r$\nІгнорувати для того, щоб пропустити цю дію"
LangString LSTR_37 1058 "Оберіть теку встановлення"
LangString LSTR_38 1058 "Оберіть теку для встановлення $(LSTR_76)."
LangString LSTR_39 1058 "Копіювання файлів"
LangString LSTR_40 1058 "Зачекайте, доки програма встановлення скопіює всі необхідні файли $(LSTR_76)."
LangString LSTR_41 1058 "Встановлення завершено"
LangString LSTR_42 1058 "Встановлення успішно завершено."
LangString LSTR_43 1058 "Встановлення перервана"
LangString LSTR_44 1058 "Встановлення не було успішно завершено."
LangString LSTR_45 1058 &Кінець
LangString LSTR_46 1058 0
LangString LSTR_47 1058 "Завершення майстра встановлення $(LSTR_76)"
LangString LSTR_48 1058 "MS Shell Dlg"
LangString LSTR_49 1058 "Для того, щоб завершити встановлення $(LSTR_76) Ваш комп'ютер повинен перезавантажитися. Зробити це зараз?"
LangString LSTR_50 1058 "Перезавантажитися зараз"
LangString LSTR_51 1058 "Я хочу перезавантажитися власноруч пізніше"
LangString LSTR_52 1058 "$(LSTR_76) встановлено на ваш комп'ютер.$\r$\n$\r$\nНатисніть Кінець для виходу."
LangString LSTR_53 1058 "&Запустити $(LSTR_76)"
LangString LSTR_54 1058 8
LangString LSTR_55 1058 "Windows 7 and above is required"
LangString LSTR_56 1058 "64-bit Windows is required"
LangString LSTR_57 1058 "Не вдалось видалити старі файли застосунку. Будь ласка, спробуйте запустити встановлювач знов."
LangString LSTR_58 1058 "Неможливо закрити Shinri Hub. $\r$\nЗакрийте його вручну та натисніть Повторити, щоб продовжити."
LangString LSTR_59 1058 "Shinri Hub запущено.$\r$\nКлацніть ОК для завершення.$\r$\nЯкщо завершення не вдалось — спробуйте вручну."
LangString LSTR_60 1058 "Closing running Shinri Hub..."
LangString LSTR_61 1058 "По вибору"
LangString LSTR_62 1058 Скасувати
LangString LSTR_63 1058 "Программа встановить $(LSTR_76) у вказану теку. Для того, щоб встановити программу в іншу теку, натисніть Огляд і вкажіть потрібну теку. $_CLICK"
LangString LSTR_64 1058 "Каталог призначення"
LangString LSTR_65 1058 О&гляд...
LangString LSTR_66 1058 "Виберіть теку для встановлення $(LSTR_76):"
LangString LSTR_67 1058 "< &Назад"
LangString LSTR_68 1058 &Встановити
LangString LSTR_69 1058 "Натисніть Встановити для початку процессу встановлення"
LangString LSTR_70 1058 Подробиці
LangString LSTR_71 1058 Завершено
LangString LSTR_72 1058 "&Далі >"
LangString LSTR_73 1058 "Натисніть Далі щоб продовжити"
LangString LSTR_74 1058 " "
LangString LSTR_75 1058 &Закрити
LangString LSTR_76 1058 "Shinri Hub"


; LANG: 2070
LangString LSTR_0 2070 "Shinri Hub 26.11.23196"
LangString LSTR_1 2070 "Instalação de $(LSTR_2)"
LangString LSTR_2 2070 "Shinri Hub"
LangString LSTR_3 2070 "Espaço disponível: "
LangString LSTR_4 2070 "Espaço necessário: "
LangString LSTR_5 2070 "Não foi possível escrever: "
LangString LSTR_6 2070 "Falha ao copiar"
LangString LSTR_7 2070 "Copiar para "
LangString LSTR_8 2070 "Símbolo não encontrado: "
LangString LSTR_9 2070 "Não foi possível carregar: "
LangString LSTR_10 2070 "Criando diretório: "
LangString LSTR_11 2070 "Criando atalho: "
LangString LSTR_13 2070 "Apagando ficheiro: "
LangString LSTR_14 2070 "Apagar ao reiniciar: "
LangString LSTR_15 2070 "Erro ao criar atalho: "
LangString LSTR_17 2070 "Erro ao descomprimir dados! Instalador corrompido?"
LangString LSTR_20 2070 "Executando: "
LangString LSTR_21 2070 "Extraindo: "
LangString LSTR_22 2070 "Extraindo: erro ao escrever ficheiro "
LangString LSTR_23 2070 "Instalador corrompido: código de operação inválido"
LangString LSTR_24 2070 "Sem OLE para: "
LangString LSTR_25 2070 "Pasta de destino: "
LangString LSTR_26 2070 "Removendo pasta: "
LangString LSTR_27 2070 "Renomear ao reiniciar: "
LangString LSTR_28 2070 "Renomeando: "
LangString LSTR_29 2070 "Ignorado: "
LangString LSTR_30 2070 "Copiar detalhes para a Área de Transfêrencia"
LangString LSTR_32 2070 B
LangString LSTR_33 2070 " K"
LangString LSTR_34 2070 " M"
LangString LSTR_35 2070 " G"
LangString LSTR_36 2070 "Erro ao abrir ficheiro para escrita: $\r$\n$\t$\"$0$\"$\r$\nClique em Abortar para abortar a instalação,$\r$\nRepetir para tentar novamente a escrita do ficheiro, ou$\r$\nIgnorar para ignorar este ficheiro."
LangString LSTR_37 2070 "Escolha do Local da Instalação"
LangString LSTR_38 2070 "Escolha a pasta na qual deseja instalar o $(LSTR_76)."
LangString LSTR_39 2070 Instalando
LangString LSTR_40 2070 "Por favor, aguarde enquanto o $(LSTR_76) está sendo instalado."
LangString LSTR_41 2070 "Instalação Completa"
LangString LSTR_42 2070 "A instalação foi concluída com sucesso."
LangString LSTR_43 2070 "Instalação Abortada"
LangString LSTR_44 2070 "A instalação não foi concluída com sucesso."
LangString LSTR_45 2070 &Terminar
LangString LSTR_46 2070 0
LangString LSTR_47 2070 "Concluindo o Assistente de Instalação do $(LSTR_76)"
LangString LSTR_48 2070 "MS Shell Dlg"
LangString LSTR_49 2070 "O seu computador deve ser reiniciado para concluír a instalação do $(LSTR_76). Deseja reiniciar agora?"
LangString LSTR_50 2070 "Reiniciar Agora"
LangString LSTR_51 2070 "Eu quero reiniciar manualmente depois"
LangString LSTR_52 2070 "$(LSTR_76) foi instalado no seu computador.$\r$\n$\r$\nClique em Terminar para fechar este assistente."
LangString LSTR_53 2070 "&Executar $(LSTR_76)"
LangString LSTR_54 2070 8
LangString LSTR_55 2070 "Windows 7 and above is required"
LangString LSTR_56 2070 "64-bit Windows is required"
LangString LSTR_57 2070 "Falha ao desinstalar os ficheiros da aplicação antiga. Por favor, experimente correr novamente o instalador."
LangString LSTR_58 2070 "Não é possível fechar o Shinri Hub. $\r$\nPor favor, feche-o manualmente e clique em ´Tentar novamente' para continuar."
LangString LSTR_59 2070 "Shinri Hub está a correr.$\r$\nClique em OK para o encerrar.$\r$\nSe não encerrar, tente encerrá-lo manualmente."
LangString LSTR_60 2070 "Closing running Shinri Hub..."
LangString LSTR_61 2070 Personalizado
LangString LSTR_62 2070 Cancelar
LangString LSTR_63 2070 "O $(LSTR_76) será instalado na seguinte pasta. Para instalar numa pasta diferente, clique em 'Procurar...' e escolha outra pasta. $_CLICK"
LangString LSTR_64 2070 "Pasta de Destino"
LangString LSTR_65 2070 &Procurar...
LangString LSTR_66 2070 "Escolha uma pasta para instalar o $(LSTR_76):"
LangString LSTR_67 2070 "< &Anterior"
LangString LSTR_68 2070 &Instalar
LangString LSTR_69 2070 "Clique em 'Instalar' para iniciar a instalação."
LangString LSTR_70 2070 "Ver &Detalhes"
LangString LSTR_71 2070 Concluído
LangString LSTR_72 2070 "&Seguinte >"
LangString LSTR_73 2070 "Clique em 'Seguinte' para continuar."
LangString LSTR_74 2070 " "
LangString LSTR_75 2070 &Fechar
LangString LSTR_76 2070 "Shinri Hub"


; LANG: 1044
LangString LSTR_0 1044 "Shinri Hub 26.11.23196"
LangString LSTR_1 1044 "$(LSTR_2) installasjon"
LangString LSTR_2 1044 "Shinri Hub"
LangString LSTR_3 1044 "Ledig plass: "
LangString LSTR_4 1044 "Nødvendig plass: "
LangString LSTR_5 1044 "Kan ikke skrive: "
LangString LSTR_6 1044 "Kopiering mislyktes"
LangString LSTR_7 1044 "Kopier til "
LangString LSTR_8 1044 "Kunne ikke finne symbol: "
LangString LSTR_9 1044 "Kunne ikke laste: "
LangString LSTR_10 1044 "Lag mappe: "
LangString LSTR_11 1044 "Lag snarvei: "
LangString LSTR_13 1044 "Slett fil: "
LangString LSTR_14 1044 "Slett ved omstart: "
LangString LSTR_15 1044 "Feil under opprettelse av snarvei: "
LangString LSTR_17 1044 "Feil under utpakking av data! Installasjonsprogrammet kan være skadet."
LangString LSTR_20 1044 "Kjøre: "
LangString LSTR_21 1044 "Pakk ut: "
LangString LSTR_22 1044 "Pakk ut: Feil under skriving til fil "
LangString LSTR_23 1044 "Installasjonsprogrammet er skadet: ukjent kode"
LangString LSTR_24 1044 "Ingen OLE for: "
LangString LSTR_25 1044 "Ut-mappe: "
LangString LSTR_26 1044 "Fjern mappe: "
LangString LSTR_27 1044 "Gi nytt navn ved omstart: "
LangString LSTR_28 1044 "Gi nytt navn: "
LangString LSTR_29 1044 "Hoppet over: "
LangString LSTR_30 1044 "Kopier detaljer til utklippstavlen"
LangString LSTR_32 1044 B
LangString LSTR_33 1044 " K"
LangString LSTR_34 1044 " M"
LangString LSTR_35 1044 " G"
LangString LSTR_36 1044 "Feil under åpning av fil for skriving: $\r$\n$\t\$\"$0\$\"$\r$\nTrykk Avbryt for å avbryte installasjonen,$\r$\nPrøv igjen for å prøve igjen, eller$\r$\nIgnorer for å hoppe over denne filen"
LangString LSTR_37 1044 "Velg installasjonsmappe"
LangString LSTR_38 1044 "Velg hvilken mappe du vil installere $(LSTR_76) i."
LangString LSTR_39 1044 "Installasjonen pågår"
LangString LSTR_40 1044 "Vennligst vent mens $(LSTR_76) blir installert."
LangString LSTR_41 1044 "Installasjonen er ferdig"
LangString LSTR_42 1044 "Installasjonen ble fullført uten feil."
LangString LSTR_43 1044 "Installasjonen er avbrutt"
LangString LSTR_44 1044 "Installasjonen ble ikke fullført riktig."
LangString LSTR_45 1044 &Ferdig
LangString LSTR_46 1044 0
LangString LSTR_47 1044 "Avslutter $(LSTR_76) installasjonsveiviser"
LangString LSTR_48 1044 "MS Shell Dlg"
LangString LSTR_49 1044 "Du må starte datamaskinen på nytt for å fullføre installasjonen av $(LSTR_76). Vil du starte datamaskinen på nytt nå?"
LangString LSTR_50 1044 "Ja. Start datamaskinen på nytt nå"
LangString LSTR_51 1044 "Nei. Jeg vil starte datamaskinen på nytt senere"
LangString LSTR_52 1044 "$(LSTR_76) er klart til bruk på din datamskin.$\r$\n$\r$\nTrykk Ferdig for å avslutte installasjonsprogrammet."
LangString LSTR_53 1044 "&Kjør $(LSTR_76)"
LangString LSTR_54 1044 8
LangString LSTR_55 1044 "Windows 7 and above is required"
LangString LSTR_56 1044 "64-bit Windows is required"
LangString LSTR_57 1044 "Failed to uninstall old application files. Please try running the installer again."
LangString LSTR_58 1044 "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
LangString LSTR_59 1044 "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
LangString LSTR_60 1044 "Closing running Shinri Hub..."
LangString LSTR_61 1044 Egendefinert
LangString LSTR_62 1044 Avbryt
LangString LSTR_63 1044 "$(LSTR_2) vil bli installert i følgende mappe. For å velge en annen mappe, trykk Bla gjennom. $_CLICK"
LangString LSTR_64 1044 Målmappe
LangString LSTR_65 1044 "Bla &gjennom..."
LangString LSTR_66 1044 "Velg mappe du vil installere $(LSTR_2) i:"
LangString LSTR_67 1044 "< &Tilbake"
LangString LSTR_68 1044 &Installer
LangString LSTR_69 1044 "Trykk Installer for å starte installasjonen."
LangString LSTR_70 1044 "Vis &detaljer"
LangString LSTR_71 1044 Ferdig
LangString LSTR_72 1044 "&Neste >"
LangString LSTR_73 1044 "Trykk Neste for å fortsette."
LangString LSTR_74 1044 " "
LangString LSTR_75 1044 &Lukk
LangString LSTR_76 1044 "Shinri Hub"


; LANG: 1030
LangString LSTR_0 1030 "Shinri Hub 26.11.23196"
LangString LSTR_1 1030 "$(LSTR_2) Installation"
LangString LSTR_2 1030 "Shinri Hub"
LangString LSTR_3 1030 "Ledig plads: "
LangString LSTR_4 1030 "Nødvendig plads: "
LangString LSTR_5 1030 "Kan ikke skrive: "
LangString LSTR_6 1030 "Kopiering mislykkedes"
LangString LSTR_7 1030 "Kopier til "
LangString LSTR_8 1030 "Kunne ikke finde symbol: "
LangString LSTR_9 1030 "Kunne ikke indlæse: "
LangString LSTR_10 1030 "Opret mappe: "
LangString LSTR_11 1030 "Opret genvej: "
LangString LSTR_13 1030 "Slet fil: "
LangString LSTR_14 1030 "Slet ved genstart: "
LangString LSTR_15 1030 "Fejl ved oprettelse af genvej: "
LangString LSTR_17 1030 "Fejl ved udpakning af data! Beskadiget installationsguide?"
LangString LSTR_20 1030 "Kør: "
LangString LSTR_21 1030 "Udpak: "
LangString LSTR_22 1030 "Udpak: fejl ved skrivning til fil "
LangString LSTR_23 1030 "Beskadiget installationsguide: ugyldig opcode"
LangString LSTR_24 1030 "Ingen OLE for: "
LangString LSTR_25 1030 "Outputmappe: "
LangString LSTR_26 1030 "Slet mappe: "
LangString LSTR_27 1030 "Omdøb ved genstart: "
LangString LSTR_28 1030 "Omdøb: "
LangString LSTR_29 1030 "Sprunget over: "
LangString LSTR_30 1030 "Kopier detaljer til udklipsholderen"
LangString LSTR_32 1030 B
LangString LSTR_33 1030 " K"
LangString LSTR_34 1030 " M"
LangString LSTR_35 1030 " G"
LangString LSTR_36 1030 "Fejl ved skrivning af fil til skrivning: $\r$\n$\r$\n$0$\r$\n$\r$\nTryk på Annuller for at stoppe installationen,$\r$\nPrøv igen for at prøve igen, eller$\r$\nIgnorer for at springe over denne fil."
LangString LSTR_37 1030 "Vælg installationsmappe"
LangString LSTR_38 1030 "Vælg den mappe hvori du vil installere $(LSTR_76)."
LangString LSTR_39 1030 Installerer
LangString LSTR_40 1030 "Vent venligst mens $(LSTR_76) bliver installeret."
LangString LSTR_41 1030 "Installation gennemført"
LangString LSTR_42 1030 "Installationsguiden blev gennemført."
LangString LSTR_43 1030 "Installation afbrudt"
LangString LSTR_44 1030 "Installationsguiden blev ikke gennemført."
LangString LSTR_45 1030 &Afslut
LangString LSTR_46 1030 0
LangString LSTR_47 1030 "Gennemfører $(LSTR_76) installationsguiden"
LangString LSTR_48 1030 "MS Shell Dlg"
LangString LSTR_49 1030 "Din computer skal genstartes for at gennemføre installationen af $(LSTR_76). Vil du genstarte nu?"
LangString LSTR_50 1030 "Genstart nu"
LangString LSTR_51 1030 "Jeg genstarter selv på et senere tidspunkt"
LangString LSTR_52 1030 "$(LSTR_76) er blevet installeret på din computer.$\r$\n$\r$\nTryk på Afslut for at lukke installationsguiden."
LangString LSTR_53 1030 "&Kør $(LSTR_76)"
LangString LSTR_54 1030 8
LangString LSTR_55 1030 "Windows 7 og højere er nødvendig"
LangString LSTR_56 1030 "64-bit Windows er påkrævet"
LangString LSTR_57 1030 "Det lykkedes ikke at afinstallere gamle applikationsfiler. Prøv venligst at køre installationsprogrammet igen."
LangString LSTR_58 1030 "Shinri Hub kan ikke afsluttes $\r$\nLuk den manuelt og klik Prøv igen for at fortsætte"
LangString LSTR_59 1030 "Shinri Hub kører.$\r$\nKlik OK for at lukke den.$\r$\nHvis den ikke lukker, kan du prøve at lukke den manuelt."
LangString LSTR_60 1030 "Closing running Shinri Hub..."
LangString LSTR_61 1030 Brugerdefineret
LangString LSTR_62 1030 Annuller
LangString LSTR_63 1030 "Installationsguiden vil installere $(LSTR_76) i følgende mappe. For at installere i en anden mappe, tryk på Gennemse og vælg en anden mappe. $_CLICK"
LangString LSTR_64 1030 Destinationsmappe
LangString LSTR_65 1030 G&ennemse...
LangString LSTR_66 1030 "Vælg den mappe hvori $(LSTR_76) skal installeres:"
LangString LSTR_67 1030 "< &Tilbage"
LangString LSTR_68 1030 &Installer
LangString LSTR_69 1030 "Tryk på Installer for at starte installationen."
LangString LSTR_70 1030 "Vis &detaljer"
LangString LSTR_71 1030 Gennemført
LangString LSTR_72 1030 "&Næste >"
LangString LSTR_73 1030 "Tryk på Næste for at fortsætte."
LangString LSTR_74 1030 " "
LangString LSTR_75 1030 &Luk
LangString LSTR_76 1030 "Shinri Hub"


; LANG: 3082
LangString LSTR_0 3082 "Shinri Hub 26.11.23196"
LangString LSTR_1 3082 "Instalación de $(LSTR_2)"
LangString LSTR_2 3082 "Shinri Hub"
LangString LSTR_3 3082 "Espacio disponible:"
LangString LSTR_4 3082 "Espacio requerido:"
LangString LSTR_5 3082 "No pudo escribirse: "
LangString LSTR_6 3082 "Copia fallida"
LangString LSTR_7 3082 "Copiar a "
LangString LSTR_8 3082 "No se encontró simbolo: "
LangString LSTR_9 3082 "No pudo cargarse: "
LangString LSTR_10 3082 "Creando carpeta: "
LangString LSTR_11 3082 "Creando acceso directo: "
LangString LSTR_13 3082 "Borrar archivo: "
LangString LSTR_14 3082 "Borrar al reinicio: "
LangString LSTR_15 3082 "Error creando acceso directo: "
LangString LSTR_17 3082 "¡Error descomprimiendo datos! ¿Instalador corrupto?"
LangString LSTR_20 3082 "Ejecutando: "
LangString LSTR_21 3082 "Extrayendo: "
LangString LSTR_22 3082 "Extrayendo: error escribiendo al archivo "
LangString LSTR_23 3082 "Instalador corrupto: código de operación no válido"
LangString LSTR_24 3082 "Sin OLE para: "
LangString LSTR_25 3082 "Carpeta de salida: "
LangString LSTR_26 3082 "Eliminar carpeta: "
LangString LSTR_27 3082 "Renombrar al reinicio: "
LangString LSTR_28 3082 "Renombrar: "
LangString LSTR_29 3082 "Omitido: "
LangString LSTR_30 3082 "Copiar Detalles al Portapapeles"
LangString LSTR_32 3082 B
LangString LSTR_33 3082 " K"
LangString LSTR_34 3082 " M"
LangString LSTR_35 3082 " G"
LangString LSTR_36 3082 "Error abriendo archivo para escribir: $\r$\n$\r$\n$0$\r$\n$\r$\nPresione Abortar para detener la instalación,$\r$\nReintentar para probar otra vez, o$\r$\nOmitir para ignorar este archivo."
LangString LSTR_37 3082 "Elegir lugar de instalación"
LangString LSTR_38 3082 "Elija la carpeta para instalar $(LSTR_76)."
LangString LSTR_39 3082 Instalando
LangString LSTR_40 3082 "Por favor espere mientras $(LSTR_76) se instala."
LangString LSTR_41 3082 "Instalación Finalizada"
LangString LSTR_42 3082 "La instalación se ha finalizado correctamente."
LangString LSTR_43 3082 "Instalación Abortada"
LangString LSTR_44 3082 "La instalación no se terminó correctamente."
LangString LSTR_45 3082 &Terminar
LangString LSTR_46 3082 0
LangString LSTR_47 3082 "Finalizando el Asistente de Instalación de $(LSTR_76)"
LangString LSTR_48 3082 "MS Shell Dlg"
LangString LSTR_49 3082 "Su sistema debe ser reiniciado para poder finalizar la instalación de $(LSTR_76). ¿Desea reiniciar ahora?"
LangString LSTR_50 3082 "Reiniciar ahora"
LangString LSTR_51 3082 "Deseo reiniciar manualmente más tarde"
LangString LSTR_52 3082 "$(LSTR_76) ha sido instalado en su sistema.$\r$\n$\r$\nPresione Terminar para cerrar este asistente."
LangString LSTR_53 3082 "&Ejecutar $(LSTR_76)"
LangString LSTR_54 3082 8
LangString LSTR_55 3082 "Se necesita Windows 7 o una versión superior."
LangString LSTR_56 3082 "Windows de 64 bits es requerido"
LangString LSTR_57 3082 "Fallo al desinstalar archivos antiguos de la aplicación. Inténtalo de nuevo más tarde."
LangString LSTR_58 3082 "No se puede cerrar Shinri Hub. $\r$\nPor favor cierra la aplicación manualmente y haz clic en reintentar para continuar."
LangString LSTR_59 3082 "Shinri Hub está activa.$\r$\nHaz clic en Aceptar para cerrarla.$\r$\nSi no se cierra, inténtalo manualmente."
LangString LSTR_60 3082 "Closing running Shinri Hub..."
LangString LSTR_61 3082 Personalizada
LangString LSTR_62 3082 Cancelar
LangString LSTR_63 3082 "El programa de instalación instalará $(LSTR_76) en la siguiente carpeta. Para instalar en una carpeta diferente, presione Examinar y seleccione otra carpeta. $_CLICK"
LangString LSTR_64 3082 "Carpeta de Destino"
LangString LSTR_65 3082 &Examinar...
LangString LSTR_66 3082 "Seleccione la carpeta en la que instalará $(LSTR_76):"
LangString LSTR_67 3082 "< &Atrás"
LangString LSTR_68 3082 &Instalar
LangString LSTR_69 3082 "Presione Instalar para iniciar la instalación."
LangString LSTR_70 3082 "Mostrar &detalles"
LangString LSTR_71 3082 Finalizado
LangString LSTR_72 3082 "&Siguiente >"
LangString LSTR_73 3082 "Presione Siguiente para continuar."
LangString LSTR_74 3082 " "
LangString LSTR_75 3082 &Cerrar
LangString LSTR_76 3082 "Shinri Hub"


; LANG: 1035
LangString LSTR_0 1035 "Shinri Hub 26.11.23196"
LangString LSTR_1 1035 "$(LSTR_2) -ohjelman asennus"
LangString LSTR_2 1035 "Shinri Hub"
LangString LSTR_3 1035 "Tilaa vapaana: "
LangString LSTR_4 1035 "Tarvittava tila: "
LangString LSTR_5 1035 "Ei voi kirjoittaa: "
LangString LSTR_6 1035 "Kopiointi epäonnistui"
LangString LSTR_7 1035 "Kopioidaan kohteeseen "
LangString LSTR_8 1035 "Symbolia ei löytynyt: "
LangString LSTR_9 1035 "Ei voitu ladata: "
LangString LSTR_10 1035 "Luo kansio: "
LangString LSTR_11 1035 "Luo pikakuvake: "
LangString LSTR_13 1035 "Poista: "
LangString LSTR_14 1035 "Poista käynnistyksen yhteydessä: "
LangString LSTR_15 1035 "Virhe luotaessa pikakuvaketta: "
LangString LSTR_17 1035 "Pakettia ei voitu purkaa. Korruptoitunut asennusohjelma?"
LangString LSTR_20 1035 "Suorita: "
LangString LSTR_21 1035 "Pura: "
LangString LSTR_22 1035 "Pura: tiedostoon ei voitu kirjoittaa "
LangString LSTR_23 1035 "Asennuspaketti on vioittunut: virheellinen opcode"
LangString LSTR_24 1035 "Ei OLEa: "
LangString LSTR_25 1035 "Kansio: "
LangString LSTR_26 1035 "Poista kansio: "
LangString LSTR_27 1035 "Muuta nimi uudelleenkäynnistyksen yhteydessä: "
LangString LSTR_28 1035 "Muuta nimi: "
LangString LSTR_29 1035 "Ohitettiin: "
LangString LSTR_30 1035 "Kopioi tiedot leikepöydälle"
LangString LSTR_32 1035 t
LangString LSTR_33 1035 " k"
LangString LSTR_34 1035 " M"
LangString LSTR_35 1035 " G"
LangString LSTR_36 1035 "Tiedostoon ei voitu kirjoittaa: $\r$\n$\t$\"$0$\"$\r$\nLopeta asennus valitsemalla Hylkää,$\r$\nyritä uudelleen valitsemalla Uudelleen, tai$\r$\nohita tiedosto valitsemalla Ohita"
LangString LSTR_37 1035 "Valitse asennuskansio"
LangString LSTR_38 1035 "Valitse kansio, johon haluat asentaa $(LSTR_76) -ohjelman."
LangString LSTR_39 1035 Asennetaan
LangString LSTR_40 1035 "Odota... $(LSTR_76) asennetaan..."
LangString LSTR_41 1035 "Asennus valmis."
LangString LSTR_42 1035 "Asennus onnistui."
LangString LSTR_43 1035 "Asennus keskeytyi."
LangString LSTR_44 1035 "Asennus ei onnistunut."
LangString LSTR_45 1035 &Valmis
LangString LSTR_46 1035 0
LangString LSTR_47 1035 "$(LSTR_76) -ohjelman asennus on valmis"
LangString LSTR_48 1035 "MS Shell Dlg"
LangString LSTR_49 1035 "$(LSTR_76) -ohjelman asennuksen viimeisteleminen edellyttää tietokoneen uudelleenkäynnistämistä. Haluatko käynnistää tietokoneen uudelleen nyt?"
LangString LSTR_50 1035 "Käynnistä uudelleen nyt"
LangString LSTR_51 1035 "Käynnistän uudelleen myöhemmin"
LangString LSTR_52 1035 "$(LSTR_76) on asennettu tietokoneelle.$\r$\n$\r$\nValitse Valmis sulkeaksesi asennusohjelman."
LangString LSTR_53 1035 "Käynnistä $(LSTR_76)"
LangString LSTR_54 1035 8
LangString LSTR_55 1035 "Windows 7 tai uudempi vaaditaan"
LangString LSTR_56 1035 "64-bittinen Windows on pakollinen"
LangString LSTR_57 1035 "Vanhojen sovellustiedostojen poisto epäonnistui. Yritä ajaa asennusohjelma uudelleen."
LangString LSTR_58 1035 "Shinri Hub ei voi sulkea. $\r$\nSulje se itse ja napsauta Yritä uudelleen jatkaaksesi."
LangString LSTR_59 1035 "Shinri Hub on käynnissä.$\r$\nValitse OK sulkeaksesi sen.$\r$\nJos se ei sulkeudu, yritä sulkea se manuaalisesti."
LangString LSTR_60 1035 "Closing running Shinri Hub..."
LangString LSTR_61 1035 Oma
LangString LSTR_62 1035 Peruuta
LangString LSTR_63 1035 "$(LSTR_76) -ohjelma asennetaan seuraavaan kansioon. Jos haluat asentaa sen johonkin muuhun kansioon, valitse Selaa, ja valitse toinen kansio. $_CLICK"
LangString LSTR_64 1035 Kohdekansio
LangString LSTR_65 1035 S&elaa...
LangString LSTR_66 1035 "Valitse kansio, johon haluat asentaa ohjelman $(LSTR_76):"
LangString LSTR_67 1035 "< &Takaisin"
LangString LSTR_68 1035 &Asenna
LangString LSTR_69 1035 "Valitse Asenna aloittaaksesi asennuksen."
LangString LSTR_70 1035 "&Näytä tiedot"
LangString LSTR_71 1035 Valmis
LangString LSTR_72 1035 "&Seuraava >"
LangString LSTR_73 1035 "Valitse Seuraava jatkaaksesi."
LangString LSTR_74 1035 " "
LangString LSTR_75 1035 &Sulje
LangString LSTR_76 1035 "Shinri Hub"


; LANG: 1043
LangString LSTR_0 1043 "Shinri Hub 26.11.23196"
LangString LSTR_1 1043 $(LSTR_2)-installatie
LangString LSTR_2 1043 "Shinri Hub"
LangString LSTR_3 1043 "Beschikbare ruimte: "
LangString LSTR_4 1043 "Vereiste ruimte: "
LangString LSTR_5 1043 "Kon niet schrijven: "
LangString LSTR_6 1043 "Kopiëren mislukt"
LangString LSTR_7 1043 "Kopiëren naar "
LangString LSTR_8 1043 "Kon symbool niet vinden: "
LangString LSTR_9 1043 "Kon niet laden: "
LangString LSTR_10 1043 "Map maken: "
LangString LSTR_11 1043 "Snelkoppeling maken: "
LangString LSTR_13 1043 "Bestand verwijderen: "
LangString LSTR_14 1043 "Verwijderen na opnieuw opstarten: "
LangString LSTR_15 1043 "Fout bij maken snelkoppeling: "
LangString LSTR_17 1043 "Fout bij uitpakken van gegevens! Gegevens beschadigd?"
LangString LSTR_20 1043 "Uitvoeren: "
LangString LSTR_21 1043 "Uitpakken: "
LangString LSTR_22 1043 "Uitpakken: fout bij schrijven naar bestand "
LangString LSTR_23 1043 "Installatieprogramma beschadigd: ongeldige opcode"
LangString LSTR_24 1043 "Geen OLE voor: "
LangString LSTR_25 1043 "Uitvoermap: "
LangString LSTR_26 1043 "Map verwijderen: "
LangString LSTR_27 1043 "Hernoemen na opnieuw opstarten: "
LangString LSTR_28 1043 "Hernoemen: "
LangString LSTR_29 1043 "Overgeslagen: "
LangString LSTR_30 1043 "Details kopiëren naar klembord"
LangString LSTR_32 1043 B
LangString LSTR_33 1043 " K"
LangString LSTR_34 1043 " M"
LangString LSTR_35 1043 " G"
LangString LSTR_36 1043 "Fout bij het schrijven naar bestand: $\r$\n$\r$\n$0$\r$\n$\r$\nKlik op Afbreken om de installatie te stoppen,$\r$\nOpnieuw om het opnieuw te proberen of$\r$\nNegeren om dit bestand over te slaan."
LangString LSTR_37 1043 "Installatielocatie kiezen"
LangString LSTR_38 1043 "Kies de map waarin u $(LSTR_76) wilt installeren."
LangString LSTR_39 1043 "Bezig met installeren"
LangString LSTR_40 1043 "Een ogenblik geduld terwijl $(LSTR_76) wordt geïnstalleerd."
LangString LSTR_41 1043 "Installatie voltooid"
LangString LSTR_42 1043 "De installatie is succesvol voltooid."
LangString LSTR_43 1043 "Installatie afgebroken"
LangString LSTR_44 1043 "De installatie is niet voltooid."
LangString LSTR_45 1043 &Voltooien
LangString LSTR_46 1043 0
LangString LSTR_47 1043 "Voltooien van de $(LSTR_76)-installatiewizard"
LangString LSTR_48 1043 "MS Shell Dlg"
LangString LSTR_49 1043 "Uw systeem moet opnieuw worden opgestart om de installatie van $(LSTR_76) te voltooien. Wilt u nu herstarten?"
LangString LSTR_50 1043 "Nu herstarten"
LangString LSTR_51 1043 "Ik wil later handmatig herstarten"
LangString LSTR_52 1043 "$(LSTR_76) is geïnstalleerd op uw systeem.$\r$\n$\r$\nKlik op Voltooien om deze wizard te sluiten."
LangString LSTR_53 1043 "$(LSTR_76) &starten"
LangString LSTR_54 1043 8
LangString LSTR_55 1043 "Windows 7 en hoger is vereist"
LangString LSTR_56 1043 "64-bit Windows is vereist"
LangString LSTR_57 1043 "Het deïnstalleren van oude applicatiebestanden is mislukt. Voer het installatiebestand opnieuw uit om te proberen dit probleem te verhelpen."
LangString LSTR_58 1043 "Shinri Hub kan niet automatisch worden afgesloten. $\r$\nSluit zelf de Shinri Hub app en klik vervolgens op ‘Opnieuw proberen’."
LangString LSTR_59 1043 "Shinri Hub is nog actief.$\r$\nKlik op OK om de applicatie te sluiten.$\r$\nAls de applicatie vervolgens nog niet sluit, probeer dan om de applicatie zelf te sluiten."
LangString LSTR_60 1043 "Closing running Shinri Hub..."
LangString LSTR_61 1043 Aangepast
LangString LSTR_62 1043 Annuleren
LangString LSTR_63 1043 "Setup zal $(LSTR_76) in de volgende map installeren. Klik op Bladeren als u $(LSTR_76) in een andere map wilt installeren en selecteer deze. $_CLICK"
LangString LSTR_64 1043 Installatiemap
LangString LSTR_65 1043 &Bladeren...
LangString LSTR_66 1043 "Selecteer de map om $(LSTR_76) in te installeren:"
LangString LSTR_67 1043 "< V&orige"
LangString LSTR_68 1043 &Installeren
LangString LSTR_69 1043 "Klik op Installeren om de installatie te beginnen."
LangString LSTR_70 1043 "&Details tonen"
LangString LSTR_71 1043 Voltooid
LangString LSTR_72 1043 "&Volgende >"
LangString LSTR_73 1043 "Klik op Volgende om verder te gaan."
LangString LSTR_74 1043 " "
LangString LSTR_75 1043 &Afsluiten
LangString LSTR_76 1043 "Shinri Hub"


; LANG: 1053
LangString LSTR_0 1053 "Shinri Hub 26.11.23196"
LangString LSTR_1 1053 "$(LSTR_2) Installation"
LangString LSTR_2 1053 "Shinri Hub"
LangString LSTR_3 1053 "Utrymme tillgängligt: "
LangString LSTR_4 1053 "Utrymme som behövs: "
LangString LSTR_5 1053 "Kan inte skriva: "
LangString LSTR_6 1053 "Kopiering misslyckades"
LangString LSTR_7 1053 "Kopiera till "
LangString LSTR_8 1053 "Kunde inte hitta symbol: "
LangString LSTR_9 1053 "Kunde inte ladda: "
LangString LSTR_10 1053 "Skapa katalog: "
LangString LSTR_11 1053 "Skapa genväg: "
LangString LSTR_13 1053 "Radera fil: "
LangString LSTR_14 1053 "Radera vid omstart: "
LangString LSTR_15 1053 "Fel vid skapande av genväg: "
LangString LSTR_17 1053 "Fel vid uppackning av data! Skadat installationspaket?"
LangString LSTR_20 1053 "Kör: "
LangString LSTR_21 1053 "Extrahera: "
LangString LSTR_22 1053 "Extrahera: fel vid skrivning till fil "
LangString LSTR_23 1053 "Installationspaket skadat: ogiltig opcode"
LangString LSTR_24 1053 "Ingen OLE för: "
LangString LSTR_25 1053 "Målkatalog: "
LangString LSTR_26 1053 "Ta bort katalog: "
LangString LSTR_27 1053 "Döp om vid omstart: "
LangString LSTR_28 1053 "Döp om: "
LangString LSTR_29 1053 "Ignorerad: "
LangString LSTR_30 1053 "Kopiera detaljinformation till klippbordet"
LangString LSTR_32 1053 B
LangString LSTR_33 1053 " K"
LangString LSTR_34 1053 " M"
LangString LSTR_35 1053 " G"
LangString LSTR_36 1053 "Fel vid Öppning av fil för skrivning: $\r$\n$\t$\"$0$\"$\r$\nKlicka på avbryt för att avbryta installationen,$\r$\nförsök igen för att försöka skriva till filen igen, eller$\r$\nIgnorera för att skippa denna fil"
LangString LSTR_37 1053 "Välj installationsväg"
LangString LSTR_38 1053 "Välj katalog att installera $(LSTR_76) i."
LangString LSTR_39 1053 Installerar
LangString LSTR_40 1053 "Var vänlig vänta medan $(LSTR_76) installeras."
LangString LSTR_41 1053 "Installationen är klar"
LangString LSTR_42 1053 "Guiden avslutades korrekt."
LangString LSTR_43 1053 "Installationen avbröts"
LangString LSTR_44 1053 "Guiden genomfördes inte korrekt."
LangString LSTR_45 1053 &Slutför
LangString LSTR_46 1053 0
LangString LSTR_47 1053 "Avslutar installationsguiden för $(LSTR_76)."
LangString LSTR_48 1053 "MS Shell Dlg"
LangString LSTR_49 1053 "Din dator måste startas om för att fullborda installationen av $(LSTR_76). Vill du starta om nu?"
LangString LSTR_50 1053 "Starta om nu"
LangString LSTR_51 1053 "Jag vill starta om själv senare"
LangString LSTR_52 1053 "$(LSTR_76) har installerats på din dator.$\r$\n$\r$\nKlicka på Slutför för att avsluta guiden."
LangString LSTR_53 1053 "&Kör $(LSTR_76)"
LangString LSTR_54 1053 8
LangString LSTR_55 1053 "Windows 7 eller högre krävs"
LangString LSTR_56 1053 "64-bitars Windows krävs"
LangString LSTR_57 1053 "Det gick inte att avinstallera gamla programfiler. Försök att köra installationsprogrammet igen."
LangString LSTR_58 1053 "Shinri Hub går inte att stängas. $\r$\nVänligen stäng det manuellt och klicka på Försök igen för att fortsätta."
LangString LSTR_59 1053 "Shinri Hub är igång.$\r$\nKlicka på OK för att stänga det.$\r$\nOm det inte stängs, försök att stänga det manuellt."
LangString LSTR_60 1053 "Closing running Shinri Hub..."
LangString LSTR_61 1053 Valfri
LangString LSTR_62 1053 Avbryt
LangString LSTR_63 1053 "Guiden kommer att installera $(LSTR_76) i följande katalog. För att installera i en annan katalog, klicka Bläddra och välj en alternativ katalog. $_CLICK"
LangString LSTR_64 1053 Målkatalog
LangString LSTR_65 1053 B&läddra...
LangString LSTR_66 1053 "Välj katalog att installera $(LSTR_76) i:"
LangString LSTR_67 1053 "< &Tillbaka"
LangString LSTR_68 1053 &Installera
LangString LSTR_69 1053 "Klicka på Installera för att starta installationen."
LangString LSTR_70 1053 "Visa &detaljer"
LangString LSTR_71 1053 Slutförd
LangString LSTR_72 1053 "&Nästa >"
LangString LSTR_73 1053 "Klicka på Nästa för att fortsätta."
LangString LSTR_74 1053 " "
LangString LSTR_75 1053 &Stäng
LangString LSTR_76 1053 "Shinri Hub"


; LANG: 1055
LangString LSTR_0 1055 "Shinri Hub 26.11.23196"
LangString LSTR_1 1055 "$(LSTR_2) Kurulumu"
LangString LSTR_2 1055 "Shinri Hub"
LangString LSTR_3 1055 "Kullanılabilir boş alan: "
LangString LSTR_4 1055 "Gereken boş alan: "
LangString LSTR_5 1055 "Yazılamadı: "
LangString LSTR_6 1055 "Kopyalama başarısız oldu"
LangString LSTR_7 1055 "Kayıt: "
LangString LSTR_8 1055 "Simge bulunamadı: "
LangString LSTR_9 1055 "Yüklenemedi: "
LangString LSTR_10 1055 "Dizin oluştur: "
LangString LSTR_11 1055 "Kısayol oluştur: "
LangString LSTR_13 1055 "Dosya sil: "
LangString LSTR_14 1055 "Açılışta sil: "
LangString LSTR_15 1055 "Kısayol oluşturulurken hata meydana geldi: "
LangString LSTR_17 1055 "Veriyi açarken hata meydana geldi! Acaba kurulum uygulaması mı bozuk?"
LangString LSTR_20 1055 "Çalıştır: "
LangString LSTR_21 1055 "Aç: "
LangString LSTR_22 1055 "Açma: Dosyaya yazarken hata meydana geldi "
LangString LSTR_23 1055 "Kurulum bozuk: Geçersiz kod"
LangString LSTR_24 1055 "OLE yok: "
LangString LSTR_25 1055 "Çıktı dizini: "
LangString LSTR_26 1055 "Dizini sil: "
LangString LSTR_27 1055 "Açılışta adını değiştir: "
LangString LSTR_28 1055 "Ad değiştir: "
LangString LSTR_29 1055 "Atlandı: "
LangString LSTR_30 1055 "Ayrıntıları panoya kopyala"
LangString LSTR_32 1055 B
LangString LSTR_33 1055 " K"
LangString LSTR_34 1055 " M"
LangString LSTR_35 1055 " G"
LangString LSTR_36 1055 "Dosya yazmak için açılırken hata meydana geldi: $\r$\n$\r$\n$0$\r$\n$\r$\nKurulumu durdurmak için Dur düğmesine,$\r$\nyeniden denemek için Yeniden Dene düğmesine,$\r$\nbu dosyayı atlamak için Yoksay düğmesine basın."
LangString LSTR_37 1055 "Hedef dizini seçimi"
LangString LSTR_38 1055 "$(LSTR_76) programını kurmak istediğiniz dizini şeçiniz."
LangString LSTR_39 1055 Kuruluyor
LangString LSTR_40 1055 "Lütfen $(LSTR_76) kurulurken bekleyiniz."
LangString LSTR_41 1055 "Kurulum Tamamlandı"
LangString LSTR_42 1055 "Kurulum başarıyla tamamlandı."
LangString LSTR_43 1055 "Kurulum İptal Edildi"
LangString LSTR_44 1055 "Kurulum tam olarak tamamlanmadı."
LangString LSTR_45 1055 &Bitir
LangString LSTR_46 1055 0
LangString LSTR_47 1055 "$(LSTR_76) Kurulum sihirbazı tamamlanıyor."
LangString LSTR_48 1055 "MS Shell Dlg"
LangString LSTR_49 1055 "$(LSTR_76) kurulumunun tamamlanması için bilgisayarınızı yeniden başlatmanız gerekiyor.Bilgisayarınızı yeniden başlatmak istiyor musunuz?"
LangString LSTR_50 1055 "Yeniden başlat"
LangString LSTR_51 1055 "Bilgisayarımı daha sonra başlatacağım."
LangString LSTR_52 1055 "$(LSTR_76)  bilgisayariniza yüklendi.$\r$\n$\r$\nLütfen 'Bitir'e basarak kurulumu sonlandırın."
LangString LSTR_53 1055 "$(LSTR_76) programını çalıştır"
LangString LSTR_54 1055 8
LangString LSTR_55 1055 "Windows 7 ve üstü gereklidir"
LangString LSTR_56 1055 "64 bit Windows gerekli"
LangString LSTR_57 1055 "Failed to uninstall old application files. Please try running the installer again."
LangString LSTR_58 1055 "Shinri Hub kapatılamaz. $\r$\nLütfen elle kapatın ve devam etmek için Tekrar'a tıklayın"
LangString LSTR_59 1055 "Shinri Hub çalışıyor.$\r$\nKapatmak için Tamam'ı tıklayın."
LangString LSTR_60 1055 "Closing running Shinri Hub..."
LangString LSTR_61 1055 Özel
LangString LSTR_62 1055 Vazgeç
LangString LSTR_63 1055 "$(LSTR_76) aşağıdaki dizinde kurulacak. Farklı bir dizinde kurmak için Gözat düğmesine basıp başka bir dizin seçin. $_CLICK"
LangString LSTR_64 1055 "Hedef Dizin"
LangString LSTR_65 1055 &Gözat...
LangString LSTR_66 1055 "$(LSTR_76) uygulamasının kurulacağı dizini seçin:"
LangString LSTR_67 1055 "< &Geri"
LangString LSTR_68 1055 &Kur
LangString LSTR_69 1055 "Kurulumu başlatmak için Kur düğmesine basın."
LangString LSTR_70 1055 "&Ayrıntıları göster"
LangString LSTR_71 1055 Tamamlandı
LangString LSTR_72 1055 "İ&leri >"
LangString LSTR_73 1055 "Devam etmek için İleri düğmesine basın."
LangString LSTR_74 1055 " "
LangString LSTR_75 1055 &Kapat
LangString LSTR_76 1055 "Shinri Hub"


; LANG: 1028
LangString LSTR_0 1028 "Shinri Hub 26.11.23196"
LangString LSTR_1 1028 "$(LSTR_2) 安裝"
LangString LSTR_2 1028 "Shinri Hub"
LangString LSTR_3 1028 "可用空間: "
LangString LSTR_4 1028 "所需空間: "
LangString LSTR_5 1028 "無法寫入: "
LangString LSTR_6 1028 "複製失敗 "
LangString LSTR_7 1028 "複製到: "
LangString LSTR_8 1028 "無法找到符號: "
LangString LSTR_9 1028 "無法載入: "
LangString LSTR_10 1028 "$\"建立資料夾: $\" "
LangString LSTR_11 1028 "建立捷徑: "
LangString LSTR_13 1028 "刪除檔案: "
LangString LSTR_14 1028 "重新開機後刪除: "
LangString LSTR_15 1028 "建立捷徑時發生錯誤: "
LangString LSTR_17 1028 解壓縮資料發生錯誤！安裝程式是否已損壞？
LangString LSTR_20 1028 "執行: "
LangString LSTR_21 1028 "抽取: "
LangString LSTR_22 1028 "抽取: 無法寫入檔案 "
LangString LSTR_23 1028 "安裝程式損毀: 無效的作業代碼 "
LangString LSTR_24 1028 "沒有 OLE 用於: "
LangString LSTR_25 1028 "輸出資料夾: "
LangString LSTR_26 1028 "移除資料夾: "
LangString LSTR_27 1028 "重新開機後重新命名: "
LangString LSTR_28 1028 "重新命名: "
LangString LSTR_29 1028 "已略過: "
LangString LSTR_30 1028 "複製細節到剪貼簿 "
LangString LSTR_32 1028 B
LangString LSTR_33 1028 " K"
LangString LSTR_34 1028 " M"
LangString LSTR_35 1028 " G"
LangString LSTR_36 1028 "無法開啟要寫入的檔案: $\r$\n$\t$\"$0$\"$\r$\n按「中止」停止安裝，$\r$\n「重試」重新嘗試寫入檔案，或$\r$\n「忽略」略過此檔案。"
LangString LSTR_37 1028 選取安裝位置
LangString LSTR_38 1028 "選取 $(LSTR_76) 要安裝的資料夾。"
LangString LSTR_39 1028 安裝
LangString LSTR_40 1028 "$(LSTR_76) 正在安裝，請稍候。"
LangString LSTR_41 1028 完成安裝
LangString LSTR_42 1028 安裝已完成。
LangString LSTR_43 1028 已中止安裝
LangString LSTR_44 1028 安裝並未完成。
LangString LSTR_45 1028 完成(&F)
LangString LSTR_46 1028 0
LangString LSTR_47 1028 "即將完成安裝 $(LSTR_76)"
LangString LSTR_48 1028 新細明體
LangString LSTR_49 1028 "電腦需要重新開機，才能完成安裝 $(LSTR_76) 。要馬上重新開機嗎？"
LangString LSTR_50 1028 是，馬上重新開機(&Y)
LangString LSTR_51 1028 否，我稍後再自行重新開機(&N)
LangString LSTR_52 1028 "已在電腦安裝 $(LSTR_76) 。$\r$\n按 [完成(F)] 關閉安裝程式。"
LangString LSTR_53 1028 "執行 $(LSTR_76)(&R)"
LangString LSTR_54 1028 9
LangString LSTR_55 1028 "需要 Windows 7 或更新的系統"
LangString LSTR_56 1028 "需要 64 位元 Windows 系統"
LangString LSTR_57 1028 "無法俺安裝舊的應用程式檔案。 請嘗試再次執行安裝程式。"
LangString LSTR_58 1028 "Shinri Hub 無法關閉。$\r$\n請手動關閉它，然後按一下重試以繼續。"
LangString LSTR_59 1028 "Shinri Hub 正在執行。$\r$\n點擊 OK 關閉。$\r$\n如果無法關閉，試著手動關閉。"
LangString LSTR_60 1028 "Closing running Shinri Hub..."
LangString LSTR_61 1028 自訂
LangString LSTR_62 1028 取消
LangString LSTR_63 1028 "安裝程式會將 $(LSTR_76) 安裝在以下資料夾。要安裝到不同的資料夾，按「瀏覽(B)...」並選擇其他資料夾。 $_CLICK"
LangString LSTR_64 1028 目標資料夾
LangString LSTR_65 1028 瀏覽(&R)...
LangString LSTR_66 1028 "選取要安裝 $(LSTR_76) 的資料夾: "
LangString LSTR_67 1028 "< 上一步(&B)"
LangString LSTR_68 1028 安裝(&I)
LangString LSTR_69 1028 按「安裝(I)」開始安裝。
LangString LSTR_70 1028 顯示細節(&D)
LangString LSTR_71 1028 已完成
LangString LSTR_72 1028 "下一步(&N) >"
LangString LSTR_73 1028 按「下一步(N)」繼續。
LangString LSTR_74 1028 " "
LangString LSTR_75 1028 關閉(&C)
LangString LSTR_76 1028 "Shinri Hub"


; LANG: 2052
LangString LSTR_0 2052 "Shinri Hub 26.11.23196"
LangString LSTR_1 2052 "$(LSTR_2) 安装"
LangString LSTR_2 2052 "Shinri Hub"
LangString LSTR_3 2052 "可用空间: "
LangString LSTR_4 2052 "所需空间: "
LangString LSTR_5 2052 "无法写入: "
LangString LSTR_6 2052 "复制失败 "
LangString LSTR_7 2052 "复制到: "
LangString LSTR_8 2052 "无法找到符号: "
LangString LSTR_9 2052 "无法加载: "
LangString LSTR_10 2052 "$\"创建文件夹: $\" "
LangString LSTR_11 2052 "创建快捷方式: "
LangString LSTR_13 2052 "删除文件: "
LangString LSTR_14 2052 "重新启动后删除: "
LangString LSTR_15 2052 "正在创建快捷方式时发生错误: "
LangString LSTR_17 2052 正在解压缩数据发生错误！已损坏的安装程序？
LangString LSTR_20 2052 "运行: "
LangString LSTR_21 2052 "抽取: "
LangString LSTR_22 2052 "抽取: 无法写入文件 "
LangString LSTR_23 2052 "安装损毁: 无效的操作代码 "
LangString LSTR_24 2052 "没有 OLE 用于: "
LangString LSTR_25 2052 "输出目录: "
LangString LSTR_26 2052 "移除目录: "
LangString LSTR_27 2052 "重新启动后重命名: "
LangString LSTR_28 2052 "重命名: "
LangString LSTR_29 2052 "已跳过: "
LangString LSTR_30 2052 "复制细节到剪贴板 "
LangString LSTR_32 2052 B
LangString LSTR_33 2052 " K"
LangString LSTR_34 2052 " M"
LangString LSTR_35 2052 " G"
LangString LSTR_36 2052 "不能打开要写入的文件: $\r$\n$\t$\"$0$\"$\r$\n单击 [Abort] 停止安装，$\r$\n [Retry] 重新尝试写入文件，或$\r$\n [Ignore] 忽略这个文件。"
LangString LSTR_37 2052 选定安装位置
LangString LSTR_38 2052 "选定 $(LSTR_76) 要安装的文件夹。"
LangString LSTR_39 2052 正在安装
LangString LSTR_40 2052 "$(LSTR_76) 正在安装，请等候。"
LangString LSTR_41 2052 安装完成
LangString LSTR_42 2052 安装程序已成功地运行完成。
LangString LSTR_43 2052 安装己中止
LangString LSTR_44 2052 安装程序并未成功地运行完成。
LangString LSTR_45 2052 完成(&F)
LangString LSTR_46 2052 0
LangString LSTR_47 2052 "正在完成 $(LSTR_76) 安装向导"
LangString LSTR_48 2052 宋体
LangString LSTR_49 2052 "你的系统需要重新启动，以便完成 $(LSTR_76) 的安装。现在要重新启动吗？"
LangString LSTR_50 2052 是，现在重新启动(&Y)
LangString LSTR_51 2052 否，我稍后再自行重新启动(&N)
LangString LSTR_52 2052 "$(LSTR_76) 已安装在你的系统。$\r$\n单击 [完成(F)] 关闭此向导。"
LangString LSTR_53 2052 "运行 $(LSTR_76)(&R)"
LangString LSTR_54 2052 9
LangString LSTR_55 2052 "需要 Windows 7 或以上系统"
LangString LSTR_56 2052 "需要 64 位 Windows 系统"
LangString LSTR_57 2052 "Failed to uninstall old application files. Please try running the installer again."
LangString LSTR_58 2052 "Shinri Hub 无法关闭。$\r$\n请手动关闭它，然后单击重试以继续。"
LangString LSTR_59 2052 "Shinri Hub 正在运行.$\r$\n点击“确定”关闭."
LangString LSTR_60 2052 "Closing running Shinri Hub..."
LangString LSTR_61 2052 自定义
LangString LSTR_62 2052 取消(&C)
LangString LSTR_63 2052 "Setup 将安装 $(LSTR_76) 在下列文件夹。要安装到不同文件夹，单击 [浏览(B)...] 并选择其他的文件夹。 $_CLICK"
LangString LSTR_64 2052 目标文件夹
LangString LSTR_65 2052 浏览(&B)...
LangString LSTR_66 2052 "选择要安装 $(LSTR_76) 的文件夹位置: "
LangString LSTR_67 2052 "< 上一步(&P)"
LangString LSTR_68 2052 安装(&I)
LangString LSTR_69 2052 "单击 [安装(I)] 开始安装进程。"
LangString LSTR_70 2052 显示细节(&D)
LangString LSTR_71 2052 已完成
LangString LSTR_72 2052 "下一步(&N) >"
LangString LSTR_73 2052 "单击 [下一步(N)] 继续。"
LangString LSTR_74 2052 " "
LangString LSTR_75 2052 关闭(&L)
LangString LSTR_76 2052 "Shinri Hub"


; LANG: 1046
LangString LSTR_0 1046 "Shinri Hub 26.11.23196"
LangString LSTR_1 1046 "Instalação do $(LSTR_2)"
LangString LSTR_2 1046 "Shinri Hub"
LangString LSTR_3 1046 "Espaço disponível: "
LangString LSTR_4 1046 "Espaço necessário: "
LangString LSTR_5 1046 "Não foi possível gravar: "
LangString LSTR_6 1046 "Falha ao copiar"
LangString LSTR_7 1046 "Copiar para "
LangString LSTR_8 1046 "Não foi possível localizar o símbolo: "
LangString LSTR_9 1046 "Não foi possível carregar: "
LangString LSTR_10 1046 "Criar pasta: "
LangString LSTR_11 1046 "Criar atalho: "
LangString LSTR_13 1046 "Excluir o arquivo: "
LangString LSTR_14 1046 "Excluir ao reiniciar: "
LangString LSTR_15 1046 "Erro ao criar atalho: "
LangString LSTR_17 1046 "Erro ao descomprimir os dados! Instalador corrompido?"
LangString LSTR_20 1046 "Executar: "
LangString LSTR_21 1046 "Extrair: "
LangString LSTR_22 1046 "Extrair: erro ao gravar o arquivo "
LangString LSTR_23 1046 "Instalador corrompido: opcode inválido"
LangString LSTR_24 1046 "Sem OLE para: "
LangString LSTR_25 1046 "Pasta de saída: "
LangString LSTR_26 1046 "Excluir a pasta: "
LangString LSTR_27 1046 "Renomear ao reiniciar: "
LangString LSTR_28 1046 "Renomear: "
LangString LSTR_29 1046 "Ignorado: "
LangString LSTR_30 1046 "Copiar os Detalhes para a Área de Transferência"
LangString LSTR_32 1046 B
LangString LSTR_33 1046 " K"
LangString LSTR_34 1046 " M"
LangString LSTR_35 1046 " G"
LangString LSTR_36 1046 "Erro ao abrir o arquivo para gravação: $\r$\n$\r$\n$0$\r$\n$\r$\nClique em Abortar para parar a instalação,$\r$\nRepetir para tentar de novo, ou$\r$\nIgnorar para pular este arquivo."
LangString LSTR_37 1046 "Escolha o Local da Instalação"
LangString LSTR_38 1046 "Escolha a pasta na qual instalar o $(LSTR_76)."
LangString LSTR_39 1046 Instalando
LangString LSTR_40 1046 "Por favor espere enquanto o $(LSTR_76) está sendo instalado."
LangString LSTR_41 1046 "Instalação Completa"
LangString LSTR_42 1046 "A instalação foi completada com sucesso."
LangString LSTR_43 1046 "Instalação Abortada"
LangString LSTR_44 1046 "A instalação não foi completada com sucesso."
LangString LSTR_45 1046 &Concluir
LangString LSTR_46 1046 0
LangString LSTR_47 1046 "Completando a instalação do $(LSTR_76)"
LangString LSTR_48 1046 "MS Shell Dlg"
LangString LSTR_49 1046 "Seu computador deve ser reiniciado para completar a instalação do $(LSTR_76). Você quer reiniciar agora?"
LangString LSTR_50 1046 "Reiniciar agora"
LangString LSTR_51 1046 "Eu quero reiniciar manualmente depois"
LangString LSTR_52 1046 "O $(LSTR_76) foi instalado no seu computador.$\r$\n$\r$\nClique em Concluir para fechar o instalador."
LangString LSTR_53 1046 "&Executar o $(LSTR_76)"
LangString LSTR_54 1046 8
LangString LSTR_55 1046 "Requer Windows 7 ou superior"
LangString LSTR_56 1046 "Requer Windows 64 bits"
LangString LSTR_57 1046 "Falha ao desinstalar os arquivos do aplicativo antigo. Por favor, tente iniciar o instalador novamente."
LangString LSTR_58 1046 "Não é possível fechar o Shinri Hub. $\r$\nFeche a janela do Shinri Hub e clique em Repetir para continuar."
LangString LSTR_59 1046 "Shinri Hub está funcionando.$\r$\nClique no OK para fechar.$\r$\nSe não fechar, tente fechá-lo manualmente."
LangString LSTR_60 1046 "Closing running Shinri Hub..."
LangString LSTR_61 1046 Personalizado
LangString LSTR_62 1046 Cancelar
LangString LSTR_63 1046 "O $(LSTR_76) será instalado na pasta a seguir. Para instalar em uma pasta diferente, clique em Procurar e selecione outra pasta. $_CLICK"
LangString LSTR_64 1046 "Pasta de Destino"
LangString LSTR_65 1046 P&rocurar...
LangString LSTR_66 1046 "Selecione a pasta para instalar o $(LSTR_76):"
LangString LSTR_67 1046 "< &Voltar"
LangString LSTR_68 1046 &Instalar
LangString LSTR_69 1046 "Clique em Instalar para iniciar a instalação."
LangString LSTR_70 1046 "Mostrar &detalhes"
LangString LSTR_71 1046 Completado
LangString LSTR_72 1046 "&Próximo >"
LangString LSTR_73 1046 "Clique em Próximo para continuar."
LangString LSTR_74 1046 " "
LangString LSTR_75 1046 &Fechar
LangString LSTR_76 1046 "Shinri Hub"


; LANG: 1045
LangString LSTR_0 1045 "Shinri Hub 26.11.23196"
LangString LSTR_1 1045 "Instalator $(LSTR_2)"
LangString LSTR_2 1045 "Shinri Hub"
LangString LSTR_3 1045 "Dostępne miejsce: "
LangString LSTR_4 1045 "Wymagane miejsce: "
LangString LSTR_5 1045 "Nie można zapisać: "
LangString LSTR_6 1045 "Błąd kopiowania"
LangString LSTR_7 1045 "Kopiuj do "
LangString LSTR_8 1045 "Nie można odnaleźć symbolu: "
LangString LSTR_9 1045 "Nie można wczytać: "
LangString LSTR_10 1045 "Utwórz folder: "
LangString LSTR_11 1045 "Utwórz skrót: "
LangString LSTR_13 1045 "Usuń plik: "
LangString LSTR_14 1045 "Usuń przy ponownym uruchomieniu: "
LangString LSTR_15 1045 "Błąd tworzenia skrótu: "
LangString LSTR_17 1045 "Błąd wyodrębniania danych! Uszkodzony instalator?"
LangString LSTR_20 1045 "Uruchom: "
LangString LSTR_21 1045 "Wyodrębnij: "
LangString LSTR_22 1045 "Wyodrębnij: błąd zapisu do pliku "
LangString LSTR_23 1045 "Instalator uszkodzony: nieprawidłowy kod operacji"
LangString LSTR_24 1045 "Brak OLE dla: "
LangString LSTR_25 1045 "Folder wyjściowy: "
LangString LSTR_26 1045 "Usuń folder: "
LangString LSTR_27 1045 "Zmień nazwę przy ponownym uruchomieniu: "
LangString LSTR_28 1045 "Zmień nazwę: "
LangString LSTR_29 1045 "Pominięte: "
LangString LSTR_30 1045 "Kopiuj szczegóły do schowka"
LangString LSTR_32 1045 B
LangString LSTR_33 1045 " K"
LangString LSTR_34 1045 " M"
LangString LSTR_35 1045 " G"
LangString LSTR_36 1045 "Błąd otwarcia pliku do zapisu: $\r$\n$\r$\n$0$\r$\n$\r$\nWybierz 'Anuluj', aby przerwać instalację,$\r$\n'Ponów', aby ponowić zapis do pliku lub$\r$\n'Ignoruj', aby pominąć ten plik."
LangString LSTR_37 1045 "Wybierz lokalizację dla instalacji"
LangString LSTR_38 1045 "Wybierz folder, w którym chcesz zainstalować $(LSTR_76)."
LangString LSTR_39 1045 Instalacja
LangString LSTR_40 1045 "Proszę czekać, trwa instalacja $(LSTR_76)."
LangString LSTR_41 1045 "Zakończono instalację"
LangString LSTR_42 1045 "Instalacja zakończona pomyślnie."
LangString LSTR_43 1045 "Instalacja przerwana"
LangString LSTR_44 1045 "Instalacja nie została zakończona pomyślnie."
LangString LSTR_45 1045 &Zakończ
LangString LSTR_46 1045 0
LangString LSTR_47 1045 "Kończenie pracy kreatora instalacji $(LSTR_76)"
LangString LSTR_48 1045 "MS Shell Dlg"
LangString LSTR_49 1045 "Twój komputer musi zostać ponownie uruchomiony, aby zakończyć instalację $(LSTR_76). Czy chcesz teraz ponownie uruchomić komputer?"
LangString LSTR_50 1045 "Uruchom ponownie teraz"
LangString LSTR_51 1045 "Sam uruchomię ponownie komputer później"
LangString LSTR_52 1045 "$(LSTR_76) został pomyślnie zainstalowany na Twoim komputerze.$\r$\n$\r$\nKliknij przycisk 'Zakończ', aby zakończyć działanie instalatora."
LangString LSTR_53 1045 "&Uruchom $(LSTR_76)"
LangString LSTR_54 1045 8
LangString LSTR_55 1045 "Windows 7 lub nowszy jest wymagany"
LangString LSTR_56 1045 "Windows 64-bitowy jest wymagany"
LangString LSTR_57 1045 "Nie udało się usunąć plików starej wersji aplikacji. Spróbuj ponownie uruchomić instalator."
LangString LSTR_58 1045 "Nie można zamknąć Shinri Hub. $\r$\nZamknij aplikację i kliknij Ponów, aby kontynuować."
LangString LSTR_59 1045 "Aplikacja Shinri Hub jest uruchomiona.$\r$\nKliknij OK, aby ją zamknąć.$\r$\nJeśli nie zostanie zamknięta, spróbuj zamknąć ją ręcznie."
LangString LSTR_60 1045 "Closing running Shinri Hub..."
LangString LSTR_61 1045 Użytkownika
LangString LSTR_62 1045 Anuluj
LangString LSTR_63 1045 "Instalator zainstaluje $(LSTR_76) w podanym poniżej folderze docelowym (możesz także kliknąć przycisk 'Przeglądaj...' i wybrać inny folder). $_CLICK"
LangString LSTR_64 1045 "Folder docelowy"
LangString LSTR_65 1045 &Przeglądaj...
LangString LSTR_66 1045 "Wybierz folder instalacyjny $(LSTR_76):"
LangString LSTR_67 1045 "< &Wstecz"
LangString LSTR_68 1045 &Zainstaluj
LangString LSTR_69 1045 "Kliknij przycisk 'Zainstaluj', aby rozpocząć instalację."
LangString LSTR_70 1045 "Pokaż &szczegóły"
LangString LSTR_71 1045 Zakończono
LangString LSTR_72 1045 "&Dalej >"
LangString LSTR_73 1045 "Kliknij przycisk 'Dalej', aby kontynuować."
LangString LSTR_74 1045 " "
LangString LSTR_75 1045 &Zamknij
LangString LSTR_76 1045 "Shinri Hub"


; LANG: 1038
LangString LSTR_0 1038 "Shinri Hub 26.11.23196"
LangString LSTR_1 1038 "$(LSTR_2) Telepítő"
LangString LSTR_2 1038 "Shinri Hub"
LangString LSTR_3 1038 "Szabad terület: "
LangString LSTR_4 1038 "Helyigény: "
LangString LSTR_5 1038 "Nem írható: "
LangString LSTR_6 1038 "A másolás megszakadt"
LangString LSTR_7 1038 "Másolás ide: "
LangString LSTR_8 1038 "A következő szimbólum nem található: "
LangString LSTR_9 1038 "Nem tölthető be: "
LangString LSTR_10 1038 "Mappa létrehozás: "
LangString LSTR_11 1038 "Parancsikon létrehozása: "
LangString LSTR_13 1038 "Törölt fájl: "
LangString LSTR_14 1038 "Rendszerindításkor törlendő: "
LangString LSTR_15 1038 "Hiba a parancsikon létrehozásakor: "
LangString LSTR_17 1038 "Hiba az adatok kibontásakor! Megsérült a Telepítő?"
LangString LSTR_20 1038 "Végrehajtás: "
LangString LSTR_21 1038 "Kibontás: "
LangString LSTR_22 1038 "Kibontás: Hiba a fájl írásakor "
LangString LSTR_23 1038 "Sérült a telepítő: hibás utasítás"
LangString LSTR_24 1038 "Nincs OLE: "
LangString LSTR_25 1038 "Kimeneti mappa: "
LangString LSTR_26 1038 "Mappa eltávolítása: "
LangString LSTR_27 1038 "Átnevezés rendszerindításkor: "
LangString LSTR_28 1038 "Átnevezés: "
LangString LSTR_29 1038 "Kihagyott: "
LangString LSTR_30 1038 "Adatok vágólapra másolása"
LangString LSTR_32 1038 B
LangString LSTR_33 1038 " K"
LangString LSTR_34 1038 " M"
LangString LSTR_35 1038 " G"
LangString LSTR_36 1038 "Hiba történt a fájl írásra történő megnyitásakor: $\r$\n$\t$\"$0$\"$\r$\nA Mégse gomb megnyomásával megszakíthatja a telepítést,$\r$\naz Ismét gombbal megismételheti a fájl írását,$\r$\na Kihagyás gombbal kihagyhatja ezt a fájlt."
LangString LSTR_37 1038 "Telepítési hely kiválasztása"
LangString LSTR_38 1038 "Válassza ki a(z) $(LSTR_76) telepítésének mappáját."
LangString LSTR_39 1038 "Telepítési folyamat"
LangString LSTR_40 1038 "Kis türelmet a(z) $(LSTR_76) telepítéséig."
LangString LSTR_41 1038 "Telepítés befejeződött"
LangString LSTR_42 1038 "A telepítés sikeresen befejeződött."
LangString LSTR_43 1038 "A telepítés megszakadt"
LangString LSTR_44 1038 "A telepítés sikertelen volt."
LangString LSTR_45 1038 &Befejezés
LangString LSTR_46 1038 0
LangString LSTR_47 1038 "A(z) $(LSTR_76) telepítése megtörtént."
LangString LSTR_48 1038 "MS Shell Dlg"
LangString LSTR_49 1038 "A(z) $(LSTR_76) telepítésének befejezéséhez újra kell indítani a rendszert. Most akarja újraindítani?"
LangString LSTR_50 1038 "Most indítom újra"
LangString LSTR_51 1038 "Később fogom újraindítani"
LangString LSTR_52 1038 "A(z) $(LSTR_76) telepítése megtörtént.$\r$\n$\r$\nA Befejezés gomb megnyomásával zárja be a varázslót."
LangString LSTR_53 1038 "$(LSTR_76) futtatása"
LangString LSTR_54 1038 8
LangString LSTR_55 1038 "Windows 7 vagy újabb szükséges"
LangString LSTR_56 1038 "64-bit Windows szükséges"
LangString LSTR_57 1038 "A régi alkalmazás állományait nem sikerült törölni. Kérlek futtasd a telepítőt újra!"
LangString LSTR_58 1038 "Nem sikerült bezárni a Shinri Hubt. $\r$\nZárd be kézzel, majd kattints az Újra gombra a folytatáshoz!"
LangString LSTR_59 1038 "A Shinri Hub alkalmazás fut.$\r$\nKattints az OK gombra a bezárásához.$\r$\nHa így sem áll le, zárd be manuálisan."
LangString LSTR_60 1038 "Closing running Shinri Hub..."
LangString LSTR_61 1038 Egyéni
LangString LSTR_62 1038 &Mégse
LangString LSTR_63 1038 "A $(LSTR_76) a következő mappába kerül. Másik mappa választásához kattintson a Tallózás gombra. $_CLICK"
LangString LSTR_64 1038 "Telepítés helye"
LangString LSTR_65 1038 &Tallózás...
LangString LSTR_66 1038 "A(z) $(LSTR_76) telepítési helyének kiválasztása:"
LangString LSTR_67 1038 "< &Vissza"
LangString LSTR_68 1038 &Telepítés
LangString LSTR_69 1038 "Kattintson a Telepítésre a telepítéshez."
LangString LSTR_70 1038 &Részletek
LangString LSTR_71 1038 Kész
LangString LSTR_72 1038 "&Tovább >"
LangString LSTR_73 1038 "Kattintson a Tovább-ra a folytatáshoz."
LangString LSTR_74 1038 " "
LangString LSTR_75 1038 &Bezárás
LangString LSTR_76 1038 "Shinri Hub"


; LANG: 1029
LangString LSTR_0 1029 "Shinri Hub 26.11.23196"
LangString LSTR_1 1029 "Instalace $(LSTR_2)"
LangString LSTR_2 1029 "Shinri Hub"
LangString LSTR_3 1029 "Volné místo: "
LangString LSTR_4 1029 "Požadované místo: "
LangString LSTR_5 1029 "Nelze zapsat: "
LangString LSTR_6 1029 "Kopírování se nezdařilo."
LangString LSTR_7 1029 "Kopírování do "
LangString LSTR_8 1029 "Nelze najít symbol: "
LangString LSTR_9 1029 "Nelze načíst: "
LangString LSTR_10 1029 "Vytváření složky: "
LangString LSTR_11 1029 "Vytváření zástupce: "
LangString LSTR_13 1029 "Odstraňování souboru: "
LangString LSTR_14 1029 "Odstranit při restartování: "
LangString LSTR_15 1029 "Při vytváření zástupce došlo k chybě: "
LangString LSTR_17 1029 "Při dekompresi dat došlo k chybě. Byl poškozen instalační program?"
LangString LSTR_20 1029 "Spouštění: "
LangString LSTR_21 1029 "Extrahování: "
LangString LSTR_22 1029 "Extrakce: Při zápisu souboru došlo k chybě "
LangString LSTR_23 1029 "Instalační program byl poškozen: neplatný operační kód."
LangString LSTR_24 1029 "Nedostupné OLE pro: "
LangString LSTR_25 1029 "Výstupní složka: "
LangString LSTR_26 1029 "Odstraňování složky: "
LangString LSTR_27 1029 "Přejmenovat při restartování: "
LangString LSTR_28 1029 "Přejmenováno: "
LangString LSTR_29 1029 "Přeskočeno: "
LangString LSTR_30 1029 "Zkopírovat podrobnosti do schránky"
LangString LSTR_32 1029 B
LangString LSTR_33 1029 " K"
LangString LSTR_34 1029 " M"
LangString LSTR_35 1029 " G"
LangString LSTR_36 1029 "Soubor nelze otevřít pro zápis: $\r$\n$\t$\"$0$\"$\r$\nKlikněte na tlačítko Přerušit k ukončení instalace,$\r$\nnebo na tlačítko Opakovat k zopakování akce, nebo$\r$\n na tlačítko Ignorovat k přeskočení souboru."
LangString LSTR_37 1029 "Volba umístění instalace"
LangString LSTR_38 1029 "Výběr instalační složky programu $(LSTR_76)."
LangString LSTR_39 1029 Instalace
LangString LSTR_40 1029 "Počkejte prosím na dokončení instalace programu $(LSTR_76)."
LangString LSTR_41 1029 "Instalace dokončena"
LangString LSTR_42 1029 "Instalace byla úspěšně dokončena."
LangString LSTR_43 1029 "Instalace přerušena"
LangString LSTR_44 1029 "Instalace nebyla úspěšně dokončena."
LangString LSTR_45 1029 &Dokončit
LangString LSTR_46 1029 0
LangString LSTR_47 1029 "Dokončení instalace programu $(LSTR_76)"
LangString LSTR_48 1029 "MS Shell Dlg"
LangString LSTR_49 1029 "K dokončení instalace programu $(LSTR_76) je nutné restartovat počítač. Chcete jej restartovat nyní?"
LangString LSTR_50 1029 "Restartovat nyní"
LangString LSTR_51 1029 "Restartovat později ručně"
LangString LSTR_52 1029 "Program $(LSTR_76) byl nainstalován do počítače.$\r$\n$\r$\nKliknutím na tlačítko Dokončit ukončíte tohoto průvodce."
LangString LSTR_53 1029 "&Spustit program $(LSTR_76)"
LangString LSTR_54 1029 8
LangString LSTR_55 1029 "Jsou podporovány pouze Windows 7 a vyšší"
LangString LSTR_56 1029 "Potřebujete 64-bit Windows"
LangString LSTR_57 1029 "Nepodařilo se odinstalovat staré soubory aplikace. Zkuste prosím instalátor spustit znovu."
LangString LSTR_58 1029 "Aplikaci Shinri Hub se nepodařilo zavřít. $\r$\nZavřete ji ručně a poté klikněte na Opakovat pro pokračování."
LangString LSTR_59 1029 "Shinri Hub běží.$\r$\nPotvrďte zavření stisknutím OK.$\r$\nPokud se aplikace nezavře sama, zavřete ji ručně."
LangString LSTR_60 1029 "Ukončuji aplikaci Shinri Hub..."
LangString LSTR_61 1029 Vlastní
LangString LSTR_62 1029 Storno
LangString LSTR_63 1029 "Instalační program nainstaluje program $(LSTR_76) do následující složky. Chcete-li instalovat do jiné složky, klikněte na tlačítko Procházet a vyberte jinou složku. $_CLICK"
LangString LSTR_64 1029 "Cílová složka"
LangString LSTR_65 1029 P&rocházet...
LangString LSTR_66 1029 "Výběr instalační složky programu $(LSTR_76)."
LangString LSTR_67 1029 "< &Zpět"
LangString LSTR_68 1029 &Instalovat
LangString LSTR_69 1029 "Instalaci spustíte kliknutím na tlačítko Instalovat."
LangString LSTR_70 1029 &Podrobnosti
LangString LSTR_71 1029 Dokončeno
LangString LSTR_72 1029 "&Další >"
LangString LSTR_73 1029 "Pokračujte kliknutím na tlačítko Další."
LangString LSTR_74 1029 " "
LangString LSTR_75 1029 &Zavřít
LangString LSTR_76 1029 "Shinri Hub"


; LANG: 1051
LangString LSTR_0 1051 "Shinri Hub 26.11.23196"
LangString LSTR_1 1051 "Inštalácia programu $(LSTR_2)"
LangString LSTR_2 1051 "Shinri Hub"
LangString LSTR_3 1051 "Voľné miesto na disku: "
LangString LSTR_4 1051 "Potrebné miesto na disku: "
LangString LSTR_5 1051 "Nemožno zapísať súbor: "
LangString LSTR_6 1051 "Kopírovanie zlyhalo."
LangString LSTR_7 1051 "Kopírovať do "
LangString LSTR_8 1051 "Nemožno nájsť symbol: "
LangString LSTR_9 1051 "Nemožno načítať: "
LangString LSTR_10 1051 "Vytvorený priečinok: "
LangString LSTR_11 1051 "Vytvorený odkaz: "
LangString LSTR_13 1051 "Vymazaný súbor: "
LangString LSTR_14 1051 "Vymazať po reštartovaní systému: "
LangString LSTR_15 1051 "Chyba pri vytváraní odkazu: "
LangString LSTR_17 1051 "Chyba pri dekomprimovaní dát! Inštalátor je pravdepodobne poškodený..."
LangString LSTR_20 1051 "Spustiť: "
LangString LSTR_21 1051 "Extrahuje sa: "
LangString LSTR_22 1051 "Chyba pri zápise do súboru "
LangString LSTR_23 1051 "Inštalátor je pravdepodobne poškodený, pretože obsahuje neplatný operačný kód."
LangString LSTR_24 1051 "Žiadny zápis OLE pre: "
LangString LSTR_25 1051 "Výstupný priečinok: "
LangString LSTR_26 1051 "Odstrániť priečinok: "
LangString LSTR_27 1051 "Premenovať po reštartovaní systému: "
LangString LSTR_28 1051 "Premenovať: "
LangString LSTR_29 1051 "Vynechané: "
LangString LSTR_30 1051 "Skopírovať podrobnosti do schránky"
LangString LSTR_32 1051 B
LangString LSTR_33 1051 " K"
LangString LSTR_34 1051 " M"
LangString LSTR_35 1051 " G"
LangString LSTR_36 1051 "Chyba pri otváraní súboru na zápis: $\r$\n$\r$\n$0$\r$\n$\r$\n. Ak chcete inštaláciu ukončiť, kliknite na tlačidlo Ukončiť,$\r\ ak chcete zápis súboru zopakovať, kliknite na tlačidlo Opakovať alebo kliknite na tlačidlo $\r$\nIgnorovať, ak chcete inštaláciu tohto súboru vynechať."
LangString LSTR_37 1051 "Voľba umiestnenia programu"
LangString LSTR_38 1051 "Vyberte si priečinok, do ktorého chcete nainštalovať program $(LSTR_76)."
LangString LSTR_39 1051 Inštalácia
LangString LSTR_40 1051 "Počkajte prosím, kým prebehne inštalácia programu $(LSTR_76)."
LangString LSTR_41 1051 "Ukončenie inštalácie"
LangString LSTR_42 1051 "Inštalácia bola dokončená úspešne."
LangString LSTR_43 1051 "Prerušenie inštalácie"
LangString LSTR_44 1051 "Inštaláciu sa nepodarilo dokončiť."
LangString LSTR_45 1051 &Dokončiť
LangString LSTR_46 1051 0
LangString LSTR_47 1051 "Dokončenie inštalácie programu $(LSTR_76)"
LangString LSTR_48 1051 "MS Shell Dlg"
LangString LSTR_49 1051 "Pre úplné dokončenie inštalácie programu $(LSTR_76) je potrebné reštartovať váš počítač. Chcete ho reštartovať ihneď?"
LangString LSTR_50 1051 "Reštartovať teraz"
LangString LSTR_51 1051 "Reštartovať neskôr (manuálne)"
LangString LSTR_52 1051 "Program $(LSTR_76) bol nainštalovaný do vášho počítača.$\r$\nKliknite na tlačidlo Dokončiť a tento sprievodca sa ukončí."
LangString LSTR_53 1051 "&Spustiť program $(LSTR_76)"
LangString LSTR_54 1051 8
LangString LSTR_55 1051 "Iba Windows 7 a vyššie je podporovaný"
LangString LSTR_56 1051 "Potrebujete 64-bit Windows"
LangString LSTR_57 1051 "Nepodarilo sa odinštalovať staré súbory aplikácie. Skúste znova spustiť inštalačný program."
LangString LSTR_58 1051 "Shinri Hub nie je možné zatvoriť. $\r$\nZatvorte ho ručne a pokračujte kliknutím na položku Opakovať."
LangString LSTR_59 1051 "Shinri Hub beží.$\r$\nKliknutím na tlačidlo OK ho zatvoríte.$\r$\nAk sa nezavrie, skúste ho zatvoriť ručne."
LangString LSTR_60 1051 "Closing running Shinri Hub..."
LangString LSTR_61 1051 Voliteľná
LangString LSTR_62 1051 Zrušiť
LangString LSTR_63 1051 "$(LSTR_76) bude nainštalovaný do nasledujúceho priečinka. Inštalovať do iného priečinka môžete po kliknutí na tlačidlo Prehľadávať a vybraní iného priečinka. $_CLICK"
LangString LSTR_64 1051 "Cieľový priečinok"
LangString LSTR_65 1051 &Prehľadávať...
LangString LSTR_66 1051 "Zvoľte priečinok, do ktorého sa nainštaluje program $(LSTR_76):"
LangString LSTR_67 1051 "< &Späť"
LangString LSTR_68 1051 &Nainštalovať
LangString LSTR_69 1051 "Pre spustenie inštalácie kliknite na tlačidlo Nainštalovať."
LangString LSTR_70 1051 &Podrobnosti
LangString LSTR_71 1051 Hotovo
LangString LSTR_72 1051 "Ď&alej >"
LangString LSTR_73 1051 "V inštalácii pokračujte kliknutím na tlačidlo Ďalej."
LangString LSTR_74 1051 " "
LangString LSTR_75 1051 &Zatvoriť
LangString LSTR_76 1051 "Shinri Hub"


; LANG: 1049
LangString LSTR_0 1049 "Shinri Hub 26.11.23196"
LangString LSTR_1 1049 "Установка $(LSTR_2)"
LangString LSTR_2 1049 "Shinri Hub"
LangString LSTR_3 1049 "Доступно на диске: "
LangString LSTR_4 1049 "Требуется на диске: "
LangString LSTR_5 1049 "Невозможно записать: "
LangString LSTR_6 1049 "Ошибка при копировании"
LangString LSTR_7 1049 "Копирование в "
LangString LSTR_8 1049 "Невозможно найти символ: "
LangString LSTR_9 1049 "Невозможно загрузить: "
LangString LSTR_10 1049 "Создание папки: "
LangString LSTR_11 1049 "Создание ярлыка: "
LangString LSTR_13 1049 "Удаление файла: "
LangString LSTR_14 1049 "Удаление при перезагрузке ПК: "
LangString LSTR_15 1049 "Ошибка создания ярлыка: "
LangString LSTR_17 1049 "Ошибка распаковки данных! Возможно, повреждён дистрибутив."
LangString LSTR_20 1049 "Выполнение: "
LangString LSTR_21 1049 "Извлечение: "
LangString LSTR_22 1049 "Извлечение: ошибка записи файла "
LangString LSTR_23 1049 "Дистрибутив поврежден: недопустимый код"
LangString LSTR_24 1049 "Нет OLE для: "
LangString LSTR_25 1049 "Папка установки: "
LangString LSTR_26 1049 "Удаление папки: "
LangString LSTR_27 1049 "Переименование при перезагрузке ПК: "
LangString LSTR_28 1049 "Переименование: "
LangString LSTR_29 1049 "Пропуск: "
LangString LSTR_30 1049 "Копировать сведения в буфер обмена"
LangString LSTR_32 1049 байт
LangString LSTR_33 1049 " К"
LangString LSTR_34 1049 " М"
LangString LSTR_35 1049 " Г"
LangString LSTR_36 1049 "Невозможно открыть файл для записи: $\r$\n$\t$\"$0$\"$\r$\n$\"Прервать$\": остановить установку;$\r$\n$\"Повтор$\": повторить попытку;$\r$\n$\"Пропуск$\": пропустить это действие."
LangString LSTR_37 1049 "Выбор папки установки"
LangString LSTR_38 1049 "Выберите папку для установки $(LSTR_76)."
LangString LSTR_39 1049 "Копирование файлов"
LangString LSTR_40 1049 "Подождите, идет копирование файлов $(LSTR_76)..."
LangString LSTR_41 1049 "Установка завершена"
LangString LSTR_42 1049 "Установка успешно завершена."
LangString LSTR_43 1049 "Установка прервана"
LangString LSTR_44 1049 "Установка не завершена."
LangString LSTR_45 1049 &Готово
LangString LSTR_46 1049 0
LangString LSTR_47 1049 "Завершение работы мастера установки $(LSTR_76)"
LangString LSTR_48 1049 "MS Shell Dlg"
LangString LSTR_49 1049 "Для завершения установки $(LSTR_76) необходимо перезагрузить компьютер. Хотите сделать это сейчас?"
LangString LSTR_50 1049 "Да, перезагрузить ПК сейчас"
LangString LSTR_51 1049 "Нет, я перезагружу ПК позже"
LangString LSTR_52 1049 "Установка $(LSTR_76) выполнена.$\r$\n$\r$\nНажмите кнопку $\"Готово$\" для выхода из программы установки."
LangString LSTR_53 1049 "&Запустить $(LSTR_76)"
LangString LSTR_54 1049 8
LangString LSTR_55 1049 "Требуется Windows 7 и выше"
LangString LSTR_56 1049 "Требуется Windows 64-bit"
LangString LSTR_57 1049 "Не удалось удалить старые файлы приложения. Пожалуйста, попробуйте запустить установщик заново."
LangString LSTR_58 1049 "Не удалось закрыть Shinri Hub. $\r$\nПожалуйста, закройте Shinri Hub вручную и нажмите «Повторить», чтобы продолжить."
LangString LSTR_59 1049 "Приложение Shinri Hub запущено.$\r$\nНажмите «OK», чтобы закрыть его.$\r$\nЕсли оно не закрывается, попробуйте закрыть его вручную."
LangString LSTR_60 1049 "Closing running Shinri Hub..."
LangString LSTR_61 1049 "По выбору"
LangString LSTR_62 1049 Отмена
LangString LSTR_63 1049 "Программа установит $(LSTR_76) в указанную папку. Чтобы установить приложение в другую папку, нажмите кнопку $\"Обзор$\" и укажите ее. $_CLICK"
LangString LSTR_64 1049 "Папка установки"
LangString LSTR_65 1049 "О&бзор ..."
LangString LSTR_66 1049 "Укажите папку для установки $(LSTR_76):"
LangString LSTR_67 1049 "< &Назад"
LangString LSTR_68 1049 &Установить
LangString LSTR_69 1049 "Нажмите кнопку $\"Установить$\", чтобы установить программу."
LangString LSTR_70 1049 &Детали...
LangString LSTR_71 1049 Готово
LangString LSTR_72 1049 "&Далее >"
LangString LSTR_73 1049 "Нажмите кнопку $\"Далее$\" для продолжения."
LangString LSTR_74 1049 " "
LangString LSTR_75 1049 &Закрыть
LangString LSTR_76 1049 "Shinri Hub"


; LANG: 1042
LangString LSTR_0 1042 "Shinri Hub 26.11.23196"
LangString LSTR_1 1042 "$(LSTR_2) 설치"
LangString LSTR_2 1042 "Shinri Hub"
LangString LSTR_3 1042 "남은 디스크 공간: "
LangString LSTR_4 1042 "필요한 디스크 공간: "
LangString LSTR_5 1042 "기록할 수 없음: "
LangString LSTR_6 1042 "복사 실패"
LangString LSTR_7 1042 "파일 복사 "
LangString LSTR_8 1042 "심볼을 찾을 수 없음: "
LangString LSTR_9 1042 "불러올 수 없음: "
LangString LSTR_10 1042 "폴더 생성: "
LangString LSTR_11 1042 "바로 가기 생성: "
LangString LSTR_13 1042 "파일 삭제: "
LangString LSTR_14 1042 "재부팅시 삭제: "
LangString LSTR_15 1042 "바로 가기 생성 오류: "
LangString LSTR_17 1042 "압축 해제중 오류 발생! 설치 파일이 손상되었습니다."
LangString LSTR_20 1042 "실행: "
LangString LSTR_21 1042 "압축 해제: "
LangString LSTR_22 1042 "압축 해제: 파일을 기록하는 도중 오류 발생 "
LangString LSTR_23 1042 "인스톨러 손상됨: 잘못된 실행코드"
LangString LSTR_24 1042 "OLE 정보 없음: "
LangString LSTR_25 1042 "대상 폴더: "
LangString LSTR_26 1042 "폴더 삭제: "
LangString LSTR_27 1042 "재부팅시 이름 변경: "
LangString LSTR_28 1042 "이름 변경: "
LangString LSTR_29 1042 "건너뜀: "
LangString LSTR_30 1042 "자세한 내용을 클립보드로 복사"
LangString LSTR_32 1042 B
LangString LSTR_33 1042 " K"
LangString LSTR_34 1042 " M"
LangString LSTR_35 1042 " G"
LangString LSTR_36 1042 "다음 파일을 열 수 없습니다.: $\r$\n$\t$\"$0$\"$\r$\n'중단'을 눌러 설치를 종료하거나,$\r'다시 시도'를 눌러 다시 시도해 보거나,$\r'무시'를 눌러 이 파일을 건너 뛰세요."
LangString LSTR_37 1042 "설치 위치 선택"
LangString LSTR_38 1042 "$(LSTR_76)(을)를 설치할 폴더를 선택해 주세요."
LangString LSTR_39 1042 설치중
LangString LSTR_40 1042 "$(LSTR_76)(을)를 설치하는 동안 잠시 기다려 주세요."
LangString LSTR_41 1042 "설치 완료"
LangString LSTR_42 1042 "설치가 성공적으로 완료되었습니다."
LangString LSTR_43 1042 "설치 취소"
LangString LSTR_44 1042 "프로그램 설치가 취소되었습니다."
LangString LSTR_45 1042 마침
LangString LSTR_46 1042 0
LangString LSTR_47 1042 "$(LSTR_76) 설치 완료"
LangString LSTR_48 1042 굴림
LangString LSTR_49 1042 "$(LSTR_76)의 설치를 완료하기 위해서는 시스템을 다시 시작해야 합니다. 지금 재부팅 하시겠습니까?"
LangString LSTR_50 1042 "지금 재부팅 하겠습니다."
LangString LSTR_51 1042 "나중에 재부팅 하겠습니다."
LangString LSTR_52 1042 "$(LSTR_76)의 설치가 완료되었습니다.$\r$\n$\r$\n설치 프로그램을 마치려면 '마침' 버튼을 눌러 주세요."
LangString LSTR_53 1042 "$(LSTR_76) 실행하기(&R)"
LangString LSTR_54 1042 9
LangString LSTR_55 1042 "Windows 7 이상이 필요합니다"
LangString LSTR_56 1042 "64 비트 버전의 Windows가 필요합니다"
LangString LSTR_57 1042 "Failed to uninstall old application files. Please try running the installer again."
LangString LSTR_58 1042 "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
LangString LSTR_59 1042 "Shinri Hub이(가) 실행 중입니다. $\r$\nOK을 클릭하면 종료됩니다."
LangString LSTR_60 1042 "Closing running Shinri Hub..."
LangString LSTR_61 1042 "사용자 정의"
LangString LSTR_62 1042 취소
LangString LSTR_63 1042 "$(LSTR_76)(을)를 다음 폴더에 설치할 예정입니다. $\r$\n다른 폴더에 설치하고 싶으시면 '찾아보기' 버튼을 눌러서 다른 폴더를 선택해 주세요. $_CLICK"
LangString LSTR_64 1042 "설치 폴더"
LangString LSTR_65 1042 찾아보기...
LangString LSTR_66 1042 "$(LSTR_76)(을)를 다음 폴더에 설치합니다:"
LangString LSTR_67 1042 "< 뒤로"
LangString LSTR_68 1042 설치
LangString LSTR_69 1042 "설치를 시작하시려면 '설치' 버튼을 눌러 주세요."
LangString LSTR_70 1042 "자세히 보기"
LangString LSTR_71 1042 완료
LangString LSTR_72 1042 "다음 >"
LangString LSTR_73 1042 "계속하시려면 '다음' 버튼을 눌러 주세요."
LangString LSTR_74 1042 " "
LangString LSTR_75 1042 닫음
LangString LSTR_76 1042 "Shinri Hub"


; LANG: 1041
LangString LSTR_0 1041 "Shinri Hub 26.11.23196"
LangString LSTR_1 1041 "$(LSTR_2) セットアップ"
LangString LSTR_2 1041 "Shinri Hub"
LangString LSTR_3 1041 "利用可能なディスクスペース： "
LangString LSTR_4 1041 "必要なディスクスペース： "
LangString LSTR_5 1041 作成できません：
LangString LSTR_6 1041 コピーは失敗しました
LangString LSTR_7 1041 コピーします
LangString LSTR_8 1041 シンボルを見つけることができません：
LangString LSTR_9 1041 ロードすることができません：
LangString LSTR_10 1041 フォルダの作成：
LangString LSTR_11 1041 ショートカットの作成：
LangString LSTR_13 1041 ファイルの削除：
LangString LSTR_14 1041 リブート時に削除：
LangString LSTR_15 1041 ショートカットの作成エラー：
LangString LSTR_17 1041 データの抽出エラー$\r$\n$\r$\nインストーラが破損しています。
LangString LSTR_20 1041 実行：
LangString LSTR_21 1041 抽出：
LangString LSTR_22 1041 抽出：ファイル作成エラー
LangString LSTR_23 1041 インストールの不正：無効なopcode
LangString LSTR_24 1041 OLEがありません：
LangString LSTR_25 1041 出力先フォルダ：
LangString LSTR_26 1041 フォルダの削除：
LangString LSTR_27 1041 リブート時に名前の変更：
LangString LSTR_28 1041 名前の変更：
LangString LSTR_29 1041 スキップ：
LangString LSTR_30 1041 クリップボードへ詳細をコピー
LangString LSTR_32 1041 B
LangString LSTR_33 1041 " K"
LangString LSTR_34 1041 " M"
LangString LSTR_35 1041 " G"
LangString LSTR_36 1041 "初期ファイルの作成エラー：$\r$\n$\t$\"$0$\"$\r$\nインストールを中止するには中止を,$\r$\n再びこのファイルの作成を試みるには再試行を, また$\r$\nこのファイルをスキップして続けるには無視を押してください"
LangString LSTR_37 1041 インストール先を選んでください。
LangString LSTR_38 1041 $(LSTR_76)をインストールするフォルダを選んでください。
LangString LSTR_39 1041 インストール
LangString LSTR_40 1041 $(LSTR_76)をインストールしています。しばらくお待ちください。
LangString LSTR_41 1041 インストールの完了
LangString LSTR_42 1041 インストールに成功しました。
LangString LSTR_43 1041 インストールの中止
LangString LSTR_44 1041 セットアップは正常に完了されませんでした。
LangString LSTR_45 1041 完了(&F)
LangString LSTR_46 1041 0
LangString LSTR_47 1041 "$(LSTR_76) セットアップ ウィザードは完了しました。"
LangString LSTR_48 1041 "ＭＳ Ｐゴシック"
LangString LSTR_49 1041 "$(LSTR_76) のインストールを完了するには、このコンピュータを再起動する必要があります。今すぐ再起動しますか？"
LangString LSTR_50 1041 今すぐ再起動する
LangString LSTR_51 1041 後で手動で再起動する
LangString LSTR_52 1041 "$(LSTR_76)は、このコンピュータにインストールされました。$\r$\n$\r$\nウィザードを閉じるには [完了] を押してください。"
LangString LSTR_53 1041 $(LSTR_76)を実行(&R)
LangString LSTR_54 1041 9
LangString LSTR_55 1041 "Windows 7 以降が必要です"
LangString LSTR_56 1041 "64 ビット バージョンの Windows が必要です"
LangString LSTR_57 1041 古いアプリケーションファイルのアンインストールに失敗しました。もう一度インストーラーを実行してみてください。
LangString LSTR_58 1041 "Shinri Hubが終了できません。$\r$\n手動で閉じて、『再試行』をクリックしてください。"
LangString LSTR_59 1041 "Shinri Hub が起動しています。$\r$\nOKをクリックして、閉じてください。$\r$\n閉じない場合は、手動で閉じてください。"
LangString LSTR_60 1041 "Closing running Shinri Hub..."
LangString LSTR_61 1041 カスタム
LangString LSTR_62 1041 キャンセル
LangString LSTR_63 1041 "$(LSTR_76)を以下のフォルダにインストールします。異なったフォルダにインストールするには、[参照] を押して、別のフォルダを選択してください。 $_CLICK"
LangString LSTR_64 1041 "インストール先 フォルダ"
LangString LSTR_65 1041 参照(&R)...
LangString LSTR_66 1041 $(LSTR_76)をインストールするフォルダを選択してください：
LangString LSTR_67 1041 "< 戻る(&B)"
LangString LSTR_68 1041 インストール
LangString LSTR_69 1041 "インストールを始めるには [インストール] をクリックして下さい。"
LangString LSTR_70 1041 詳細を表示(&D)
LangString LSTR_71 1041 完了
LangString LSTR_72 1041 "次へ(&N) >"
LangString LSTR_73 1041 "続けるには [次へ] をクリックして下さい。"
LangString LSTR_74 1041 " "
LangString LSTR_75 1041 閉じる(&C)
LangString LSTR_76 1041 "Shinri Hub"


; LANG: 1036
LangString LSTR_0 1036 "Shinri Hub 26.11.23196"
LangString LSTR_1 1036 "Installation de $(LSTR_2)"
LangString LSTR_2 1036 "Shinri Hub"
LangString LSTR_3 1036 "Espace disponible : "
LangString LSTR_4 1036 "Espace requis : "
LangString LSTR_5 1036 "Impossible d'écrire : "
LangString LSTR_6 1036 "Échec de la copie"
LangString LSTR_7 1036 "Copier vers "
LangString LSTR_8 1036 "Impossible de trouver un symbole : "
LangString LSTR_9 1036 "Impossible de charger : "
LangString LSTR_10 1036 "Création du dossier : "
LangString LSTR_11 1036 "Création du raccourci : "
LangString LSTR_13 1036 "Suppression : "
LangString LSTR_14 1036 "Suppression au redémarrage : "
LangString LSTR_15 1036 "Erreur lors de la création du raccourci : "
LangString LSTR_17 1036 "Erreur lors de la décompression des données ! Installation corrompue ?"
LangString LSTR_20 1036 "Exécution : "
LangString LSTR_21 1036 "Extraction : "
LangString LSTR_22 1036 "Extraction : erreur d'écriture du fichier "
LangString LSTR_23 1036 "Installation corrompue : opcode incorrect"
LangString LSTR_24 1036 "Pas de OLE pour : "
LangString LSTR_25 1036 "Destination : "
LangString LSTR_26 1036 "Suppression du dossier : "
LangString LSTR_27 1036 "Renommer au redémarrage : "
LangString LSTR_28 1036 "Renommer : "
LangString LSTR_29 1036 "Passé : "
LangString LSTR_30 1036 "Copier les Détails dans le Presse-papier"
LangString LSTR_32 1036 o
LangString LSTR_33 1036 " K"
LangString LSTR_34 1036 " M"
LangString LSTR_35 1036 " G"
LangString LSTR_36 1036 "Erreur lors de l'ouverture du fichier en écriture : $\r$\n$\t$\"$0$\"$\r$\nAppuyez sur Abandonner pour annuler l'installation,$\r$\nRéessayer pour réessayer l'écriture du fichier, ou$\r$\nIgnorer pour passer ce fichier"
LangString LSTR_37 1036 "Choisissez le dossier d'installation"
LangString LSTR_38 1036 "Choisissez le dossier dans lequel installer $(LSTR_76)."
LangString LSTR_39 1036 "Installation en cours"
LangString LSTR_40 1036 "Veuillez patienter pendant l'installation de $(LSTR_76)."
LangString LSTR_41 1036 "Installation terminée"
LangString LSTR_42 1036 "L'installation s'est terminée avec succès."
LangString LSTR_43 1036 "Installation interrompue"
LangString LSTR_44 1036 "L'installation n'a pas été terminée."
LangString LSTR_45 1036 &Fermer
LangString LSTR_46 1036 0
LangString LSTR_47 1036 "Fin de l'installation de $(LSTR_76)"
LangString LSTR_48 1036 "MS Shell Dlg"
LangString LSTR_49 1036 "Votre ordinateur doit être redémarré afin de compléter l'installation de $(LSTR_76). Souhaitez-vous redémarrer maintenant ?"
LangString LSTR_50 1036 "Redémarrer maintenant"
LangString LSTR_51 1036 "Je souhaite redémarrer moi-même plus tard"
LangString LSTR_52 1036 "$(LSTR_76) a été installé sur votre ordinateur.$\r$\n$\r$\nCliquez sur Fermer pour quitter le programme d'installation."
LangString LSTR_53 1036 "Lancer $(LSTR_76)"
LangString LSTR_54 1036 8
LangString LSTR_55 1036 "Windows 7 (ou une version ultérieure) est requis"
LangString LSTR_56 1036 "Windows 64 bits est requis"
LangString LSTR_57 1036 "Échec de désinstallation des anciens fichiers d'application . Veuillez réessayer d’exécuter l'installeur."
LangString LSTR_58 1036 "Shinri Hub ne peut pas être fermé. $\r$\nVeuillez la fermer manuellement et cliquez sur Réessayer pour continuer."
LangString LSTR_59 1036 "Shinri Hub est en cours d’utilisation. $\r$\nCliquez sur «OK» pour fermer ce programme."
LangString LSTR_60 1036 "Closing running Shinri Hub..."
LangString LSTR_61 1036 Personnalisée
LangString LSTR_62 1036 Annuler
LangString LSTR_63 1036 "Ceci installera $(LSTR_76) dans le dossier suivant. Pour installer dans un autre dossier, cliquez sur Parcourir et choisissez un autre dossier. $_CLICK"
LangString LSTR_64 1036 "Dossier d'installation"
LangString LSTR_65 1036 P&arcourir...
LangString LSTR_66 1036 "Sélectionnez le dossier d'installation pour $(LSTR_76) :"
LangString LSTR_67 1036 "< &Précédent"
LangString LSTR_68 1036 &Installer
LangString LSTR_69 1036 "Cliquez sur Installer pour démarrer l'installation."
LangString LSTR_70 1036 "P&lus d'infos"
LangString LSTR_71 1036 Terminé
LangString LSTR_72 1036 "&Suivant >"
LangString LSTR_73 1036 "Cliquez sur Suivant pour continuer."
LangString LSTR_74 1036 " "
LangString LSTR_75 1036 &Fermer
LangString LSTR_76 1036 "Shinri Hub"


; LANG: 1040
LangString LSTR_0 1040 "Shinri Hub 26.11.23196"
LangString LSTR_1 1040 "Installazione di $(LSTR_2)"
LangString LSTR_2 1040 "Shinri Hub"
LangString LSTR_3 1040 "Spazio disponibile: "
LangString LSTR_4 1040 "Spazio richiesto: "
LangString LSTR_5 1040 "Impossibile scrivere: "
LangString LSTR_6 1040 "Copia fallita"
LangString LSTR_7 1040 "Copia in "
LangString LSTR_8 1040 "Impossibile trovare il simbolo: "
LangString LSTR_9 1040 "Impossibile caricare: "
LangString LSTR_10 1040 "Creazione cartella: "
LangString LSTR_11 1040 "Creazione collegamento: "
LangString LSTR_13 1040 "Eliminazione file: "
LangString LSTR_14 1040 "Elimina al riavvio: "
LangString LSTR_15 1040 "Errore nella creazione del collegamento: "
LangString LSTR_17 1040 "Errore nella decompressione dei dati! Probabile programma di installazione corrotto."
LangString LSTR_20 1040 "Esecuzione di: "
LangString LSTR_21 1040 "Estrazione di: "
LangString LSTR_22 1040 "Estrazione: errore nella scrittura nel file "
LangString LSTR_23 1040 "Programma di installazione corrotto: opcode non valido"
LangString LSTR_24 1040 "Nessuna OLE per: "
LangString LSTR_25 1040 "Cartella destinazione: "
LangString LSTR_26 1040 "Rimozione cartella: "
LangString LSTR_27 1040 "Al riavvio rinomina: "
LangString LSTR_28 1040 "Rinomina "
LangString LSTR_29 1040 "Saltato: "
LangString LSTR_30 1040 "Copia i dettagli negli Appunti"
LangString LSTR_32 1040 B
LangString LSTR_33 1040 " K"
LangString LSTR_34 1040 " M"
LangString LSTR_35 1040 " G"
LangString LSTR_36 1040 "Errore nell'apertura del file per la scrittura: $\r$\n$\t$\"$0$\"$\r$\nSeleziona 'Termina' per interrompere l'installazione,$\r$\n'Riprova' per ritentare, oppure$\r$\n'Ignora' per saltare questo file."
LangString LSTR_37 1040 "Selezione cartella installazione"
LangString LSTR_38 1040 "Seleziona la cartella nella quale installare $(LSTR_76)."
LangString LSTR_39 1040 Installazione
LangString LSTR_40 1040 "Attendi il completamento dell'installazione di $(LSTR_76)."
LangString LSTR_41 1040 "Installazione completata"
LangString LSTR_42 1040 "L'installazione è stata completata correttamente."
LangString LSTR_43 1040 "Installazione interrotta"
LangString LSTR_44 1040 "L'installazione non è stata completata correttamente."
LangString LSTR_45 1040 &Chiudi
LangString LSTR_46 1040 0
LangString LSTR_47 1040 "Installazione di $(LSTR_76) completata."
LangString LSTR_48 1040 "MS Shell Dlg"
LangString LSTR_49 1040 "Per completare l'installazione di $(LSTR_76) il computer deve essere riavviato. Vuoi riavviarlo ora?"
LangString LSTR_50 1040 "Riavvia ora"
LangString LSTR_51 1040 "Preferisco riavviarlo manualmente più tardi"
LangString LSTR_52 1040 "$(LSTR_76) è stato installato.$\r$\n$\r$\nScegli 'Chiudi' per chiudere il programma di installazione."
LangString LSTR_53 1040 "Esegui $(LSTR_76)"
LangString LSTR_54 1040 8
LangString LSTR_55 1040 "È necessario disporre del sistema operativo Windows 7 e successivi"
LangString LSTR_56 1040 "È necessario disporre di un sistema operativo Windows a 64 bit"
LangString LSTR_57 1040 "Impossibile disinstallare i vecchi file dell'applicazione. Per favore, prova ad eseguire di nuovo il programma di installazione."
LangString LSTR_58 1040 "Shinri Hub non può essere chiuso. $\r$\nPer favore, chiudilo manualmente e clicca su Riprova per continuare."
LangString LSTR_59 1040 "Shinri Hub è in esecuzione.$\r$\nClicca su OK per chiuderla.$\r$\nSe non si chiude, prova a chiuderla manualmente."
LangString LSTR_60 1040 "Closing running Shinri Hub..."
LangString LSTR_61 1040 Personalizzata
LangString LSTR_62 1040 Annulla
LangString LSTR_63 1040 "Questa procedura installerà $(LSTR_76) in questa cartella.$\r$\nPer installare in una cartella diversa, seleziona 'Sfoglia' e scegli un'altra cartella.$\r$\n$_CLICK"
LangString LSTR_64 1040 "Cartella destinazione"
LangString LSTR_65 1040 S&foglia...
LangString LSTR_66 1040 "Seleziona la cartella dove installare $(LSTR_76):"
LangString LSTR_67 1040 "< &Indietro"
LangString LSTR_68 1040 Ins&talla
LangString LSTR_69 1040 "Per avviare l'installazione, seleziona 'Installa'."
LangString LSTR_70 1040 "Visualizza &dettagli"
LangString LSTR_71 1040 "Installazione completata"
LangString LSTR_72 1040 "&Avanti >"
LangString LSTR_73 1040 "Per proseguire, seleziona 'Avanti'."
LangString LSTR_74 1040 " "
LangString LSTR_75 1040 &Fine
LangString LSTR_76 1040 "Shinri Hub"


; LANG: 1031
LangString LSTR_0 1031 "Shinri Hub 26.11.23196"
LangString LSTR_1 1031 "Installation von $(LSTR_2)"
LangString LSTR_2 1031 "Shinri Hub"
LangString LSTR_3 1031 "Verfügbarer Speicher: "
LangString LSTR_4 1031 "Benötigter Speicher: "
LangString LSTR_5 1031 "Fehler beim Schreiben: "
LangString LSTR_6 1031 "Kopieren fehlgeschlagen"
LangString LSTR_7 1031 "Wird kopiert nach "
LangString LSTR_8 1031 "Symbol ist nicht vorhanden: "
LangString LSTR_9 1031 "Fehler beim Laden von "
LangString LSTR_10 1031 "Verzeichnis wird erstellt: "
LangString LSTR_11 1031 "Verknüpfung wird erstellt: "
LangString LSTR_13 1031 "Datei wird gelöscht: "
LangString LSTR_14 1031 "Datei wird nach Neustart gelöscht: "
LangString LSTR_15 1031 "Fehler beim Erstellen der Verknüpfung: "
LangString LSTR_17 1031 "Fehler beim Entpacken. Ist das Installations-Programm beschädigt?"
LangString LSTR_20 1031 "Wird gestartet: "
LangString LSTR_21 1031 "Wird entpackt: "
LangString LSTR_22 1031 "Entpacken: Fehler beim Schreiben der Datei "
LangString LSTR_23 1031 "Beschädigtes Installations-Programm: ungültiger Befehlscode"
LangString LSTR_24 1031 "Kein OLE für: "
LangString LSTR_25 1031 "Zielverzeichnis: "
LangString LSTR_26 1031 "Verzeichnis wird entfernt: "
LangString LSTR_27 1031 "Umbenennen nach Neustart: "
LangString LSTR_28 1031 "Umbenennen: "
LangString LSTR_29 1031 "Übersprungen: "
LangString LSTR_30 1031 "Details in die Zwischenablage kopieren"
LangString LSTR_32 1031 B
LangString LSTR_33 1031 " K"
LangString LSTR_34 1031 " M"
LangString LSTR_35 1031 " G"
LangString LSTR_36 1031 "Fehler beim Überschreiben der Datei: $\r$\n$\t$\"$0$\"$\r$\nKlicken Sie auf Abbrechen, um abzubrechen,$\r$\nauf Wiederholen, um den Schreibvorgang erneut zu versuchen,$\r$\noder auf Ignorieren, um diese Datei zu überspringen."
LangString LSTR_37 1031 "Zielverzeichnis auswählen"
LangString LSTR_38 1031 "Wählen Sie das Verzeichnis aus, in das $(LSTR_76) installiert werden soll."
LangString LSTR_39 1031 "Wird installiert ..."
LangString LSTR_40 1031 "Bitte warten Sie, während $(LSTR_76) installiert wird."
LangString LSTR_41 1031 "Die Installation ist vollständig"
LangString LSTR_42 1031 "Die Installation wurde erfolgreich abgeschlossen."
LangString LSTR_43 1031 "Abbruch der Installation"
LangString LSTR_44 1031 "Die Installation wurde nicht vollständig abgeschlossen."
LangString LSTR_45 1031 &Fertigstellen
LangString LSTR_46 1031 0
LangString LSTR_47 1031 "Die Installation von $(LSTR_76) wird abgeschlossen"
LangString LSTR_48 1031 "MS Shell Dlg"
LangString LSTR_49 1031 "Windows muss neu gestartet werden, um die Installation von $(LSTR_76) zu vervollständigen. Möchten Sie Windows jetzt neu starten?"
LangString LSTR_50 1031 "Jetzt neu starten"
LangString LSTR_51 1031 "Windows später selbst neu starten"
LangString LSTR_52 1031 "$(LSTR_76) wurde auf Ihrem Computer installiert.$\r$\n$\r$\nKlicken Sie auf Fertigstellen, um den Installations-Assistenten zu schließen."
LangString LSTR_53 1031 "$(LSTR_76) ausführen"
LangString LSTR_54 1031 8
LangString LSTR_55 1031 "Windows 7 oder höher wird vorausgesetzt"
LangString LSTR_56 1031 "64-Bit-Windows wird vorausgesetzt"
LangString LSTR_57 1031 "Die alten Anwendungsdateien konnten nicht deinstalliert werden. Bitte versuche, das Installationsprogramm erneut auszuführen."
LangString LSTR_58 1031 "Shinri Hub kann nicht geschlossen werden. $\r$\nSchließ es bitte manuell und klicke auf »Wiederholen«, um fortzufahren."
LangString LSTR_59 1031 "Shinri Hub wird gerade ausgeführt.$\r$\nKlicke auf »OK«, um es zu schließen.$\r$\nFalls es sich nicht schließen lässt, versuche es manuell zu schließen."
LangString LSTR_60 1031 "Closing running Shinri Hub..."
LangString LSTR_61 1031 Benutzerdefiniert
LangString LSTR_62 1031 Abbrechen
LangString LSTR_63 1031 "$(LSTR_76) wird in das unten angegebene Verzeichnis installiert. Falls Sie in ein anderes Verzeichnis installieren möchten, klicken Sie auf Durchsuchen und wählen Sie ein anderes Verzeichnis aus. $_CLICK"
LangString LSTR_64 1031 Zielverzeichnis
LangString LSTR_65 1031 "&Durchsuchen ..."
LangString LSTR_66 1031 "Wählen Sie das Verzeichnis aus, in das Sie $(LSTR_76) installieren möchten:"
LangString LSTR_67 1031 "< &Zurück"
LangString LSTR_68 1031 &Installieren
LangString LSTR_69 1031 "Klicken Sie auf Installieren, um die Installation zu starten."
LangString LSTR_70 1031 "&Details anzeigen"
LangString LSTR_71 1031 Fertig
LangString LSTR_72 1031 "&Weiter >"
LangString LSTR_73 1031 "Klicken Sie auf Weiter, um fortzufahren."
LangString LSTR_74 1031 " "
LangString LSTR_75 1031 &Beenden
LangString LSTR_76 1031 "Shinri Hub"


; LANG: 1033
LangString LSTR_0 1033 "Shinri Hub 26.11.23196"
LangString LSTR_1 1033 "$(LSTR_2) Setup"
LangString LSTR_2 1033 "Shinri Hub"
LangString LSTR_3 1033 "Space available: "
LangString LSTR_4 1033 "Space required: "
LangString LSTR_5 1033 "Can't write: "
LangString LSTR_6 1033 "Copy failed"
LangString LSTR_7 1033 "Copy to "
LangString LSTR_8 1033 "Could not find symbol: "
LangString LSTR_9 1033 "Could not load: "
LangString LSTR_10 1033 "Create folder: "
LangString LSTR_11 1033 "Create shortcut: "
LangString LSTR_13 1033 "Delete file: "
LangString LSTR_14 1033 "Delete on reboot: "
LangString LSTR_15 1033 "Error creating shortcut: "
LangString LSTR_17 1033 "Error decompressing data! Corrupted installer?"
LangString LSTR_20 1033 "Execute: "
LangString LSTR_21 1033 "Extract: "
LangString LSTR_22 1033 "Extract: error writing to file "
LangString LSTR_23 1033 "Installer corrupted: invalid opcode"
LangString LSTR_24 1033 "No OLE for: "
LangString LSTR_25 1033 "Output folder: "
LangString LSTR_26 1033 "Remove folder: "
LangString LSTR_27 1033 "Rename on reboot: "
LangString LSTR_28 1033 "Rename: "
LangString LSTR_29 1033 "Skipped: "
LangString LSTR_30 1033 "Copy Details To Clipboard"
LangString LSTR_32 1033 B
LangString LSTR_33 1033 " K"
LangString LSTR_34 1033 " M"
LangString LSTR_35 1033 " G"
LangString LSTR_36 1033 "Error opening file for writing: $\r$\n$\r$\n$0$\r$\n$\r$\nClick Abort to stop the installation,$\r$\nRetry to try again, or$\r$\nIgnore to skip this file."
LangString LSTR_37 1033 "Choose Install Location"
LangString LSTR_38 1033 "Choose the folder in which to install $(LSTR_76)."
LangString LSTR_39 1033 Installing
LangString LSTR_40 1033 "Please wait while $(LSTR_76) is being installed."
LangString LSTR_41 1033 "Installation Complete"
LangString LSTR_42 1033 "Setup was completed successfully."
LangString LSTR_43 1033 "Installation Aborted"
LangString LSTR_44 1033 "Setup was not completed successfully."
LangString LSTR_45 1033 &Finish
LangString LSTR_46 1033 0
LangString LSTR_47 1033 "Completing $(LSTR_76) Setup"
LangString LSTR_48 1033 "MS Shell Dlg"
LangString LSTR_49 1033 "Your computer must be restarted in order to complete the installation of $(LSTR_76). Do you want to reboot now?"
LangString LSTR_50 1033 "Reboot now"
LangString LSTR_51 1033 "I want to manually reboot later"
LangString LSTR_52 1033 "$(LSTR_76) has been installed on your computer.$\r$\n$\r$\nClick Finish to close Setup."
LangString LSTR_53 1033 "&Run $(LSTR_76)"
LangString LSTR_54 1033 8
LangString LSTR_55 1033 "Windows 7 and above is required"
LangString LSTR_56 1033 "64-bit Windows is required"
LangString LSTR_57 1033 "Failed to uninstall old application files. Please try running the installer again."
LangString LSTR_58 1033 "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
LangString LSTR_59 1033 "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
LangString LSTR_60 1033 "Closing running Shinri Hub..."
LangString LSTR_61 1033 Custom
LangString LSTR_62 1033 Cancel
LangString LSTR_63 1033 "Setup will install $(LSTR_76) in the following folder. To install in a different folder, click Browse and select another folder. $_CLICK"
LangString LSTR_64 1033 "Destination Folder"
LangString LSTR_65 1033 B&rowse...
LangString LSTR_66 1033 "Select the folder to install $(LSTR_76) in:"
LangString LSTR_67 1033 "< &Back"
LangString LSTR_68 1033 &Install
LangString LSTR_69 1033 "Click Install to start the installation."
LangString LSTR_70 1033 "Show &details"
LangString LSTR_71 1033 Completed
LangString LSTR_72 1033 "&Next >"
LangString LSTR_73 1033 "Click Next to continue."
LangString LSTR_74 1033 " "
LangString LSTR_75 1033 &Close
LangString LSTR_76 1033 "Shinri Hub"


; --------------------
; VARIABLES: 68

Var _0_
Var _1_
Var _2_
Var _3_
Var _4_
Var _5_
Var _6_
Var _7_
Var _8_
Var _9_
Var _10_
Var _11_
Var _12_
Var _13_
Var _14_
Var _15_
Var _16_
Var _17_
Var _18_
Var _19_
Var _20_
Var _21_
Var _22_
Var _23_
Var _24_
Var _25_
Var _26_
Var _27_
Var _28_
Var _29_
Var _30_
Var _31_
Var _32_
Var _33_
Var _34_
Var _35_
Var _36_
Var _37_
Var _38_
Var _39_
Var _40_
Var _41_
Var _42_
Var _43_
Var _44_
Var _45_
Var _46_
Var _47_
Var _48_
Var _49_
Var _50_
Var _51_
Var _52_
Var _53_
Var _54_
Var _55_
Var _56_
Var _57_
Var _58_
Var _59_
Var _60_
Var _61_
Var _62_
Var _63_
Var _64_
Var _65_
Var _66_
Var _67_


InstType $(LSTR_61)    ;  Custom
; wininit = $WINDIR\wininit.ini


; --------------------
; PAGES: 4

; Page 0
Page directory func_152 func_156 func_164 /ENABLECANCEL
  DirText $(LSTR_63) $(LSTR_64) $(LSTR_65) $(LSTR_66)    ;  "Setup will install $(LSTR_76) in the following folder. To install in a different folder, click Browse and select another folder. $_CLICK" "Destination Folder" B&rowse... "Select the folder to install $(LSTR_76) in:" "Shinri Hub" "Shinri Hub"
  DirVar $CMDLINE

; Page 1
Page instfiles func_172 func_176 func_182
  CompletedText $(LSTR_71)    ;  Completed
  DetailsButtonText $(LSTR_70)    ;  "Show &details"

/*
; Page 2
Page COMPLETED
*/

; Page 3
Page custom func_234 func_456


; --------------------
; SECTIONS: 1
; COMMANDS: 1316

Function func_0
  Exch $R3
    ; Push $R3
    ; Exch
    ; Pop $R3
  Push $0
  Push $1
  Push $2
  Push $3
  Push $4
  Push $5
  Push $R0
  Push $R1
  Push $R9
  Push $R8
  StrCpy $0 ""
  StrCpy $1 ""
  StrCpy $2 ""
  StrCpy $3 ""
  StrCpy $4 ""
  StrCpy $5 ""
  IntCmp $R3 0 0 label_30 label_30
  System::Call "kernel32::GetCurrentProcess() i.R0"
    ; Call Initialize_____Plugins
    ; SetOverwrite off
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "kernel32::GetCurrentProcess() i.R0"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call "Kernel32::GetProcessId(i R0) i.R3"
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "Kernel32::GetProcessId(i R0) i.R3"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_30:
  System::Call "Kernel32::CreateToolhelp32Snapshot(i 2,i R3) i.R0"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "Kernel32::CreateToolhelp32Snapshot(i 2,i R3) i.R0"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  IntCmp $R0 -1 label_92
  System::Alloc 1024
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push 1024
    ; CallInstDLL $PLUGINSDIR\System.dll Alloc
  Pop $R9
  System::Call "*$R9(i 556)"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "*$R9(i 556)"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call "Kernel32::Process32FirstW(i R0, i $R9) i.R1"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "Kernel32::Process32FirstW(i R0, i $R9) i.R1"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  StrCmp $R1 0 label_92
label_53:
  System::Call *$R9(i,i,i.R1)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push *$R9(i,i,i.R1)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  IntCmp $R1 $R3 label_65
  System::Call "Kernel32::Process32NextW(i R0, i $R9) i.R1"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "Kernel32::Process32NextW(i R0, i $R9) i.R1"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  IntCmp $R1 0 0 label_53 label_53
label_65:
  System::Call *$R9(i,i,i.r0,i,i,i.r1,i.r2,i.r3,i,&w256.r4)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push *$R9(i,i,i.r0,i,i,i.r1,i.r2,i.r3,i,&w256.r4)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Free $R9
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push $R9
    ; CallInstDLL $PLUGINSDIR\System.dll Free
  System::Call "Kernel32::CloseToolhelp32Snapshot(i R0)"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "Kernel32::CloseToolhelp32Snapshot(i R0)"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call "Kernel32::OpenProcess(i 1040, i 0, i r0)i .R0"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "Kernel32::OpenProcess(i 1040, i 0, i r0)i .R0"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  StrCmp $R0 0 label_92
  IntOp $R8 0 + 256
  System::Call "psapi::GetModuleFileNameExW(i R0,i 0,t .r5, *i $R8)i .R1"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "psapi::GetModuleFileNameExW(i R0,i 0,t .r5, *i $R8)i .R1"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_92:
  Pop $R8
  Pop $R9
  Pop $R1
  Pop $R0
  Exch $5
    ; Push $5
    ; Exch
    ; Pop $5
  Exch
  Exch $4
    ; Push $4
    ; Exch
    ; Pop $4
  Exch 2
  Exch $3
    ; Push $3
    ; Exch
    ; Pop $3
  Exch 3
  Exch $2
    ; Push $2
    ; Exch
    ; Pop $2
  Exch 4
  Pop $1
  Exch 4
  Exch $0
    ; Push $0
    ; Exch
    ; Pop $0
  Exch 5
  Pop $R3
FunctionEnd


Function func_120
  Exch $_24_
    ; Push $_24_
    ; Exch
    ; Pop $_24_
  Exch
  Exch $_23_
    ; Push $_23_
    ; Exch
    ; Pop $_23_
  StrCpy $_29_ ""
  StrCpy $_25_ -1
  StrLen $_26_ $_24_
  StrLen $_28_ $_23_
label_131:
  IntOp $_25_ $_25_ + 1
  StrCpy $_27_ $_23_ $_26_ $_25_
  StrCmp $_27_ $_24_ label_136
  StrCmp $_25_ $_28_ label_138
  Goto label_131
label_136:
  StrCpy $_29_ $_24_
  Goto label_138
label_138:
  Pop $_24_
  Exch $_29_
    ; Push $_29_
    ; Exch
    ; Pop $_29_
FunctionEnd


Function func_143
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; AllowSkipFiles on
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_151
  Abort
label_151:
FunctionEnd


Function func_152    ; Page 0, Pre
  Call func_143
  SendMessage $_6_ ${WM_SETTEXT} 0 STR:$(LSTR_37)    ;  "Choose Install Location"
  SendMessage $_8_ ${WM_SETTEXT} 0 STR:$(LSTR_38)    ;  "Choose the folder in which to install $(LSTR_76)." "Shinri Hub"
FunctionEnd


Function func_156    ; Page 0, Show
  FindWindow $_30_ "#32770" "" $HWNDPARENT
  GetDlgItem $_31_ $_30_ 1006
  GetDlgItem $_32_ $_30_ 1020
  GetDlgItem $_33_ $_30_ 1019
  GetDlgItem $_34_ $_30_ 1001
  GetDlgItem $_35_ $_30_ 1023
  GetDlgItem $_36_ $_30_ 1024
FunctionEnd


Function func_164    ; Page 0, Leave
FunctionEnd


Function func_165
  Push $INSTDIR
  Push "Shinri Hub"
  Call func_120
  Pop $0
  StrCmp $0 "" 0 label_171
  StrCpy $INSTDIR "$INSTDIR\Shinri Hub"
label_171:
FunctionEnd


Function func_172    ; Page 1, Pre
  Call func_165
  SendMessage $_6_ ${WM_SETTEXT} 0 STR:$(LSTR_39)    ;  Installing
  SendMessage $_8_ ${WM_SETTEXT} 0 STR:$(LSTR_40)    ;  "Please wait while $(LSTR_76) is being installed." "Shinri Hub"
FunctionEnd


Function func_176    ; Page 1, Show
  FindWindow $_37_ "#32770" "" $HWNDPARENT
  GetDlgItem $_38_ $_37_ 1006
  GetDlgItem $_39_ $_37_ 1004
  GetDlgItem $_40_ $_37_ 1027
  GetDlgItem $_41_ $_37_ 1016
FunctionEnd


Function func_182    ; Page 1, Leave
  IfAbort label_186
  SendMessage $_6_ ${WM_SETTEXT} 0 STR:$(LSTR_41)    ;  "Installation Complete"
  SendMessage $_8_ ${WM_SETTEXT} 0 STR:$(LSTR_42)    ;  "Setup was completed successfully."
  Goto label_188
label_186:
  SendMessage $_6_ ${WM_SETTEXT} 0 STR:$(LSTR_43)    ;  "Installation Aborted"
  SendMessage $_8_ ${WM_SETTEXT} 0 STR:$(LSTR_44)    ;  "Setup was not completed successfully."
label_188:
  IfAbort label_189
label_189:
FunctionEnd


Function func_190
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_199
  StrCpy $1 --updated
  Goto label_200
label_199:
  StrCpy $1 ""
label_200:
  Push $_22_
  Push open
  Push $1
  StdUtils::ExecShellAsUser /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD ExecShellAsUser
  Pop $0
FunctionEnd


Function func_209
  InitPluginsDir
    ; Call Initialize_____Plugins
    ; SetDetailsPrint lastused
  SetOverwrite on
  AllowSkipFiles on
  File $PLUGINSDIR\modern-wizard.bmp
  SetAutoClose true
FunctionEnd


Function func_214
  LockWindow on
  ShowWindow $_12_ ${SW_HIDE}
  ShowWindow $_11_ ${SW_HIDE}
  ShowWindow $_6_ ${SW_HIDE}
  ShowWindow $_8_ ${SW_HIDE}
  ShowWindow $_10_ ${SW_HIDE}
  ShowWindow $_13_ ${SW_HIDE}
  ShowWindow $_14_ ${SW_SHOWNORMAL}
  LockWindow off
FunctionEnd


Function func_224
  LockWindow on
  ShowWindow $_12_ ${SW_SHOWNORMAL}
  ShowWindow $_11_ ${SW_SHOWNORMAL}
  ShowWindow $_6_ ${SW_SHOWNORMAL}
  ShowWindow $_8_ ${SW_SHOWNORMAL}
  ShowWindow $_10_ ${SW_SHOWNORMAL}
  ShowWindow $_13_ ${SW_SHOWNORMAL}
  ShowWindow $_14_ ${SW_HIDE}
  LockWindow off
FunctionEnd


Function func_234    ; Page 3, Pre
  SendMessage $_15_ ${WM_SETTEXT} 0 STR:$(LSTR_45)    ;  &Finish
  nsDialogs::Create 1044
    ; Call Initialize_____Plugins
    ; SetOverwrite off
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push 1044
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll Create
  Pop $_42_
  nsDialogs::SetRTL $(LSTR_46)    ;  0
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_46)    ;  0
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll SetRTL
  SetCtlColors $_42_ "" 0xFFFFFF
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x0000000E|0x00000100 0 0u 0u 109u 193u ""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push ""
    ; Push 193u
    ; Push 109u
    ; Push 0u
    ; Push 0u
    ; Push 0
    ; Push 0x40000000|0x10000000|0x04000000|0x0000000E|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_43_
  Push $0
  Push $R0
  Push $PLUGINSDIR\modern-wizard.bmp
  StrCpy $R0 $_43_
  System::Call user32::GetClientRect(pR0,@r0)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push user32::GetClientRect(pR0,@r0)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call *$0(i,i,i.r0,i.s)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push *$0(i,i,i.r0,i.s)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Exch
  System::Call "user32::LoadImage(p0, ts, i0, ir0, is, i0x0010) p.r0"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "user32::LoadImage(p0, ts, i0, ir0, is, i0x0010) p.r0"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  SendMessage $R0 0x0172 0 $0
  Pop $R0
  Exch $0
    ; Push $0
    ; Exch
    ; Pop $0
  Pop $_44_
  IfRebootFlag 0 label_352
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 10u 195u 28u $(LSTR_47)    ;  "Completing $(LSTR_76) Setup" "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_47)    ;  "Completing $(LSTR_76) Setup" "Shinri Hub"
    ; Push 28u
    ; Push 195u
    ; Push 10u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_45_
  SetCtlColors $_45_ 0x000000 0xFFFFFF
  CreateFont $_46_ $(LSTR_48) 12 700    ;  "MS Shell Dlg"
  SendMessage $_45_ ${WM_SETFONT} $_46_ 0
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 45u 195u 40u $(LSTR_49)    ;  "Your computer must be restarted in order to complete the installation of $(LSTR_76). Do you want to reboot now?" "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_49)    ;  "Your computer must be restarted in order to complete the installation of $(LSTR_76). Do you want to reboot now?" "Shinri Hub"
    ; Push 40u
    ; Push 195u
    ; Push 45u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_47_
  SetCtlColors $_47_ 0x000000 0xFFFFFF
  nsDialogs::CreateControl BUTTON 0x40000000|0x10000000|0x04000000|0x00010000|0x00000000|0x00000C00|0x00000009|0x00002000 0 120u 90u 195u 10u $(LSTR_50)    ;  "Reboot now"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_50)    ;  "Reboot now"
    ; Push 10u
    ; Push 195u
    ; Push 90u
    ; Push 120u
    ; Push 0
    ; Push 0x40000000|0x10000000|0x04000000|0x00010000|0x00000000|0x00000C00|0x00000009|0x00002000
    ; Push BUTTON
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_50_
  SetCtlColors $_50_ 0x000000 0xFFFFFF
  nsDialogs::CreateControl BUTTON 0x40000000|0x10000000|0x04000000|0x00010000|0x00000000|0x00000C00|0x00000009|0x00002000 0 120u 115u 195u 10u $(LSTR_51)    ;  "I want to manually reboot later"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_51)    ;  "I want to manually reboot later"
    ; Push 10u
    ; Push 195u
    ; Push 115u
    ; Push 120u
    ; Push 0
    ; Push 0x40000000|0x10000000|0x04000000|0x00010000|0x00000000|0x00000C00|0x00000009|0x00002000
    ; Push BUTTON
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_51_
  SetCtlColors $_51_ 0x000000 0xFFFFFF
  SendMessage $_50_ 0x00F1 1 0
  System::Call user32::SetFocus(p$_50_)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push user32::SetFocus(p$_50_)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Goto label_402
label_352:
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 10u 195u 28u $(LSTR_47)    ;  "Completing $(LSTR_76) Setup" "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_47)    ;  "Completing $(LSTR_76) Setup" "Shinri Hub"
    ; Push 28u
    ; Push 195u
    ; Push 10u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_45_
  SetCtlColors $_45_ 0x000000 0xFFFFFF
  CreateFont $_46_ $(LSTR_48) 12 700    ;  "MS Shell Dlg"
  SendMessage $_45_ ${WM_SETFONT} $_46_ 0
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 45u 195u 40u $(LSTR_52)    ;  "$(LSTR_76) has been installed on your computer.$\r$\n$\r$\nClick Finish to close Setup." "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_52)    ;  "$(LSTR_76) has been installed on your computer.$\r$\n$\r$\nClick Finish to close Setup." "Shinri Hub"
    ; Push 40u
    ; Push 195u
    ; Push 45u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_47_
  SetCtlColors $_47_ 0x000000 0xFFFFFF
  nsDialogs::CreateControl BUTTON 0x40000000|0x10000000|0x04000000|0x00010000|0x00000000|0x00000C00|0x00000003|0x00002000 0 120u 90u 195u 10u $(LSTR_53)    ;  "&Run $(LSTR_76)" "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_53)    ;  "&Run $(LSTR_76)" "Shinri Hub"
    ; Push 10u
    ; Push 195u
    ; Push 90u
    ; Push 120u
    ; Push 0
    ; Push 0x40000000|0x10000000|0x04000000|0x00010000|0x00000000|0x00000C00|0x00000003|0x00002000
    ; Push BUTTON
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_49_
  SetCtlColors $_49_ 0x000000 0xFFFFFF
  SendMessage $_49_ 0x00F1 1 0
  System::Call user32::SetFocus(p$_49_)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push user32::SetFocus(p$_49_)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_402:
  Push $1
  System::Call *(i12,i0,p)p.r1
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push *(i12,i0,p)p.r1
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call USER32::SystemParametersInfo(i0x42,i12,pr1,i0)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push USER32::SystemParametersInfo(i0x42,i12,pr1,i0)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call *$1(i,i.s)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push *$1(i,i.s)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Pop $_52_
  System::Free $1
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push $1
    ; CallInstDLL $PLUGINSDIR\System.dll Free
  Pop $1
  IntOp $_52_ $_52_ & 0x01
  IntCmp $_52_ 0 label_442
  System::Call "UXTHEME::SetWindowTheme(p$_50_,w$\" $\",w$\" $\")"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "UXTHEME::SetWindowTheme(p$_50_,w$\" $\",w$\" $\")"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call "UXTHEME::SetWindowTheme(p$_51_,w$\" $\",w$\" $\")"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "UXTHEME::SetWindowTheme(p$_51_,w$\" $\",w$\" $\")"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call "UXTHEME::SetWindowTheme(p$_49_,w$\" $\",w$\" $\")"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "UXTHEME::SetWindowTheme(p$_49_,w$\" $\",w$\" $\")"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_442:
  Call func_214
  nsDialogs::Show
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll Show
  Call func_224
  IntCmp $_44_ 0 label_455
  System::Call gdi32::DeleteObject(ps) $_44_
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push $_44_
    ; Push gdi32::DeleteObject(ps)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_455:
FunctionEnd


Function func_456    ; Page 3, Leave
  IfRebootFlag 0 label_463
  SendMessage $_50_ 0x00F0 0 0 $_48_
  IntCmp $_48_ 1 0 label_462 label_462
  Reboot
    ; Quit
  Goto label_463
label_462:
  Return

label_463:
  SendMessage $_49_ 0x00F0 0 0 $_48_
  IntCmp $_48_ 1 0 label_466 label_466
  Call func_190
label_466:
FunctionEnd


Function .onGUIInit
  GetDlgItem $_6_ $HWNDPARENT 1037
  CreateFont $_7_ $(LSTR_48) $(LSTR_54) 700    ;  "MS Shell Dlg" 8
  SendMessage $_6_ ${WM_SETFONT} $_7_ 0
  GetDlgItem $_8_ $HWNDPARENT 1038
  SetCtlColors $_6_ 0x000000 0xFFFFFF
  SetCtlColors $_8_ 0x000000 0xFFFFFF
  GetDlgItem $_9_ $HWNDPARENT 1034
  SetCtlColors $_9_ "" 0xFFFFFF
  GetDlgItem $_10_ $HWNDPARENT 1039
  SetCtlColors $_10_ "" 0xFFFFFF
  GetDlgItem $_12_ $HWNDPARENT 1028
  SetCtlColors $_12_ /BRANDING ""
  GetDlgItem $_11_ $HWNDPARENT 1256
  SetCtlColors $_11_ /BRANDING ""
  SendMessage $_11_ ${WM_SETTEXT} 0 "STR:$(LSTR_0) "    ;  "Shinri Hub 26.11.23196"
  GetDlgItem $_13_ $HWNDPARENT 1035
  GetDlgItem $_14_ $HWNDPARENT 1045
  GetDlgItem $_15_ $HWNDPARENT 1
  GetDlgItem $_16_ $HWNDPARENT 2
  GetDlgItem $_17_ $HWNDPARENT 3
  Call func_209
FunctionEnd


Function .onUserAbort
FunctionEnd


Function .onInit
  Call func_1300
  SetOutPath $INSTDIR
  Call :label_494
  Goto label_582
label_494:
  StrCmp $_53_ "" label_496
  Return

label_496:
  Push $0
  Push $1
  Push $2
  Push $3
  Push $R0
  System::Call *(&i284)p.r0
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push *(&i284)p.r0
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call "*$0(i 284)"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "*$0(i 284)"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call kernel32::GetVersionEx(pr0)i.r3
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::GetVersionEx(pr0)i.r3
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  IntCmp $3 0 0 label_527 label_527
  System::Call "*$0(i 276)"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "*$0(i 276)"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call kernel32::GetVersionEx(pr0)i.r3
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::GetVersionEx(pr0)i.r3
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_527:
  System::Call *$0(i.s,i.r1,i.r2,i.r3,i.s,&t128.s,&i2.s,&i2,&i2,&i1.s,&i1)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push *$0(i.s,i.r1,i.r2,i.r3,i.s,&t128.s,&i2.s,&i2,&i2,&i1.s,&i1)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Free $0
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push $0
    ; CallInstDLL $PLUGINSDIR\System.dll Free
  IntOp $3 $3 & 0xFFFF
  Pop $R0
  Pop $0
  IntCmp $0 2 0 label_543 label_543
  IntOp $_54_ $_54_ | 0x80000000
  IntOp $_53_ $_53_ | 0x80000000
label_543:
  IntCmp $R0 284 0 label_553 label_553
  Pop $0
  Exch
  Pop $0
  IntCmp $0 1 label_551
  IntOp $_54_ $_54_ | 0x40000000
  IntCmp $0 2 0 label_551 label_551
  IntOp $_54_ $_54_ | 0x20000000
label_551:
  Pop $0
  Goto label_561
label_553:
  Pop $0
  Pop $R0
  Pop $R0
  StrCpy $R0 $0 13
  StrCmp $R0 "Service Pack " 0 label_560
  StrCpy $0 $0 "" 13
  Goto label_561
label_560:
  StrCpy $0 0
label_561:
  IntOp $0 $0 << 16
  IntOp $_54_ $_54_ | $0
  IntOp $0 $_54_ & 0x40000000
  IntCmp $0 0 0 label_568 label_568
  IntCmp $1 5 0 label_568 label_568
  IntCmp $2 2 0 label_568 label_568
  StrCpy $2 1
label_568:
  IntCmp $0 0 label_571
  IntCmp 6 $1 0 0 label_571
  IntOp $_53_ $_53_ | 0x00000001
label_571:
  IntOp $1 $1 << 24
  IntOp $_53_ $_53_ | $1
  IntOp $0 $2 << 16
  IntOp $_53_ $_53_ | $0
  IntOp $_54_ $_54_ | $3
  Pop $R0
  Pop $3
  Pop $2
  Pop $1
  Pop $0
  Return

label_582:
  IntCmp $_53_ 0x85000000 0 label_584 label_584
  Goto label_586
label_584:
  Call :label_494
  IntCmp $_53_ 0x045a0000 0 label_587 label_587
label_586:
  Goto label_589
label_587:
  Call :label_494
  IntCmp $_53_ 0x85010000 0 label_590 label_590
label_589:
  Goto label_592
label_590:
  Call :label_494
  IntCmp $_53_ 0x86000000 0 label_594 label_594
label_592:
  MessageBox MB_OK $(LSTR_55)    ;  "Windows 7 and above is required"
  Quit
label_594:
  System::Call kernel32::GetCurrentProcess()p.s
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::GetCurrentProcess()p.s
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call kernel32::IsWow64Process2(ps,*i0s,*i)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::IsWow64Process2(ps,*i0s,*i)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Push |
  System::Call kernel32::IsWow64Process(p-1,*i0s)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::IsWow64Process(p-1,*i0s)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Int64Op
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\System.dll Int64Op
  Pop $_52_
  StrCmp $_52_ 0 label_618
  SetRegView 64
  Goto label_620
label_618:
  MessageBox MB_OK|MB_ICONEXCLAMATION $(LSTR_56)    ;  "64-bit Windows is required"
  Quit
label_620:
  UAC::_ 3s
    ; Call Initialize_____Plugins
    ; AllowSkipFiles on
    ; File $PLUGINSDIR\UAC.dll
    ; SetDetailsPrint lastused
    ; Push 3s
    ; CallInstDLL $PLUGINSDIR\UAC.dll _
  Pop $_52_
  StrCmp $_52_ 0 0 label_654
    ; ShowWindow $HWNDPARENT ${SW_SHOW}
  BringToFront
  System::Call "kernel32::CreateMutex(p0, i1, t$\"2546e024-4dd1-5359-9177-3b6a84280dd8$\")?e"
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "kernel32::CreateMutex(p0, i1, t$\"2546e024-4dd1-5359-9177-3b6a84280dd8$\")?e"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Pop $0
  IntCmpU $0 183 0 label_654 label_654
  StrLen $0 $(LSTR_1)    ;  "$(LSTR_2) Setup" "Shinri Hub"
  IntOp $0 $0 + 1
  StrCpy $1 ""
label_639:
  FindWindow $1 "#32770" "" "" $1
  StrCmp 0 $1 label_653
  System::Call "user32::GetWindowText(pr1, t.r2, ir0)"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "user32::GetWindowText(pr1, t.r2, ir0)"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  StrCmp $2 $(LSTR_1) 0 label_639    ;  "$(LSTR_2) Setup" "Shinri Hub"
  SendMessage $1 0x112 0xF120 0 /TIMEOUT=2000
  System::Call user32::SetForegroundWindow(pr1)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push user32::SetForegroundWindow(pr1)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_653:
  Abort
label_654:
  StrCpy $_18_ all
  SetShellVarContext all
  ReadRegStr $_19_ HKLM Software\2546e024-4dd1-5359-9177-3b6a84280dd8 InstallLocation
  StrCmp $_19_ "" label_660
  StrCpy $INSTDIR $_19_
  Goto label_685
label_660:
  StrCpy $0 $PROGRAMFILES
  System::Call kernel32::GetCurrentProcess()p.s
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::GetCurrentProcess()p.s
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call kernel32::IsWow64Process2(ps,*i0s,*i)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::IsWow64Process2(ps,*i0s,*i)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Push |
  System::Call kernel32::IsWow64Process(p-1,*i0s)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::IsWow64Process(p-1,*i0s)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Int64Op
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\System.dll Int64Op
  Pop $_52_
  StrCmp $_52_ 0 label_684
  StrCpy $0 $PROGRAMFILES64
label_684:
  StrCpy $INSTDIR "$0\Shinri Hub"
label_685:
  Push $R8
  Push $R9
  Push $R7
  Push $R6
  Push $R5
  Push 0
  StdUtils::GetAllParameters /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD GetAllParameters
  Pop $R8
  StrCpy $R9 ""
  StrLen $R7 $R8
  IntOp $R7 $R7 - 2
  StrCpy $R6 0
label_700:
  StrCpy $R5 $R8 3 $R6
  StrCmp $R5 /D= 0 label_703
  Goto label_704
label_703:
  StrCmp $R5 /d= 0 label_707
label_704:
  IntOp $R6 $R6 + 3
  StrCpy $R9 $R8 "" $R6
  Goto label_711
label_707:
  IntOp $R6 $R6 + 1
label_708:
  IntCmp $R6 $R7 label_700 label_700 label_711
  Goto label_708
  Goto label_711
label_711:
  StrCpy $R0 $R9
  Pop $R5
  Pop $R6
  Pop $R7
  Pop $R9
  Pop $R8
  StrCmp $R0 "" label_719
  StrCpy $INSTDIR $R0
label_719:
FunctionEnd


Function func_720
  Exch $R0
    ; Push $R0
    ; Exch
    ; Pop $R0
  Push $R1
  Push $R2
  Push $R3
  StrCpy $R2 -1
label_727:
  IntOp $R2 $R2 + 1
  StrCpy $R3 $R0 1 $R2
  StrCmp $R3 "" 0 label_732
  StrCpy $R0 ""
  Goto label_743
label_732:
  StrCmp $R3 $\" 0 label_727
  IntOp $R2 $R2 + 1
  StrCpy $R0 $R0 "" $R2
  StrCpy $R2 0
label_736:
  IntOp $R2 $R2 + 1
  StrCpy $R3 $R0 1 $R2
  StrCmp $R3 "" 0 label_741
  StrCpy $R0 ""
  Goto label_743
label_741:
  StrCmp $R3 $\" 0 label_736
  StrCpy $R0 $R0 $R2
label_743:
  Pop $R3
  Pop $R2
  Pop $R1
  Exch $R0
    ; Push $R0
    ; Exch
    ; Pop $R0
FunctionEnd


Function func_750
  Exch $R0
    ; Push $R0
    ; Exch
    ; Pop $R0
  Push $R1
  Push $R2
  Push $R3
  StrCpy $R1 0
  StrLen $R2 $R0
label_758:
  IntOp $R1 $R1 + 1
  IntCmp $R1 $R2 label_763 0 label_763
  StrCpy $R3 $R0 1 -$R1
  StrCmp $R3 \ label_763
  Goto label_758
label_763:
  StrCpy $R0 $R0 -$R1
  Pop $R3
  Pop $R2
  Pop $R1
  Exch $R0
    ; Push $R0
    ; Exch
    ; Pop $R0
FunctionEnd


Function func_771
  Exch $_56_
    ; Push $_56_
    ; Exch
    ; Pop $_56_
  StrCmp $_56_ SHELL_CONTEXT 0 label_776
  Goto label_777
label_776:
  StrCmp $_56_ HKEY_CURRENT_USER 0 label_777
label_777:
  IfErrors 0 label_780
  DetailPrint "Uninstall was not successful. Not able to launch uninstaller!"
  Return

label_780:
  StrCmp $R0 0 label_785
  MessageBox MB_OK|MB_ICONEXCLAMATION "$(LSTR_57): $R0"    ;  "Failed to uninstall old application files. Please try running the installer again."
  DetailPrint "Uninstall was not successful. Uninstaller error code: $R0."
  SetErrorLevel 2
  Quit
label_785:
FunctionEnd


Function func_786
  ClearErrors
  Exch $_61_
    ; Push $_61_
    ; Exch
    ; Pop $_61_
  Push 0
  Pop $R0
  StrCmp $_61_ SHELL_CONTEXT 0 label_795
  ReadRegStr $_60_ SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 UninstallString
  Goto label_802
label_795:
  StrCmp $_61_ HKEY_CURRENT_USER 0 label_798
  ReadRegStr $_60_ HKCU Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 UninstallString
  Goto label_802
label_798:
  StrCmp $_61_ HKEY_LOCAL_MACHINE 0 label_801
  ReadRegStr $_60_ HKLM Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 UninstallString
  Goto label_802
label_801:
  MessageBox MB_OK "Unsupported $_61_"
label_802:
  StrCmp $_60_ "" 0 label_806
  StrCmp $_60_ "" 0 label_806
  ClearErrors
  Return

label_806:
  Push $_60_
  Call func_720
  Pop $_57_
  StrCmp $_61_ SHELL_CONTEXT 0 label_812
  ReadRegStr $_59_ SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 InstallLocation
  Goto label_819
label_812:
  StrCmp $_61_ HKEY_CURRENT_USER 0 label_815
  ReadRegStr $_59_ HKCU Software\2546e024-4dd1-5359-9177-3b6a84280dd8 InstallLocation
  Goto label_819
label_815:
  StrCmp $_61_ HKEY_LOCAL_MACHINE 0 label_818
  ReadRegStr $_59_ HKLM Software\2546e024-4dd1-5359-9177-3b6a84280dd8 InstallLocation
  Goto label_819
label_818:
  MessageBox MB_OK "Unsupported $_61_"
label_819:
  StrCmp $_59_ "" 0 label_824
  StrCmp $_57_ "" label_824
  Push $_57_
  Call func_750
  Pop $_59_
label_824:
  StrCmp $_59_ "" 0 label_828
  StrCmp $_57_ "" 0 label_828
  ClearErrors
  Return

label_828:
  StrCmp $_18_ CurrentUser 0 label_830
  Goto label_831
label_830:
  StrCmp $_61_ HKEY_CURRENT_USER 0 label_833
label_831:
  StrCpy $0 /currentuser
  Goto label_834
label_833:
  StrCpy $0 /allusers
label_834:
  StrCpy $_55_ true
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true label_843
  StrCpy $_55_ false
label_843:
  StrCmp $_55_ true 0 label_857
  StrCmp $_61_ SHELL_CONTEXT 0 label_847
  ReadRegStr $R5 SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 KeepShortcuts
  Goto label_854
label_847:
  StrCmp $_61_ HKEY_CURRENT_USER 0 label_850
  ReadRegStr $R5 HKCU Software\2546e024-4dd1-5359-9177-3b6a84280dd8 KeepShortcuts
  Goto label_854
label_850:
  StrCmp $_61_ HKEY_LOCAL_MACHINE 0 label_853
  ReadRegStr $R5 HKLM Software\2546e024-4dd1-5359-9177-3b6a84280dd8 KeepShortcuts
  Goto label_854
label_853:
  MessageBox MB_OK "Unsupported $_61_"
label_854:
  StrCmp $R5 true 0 label_857
  IfFileExists $_21_ 0 label_857
  StrCpy $0 "$0 --keep-shortcuts"
label_857:
  Push delete-app-data
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_866
  StrCpy $0 "$0 --delete-app-data"
  Goto label_867
label_866:
  StrCpy $0 "$0 --updated"
label_867:
  StrCpy $_58_ $PLUGINSDIR\old-uninstaller.exe
  Push $_58_
  StdUtils::GetParentPath /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD GetParentPath
  Pop $R5
  CreateDirectory $R5
  ClearErrors
  CopyFiles /SILENT $_57_ $_58_    ; $(LSTR_7)$_58_    ;  "Copy to "
  StrCpy $R5 0
label_878:
  IntOp $R5 $R5 + 1
  IntCmp $R5 5 label_882 label_882
  MessageBox MB_RETRYCANCEL|MB_ICONEXCLAMATION $(LSTR_58) /SD IDCANCEL IDRETRY label_882    ;  "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
  Return

label_882:
  ExecWait "$\"$_58_$\" /S /KEEP_APP_DATA $0 _?=$_59_" $R0
  IfErrors label_884 label_886
label_884:
  ExecWait "$\"$_57_$\" /S /KEEP_APP_DATA $0 _?=$_59_" $R0
  IfErrors label_890
label_886:
  StrCmp $R0 0 0 label_888
  Return

label_888:
  Sleep 1000
  Goto label_878
label_890:
  SetErrors
FunctionEnd


Section install ; Section_0
  ; AddSize 262268
  InitPluginsDir
    ; Call Initialize_____Plugins
    ; SetDetailsPrint lastused
  IfSilent label_896
  SetDetailsPrint none
label_896:
  StrCpy $_21_ "$INSTDIR\Shinri Hub.exe"
  ReadRegStr $_4_ SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 ShortcutName
  StrCmp $_4_ "" 0 label_900
  StrCpy $_4_ "Shinri Hub"
label_900:
  StrCpy $_3_ $DESKTOP\$_4_.lnk
  StrCpy $_2_ "$DESKTOP\Shinri Hub.lnk"
  ReadRegStr $_5_ SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 MenuDirectory
  StrCmp $_5_ "" 0 label_906
  StrCpy $_1_ $SMPROGRAMS\$_4_.lnk
  Goto label_907
label_906:
  StrCpy $_1_ $SMPROGRAMS\$_5_\$_4_.lnk
label_907:
  StrCpy $_0_ "$SMPROGRAMS\Shinri Hub.lnk"
  UAC::_ 3s
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\UAC.dll
    ; SetDetailsPrint lastused
    ; Push 3s
    ; CallInstDLL $PLUGINSDIR\UAC.dll _
  Pop $_52_
  StrCmp $_52_ 0 0 label_1063
  StrCpy $_62_ $SYSDIR\cmd.exe
  StrCpy $_63_ $SYSDIR\WindowsPowerShell\v1.0\powershell.exe
  nsExec::Exec "$\"$_63_$\" -C $\"if (Get-Command Get-CimInstance -ErrorAction SilentlyContinue) { exit 0 } else { exit 1 }$\""
    ; Call Initialize_____Plugins
    ; AllowSkipFiles on
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_63_$\" -C $\"if (Get-Command Get-CimInstance -ErrorAction SilentlyContinue) { exit 0 } else { exit 1 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $0
  StrCmp $0 0 0 label_930
  nsExec::Exec "$\"$_63_$\" -C $\"if ((Get-ExecutionPolicy -Scope Process) -eq 'Restricted') { exit 1 } else { exit 0 }$\""
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_63_$\" -C $\"if ((Get-ExecutionPolicy -Scope Process) -eq 'Restricted') { exit 1 } else { exit 0 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $0
label_930:
  StrCmp $0 0 label_932
  StrCpy $0 1
label_932:
  StrCpy $_64_ $0
  Push 0
  Call func_0
  Pop $3
  Pop $2
  Pop $1
  Pop $4
  Pop $_20_
  StrCmp $3 "Shinri Hub.exe" label_1063
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_949
  Sleep 300
label_949:
  StrCmp $_64_ 0 0 label_957
  nsExec::Exec "$\"$_63_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_63_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
  Goto label_963
label_957:
  nsExec::Exec "$\"$_62_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_62_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
label_963:
  StrCmp $R0 0 0 label_1063
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_973
  Sleep 1000
  Goto label_975
label_973:
  MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION $(LSTR_59) /SD IDOK IDOK label_975    ;  "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
  Quit
label_975:
  DetailPrint $(LSTR_60)    ;  "Closing running Shinri Hub..."
  Push $0
  StrCmp 0 1 0 label_983
  StrCmp $_64_ 0 0 label_981
  StrCpy $0 -Force
  Goto label_982
label_981:
  StrCpy $0 /F
label_982:
  Goto label_984
label_983:
  StrCpy $0 ""
label_984:
  StrCmp $_64_ 0 0 label_991
  nsExec::Exec "$\"$_63_$\" -C $\"Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')} | % { Stop-Process -Id $$_.ProcessId $0 }$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_63_$\" -C $\"Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')} | % { Stop-Process -Id $$_.ProcessId $0 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Goto label_996
label_991:
  nsExec::Exec "taskkill /IM $\"Shinri Hub.exe$\" /FI $\"PID ne $_20_$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "taskkill /IM $\"Shinri Hub.exe$\" /FI $\"PID ne $_20_$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
label_996:
  Pop $0
  Sleep 300
  StrCpy $R1 0
label_999:
  IntOp $R1 $R1 + 1
  StrCmp $_64_ 0 0 label_1008
  nsExec::Exec "$\"$_63_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_63_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
  Goto label_1014
label_1008:
  nsExec::Exec "$\"$_62_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_62_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
label_1014:
  StrCmp $R0 0 0 label_1057
  Sleep 1000
  Push $0
  StrCmp 1 1 0 label_1023
  StrCmp $_64_ 0 0 label_1021
  StrCpy $0 -Force
  Goto label_1022
label_1021:
  StrCpy $0 /F
label_1022:
  Goto label_1024
label_1023:
  StrCpy $0 ""
label_1024:
  StrCmp $_64_ 0 0 label_1031
  nsExec::Exec "$\"$_63_$\" -C $\"Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')} | % { Stop-Process -Id $$_.ProcessId $0 }$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_63_$\" -C $\"Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')} | % { Stop-Process -Id $$_.ProcessId $0 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Goto label_1036
label_1031:
  nsExec::Exec "taskkill /IM $\"Shinri Hub.exe$\" /FI $\"PID ne $_20_$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "taskkill /IM $\"Shinri Hub.exe$\" /FI $\"PID ne $_20_$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
label_1036:
  Pop $0
  StrCmp $_64_ 0 0 label_1045
  nsExec::Exec "$\"$_63_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_63_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
  Goto label_1051
label_1045:
  nsExec::Exec "$\"$_62_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_62_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
label_1051:
  StrCmp $R0 0 0 label_1055
  DetailPrint "Waiting for $\"Shinri Hub$\" to close."
  Sleep 2000
  Goto label_1056
label_1055:
  Goto label_1063
label_1056:
  Goto label_1058
label_1057:
  Goto label_1063
label_1058:
  IntCmp $R1 1 label_1062 label_1062
  MessageBox MB_RETRYCANCEL|MB_ICONEXCLAMATION $(LSTR_58) /SD IDCANCEL IDRETRY label_999    ;  "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
  Quit
  Goto label_1063
label_1062:
  Goto label_999
label_1063:
  StrCpy $_65_ false
  StrCpy $_55_ true
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true label_1073
  StrCpy $_55_ false
label_1073:
  StrCmp $_55_ true 0 label_1078
  ReadRegStr $R1 SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 KeepShortcuts
  StrCmp $R1 true 0 label_1078
  IfFileExists $_21_ 0 label_1078
  StrCpy $_65_ true
label_1078:
  Push SHELL_CONTEXT
  Call func_786
  Push SHELL_CONTEXT
  Call func_771
  StrCmp $_18_ all 0 label_1087
  Push HKEY_CURRENT_USER
  Call func_786
  Push HKEY_CURRENT_USER
  Call func_771
label_1087:
  SetOutPath $INSTDIR
  System::Call kernel32::GetCurrentProcess()p.s
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::GetCurrentProcess()p.s
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call kernel32::IsWow64Process2(ps,*i0s,*i)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::IsWow64Process2(ps,*i0s,*i)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Push |
  System::Call kernel32::IsWow64Process(p-1,*i0s)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::IsWow64Process(p-1,*i0s)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Int64Op
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\System.dll Int64Op
  Pop $_52_
  StrCmp $_52_ 0 label_1111
  Goto label_1130
label_1111:
  System::Call kernel32::GetCurrentProcess()p.s
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::GetCurrentProcess()p.s
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call kernel32::IsWow64Process2(ps,*i,*i0s)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push kernel32::IsWow64Process2(ps,*i,*i0s)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Pop $_52_
  IntCmp $_52_ 0 0 label_1129 label_1129
  System::Call *0x7FFE002E(&i2.s)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push *0x7FFE002E(&i2.s)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Pop $_52_
label_1129:
  IntCmp $_52_ 43620 0 label_1131 label_1131
label_1130:
  StrCpy $_66_ 64
label_1131:
  StrCmp $_66_ ARM64 0 label_1133
  Goto label_1136
label_1133:
  StrCmp $_66_ 64 0 label_1136
  SetOverwrite on
  AllowSkipFiles on
  File $PLUGINSDIR\app-64.7z
  Goto label_1136
label_1136:
  Push $OUTDIR
  CreateDirectory $PLUGINSDIR\7z-out
  ClearErrors
  SetOutPath $PLUGINSDIR\7z-out
  nsis7z::Extract $PLUGINSDIR\app-$_66_.7z
    ; Call Initialize_____Plugins
    ; SetOverwrite off
    ; File $PLUGINSDIR\nsis7z.dll
    ; SetDetailsPrint lastused
    ; Push $PLUGINSDIR\app-$_66_.7z
    ; CallInstDLL $PLUGINSDIR\nsis7z.dll Extract
  Pop $R0
  SetOutPath $R0
  StrCpy $R1 0
label_1148:
  IntOp $R1 $R1 + 1
  CopyFiles /SILENT $PLUGINSDIR\7z-out\* $OUTDIR    ; $(LSTR_7)$OUTDIR    ;  "Copy to "
  IfErrors 0 label_1166
  DetailPrint "Can't modify $\"Shinri Hub$\"'s files."
  IntCmp $R1 5 label_1155 0 label_1155
  Goto label_1164
  Goto label_1156
label_1155:
  MessageBox MB_RETRYCANCEL|MB_ICONEXCLAMATION $(LSTR_58) /SD IDRETRY IDCANCEL label_1163    ;  "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
label_1156:
  RMDir /r $PLUGINSDIR\7z-out
  nsis7z::Extract $PLUGINSDIR\app-$_66_.7z
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\nsis7z.dll
    ; SetDetailsPrint lastused
    ; Push $PLUGINSDIR\app-$_66_.7z
    ; CallInstDLL $PLUGINSDIR\nsis7z.dll Extract
  Goto label_1166
label_1163:
  Quit
label_1164:
  Sleep 1000
  Goto label_1148
label_1166:
  StrCmp $_66_ ARM64 0 label_1168
  Goto label_1170
label_1168:
  StrCmp $_66_ 64 0 label_1170
  Goto label_1170
label_1170:
  StrCmp $_18_ all 0 label_1172
  SetShellVarContext current
label_1172:
  Push $LOCALAPPDATA\st-hub-updater\installer.exe
  StdUtils::GetParentPath /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD GetParentPath
  Pop $R5
  CreateDirectory $R5
  ClearErrors
  CopyFiles /SILENT $EXEPATH $LOCALAPPDATA\st-hub-updater\installer.exe    ; $(LSTR_7)$LOCALAPPDATA\st-hub-updater\installer.exe    ;  "Copy to "
  StrCmp $_18_ all 0 label_1183
  SetShellVarContext all
label_1183:
  SetOverwrite on
  AllowSkipFiles on
  File "Uninstall Shinri Hub.exe"
  WriteRegStr SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 InstallLocation $INSTDIR
  WriteRegStr SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 KeepShortcuts true
  WriteRegStr SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 ShortcutName "Shinri Hub"
  StrCmp $_18_ all 0 label_1191
  StrCpy $0 /allusers
  StrCpy $1 ""
  Goto label_1193
label_1191:
  StrCpy $0 /currentuser
  StrCpy $1 ""
label_1193:
  WriteRegStr SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 DisplayName "Shinri Hub 26.11.23196$1"
  StrCpy $2 "$INSTDIR\Uninstall Shinri Hub.exe"
  WriteRegStr SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 UninstallString "$\"$2$\" $0"
  WriteRegStr SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 QuietUninstallString "$\"$2$\" $0 /S"
  WriteRegStr SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 DisplayVersion 26.11.23196
  WriteRegStr SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 DisplayIcon $_21_,0
  WriteRegStr SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 Comments "Shinri Hub"
  WriteRegDWORD SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 NoModify 1
  WriteRegDWORD SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 NoRepair 1
  IntFmt $0 0x%08X 539984
  WriteRegDWORD SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8 EstimatedSize $0
  StrCmp $_65_ false 0 label_1216
  StrCmp $_5_ "" label_1207
  RMDir $SMPROGRAMS\$_5_
label_1207:
  CreateShortCut $_0_ $_21_ "" $_21_ "" "" "" "Shinri Hub"
  ClearErrors
  WinShell::SetLnkAUMI $_0_ stred.shinritrial.hub
    ; Call Initialize_____Plugins
    ; SetOverwrite off
    ; File $PLUGINSDIR\WinShell.dll
    ; SetDetailsPrint lastused
    ; Push stred.shinritrial.hub
    ; Push $_0_
    ; CallInstDLL $PLUGINSDIR\WinShell.dll SetLnkAUMI
  Goto label_1232
label_1216:
  StrCmp $_1_ $_0_ label_1232
  IfFileExists $_1_ 0 label_1232
  Rename $_1_ $_0_ ;  $_1_->$_0_
  WinShell::UninstShortcut $_1_
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\WinShell.dll
    ; SetDetailsPrint lastused
    ; Push $_1_
    ; CallInstDLL $PLUGINSDIR\WinShell.dll UninstShortcut
  WinShell::SetLnkAUMI $_0_ stred.shinritrial.hub
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\WinShell.dll
    ; SetDetailsPrint lastused
    ; Push stred.shinritrial.hub
    ; Push $_0_
    ; CallInstDLL $PLUGINSDIR\WinShell.dll SetLnkAUMI
  StrCmp $_5_ "" label_1232
  RMDir $SMPROGRAMS\$_5_
label_1232:
  Push no-desktop-shortcut
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true label_1268
  StrCmp $_65_ false 0 label_1249
  CreateShortCut $_2_ $_21_ "" $_21_ "" "" "" "Shinri Hub"
  ClearErrors
  WinShell::SetLnkAUMI $_2_ stred.shinritrial.hub
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\WinShell.dll
    ; SetDetailsPrint lastused
    ; Push stred.shinritrial.hub
    ; Push $_2_
    ; CallInstDLL $PLUGINSDIR\WinShell.dll SetLnkAUMI
  Goto label_1263
label_1249:
  StrCmp $_3_ $_2_ label_1263
  IfFileExists $_3_ 0 label_1263
  Rename $_3_ $_2_ ;  $_3_->$_2_
  WinShell::UninstShortcut $_3_
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\WinShell.dll
    ; SetDetailsPrint lastused
    ; Push $_3_
    ; CallInstDLL $PLUGINSDIR\WinShell.dll UninstShortcut
  WinShell::SetLnkAUMI $_2_ stred.shinritrial.hub
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\WinShell.dll
    ; SetDetailsPrint lastused
    ; Push stred.shinritrial.hub
    ; Push $_2_
    ; CallInstDLL $PLUGINSDIR\WinShell.dll SetLnkAUMI
label_1263:
  System::Call "Shell32::SHChangeNotify(i 0x8000000, i 0, i 0, i 0)"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "Shell32::SHChangeNotify(i 0x8000000, i 0, i 0, i 0)"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_1268:
  IfFileExists $_0_ 0 label_1271
  StrCpy $_22_ $_0_
  Goto label_1272
label_1271:
  StrCpy $_22_ "$INSTDIR\Shinri Hub.exe"
label_1272:
  Push force-run
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_1299
  IfSilent 0 label_1299
  HideWindow
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_1290
  StrCpy $_67_ --updated
  Goto label_1291
label_1290:
  StrCpy $_67_ ""
label_1291:
  Push $_22_
  Push open
  Push $_67_
  StdUtils::ExecShellAsUser /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD ExecShellAsUser
  Pop $0
label_1299:
SectionEnd


Function func_1300
  SectionSetSize 0 539985
FunctionEnd


/*
Function Initialize_____Plugins
  SetDetailsPrint none
  StrCmp $PLUGINSDIR "" 0 label_1312
  Push $0
  SetErrors
  GetTempFileName $0
  Delete $0
  CreateDirectory $0 ; CreateRestrictedDirectory
  IfErrors label_1313
  StrCpy $PLUGINSDIR $0
  Pop $0
label_1312:
  Return

label_1313:
  MessageBox MB_OK|MB_ICONSTOP "Error! Can't initialize plug-ins directory. Please try again later." /SD IDOK
  Quit
FunctionEnd
*/



; --------------------
; UNREFERENCED STRINGS:

/*
17 CommonFilesDir
52 "$PROGRAMFILES\Common Files"
68 $COMMONFILES
*/
