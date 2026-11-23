; NSIS script (UTF-8) NSIS-3 Unicode
; Uninstall

Unicode true
SetCompressor zlib

; --------------------
; HEADER SIZE: 145974
; START HEADER SIZE: 300
; MAX STRING LENGTH: 8192
; STRING CHARS: 44561

OutFile [NSIS].exe
!include WinMessages.nsh

ShowUninstDetails nevershow


; --------------------
; LANG TABLES: 26
; LANG STRINGS: 71

Name "Shinri Hub"
BrandingText "Shinri Hub 26.11.23196"

; LANG: 1066
LangString LSTR_0 1066 "Shinri Hub 26.11.23196"
LangString LSTR_1 1066 "Gỡ bỏ $(LSTR_2)"
LangString LSTR_2 1066 "Shinri Hub"
LangString LSTR_5 1066 "Không thể ghi: "
LangString LSTR_8 1066 "Không thể tìm thấy biểu tượng: "
LangString LSTR_9 1066 "Không thể nạp: "
LangString LSTR_10 1066 "Tạo thư mục: "
LangString LSTR_12 1066 "Tạo trình gỡ bỏ: "
LangString LSTR_13 1066 "Xóa tập tin: "
LangString LSTR_14 1066 "Xóa khi khởi động lại: "
LangString LSTR_16 1066 "Lỗi khi tạo: "
LangString LSTR_17 1066 "Lỗi khi giải nén dữ liệu! Có thể trình cài đặt đã bị hỏng?"
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
LangString LSTR_36 1066 "Lỗi khi mở tập tin để ghi: $\r$\n$\r$\n$0$\r$\n$\r$\nNhấn “Hủy bỏ” để ngừng việc cài đặt,$\r$\n“Thử lại” để thử lần nữa, hoặc$\r$\n“Bỏ qua” để bỏ qua tập tin này."
LangString LSTR_37 1066 0
LangString LSTR_38 1066 "Chào mừng đến với Trợ lí Gỡ bỏ $(LSTR_70)"
LangString LSTR_39 1066 "MS Shell Dlg"
LangString LSTR_40 1066 "Trình trợ lí sẽ hướng dẫn bạn việc gỡ bỏ $(LSTR_70).$\r$\n$\r$\nTrước khi bắt đầu gỡ bỏ, hãy chắc chắn rằng $(LSTR_70) đang không chạy.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1066 "Gỡ bỏ"
LangString LSTR_42 1066 "Vui lòng đợi trong khi $(LSTR_70) đang được gỡ bỏ."
LangString LSTR_43 1066 "Gỡ bỏ hoàn tất"
LangString LSTR_44 1066 "Việc gỡ bỏ đã hoàn tất thành công."
LangString LSTR_45 1066 "Gỡ bỏ bị hủy"
LangString LSTR_46 1066 "Việc gỡ bỏ không hoàn tất thành công."
LangString LSTR_47 1066 "&Hoàn thành"
LangString LSTR_48 1066 "Hoàn tất Gỡ bỏ $(LSTR_70)"
LangString LSTR_49 1066 "Máy tính của bạn phải được khởi động lại để hoàn tất việc gỡ bỏ $(LSTR_70). Bạn có muốn khởi động lại ngay không?"
LangString LSTR_50 1066 "Khởi động lại ngay"
LangString LSTR_51 1066 "Tôi muốn khởi động lại sau"
LangString LSTR_52 1066 "$(LSTR_70) đã được gỡ bỏ khỏi máy tính của bạn.$\r$\n$\r$\nNhấn “Hoàn thành” để đóng Trình trợ lí."
LangString LSTR_53 1066 8
LangString LSTR_54 1066 "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
LangString LSTR_55 1066 "Closing running Shinri Hub..."
LangString LSTR_56 1066 "Shinri Hub không thể đóng $\r$\nVui lòng đóng nó theo cách thủ công và nhấp vào Thử lại để tiếp tục."
LangString LSTR_57 1066 "Windows 7 and above is required"
LangString LSTR_58 1066 "64-bit Windows is required"
LangString LSTR_59 1066 "Tùy biến"
LangString LSTR_60 1066 "Hủy bỏ"
LangString LSTR_61 1066 "< &Lùi"
LangString LSTR_62 1066 "&Gỡ bỏ"
LangString LSTR_63 1066 "Nhấn “Gỡ bỏ” để bắt đầu việc gỡ bỏ."
LangString LSTR_64 1066 "&Hiện chi tiết"
LangString LSTR_65 1066 "Hoàn thành"
LangString LSTR_66 1066 "&Tiến >"
LangString LSTR_67 1066 "Nhấn Tiến để tiếp tục."
LangString LSTR_68 1066 " "
LangString LSTR_69 1066 Đón&g
LangString LSTR_70 1066 "Shinri Hub"


; LANG: 1054
LangString LSTR_0 1054 "Shinri Hub 26.11.23196"
LangString LSTR_1 1054 "$(LSTR_2) ยกเลิกการติดตั้ง"
LangString LSTR_2 1054 "Shinri Hub"
LangString LSTR_5 1054 "ไม่สามารถเขียน: "
LangString LSTR_8 1054 "ไม่สามารถหาสัญลักษณ์ได้: "
LangString LSTR_9 1054 "ไม่สามารถโหลดได้: "
LangString LSTR_10 1054 "สร้างแฟ้ม: "
LangString LSTR_12 1054 "สร้างตัวยกเลิกการติดตั้ง: "
LangString LSTR_13 1054 "ลบไฟล์: "
LangString LSTR_14 1054 "ลบตอนรีบูท: "
LangString LSTR_16 1054 "มีปัญหาในการสร้างไฟล์: "
LangString LSTR_17 1054 "มีปัญหาในการคลายข้อมูล! เกิดข้อผิดพลาดจากตัวติดตั้ง?"
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
LangString LSTR_36 1054 "ไม่สามารถเปิดไฟล์สำหรับเขียนได้: $\r$\n$\r$\n$0$\r$\n$\r$\nกด ยกเลิก เพื่อหยุดการติดตั้ง,$\r$\nลองอีกครั้ง เพื่อลองอีกครั้ง, หรือ$\r$\nเพิกเฉย เพื่อข้ามไฟล์นี้."
LangString LSTR_37 1054 0
LangString LSTR_38 1054 "ยินดีต้อนรับสู่การยกเลิกการติดตั้งอัติโนมัติของ $(LSTR_70)"
LangString LSTR_39 1054 "MS Shell Dlg"
LangString LSTR_40 1054 "ตัวติดตั้งอัติโนมัตินี้จะนำคุณไปสู่การยกเลิกการติดตั้งของ $(LSTR_70).$\r$\n$\r$\nการจะเริ่มการยกเลิกการติดตั้งนี้, โปรดตรวจสอบว่า $(LSTR_70) ไม่ได้ใช้อยู่$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1054 กำลังยกเลิกการติดตั้ง
LangString LSTR_42 1054 "โปรดรอในขณะที่ $(LSTR_70) กำลังถูกยกเลิกการติดตั้ง."
LangString LSTR_43 1054 การยกเลิกการติดตั้งเสร็จสิ้น
LangString LSTR_44 1054 การยกเลิกการติดตั้งเสร็จสิ้นโดยสมบูรณ์
LangString LSTR_45 1054 การยกเลิกการติดตั้งถูกยกเลิก
LangString LSTR_46 1054 การยกเลิกการติดตั้งไม่สำเร็จ
LangString LSTR_47 1054 &เสร็จสิ้น
LangString LSTR_48 1054 "การยกเลิกการติดตั้งอัติโนมัติของ $(LSTR_70) กำลังเสร็จสมบูรณ์"
LangString LSTR_49 1054 "เครื่องคอมพิวเตอร์ของคุณจำเป็นต้องรีสตาร์ทในการที่จะทำการยกเลิกการติดตั้งของ $(LSTR_70) เสร็จสิ้น, คุณต้องการจะรีบูทเดี๋ยวนี้ไหม?"
LangString LSTR_50 1054 "รีบูท เดี๋ยวนี้"
LangString LSTR_51 1054 "ฉันต้องการ รีบูทด้วยตนเอง ทีหลัง"
LangString LSTR_52 1054 "$(LSTR_70) ได้ถูกยกเลิกออกจากเครื่องคอมพิวเตอร์ของคุณแล้ว $\r$\n$\r$\nกด เสร็จสิ้น เพื่อปิดหน้าจอติดตั้งอัติโนมัติ"
LangString LSTR_53 1054 8
LangString LSTR_54 1054 "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
LangString LSTR_55 1054 "Closing running Shinri Hub..."
LangString LSTR_56 1054 "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
LangString LSTR_57 1054 "Windows 7 and above is required"
LangString LSTR_58 1054 "64-bit Windows is required"
LangString LSTR_59 1054 กำหนดเอง
LangString LSTR_60 1054 ยกเลิก
LangString LSTR_61 1054 "< &กลับไป"
LangString LSTR_62 1054 &ยกเลิกการติดตั้ง
LangString LSTR_63 1054 "กด  ยกเลิกการติดตั้ง เพื่อยกเลิกการติดตั้ง"
LangString LSTR_64 1054 &รายละเอียด
LangString LSTR_65 1054 เสร็จสิ้นแล้ว
LangString LSTR_66 1054 "&ต่อไป >"
LangString LSTR_67 1054 "กด ต่อไป เพื่อเริ่มระบบอัติโนมัติ"
LangString LSTR_68 1054 " "
LangString LSTR_69 1054 &ปิด
LangString LSTR_70 1054 "Shinri Hub"


; LANG: 1025
LangString LSTR_0 1025 "Shinri Hub 26.11.23196"
LangString LSTR_1 1025 "إزالة $(LSTR_2)"
LangString LSTR_2 1025 "Shinri Hub"
LangString LSTR_5 1025 "لا يستطيع الكتابة: "
LangString LSTR_8 1025 "لم يتمكن من إيجاد الرمز :"
LangString LSTR_9 1025 "لم يتمكن من تحميل :"
LangString LSTR_10 1025 "إنشاء مجلد"
LangString LSTR_12 1025 "إنشاء مزيل: "
LangString LSTR_13 1025 "حذف ملف: "
LangString LSTR_14 1025 "حذف بعد إعادة التشغيل: "
LangString LSTR_16 1025 "خلل أثناء إنشاء :"
LangString LSTR_17 1025 "خلل أثناء فتح البيانات المضغوطة! منصب تالف؟"
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
LangString LSTR_36 1025 "حدث خلل أثناء فتح ملف للكتابة: $\r$\n$\t\$\"$0\$\"$\r$\nإضغط إلغاء لإلغاء التنصيب،$\r$\nمحاولة لإعادة محاولة كتابة الملف،$\r$\n تجاهل لتخطي الملف"
LangString LSTR_37 1025 1
LangString LSTR_38 1025 "مرحبا بك في مرشد إزالة $(LSTR_70) "
LangString LSTR_39 1025 "MS Shell Dlg"
LangString LSTR_40 1025 "هذا المرشد سيدلّك أثناء إزالة $(LSTR_70).$\r$\n$\r$\n قبل البدء بالإزالة، يرجى التأكد من أن $(LSTR_70) غير شغّال.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1025 إزالة
LangString LSTR_42 1025 "الرجاء الإنتظار أثناء إزالة $(LSTR_70)."
LangString LSTR_43 1025 إنتهى
LangString LSTR_44 1025 "إنتهت عملية الإزالة بنجاح."
LangString LSTR_45 1025 "إلغاء الإزالة"
LangString LSTR_46 1025 "لم تنتهي الإزالة بنجاح."
LangString LSTR_47 1025 &إنهاء
LangString LSTR_48 1025 "إنهاء مرشد إزالة $(LSTR_70)"
LangString LSTR_49 1025 "يجب إعادة تشغيل الجهاز لإنهاء إزالة $(LSTR_70). هل تريد إعادة التشغيل الآن؟"
LangString LSTR_50 1025 "أعد التشغيل الآن"
LangString LSTR_51 1025 "أرغب في إعادة تشغيل الجهاز في وقت لاحق"
LangString LSTR_52 1025 "لقد تم إزالة $(LSTR_70) من الجهاز.$\r$\n$\r$\n إضغط إنهاء لإغلاق المرشد."
LangString LSTR_53 1025 8
LangString LSTR_54 1025 "‫إن Shinri Hub يعمل حاليا.$\r$\n‫يُرجى الضغط على « موافق » لغلقه.$\r$\n‫إذا لم يُغلَق، يُرجى محاولة إغلاقه يدويا."
LangString LSTR_55 1025 "Closing running Shinri Hub..."
LangString LSTR_56 1025 "‫لم يتمكن من إغلاق Shinri Hub $\r$\nيُرجى إغلاقه يدويا ثم الضغط مرة أخرى على إعادة المحاولة للاستمرار.‏"
LangString LSTR_57 1025 "Windows 7 and above is required"
LangString LSTR_58 1025 "64-bit Windows is required"
LangString LSTR_59 1025 إختياري
LangString LSTR_60 1025 إلغاء
LangString LSTR_61 1025 "< ال&سابق"
LangString LSTR_62 1025 &إزالة
LangString LSTR_63 1025 "إضغط على إزالة لتشغيل الإزالة."
LangString LSTR_64 1025 "إ&ظهار التفاصيل"
LangString LSTR_65 1025 إنتهى
LangString LSTR_66 1025 "ال&تالي >"
LangString LSTR_67 1025 "إضغط على التالي للمتابعة."
LangString LSTR_68 1025 " "
LangString LSTR_69 1025 إ&غلاق
LangString LSTR_70 1025 "Shinri Hub"


; LANG: 1058
LangString LSTR_0 1058 "Shinri Hub 26.11.23196"
LangString LSTR_1 1058 "$(LSTR_2) Видалення"
LangString LSTR_2 1058 "Shinri Hub"
LangString LSTR_5 1058 "Неможливо записати: "
LangString LSTR_8 1058 "Неможливо знайти символ: "
LangString LSTR_9 1058 "Неможливо завантажити: "
LangString LSTR_10 1058 "Створення теки: "
LangString LSTR_12 1058 "Створення програми видалення: "
LangString LSTR_13 1058 "Видалення файлу: "
LangString LSTR_14 1058 "Видалення після перезавантаження: "
LangString LSTR_16 1058 "Помилка створення: "
LangString LSTR_17 1058 "Неможливо витягнути дані. Можливо пошкоджений дистрибутив."
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
LangString LSTR_36 1058 "Неможливо відкрити файл для запису: $\r$\n$\t$\"$0$\"$\r$\nНатисніть Перервати для того, щоб перервати встановлення,$\r$\nПовторити для того, щоб повторити спробу запису у файл, чи$\r$\nІгнорувати для того, щоб пропустити цю дію"
LangString LSTR_37 1058 0
LangString LSTR_38 1058 "Ласкаво просимо до Майстра Видалення $(LSTR_70)"
LangString LSTR_39 1058 "MS Shell Dlg"
LangString LSTR_40 1058 "Даний майстер допоможе видалити $(LSTR_70).$\r$\n$\r$\nПеред початком видалення, перевірте, чи не запущено $(LSTR_70).$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1058 Видалення
LangString LSTR_42 1058 "Зачекайте, йде видалення файлів $(LSTR_70)."
LangString LSTR_43 1058 "Видалення завершено"
LangString LSTR_44 1058 "Видалення програми було успішно завершено."
LangString LSTR_45 1058 "Видалення перервано"
LangString LSTR_46 1058 "Видалення не було виконано повністю."
LangString LSTR_47 1058 &Кінець
LangString LSTR_48 1058 "Завершення Майстра Видалення $(LSTR_70)"
LangString LSTR_49 1058 "Для того, щоб закінчити видалення $(LSTR_70) ваш комп'ютер повинен перезавантажитися. Ви бажаєте зробити це зараз?"
LangString LSTR_50 1058 "Перезавантажитися зараз"
LangString LSTR_51 1058 "Я хочу перезавантажитися власноруч пізніше"
LangString LSTR_52 1058 "$(LSTR_70) видалений з вашого комп'ютера.$\r$\n$\r$\nНатисніть Вихід, щоб закрити Майстра."
LangString LSTR_53 1058 8
LangString LSTR_54 1058 "Shinri Hub запущено.$\r$\nКлацніть ОК для завершення.$\r$\nЯкщо завершення не вдалось — спробуйте вручну."
LangString LSTR_55 1058 "Closing running Shinri Hub..."
LangString LSTR_56 1058 "Неможливо закрити Shinri Hub. $\r$\nЗакрийте його вручну та натисніть Повторити, щоб продовжити."
LangString LSTR_57 1058 "Windows 7 and above is required"
LangString LSTR_58 1058 "64-bit Windows is required"
LangString LSTR_59 1058 "По вибору"
LangString LSTR_60 1058 Скасувати
LangString LSTR_61 1058 "< &Назад"
LangString LSTR_62 1058 Видалити
LangString LSTR_63 1058 "Натисніть Видалить для початку процессу видалення"
LangString LSTR_64 1058 Подробиці
LangString LSTR_65 1058 Завершено
LangString LSTR_66 1058 "&Далі >"
LangString LSTR_67 1058 "Натисніть Далі щоб продовжити"
LangString LSTR_68 1058 " "
LangString LSTR_69 1058 &Закрити
LangString LSTR_70 1058 "Shinri Hub"


