use crate::try_outs::generics::Animal;

pub struct Alien{
    pub name: String,
    pub race: String
}

impl Animal for Alien {}

