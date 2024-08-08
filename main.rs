use rand::{thread_rng,seq::SliceRandom};

#[derive(Debug)]

struct Deck{
    cards:Vec<String>,
}

impl Deck{
    fn new()->Self{
            //list of suits
        //list of values

        //double nested for loop

        let suits=["Hearts","Spades","Diamonds"];
        let values=["Ace","Two","Three"];

        let mut cards=vec![];

        for suit in suits{
            for value in values{
                let card=format!("{} of {}",value,suit);
                cards.push(card);
        }
    }
        let deck:Deck=Deck{cards};
        return deck;
    }

    fn shuffle(&mut self){
        let mut rng=thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self,num_cards:usize)->Vec<String>{
        self.cards.split_off(self.cards.len()-num_cards)
    }
}

fn main() {
    let mut deck=Deck::new();
    deck.shuffle();
    let cards=deck.deal(3);
    println!("Here's your deck: {:#?}",deck);
    println!("Here's your hand:{:#?}",cards);
}