; LANG: 2070
LangString LSTR_0 2070 "Shinri Hub 26.11.23196"
LangString LSTR_1 2070 "Desinstalação de $(LSTR_2)"
LangString LSTR_2 2070 "Shinri Hub"
LangString LSTR_5 2070 "Não foi possível escrever: "
LangString LSTR_8 2070 "Símbolo não encontrado: "
LangString LSTR_9 2070 "Não foi possível carregar: "
LangString LSTR_10 2070 "Criando diretório: "
LangString LSTR_12 2070 "Criando desinstalador: "
LangString LSTR_13 2070 "Apagando ficheiro: "
LangString LSTR_14 2070 "Apagar ao reiniciar: "
LangString LSTR_16 2070 "Erro ao criar: "
LangString LSTR_17 2070 "Erro ao descomprimir dados! Instalador corrompido?"
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
LangString LSTR_36 2070 "Erro ao abrir ficheiro para escrita: $\r$\n$\t$\"$0$\"$\r$\nClique em Abortar para abortar a instalação,$\r$\nRepetir para tentar novamente a escrita do ficheiro, ou$\r$\nIgnorar para ignorar este ficheiro."
LangString LSTR_37 2070 0
LangString LSTR_38 2070 "Bem vindo ao Assistente de desinstalação do $(LSTR_70)"
LangString LSTR_39 2070 "MS Shell Dlg"
LangString LSTR_40 2070 "Este assistente ajudá-lo-á durante a desinstalação do $(LSTR_70).$\r$\n$\r$\nAntes de iniciar a desinstalação, certifique-se de que o $(LSTR_70) não está em execução.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 2070 Desinstalando
LangString LSTR_42 2070 "Por favor, aguarde enquanto o $(LSTR_70) está sendo desinstalado."
LangString LSTR_43 2070 "Desinstalação Completa"
LangString LSTR_44 2070 "A desinstalação foi concluída com sucesso."
LangString LSTR_45 2070 "Desinstalação Abortada"
LangString LSTR_46 2070 "A desinstalação não foi concluída com sucesso"
LangString LSTR_47 2070 &Terminar
LangString LSTR_48 2070 "Concluíndo o assistente de desisntalação do $(LSTR_70)"
LangString LSTR_49 2070 "O seu computador deve ser reiniciado para concluír a desinstalação do $(LSTR_70). Deseja reiniciar agora?"
LangString LSTR_50 2070 "Reiniciar Agora"
LangString LSTR_51 2070 "Eu quero reiniciar manualmente depois"
LangString LSTR_52 2070 "$(LSTR_70) foi removido do seu computador.$\r$\n$\r$\nClique em Terminar para fechar este assistente."
LangString LSTR_53 2070 8
LangString LSTR_54 2070 "Shinri Hub está a correr.$\r$\nClique em OK para o encerrar.$\r$\nSe não encerrar, tente encerrá-lo manualmente."
LangString LSTR_55 2070 "Closing running Shinri Hub..."
LangString LSTR_56 2070 "Não é possível fechar o Shinri Hub. $\r$\nPor favor, feche-o manualmente e clique em ´Tentar novamente' para continuar."
LangString LSTR_57 2070 "Windows 7 and above is required"
LangString LSTR_58 2070 "64-bit Windows is required"
LangString LSTR_59 2070 Personalizado
LangString LSTR_60 2070 Cancelar
LangString LSTR_61 2070 "< &Anterior"
LangString LSTR_62 2070 &Desinstalar
LangString LSTR_63 2070 "Clique em 'Desinstalar' para iniciar a desinstalação."
LangString LSTR_64 2070 "Ver &Detalhes"
LangString LSTR_65 2070 Concluído
LangString LSTR_66 2070 "&Seguinte >"
LangString LSTR_67 2070 "Clique em 'Seguinte' para continuar."
LangString LSTR_68 2070 " "
LangString LSTR_69 2070 &Fechar
LangString LSTR_70 2070 "Shinri Hub"


; LANG: 1044
LangString LSTR_0 1044 "Shinri Hub 26.11.23196"
LangString LSTR_1 1044 "$(LSTR_2) avinstallasjon"
LangString LSTR_2 1044 "Shinri Hub"
LangString LSTR_5 1044 "Kan ikke skrive: "
LangString LSTR_8 1044 "Kunne ikke finne symbol: "
LangString LSTR_9 1044 "Kunne ikke laste: "
LangString LSTR_10 1044 "Lag mappe: "
LangString LSTR_12 1044 "Avinstallasjon laget: "
LangString LSTR_13 1044 "Slett fil: "
LangString LSTR_14 1044 "Slett ved omstart: "
LangString LSTR_16 1044 "Feil under opprettelse av: "
LangString LSTR_17 1044 "Feil under utpakking av data! Installasjonsprogrammet kan være skadet."
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
LangString LSTR_36 1044 "Feil under åpning av fil for skriving: $\r$\n$\t\$\"$0\$\"$\r$\nTrykk Avbryt for å avbryte installasjonen,$\r$\nPrøv igjen for å prøve igjen, eller$\r$\nIgnorer for å hoppe over denne filen"
LangString LSTR_37 1044 0
LangString LSTR_38 1044 "Velkommen til veiviseren for avinstallasjon av $(LSTR_70)"
LangString LSTR_39 1044 "MS Shell Dlg"
LangString LSTR_40 1044 "Denne veiviseren vil lede deg gjennom avinstallasjonen av $(LSTR_70).$\r$\n$\r$\nFør du fortsetter må du forsikre deg om at $(LSTR_70) ikke kjører.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1044 "Avinstallasjon pågår"
LangString LSTR_42 1044 "Vennligst vent mens $(LSTR_70) blir avinstallert."
LangString LSTR_43 1044 "Avinstallasjon ferdig"
LangString LSTR_44 1044 "Avinstallasjonen ble utført uten feil."
LangString LSTR_45 1044 "Avinstallasjon avbrutt"
LangString LSTR_46 1044 "Avinstallasjonen ble ikke utført riktig."
LangString LSTR_47 1044 &Ferdig
LangString LSTR_48 1044 "Fullfører avinstallasjonen av $(LSTR_70)"
LangString LSTR_49 1044 "Datamaskinen må starte på nytt for å fullføre avinstallasjonen av $(LSTR_70). Vil du starte datamaskinen på nytt nå?"
LangString LSTR_50 1044 "Ja. Start datamaskinen på nytt nå"
LangString LSTR_51 1044 "Nei. Jeg vil starte datamaskinen på nytt senere"
LangString LSTR_52 1044 "$(LSTR_70) har blitt avinstallert fra din datamaskin.$\r$\n$\r$\nTrykk på ferdig for å avslutte denne veiviseren."
LangString LSTR_53 1044 8
LangString LSTR_54 1044 "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
LangString LSTR_55 1044 "Closing running Shinri Hub..."
LangString LSTR_56 1044 "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
LangString LSTR_57 1044 "Windows 7 and above is required"
LangString LSTR_58 1044 "64-bit Windows is required"
LangString LSTR_59 1044 Egendefinert
LangString LSTR_60 1044 Avbryt
LangString LSTR_61 1044 "< &Tilbake"
LangString LSTR_62 1044 &Avinstaller
LangString LSTR_63 1044 "Trykk Avinstaller for å starte avinstallasjonen."
LangString LSTR_64 1044 "Vis &detaljer"
LangString LSTR_65 1044 Ferdig
LangString LSTR_66 1044 "&Neste >"
LangString LSTR_67 1044 "Trykk Neste for å fortsette."
LangString LSTR_68 1044 " "
LangString LSTR_69 1044 &Lukk
LangString LSTR_70 1044 "Shinri Hub"


; LANG: 1030
LangString LSTR_0 1030 "Shinri Hub 26.11.23196"
LangString LSTR_1 1030 "$(LSTR_2) Afinstallation"
LangString LSTR_2 1030 "Shinri Hub"
LangString LSTR_5 1030 "Kan ikke skrive: "
LangString LSTR_8 1030 "Kunne ikke finde symbol: "
LangString LSTR_9 1030 "Kunne ikke indlæse: "
LangString LSTR_10 1030 "Opret mappe: "
LangString LSTR_12 1030 "Afinstallationsguide oprettet: "
LangString LSTR_13 1030 "Slet fil: "
LangString LSTR_14 1030 "Slet ved genstart: "
LangString LSTR_16 1030 "Fejl ved oprettelse: "
LangString LSTR_17 1030 "Fejl ved udpakning af data! Beskadiget installationsguide?"
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
LangString LSTR_36 1030 "Fejl ved skrivning af fil til skrivning: $\r$\n$\r$\n$0$\r$\n$\r$\nTryk på Annuller for at stoppe installationen,$\r$\nPrøv igen for at prøve igen, eller$\r$\nIgnorer for at springe over denne fil."
LangString LSTR_37 1030 0
LangString LSTR_38 1030 "Velkommen til $(LSTR_70) afinstallationsguiden"
LangString LSTR_39 1030 "MS Shell Dlg"
LangString LSTR_40 1030 "Denne guide hjælper dig gennem afinstallationen af $(LSTR_70).$\r$\n$\r$\nFør start af afinstallationen skal du være sikker på at $(LSTR_70) ikke kører.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1030 Afinstallerer
LangString LSTR_42 1030 "Vent venligst mens $(LSTR_70) bliver afinstalleret."
LangString LSTR_43 1030 "Afinstallation gennemført"
LangString LSTR_44 1030 "Afinstallationen blev gennemført."
LangString LSTR_45 1030 "Afinstallation afbrudt"
LangString LSTR_46 1030 "Afinstallationen blev ikke gennemført."
LangString LSTR_47 1030 &Afslut
LangString LSTR_48 1030 "Gennemfører $(LSTR_70) afinstallationsguiden"
LangString LSTR_49 1030 "Din computer skal genstartes for at gennemføre afinstallationen af $(LSTR_70). Vil du genstarte nu?"
LangString LSTR_50 1030 "Genstart nu"
LangString LSTR_51 1030 "Jeg genstarter selv på et senere tidspunkt"
LangString LSTR_52 1030 "$(LSTR_70) er blevet afinstalleret fra din computer.$\r$\n$\r$\nTryk på Afslut for at lukke installationsguiden."
LangString LSTR_53 1030 8
LangString LSTR_54 1030 "Shinri Hub kører.$\r$\nKlik OK for at lukke den.$\r$\nHvis den ikke lukker, kan du prøve at lukke den manuelt."
LangString LSTR_55 1030 "Closing running Shinri Hub..."
LangString LSTR_56 1030 "Shinri Hub kan ikke afsluttes $\r$\nLuk den manuelt og klik Prøv igen for at fortsætte"
LangString LSTR_57 1030 "Windows 7 og højere er nødvendig"
LangString LSTR_58 1030 "64-bit Windows er påkrævet"
LangString LSTR_59 1030 Brugerdefineret
LangString LSTR_60 1030 Annuller
LangString LSTR_61 1030 "< &Tilbage"
LangString LSTR_62 1030 &Afinstaller
LangString LSTR_63 1030 "Tryk på Afinstaller for at starte afinstallationen."
LangString LSTR_64 1030 "Vis &detaljer"
LangString LSTR_65 1030 Gennemført
LangString LSTR_66 1030 "&Næste >"
LangString LSTR_67 1030 "Tryk på Næste for at fortsætte."
LangString LSTR_68 1030 " "
LangString LSTR_69 1030 &Luk
LangString LSTR_70 1030 "Shinri Hub"


; LANG: 3082
LangString LSTR_0 3082 "Shinri Hub 26.11.23196"
LangString LSTR_1 3082 "Desinstalación de $(LSTR_2)"
LangString LSTR_2 3082 "Shinri Hub"
LangString LSTR_5 3082 "No pudo escribirse: "
LangString LSTR_8 3082 "No se encontró simbolo: "
LangString LSTR_9 3082 "No pudo cargarse: "
LangString LSTR_10 3082 "Creando carpeta: "
LangString LSTR_12 3082 "Creando desinstalador: "
LangString LSTR_13 3082 "Borrar archivo: "
LangString LSTR_14 3082 "Borrar al reinicio: "
LangString LSTR_16 3082 "Error creando: "
LangString LSTR_17 3082 "¡Error descomprimiendo datos! ¿Instalador corrupto?"
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
LangString LSTR_36 3082 "Error abriendo archivo para escribir: $\r$\n$\r$\n$0$\r$\n$\r$\nPresione Abortar para detener la instalación,$\r$\nReintentar para probar otra vez, o$\r$\nOmitir para ignorar este archivo."
LangString LSTR_37 3082 0
LangString LSTR_38 3082 "Bienvenido al Asistente de Desinstalación de $(LSTR_70)"
LangString LSTR_39 3082 "MS Shell Dlg"
LangString LSTR_40 3082 "Este asistente le guiará durante la desinstalación de $(LSTR_70).$\r$\n$\r$\nAntes de iniciar la desinstalación, asegúrese de que $(LSTR_70) no se está ejecutando.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 3082 Desinstalando
LangString LSTR_42 3082 "Por favor espere mientras $(LSTR_70) se desinstala."
LangString LSTR_43 3082 "Desinstalación Finalizada"
LangString LSTR_44 3082 "La desinstalación se ha finalizado correctamente."
LangString LSTR_45 3082 "Desinstalación Abortada"
LangString LSTR_46 3082 "La desinstalación no se terminó correctamente."
LangString LSTR_47 3082 &Terminar
LangString LSTR_48 3082 "Completando el Asistente de Desinstalación de $(LSTR_70)"
LangString LSTR_49 3082 "Su computadora debe ser reiniciada para finalizar la desinstalación de $(LSTR_70). ¿Desea reiniciar ahora?"
LangString LSTR_50 3082 "Reiniciar ahora"
LangString LSTR_51 3082 "Deseo reiniciar manualmente más tarde"
LangString LSTR_52 3082 "$(LSTR_70) ha sido desinstalado de su sistema.$\r$\n$\r$\nPresione Terminar para cerrar este asistente."
LangString LSTR_53 3082 8
LangString LSTR_54 3082 "Shinri Hub está activa.$\r$\nHaz clic en Aceptar para cerrarla.$\r$\nSi no se cierra, inténtalo manualmente."
LangString LSTR_55 3082 "Closing running Shinri Hub..."
LangString LSTR_56 3082 "No se puede cerrar Shinri Hub. $\r$\nPor favor cierra la aplicación manualmente y haz clic en reintentar para continuar."
LangString LSTR_57 3082 "Se necesita Windows 7 o una versión superior."
LangString LSTR_58 3082 "Windows de 64 bits es requerido"
LangString LSTR_59 3082 Personalizada
LangString LSTR_60 3082 Cancelar
LangString LSTR_61 3082 "< &Atrás"
LangString LSTR_62 3082 &Desinstalar
LangString LSTR_63 3082 "Presione Desinstalar para iniciar la desinstalación."
LangString LSTR_64 3082 "Mostrar &detalles"
LangString LSTR_65 3082 Finalizado
LangString LSTR_66 3082 "&Siguiente >"
LangString LSTR_67 3082 "Presione Siguiente para continuar."
LangString LSTR_68 3082 " "
LangString LSTR_69 3082 &Cerrar
LangString LSTR_70 3082 "Shinri Hub"


