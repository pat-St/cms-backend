
#[get("/")]
pub fn hello() -> &'static str {
	"I'm up!"
}