
#[derive(Debug)]
struct Deck{

    cards:Vec<String>

}

impl Deck {

    fn new()->Self{
         
         Deck { cards:vec![] }

    }
    
}
fn main() {
    
     let mut deck=Deck::new();
     
     println!("{:#?}",deck);
 
}
