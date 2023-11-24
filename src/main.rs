sada::sada! {
    use actix_web::{جلب, App, HttpServer, Responder};
    use reqwest;
    use serde_json::Value;

    #[جلب("/")]
    غير_متزامنة دالة جلب_فكرة() -> تنفيذ Responder {
        دع إجابة_نص = طابق reqwest::جلب("https://itsthisforthat.com/api.php?json").انتظر {
            حسنا(إجابة) => إجابة.text().انتظر.فك(),
            خطأ(يخطئ) => {
                هاطبع!("خطأ : {}", يخطئ);
                return "Error fetching from API";
            }
        };

        دع إجابة: نتيجة<Value, _> = serde_json::from_str(&إجابة_نص);

        طابق إجابة {
            حسنا(فكرة_إجابة) => {
                اطبع!("هذا: {}", فكرة_إجابة["this"]);
                اطبع!("ذلك: {}", فكرة_إجابة["that"]);
                "Options printed to console"
            }
            خطأ(يخطئ) => {
                هاطبع!("خطأ : {}", يخطئ);
                "Error deserializing JSON"
            }
        }
    }

    #[actix_web::رئيسي]
    غير_متزامنة دالة رئيسي() -> std::دخ::نتيجة<()> {
        HttpServer::new(|| {
            App::new().service(جلب_فكرة)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .انتظر
    }
}
