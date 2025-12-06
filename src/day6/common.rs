pub enum OperatedGroup {
    Sum(u64),
    Product(u64),
}

impl From<&str> for OperatedGroup {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Sum(0),
            "*" => Self::Product(1),
            _ => panic!(),
        }
    }
}

impl OperatedGroup {
    pub fn apply(self, value: u64) -> Self {
        match self {
            OperatedGroup::Sum(x) => OperatedGroup::Sum(x + value),
            OperatedGroup::Product(x) => OperatedGroup::Product(x * value),
        }
    }

    pub fn value(&self) -> u64 {
        match self {
            OperatedGroup::Sum(x) => *x,
            OperatedGroup::Product(x) => *x,
        }
    }
}
