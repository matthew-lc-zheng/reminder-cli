
enum Priority{
    important,
    urgent,
    first,
    last,
}

struct Date{
    year:u16,
    month:u8,
    day:u8,
}

struct Clock{
    hour:u8,
    minute:u8,
    second:u8,
}

struct Deadline{
    date:Date,
    clock:Clock,
}

struct Event{
    title:String,
    id:u32,
    notes:String,
    deadline:Deadline,
    priority:u8
}

struct Book{
    owner:String,
    id:u32,

}


pub fn add_event(){}

pub fn create_list(){}

pub fn all_lists(){}

pub fn all_events(){

}

pub fn load_list(){}

pub fn finish_event(){}