; LANG: 1035
LangString LSTR_0 1035 "Shinri Hub 26.11.23196"
LangString LSTR_1 1035 "$(LSTR_2) -ohjelman poisto"
LangString LSTR_2 1035 "Shinri Hub"
LangString LSTR_5 1035 "Ei voi kirjoittaa: "
LangString LSTR_8 1035 "Symbolia ei löytynyt: "
LangString LSTR_9 1035 "Ei voitu ladata: "
LangString LSTR_10 1035 "Luo kansio: "
LangString LSTR_12 1035 "Poisto-ohjelma luotiin: "
LangString LSTR_13 1035 "Poista: "
LangString LSTR_14 1035 "Poista käynnistyksen yhteydessä: "
LangString LSTR_16 1035 "Virhe luotaessa: "
LangString LSTR_17 1035 "Pakettia ei voitu purkaa. Korruptoitunut asennusohjelma?"
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
LangString LSTR_36 1035 "Tiedostoon ei voitu kirjoittaa: $\r$\n$\t$\"$0$\"$\r$\nLopeta asennus valitsemalla Hylkää,$\r$\nyritä uudelleen valitsemalla Uudelleen, tai$\r$\nohita tiedosto valitsemalla Ohita"
LangString LSTR_37 1035 0
LangString LSTR_38 1035 "Tervetuloa $(LSTR_70) -ohjelman poisto-ohjelmaan"
LangString LSTR_39 1035 "MS Shell Dlg"
LangString LSTR_40 1035 "Saat tarvittavia ohjeita sitä mukaa kuin $(LSTR_70) -ohjelman poisto edistyy.$\r$\n$\r$\nEnnen poiston aloittamista varmista, ettei $(LSTR_70) ole käynnissä.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1035 Poistetaan
LangString LSTR_42 1035 "Odota... $(LSTR_70) poistetaan."
LangString LSTR_43 1035 "Poisto valmis."
LangString LSTR_44 1035 "Poisto onnistui."
LangString LSTR_45 1035 "Poisto keskeytyi."
LangString LSTR_46 1035 "Poisto epäonnistui."
LangString LSTR_47 1035 &Valmis
LangString LSTR_48 1035 "$(LSTR_70) on poistettu"
LangString LSTR_49 1035 "$(LSTR_70) -ohjelman poiston viimeisteleminen edellyttää tietokoneen uudelleenkäynnistämistä. Haluatko käynnistää tietokoneen uudelleen nyt?"
LangString LSTR_50 1035 "Käynnistä uudelleen nyt"
LangString LSTR_51 1035 "Käynnistän uudelleen myöhemmin"
LangString LSTR_52 1035 "$(LSTR_70) on poistettu tietokoneelta.$\r$\n$\r$\nValitse Lopeta sulkeaksesi poisto-ohjelman."
LangString LSTR_53 1035 8
LangString LSTR_54 1035 "Shinri Hub on käynnissä.$\r$\nValitse OK sulkeaksesi sen.$\r$\nJos se ei sulkeudu, yritä sulkea se manuaalisesti."
LangString LSTR_55 1035 "Closing running Shinri Hub..."
LangString LSTR_56 1035 "Shinri Hub ei voi sulkea. $\r$\nSulje se itse ja napsauta Yritä uudelleen jatkaaksesi."
LangString LSTR_57 1035 "Windows 7 tai uudempi vaaditaan"
LangString LSTR_58 1035 "64-bittinen Windows on pakollinen"
LangString LSTR_59 1035 Oma
LangString LSTR_60 1035 Peruuta
LangString LSTR_61 1035 "< &Takaisin"
LangString LSTR_62 1035 &Poista
LangString LSTR_63 1035 "Valitse Poista poistaaksesi asennuksen."
LangString LSTR_64 1035 "&Näytä tiedot"
LangString LSTR_65 1035 Valmis
LangString LSTR_66 1035 "&Seuraava >"
LangString LSTR_67 1035 "Valitse Seuraava jatkaaksesi."
LangString LSTR_68 1035 " "
LangString LSTR_69 1035 &Sulje
LangString LSTR_70 1035 "Shinri Hub"


; LANG: 1043
LangString LSTR_0 1043 "Shinri Hub 26.11.23196"
LangString LSTR_1 1043 $(LSTR_2)-deïnstallatie
LangString LSTR_2 1043 "Shinri Hub"
LangString LSTR_5 1043 "Kon niet schrijven: "
LangString LSTR_8 1043 "Kon symbool niet vinden: "
LangString LSTR_9 1043 "Kon niet laden: "
LangString LSTR_10 1043 "Map maken: "
LangString LSTR_12 1043 "Deïnstallatieprogramma gemaakt: "
LangString LSTR_13 1043 "Bestand verwijderen: "
LangString LSTR_14 1043 "Verwijderen na opnieuw opstarten: "
LangString LSTR_16 1043 "Fout bij maken: "
LangString LSTR_17 1043 "Fout bij uitpakken van gegevens! Gegevens beschadigd?"
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
LangString LSTR_36 1043 "Fout bij het schrijven naar bestand: $\r$\n$\r$\n$0$\r$\n$\r$\nKlik op Afbreken om de installatie te stoppen,$\r$\nOpnieuw om het opnieuw te proberen of$\r$\nNegeren om dit bestand over te slaan."
LangString LSTR_37 1043 0
LangString LSTR_38 1043 "Welkom bij de $(LSTR_70)-deïnstallatiewizard"
LangString LSTR_39 1043 "MS Shell Dlg"
LangString LSTR_40 1043 "Deze wizard zal $(LSTR_70) van uw syteem verwijderen.$\r$\n$\r$\nControleer voordat u begint met verwijderen of $(LSTR_70) is afgesloten.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1043 "Bezig met verwijderen"
LangString LSTR_42 1043 "Een ogenblik geduld terwijl $(LSTR_70) van uw systeem wordt verwijderd."
LangString LSTR_43 1043 "Verwijderen gereed"
LangString LSTR_44 1043 "$(LSTR_70) is van uw systeem verwijderd."
LangString LSTR_45 1043 "Verwijderen afgebroken"
LangString LSTR_46 1043 "$(LSTR_70) is niet volledig van uw systeem verwijderd."
LangString LSTR_47 1043 &Voltooien
LangString LSTR_48 1043 "Voltooien van de $(LSTR_70)-deïnstallatiewizard"
LangString LSTR_49 1043 "Uw systeem moet opnieuw worden opgestart om het verwijderen van $(LSTR_70) te voltooien. Wilt u nu herstarten?"
LangString LSTR_50 1043 "Nu herstarten"
LangString LSTR_51 1043 "Ik wil later handmatig herstarten"
LangString LSTR_52 1043 "$(LSTR_70) is van uw systeem verwijderd.$\r$\n$\r$\nKlik op Voltooien om deze wizard te sluiten."
LangString LSTR_53 1043 8
LangString LSTR_54 1043 "Shinri Hub is nog actief.$\r$\nKlik op OK om de applicatie te sluiten.$\r$\nAls de applicatie vervolgens nog niet sluit, probeer dan om de applicatie zelf te sluiten."
LangString LSTR_55 1043 "Closing running Shinri Hub..."
LangString LSTR_56 1043 "Shinri Hub kan niet automatisch worden afgesloten. $\r$\nSluit zelf de Shinri Hub app en klik vervolgens op ‘Opnieuw proberen’."
LangString LSTR_57 1043 "Windows 7 en hoger is vereist"
LangString LSTR_58 1043 "64-bit Windows is vereist"
LangString LSTR_59 1043 Aangepast
LangString LSTR_60 1043 Annuleren
LangString LSTR_61 1043 "< V&orige"
LangString LSTR_62 1043 &Verwijderen
LangString LSTR_63 1043 "Klik op Verwijderen om de deïnstallatie te beginnen."
LangString LSTR_64 1043 "&Details tonen"
LangString LSTR_65 1043 Voltooid
LangString LSTR_66 1043 "&Volgende >"
LangString LSTR_67 1043 "Klik op Volgende om verder te gaan."
LangString LSTR_68 1043 " "
LangString LSTR_69 1043 &Afsluiten
LangString LSTR_70 1043 "Shinri Hub"


; LANG: 1053
LangString LSTR_0 1053 "Shinri Hub 26.11.23196"
LangString LSTR_1 1053 "$(LSTR_2) Avinstallation"
LangString LSTR_2 1053 "Shinri Hub"
LangString LSTR_5 1053 "Kan inte skriva: "
LangString LSTR_8 1053 "Kunde inte hitta symbol: "
LangString LSTR_9 1053 "Kunde inte ladda: "
LangString LSTR_10 1053 "Skapa katalog: "
LangString LSTR_12 1053 "Skapade avinstallationsprogram: "
LangString LSTR_13 1053 "Radera fil: "
LangString LSTR_14 1053 "Radera vid omstart: "
LangString LSTR_16 1053 "Fel vid skapande: "
LangString LSTR_17 1053 "Fel vid uppackning av data! Skadat installationspaket?"
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
LangString LSTR_36 1053 "Fel vid Öppning av fil för skrivning: $\r$\n$\t$\"$0$\"$\r$\nKlicka på avbryt för att avbryta installationen,$\r$\nförsök igen för att försöka skriva till filen igen, eller$\r$\nIgnorera för att skippa denna fil"
LangString LSTR_37 1053 0
LangString LSTR_38 1053 "Välkommen till avinstallationsguiden för $(LSTR_70)."
LangString LSTR_39 1053 "MS Shell Dlg"
LangString LSTR_40 1053 "Denna guide tar dig igenom avinstallationen av $(LSTR_70).$\r$\n$\r$\nInnan du startar avinstallationen, försäkra dig om att $(LSTR_70) inte körs.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1053 Avinstallerar
LangString LSTR_42 1053 "Var vänlig vänta medan $(LSTR_70) avinstalleras."
LangString LSTR_43 1053 "Avinstallationen genomförd"
LangString LSTR_44 1053 "Avinstallationen genomfördes korrekt."
LangString LSTR_45 1053 "Avinstallationen avbruten"
LangString LSTR_46 1053 "Avinstallationen genomfördes inte korrekt."
LangString LSTR_47 1053 &Slutför
LangString LSTR_48 1053 "Avslutar avinstallationsguiden för $(LSTR_70)."
LangString LSTR_49 1053 "Din dator måste startas om för att fullborda avinstallationen av $(LSTR_70). Vill du starta om nu?"
LangString LSTR_50 1053 "Starta om nu"
LangString LSTR_51 1053 "Jag vill starta om själv senare"
LangString LSTR_52 1053 "$(LSTR_70) komponenter har avinstallerats från din dator.$\r$\n$\r$\nKlicka på Slutför för att avsluta guiden."
LangString LSTR_53 1053 8
LangString LSTR_54 1053 "Shinri Hub är igång.$\r$\nKlicka på OK för att stänga det.$\r$\nOm det inte stängs, försök att stänga det manuellt."
LangString LSTR_55 1053 "Closing running Shinri Hub..."
LangString LSTR_56 1053 "Shinri Hub går inte att stängas. $\r$\nVänligen stäng det manuellt och klicka på Försök igen för att fortsätta."
LangString LSTR_57 1053 "Windows 7 eller högre krävs"
LangString LSTR_58 1053 "64-bitars Windows krävs"
LangString LSTR_59 1053 Valfri
LangString LSTR_60 1053 Avbryt
LangString LSTR_61 1053 "< &Tillbaka"
LangString LSTR_62 1053 &Avinstallera
LangString LSTR_63 1053 "Klicka på Avinstallera för att starta avinstallationen."
LangString LSTR_64 1053 "Visa &detaljer"
LangString LSTR_65 1053 Slutförd
LangString LSTR_66 1053 "&Nästa >"
LangString LSTR_67 1053 "Klicka på Nästa för att fortsätta."
LangString LSTR_68 1053 " "
LangString LSTR_69 1053 &Stäng
LangString LSTR_70 1053 "Shinri Hub"


; LANG: 1055
LangString LSTR_0 1055 "Shinri Hub 26.11.23196"
LangString LSTR_1 1055 "$(LSTR_2) Kaldırma"
LangString LSTR_2 1055 "Shinri Hub"
LangString LSTR_5 1055 "Yazılamadı: "
LangString LSTR_8 1055 "Simge bulunamadı: "
LangString LSTR_9 1055 "Yüklenemedi: "
LangString LSTR_10 1055 "Dizin oluştur: "
LangString LSTR_12 1055 "Kaldırma uygulaması oluştur: "
LangString LSTR_13 1055 "Dosya sil: "
LangString LSTR_14 1055 "Açılışta sil: "
LangString LSTR_16 1055 "Oluşturma hatası: "
LangString LSTR_17 1055 "Veriyi açarken hata meydana geldi! Acaba kurulum uygulaması mı bozuk?"
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
LangString LSTR_36 1055 "Dosya yazmak için açılırken hata meydana geldi: $\r$\n$\r$\n$0$\r$\n$\r$\nKurulumu durdurmak için Dur düğmesine,$\r$\nyeniden denemek için Yeniden Dene düğmesine,$\r$\nbu dosyayı atlamak için Yoksay düğmesine basın."
LangString LSTR_37 1055 0
LangString LSTR_38 1055 "$(LSTR_70) Programını Kaldırma Sihirbazına Hoş Geldiniz"
LangString LSTR_39 1055 "MS Shell Dlg"
LangString LSTR_40 1055 "Bu sihirbaz size $(LSTR_70) programının kadırılımı boyunca rehberlik edecektir.$\r$\n$\r$\nKaldırım işlemeni başlatmadan önce çalışan diğer programlari kapatmanızı öneririz. Böylece bilgisayarınızı yeniden başlatmadan bazı sistem dosyaları sorunsuz kaldırılabilir.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1055 Kaldırılıyor
LangString LSTR_42 1055 "Litfen $(LSTR_70) programı sisteminizden kaldırılırken bekleyiniz."
LangString LSTR_43 1055 "Kaldırma İşlemi Tamamlandır"
LangString LSTR_44 1055 "Kaldırma işlemi başarıyla tamamlandı."
LangString LSTR_45 1055 "Kaldırma İşlemi İptal Edildi"
LangString LSTR_46 1055 "Kaldırma İşlemi tamamlanamadı."
LangString LSTR_47 1055 &Bitir
LangString LSTR_48 1055 "$(LSTR_70) Programı Kaldırma Sihirbazı Tamamlanıyor"
LangString LSTR_49 1055 "$(LSTR_70) programını kaldırma işleminin tamamlanması için bilgisayarınızın yeniden başlatılması gerekiyor. Bilgisayarınızın şimdi yeniden başlatılmasını ister misiniz?"
LangString LSTR_50 1055 "Yeniden başlat"
LangString LSTR_51 1055 "Bilgisayarımı daha sonra başlatacağım."
LangString LSTR_52 1055 "$(LSTR_70) programı sisteminizden kaldırıldı.$\r$\n$\r$\nSihirbazı kapatmak için 'bitir'e basınız."
LangString LSTR_53 1055 8
LangString LSTR_54 1055 "Shinri Hub çalışıyor.$\r$\nKapatmak için Tamam'ı tıklayın."
LangString LSTR_55 1055 "Closing running Shinri Hub..."
LangString LSTR_56 1055 "Shinri Hub kapatılamaz. $\r$\nLütfen elle kapatın ve devam etmek için Tekrar'a tıklayın"
LangString LSTR_57 1055 "Windows 7 ve üstü gereklidir"
LangString LSTR_58 1055 "64 bit Windows gerekli"
LangString LSTR_59 1055 Özel
LangString LSTR_60 1055 Vazgeç
LangString LSTR_61 1055 "< &Geri"
LangString LSTR_62 1055 &Kaldır
LangString LSTR_63 1055 "Kaldırmayı başlatmak için Kaldır düğmesine basın."
LangString LSTR_64 1055 "&Ayrıntıları göster"
LangString LSTR_65 1055 Tamamlandı
LangString LSTR_66 1055 "İ&leri >"
LangString LSTR_67 1055 "Devam etmek için İleri düğmesine basın."
LangString LSTR_68 1055 " "
LangString LSTR_69 1055 &Kapat
LangString LSTR_70 1055 "Shinri Hub"


; LANG: 1028
LangString LSTR_0 1028 "Shinri Hub 26.11.23196"
LangString LSTR_1 1028 "$(LSTR_2) 解除安裝"
LangString LSTR_2 1028 "Shinri Hub"
LangString LSTR_5 1028 "無法寫入: "
LangString LSTR_8 1028 "無法找到符號: "
LangString LSTR_9 1028 "無法載入: "
LangString LSTR_10 1028 "$\"建立資料夾: $\" "
LangString LSTR_12 1028 "建立解除安裝程式: "
LangString LSTR_13 1028 "刪除檔案: "
LangString LSTR_14 1028 "重新開機後刪除: "
LangString LSTR_16 1028 "建立時發生錯誤: "
LangString LSTR_17 1028 解壓縮資料發生錯誤！安裝程式是否已損壞？
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
LangString LSTR_36 1028 "無法開啟要寫入的檔案: $\r$\n$\t$\"$0$\"$\r$\n按「中止」停止安裝，$\r$\n「重試」重新嘗試寫入檔案，或$\r$\n「忽略」略過此檔案。"
LangString LSTR_37 1028 0
LangString LSTR_38 1028 "歡迎使用 $(LSTR_70) 解除安裝精靈"
LangString LSTR_39 1028 新細明體
LangString LSTR_40 1028 "本精靈會引導您解除安裝 $(LSTR_70)。$\r$\n$\r$\n在開始解除安裝之前，請確認 $(LSTR_70) 並未有執行。$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1028 解除安裝
LangString LSTR_42 1028 "$(LSTR_70) 正在解除安裝，請稍候。"
LangString LSTR_43 1028 完成解除安裝
LangString LSTR_44 1028 解除安裝已完成。
LangString LSTR_45 1028 已中止解除安裝
LangString LSTR_46 1028 解除安裝並未完成。
LangString LSTR_47 1028 完成(&F)
LangString LSTR_48 1028 "即將完成解除安裝 $(LSTR_70)"
LangString LSTR_49 1028 "電腦需要重新開機，才能完成解除安裝 $(LSTR_70) 。要馬上重新開機嗎？"
LangString LSTR_50 1028 是，馬上重新開機(&Y)
LangString LSTR_51 1028 否，我稍後再自行重新開機(&N)
LangString LSTR_52 1028 "已自電腦解除安裝 $(LSTR_70) 。$\r$\n$\r$\n按 [完成(F)] 關閉安裝程式。"
LangString LSTR_53 1028 9
LangString LSTR_54 1028 "Shinri Hub 正在執行。$\r$\n點擊 OK 關閉。$\r$\n如果無法關閉，試著手動關閉。"
LangString LSTR_55 1028 "Closing running Shinri Hub..."
LangString LSTR_56 1028 "Shinri Hub 無法關閉。$\r$\n請手動關閉它，然後按一下重試以繼續。"
LangString LSTR_57 1028 "需要 Windows 7 或更新的系統"
LangString LSTR_58 1028 "需要 64 位元 Windows 系統"
LangString LSTR_59 1028 自訂
LangString LSTR_60 1028 取消
LangString LSTR_61 1028 "< 上一步(&B)"
LangString LSTR_62 1028 解除安裝(&U)
LangString LSTR_63 1028 按「解除安裝(U)」開始解除安裝。
LangString LSTR_64 1028 顯示細節(&D)
LangString LSTR_65 1028 已完成
LangString LSTR_66 1028 "下一步(&N) >"
LangString LSTR_67 1028 按「下一步(N)」繼續。
LangString LSTR_68 1028 " "
LangString LSTR_69 1028 關閉(&C)
LangString LSTR_70 1028 "Shinri Hub"


