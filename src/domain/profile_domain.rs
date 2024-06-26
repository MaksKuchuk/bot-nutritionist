use std::str::FromStr;

#[derive(Clone, Default)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        let string_literal = match self {
            Gender::Male => "Мужской",
            Gender::Female => "Женский",
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
            _ => Err(()),
        }
    }
}

#[derive(Clone, Default)]
pub enum PhysicalActivityLevel {
    #[default]
    Minimal,
    Little,
    Average,
    AboveAverage,
    Increased,
    High,
    VeryHigh,
}

impl ToString for PhysicalActivityLevel {
    fn to_string(&self) -> String {
        let string_literal = match self {
            PhysicalActivityLevel::Minimal => "Минимальный",
            PhysicalActivityLevel::Little => "Небольшой",
            PhysicalActivityLevel::Average => "Средний",
            PhysicalActivityLevel::AboveAverage => "Выше среднего",
            PhysicalActivityLevel::Increased => "Повышенный",
            PhysicalActivityLevel::High => "Высокий",
            PhysicalActivityLevel::VeryHigh => "Очень высокий",
        };

        string_literal.to_owned()
    }
}

impl FromStr for PhysicalActivityLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Минимальный" => Ok(PhysicalActivityLevel::Minimal),
            "Небольшой" => Ok(PhysicalActivityLevel::Little),
            "Средний" => Ok(PhysicalActivityLevel::Average),
            "Выше среднего" => Ok(PhysicalActivityLevel::AboveAverage),
            "Повышенный" => Ok(PhysicalActivityLevel::Increased),
            "Высокий" => Ok(PhysicalActivityLevel::High),
            "Очень высокий" => Ok(PhysicalActivityLevel::VeryHigh),
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
