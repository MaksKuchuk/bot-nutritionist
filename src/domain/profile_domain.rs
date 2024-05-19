use std::str::FromStr;

#[derive(Clone, Default)]
pub enum Gender {
    #[default]
    Male,
    Female,
    RyanGosling,
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        let string_literal = match self {
            Gender::Male => "Мужской",
            Gender::Female => "Женский",
            Gender::RyanGosling => "Раян Гослинг",
        };
        string_literal.to_owned()
    }
}

impl FromStr for Gender {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Мужской" => Ok(Gender::Male),
            "Женский" => Ok(Gender::Female),
            "Раян Гослинг" => Ok(Gender::RyanGosling),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Default)]
pub enum PhysicalActivityLevel {
    #[default]
    Low,
    Moderate,
    High,
}

impl ToString for PhysicalActivityLevel {
    fn to_string(&self) -> String {
        let string_literal = match self {
            PhysicalActivityLevel::Low => "Низкий",
            PhysicalActivityLevel::Moderate => "Средний",
            PhysicalActivityLevel::High => "Высокий",
        };
        string_literal.to_owned()
    }
}

impl FromStr for PhysicalActivityLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Низкий" => Ok(PhysicalActivityLevel::Low),
            "Средний" => Ok(PhysicalActivityLevel::Moderate),
            "Высокий" => Ok(PhysicalActivityLevel::High),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Default)]
pub enum Goal {
    #[default]
    WeightLoss,
    WeightMaintenance,
    WeightGain,
}

impl ToString for Goal {
    fn to_string(&self) -> String {
        let string_literal = match self {
            Goal::WeightLoss => "Похудение",
            Goal::WeightMaintenance => "Поддержание веса",
            Goal::WeightGain => "Массанабор",
        };
        string_literal.to_owned()
    }
}

impl FromStr for Goal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Похудение" => Ok(Goal::WeightLoss),
            "Поддержание веса" => Ok(Goal::WeightMaintenance),
            "Массанабор" => Ok(Goal::WeightGain),
            _ => Err(()),
        }
    }
}
