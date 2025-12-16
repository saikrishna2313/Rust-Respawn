
#[derive(Debug)]
struct Deck{  
    cards:Vec<String>
}

impl Deck {

    fn new()->Self{
         Deck { cards:vec![] }
    }
    
    fn add_cards(&mut self,cards:Vec<String>){
       self.cards=cards;
    }
}
fn main() {
    let mut deck=Deck::new();

    println!("{:#?}",deck); 
    
    let cards=vec![String::from("Ace"),String::from("Queen"),String::from("King")];

    deck.add_cards(cards);

    println!("{:#?}",deck);
}
