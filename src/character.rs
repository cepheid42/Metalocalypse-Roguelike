pub static CLASSES: [&str; 5] = ["Bardbarian", "Moshpit Bard", "Wizzards", "Bard of the Bees", "Bass Player"];

pub struct Character {
    pub char_name: String,
    pub char_class: &'static str,
    char_lvl: i32,
    str: i32,
    dex: i32,
    con: i32,
    cha: i32,
    int: i32,
    lck: i32,
    char_hp: i32,
    sp_atk: String,
    sp_abil: String
}
impl Character {
    fn new_barb(char_name: String, char_class: &'static str) -> Character {
        Character {
            char_name,
            char_class,
            char_lvl: 1,
            str: 16,
            dex: 10,
            con: 16,
            cha: 16,
            int: 14,
            lck: 14,
            char_hp: 14,
            sp_atk: "Growl".to_string(),
            sp_abil: "Brutal".to_string()
        }
    }
    fn new_mosh(char_name: String, char_class: &'static str) -> Character {
        Character {
            char_name,
            char_class,
            char_lvl: 1,
            str: 16,
            dex: 12,
            con: 18,
            cha: 12,
            int: 12,
            lck: 12,
            char_hp: 18,
            sp_atk: "Drum Solo".to_string(),
            sp_abil: "Black Out".to_string()
        }
    }
    fn new_wiz(char_name: String, char_class: &'static str) -> Character {
        Character {
            char_name,
            char_class,
            char_lvl: 1,
            str: 8,
            dex: 18,
            con: 10,
            cha: 18,
            int: 10,
            lck: 12,
            char_hp: 12,
            sp_atk: "Shred".to_string(),
            sp_abil: "Autofellatio".to_string()
        }
    }
    fn new_bee(char_name: String, char_class: &'static str) -> Character {
        Character {
            char_name,
            char_class,
            char_lvl: 1,
            str: 12,
            dex: 16,
            con: 18,
            cha: 10,
            int: 10,
            lck: 18,
            char_hp: 16,
            sp_atk: "Rhythm Player".to_string(),
            sp_abil: "Innocence".to_string()
        }
    }
    fn new_murderface(char_name: String, char_class: &'static str) -> Character {
        Character {
            char_name,
            char_class,
            char_lvl: 1,
            str: 10,
            dex: 10,
            con: 12,
            cha: 0,
            int: 8,
            lck: 0,
            char_hp: 10,
            sp_atk: "Penile Player".to_string(),
            sp_abil: "Disgusting Slob".to_string()
        }
    }
}

pub trait Player {
    fn new(char_name: String, char_class: &'static str) -> Character;
    fn info(&self) -> String;
}

impl Player for Character {
    fn new(char_name: String, char_class: &'static str) -> Character {
        match char_class {
            "Bardbarian"       => Character::new_barb(char_name, char_class),
            "Moshpit Bard"     => Character::new_mosh(char_name, char_class),
            "Wizzards"         => Character::new_wiz(char_name, char_class),
            "Bard of the Bees" => Character::new_bee(char_name, char_class),
            "Bass Player"      => Character::new_murderface(char_name, char_class),
                             _ => unreachable!()
        }
    }


    fn info(&self) -> String {
        format!("{} - Level {}\nClass: {}\nHP: {}\n\
                       Str: {}\nDex: {}\nCon: {}\n\
                       Cha: {}\nInt: {}\nLck: {}\n\
                       Special Attack: {}\n\
                       Special Ability: {}",
                self.char_name, self.char_lvl, self.char_class, self.char_hp,
                self.str, self.dex, self.con,
                self.cha, self.int, self.lck,
                self.sp_atk, self.sp_abil)
    }
}
