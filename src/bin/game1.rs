use stuff::{Interact, WorldState};
use text_engine::{stuff};
use serde::{Serialize, Deserialize};
use serde_json;
use std::{fs::File, println};

#[derive(Debug, Clone)]
enum Verb{
    Take(String), //pass item instead of a string
    Pick(String),
    Inventory,
    Equipment,
    Talk(String),
    Eat(String),
    Give(String),
    Equip(String),
    Go(String)
}


fn str_to_verb(s:&str, o:String)->Result<Verb, ()>{
    match s {
        "take" => Ok(Verb::Take(o)),
        "pick" => Ok(Verb::Pick(o)),
        "inventory" => Ok(Verb::Inventory),
        "equip" => Ok(Verb::Equip(o)),
        "talk" => Ok(Verb::Talk(o)),
        "eat" => Ok(Verb::Eat(o)),
        "give" => Ok(Verb::Give(o)),
        "equipment" => Ok(Verb::Equipment),
        "go" => Ok(Verb::Go(o)),
        _ => Err(())
    }
}

fn display_inventory(world_state:&stuff::WorldState){
    if world_state.inventory.is_empty(){
        println!("I got nuthin', sorry brah")
    } else {
        println!("In your inventory is {:?}", world_state.inventory);
    }
    
}

fn display_equipment(world_state:&stuff::WorldState){
    if world_state.equipment.is_empty(){
        println!("I got nuthin', sorry brah")
    } else {
        let george = world_state.equipment.iter();
        println!("You have {:?} equipped", world_state.equipment);
        //fix printing here probs?
    }
    
}

fn go(world_state:&mut WorldState, door_trig:String, here:&stuff::Room){
    if let Some(door)= here.doors.iter().find(|d|d.triggers.contains(&door_trig)){
        world_state.at = door.target;
        world_state.switch = true;
    }
    else{
        println!("You can't go there.");
    }
}

fn pick(world_state:&mut stuff::WorldState, object:String){
    if let Some(thing)= world_state.room_items[world_state.at.0].iter().find(|d|d.name.contains(&object)){
        println!("{} - {}", thing.name, thing.descr);
    }
    else{
        println!("You can't pick {} up.", object);
    }
}

fn equip(world_state:&mut stuff::WorldState, object:String){
    match world_state.room_items[world_state.at.0].iter().find::<_>(|d|d.name.contains(&object)&&d.wearable) {
        Some(thing) => {
            world_state.equipment.push(*thing);
            println!("{}", thing);
        }
        None => {  println!("You can't equip {}.", object);
                        
        }
    }
    match world_state.inventory.iter().find::<_>(|d|d.name.contains(&object)&&d.wearable) {
        Some(thing) => {
            world_state.equipment.push(*thing);
            println!("{}", thing);
        }
        None => { println!("You can't equip {}.", object);
                
        }
    }
}

fn take(world_state:&mut stuff::WorldState, object:String){ // add get rid of object in room items
    match world_state.room_items[world_state.at.0].iter().find::<_>(|d|d.name.contains(&object)) {
        Some(thing) => {
            world_state.inventory.push(*thing);
            println!("{}", thing);
        }
        None => {
            {
                    println!("You can't take {}.", object);
                }
        }
    }
}

fn talk<'a>(person:&String, rooms:&'a std::vec::Vec<text_engine::stuff::Room>, world_state:&mut stuff::WorldState)->Option<&'a stuff::Person>{

    match rooms[world_state.at.0].people.iter().find::<_>(|d|d.name.contains(person.as_str())) {
        Some(subject) => {
            println!("you are talking to {}, (leave) to end conversation", subject.name);
            println!("{} - {}", subject.name, subject.greeting);
            world_state.dialogue_state = true;
            return Some(subject);
        }
        None => { println!("You can't talk to {}.", person);
            return None;
        }
    }
}

fn dialogue(person:&stuff::Person, input:&mut String, world_state:&mut stuff::WorldState){
    if input.eq_ignore_ascii_case(person.dialogue[1][0].as_str()){ // if they get the right answer
        println!("{}\n", person.dialogue[1][1]);
        return;
    } else if input.eq_ignore_ascii_case("leave"){
        world_state.dialogue_state = false;
        return;
    } else {
        for i in person.dialogue.as_slice(){
            if input.eq_ignore_ascii_case(i[0].as_str()){
                println!("{}\n", i[1]);  
                return;
            } 
        }
    } 
    println!("{}\n", person.dialogue[0][1]); // if they say something undefined
    
}


