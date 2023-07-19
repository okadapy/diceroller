use rand::Rng;
#[allow(dead_code)]

#[derive(Copy, Clone, Debug)]
enum RollType {
    Advantage,
    Disadvantage,
    Normal,
}

#[derive(Debug)]
enum RollResult {
    Advantage(RollAdvantage),
    Normal(Roll),
}

impl Into<Roll> for RollResult {
    fn into(self) -> Roll {
        match self {
            RollResult::Normal(v) => v,
            _ => panic!("Wrong type of Roll!"),
        }
    }
}

impl Into<RollAdvantage> for RollResult {
    fn into(self) -> RollAdvantage {
        match self {
            RollResult::Advantage(v) => v,
            _ => panic!("Wrong type of Roll!"),
        }
    }
}

#[derive(Debug)]
struct Roll {
    value: i32,
}

#[derive(Debug)]
struct RollAdvantage {
    result: Roll,
    other: Roll,
}

struct Dice {
    rolls: i32,
    sides: i32,
    roll_type: RollType,
}

impl Dice {
    fn new(rolls: i32, sides: i32, roll_type: RollType) -> Self {
        Dice {
            rolls,
            sides,
            roll_type,
        }
    }

    fn roll(&self) -> RollResult {
        return match self.roll_type {
            RollType::Advantage => self.roll_advantage(true),
            RollType::Disadvantage => self.roll_advantage(false),
            RollType::Normal => self.roll_normal(),
        };
    }

    fn roll_normal(&self) -> RollResult {
        let mut values_vec: Vec<i32> = vec![];
        let mut  value = 0;
        for _ in 1..=self.rolls {
            let roll = rand::thread_rng().gen_range(1..=self.sides);
            value = value + roll;
            values_vec.push(roll);
        }
        RollResult::Normal(Roll {
            value
        })
    }

    fn roll_advantage(&self, advantage: bool) -> RollResult {
        let v1: Roll = self.roll_normal().into();
        let v2: Roll = self.roll_normal().into();
        let res: RollAdvantage = match advantage {
            false => {
                if v1.value > v2.value {
                    RollAdvantage {
                        result: v2,
                        other: v1,
                    }
                } else {
                    RollAdvantage {
                        result: v1,
                        other: v2,
                    }
                }
            }
            true => {
                if v1.value > v2.value {
                    RollAdvantage {
                        result: v1,
                        other: v2,
                    }
                } else {
                    RollAdvantage {
                        result: v2,
                        other: v1,
                    }
                }
            }
        };
        RollResult::Advantage(res)
    }
}
fn main() {
    for i in vec![RollType::Normal, RollType::Disadvantage, RollType::Advantage] {
        let dice = Dice::new(1, 20, i);
        println!("{:?}", i.clone());
        match i {
            RollType::Normal => {
                let result: Roll = dice.roll().into();
                println!("result: {}", result.value);
            }
            _ => {
                let result: RollAdvantage = dice.roll().into();
                println!("result: {}, other: {}", result.result.value, result.other.value);
            }
        }
    }
}
