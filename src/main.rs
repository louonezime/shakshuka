sada::sada! {
    use actix_web::{جلب, App, HttpServer, Responder};
    use reqwest;
    use serde_json::Value;

    #[جلب("/")]
    غير_متزامنة دالة جلب_idea() -> تنفيذ Responder {
        دع response_text = طابق reqwest::جلب("https://itsthisforthat.com/api.php?json").انتظر {
            حسنا(response) => response.text().انتظر.فك(),
            خطأ(err) => {
                هاطبع!("Error fetching from API: {}", err);
                return "Error fetching from API";
            }
        };

        دع response: نتيجة<Value, _> = serde_json::from_str(&response_text);

        طابق response {
            حسنا(idea_response) => {
                اطبع!("This: {}", idea_response["this"]);
                اطبع!("That: {}", idea_response["that"]);
                "Options printed to console"
            }
            خطأ(err) => {
                هاطبع!("Error deserializing JSON: {}", err);
                "Error deserializing JSON"
            }
        }
    }

    #[actix_web::رئيسي]
    غير_متزامنة دالة رئيسي() -> std::دخ::نتيجة<()> {
        HttpServer::new(|| {
            App::new().service(جلب_idea)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .انتظر
    }
}
