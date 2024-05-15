
use reqwest;

fn scrape_data(url: &str){

    let req = match reqwest::blocking::get(url){
        Ok(data) => data,
        Err(e) => {
            println!(" Error: {e}");
            return;
        }
    };

    assert!(req.status().is_success());

    let texto = match req.text() {
        Ok(t) => t,
        Err(e) => {
            println!("Error {e}");
            return;
        }
        
    };

    let texto = texto.trim();

    let lines = texto.split("\n");

    let mut txt: Vec<String> = vec![];

    for line in lines{

        let line: String = String::from(line);
        //line.push('$');
        
        if line.contains("<p>") || line.contains("</p>"){

            let mut rm = String::from("");
            let mut rep = String::from("");
            

            for ch in line.chars(){

                if ch == '<' && rm == "" {
                    rm.push(ch);    
                }

                if ch == '>' && rm == "<" {
                    rm = rm.replace("<", "");
                }

                if rm == "" {
                    rep.push(ch);
                }
            }
            txt.push(rep);
        }

    }

    for t in &txt {

        let t = t.replace(">", "");
         
        println!("{t}");

    }

}

fn main() {
    println!("Follow the data as requested: ");

    let url: &str = "https://pt.wikipedia.org/wiki/Alan_Turing";

    scrape_data(url);

    
}
