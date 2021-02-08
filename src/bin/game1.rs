use stuff::{PersonID, WorldState};
use text_engine::{stuff};
use serde_json;
use std::{fs::File, println};

#[derive(Debug, Clone)]
enum Verb{
    Take(String),
    View(String),
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
        "view" => Ok(Verb::View(o)),
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
        let mut inventory_string:String = "".to_string();
        for object in &world_state.inventory{
            inventory_string.push_str(object.name);
            inventory_string.push_str(", ");
        }
        println!("In your inventory is [{}]", inventory_string);
    }
    
}

fn display_equipment(world_state:&stuff::WorldState){
    if world_state.equipment.is_empty(){
        println!("I got nuthin', sorry brah")
    } else {
        let mut equipment_string:String = "".to_string();
        for object in &world_state.equipment{
            equipment_string.push_str(object.name);
            equipment_string.push_str(", ");
        }
        println!("You have [{}] equipped", equipment_string);
        
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

fn view(world_state:&mut stuff::WorldState, object:String){
    if let Some(thing)= world_state.inventory.iter().find(|d|d.name.contains(&object)){
        println!("{} - {}", thing.name, thing.descr);
    }
    else{
        println!("You can't view {} .", object);
    }
}

fn equip(world_state:&mut stuff::WorldState, object:String){
    if let Some(thing) = world_state.room_items[world_state.at.0].iter().find::<_>(|d|d.name.contains(&object)&&d.wearable) {
        world_state.equipment.push(*thing);
        println!("Equipping {}", thing.name);
    
    } else if let Some(thing) = world_state.inventory.iter().find::<_>(|d|d.name.contains(&object)&&d.wearable) {
        world_state.equipment.push(*thing);
        println!("Equipping {}", thing.name);
    } else {
        { println!("You can't equip {}.", object);
                
            }
    }
}

fn take(world_state:&mut stuff::WorldState, object:String){ // add get rid of object in room items
    match world_state.room_items[world_state.at.0].iter().find::<_>(|d|d.name.contains(&object)) {
        Some(thing) => {
            world_state.inventory.push(*thing);
            println!("Adding {} to inventory", thing.name);
        }
        None => {
            {
                    println!("You can't take {}.", object);
                }
        }
    }
}

fn take_from(world_state:&mut stuff::WorldState, object:String, person:PersonID){ // add get rid of object in room items
    match world_state.people_items[person.0].iter().find::<_>(|d|d.name.contains(&object)) {
        Some(thing) => {
            world_state.inventory.push(*thing);
            println!("Adding {} to inventory", thing.name);
            world_state.dialogue_state=false;
        }
        None => {
            {
                    println!("You can't take {}.", object);
                }
        }
    }
}

fn talk<'a>(person:&String, rooms:&'a std::vec::Vec<text_engine::stuff::Room>, world_state:&mut stuff::WorldState)->Option<&'a stuff::Person>{

    match rooms[world_state.at.0].people.iter().find::<_>(|d|d.name.contains(person.to_uppercase().as_str())) {
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
    let newinput = input.as_str().trim().to_lowercase();
    if newinput.eq_ignore_ascii_case(person.dialogue[1][0].as_str()){ // if they get to receive item
        println!("{} - {}", person.name, person.dialogue[1][1]);
        let gift = person.gift.as_str();
        take_from(world_state, gift.to_string(), person.id);
        world_state.dialogue_state = false;
        return;
    } else if newinput.eq_ignore_ascii_case("leave"){
        world_state.dialogue_state = false;
        return;
    } else {
        for i in person.dialogue.as_slice(){
            if newinput.eq_ignore_ascii_case(i[0].as_str()){
                println!("{} - {}", person.name, i[1]);  
                return;
            } 
        }
    } 
    println!("{} - {}", person.name, person.dialogue[0][1]); // if they say something undefined
    
}

fn give(giftee:&stuff::Person, gift:&String, world_state:&mut stuff::WorldState){
    match world_state.inventory.iter().find::<_>(|d|d.name.contains(gift.as_str())) {
        Some(d) => 
            if gift.eq_ignore_ascii_case(&giftee.wants){
                world_state.people_items[giftee.id.0].push(*d);
                
                println!("You're giving {} {}. \n {}", giftee.name, gift, giftee.dialogue[1][1]);
                take_from(world_state, giftee.gift.to_string(), giftee.id);
                
                
            } else {
                println!("You can't give {} {}", giftee.name, gift)
            }
        None => println!("You don't have {} in your inventory.", gift)
    }
    
}

fn main() {
    use std::io;
    // We need the Write trait so we can flush stdout
    use std::io::Write;
    use std::path::Path;

    println!("\n \n  ,_ o \n / //\\, \n  \\>> |\n    \\\\, ");

    let json_file_path = Path::new("./src/bin/assets.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let deserialized_map: Vec<stuff::Room> = serde_json::from_reader(json_file).expect("error while reading json");
    
   
    let mut world_state = stuff::WorldState {
        room_items: vec![vec![], vec![], vec![stuff::Item{name:"scarf", descr:"Long scarf made of wool and only slightly itchy.", wearable:true}], 
                    vec![stuff::Item{name:"boots", descr:"Large hunks of plastic and metal, are these really meant for human feet??", wearable: true}], vec![], vec![], vec![], vec![], vec![], vec![]],
        inventory: vec![stuff::Item{name:"bacon", descr:"uncooked bacon, what a weird thing to find in your pocket", wearable: false}],
        people_items: vec![
                vec![stuff::Item{name:"gloves", descr:"Black waterproof mittens with a fleece lining", wearable: true}],
                vec![stuff::Item{name:"goggles", descr:"Shining duochrome green and purple, you can tell they protect your eyes well", wearable:true}],
                vec![stuff::Item{name:"coat", descr:"A Bright green coat covered with a leafy pattern", wearable: true}],
                vec![stuff::Item{name:"helmet", descr:"A dark grey silver helmet with many stickers on them. One of them is a Harvey Mudd sticker!", wearable: true}],
                vec![stuff::Item{name:"bucket", descr: "Gallon size bucket of neapolitan ice cream. Maybe not the tastiest, but it's there, I guess.", wearable:false}]],
        equipment: vec![],
        progress: 0,
        at: stuff::RoomID(0),
        switch: false,
        dialogue_state:false
    };
    let rooms = deserialized_map;
    
    let mut input = String::new();

    let mut talk_to = None;
    let mut verb;
    let mut object;
    let mut iter;
    let mut action:Result<Verb, ()>;
    let needs = vec![
        stuff::Item{name:"goggles", descr:"Shining duochrome green and purple, you can tell they protect your eyes well", wearable:true},
        stuff::Item{name:"gloves", descr:"Black waterproof mittens with a fleece lining", wearable: true},
        stuff::Item{name:"coat", descr:"A Bright green coat covered with a leafy pattern", wearable: true},
        stuff::Item{name:"boots", descr:"Large hunks of plastic and metal, are these really meant for human feet??", wearable: true},
        stuff::Item{name:"scarf", descr:"Long scarf made of wool and only slightly itchy.", wearable:true}
        ];
    println!("Skiing in Slatyfork");
    println!("============================");
    println!();
    println!("You wake up on a mountain in the backcountry of West Virginia with a sudden and deep urge to go skiing, but you're missing your ski gear! You need some equipment, but you seem to have some bacon in your pocket! \n * goggles \n * gloves \n * coat \n * scarf \n * boots\n**************");
    while !(stuff::check_win(&world_state, &needs)) {
        world_state.switch = false;
        
        
        // We don't want to move out of rooms, so we take a reference
        let here = &rooms[world_state.at.0];
        
        println!(
            "* * * * * * * * * * * * * * *\n{}\n* * * * * * * * * * * * * * * \n{}\n",
            here.name, here.descr
        );
        while world_state.switch==false {
            print!("What will you do?\n> ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut input = input.trim().to_lowercase();
            iter = input.split_whitespace();
                
            

            verb = match iter.next(){
                Some(x)=>x,
                None => ""
            };
            object = match iter.next(){
                Some(x)=>x,
                None => ""
            };
           
            action = str_to_verb(verb, object.to_string());
            


            match action { // can wrap these in ok()
                Ok(Verb::Take(object)) => take(&mut world_state, object),
                Ok(Verb::View(object)) => view(&mut world_state, object),
                Ok(Verb::Inventory) => display_inventory(&world_state),
                Ok(Verb::Equip(object)) => equip(&mut world_state, object),
                Ok(Verb::Talk(object)) => talk_to=talk(&object, &rooms, &mut world_state),
                Ok(Verb::Eat(_object)) => break,
                Ok(Verb::Give(_object)) => println!("You need to be talking to to someone to give them a gift"),
                Ok(Verb::Equipment) => display_equipment(&world_state),
                Ok(Verb::Go(object))=> go(&mut world_state, object, here),
                Err(()) => println!("That action is invalid, try a command of the form \"verb + object\". You can also see your (inventory) or (equipment)")
                

            }
            // what to do when you're talking to someone
            while world_state.dialogue_state{
                print!("> ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let mut input = input.trim().to_lowercase();

                iter = input.split_whitespace();
                
            

            verb = match iter.next(){
                Some(x)=>x,
                None => ""
            };
            object = match iter.next(){
                Some(x)=>x,
                None => ""
            };
           

            action = str_to_verb(verb, object.to_string());

            match action {
                Ok(Verb::Give(object)) => give(talk_to.unwrap(), &object, &mut world_state),
                Ok(_) => (),
                Err(()) => match talk_to{
                    Some(person) => dialogue(person, &mut input, &mut world_state),
                    None => ()
                    }
            }

            }
        }
    }
    println!("* * * * * * * * * * * * * * * * * \n You won!! You get to go skiing!! ... only 1 hour later than you expected \n \n  ,_ o \n / //\\, \n  \\>> |\n    \\\\, ");
}