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

impl RollResult {
    fn try_into_advantage(self) -> Result<RollAdvantage, Self> {
        if let Self::Advantage(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    fn try_into_normal(self) -> Result<Roll, Self> {
        if let Self::Normal(v) = self {
            Ok(v)
        } else {
            Err(self)
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
        match self.roll_type {
            RollType::Advantage => self.roll_advantage(true),
            RollType::Disadvantage => self.roll_advantage(false),
            RollType::Normal => self.roll_normal(),
        }
    }

    fn roll_normal(&self) -> RollResult {
        let mut values_vec: Vec<i32> = vec![];
        let mut value = 0;
        for _ in 1..=self.rolls {
            let roll = rand::thread_rng().gen_range(1..=self.sides);
            value += roll;
            values_vec.push(roll);
        }
        RollResult::Normal(Roll { value })
    }

    fn roll_advantage(&self, advantage: bool) -> RollResult {
        let v1: Roll = self.roll_normal().try_into_normal().unwrap();
        let v2: Roll = self.roll_normal().try_into_normal().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_roll_range_test() {
        for sides in 2..=20 {
            for rolls in 1..=10 {
                for _i in 0..=5000 {
                    let dice: Dice = Dice::new(rolls, sides, RollType::Normal);
                    let roll: Roll = dice.roll().try_into_normal().unwrap();
                    assert!(
                        roll.value >= 1 && roll.value <= rolls * sides,
                        "rolls = {}, sides = {}; rolls*sides = {}",
                        rolls,
                        sides,
                        rolls * sides
                    );
                }
            }
        }
    }

    #[test]
    fn advantage_roll_value_test() {
        let roll_types_vec: Vec<RollType> = vec![RollType::Advantage, RollType::Disadvantage];
        for roll_type in roll_types_vec {
            for sides in 2..=20 {
                for rolls in 1..=10 {
                    for _i in 0..=5000 {
                        let dice: Dice = Dice::new(rolls, sides, roll_type);
                        let roll: RollAdvantage = dice.roll().try_into_advantage().unwrap();
                        match roll_type {
                            RollType::Advantage => assert!(
                                roll.result.value >= roll.other.value,
                                "Result = {}, Other = {}",
                                roll.result.value,
                                roll.other.value
                            ),
                            RollType::Disadvantage => assert!(
                                roll.result.value <= roll.other.value,
                                "Result = {}, Other = {}",
                                roll.result.value,
                                roll.other.value
                            ),
                            _ => panic!("Bad test configuration!"),
                        }
                    }
                }
            }
        }
    }
}

fn main() {}
