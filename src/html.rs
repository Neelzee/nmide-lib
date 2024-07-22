use crate::attribute::Attribute;

pub enum Html {
    Div {
        attr: Vec<Attribute>,
        kids: Vec<Html>,
    },
    P {
        attr: Vec<Attribute>,
        kids: Vec<Html>,
    },

    Img {
        attr: Vec<Attribute>,
        kids: Vec<Html>,
    },
    Span {
        attr: Vec<Attribute>,
        kids: Vec<Html>,
    },
    H1 {
        attr: Vec<Attribute>,
        kids: Vec<Html>,
    },
    H2 {
        attr: Vec<Attribute>,
        kids: Vec<Html>,
    },
    H3 {
        attr: Vec<Attribute>,
        kids: Vec<Html>,
    },
    Text(String),
}
