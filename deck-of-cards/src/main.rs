
#[derive(Debug)]
struct Deck{
    cards:Vec<String>
}

impl Deck{
     fn new()->Self{

        let suits=["Hearts","Daimonds","Spades"];
        let values=["One","Two","Ace"];
        let mut cards=vec![];

        for suite in suits{
             for value in values{
                cards.push(format!("{} of {}",suite,value));
             }
        }

        Deck{cards}
     }
}


fn main(){
    
    let deck = Deck::new();
    println!("Deck of Cards:{:#?}",deck);
   

}