#[derive(Copy, Clone)]
pub enum UsersOrdering {
    Name = 0,
    NameDescending,
    JapaneseName,
    JapaneseNameDescending,
    SignUpDate,
    SignUpDateDescending,
    Role,
    RoleDescending,
}

impl From<u8> for UsersOrdering {
    fn from(i: u8) -> Self {
        match i {
            0 => UsersOrdering::Name,
            1 => UsersOrdering::NameDescending,
            2 => UsersOrdering::JapaneseName,
            3 => UsersOrdering::JapaneseNameDescending,
            4 => UsersOrdering::SignUpDate,
            5 => UsersOrdering::SignUpDateDescending,
            6 => UsersOrdering::Role,
            7 => UsersOrdering::RoleDescending,
            _ => UsersOrdering::Name,
        }
    }
}

impl Default for UsersOrdering {
    fn default() -> UsersOrdering {
        UsersOrdering::Name
    }
}

impl UsersOrdering {
    pub fn to_parameters(self) -> Vec<String> {
        let is_descending = ((self as u8) & 1) == 1;

        let ordering = if is_descending {
            UsersOrdering::from(((self as u8) - 1))
        } else {
            self
        };

        let mut string = "";

        match ordering {
            UsersOrdering::JapaneseName => string = "name.jap",
            UsersOrdering::SignUpDate => string = "signup",
            UsersOrdering::Role => string = "role",
            _ => {}
        }

        let mut list = Vec::new();

        if !string.is_empty() {
            list.push(format!("{}", string));
        }

        if is_descending {
            list.push("direction=desc".to_string());
        }

        list
    }
}