; LANG: 2052
LangString LSTR_0 2052 "Shinri Hub 26.11.23196"
LangString LSTR_1 2052 "$(LSTR_2) 解除安装"
LangString LSTR_2 2052 "Shinri Hub"
LangString LSTR_5 2052 "无法写入: "
LangString LSTR_8 2052 "无法找到符号: "
LangString LSTR_9 2052 "无法加载: "
LangString LSTR_10 2052 "$\"创建文件夹: $\" "
LangString LSTR_12 2052 "创建解除安装程序: "
LangString LSTR_13 2052 "删除文件: "
LangString LSTR_14 2052 "重新启动后删除: "
LangString LSTR_16 2052 "正在创建时发生错误: "
LangString LSTR_17 2052 正在解压缩数据发生错误！已损坏的安装程序？
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
LangString LSTR_36 2052 "不能打开要写入的文件: $\r$\n$\t$\"$0$\"$\r$\n单击 [Abort] 停止安装，$\r$\n [Retry] 重新尝试写入文件，或$\r$\n [Ignore] 忽略这个文件。"
LangString LSTR_37 2052 0
LangString LSTR_38 2052 "欢迎使用 $(LSTR_70) 解除安装向导"
LangString LSTR_39 2052 宋体
LangString LSTR_40 2052 "这个向导将全程指引你 $(LSTR_70) 的解除安装进程。$\r$\n$\r$\n在开始解除安装之前，确认 $(LSTR_70) 并未运行当中。$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 2052 正在解除安装
LangString LSTR_42 2052 "$(LSTR_70) 正在解除安装，请等候。"
LangString LSTR_43 2052 解除安装已完成
LangString LSTR_44 2052 解除安装程序已成功地运行完成。
LangString LSTR_45 2052 解除安装已中止
LangString LSTR_46 2052 解除安装程序并未成功地运行完成。
LangString LSTR_47 2052 完成(&F)
LangString LSTR_48 2052 "正在完成 $(LSTR_70) 解除安装向导"
LangString LSTR_49 2052 "计算机需要重新启动，以便完成 $(LSTR_70) 的解除安装。现在想要重新启动吗？"
LangString LSTR_50 2052 是，现在重新启动(&Y)
LangString LSTR_51 2052 否，我稍后再自行重新启动(&N)
LangString LSTR_52 2052 "$(LSTR_70) 已从你的计算机解除安装。$\r$\n$\r$\n单击 [完成] 关闭这个向导。"
LangString LSTR_53 2052 9
LangString LSTR_54 2052 "Shinri Hub 正在运行.$\r$\n点击“确定”关闭."
LangString LSTR_55 2052 "Closing running Shinri Hub..."
LangString LSTR_56 2052 "Shinri Hub 无法关闭。$\r$\n请手动关闭它，然后单击重试以继续。"
LangString LSTR_57 2052 "需要 Windows 7 或以上系统"
LangString LSTR_58 2052 "需要 64 位 Windows 系统"
LangString LSTR_59 2052 自定义
LangString LSTR_60 2052 取消(&C)
LangString LSTR_61 2052 "< 上一步(&P)"
LangString LSTR_62 2052 解除安装(&U)
LangString LSTR_63 2052 "单击 [解除安装(U)] 开始解除安装进程。"
LangString LSTR_64 2052 显示细节(&D)
LangString LSTR_65 2052 已完成
LangString LSTR_66 2052 "下一步(&N) >"
LangString LSTR_67 2052 "单击 [下一步(N)] 继续。"
LangString LSTR_68 2052 " "
LangString LSTR_69 2052 关闭(&L)
LangString LSTR_70 2052 "Shinri Hub"


; LANG: 1046
LangString LSTR_0 1046 "Shinri Hub 26.11.23196"
LangString LSTR_1 1046 "Desinstalação do $(LSTR_2)"
LangString LSTR_2 1046 "Shinri Hub"
LangString LSTR_5 1046 "Não foi possível gravar: "
LangString LSTR_8 1046 "Não foi possível localizar o símbolo: "
LangString LSTR_9 1046 "Não foi possível carregar: "
LangString LSTR_10 1046 "Criar pasta: "
LangString LSTR_12 1046 "Desinstalador criado: "
LangString LSTR_13 1046 "Excluir o arquivo: "
LangString LSTR_14 1046 "Excluir ao reiniciar: "
LangString LSTR_16 1046 "Erro ao criar: "
LangString LSTR_17 1046 "Erro ao descomprimir os dados! Instalador corrompido?"
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
LangString LSTR_36 1046 "Erro ao abrir o arquivo para gravação: $\r$\n$\r$\n$0$\r$\n$\r$\nClique em Abortar para parar a instalação,$\r$\nRepetir para tentar de novo, ou$\r$\nIgnorar para pular este arquivo."
LangString LSTR_37 1046 0
LangString LSTR_38 1046 "Bem-vindo ao Desinstalador do $(LSTR_70)"
LangString LSTR_39 1046 "MS Shell Dlg"
LangString LSTR_40 1046 "Este assistente guiará você durante a desinstalação do $(LSTR_70).$\r$\n$\r$\nAntes de iniciar a desinstalação, certifique-se que o $(LSTR_70) não está em execução.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1046 Desinstalando
LangString LSTR_42 1046 "Por favor espere enquanto o $(LSTR_70) está sendo desinstalado."
LangString LSTR_43 1046 "Desinstalação Completa"
LangString LSTR_44 1046 "A desinstalação foi completada com sucesso."
LangString LSTR_45 1046 "Desinstalação Abortada"
LangString LSTR_46 1046 "A desinstalação não foi completada com sucesso."
LangString LSTR_47 1046 &Concluir
LangString LSTR_48 1046 "Concluindo a desinstalação do $(LSTR_70)"
LangString LSTR_49 1046 "Seu computador deve ser reiniciado para completar a desinstalação do $(LSTR_70). Você quer reiniciar agora?"
LangString LSTR_50 1046 "Reiniciar agora"
LangString LSTR_51 1046 "Eu quero reiniciar manualmente depois"
LangString LSTR_52 1046 "O $(LSTR_70) foi desinstalado do seu computador.$\r$\n$\r$\nClique em Concluir para fechar o instalador."
LangString LSTR_53 1046 8
LangString LSTR_54 1046 "Shinri Hub está funcionando.$\r$\nClique no OK para fechar.$\r$\nSe não fechar, tente fechá-lo manualmente."
LangString LSTR_55 1046 "Closing running Shinri Hub..."
LangString LSTR_56 1046 "Não é possível fechar o Shinri Hub. $\r$\nFeche a janela do Shinri Hub e clique em Repetir para continuar."
LangString LSTR_57 1046 "Requer Windows 7 ou superior"
LangString LSTR_58 1046 "Requer Windows 64 bits"
LangString LSTR_59 1046 Personalizado
LangString LSTR_60 1046 Cancelar
LangString LSTR_61 1046 "< &Voltar"
LangString LSTR_62 1046 &Desinstalar
LangString LSTR_63 1046 "Clique em Desinstalar para iniciar a desinstalação."
LangString LSTR_64 1046 "Mostrar &detalhes"
LangString LSTR_65 1046 Completado
LangString LSTR_66 1046 "&Próximo >"
LangString LSTR_67 1046 "Clique em Próximo para continuar."
LangString LSTR_68 1046 " "
LangString LSTR_69 1046 &Fechar
LangString LSTR_70 1046 "Shinri Hub"


; LANG: 1045
LangString LSTR_0 1045 "Shinri Hub 26.11.23196"
LangString LSTR_1 1045 "Deinstalator $(LSTR_2)"
LangString LSTR_2 1045 "Shinri Hub"
LangString LSTR_5 1045 "Nie można zapisać: "
LangString LSTR_8 1045 "Nie można odnaleźć symbolu: "
LangString LSTR_9 1045 "Nie można wczytać: "
LangString LSTR_10 1045 "Utwórz folder: "
LangString LSTR_12 1045 "Utworzono deinstalator: "
LangString LSTR_13 1045 "Usuń plik: "
LangString LSTR_14 1045 "Usuń przy ponownym uruchomieniu: "
LangString LSTR_16 1045 "Błąd tworzenia: "
LangString LSTR_17 1045 "Błąd wyodrębniania danych! Uszkodzony instalator?"
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
LangString LSTR_36 1045 "Błąd otwarcia pliku do zapisu: $\r$\n$\r$\n$0$\r$\n$\r$\nWybierz 'Anuluj', aby przerwać instalację,$\r$\n'Ponów', aby ponowić zapis do pliku lub$\r$\n'Ignoruj', aby pominąć ten plik."
LangString LSTR_37 1045 0
LangString LSTR_38 1045 "Witamy w kreatorze deinstalacji $(LSTR_70)"
LangString LSTR_39 1045 "MS Shell Dlg"
LangString LSTR_40 1045 "Kreator ten pomoże Ci odinstalować $(LSTR_70).$\r$\n$\r$\nPrzed rozpoczęciem deinstalacji, upewnij się, że $(LSTR_70) nie jest uruchomiony.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1045 Deinstalacja
LangString LSTR_42 1045 "Proszę czekać, trwa deinstalacja $(LSTR_70)."
LangString LSTR_43 1045 "Zakończono deinstalację"
LangString LSTR_44 1045 "Odinstalowanie zakończone pomyślnie."
LangString LSTR_45 1045 "Deinstalacja przerwana"
LangString LSTR_46 1045 "Deinstalacja nie została zakończona pomyślnie."
LangString LSTR_47 1045 &Zakończ
LangString LSTR_48 1045 "Kończenie pracy kreatora deinstalacji $(LSTR_70)"
LangString LSTR_49 1045 "Twój komputer musi zostać ponownie uruchomiony w celu zakończenia deinstalacji $(LSTR_70). Czy chcesz teraz ponownie uruchomić komputer?"
LangString LSTR_50 1045 "Uruchom ponownie teraz"
LangString LSTR_51 1045 "Sam uruchomię ponownie komputer później"
LangString LSTR_52 1045 "$(LSTR_70) został odinstalowany z Twojego komputera.$\r$\n$\r$\nKliknij przycisk 'Zakończ', aby zakończyć działanie deinstalatora."
LangString LSTR_53 1045 8
LangString LSTR_54 1045 "Aplikacja Shinri Hub jest uruchomiona.$\r$\nKliknij OK, aby ją zamknąć.$\r$\nJeśli nie zostanie zamknięta, spróbuj zamknąć ją ręcznie."
LangString LSTR_55 1045 "Closing running Shinri Hub..."
LangString LSTR_56 1045 "Nie można zamknąć Shinri Hub. $\r$\nZamknij aplikację i kliknij Ponów, aby kontynuować."
LangString LSTR_57 1045 "Windows 7 lub nowszy jest wymagany"
LangString LSTR_58 1045 "Windows 64-bitowy jest wymagany"
LangString LSTR_59 1045 Użytkownika
LangString LSTR_60 1045 Anuluj
LangString LSTR_61 1045 "< &Wstecz"
LangString LSTR_62 1045 &Odinstaluj
LangString LSTR_63 1045 "Kliknij przycisk 'Odinstaluj', aby rozpocząć deinstalację."
LangString LSTR_64 1045 "Pokaż &szczegóły"
LangString LSTR_65 1045 Zakończono
LangString LSTR_66 1045 "&Dalej >"
LangString LSTR_67 1045 "Kliknij przycisk 'Dalej', aby kontynuować."
LangString LSTR_68 1045 " "
LangString LSTR_69 1045 &Zamknij
LangString LSTR_70 1045 "Shinri Hub"


; LANG: 1038
LangString LSTR_0 1038 "Shinri Hub 26.11.23196"
LangString LSTR_1 1038 "$(LSTR_2) Eltávolító"
LangString LSTR_2 1038 "Shinri Hub"
LangString LSTR_5 1038 "Nem írható: "
LangString LSTR_8 1038 "A következő szimbólum nem található: "
LangString LSTR_9 1038 "Nem tölthető be: "
LangString LSTR_10 1038 "Mappa létrehozás: "
LangString LSTR_12 1038 "Létrehozott eltávolító: "
LangString LSTR_13 1038 "Törölt fájl: "
LangString LSTR_14 1038 "Rendszerindításkor törlendő: "
LangString LSTR_16 1038 "Hiba a létrehozáskor: "
LangString LSTR_17 1038 "Hiba az adatok kibontásakor! Megsérült a Telepítő?"
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
LangString LSTR_36 1038 "Hiba történt a fájl írásra történő megnyitásakor: $\r$\n$\t$\"$0$\"$\r$\nA Mégse gomb megnyomásával megszakíthatja a telepítést,$\r$\naz Ismét gombbal megismételheti a fájl írását,$\r$\na Kihagyás gombbal kihagyhatja ezt a fájlt."
LangString LSTR_37 1038 0
LangString LSTR_38 1038 "Üdvözli a(z) $(LSTR_70) Eltávolító Varázsló"
LangString LSTR_39 1038 "MS Shell Dlg"
LangString LSTR_40 1038 "Ez a varázsló segíti a(z) $(LSTR_70) eltávolításában.$\r$\n$\r$\nMielőtt elkezdi az eltávilítást győződjön meg arról, hogy a(z) $(LSTR_70) nem fut.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1038 "Eltávolítási folyamat"
LangString LSTR_42 1038 "Kis türelmet a(z) $(LSTR_70) eltávolításáig."
LangString LSTR_43 1038 "Az eltávolítás befejeződött"
LangString LSTR_44 1038 "Az eltávolítás sikeresen befejeződött."
LangString LSTR_45 1038 "Az eltávolítás megszakadt"
LangString LSTR_46 1038 "Az eltávolítás sikertelen volt."
LangString LSTR_47 1038 &Befejezés
LangString LSTR_48 1038 "A(z) $(LSTR_70) eltávolítás varázslójának befejezése."
LangString LSTR_49 1038 "A számítógépet újra kell indítani, hogy a(z) $(LSTR_70) eltávolítása teljes legyen. Akarja most újraindítani a rendszert?"
LangString LSTR_50 1038 "Most indítom újra"
LangString LSTR_51 1038 "Később fogom újraindítani"
LangString LSTR_52 1038 "A(z) $(LSTR_70) eltávolítása sikeresen befejeződött.$\r$\n$\r$\nA Finish-re kattintva bezárul ez a varázsló."
LangString LSTR_53 1038 8
LangString LSTR_54 1038 "A Shinri Hub alkalmazás fut.$\r$\nKattints az OK gombra a bezárásához.$\r$\nHa így sem áll le, zárd be manuálisan."
LangString LSTR_55 1038 "Closing running Shinri Hub..."
LangString LSTR_56 1038 "Nem sikerült bezárni a Shinri Hubt. $\r$\nZárd be kézzel, majd kattints az Újra gombra a folytatáshoz!"
LangString LSTR_57 1038 "Windows 7 vagy újabb szükséges"
LangString LSTR_58 1038 "64-bit Windows szükséges"
LangString LSTR_59 1038 Egyéni
LangString LSTR_60 1038 &Mégse
LangString LSTR_61 1038 "< &Vissza"
LangString LSTR_62 1038 &Eltávolítás
LangString LSTR_63 1038 "Kattintson az Eltávolításra az eltávolításhoz."
LangString LSTR_64 1038 &Részletek
LangString LSTR_65 1038 Kész
LangString LSTR_66 1038 "&Tovább >"
LangString LSTR_67 1038 "Kattintson a Tovább-ra a folytatáshoz."
LangString LSTR_68 1038 " "
LangString LSTR_69 1038 &Bezárás
LangString LSTR_70 1038 "Shinri Hub"