fn main() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;
    use std::path::Path;

    let json_file_path = Path::new("./src/bin/assets.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let deserialized_map: Vec<stuff::Room> = serde_json::from_reader(json_file).expect("error while reading json");
    
   
    let mut world_state = stuff::WorldState {
        room_items: vec![vec![stuff::Item{name: "apple", descr: "blah", wearable:false}]],
        inventory: vec![],
        people_items: vec![],
        equipment: vec![],
        progress: 0,
        at: stuff::RoomID(0),
        switch: false,
        dialogue_state:false
    };
    let rooms = deserialized_map;
    //let end_rooms = [stuff::RoomID(2), stuff::RoomID(3)];
    let end_rooms = [];
    let mut input = String::new();

    let mut talk_to = None;
    let mut verb;
    let mut object;
    let mut iter;
    let input_arr: Vec<&str>;
    let mut action:Result<Verb, ()>;
    let needs = vec![
        stuff::Item{name:"goggles", descr:"Shining duochrome green and purple, you can tell they protect your eyes well", wearable:true},
        stuff::Item{name:"gloves", descr:"Black waterproof mittens with a fleece lining", wearable: true},
        stuff::Item{name:"coat", descr:"A Bright green coat covered with a leafy pattern", wearable: true},
        stuff::Item{name:"boots", descr:"Large hunks of plastic and metal, are these really meant for human feet??", wearable: true},
        stuff::Item{name:"long johns", descr:"Thin shirt and leggings made of wool and only slightly itchy.", wearable:true}
        ];
    println!("Skiing in Slatyfork");
    println!("============================");
    println!();
    println!("You wake up on a mountain in the backcountry of West Virginia with a sudden and deep urge to go skiing, but you're missing your ski gear! You need some equipment! \n * goggles \n * gloves \n * coat \n * long johns \n * boots\n**************");
    while !(stuff::check_win(&world_state, &needs)) {
        world_state.switch = false;
        //println!("top of loop, {}", worl);
        
        // We don't want to move out of rooms, so we take a reference
        let here = &rooms[world_state.at.0];
        
        println!(
            "{}\n* * * * * * * * * * * * * * * \n{}\n",
            here.name, here.descr
        );
        if end_rooms.contains(&world_state.at) {
            break;
        }
        while world_state.switch==false {
            print!("What will you do?\n> ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut input = input.trim().to_lowercase();
            iter = input.split_whitespace();
            // input_arr = iter.collect();
            
            
            // for i in 0..input_arr.len(){
            //     match input_arr[i] {
            //         "to" => input_arr.remove(i),
            //         "up" =>input_arr.remove(i),
            //          _ => ()
            //     }
            // }
                
            

            verb = match iter.next(){
                Some(x)=>x,
                None => ""
            };
            object = match iter.next(){
                Some(x)=>x,
                None => ""
            };
           
                //input_arr.append(word);
            action = str_to_verb(verb, object.to_string());
                //println!("{:?}", verb);


            match action { // can wrap these in ok()
                Ok(Verb::Take(object)) => take(&mut world_state, object),
                Ok(Verb::Pick(object)) => pick(&mut world_state, object),
                Ok(Verb::Inventory) => display_inventory(&world_state),
                Ok(Verb::Equip(object)) => equip(&mut world_state, object),
                Ok(Verb::Talk(object)) => talk_to=talk(&object, &rooms, &mut world_state),
                Ok(Verb::Eat(object)) => break,
                Ok(Verb::Give(object)) => println!("You want to give"),
                Ok(Verb::Equipment) => display_equipment(&world_state),
                Ok(Verb::Go(object))=> go(&mut world_state, object, here),
                Err(()) => println!("That action is invalid, try a command of the form \"verb + object\". You can also see your (inventory) or (equipment)")
                

            }
            while world_state.dialogue_state{
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let mut input = input.trim().to_lowercase();
                
                match talk_to{
                    Some(person) => dialogue(person, &mut input, &mut world_state),
                    None => ()
                }

            }
        }
    }
}