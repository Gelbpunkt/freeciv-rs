use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

/// A technology in the research tree.
///
/// Technologies can be researched only if all the requried technologies to
/// research a technology have been researched. Each technology has a bulb
/// requirement, which is the amount of science output required to research this
/// technology. Researching a technology can take several turns, until the sum
/// of the science output during the turns meets the bulb requirement.
#[derive(Debug)]
pub struct Technology {
    name: &'static str,
    requirements: &'static [&'static Technology],
    bulbs: u16,
}

impl PartialEq<Technology> for Technology {
    fn eq(&self, other: &Technology) -> bool {
        self.name == other.name
    }
}

impl Eq for Technology {}

impl Hash for Technology {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Technology {
    /// Insert all technologies required to research this technology and their
    /// requirements recursively into the set.
    fn techs_required_recursive(&self, set: &mut HashSet<&'static Self>) {
        for tech in self.requirements {
            set.insert(tech);
            tech.techs_required_recursive(set);
        }
    }

    /// Calculate the total amount of bulbs required to research this technology
    /// and all its requirements recursively.
    #[must_use]
    pub fn total_bulbs(&self) -> u16 {
        let mut techs = HashSet::new();
        self.techs_required_recursive(&mut techs);
        techs.into_iter().map(|t| t.bulbs).sum::<u16>() + self.bulbs
    }

