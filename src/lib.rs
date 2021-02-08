
// this provides all the structs needed for text-engine games
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
     
    
    impl std::fmt::Display for Item {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}:{}", self.name, self.descr)
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Person{
        pub name: String,
        pub id : PersonID,
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
        pub triggers: Vec<String>
    }

    

    pub struct WorldState {
        pub room_items: Vec<Vec<Item>>,
        pub inventory: Vec<Item>,
        pub people_items: Vec<Vec<Item>>,
        pub equipment: Vec<Item>,
        pub progress: u8,
        pub at: RoomID,
        pub switch: bool,
        pub dialogue_state: bool
    }

    #[derive(PartialEq, Eq, Clone, Copy, Deserialize, Debug)]
    pub struct RoomID( pub usize);

    #[derive(PartialEq, Eq, Clone, Copy, Deserialize, Debug)]
    pub struct PersonID( pub usize);

   pub fn check_win(world_state:&WorldState, needs:&Vec<Item>)->bool{
        for x in needs{
            if !(world_state.equipment.contains(&x)){
                return false;
            }
        }
        return true;
   }
}