; LANG: 1029
LangString LSTR_0 1029 "Shinri Hub 26.11.23196"
LangString LSTR_1 1029 "Odinstalace $(LSTR_2)"
LangString LSTR_2 1029 "Shinri Hub"
LangString LSTR_5 1029 "Nelze zapsat: "
LangString LSTR_8 1029 "Nelze najít symbol: "
LangString LSTR_9 1029 "Nelze načíst: "
LangString LSTR_10 1029 "Vytváření složky: "
LangString LSTR_12 1029 "Vytváření odinstalačního programu: "
LangString LSTR_13 1029 "Odstraňování souboru: "
LangString LSTR_14 1029 "Odstranit při restartování: "
LangString LSTR_16 1029 "Při vytváření došlo k chybě: "
LangString LSTR_17 1029 "Při dekompresi dat došlo k chybě. Byl poškozen instalační program?"
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
LangString LSTR_36 1029 "Soubor nelze otevřít pro zápis: $\r$\n$\t$\"$0$\"$\r$\nKlikněte na tlačítko Přerušit k ukončení instalace,$\r$\nnebo na tlačítko Opakovat k zopakování akce, nebo$\r$\n na tlačítko Ignorovat k přeskočení souboru."
LangString LSTR_37 1029 0
LangString LSTR_38 1029 "Vítejte v průvodci odinstalace programu $(LSTR_70)"
LangString LSTR_39 1029 "MS Shell Dlg"
LangString LSTR_40 1029 "Tento průvodce vás provede odinstalací programu $(LSTR_70).$\r$\n$\r$\nPřed spuštěním odinstalace se přesvědčte, zda program $(LSTR_70) není spuštěn.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1029 Odinstalace
LangString LSTR_42 1029 "Počkejte prosím na dokončení odinstalace programu $(LSTR_70)."
LangString LSTR_43 1029 "Odinstalace dokončena"
LangString LSTR_44 1029 "Odinstalace byla úspěšně dokončena."
LangString LSTR_45 1029 "Odinstalace přerušena"
LangString LSTR_46 1029 "Odinstalace nebyla úspěšně dokončena."
LangString LSTR_47 1029 &Dokončit
LangString LSTR_48 1029 "Dokončení odinstalace programu $(LSTR_70)"
LangString LSTR_49 1029 "K dokončení odinstalace $(LSTR_70) je nutné restartovat počítač. Chcete jej restartovat nyní?"
LangString LSTR_50 1029 "Restartovat nyní"
LangString LSTR_51 1029 "Restartovat později ručně"
LangString LSTR_52 1029 "Program $(LSTR_70) byl odinstalován z počítače.$\r$\n$\r$\nKliknutím na tlačítko Dokončit ukončíte tohoto průvodce."
LangString LSTR_53 1029 8
LangString LSTR_54 1029 "Shinri Hub běží.$\r$\nPotvrďte zavření stisknutím OK.$\r$\nPokud se aplikace nezavře sama, zavřete ji ručně."
LangString LSTR_55 1029 "Ukončuji aplikaci Shinri Hub..."
LangString LSTR_56 1029 "Aplikaci Shinri Hub se nepodařilo zavřít. $\r$\nZavřete ji ručně a poté klikněte na Opakovat pro pokračování."
LangString LSTR_57 1029 "Jsou podporovány pouze Windows 7 a vyšší"
LangString LSTR_58 1029 "Potřebujete 64-bit Windows"
LangString LSTR_59 1029 Vlastní
LangString LSTR_60 1029 Storno
LangString LSTR_61 1029 "< &Zpět"
LangString LSTR_62 1029 &Odinstalovat
LangString LSTR_63 1029 "Odinstalaci spustíte kliknutím na tlačítko Odinstalovat."
LangString LSTR_64 1029 &Podrobnosti
LangString LSTR_65 1029 Dokončeno
LangString LSTR_66 1029 "&Další >"
LangString LSTR_67 1029 "Pokračujte kliknutím na tlačítko Další."
LangString LSTR_68 1029 " "
LangString LSTR_69 1029 &Zavřít
LangString LSTR_70 1029 "Shinri Hub"


; LANG: 1051
LangString LSTR_0 1051 "Shinri Hub 26.11.23196"
LangString LSTR_1 1051 "Odinštalovanie programu $(LSTR_2)"
LangString LSTR_2 1051 "Shinri Hub"
LangString LSTR_5 1051 "Nemožno zapísať súbor: "
LangString LSTR_8 1051 "Nemožno nájsť symbol: "
LangString LSTR_9 1051 "Nemožno načítať: "
LangString LSTR_10 1051 "Vytvorený priečinok: "
LangString LSTR_12 1051 "Program pre odinštalovanie: "
LangString LSTR_13 1051 "Vymazaný súbor: "
LangString LSTR_14 1051 "Vymazať po reštartovaní systému: "
LangString LSTR_16 1051 "Chyba pri vytváraní: "
LangString LSTR_17 1051 "Chyba pri dekomprimovaní dát! Inštalátor je pravdepodobne poškodený..."
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
LangString LSTR_36 1051 "Chyba pri otváraní súboru na zápis: $\r$\n$\r$\n$0$\r$\n$\r$\n. Ak chcete inštaláciu ukončiť, kliknite na tlačidlo Ukončiť,$\r\ ak chcete zápis súboru zopakovať, kliknite na tlačidlo Opakovať alebo kliknite na tlačidlo $\r$\nIgnorovať, ak chcete inštaláciu tohto súboru vynechať."
LangString LSTR_37 1051 0
LangString LSTR_38 1051 "Vitajte v sprievodcovi odinštalovaním programu $(LSTR_70)"
LangString LSTR_39 1051 "MS Shell Dlg"
LangString LSTR_40 1051 "Tento sprievodca vás prevedie procesom odinštalovania programu $(LSTR_70).$\r$\n$\r$\nPred spustením procesu odinštalovania sa uistite, že program $(LSTR_70) nie je práve aktívny.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1051 Odinštalovanie
LangString LSTR_42 1051 "Čakajte prosím, kým prebehne odinštalovanie programu $(LSTR_70)."
LangString LSTR_43 1051 "Ukončenie odinštalovania"
LangString LSTR_44 1051 "Odinštalovanie bolo úspešne dokončené."
LangString LSTR_45 1051 "Prerušenie odinštalovania"
LangString LSTR_46 1051 "Odinštalovanie sa neukončilo úspešne."
LangString LSTR_47 1051 &Dokončiť
LangString LSTR_48 1051 "Dokončenie sprievodcu odinštalovaním"
LangString LSTR_49 1051 "Pre úplné dokončenie odinštalovania programu $(LSTR_70) je nutné reštartovať váš počítač. Chcete ho reštartovať ihneď?"
LangString LSTR_50 1051 "Reštartovať teraz"
LangString LSTR_51 1051 "Reštartovať neskôr (manuálne)"
LangString LSTR_52 1051 "Program $(LSTR_70) bol odinštalovaný z vášho počítača.$\r$\n$\r$\nKliknite na tlačidlo Dokončiť a tento sprievodca sa ukončí."
LangString LSTR_53 1051 8
LangString LSTR_54 1051 "Shinri Hub beží.$\r$\nKliknutím na tlačidlo OK ho zatvoríte.$\r$\nAk sa nezavrie, skúste ho zatvoriť ručne."
LangString LSTR_55 1051 "Closing running Shinri Hub..."
LangString LSTR_56 1051 "Shinri Hub nie je možné zatvoriť. $\r$\nZatvorte ho ručne a pokračujte kliknutím na položku Opakovať."
LangString LSTR_57 1051 "Iba Windows 7 a vyššie je podporovaný"
LangString LSTR_58 1051 "Potrebujete 64-bit Windows"
LangString LSTR_59 1051 Voliteľná
LangString LSTR_60 1051 Zrušiť
LangString LSTR_61 1051 "< &Späť"
LangString LSTR_62 1051 &Odinštalovať
LangString LSTR_63 1051 "Pre spustenie procesu odinštalovania kliknite na tlačidlo Odinštalovať."
LangString LSTR_64 1051 &Podrobnosti
LangString LSTR_65 1051 Hotovo
LangString LSTR_66 1051 "Ď&alej >"
LangString LSTR_67 1051 "V inštalácii pokračujte kliknutím na tlačidlo Ďalej."
LangString LSTR_68 1051 " "
LangString LSTR_69 1051 &Zatvoriť
LangString LSTR_70 1051 "Shinri Hub"


; LANG: 1049
LangString LSTR_0 1049 "Shinri Hub 26.11.23196"
LangString LSTR_1 1049 "Удаление $(LSTR_2)"
LangString LSTR_2 1049 "Shinri Hub"
LangString LSTR_5 1049 "Невозможно записать: "
LangString LSTR_8 1049 "Невозможно найти символ: "
LangString LSTR_9 1049 "Невозможно загрузить: "
LangString LSTR_10 1049 "Создание папки: "
LangString LSTR_12 1049 "Создание программы удаления: "
LangString LSTR_13 1049 "Удаление файла: "
LangString LSTR_14 1049 "Удаление при перезагрузке ПК: "
LangString LSTR_16 1049 "Ошибка создания: "
LangString LSTR_17 1049 "Ошибка распаковки данных! Возможно, повреждён дистрибутив."
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
LangString LSTR_36 1049 "Невозможно открыть файл для записи: $\r$\n$\t$\"$0$\"$\r$\n$\"Прервать$\": остановить установку;$\r$\n$\"Повтор$\": повторить попытку;$\r$\n$\"Пропуск$\": пропустить это действие."
LangString LSTR_37 1049 0
LangString LSTR_38 1049 "Вас приветствует мастер удаления $(LSTR_70)"
LangString LSTR_39 1049 "MS Shell Dlg"
LangString LSTR_40 1049 "Эта программа удалит $(LSTR_70) из вашего компьютера.$\r$\n$\r$\nПеред началом удаления убедитесь, что программа $(LSTR_70) не запущена.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1049 Удаление
LangString LSTR_42 1049 "Подождите, идет удаление файлов $(LSTR_70)..."
LangString LSTR_43 1049 "Удаление завершено"
LangString LSTR_44 1049 "Удаление программы успешно завершено."
LangString LSTR_45 1049 "Удаление прервано"
LangString LSTR_46 1049 "Удаление произведено не полностью."
LangString LSTR_47 1049 &Готово
LangString LSTR_48 1049 "Завершение работы мастера удаления $(LSTR_70)"
LangString LSTR_49 1049 "Для завершения удаления $(LSTR_70) нужно перезагрузить компьютер. Хотите сделать это сейчас?"
LangString LSTR_50 1049 "Да, перезагрузить ПК сейчас"
LangString LSTR_51 1049 "Нет, я перезагружу ПК позже"
LangString LSTR_52 1049 "Программа $(LSTR_70) удалена из вашего компьютера.$\r$\n$\r$\nНажмите кнопку $\"Готово$\"для выхода из программы удаления."
LangString LSTR_53 1049 8
LangString LSTR_54 1049 "Приложение Shinri Hub запущено.$\r$\nНажмите «OK», чтобы закрыть его.$\r$\nЕсли оно не закрывается, попробуйте закрыть его вручную."
LangString LSTR_55 1049 "Closing running Shinri Hub..."
LangString LSTR_56 1049 "Не удалось закрыть Shinri Hub. $\r$\nПожалуйста, закройте Shinri Hub вручную и нажмите «Повторить», чтобы продолжить."
LangString LSTR_57 1049 "Требуется Windows 7 и выше"
LangString LSTR_58 1049 "Требуется Windows 64-bit"
LangString LSTR_59 1049 "По выбору"
LangString LSTR_60 1049 Отмена
LangString LSTR_61 1049 "< &Назад"
LangString LSTR_62 1049 Уд&алить
LangString LSTR_63 1049 "Нажмите кнопку $\"Удалить$\", чтобы удалить программу."
LangString LSTR_64 1049 &Детали...
LangString LSTR_65 1049 Готово
LangString LSTR_66 1049 "&Далее >"
LangString LSTR_67 1049 "Нажмите кнопку $\"Далее$\" для продолжения."
LangString LSTR_68 1049 " "
LangString LSTR_69 1049 &Закрыть
LangString LSTR_70 1049 "Shinri Hub"


; LANG: 1042
LangString LSTR_0 1042 "Shinri Hub 26.11.23196"
LangString LSTR_1 1042 "$(LSTR_2) 제거"
LangString LSTR_2 1042 "Shinri Hub"
LangString LSTR_5 1042 "기록할 수 없음: "
LangString LSTR_8 1042 "심볼을 찾을 수 없음: "
LangString LSTR_9 1042 "불러올 수 없음: "
LangString LSTR_10 1042 "폴더 생성: "
LangString LSTR_12 1042 "언인스톨러 생성: "
LangString LSTR_13 1042 "파일 삭제: "
LangString LSTR_14 1042 "재부팅시 삭제: "
LangString LSTR_16 1042 "생성 실패: "
LangString LSTR_17 1042 "압축 해제중 오류 발생! 설치 파일이 손상되었습니다."
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
LangString LSTR_36 1042 "다음 파일을 열 수 없습니다.: $\r$\n$\t$\"$0$\"$\r$\n'중단'을 눌러 설치를 종료하거나,$\r'다시 시도'를 눌러 다시 시도해 보거나,$\r'무시'를 눌러 이 파일을 건너 뛰세요."
LangString LSTR_37 1042 0
LangString LSTR_38 1042 "$(LSTR_70) 제거를 시작합니다."
LangString LSTR_39 1042 굴림
LangString LSTR_40 1042 "이 프로그램은 당신의 컴퓨터에서 $(LSTR_70)(을)를 제거할 것입니다.$\r$\n$\r$\n제거를 시작하기 전에 $(LSTR_70)(을)를 종료하여 주시기 바랍니다.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1042 제거중
LangString LSTR_42 1042 "$(LSTR_70)(을)를 제거하는 동안 잠시 기다려 주시기 바랍니다."
LangString LSTR_43 1042 "제거 마침"
LangString LSTR_44 1042 "프로그램을 성공적으로 제거하였습니다."
LangString LSTR_45 1042 "프로그램 제거 취소"
LangString LSTR_46 1042 "프로그램 제거가 취소되었습니다."
LangString LSTR_47 1042 마침
LangString LSTR_48 1042 "$(LSTR_70) 제거 완료"
LangString LSTR_49 1042 "$(LSTR_70)의 제거를 완료하기 위해서는 시스템을 다시 시작해야 합니다. 지금 재부팅 하시겠습니까?"
LangString LSTR_50 1042 "지금 재부팅 하겠습니다."
LangString LSTR_51 1042 "나중에 재부팅 하겠습니다."
LangString LSTR_52 1042 "$(LSTR_70)의 제거가 완료 되었습니다.$\r$\n$\r$\n설치 프로그램을 마치려면 '마침' 버튼을 눌러 주세요."
LangString LSTR_53 1042 9
LangString LSTR_54 1042 "Shinri Hub이(가) 실행 중입니다. $\r$\nOK을 클릭하면 종료됩니다."
LangString LSTR_55 1042 "Closing running Shinri Hub..."
LangString LSTR_56 1042 "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
LangString LSTR_57 1042 "Windows 7 이상이 필요합니다"
LangString LSTR_58 1042 "64 비트 버전의 Windows가 필요합니다"
LangString LSTR_59 1042 "사용자 정의"
LangString LSTR_60 1042 취소
LangString LSTR_61 1042 "< 뒤로"
LangString LSTR_62 1042 제거
LangString LSTR_63 1042 "'제거' 버튼을 누르면 제거가 시작됩니다."
LangString LSTR_64 1042 "자세히 보기"
LangString LSTR_65 1042 완료
LangString LSTR_66 1042 "다음 >"
LangString LSTR_67 1042 "계속하시려면 '다음' 버튼을 눌러 주세요."
LangString LSTR_68 1042 " "
LangString LSTR_69 1042 닫음
LangString LSTR_70 1042 "Shinri Hub"


