
enum Priority{
    important,
    
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