    /// Calculate the total amount of bulbs required to research this technology
    /// and all its requirements and the same for another different technology
    /// recursively.
    ///
    /// This method is intended for developers who wish to add a new technology
    /// and have its dependencies and the total bulbs required for the new
    /// technology at hand. Then, the total bulbs for both requirements can
    /// be calculated and substracted from the total bulbs required to get the
    /// amount of bulbs required for the new technology.
    #[must_use]
    pub fn total_bulbs_for_combined(&self, other: &'static Technology) -> u16 {
        let mut techs = HashSet::new();
        self.techs_required_recursive(&mut techs);
        other.techs_required_recursive(&mut techs);
        techs.into_iter().map(|t| t.bulbs).sum::<u16>() + self.bulbs + other.bulbs
    }
}

pub static ADVANCED_FLIGHT: Technology = Technology {
    name: "Advanced Flight",
    requirements: &[&RADIO, &MACHINE_TOOLS],
    bulbs: 1710,
};

pub static ALPHABET: Technology = Technology {
    name: "Alphabet",
    requirements: &[],
    bulbs: 30,
};

pub static AMPHIBIOUS_WARFARE: Technology = Technology {
    name: "Amphibious Warfare",
    requirements: &[&ENGINEERING, &TACTICS],
    bulbs: 1350,
};

pub static ASTRONOMY: Technology = Technology {
    name: "Astronomy",
    requirements: &[&MATHEMATICS, &MYSTICISM],
    bulbs: 180,
};

pub static ATOMIC_THEORY: Technology = Technology {
    name: "Atomic Theory",
    requirements: &[&CHEMISTRY, &REFRIGERATION],
    bulbs: 1110,
};

pub static AUTOMOBILE: Technology = Technology {
    name: "Automobile",
    requirements: &[&STEEL, &COMBUSTION],
    bulbs: 1380,
};

pub static BANKING: Technology = Technology {
    name: "Banking",
    requirements: &[&THE_REPUBLIC, &TRADE],
    bulbs: 300,
};

pub static BRIDGE_BUILDING: Technology = Technology {
    name: "Bridge Building",
    requirements: &[&THE_WHEEL, &CONSTRUCTION],
    bulbs: 240,
};

pub static BRONZE_WORKING: Technology = Technology {
    name: "Bronze Working",
    requirements: &[],
    bulbs: 30,
};

pub static CEREMONIAL_BURIAL: Technology = Technology {
    name: "Ceremonial Burial",
    requirements: &[],
    bulbs: 30,
};

pub static CHEMISTRY: Technology = Technology {
    name: "Chemistry",
    requirements: &[&UNIVERSITY, &MEDICINE],
    bulbs: 480,
};

pub static CHIVALRY: Technology = Technology {
    name: "Chivalry",
    requirements: &[&FEUDALISM, &HORSEBACK_RIDING],
    bulbs: 300,
};

pub static CODE_OF_LAWS: Technology = Technology {
    name: "Code of Laws",
    requirements: &[&ALPHABET],
    bulbs: 60,
};

pub static COMBINED_ARMS: Technology = Technology {
    name: "Combined Arms",
    requirements: &[&MOBILE_WARFARE, &ADVANCED_FLIGHT],
    bulbs: 1800,
};

pub static COMBUSTION: Technology = Technology {
    name: "Combustion",
    requirements: &[&ENGINEERING, &REFINING],
    bulbs: 1320,
};

pub static COMMUNISM: Technology = Technology {
    name: "Communism",
    requirements: &[&INDUSTRIALIZATION, &THEOLOGY],
    bulbs: 1260,
};

pub static COMPUTERS: Technology = Technology {
    name: "Computers",
    requirements: &[&MINIATURIZATION, &RADIO],
    bulbs: 1680,
};

pub static CONSCRIPTION: Technology = Technology {
    name: "Conscription",
    requirements: &[&METALLURGY, &DEMOCRACY],
    bulbs: 780,
};

pub static CONSTRUCTION: Technology = Technology {
    name: "Construction",
    requirements: &[&MASONRY, &IRON_WORKING],
    bulbs: 150,
};

pub static THE_CORPORATION: Technology = Technology {
    name: "The Corporation",
    requirements: &[&ECONOMICS, &INDUSTRIALIZATION],
    bulbs: 1200,
};

pub static CURRENCY: Technology = Technology {
    name: "Currency",
    requirements: &[&BRONZE_WORKING],
    bulbs: 60,
};

pub static DEMOCRACY: Technology = Technology {
    name: "Democracy",
    requirements: &[&BANKING, &INVENTION],
    bulbs: 570,
};

pub static ECONOMICS: Technology = Technology {
    name: "Economics",
    requirements: &[&UNIVERSITY, &BANKING],
    bulbs: 510,
};

pub static ELECTRICITY: Technology = Technology {
    name: "Electricity",
    requirements: &[&THEORY_OF_GRAVITY, &METALLURGY],
    bulbs: 900,
};

pub static ELECTRONICS: Technology = Technology {
    name: "Electronics",
    requirements: &[&ELECTRICITY, &THE_CORPORATION],
    bulbs: 1290,
};

pub static ENGINEERING: Technology = Technology {
    name: "Engineering",
    requirements: &[&ELECTRICITY, &STEAM_ENGINE],
    bulbs: 1110,
};

pub static ENVIROMENTALISM: Technology = Technology {
    name: "Enviromentalism",
    requirements: &[&SPACE_FLIGHT, &RECYCLING],
    bulbs: 1980,
};

pub static ESPIONAGE: Technology = Technology {
    name: "Espionage",
    requirements: &[&DEMOCRACY, &COMMUNISM],
    bulbs: 1320,
};

pub static EXPLOSIVES: Technology = Technology {
    name: "Explosives",
    requirements: &[&CHEMISTRY, &GUNPOWDER],
    bulbs: 810,
};

pub static FEUDALISM: Technology = Technology {
    name: "Feudalism",
    requirements: &[&MONARCHY, &IRON_WORKING],
    bulbs: 240,
};

pub static FLIGHT: Technology = Technology {
    name: "Flight",
    requirements: &[&COMBUSTION, &TACTICS],
    bulbs: 1500,
};

pub static FUSION_POWER: Technology = Technology {
    name: "Fusion Power",
    requirements: &[&SUPERCONDUCTORS, &LABOR_UNION],
    bulbs: 2310,
};

pub static GENETIC_ENGINEERING: Technology = Technology {
    name: "Genetic Engineering",
    requirements: &[&THE_CORPORATION, &REFRIGERATION],
    bulbs: 1350,
};

pub static GUERILLA_WARFARE: Technology = Technology {
    name: "Guerilla Warfare",
    requirements: &[&COMMUNISM, &TACTICS],
    bulbs: 1440,
};

pub static GUNPOWDER: Technology = Technology {
    name: "Gunpowder",
    requirements: &[&INVENTION, &FEUDALISM],
    bulbs: 510,
};

pub static HORSEBACK_RIDING: Technology = Technology {
    name: "Horseback Riding",
    requirements: &[],
    bulbs: 30,
};

pub static INDUSTRIALIZATION: Technology = Technology {
    name: "Industrialization",
    requirements: &[&BANKING, &RAILROAD],
    bulbs: 1140,
};

pub static INVENTION: Technology = Technology {
    name: "Invention",
    requirements: &[&LITERACY, &BRIDGE_BUILDING],
    bulbs: 390,
};

pub static IRON_WORKING: Technology = Technology {
    name: "Iron Working",
    requirements: &[&BRONZE_WORKING, &WARRIOR_CODE],
    bulbs: 90,
};

pub static LABOR_UNION: Technology = Technology {
    name: "Labor Union",
    requirements: &[&COMMUNISM, &MOBILE_WARFARE],
    bulbs: 1740,
};

pub static LASER: Technology = Technology {
    name: "Laser",
    requirements: &[&NUCLEAR_POWER, &COMPUTERS],
    bulbs: 1950,
};

pub static LEADERSHIP: Technology = Technology {
    name: "Leadership",
    requirements: &[&GUNPOWDER, &CHIVALRY],
    bulbs: 570,
};

pub static LITERACY: Technology = Technology {
    name: "Literacy",
    requirements: &[&WRITING, &CODE_OF_LAWS],
    bulbs: 120,
};

pub static MACHINE_TOOLS: Technology = Technology {
    name: "Machine Tools",
    requirements: &[&STEEL, &TACTICS],
    bulbs: 1440,
};

pub static MAGNETISM: Technology = Technology {
    name: "Magnetism",
    requirements: &[&ASTRONOMY, &SEAFARING],
    bulbs: 300,
};

pub static MAP_MAKING: Technology = Technology {
    name: "Map Making",
    requirements: &[&ALPHABET],
    bulbs: 60,
};

pub static MASONRY: Technology = Technology {
    name: "Masonry",
    requirements: &[],
    bulbs: 30,
};

pub static MASS_PRODUCTION: Technology = Technology {
    name: "Mass Production",
    requirements: &[&THE_CORPORATION, &AUTOMOBILE],
    bulbs: 1470,
};

pub static MATHEMATICS: Technology = Technology {
    name: "Mathematics",
    requirements: &[&ALPHABET, &MASONRY],
    bulbs: 90,
};

pub static MEDICINE: Technology = Technology {
    name: "Medicine",
    requirements: &[&PHILOSOPHY, &TRADE],
    bulbs: 360,
};

pub static METALLURGY: Technology = Technology {
    name: "Metallurgy",
    requirements: &[&MATHEMATICS, &GUNPOWDER],
    bulbs: 570,
};

pub static MINIATURIZATION: Technology = Technology {
    name: "Miniaturization",
    requirements: &[&ELECTRONICS, &COMBUSTION],
    bulbs: 1440,
};

pub static MOBILE_WARFARE: Technology = Technology {
    name: "Mobile Warfare",
    requirements: &[&AUTOMOBILE, &MACHINE_TOOLS],
    bulbs: 1590,
};

pub static MONARCHY: Technology = Technology {
    name: "Monarchy",
    requirements: &[&CODE_OF_LAWS, &CEREMONIAL_BURIAL],
    bulbs: 120,
};

pub static MONOTHEISM: Technology = Technology {
    name: "Monotheism",
    requirements: &[&ASTRONOMY, &POLYTHEISM],
    bulbs: 270,
};

pub static MYSTICISM: Technology = Technology {
    name: "Mysticism",
    requirements: &[&CEREMONIAL_BURIAL],
    bulbs: 60,
};

pub static NAVIGATION: Technology = Technology {
    name: "Navigation",
    requirements: &[&PHYSICS, &INVENTION],
    bulbs: 690,
};

pub static NUCLEAR_FISSION: Technology = Technology {
    name: "Nuclear Fission",
    requirements: &[&ATOMIC_THEORY, &MASS_PRODUCTION],
    bulbs: 1590,
};

pub static NUCLEAR_POWER: Technology = Technology {
    name: "Nuclear Power",
    requirements: &[&NUCLEAR_FISSION, &MINIATURIZATION],
    bulbs: 1680,
};

pub static PHILOSOPHY: Technology = Technology {
    name: "Philosophy",
    requirements: &[&LITERACY, &MYSTICISM],
    bulbs: 210,
};

pub static PHYSICS: Technology = Technology {
    name: "Physics",
    requirements: &[&MAGNETISM, &THE_WHEEL],
    bulbs: 390,
};

pub static PLASTICS: Technology = Technology {
    name: "Plastics",
    requirements: &[&MASS_PRODUCTION, &ROBOTICS],
    bulbs: 1890,
};

pub static POLYTHEISM: Technology = Technology {
    name: "Polytheism",
    requirements: &[&CEREMONIAL_BURIAL, &HORSEBACK_RIDING],
    bulbs: 90,
};

pub static POTTERY: Technology = Technology {
    name: "Pottery",
    requirements: &[],
    bulbs: 30,
};

pub static RADIO: Technology = Technology {
    name: "Radio",
    requirements: &[&ELECTRONICS, &FLIGHT],
    bulbs: 1620,
};

pub static RAILROAD: Technology = Technology {
    name: "Railroad",
    requirements: &[&METALLURGY, &STEAM_ENGINE],
    bulbs: 1050,
};

pub static RECYCLING: Technology = Technology {
    name: "Recycling",
    requirements: &[&MASS_PRODUCTION, &SANITATION],
    bulbs: 1530,
};

pub static REFINING: Technology = Technology {
    name: "Refining",
    requirements: &[&INDUSTRIALIZATION, &EXPLOSIVES],
    bulbs: 1200,
};

pub static REFRIGERATION: Technology = Technology {
    name: "Refrigeration",
    requirements: &[&ELECTRICITY, &SANITATION],
    bulbs: 1050,
};

pub static ROBOTICS: Technology = Technology {
    name: "Robotics",
    requirements: &[&COMPUTERS, &MOBILE_WARFARE],
    bulbs: 1830,
};

pub static ROCKETRY: Technology = Technology {
    name: "Rocketry",
    requirements: &[&MINIATURIZATION, &ADVANCED_FLIGHT],
    bulbs: 1770,
};

pub static SANITATION: Technology = Technology {
    name: "Sanitation",
    requirements: &[&MEDICINE, &BRIDGE_BUILDING],
    bulbs: 600,
};

pub static SEAFARING: Technology = Technology {
    name: "Seafaring",
    requirements: &[&MAP_MAKING, &POTTERY],
    bulbs: 120,
};

pub static SPACE_FLIGHT: Technology = Technology {
    name: "Space Flight",
    requirements: &[&COMPUTERS, &ROCKETRY],
    bulbs: 1830,
};

pub static STEALTH: Technology = Technology {
    name: "Stealth",
    requirements: &[&PLASTICS, &SPACE_FLIGHT],
    bulbs: 2010,
};

pub static STEAM_ENGINE: Technology = Technology {
    name: "Steam Engine",
    requirements: &[&CHEMISTRY, &NAVIGATION],
    bulbs: 900,
};

pub static STEEL: Technology = Technology {
    name: "Steel",
    requirements: &[&INDUSTRIALIZATION, &ENGINEERING],
    bulbs: 1260,
};

pub static SUPERCONDUCTORS: Technology = Technology {
    name: "Superconductors",
    requirements: &[&LASER, &SPACE_FLIGHT],
    bulbs: 2100,
};

pub static TACTICS: Technology = Technology {
    name: "Tactics",
    requirements: &[&CONSCRIPTION, &LEADERSHIP],
    bulbs: 870,
};

pub static THE_REPUBLIC: Technology = Technology {
    name: "The Republic",
    requirements: &[&CODE_OF_LAWS, &LITERACY],
    bulbs: 150,
};

pub static THE_WHEEL: Technology = Technology {
    name: "The Wheel",
    requirements: &[&HORSEBACK_RIDING],
    bulbs: 60,
};

pub static THEOLOGY: Technology = Technology {
    name: "Theology",
    requirements: &[&PHILOSOPHY, &MONOTHEISM],
    bulbs: 420,
};

pub static THEORY_OF_GRAVITY: Technology = Technology {
    name: "Theory of Gravity",
    requirements: &[&UNIVERSITY, &PHYSICS],
    bulbs: 570,
};

pub static TRADE: Technology = Technology {
    name: "Trade",
    requirements: &[&POTTERY, &CURRENCY],
    bulbs: 120,
};

pub static UNIVERSITY: Technology = Technology {
    name: "University",
    requirements: &[&MATHEMATICS, &PHILOSOPHY],
    bulbs: 300,
};

pub static WARRIOR_CODE: Technology = Technology {
    name: "Warrior Code",
    requirements: &[],
    bulbs: 30,
};

pub static WRITING: Technology = Technology {
    name: "Writing",
    requirements: &[&ALPHABET],
    bulbs: 60,
};

pub static ALL_TECHNOLOGIES: &[&Technology] = &[
    &ADVANCED_FLIGHT,
    &ALPHABET,
    &AMPHIBIOUS_WARFARE,
    &ASTRONOMY,
    &ATOMIC_THEORY,
    &AUTOMOBILE,
    &BANKING,
    &BRIDGE_BUILDING,
    &BRONZE_WORKING,
    &CEREMONIAL_BURIAL,
    &CHEMISTRY,
    &CHIVALRY,
    &CODE_OF_LAWS,
    &COMBINED_ARMS,
    &COMBUSTION,
    &COMMUNISM,
    &COMPUTERS,
    &CONSCRIPTION,
    &CONSTRUCTION,
    &THE_CORPORATION,
    &CURRENCY,
    &DEMOCRACY,
    &ECONOMICS,
    &ELECTRICITY,
    &ELECTRONICS,
    &ENGINEERING,
    &ENVIROMENTALISM,
    &ESPIONAGE,
    &EXPLOSIVES,
    &FEUDALISM,
    &FLIGHT,
    &FUSION_POWER,
    &GENETIC_ENGINEERING,
    &GUERILLA_WARFARE,
    &GUNPOWDER,
    &HORSEBACK_RIDING,
    &INDUSTRIALIZATION,
    &INVENTION,
    &IRON_WORKING,
    &LABOR_UNION,
    &LASER,
    &LEADERSHIP,
    &LITERACY,
    &MACHINE_TOOLS,
    &MAGNETISM,
    &MAP_MAKING,
    &MASONRY,
    &MASS_PRODUCTION,
    &MATHEMATICS,
    &MEDICINE,
    &METALLURGY,
    &MINIATURIZATION,
    &MOBILE_WARFARE,
    &MONARCHY,
    &MONOTHEISM,
    &MYSTICISM,
    &NAVIGATION,
    &NUCLEAR_FISSION,
    &NUCLEAR_POWER,
    &PHILOSOPHY,
    &PHYSICS,
    &PLASTICS,
    &POLYTHEISM,
    &POTTERY,
    &RADIO,
    &RAILROAD,
    &RECYCLING,
    &REFINING,
    &REFRIGERATION,
    &ROBOTICS,
    &ROCKETRY,
    &SANITATION,
    &SEAFARING,
    &SPACE_FLIGHT,
    &STEALTH,
    &STEAM_ENGINE,
    &STEEL,
    &SUPERCONDUCTORS,
    &TACTICS,
    &THE_REPUBLIC,
    &THE_WHEEL,
    &THEOLOGY,
    &THEORY_OF_GRAVITY,
    &TRADE,
    &UNIVERSITY,
    &WARRIOR_CODE,
    &WRITING,
];

#[test]
fn test_technology_requirements_finite_recursion() {
    for technology in ALL_TECHNOLOGIES {
        let _ = technology.total_bulbs();
    }
}