; LANG: 1041
LangString LSTR_0 1041 "Shinri Hub 26.11.23196"
LangString LSTR_1 1041 "$(LSTR_2) アンインストール"
LangString LSTR_2 1041 "Shinri Hub"
LangString LSTR_5 1041 作成できません：
LangString LSTR_8 1041 シンボルを見つけることができません：
LangString LSTR_9 1041 ロードすることができません：
LangString LSTR_10 1041 フォルダの作成：
LangString LSTR_12 1041 アンインストーラの作成：
LangString LSTR_13 1041 ファイルの削除：
LangString LSTR_14 1041 リブート時に削除：
LangString LSTR_16 1041 作成エラー：
LangString LSTR_17 1041 データの抽出エラー$\r$\n$\r$\nインストーラが破損しています。
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
LangString LSTR_36 1041 "初期ファイルの作成エラー：$\r$\n$\t$\"$0$\"$\r$\nインストールを中止するには中止を,$\r$\n再びこのファイルの作成を試みるには再試行を, また$\r$\nこのファイルをスキップして続けるには無視を押してください"
LangString LSTR_37 1041 0
LangString LSTR_38 1041 "$(LSTR_70) アンインストール ウィザードへようこそ"
LangString LSTR_39 1041 "ＭＳ Ｐゴシック"
LangString LSTR_40 1041 このウィザードは、$(LSTR_70)のアンインストールをガイドしていきます。$\r$\n$\r$\nアンインストールを開始する前に、$(LSTR_70)が起動していないことを確認して下さい。$\r$\n$\r$\n$_CLICK
LangString LSTR_41 1041 アンインストール
LangString LSTR_42 1041 $(LSTR_70)をアンインストールしています。しばらくお待ちください。
LangString LSTR_43 1041 アンインストールの完了
LangString LSTR_44 1041 アンインストールに成功しました。
LangString LSTR_45 1041 アンインストールの中止
LangString LSTR_46 1041 アンインストールは正常に完了されませんでした。
LangString LSTR_47 1041 完了(&F)
LangString LSTR_48 1041 "$(LSTR_70) アンインストール ウィザードは完了しました。"
LangString LSTR_49 1041 "$(LSTR_70) のアンインストールを完了するには、このコンピュータを再起動する必要があります。今すぐ再起動しますか？"
LangString LSTR_50 1041 今すぐ再起動する
LangString LSTR_51 1041 後で手動で再起動する
LangString LSTR_52 1041 "$(LSTR_70)は、このコンピュータからアンインストールされました。$\r$\n$\r$\nウィザードを閉じるには [完了] を押してください。"
LangString LSTR_53 1041 9
LangString LSTR_54 1041 "Shinri Hub が起動しています。$\r$\nOKをクリックして、閉じてください。$\r$\n閉じない場合は、手動で閉じてください。"
LangString LSTR_55 1041 "Closing running Shinri Hub..."
LangString LSTR_56 1041 "Shinri Hubが終了できません。$\r$\n手動で閉じて、『再試行』をクリックしてください。"
LangString LSTR_57 1041 "Windows 7 以降が必要です"
LangString LSTR_58 1041 "64 ビット バージョンの Windows が必要です"
LangString LSTR_59 1041 カスタム
LangString LSTR_60 1041 キャンセル
LangString LSTR_61 1041 "< 戻る(&B)"
LangString LSTR_62 1041 ｱﾝｲﾝｽﾄｰﾙ(&U)
LangString LSTR_63 1041 "アンインストールを始めるには [ｱﾝｲﾝｽﾄｰﾙ] をクリックして下さい。"
LangString LSTR_64 1041 詳細を表示(&D)
LangString LSTR_65 1041 完了
LangString LSTR_66 1041 "次へ(&N) >"
LangString LSTR_67 1041 "続けるには [次へ] をクリックして下さい。"
LangString LSTR_68 1041 " "
LangString LSTR_69 1041 閉じる(&C)
LangString LSTR_70 1041 "Shinri Hub"


; LANG: 1036
LangString LSTR_0 1036 "Shinri Hub 26.11.23196"
LangString LSTR_1 1036 "Désinstallation de $(LSTR_2)"
LangString LSTR_2 1036 "Shinri Hub"
LangString LSTR_5 1036 "Impossible d'écrire : "
LangString LSTR_8 1036 "Impossible de trouver un symbole : "
LangString LSTR_9 1036 "Impossible de charger : "
LangString LSTR_10 1036 "Création du dossier : "
LangString LSTR_12 1036 "Création de la désinstallation : "
LangString LSTR_13 1036 "Suppression : "
LangString LSTR_14 1036 "Suppression au redémarrage : "
LangString LSTR_16 1036 "Erreur de la création : "
LangString LSTR_17 1036 "Erreur lors de la décompression des données ! Installation corrompue ?"
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
LangString LSTR_36 1036 "Erreur lors de l'ouverture du fichier en écriture : $\r$\n$\t$\"$0$\"$\r$\nAppuyez sur Abandonner pour annuler l'installation,$\r$\nRéessayer pour réessayer l'écriture du fichier, ou$\r$\nIgnorer pour passer ce fichier"
LangString LSTR_37 1036 0
LangString LSTR_38 1036 "Bienvenue dans le programme de désinstallation de $(LSTR_70)"
LangString LSTR_39 1036 "MS Shell Dlg"
LangString LSTR_40 1036 "Vous êtes sur le point de désinstaller $(LSTR_70) de votre ordinateur.$\r$\n$\r$\nAvant d'amorcer la désinstallation, assurez-vous que $(LSTR_70) ne soit pas en cours d'exécution.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1036 "Désinstallation en cours"
LangString LSTR_42 1036 "Veuillez patienter pendant la désinstallation de $(LSTR_70)."
LangString LSTR_43 1036 "Désinstallation terminée"
LangString LSTR_44 1036 "La désinstallation s'est terminée avec succès."
LangString LSTR_45 1036 "Désinstallation interrompue"
LangString LSTR_46 1036 "La désinstallation n'a pas été terminée."
LangString LSTR_47 1036 &Fermer
LangString LSTR_48 1036 "Fin de la désinstallation de $(LSTR_70)"
LangString LSTR_49 1036 "Votre ordinateur doit être redémarré pour terminer la désinstallation de $(LSTR_70). Souhaitez-vous redémarrer maintenant ?"
LangString LSTR_50 1036 "Redémarrer maintenant"
LangString LSTR_51 1036 "Je souhaite redémarrer moi-même plus tard"
LangString LSTR_52 1036 "$(LSTR_70) a été supprimé de votre ordinateur.$\r$\n$\r$\nCliquez sur Fermer pour quitter le programme d'installation."
LangString LSTR_53 1036 8
LangString LSTR_54 1036 "Shinri Hub est en cours d’utilisation. $\r$\nCliquez sur «OK» pour fermer ce programme."
LangString LSTR_55 1036 "Closing running Shinri Hub..."
LangString LSTR_56 1036 "Shinri Hub ne peut pas être fermé. $\r$\nVeuillez la fermer manuellement et cliquez sur Réessayer pour continuer."
LangString LSTR_57 1036 "Windows 7 (ou une version ultérieure) est requis"
LangString LSTR_58 1036 "Windows 64 bits est requis"
LangString LSTR_59 1036 Personnalisée
LangString LSTR_60 1036 Annuler
LangString LSTR_61 1036 "< &Précédent"
LangString LSTR_62 1036 &Désinstaller
LangString LSTR_63 1036 "Cliquez sur Désinstaller pour démarrer la désinstallation."
LangString LSTR_64 1036 "P&lus d'infos"
LangString LSTR_65 1036 Terminé
LangString LSTR_66 1036 "&Suivant >"
LangString LSTR_67 1036 "Cliquez sur Suivant pour continuer."
LangString LSTR_68 1036 " "
LangString LSTR_69 1036 &Fermer
LangString LSTR_70 1036 "Shinri Hub"


; LANG: 1040
LangString LSTR_0 1040 "Shinri Hub 26.11.23196"
LangString LSTR_1 1040 "Disinstallazione di $(LSTR_2)"
LangString LSTR_2 1040 "Shinri Hub"
LangString LSTR_5 1040 "Impossibile scrivere: "
LangString LSTR_8 1040 "Impossibile trovare il simbolo: "
LangString LSTR_9 1040 "Impossibile caricare: "
LangString LSTR_10 1040 "Creazione cartella: "
LangString LSTR_12 1040 "Creazione programma disinstallazione: "
LangString LSTR_13 1040 "Eliminazione file: "
LangString LSTR_14 1040 "Elimina al riavvio: "
LangString LSTR_16 1040 "Errore nella creazione di: "
LangString LSTR_17 1040 "Errore nella decompressione dei dati! Probabile programma di installazione corrotto."
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
LangString LSTR_36 1040 "Errore nell'apertura del file per la scrittura: $\r$\n$\t$\"$0$\"$\r$\nSeleziona 'Termina' per interrompere l'installazione,$\r$\n'Riprova' per ritentare, oppure$\r$\n'Ignora' per saltare questo file."
LangString LSTR_37 1040 0
LangString LSTR_38 1040 "Disinstallazione di $(LSTR_70)"
LangString LSTR_39 1040 "MS Shell Dlg"
LangString LSTR_40 1040 "Questa programma ti guiderà nella disinstallazione di $(LSTR_70).$\r$\n$\r$\nPrima di iniziare la disinstallazione, assicurati che $(LSTR_2) non sia in esecuzione.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1040 Disinstallazione
LangString LSTR_42 1040 "Attendi il completamento della disinstallazione di $(LSTR_70)."
LangString LSTR_43 1040 "Disinstallazione completata"
LangString LSTR_44 1040 "La disinstallazione è stata completata correttamente."
LangString LSTR_45 1040 "Disinstallazione interrotta"
LangString LSTR_46 1040 "La disintallazione non è stata completata correttamente."
LangString LSTR_47 1040 &Chiudi
LangString LSTR_48 1040 "Disinstallazione di $(LSTR_70) completata."
LangString LSTR_49 1040 "Per completare la disinstallazione di $(LSTR_70) il computer deve essere riavviato. Vuoi riavviarlo ora?"
LangString LSTR_50 1040 "Riavvia ora"
LangString LSTR_51 1040 "Preferisco riavviarlo manualmente più tardi"
LangString LSTR_52 1040 "$(LSTR_70) è stato disinstallato.$\r$\n$\r$\nSeleziona 'Chiudi' per chiudere questa procedura."
LangString LSTR_53 1040 8
LangString LSTR_54 1040 "Shinri Hub è in esecuzione.$\r$\nClicca su OK per chiuderla.$\r$\nSe non si chiude, prova a chiuderla manualmente."
LangString LSTR_55 1040 "Closing running Shinri Hub..."
LangString LSTR_56 1040 "Shinri Hub non può essere chiuso. $\r$\nPer favore, chiudilo manualmente e clicca su Riprova per continuare."
LangString LSTR_57 1040 "È necessario disporre del sistema operativo Windows 7 e successivi"
LangString LSTR_58 1040 "È necessario disporre di un sistema operativo Windows a 64 bit"
LangString LSTR_59 1040 Personalizzata
LangString LSTR_60 1040 Annulla
LangString LSTR_61 1040 "< &Indietro"
LangString LSTR_62 1040 &Disinstalla
LangString LSTR_63 1040 "Per avviare la disinstallazione, seleziona 'Disinstalla'."
LangString LSTR_64 1040 "Visualizza &dettagli"
LangString LSTR_65 1040 "Installazione completata"
LangString LSTR_66 1040 "&Avanti >"
LangString LSTR_67 1040 "Per proseguire, seleziona 'Avanti'."
LangString LSTR_68 1040 " "
LangString LSTR_69 1040 &Fine
LangString LSTR_70 1040 "Shinri Hub"


; LANG: 1031
LangString LSTR_0 1031 "Shinri Hub 26.11.23196"
LangString LSTR_1 1031 "Deinstallation von $(LSTR_2)"
LangString LSTR_2 1031 "Shinri Hub"
LangString LSTR_5 1031 "Fehler beim Schreiben: "
LangString LSTR_8 1031 "Symbol ist nicht vorhanden: "
LangString LSTR_9 1031 "Fehler beim Laden von "
LangString LSTR_10 1031 "Verzeichnis wird erstellt: "
LangString LSTR_12 1031 "Deinstallations-Programm wird erstellt: "
LangString LSTR_13 1031 "Datei wird gelöscht: "
LangString LSTR_14 1031 "Datei wird nach Neustart gelöscht: "
LangString LSTR_16 1031 "Fehler beim Erstellen: "
LangString LSTR_17 1031 "Fehler beim Entpacken. Ist das Installations-Programm beschädigt?"
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
LangString LSTR_36 1031 "Fehler beim Überschreiben der Datei: $\r$\n$\t$\"$0$\"$\r$\nKlicken Sie auf Abbrechen, um abzubrechen,$\r$\nauf Wiederholen, um den Schreibvorgang erneut zu versuchen,$\r$\noder auf Ignorieren, um diese Datei zu überspringen."
LangString LSTR_37 1031 0
LangString LSTR_38 1031 "Willkommen beim Deinstallations-$\r$\nAssistenten für $(LSTR_70)"
LangString LSTR_39 1031 "MS Shell Dlg"
LangString LSTR_40 1031 "Dieser Assistent wird Sie durch die Deinstallation von $(LSTR_70) begleiten.$\r$\n$\r$\nBitte beenden Sie $(LSTR_70), bevor Sie mit der Deinstallation fortfahren.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1031 "Wird deinstalliert ..."
LangString LSTR_42 1031 "Bitte warten Sie, während $(LSTR_70) entfernt wird."
LangString LSTR_43 1031 "Die Deinstallation ist vollständig"
LangString LSTR_44 1031 "Die Deinstallation wurde erfolgreich abgeschlossen."
LangString LSTR_45 1031 "Abbruch der Deinstallation"
LangString LSTR_46 1031 "Die Deinstallation wurde nicht vollständig abgeschlossen."
LangString LSTR_47 1031 &Fertigstellen
LangString LSTR_48 1031 "Die Deinstallation von $(LSTR_70) wird abgeschlossen"
LangString LSTR_49 1031 "Windows muss neu gestartet werden, um die Deinstallation von $(LSTR_70) zu vervollständigen. Möchten Sie Windows jetzt neu starten?"
LangString LSTR_50 1031 "Jetzt neu starten"
LangString LSTR_51 1031 "Windows später selbst neu starten"
LangString LSTR_52 1031 "$(LSTR_70) ist von Ihrem Computer entfernt worden.$\r$\n$\r$\nKlicken Sie auf Fertigstellen, um den Assistenten zu schließen."
LangString LSTR_53 1031 8
LangString LSTR_54 1031 "Shinri Hub wird gerade ausgeführt.$\r$\nKlicke auf »OK«, um es zu schließen.$\r$\nFalls es sich nicht schließen lässt, versuche es manuell zu schließen."
LangString LSTR_55 1031 "Closing running Shinri Hub..."
LangString LSTR_56 1031 "Shinri Hub kann nicht geschlossen werden. $\r$\nSchließ es bitte manuell und klicke auf »Wiederholen«, um fortzufahren."
LangString LSTR_57 1031 "Windows 7 oder höher wird vorausgesetzt"
LangString LSTR_58 1031 "64-Bit-Windows wird vorausgesetzt"
LangString LSTR_59 1031 Benutzerdefiniert
LangString LSTR_60 1031 Abbrechen
LangString LSTR_61 1031 "< &Zurück"
LangString LSTR_62 1031 &Deinstallieren
LangString LSTR_63 1031 "Klicken Sie auf Deinstallieren, um die Deinstallation zu starten."
LangString LSTR_64 1031 "&Details anzeigen"
LangString LSTR_65 1031 Fertig
LangString LSTR_66 1031 "&Weiter >"
LangString LSTR_67 1031 "Klicken Sie auf Weiter, um fortzufahren."
LangString LSTR_68 1031 " "
LangString LSTR_69 1031 &Beenden
LangString LSTR_70 1031 "Shinri Hub"


