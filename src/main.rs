mod parse_route;
mod request;
mod response;
mod router;
mod types;
use request::{Request};
use router::{Response, Router};


fn main() {
    println!("Hello, world!");
    //creating router
    let mut router = Router::new();

    //adding route to router
    fn handler1(_req:&mut Request,res:&mut Response){
        res.send_text("qweqwe".to_string());
    }
    
    router.get("/deneme",handler1);

    //add another route to router   
    fn handler2(_req:&mut Request, res:&mut Response){
        res.send_text("deneme".to_string());
    }
    router.get("/oo", handler2);

    //post handler example
    fn post_handler(req:&mut Request,res:&mut Response){
        res.set_status_code_raw(200);
        println!("request is => {:?}",req);
        res.send_text("post example".to_string());
    }
    router.post("/examplepost",post_handler);


    router.listen(8080);
}
