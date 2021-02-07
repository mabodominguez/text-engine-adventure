#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod parse{
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;

    //use crate::stuff::WorldState;

    #[derive(Debug, Clone)]
    pub enum GrammarItem {
        Verb(String),
        Object(String),
        Person(String)
    } 

    // pub fn parse(input:String, world_state:WorldState){
    //     input.clear();
    //         io::stdin().read_line(&mut input).unwrap();
    //         let input = input.trim();
    //         if let Some(door) = here.doors.iter().find(|d| d.triggers.contains(&input.to_string())) {
    //             // #[std(borrow)]
    //             // if let Some(msg) = &door.message {
    //             //     println!("{}", msg);
    //             // }
    //             at = door.target;
    //             break;
    //         } else {
    //             println!("You can't do that!");
    //         }

    }
    


pub mod stuff{
    use std::fmt;
    use serde::{Deserialize};


    pub enum Interact{
        Item(Item),
        Door(Door),
        Person(Person)
    }
    #[derive(Debug, Deserialize, Clone, PartialEq)]
    pub struct Item{
        pub name: &'static str,
        pub descr: &'static str,
        pub wearable: bool
    }
    impl Copy for Item{

    }
     
    
    impl std::fmt::Display for Item { // probably change based on name
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}:{}", self.name, self.descr)
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Person{
        pub name: String,
        pub greeting: String,
        pub dialogue: Vec<Vec<String>>,
        pub gift: String,
        pub wants: String
    }


    #[derive(Deserialize, Debug)]
    pub struct Room {
        pub name: String,
        pub descr: String,
        pub doors: Vec<Door>,
        pub people: Vec<Person>
    }

    #[derive(Deserialize, Debug)]
    pub struct Door {
        pub target: RoomID,
        pub triggers: Vec<String>,
        // pub message: Option<String>,
        // condition: Option<Item>
    }

    

    pub struct WorldState {
        pub room_items: Vec<Vec<Item>>,
        pub inventory: Vec<Item>,
        pub people_items: Vec<Vec<Item>>,
        pub equipment: Vec<Item>,
        pub progress: u8,
        pub at: RoomID,
        pub switch: bool,
        pub dialogue_state: bool,
        pub talk_to: Option<Person>

    }

    #[derive(PartialEq, Eq, Clone, Copy, Deserialize, Debug)]
    pub struct RoomID( pub usize);

   pub fn check_win(world_state:&WorldState, needs:&Vec<Item>)->bool{
        for x in needs{
            if !(world_state.equipment.contains(&x)){
                return false;
            }
        }
        return true;
   }
}