; LANG: 1033
LangString LSTR_0 1033 "Shinri Hub 26.11.23196"
LangString LSTR_1 1033 "$(LSTR_2) Uninstall"
LangString LSTR_2 1033 "Shinri Hub"
LangString LSTR_5 1033 "Can't write: "
LangString LSTR_8 1033 "Could not find symbol: "
LangString LSTR_9 1033 "Could not load: "
LangString LSTR_10 1033 "Create folder: "
LangString LSTR_12 1033 "Created uninstaller: "
LangString LSTR_13 1033 "Delete file: "
LangString LSTR_14 1033 "Delete on reboot: "
LangString LSTR_16 1033 "Error creating: "
LangString LSTR_17 1033 "Error decompressing data! Corrupted installer?"
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
LangString LSTR_36 1033 "Error opening file for writing: $\r$\n$\r$\n$0$\r$\n$\r$\nClick Abort to stop the installation,$\r$\nRetry to try again, or$\r$\nIgnore to skip this file."
LangString LSTR_37 1033 0
LangString LSTR_38 1033 "Welcome to $(LSTR_70) Uninstall"
LangString LSTR_39 1033 "MS Shell Dlg"
LangString LSTR_40 1033 "Setup will guide you through the uninstallation of $(LSTR_70).$\r$\n$\r$\nBefore starting the uninstallation, make sure $(LSTR_70) is not running.$\r$\n$\r$\n$_CLICK"
LangString LSTR_41 1033 Uninstalling
LangString LSTR_42 1033 "Please wait while $(LSTR_70) is being uninstalled."
LangString LSTR_43 1033 "Uninstallation Complete"
LangString LSTR_44 1033 "Uninstall was completed successfully."
LangString LSTR_45 1033 "Uninstallation Aborted"
LangString LSTR_46 1033 "Uninstall was not completed successfully."
LangString LSTR_47 1033 &Finish
LangString LSTR_48 1033 "Completing $(LSTR_70) Uninstall"
LangString LSTR_49 1033 "Your computer must be restarted in order to complete the uninstallation of $(LSTR_70). Do you want to reboot now?"
LangString LSTR_50 1033 "Reboot now"
LangString LSTR_51 1033 "I want to manually reboot later"
LangString LSTR_52 1033 "$(LSTR_70) has been uninstalled from your computer.$\r$\n$\r$\nClick Finish to close Setup."
LangString LSTR_53 1033 8
LangString LSTR_54 1033 "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
LangString LSTR_55 1033 "Closing running Shinri Hub..."
LangString LSTR_56 1033 "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
LangString LSTR_57 1033 "Windows 7 and above is required"
LangString LSTR_58 1033 "64-bit Windows is required"
LangString LSTR_59 1033 Custom
LangString LSTR_60 1033 Cancel
LangString LSTR_61 1033 "< &Back"
LangString LSTR_62 1033 &Uninstall
LangString LSTR_63 1033 "Click Uninstall to start the uninstallation."
LangString LSTR_64 1033 "Show &details"
LangString LSTR_65 1033 Completed
LangString LSTR_66 1033 "&Next >"
LangString LSTR_67 1033 "Click Next to continue."
LangString LSTR_68 1033 " "
LangString LSTR_69 1033 &Close
LangString LSTR_70 1033 "Shinri Hub"


; --------------------
; VARIABLES: 48

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


InstType un.$(LSTR_59)    ;  Custom
; uninstchild = $TEMP\Un_$1.exe
; uninstcmd = "$\"$TEMP\Un_$1.exe$\" $0 _?=$INSTDIR\"
; wininit = $WINDIR\wininit.ini


; --------------------
; PAGES: 4

; Page 0
UninstPage custom func_144 func_239 /ENABLECANCEL

; Page 1
UninstPage instfiles func_240 func_243 func_249
  CompletedText $(LSTR_65)    ;  Completed
  DetailsButtonText $(LSTR_64)    ;  "Show &details"

/*
; Page 2
UninstPage COMPLETED
*/

; Page 3
UninstPage custom func_262 func_459


; --------------------
; SECTIONS: 1
; COMMANDS: 1149

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
  InitPluginsDir
    ; Call Initialize_____Plugins
    ; SetDetailsPrint lastused
  SetOverwrite on
  AllowSkipFiles on
  File $PLUGINSDIR\modern-wizard.bmp
FunctionEnd


Function func_124
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


Function func_134
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


Function func_144    ; Page 0, Pre
  nsDialogs::Create 1044
    ; Call Initialize_____Plugins
    ; SetOverwrite off
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push 1044
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll Create
  Pop $_21_
  nsDialogs::SetRTL $(LSTR_37)    ;  0
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_37)    ;  0
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll SetRTL
  SetCtlColors $_21_ "" 0xFFFFFF
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
  Pop $_22_
  Push $0
  Push $R0
  Push $PLUGINSDIR\modern-wizard.bmp
  StrCpy $R0 $_22_
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
  Pop $_23_
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 10u 195u 28u $(LSTR_38)    ;  "Welcome to $(LSTR_70) Uninstall" "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_38)    ;  "Welcome to $(LSTR_70) Uninstall" "Shinri Hub"
    ; Push 28u
    ; Push 195u
    ; Push 10u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_24_
  SetCtlColors $_24_ 0x000000 0xFFFFFF
  CreateFont $_25_ $(LSTR_39) 12 700    ;  "MS Shell Dlg"
  SendMessage $_24_ ${WM_SETFONT} $_25_ 0
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 45u 195u 130u $(LSTR_40)    ;  "Setup will guide you through the uninstallation of $(LSTR_70).$\r$\n$\r$\nBefore starting the uninstallation, make sure $(LSTR_70) is not running.$\r$\n$\r$\n$_CLICK" "Shinri Hub" "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_40)    ;  "Setup will guide you through the uninstallation of $(LSTR_70).$\r$\n$\r$\nBefore starting the uninstallation, make sure $(LSTR_70) is not running.$\r$\n$\r$\n$_CLICK" "Shinri Hub" "Shinri Hub"
    ; Push 130u
    ; Push 195u
    ; Push 45u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_26_
  SetCtlColors $_26_ 0x000000 0xFFFFFF
  Call func_124
  nsDialogs::Show
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll Show
  Call func_134
  IntCmp $_23_ 0 label_238
  System::Call gdi32::DeleteObject(ps) $_23_
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push $_23_
    ; Push gdi32::DeleteObject(ps)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_238:
FunctionEnd


Function func_239    ; Page 0, Leave
FunctionEnd


Function func_240    ; Page 1, Pre
  SendMessage $_6_ ${WM_SETTEXT} 0 STR:$(LSTR_41)    ;  Uninstalling
  SendMessage $_8_ ${WM_SETTEXT} 0 STR:$(LSTR_42)    ;  "Please wait while $(LSTR_70) is being uninstalled." "Shinri Hub"
FunctionEnd


Function func_243    ; Page 1, Show
  FindWindow $_27_ "#32770" "" $HWNDPARENT
  GetDlgItem $_28_ $_27_ 1006
  GetDlgItem $_29_ $_27_ 1004
  GetDlgItem $_30_ $_27_ 1027
  GetDlgItem $_31_ $_27_ 1016
FunctionEnd


Function func_249    ; Page 1, Leave
  IfAbort label_253
  SendMessage $_6_ ${WM_SETTEXT} 0 STR:$(LSTR_43)    ;  "Uninstallation Complete"
  SendMessage $_8_ ${WM_SETTEXT} 0 STR:$(LSTR_44)    ;  "Uninstall was completed successfully."
  Goto label_255
label_253:
  SendMessage $_6_ ${WM_SETTEXT} 0 STR:$(LSTR_45)    ;  "Uninstallation Aborted"
  SendMessage $_8_ ${WM_SETTEXT} 0 STR:$(LSTR_46)    ;  "Uninstall was not completed successfully."
label_255:
FunctionEnd


Function func_256
  InitPluginsDir
    ; Call Initialize_____Plugins
    ; SetDetailsPrint lastused
  SetOverwrite on
  AllowSkipFiles on
  File $PLUGINSDIR\modern-wizard.bmp
  Call func_120
  SetAutoClose true
FunctionEnd


Function func_262    ; Page 3, Pre
  SendMessage $_15_ ${WM_SETTEXT} 0 STR:$(LSTR_47)    ;  &Finish
  nsDialogs::Create 1044
    ; Call Initialize_____Plugins
    ; SetOverwrite off
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push 1044
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll Create
  Pop $_32_
  nsDialogs::SetRTL $(LSTR_37)    ;  0
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_37)    ;  0
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll SetRTL
  SetCtlColors $_32_ "" 0xFFFFFF
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
  Pop $_33_
  Push $0
  Push $R0
  Push $PLUGINSDIR\modern-wizard.bmp
  StrCpy $R0 $_33_
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
  Pop $_34_
  IfRebootFlag 0 label_380
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 10u 195u 28u $(LSTR_48)    ;  "Completing $(LSTR_70) Uninstall" "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_48)    ;  "Completing $(LSTR_70) Uninstall" "Shinri Hub"
    ; Push 28u
    ; Push 195u
    ; Push 10u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_35_
  SetCtlColors $_35_ 0x000000 0xFFFFFF
  CreateFont $_36_ $(LSTR_39) 12 700    ;  "MS Shell Dlg"
  SendMessage $_35_ ${WM_SETFONT} $_36_ 0
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 45u 195u 40u $(LSTR_49)    ;  "Your computer must be restarted in order to complete the uninstallation of $(LSTR_70). Do you want to reboot now?" "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_49)    ;  "Your computer must be restarted in order to complete the uninstallation of $(LSTR_70). Do you want to reboot now?" "Shinri Hub"
    ; Push 40u
    ; Push 195u
    ; Push 45u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_37_
  SetCtlColors $_37_ 0x000000 0xFFFFFF
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
  Pop $_39_
  SetCtlColors $_39_ 0x000000 0xFFFFFF
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
  Pop $_40_
  SetCtlColors $_40_ 0x000000 0xFFFFFF
  SendMessage $_39_ 0x00F1 1 0
  System::Call user32::SetFocus(p$_39_)
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push user32::SetFocus(p$_39_)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  Goto label_410
label_380:
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 10u 195u 28u $(LSTR_48)    ;  "Completing $(LSTR_70) Uninstall" "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_48)    ;  "Completing $(LSTR_70) Uninstall" "Shinri Hub"
    ; Push 28u
    ; Push 195u
    ; Push 10u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_35_
  SetCtlColors $_35_ 0x000000 0xFFFFFF
  CreateFont $_36_ $(LSTR_39) 12 700    ;  "MS Shell Dlg"
  SendMessage $_35_ ${WM_SETFONT} $_36_ 0
  nsDialogs::CreateControl STATIC 0x40000000|0x10000000|0x04000000|0x00000100 0x00000020 120u 45u 195u 130u $(LSTR_52)    ;  "$(LSTR_70) has been uninstalled from your computer.$\r$\n$\r$\nClick Finish to close Setup." "Shinri Hub"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; Push $(LSTR_52)    ;  "$(LSTR_70) has been uninstalled from your computer.$\r$\n$\r$\nClick Finish to close Setup." "Shinri Hub"
    ; Push 130u
    ; Push 195u
    ; Push 45u
    ; Push 120u
    ; Push 0x00000020
    ; Push 0x40000000|0x10000000|0x04000000|0x00000100
    ; Push STATIC
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll CreateControl
  Pop $_37_
  SetCtlColors $_37_ 0x000000 0xFFFFFF
label_410:
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
  Pop $_41_
  System::Free $1
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push $1
    ; CallInstDLL $PLUGINSDIR\System.dll Free
  Pop $1
  IntOp $_41_ $_41_ & 0x01
  IntCmp $_41_ 0 label_445
  System::Call "UXTHEME::SetWindowTheme(p$_39_,w$\" $\",w$\" $\")"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "UXTHEME::SetWindowTheme(p$_39_,w$\" $\",w$\" $\")"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  System::Call "UXTHEME::SetWindowTheme(p$_40_,w$\" $\",w$\" $\")"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "UXTHEME::SetWindowTheme(p$_40_,w$\" $\",w$\" $\")"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_445:
  Call func_124
  nsDialogs::Show
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsDialogs.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\nsDialogs.dll Show
  Call func_134
  IntCmp $_34_ 0 label_458
  System::Call gdi32::DeleteObject(ps) $_34_
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push $_34_
    ; Push gdi32::DeleteObject(ps)
    ; CallInstDLL $PLUGINSDIR\System.dll Call
label_458:
FunctionEnd


Function func_459    ; Page 3, Leave
  IfRebootFlag 0 label_466
  SendMessage $_39_ 0x00F0 0 0 $_38_
  IntCmp $_38_ 1 0 label_465 label_465
  Reboot
    ; Quit
  Goto label_466
label_465:
  Return

label_466:
FunctionEnd


Function .onGUIInit
  GetDlgItem $_6_ $HWNDPARENT 1037
  CreateFont $_7_ $(LSTR_39) $(LSTR_53) 700    ;  "MS Shell Dlg" 8
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
  Call func_256
FunctionEnd


Function .onUserAbort
FunctionEnd


Function func_490
  StrCpy $_42_ $SYSDIR\cmd.exe
  StrCpy $_43_ $SYSDIR\WindowsPowerShell\v1.0\powershell.exe
  nsExec::Exec "$\"$_43_$\" -C $\"if (Get-Command Get-CimInstance -ErrorAction SilentlyContinue) { exit 0 } else { exit 1 }$\""
    ; Call Initialize_____Plugins
    ; AllowSkipFiles on
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_43_$\" -C $\"if (Get-Command Get-CimInstance -ErrorAction SilentlyContinue) { exit 0 } else { exit 1 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $0
  StrCmp $0 0 0 label_505
  nsExec::Exec "$\"$_43_$\" -C $\"if ((Get-ExecutionPolicy -Scope Process) -eq 'Restricted') { exit 1 } else { exit 0 }$\""
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_43_$\" -C $\"if ((Get-ExecutionPolicy -Scope Process) -eq 'Restricted') { exit 1 } else { exit 0 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $0
label_505:
  StrCmp $0 0 label_507
  StrCpy $0 1
label_507:
  StrCpy $_44_ $0
  Push 0
  Call func_0
  Pop $3
  Pop $2
  Pop $1
  Pop $4
  Pop $_20_
  StrCmp $3 "Shinri Hub.exe" label_638
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; AllowSkipFiles on
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_524
  Sleep 300
label_524:
  StrCmp $_44_ 0 0 label_532
  nsExec::Exec "$\"$_43_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_43_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
  Goto label_538
label_532:
  nsExec::Exec "$\"$_42_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_42_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
label_538:
  StrCmp $R0 0 0 label_638
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_548
  Sleep 1000
  Goto label_550
label_548:
  MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION $(LSTR_54) /SD IDOK IDOK label_550    ;  "Shinri Hub is running.$\r$\nClick OK to close it.$\r$\nIf it doesn't close, try closing it manually."
  Quit
label_550:
  DetailPrint $(LSTR_55)    ;  "Closing running Shinri Hub..."
  Push $0
  StrCmp 0 1 0 label_558
  StrCmp $_44_ 0 0 label_556
  StrCpy $0 -Force
  Goto label_557
label_556:
  StrCpy $0 /F
label_557:
  Goto label_559
label_558:
  StrCpy $0 ""
label_559:
  StrCmp $_44_ 0 0 label_566
  nsExec::Exec "$\"$_43_$\" -C $\"Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')} | % { Stop-Process -Id $$_.ProcessId $0 }$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_43_$\" -C $\"Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')} | % { Stop-Process -Id $$_.ProcessId $0 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Goto label_571
label_566:
  nsExec::Exec "taskkill /IM $\"Shinri Hub.exe$\" /FI $\"PID ne $_20_$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "taskkill /IM $\"Shinri Hub.exe$\" /FI $\"PID ne $_20_$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
label_571:
  Pop $0
  Sleep 300
  StrCpy $R1 0
label_574:
  IntOp $R1 $R1 + 1
  StrCmp $_44_ 0 0 label_583
  nsExec::Exec "$\"$_43_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_43_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
  Goto label_589
label_583:
  nsExec::Exec "$\"$_42_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_42_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
label_589:
  StrCmp $R0 0 0 label_632
  Sleep 1000
  Push $0
  StrCmp 1 1 0 label_598
  StrCmp $_44_ 0 0 label_596
  StrCpy $0 -Force
  Goto label_597
label_596:
  StrCpy $0 /F
label_597:
  Goto label_599
label_598:
  StrCpy $0 ""
label_599:
  StrCmp $_44_ 0 0 label_606
  nsExec::Exec "$\"$_43_$\" -C $\"Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')} | % { Stop-Process -Id $$_.ProcessId $0 }$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_43_$\" -C $\"Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')} | % { Stop-Process -Id $$_.ProcessId $0 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Goto label_611
label_606:
  nsExec::Exec "taskkill /IM $\"Shinri Hub.exe$\" /FI $\"PID ne $_20_$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "taskkill /IM $\"Shinri Hub.exe$\" /FI $\"PID ne $_20_$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
label_611:
  Pop $0
  StrCmp $_44_ 0 0 label_620
  nsExec::Exec "$\"$_43_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_43_$\" -C $\"if ((Get-CimInstance -ClassName Win32_Process | ? {$$_.Path -and $$_.Path.StartsWith('$INSTDIR', 'CurrentCultureIgnoreCase')}).Count -gt 0) { exit 0 } else { exit 1 }$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
  Goto label_626
label_620:
  nsExec::Exec "$\"$_42_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\nsExec.dll
    ; SetDetailsPrint lastused
    ; Push "$\"$_42_$\" /C tasklist /FI $\"IMAGENAME eq Shinri Hub.exe$\" /FO CSV /NH | $\"$SYSDIR\findstr.exe$\" /B /I /C:$\"\$\"Shinri Hub.exe\$\"$\""
    ; CallInstDLL $PLUGINSDIR\nsExec.dll Exec
  Pop $R0
