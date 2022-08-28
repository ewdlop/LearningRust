fn greet_wolrd(){
    println!("Hello, world!");

    let southern_germany : &str = "Grüß Gott!"; 
    let japan : &str = "ハロー・ワールド";

    let regions : [&str; 2] = [southern_germany, japan];

    for region in regions.iter() {
        println!("{}", region);
    }
}

fn main() {
    
    greet_wolrd();

    let penguin_data: &str ="\
        common name,length (cm)
        Little penguin,33
        Yellow-eyed penguin,65
        Fiordland penguin,60
        Invalid,data";
    
    let records = penguin_data.lines();

    for (i , record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<&str> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name : &str = fields[0];
        let maybe_length: Result<f32,_> = fields[1].parse();

        if maybe_length.is_err(){
            continue
        }

        let length = maybe_length.unwrap();
        println!("{} is {} cm long", name, length);
    }
    
}