label_626:
  StrCmp $R0 0 0 label_630
  DetailPrint "Waiting for $\"Shinri Hub$\" to close."
  Sleep 2000
  Goto label_631
label_630:
  Goto label_638
label_631:
  Goto label_633
label_632:
  Goto label_638
label_633:
  IntCmp $R1 1 label_637 label_637
  MessageBox MB_RETRYCANCEL|MB_ICONEXCLAMATION $(LSTR_56) /SD IDCANCEL IDRETRY label_574    ;  "Shinri Hub cannot be closed. $\r$\nPlease close it manually and click Retry to continue."
  Quit
  Goto label_638
label_637:
  Goto label_574
label_638:
FunctionEnd


Function .onInit
  SetOutPath $INSTDIR
  Call :label_642
  Goto label_730
label_642:
  StrCmp $_45_ "" label_644
  Return

label_644:
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
  IntCmp $3 0 0 label_675 label_675
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
label_675:
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
  IntCmp $0 2 0 label_691 label_691
  IntOp $_46_ $_46_ | 0x80000000
  IntOp $_45_ $_45_ | 0x80000000
label_691:
  IntCmp $R0 284 0 label_701 label_701
  Pop $0
  Exch
  Pop $0
  IntCmp $0 1 label_699
  IntOp $_46_ $_46_ | 0x40000000
  IntCmp $0 2 0 label_699 label_699
  IntOp $_46_ $_46_ | 0x20000000
label_699:
  Pop $0
  Goto label_709
label_701:
  Pop $0
  Pop $R0
  Pop $R0
  StrCpy $R0 $0 13
  StrCmp $R0 "Service Pack " 0 label_708
  StrCpy $0 $0 "" 13
  Goto label_709
label_708:
  StrCpy $0 0
label_709:
  IntOp $0 $0 << 16
  IntOp $_46_ $_46_ | $0
  IntOp $0 $_46_ & 0x40000000
  IntCmp $0 0 0 label_716 label_716
  IntCmp $1 5 0 label_716 label_716
  IntCmp $2 2 0 label_716 label_716
  StrCpy $2 1
label_716:
  IntCmp $0 0 label_719
  IntCmp 6 $1 0 0 label_719
  IntOp $_45_ $_45_ | 0x00000001
label_719:
  IntOp $1 $1 << 24
  IntOp $_45_ $_45_ | $1
  IntOp $0 $2 << 16
  IntOp $_45_ $_45_ | $0
  IntOp $_46_ $_46_ | $3
  Pop $R0
  Pop $3
  Pop $2
  Pop $1
  Pop $0
  Return

label_730:
  IntCmp $_45_ 0x85000000 0 label_732 label_732
  Goto label_734
label_732:
  Call :label_642
  IntCmp $_45_ 0x045a0000 0 label_735 label_735
label_734:
  Goto label_737
label_735:
  Call :label_642
  IntCmp $_45_ 0x85010000 0 label_738 label_738
label_737:
  Goto label_740
label_738:
  Call :label_642
  IntCmp $_45_ 0x86000000 0 label_742 label_742
label_740:
  MessageBox MB_OK $(LSTR_57)    ;  "Windows 7 and above is required"
  Quit
label_742:
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
  Pop $_41_
  StrCmp $_41_ 0 label_766
  SetRegView 64
  Goto label_768
label_766:
  MessageBox MB_OK|MB_ICONEXCLAMATION $(LSTR_58)    ;  "64-bit Windows is required"
  Quit
label_768:
  Call :label_770
  Goto label_803
label_770:
  StrCmp $CMDLINE "" 0 label_773
  Push ""
  Return

label_773:
  Push $0
  Push $1
  Push $2
  Push $3
  StrLen $1 $CMDLINE
  StrCpy $2 2
  StrCpy $3 $CMDLINE 1
  StrCmp $3 $\" label_782
  StrCpy $3 " "
label_782:
  IntCmp $2 $1 label_786 0 label_786
  StrCpy $0 $CMDLINE 1 $2
  IntOp $2 $2 + 1
  StrCmp $3 $0 0 label_782
label_786:
  IntCmp $2 $1 label_791 0 label_791
  StrCpy $0 $CMDLINE 1 $2
  StrCmp $0 " " 0 label_791
  IntOp $2 $2 + 1
  Goto label_786
label_791:
  StrCpy $0 $CMDLINE "" $2
label_792:
  StrCpy $1 $0 1 -1
  StrCmp $1 " " 0 label_796
  StrCpy $0 $0 -1
  Goto label_792
label_796:
  Pop $3
  Pop $2
  Pop $1
  Exch $0
    ; Push $0
    ; Exch
    ; Pop $0
  Return

label_803:
  Pop $R0
  Push $R0
  Push /S
  Call :label_808
  Goto label_897
label_808:
  Exch $1
    ; Push $1
    ; Exch
    ; Pop $1
  Exch
  Exch $0
    ; Push $0
    ; Exch
    ; Pop $0
  Exch
  Push $2
  Push $3
  Push $4
  Push $5
  Push $6
  Push $7
  ClearErrors
  StrCpy $2 $1 "" 1
  StrCpy $1 $1 1
  StrLen $3 $2
  StrCpy $7 0
label_827:
  StrCpy $4 -1
  StrCpy $6 ""
label_829:
  IntOp $4 $4 + 1
  StrCpy $5 $0 1 $4
  StrCmp $5$7 0 label_884
  StrCmp $5 "" label_871
  StrCmp $5 $\" 0 label_840
  StrCmp $6 "" 0 label_837
  StrCpy $6 $\"
  Goto label_829
label_837:
  StrCmp $6 $\" 0 label_840
  StrCpy $6 ""
  Goto label_829
label_840:
  StrCmp $5 ' 0 label_847
  StrCmp $6 "" 0 label_844
  StrCpy $6 '
  Goto label_829
label_844:
  StrCmp $6 ' 0 label_847
  StrCpy $6 ""
  Goto label_829
label_847:
  StrCmp $5 ` 0 label_854
  StrCmp $6 "" 0 label_851
  StrCpy $6 `
  Goto label_829
label_851:
  StrCmp $6 ` 0 label_854
  StrCpy $6 ""
  Goto label_829
label_854:
  StrCmp $6 $\" label_829
  StrCmp $6 ' label_829
  StrCmp $6 ` label_829
  StrCmp $5 $1 0 label_829
  StrCmp $7 0 label_859 label_871
label_859:
  IntOp $4 $4 + 1
  StrCpy $5 $0 $3 $4
  StrCmp $5 "" label_884
  StrCmp $5 $2 0 label_829
  IntOp $4 $4 + $3
  StrCpy $0 $0 "" $4
label_865:
  StrCpy $4 $0 1
  StrCmp $4 " " 0 label_869
  StrCpy $0 $0 "" 1
  Goto label_865
label_869:
  StrCpy $7 1
  Goto label_827
label_871:
  StrCpy $0 $0 $4
label_872:
  StrCpy $4 $0 1 -1
  StrCmp $4 " " 0 label_876
  StrCpy $0 $0 -1
  Goto label_872
label_876:
  StrCpy $3 $0 1
  StrCpy $4 $0 1 -1
  StrCmp $3 $4 0 label_886
  StrCmp $3 $\" label_882
  StrCmp $3 ' label_882
  StrCmp $3 ` 0 label_886
label_882:
  StrCpy $0 $0 -1 1
  Goto label_886
label_884:
  SetErrors
  StrCpy $0 ""
label_886:
  Pop $7
  Pop $6
  Pop $5
  Pop $4
  Pop $3
  Pop $2
  Pop $1
  Exch $0
    ; Push $0
    ; Exch
    ; Pop $0
  Return

label_897:
  Pop $R1
  IfErrors label_900
  SetSilent silent
label_900:
  IfSilent 0 label_903
  Call func_490
  Goto label_903
label_903:
  StrCpy $_18_ all
  SetShellVarContext all
  UAC::_ 2s
    ; Call Initialize_____Plugins
    ; AllowSkipFiles on
    ; File $PLUGINSDIR\UAC.dll
    ; SetDetailsPrint lastused
    ; Push 2s
    ; CallInstDLL $PLUGINSDIR\UAC.dll _
  Pop $_41_
  StrCmp $_41_ 0 0 label_919
  ShowWindow $HWNDPARENT ${SW_HIDE}
  UAC::_ 0
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\UAC.dll
    ; SetDetailsPrint lastused
    ; Push 0
    ; CallInstDLL $PLUGINSDIR\UAC.dll _
  Quit
label_919:
  ReadRegStr $_19_ HKLM Software\2546e024-4dd1-5359-9177-3b6a84280dd8 InstallLocation
  StrCmp $_19_ "" label_923
  StrCpy $INSTDIR $_19_
  Goto label_948
label_923:
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
  Pop $_41_
  StrCmp $_41_ 0 label_947
  StrCpy $0 $PROGRAMFILES64
label_947:
  StrCpy $INSTDIR "$0\Shinri Hub"
label_948:
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
label_963:
  StrCpy $R5 $R8 3 $R6
  StrCmp $R5 /D= 0 label_966
  Goto label_967
label_966:
  StrCmp $R5 /d= 0 label_970
label_967:
  IntOp $R6 $R6 + 3
  StrCpy $R9 $R8 "" $R6
  Goto label_974
label_970:
  IntOp $R6 $R6 + 1
label_971:
  IntCmp $R6 $R7 label_963 label_963 label_974
  Goto label_971
  Goto label_974
label_974:
  StrCpy $R0 $R9
  Pop $R5
  Pop $R6
  Pop $R7
  Pop $R9
  Pop $R8
  StrCmp $R0 "" label_982
  StrCpy $INSTDIR $R0
label_982:
FunctionEnd


Function func_983
  Exch $R0
    ; Push $R0
    ; Exch
    ; Pop $R0
  Push $R1
  Push $R2
  Push $R3
  StrCpy $R3 $INSTDIR$R0\*.*
  FindFirst $R1 $R2 $R3
label_991:
  StrCmp $R2 "" label_1011
  StrCmp $R2 . label_1009
  StrCmp $R2 .. label_1009
  IfFileExists $INSTDIR$R0\$R2\*.* label_995 label_1002
label_995:
  CreateDirectory $PLUGINSDIR\old-install$R0\$R2
  Push $R0\$R2
  Call func_983
  Pop $R3
  StrCmp $R3 0 label_1001
  Goto label_1012
label_1001:
  Goto label_1009
label_1002:
  ClearErrors
  Rename $INSTDIR$R0\$R2 $PLUGINSDIR\old-install$R0\$R2 ;  $INSTDIR$R0\$R2->$PLUGINSDIR\old-install$R0\$R2
  StrCmp $R0\$R2 "Uninstall Shinri Hub.exe" 0 label_1006
  ClearErrors
label_1006:
  IfErrors 0 label_1009
  StrCpy $R3 $INSTDIR$R0\$R2
  Goto label_1012
label_1009:
  FindNext $R1 $R2
  Goto label_991
label_1011:
  StrCpy $R3 0
label_1012:
  FindClose $R1
  StrCpy $R0 $R3
  Pop $R3
  Pop $R2
  Pop $R1
  Exch $R0
    ; Push $R0
    ; Exch
    ; Pop $R0
FunctionEnd


Function func_1021
  Exch $R0
    ; Push $R0
    ; Exch
    ; Pop $R0
  Push $R1
  Push $R2
  Push $R3
  StrCpy $R3 $PLUGINSDIR\old-install$R0\*.*
  FindFirst $R1 $R2 $R3
label_1029:
  StrCmp $R2 "" label_1041
  StrCmp $R2 . label_1039
  StrCmp $R2 .. label_1039
  IfFileExists $INSTDIR$R0\$R2\*.* label_1033 label_1038
label_1033:
  CreateDirectory $INSTDIR$R0\$R2
  Push $R0\$R2
  Call func_1021
  Pop $R3
  Goto label_1039
label_1038:
  Rename $PLUGINSDIR\old-install$R0\$R2 $INSTDIR$R0\$R2 ;  $PLUGINSDIR\old-install$R0\$R2->$INSTDIR$R0\$R2
label_1039:
  FindNext $R1 $R2
  Goto label_1029
label_1041:
  StrCpy $R0 0
  FindClose $R1
  Pop $R3
  Pop $R2
  Pop $R1
  Exch $R0
    ; Push $R0
    ; Exch
    ; Pop $R0
FunctionEnd


Section Uninstall ; Section_0
  SectionIn 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 RO
  IfSilent label_1052
  Call func_490
label_1052:
  ReadRegStr $_4_ SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 ShortcutName
  StrCmp $_4_ "" 0 label_1055
  StrCpy $_4_ "Shinri Hub"
label_1055:
  StrCpy $_3_ $DESKTOP\$_4_.lnk
  StrCpy $_2_ "$DESKTOP\Shinri Hub.lnk"
  ReadRegStr $_5_ SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 MenuDirectory
  StrCmp $_5_ "" 0 label_1061
  StrCpy $_1_ $SMPROGRAMS\$_4_.lnk
  Goto label_1062
label_1061:
  StrCpy $_1_ $SMPROGRAMS\$_5_\$_4_.lnk
label_1062:
  StrCpy $_0_ "$SMPROGRAMS\Shinri Hub.lnk"
  Push updated
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true 0 label_1080
  CreateDirectory $PLUGINSDIR\old-install
  Push ""
  Call func_983
  Pop $R0
  StrCmp $R0 0 label_1080
  DetailPrint "File is busy, aborting: $R0"
  Push ""
  Call func_1021
  Pop $R0
  Abort "Can't rename $\"$INSTDIR$\" to $\"$PLUGINSDIR\old-install$\"."
label_1080:
  SetOutPath $TEMP
  RMDir /r $INSTDIR
  Push keep-shortcuts
  StdUtils::TestParameter /NOUNLOAD
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\StdUtils.dll
    ; SetDetailsPrint lastused
    ; CallInstDLL $PLUGINSDIR\StdUtils.dll /NOUNLOAD TestParameter
  Pop $R9
  StrCmp $R9 true label_1109
  WinShell::UninstAppUserModelId stred.shinritrial.hub
    ; Call Initialize_____Plugins
    ; AllowSkipFiles on
    ; File $PLUGINSDIR\WinShell.dll
    ; SetDetailsPrint lastused
    ; Push stred.shinritrial.hub
    ; CallInstDLL $PLUGINSDIR\WinShell.dll UninstAppUserModelId
  WinShell::UninstShortcut $_3_
    ; Call Initialize_____Plugins
    ; AllowSkipFiles off
    ; File $PLUGINSDIR\WinShell.dll
    ; SetDetailsPrint lastused
    ; Push $_3_
    ; CallInstDLL $PLUGINSDIR\WinShell.dll UninstShortcut
  Delete $_3_
  WinShell::UninstShortcut $_1_
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\WinShell.dll
    ; SetDetailsPrint lastused
    ; Push $_1_
    ; CallInstDLL $PLUGINSDIR\WinShell.dll UninstShortcut
  Delete $_1_
  ReadRegStr $R1 SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8 MenuDirectory
  StrCmp $R1 "" label_1109
  RMDir $SMPROGRAMS\$R1
label_1109:
  System::Call "shell32::SHChangeNotify(i, i, i, i) v (0x08000000, 0, 0, 0)"
    ; Call Initialize_____Plugins
    ; File $PLUGINSDIR\System.dll
    ; SetDetailsPrint lastused
    ; Push "shell32::SHChangeNotify(i, i, i, i) v (0x08000000, 0, 0, 0)"
    ; CallInstDLL $PLUGINSDIR\System.dll Call
  StrCpy $_47_ 0
  ClearErrors
  Call :label_770
  Pop $R0
  Push $R0
  Push --delete-app-data
  Call :label_808
  Pop $R1
  IfErrors 0 label_1124
  Goto label_1125
label_1124:
  StrCpy $_47_ 1
label_1125:
  StrCmp $_47_ 1 0 label_1132
  StrCmp $_18_ all 0 label_1128
  SetShellVarContext current
label_1128:
  RMDir /r "$APPDATA\Shinri Hub"
  RMDir /r $APPDATA\st-hub
  StrCmp $_18_ all 0 label_1132
  SetShellVarContext all
label_1132:
  DeleteRegKey SHCTX Software\Microsoft\Windows\CurrentVersion\Uninstall\2546e024-4dd1-5359-9177-3b6a84280dd8
  DeleteRegKey SHCTX Software\2546e024-4dd1-5359-9177-3b6a84280dd8
SectionEnd


/*
Function Initialize_____Plugins
  SetDetailsPrint none
  StrCmp $PLUGINSDIR "" 0 label_1145
  Push $0
  SetErrors
  GetTempFileName $0
  Delete $0
  CreateDirectory $0 ; CreateRestrictedDirectory
  IfErrors label_1146
  StrCpy $PLUGINSDIR $0
  Pop $0
label_1145:
  Return

label_1146:
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
