use crate::element::{Element, Isotope, PeriodicTable};
use std::sync::LazyLock;

pub fn populate_periodic_table(table: &mut PeriodicTable) {
    let mut elt = Element {
        symbol: String::from("Ac"),
        most_abundant_isotope: 0,
        most_abundant_mass: 227.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 227.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ag"),
        most_abundant_isotope: 107,
        most_abundant_mass: 106.905097,
        element_number: 107,
        ..Default::default()
    };
    elt.isotopes.insert(
        109,
        Isotope {
            mass: 108.904752,
            abundance: 0.481610,
            neutrons: 109,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        107,
        Isotope {
            mass: 106.905097,
            abundance: 0.518390,
            neutrons: 107,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Al"),
        most_abundant_isotope: 27,
        most_abundant_mass: 26.981539,
        element_number: 27,
        ..Default::default()
    };
    elt.isotopes.insert(
        27,
        Isotope {
            mass: 26.981539,
            abundance: 1.000000,
            neutrons: 27,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Am"),
        most_abundant_isotope: 0,
        most_abundant_mass: 243.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 243.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ar"),
        most_abundant_isotope: 40,
        most_abundant_mass: 39.962383,
        element_number: 40,
        ..Default::default()
    };
    elt.isotopes.insert(
        36,
        Isotope {
            mass: 35.967545,
            abundance: 0.003365,
            neutrons: 36,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        38,
        Isotope {
            mass: 37.962732,
            abundance: 0.000632,
            neutrons: 38,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        40,
        Isotope {
            mass: 39.962383,
            abundance: 0.996003,
            neutrons: 40,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("As"),
        most_abundant_isotope: 75,
        most_abundant_mass: 74.921597,
        element_number: 75,
        ..Default::default()
    };
    elt.isotopes.insert(
        75,
        Isotope {
            mass: 74.921597,
            abundance: 1.000000,
            neutrons: 75,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("At"),
        most_abundant_isotope: 0,
        most_abundant_mass: 210.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 210.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Au"),
        most_abundant_isotope: 197,
        most_abundant_mass: 196.966569,
        element_number: 197,
        ..Default::default()
    };
    elt.isotopes.insert(
        197,
        Isotope {
            mass: 196.966569,
            abundance: 1.000000,
            neutrons: 197,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("B"),
        most_abundant_isotope: 11,
        most_abundant_mass: 11.009305,
        element_number: 11,
        ..Default::default()
    };
    elt.isotopes.insert(
        10,
        Isotope {
            mass: 10.012937,
            abundance: 0.199000,
            neutrons: 10,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        11,
        Isotope {
            mass: 11.009305,
            abundance: 0.801000,
            neutrons: 11,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ba"),
        most_abundant_isotope: 138,
        most_abundant_mass: 137.905247,
        element_number: 138,
        ..Default::default()
    };
    elt.isotopes.insert(
        130,
        Isotope {
            mass: 129.906321,
            abundance: 0.001060,
            neutrons: 130,
            neutron_shift: -8,
        },
    );
    elt.isotopes.insert(
        132,
        Isotope {
            mass: 131.905061,
            abundance: 0.001010,
            neutrons: 132,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        134,
        Isotope {
            mass: 133.904508,
            abundance: 0.024170,
            neutrons: 134,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        135,
        Isotope {
            mass: 134.905689,
            abundance: 0.065920,
            neutrons: 135,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        136,
        Isotope {
            mass: 135.904576,
            abundance: 0.078540,
            neutrons: 136,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        137,
        Isotope {
            mass: 136.905827,
            abundance: 0.112320,
            neutrons: 137,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        138,
        Isotope {
            mass: 137.905247,
            abundance: 0.716980,
            neutrons: 138,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Be"),
        most_abundant_isotope: 9,
        most_abundant_mass: 9.012182,
        element_number: 9,
        ..Default::default()
    };
    elt.isotopes.insert(
        9,
        Isotope {
            mass: 9.012182,
            abundance: 1.000000,
            neutrons: 9,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Bh"),
        most_abundant_isotope: 0,
        most_abundant_mass: 272.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 272.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Bi"),
        most_abundant_isotope: 209,
        most_abundant_mass: 208.980399,
        element_number: 209,
        ..Default::default()
    };
    elt.isotopes.insert(
        209,
        Isotope {
            mass: 208.980399,
            abundance: 1.000000,
            neutrons: 209,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Bk"),
        most_abundant_isotope: 0,
        most_abundant_mass: 247.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 247.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Br"),
        most_abundant_isotope: 79,
        most_abundant_mass: 78.918337,
        element_number: 79,
        ..Default::default()
    };
    elt.isotopes.insert(
        81,
        Isotope {
            mass: 80.916291,
            abundance: 0.493100,
            neutrons: 81,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        79,
        Isotope {
            mass: 78.918337,
            abundance: 0.506900,
            neutrons: 79,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("C"),
        most_abundant_isotope: 12,
        most_abundant_mass: 12.000000,
        element_number: 12,
        ..Default::default()
    };
    elt.isotopes.insert(
        13,
        Isotope {
            mass: 13.003355,
            abundance: 0.010700,
            neutrons: 13,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        12,
        Isotope {
            mass: 12.000000,
            abundance: 0.989300,
            neutrons: 12,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ca"),
        most_abundant_isotope: 40,
        most_abundant_mass: 39.962591,
        element_number: 40,
        ..Default::default()
    };
    elt.isotopes.insert(
        43,
        Isotope {
            mass: 42.958767,
            abundance: 0.001350,
            neutrons: 43,
            neutron_shift: 3,
        },
    );
    elt.isotopes.insert(
        46,
        Isotope {
            mass: 45.953693,
            abundance: 0.000040,
            neutrons: 46,
            neutron_shift: 6,
        },
    );
    elt.isotopes.insert(
        48,
        Isotope {
            mass: 47.952534,
            abundance: 0.001870,
            neutrons: 48,
            neutron_shift: 8,
        },
    );
    elt.isotopes.insert(
        42,
        Isotope {
            mass: 41.958618,
            abundance: 0.006470,
            neutrons: 42,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        44,
        Isotope {
            mass: 43.955482,
            abundance: 0.020860,
            neutrons: 44,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        40,
        Isotope {
            mass: 39.962591,
            abundance: 0.969410,
            neutrons: 40,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Cd"),
        most_abundant_isotope: 114,
        most_abundant_mass: 113.903358,
        element_number: 114,
        ..Default::default()
    };
    elt.isotopes.insert(
        106,
        Isotope {
            mass: 105.906459,
            abundance: 0.012500,
            neutrons: 106,
            neutron_shift: -8,
        },
    );
    elt.isotopes.insert(
        108,
        Isotope {
            mass: 107.904184,
            abundance: 0.008900,
            neutrons: 108,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        116,
        Isotope {
            mass: 115.904756,
            abundance: 0.074900,
            neutrons: 116,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        110,
        Isotope {
            mass: 109.903002,
            abundance: 0.124900,
            neutrons: 110,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        113,
        Isotope {
            mass: 112.904402,
            abundance: 0.122200,
            neutrons: 113,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        111,
        Isotope {
            mass: 110.904178,
            abundance: 0.128000,
            neutrons: 111,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        112,
        Isotope {
            mass: 111.902758,
            abundance: 0.241300,
            neutrons: 112,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        114,
        Isotope {
            mass: 113.903358,
            abundance: 0.287300,
            neutrons: 114,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ce"),
        most_abundant_isotope: 140,
        most_abundant_mass: 139.905439,
        element_number: 140,
        ..Default::default()
    };
    elt.isotopes.insert(
        136,
        Isotope {
            mass: 135.907172,
            abundance: 0.001850,
            neutrons: 136,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        138,
        Isotope {
            mass: 137.905991,
            abundance: 0.002510,
            neutrons: 138,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        142,
        Isotope {
            mass: 141.909244,
            abundance: 0.111140,
            neutrons: 142,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        140,
        Isotope {
            mass: 139.905439,
            abundance: 0.884500,
            neutrons: 140,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Cf"),
        most_abundant_isotope: 0,
        most_abundant_mass: 251.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 251.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Cl"),
        most_abundant_isotope: 35,
        most_abundant_mass: 34.968853,
        element_number: 35,
        ..Default::default()
    };
    elt.isotopes.insert(
        37,
        Isotope {
            mass: 36.965903,
            abundance: 0.242400,
            neutrons: 37,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        35,
        Isotope {
            mass: 34.968853,
            abundance: 0.757600,
            neutrons: 35,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Cm"),
        most_abundant_isotope: 0,
        most_abundant_mass: 247.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 247.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Cn"),
        most_abundant_isotope: 0,
        most_abundant_mass: 285.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 285.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Co"),
        most_abundant_isotope: 59,
        most_abundant_mass: 58.933195,
        element_number: 59,
        ..Default::default()
    };
    elt.isotopes.insert(
        59,
        Isotope {
            mass: 58.933195,
            abundance: 1.000000,
            neutrons: 59,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Cr"),
        most_abundant_isotope: 52,
        most_abundant_mass: 51.940508,
        element_number: 52,
        ..Default::default()
    };
    elt.isotopes.insert(
        54,
        Isotope {
            mass: 53.938880,
            abundance: 0.023650,
            neutrons: 54,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        50,
        Isotope {
            mass: 49.946044,
            abundance: 0.043450,
            neutrons: 50,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        53,
        Isotope {
            mass: 52.940649,
            abundance: 0.095010,
            neutrons: 53,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        52,
        Isotope {
            mass: 51.940508,
            abundance: 0.837890,
            neutrons: 52,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Cs"),
        most_abundant_isotope: 133,
        most_abundant_mass: 132.905452,
        element_number: 133,
        ..Default::default()
    };
    elt.isotopes.insert(
        133,
        Isotope {
            mass: 132.905452,
            abundance: 1.000000,
            neutrons: 133,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Cu"),
        most_abundant_isotope: 63,
        most_abundant_mass: 62.929597,
        element_number: 63,
        ..Default::default()
    };
    elt.isotopes.insert(
        65,
        Isotope {
            mass: 64.927790,
            abundance: 0.308500,
            neutrons: 65,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        63,
        Isotope {
            mass: 62.929597,
            abundance: 0.691500,
            neutrons: 63,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Db"),
        most_abundant_isotope: 0,
        most_abundant_mass: 268.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 268.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ds"),
        most_abundant_isotope: 0,
        most_abundant_mass: 281.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 281.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Dy"),
        most_abundant_isotope: 164,
        most_abundant_mass: 163.929175,
        element_number: 164,
        ..Default::default()
    };
    elt.isotopes.insert(
        156,
        Isotope {
            mass: 155.924283,
            abundance: 0.000560,
            neutrons: 156,
            neutron_shift: -8,
        },
    );
    elt.isotopes.insert(
        158,
        Isotope {
            mass: 157.924409,
            abundance: 0.000950,
            neutrons: 158,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        160,
        Isotope {
            mass: 159.925197,
            abundance: 0.023290,
            neutrons: 160,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        161,
        Isotope {
            mass: 160.926933,
            abundance: 0.188890,
            neutrons: 161,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        162,
        Isotope {
            mass: 161.926798,
            abundance: 0.254750,
            neutrons: 162,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        163,
        Isotope {
            mass: 162.928731,
            abundance: 0.248960,
            neutrons: 163,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        164,
        Isotope {
            mass: 163.929175,
            abundance: 0.282600,
            neutrons: 164,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Er"),
        most_abundant_isotope: 166,
        most_abundant_mass: 165.930293,
        element_number: 166,
        ..Default::default()
    };
    elt.isotopes.insert(
        162,
        Isotope {
            mass: 161.928778,
            abundance: 0.001390,
            neutrons: 162,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        164,
        Isotope {
            mass: 163.929200,
            abundance: 0.016010,
            neutrons: 164,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        170,
        Isotope {
            mass: 169.935464,
            abundance: 0.149100,
            neutrons: 170,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        167,
        Isotope {
            mass: 166.932048,
            abundance: 0.228690,
            neutrons: 167,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        168,
        Isotope {
            mass: 167.932370,
            abundance: 0.269780,
            neutrons: 168,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        166,
        Isotope {
            mass: 165.930293,
            abundance: 0.335030,
            neutrons: 166,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Es"),
        most_abundant_isotope: 0,
        most_abundant_mass: 252.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 252.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Eu"),
        most_abundant_isotope: 153,
        most_abundant_mass: 152.921230,
        element_number: 153,
        ..Default::default()
    };
    elt.isotopes.insert(
        151,
        Isotope {
            mass: 150.919850,
            abundance: 0.478100,
            neutrons: 151,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        153,
        Isotope {
            mass: 152.921230,
            abundance: 0.521900,
            neutrons: 153,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("F"),
        most_abundant_isotope: 19,
        most_abundant_mass: 18.998403,
        element_number: 19,
        ..Default::default()
    };
    elt.isotopes.insert(
        19,
        Isotope {
            mass: 18.998403,
            abundance: 1.000000,
            neutrons: 19,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Fe"),
        most_abundant_isotope: 56,
        most_abundant_mass: 55.934937,
        element_number: 56,
        ..Default::default()
    };
    elt.isotopes.insert(
        58,
        Isotope {
            mass: 57.933276,
            abundance: 0.002820,
            neutrons: 58,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        57,
        Isotope {
            mass: 56.935394,
            abundance: 0.021190,
            neutrons: 57,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        56,
        Isotope {
            mass: 55.934937,
            abundance: 0.917540,
            neutrons: 56,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Fm"),
        most_abundant_isotope: 0,
        most_abundant_mass: 257.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 257.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Fr"),
        most_abundant_isotope: 0,
        most_abundant_mass: 223.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 223.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ga"),
        most_abundant_isotope: 69,
        most_abundant_mass: 68.925574,
        element_number: 69,
        ..Default::default()
    };
    elt.isotopes.insert(
        71,
        Isotope {
            mass: 70.924701,
            abundance: 0.398920,
            neutrons: 71,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        69,
        Isotope {
            mass: 68.925574,
            abundance: 0.601080,
            neutrons: 69,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Gd"),
        most_abundant_isotope: 158,
        most_abundant_mass: 157.924104,
        element_number: 158,
        ..Default::default()
    };
    elt.isotopes.insert(
        152,
        Isotope {
            mass: 151.919791,
            abundance: 0.002000,
            neutrons: 152,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        154,
        Isotope {
            mass: 153.920866,
            abundance: 0.021800,
            neutrons: 154,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        155,
        Isotope {
            mass: 154.922622,
            abundance: 0.148000,
            neutrons: 155,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        157,
        Isotope {
            mass: 156.923960,
            abundance: 0.156500,
            neutrons: 157,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        156,
        Isotope {
            mass: 155.922123,
            abundance: 0.204700,
            neutrons: 156,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        160,
        Isotope {
            mass: 159.927054,
            abundance: 0.218600,
            neutrons: 160,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        158,
        Isotope {
            mass: 157.924104,
            abundance: 0.248400,
            neutrons: 158,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ge"),
        most_abundant_isotope: 74,
        most_abundant_mass: 73.921178,
        element_number: 74,
        ..Default::default()
    };
    elt.isotopes.insert(
        73,
        Isotope {
            mass: 72.923459,
            abundance: 0.077600,
            neutrons: 73,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        76,
        Isotope {
            mass: 75.921403,
            abundance: 0.078300,
            neutrons: 76,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        70,
        Isotope {
            mass: 69.924247,
            abundance: 0.203800,
            neutrons: 70,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        72,
        Isotope {
            mass: 71.922076,
            abundance: 0.273100,
            neutrons: 72,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        74,
        Isotope {
            mass: 73.921178,
            abundance: 0.367200,
            neutrons: 74,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("H"),
        most_abundant_isotope: 1,
        most_abundant_mass: 1.007825,
        element_number: 1,
        ..Default::default()
    };
    elt.isotopes.insert(
        2,
        Isotope {
            mass: 2.014102,
            abundance: 0.000115,
            neutrons: 2,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        1,
        Isotope {
            mass: 1.007825,
            abundance: 0.999885,
            neutrons: 1,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("H+"),
        most_abundant_isotope: 1,
        most_abundant_mass: 1.007276,
        element_number: 1,
        ..Default::default()
    };
    elt.isotopes.insert(
        1,
        Isotope {
            mass: 1.007276,
            abundance: 1.000000,
            neutrons: 1,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("He"),
        most_abundant_isotope: 4,
        most_abundant_mass: 4.002603,
        element_number: 4,
        ..Default::default()
    };
    elt.isotopes.insert(
        3,
        Isotope {
            mass: 3.016029,
            abundance: 0.000001,
            neutrons: 3,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        4,
        Isotope {
            mass: 4.002603,
            abundance: 0.999999,
            neutrons: 4,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Hf"),
        most_abundant_isotope: 180,
        most_abundant_mass: 179.946550,
        element_number: 180,
        ..Default::default()
    };
    elt.isotopes.insert(
        174,
        Isotope {
            mass: 173.940046,
            abundance: 0.001600,
            neutrons: 174,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        176,
        Isotope {
            mass: 175.941409,
            abundance: 0.052600,
            neutrons: 176,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        179,
        Isotope {
            mass: 178.945816,
            abundance: 0.136200,
            neutrons: 179,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        177,
        Isotope {
            mass: 176.943221,
            abundance: 0.186000,
            neutrons: 177,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        178,
        Isotope {
            mass: 177.943699,
            abundance: 0.272800,
            neutrons: 178,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        180,
        Isotope {
            mass: 179.946550,
            abundance: 0.350800,
            neutrons: 180,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Hg"),
        most_abundant_isotope: 202,
        most_abundant_mass: 201.970643,
        element_number: 202,
        ..Default::default()
    };
    elt.isotopes.insert(
        196,
        Isotope {
            mass: 195.965833,
            abundance: 0.001500,
            neutrons: 196,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        204,
        Isotope {
            mass: 203.973494,
            abundance: 0.068700,
            neutrons: 204,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        198,
        Isotope {
            mass: 197.966769,
            abundance: 0.099700,
            neutrons: 198,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        201,
        Isotope {
            mass: 200.970302,
            abundance: 0.131800,
            neutrons: 201,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        199,
        Isotope {
            mass: 198.968280,
            abundance: 0.168700,
            neutrons: 199,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        200,
        Isotope {
            mass: 199.968326,
            abundance: 0.231000,
            neutrons: 200,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        202,
        Isotope {
            mass: 201.970643,
            abundance: 0.298600,
            neutrons: 202,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ho"),
        most_abundant_isotope: 165,
        most_abundant_mass: 164.930322,
        element_number: 165,
        ..Default::default()
    };
    elt.isotopes.insert(
        165,
        Isotope {
            mass: 164.930322,
            abundance: 1.000000,
            neutrons: 165,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Hs"),
        most_abundant_isotope: 0,
        most_abundant_mass: 270.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 270.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("I"),
        most_abundant_isotope: 127,
        most_abundant_mass: 126.904473,
        element_number: 127,
        ..Default::default()
    };
    elt.isotopes.insert(
        127,
        Isotope {
            mass: 126.904473,
            abundance: 1.000000,
            neutrons: 127,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("In"),
        most_abundant_isotope: 115,
        most_abundant_mass: 114.903878,
        element_number: 115,
        ..Default::default()
    };
    elt.isotopes.insert(
        113,
        Isotope {
            mass: 112.904058,
            abundance: 0.042900,
            neutrons: 113,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        115,
        Isotope {
            mass: 114.903878,
            abundance: 0.957100,
            neutrons: 115,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ir"),
        most_abundant_isotope: 193,
        most_abundant_mass: 192.962926,
        element_number: 193,
        ..Default::default()
    };
    elt.isotopes.insert(
        191,
        Isotope {
            mass: 190.960594,
            abundance: 0.373000,
            neutrons: 191,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        193,
        Isotope {
            mass: 192.962926,
            abundance: 0.627000,
            neutrons: 193,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("K"),
        most_abundant_isotope: 39,
        most_abundant_mass: 38.963707,
        element_number: 39,
        ..Default::default()
    };
    elt.isotopes.insert(
        40,
        Isotope {
            mass: 39.963998,
            abundance: 0.000117,
            neutrons: 40,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        41,
        Isotope {
            mass: 40.961826,
            abundance: 0.067302,
            neutrons: 41,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        39,
        Isotope {
            mass: 38.963707,
            abundance: 0.932581,
            neutrons: 39,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Kr"),
        most_abundant_isotope: 84,
        most_abundant_mass: 83.911507,
        element_number: 84,
        ..Default::default()
    };
    elt.isotopes.insert(
        78,
        Isotope {
            mass: 77.920365,
            abundance: 0.003550,
            neutrons: 78,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        80,
        Isotope {
            mass: 79.916379,
            abundance: 0.022860,
            neutrons: 80,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        82,
        Isotope {
            mass: 81.913484,
            abundance: 0.115930,
            neutrons: 82,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        83,
        Isotope {
            mass: 82.914136,
            abundance: 0.115000,
            neutrons: 83,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        86,
        Isotope {
            mass: 85.910611,
            abundance: 0.172790,
            neutrons: 86,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        84,
        Isotope {
            mass: 83.911507,
            abundance: 0.569870,
            neutrons: 84,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("La"),
        most_abundant_isotope: 139,
        most_abundant_mass: 138.906353,
        element_number: 139,
        ..Default::default()
    };
    elt.isotopes.insert(
        138,
        Isotope {
            mass: 137.907112,
            abundance: 0.000900,
            neutrons: 138,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        139,
        Isotope {
            mass: 138.906353,
            abundance: 0.999100,
            neutrons: 139,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Li"),
        most_abundant_isotope: 7,
        most_abundant_mass: 7.016005,
        element_number: 7,
        ..Default::default()
    };
    elt.isotopes.insert(
        6,
        Isotope {
            mass: 6.015123,
            abundance: 0.075900,
            neutrons: 6,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        7,
        Isotope {
            mass: 7.016005,
            abundance: 0.924100,
            neutrons: 7,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Lr"),
        most_abundant_isotope: 0,
        most_abundant_mass: 262.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 262.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Lu"),
        most_abundant_isotope: 175,
        most_abundant_mass: 174.940772,
        element_number: 175,
        ..Default::default()
    };
    elt.isotopes.insert(
        176,
        Isotope {
            mass: 175.942686,
            abundance: 0.025900,
            neutrons: 176,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        175,
        Isotope {
            mass: 174.940772,
            abundance: 0.974100,
            neutrons: 175,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Md"),
        most_abundant_isotope: 0,
        most_abundant_mass: 258.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 258.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Mg"),
        most_abundant_isotope: 24,
        most_abundant_mass: 23.985042,
        element_number: 24,
        ..Default::default()
    };
    elt.isotopes.insert(
        25,
        Isotope {
            mass: 24.985837,
            abundance: 0.100000,
            neutrons: 25,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        26,
        Isotope {
            mass: 25.982593,
            abundance: 0.110100,
            neutrons: 26,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        24,
        Isotope {
            mass: 23.985042,
            abundance: 0.789900,
            neutrons: 24,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Mn"),
        most_abundant_isotope: 55,
        most_abundant_mass: 54.938045,
        element_number: 55,
        ..Default::default()
    };
    elt.isotopes.insert(
        55,
        Isotope {
            mass: 54.938045,
            abundance: 1.000000,
            neutrons: 55,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Mo"),
        most_abundant_isotope: 98,
        most_abundant_mass: 97.905408,
        element_number: 98,
        ..Default::default()
    };
    elt.isotopes.insert(
        94,
        Isotope {
            mass: 93.905088,
            abundance: 0.092300,
            neutrons: 94,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        100,
        Isotope {
            mass: 99.907477,
            abundance: 0.096700,
            neutrons: 100,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        97,
        Isotope {
            mass: 96.906021,
            abundance: 0.095600,
            neutrons: 97,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        92,
        Isotope {
            mass: 91.906811,
            abundance: 0.147700,
            neutrons: 92,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        95,
        Isotope {
            mass: 94.905842,
            abundance: 0.159000,
            neutrons: 95,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        96,
        Isotope {
            mass: 95.904680,
            abundance: 0.166800,
            neutrons: 96,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        98,
        Isotope {
            mass: 97.905408,
            abundance: 0.241900,
            neutrons: 98,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Mt"),
        most_abundant_isotope: 0,
        most_abundant_mass: 276.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 276.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("N"),
        most_abundant_isotope: 14,
        most_abundant_mass: 14.003074,
        element_number: 14,
        ..Default::default()
    };
    elt.isotopes.insert(
        15,
        Isotope {
            mass: 15.000109,
            abundance: 0.003640,
            neutrons: 15,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        14,
        Isotope {
            mass: 14.003074,
            abundance: 0.996360,
            neutrons: 14,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Na"),
        most_abundant_isotope: 23,
        most_abundant_mass: 22.989769,
        element_number: 23,
        ..Default::default()
    };
    elt.isotopes.insert(
        23,
        Isotope {
            mass: 22.989769,
            abundance: 1.000000,
            neutrons: 23,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Nb"),
        most_abundant_isotope: 93,
        most_abundant_mass: 92.906378,
        element_number: 93,
        ..Default::default()
    };
    elt.isotopes.insert(
        93,
        Isotope {
            mass: 92.906378,
            abundance: 1.000000,
            neutrons: 93,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Nd"),
        most_abundant_isotope: 142,
        most_abundant_mass: 141.907723,
        element_number: 142,
        ..Default::default()
    };
    elt.isotopes.insert(
        148,
        Isotope {
            mass: 147.916893,
            abundance: 0.057000,
            neutrons: 148,
            neutron_shift: 6,
        },
    );
    elt.isotopes.insert(
        150,
        Isotope {
            mass: 149.920891,
            abundance: 0.056000,
            neutrons: 150,
            neutron_shift: 8,
        },
    );
    elt.isotopes.insert(
        145,
        Isotope {
            mass: 144.912574,
            abundance: 0.083000,
            neutrons: 145,
            neutron_shift: 3,
        },
    );
    elt.isotopes.insert(
        143,
        Isotope {
            mass: 142.909814,
            abundance: 0.122000,
            neutrons: 143,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        146,
        Isotope {
            mass: 145.913117,
            abundance: 0.172000,
            neutrons: 146,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        144,
        Isotope {
            mass: 143.910087,
            abundance: 0.238000,
            neutrons: 144,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        142,
        Isotope {
            mass: 141.907723,
            abundance: 0.272000,
            neutrons: 142,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ne"),
        most_abundant_isotope: 20,
        most_abundant_mass: 19.992440,
        element_number: 20,
        ..Default::default()
    };
    elt.isotopes.insert(
        21,
        Isotope {
            mass: 20.993847,
            abundance: 0.002700,
            neutrons: 21,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        22,
        Isotope {
            mass: 21.991385,
            abundance: 0.092500,
            neutrons: 22,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        20,
        Isotope {
            mass: 19.992440,
            abundance: 0.904800,
            neutrons: 20,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ni"),
        most_abundant_isotope: 58,
        most_abundant_mass: 57.935343,
        element_number: 58,
        ..Default::default()
    };
    elt.isotopes.insert(
        61,
        Isotope {
            mass: 60.931056,
            abundance: 0.011399,
            neutrons: 61,
            neutron_shift: 3,
        },
    );
    elt.isotopes.insert(
        64,
        Isotope {
            mass: 63.927966,
            abundance: 0.009256,
            neutrons: 64,
            neutron_shift: 6,
        },
    );
    elt.isotopes.insert(
        62,
        Isotope {
            mass: 61.928345,
            abundance: 0.036345,
            neutrons: 62,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        60,
        Isotope {
            mass: 59.930786,
            abundance: 0.262231,
            neutrons: 60,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        58,
        Isotope {
            mass: 57.935343,
            abundance: 0.680769,
            neutrons: 58,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("No"),
        most_abundant_isotope: 0,
        most_abundant_mass: 259.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 259.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Np"),
        most_abundant_isotope: 0,
        most_abundant_mass: 237.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 237.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("O"),
        most_abundant_isotope: 16,
        most_abundant_mass: 15.994915,
        element_number: 16,
        ..Default::default()
    };
    elt.isotopes.insert(
        17,
        Isotope {
            mass: 16.999132,
            abundance: 0.000380,
            neutrons: 17,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        18,
        Isotope {
            mass: 17.999161,
            abundance: 0.002050,
            neutrons: 18,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        16,
        Isotope {
            mass: 15.994915,
            abundance: 0.997570,
            neutrons: 16,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Os"),
        most_abundant_isotope: 192,
        most_abundant_mass: 191.961481,
        element_number: 192,
        ..Default::default()
    };
    elt.isotopes.insert(
        184,
        Isotope {
            mass: 183.952489,
            abundance: 0.000200,
            neutrons: 184,
            neutron_shift: -8,
        },
    );
    elt.isotopes.insert(
        186,
        Isotope {
            mass: 185.953838,
            abundance: 0.015900,
            neutrons: 186,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        187,
        Isotope {
            mass: 186.955750,
            abundance: 0.019600,
            neutrons: 187,
            neutron_shift: -5,
        },
    );
    elt.isotopes.insert(
        188,
        Isotope {
            mass: 187.955838,
            abundance: 0.132400,
            neutrons: 188,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        189,
        Isotope {
            mass: 188.958147,
            abundance: 0.161500,
            neutrons: 189,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        190,
        Isotope {
            mass: 189.958447,
            abundance: 0.262600,
            neutrons: 190,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        192,
        Isotope {
            mass: 191.961481,
            abundance: 0.407800,
            neutrons: 192,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("P"),
        most_abundant_isotope: 31,
        most_abundant_mass: 30.973762,
        element_number: 31,
        ..Default::default()
    };
    elt.isotopes.insert(
        31,
        Isotope {
            mass: 30.973762,
            abundance: 1.000000,
            neutrons: 31,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Pa"),
        most_abundant_isotope: 231,
        most_abundant_mass: 231.035884,
        element_number: 231,
        ..Default::default()
    };
    elt.isotopes.insert(
        231,
        Isotope {
            mass: 231.035884,
            abundance: 1.000000,
            neutrons: 231,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Pb"),
        most_abundant_isotope: 208,
        most_abundant_mass: 207.976652,
        element_number: 208,
        ..Default::default()
    };
    elt.isotopes.insert(
        204,
        Isotope {
            mass: 203.973044,
            abundance: 0.014000,
            neutrons: 204,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        207,
        Isotope {
            mass: 206.975897,
            abundance: 0.221000,
            neutrons: 207,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        206,
        Isotope {
            mass: 205.974465,
            abundance: 0.241000,
            neutrons: 206,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        208,
        Isotope {
            mass: 207.976652,
            abundance: 0.524000,
            neutrons: 208,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Pd"),
        most_abundant_isotope: 106,
        most_abundant_mass: 105.903486,
        element_number: 106,
        ..Default::default()
    };
    elt.isotopes.insert(
        102,
        Isotope {
            mass: 101.905609,
            abundance: 0.010200,
            neutrons: 102,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        104,
        Isotope {
            mass: 103.904036,
            abundance: 0.111400,
            neutrons: 104,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        110,
        Isotope {
            mass: 109.905153,
            abundance: 0.117200,
            neutrons: 110,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        105,
        Isotope {
            mass: 104.905085,
            abundance: 0.223300,
            neutrons: 105,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        108,
        Isotope {
            mass: 107.903892,
            abundance: 0.264600,
            neutrons: 108,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        106,
        Isotope {
            mass: 105.903486,
            abundance: 0.273300,
            neutrons: 106,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Pm"),
        most_abundant_isotope: 0,
        most_abundant_mass: 145.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 145.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Po"),
        most_abundant_isotope: 0,
        most_abundant_mass: 209.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 209.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Pr"),
        most_abundant_isotope: 141,
        most_abundant_mass: 140.907653,
        element_number: 141,
        ..Default::default()
    };
    elt.isotopes.insert(
        141,
        Isotope {
            mass: 140.907653,
            abundance: 1.000000,
            neutrons: 141,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Pt"),
        most_abundant_isotope: 195,
        most_abundant_mass: 194.964791,
        element_number: 195,
        ..Default::default()
    };
    elt.isotopes.insert(
        190,
        Isotope {
            mass: 189.959932,
            abundance: 0.000140,
            neutrons: 190,
            neutron_shift: -5,
        },
    );
    elt.isotopes.insert(
        192,
        Isotope {
            mass: 191.961038,
            abundance: 0.007820,
            neutrons: 192,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        198,
        Isotope {
            mass: 197.967893,
            abundance: 0.071630,
            neutrons: 198,
            neutron_shift: 3,
        },
    );
    elt.isotopes.insert(
        196,
        Isotope {
            mass: 195.964952,
            abundance: 0.252420,
            neutrons: 196,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        194,
        Isotope {
            mass: 193.962680,
            abundance: 0.329670,
            neutrons: 194,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        195,
        Isotope {
            mass: 194.964791,
            abundance: 0.338320,
            neutrons: 195,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Pu"),
        most_abundant_isotope: 0,
        most_abundant_mass: 244.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 244.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ra"),
        most_abundant_isotope: 0,
        most_abundant_mass: 226.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 226.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Rb"),
        most_abundant_isotope: 85,
        most_abundant_mass: 84.911790,
        element_number: 85,
        ..Default::default()
    };
    elt.isotopes.insert(
        87,
        Isotope {
            mass: 86.909181,
            abundance: 0.278300,
            neutrons: 87,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        85,
        Isotope {
            mass: 84.911790,
            abundance: 0.721700,
            neutrons: 85,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Re"),
        most_abundant_isotope: 187,
        most_abundant_mass: 186.955753,
        element_number: 187,
        ..Default::default()
    };
    elt.isotopes.insert(
        185,
        Isotope {
            mass: 184.952955,
            abundance: 0.374000,
            neutrons: 185,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        187,
        Isotope {
            mass: 186.955753,
            abundance: 0.626000,
            neutrons: 187,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Rf"),
        most_abundant_isotope: 0,
        most_abundant_mass: 265.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 265.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Rg"),
        most_abundant_isotope: 0,
        most_abundant_mass: 280.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 280.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Rh"),
        most_abundant_isotope: 103,
        most_abundant_mass: 102.905504,
        element_number: 103,
        ..Default::default()
    };
    elt.isotopes.insert(
        103,
        Isotope {
            mass: 102.905504,
            abundance: 1.000000,
            neutrons: 103,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Rn"),
        most_abundant_isotope: 0,
        most_abundant_mass: 222.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 222.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ru"),
        most_abundant_isotope: 102,
        most_abundant_mass: 101.904349,
        element_number: 102,
        ..Default::default()
    };
    elt.isotopes.insert(
        98,
        Isotope {
            mass: 97.905287,
            abundance: 0.018700,
            neutrons: 98,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        96,
        Isotope {
            mass: 95.907598,
            abundance: 0.055400,
            neutrons: 96,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        100,
        Isotope {
            mass: 99.904219,
            abundance: 0.126000,
            neutrons: 100,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        99,
        Isotope {
            mass: 98.905939,
            abundance: 0.127600,
            neutrons: 99,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        101,
        Isotope {
            mass: 100.905582,
            abundance: 0.170600,
            neutrons: 101,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        104,
        Isotope {
            mass: 103.905433,
            abundance: 0.186200,
            neutrons: 104,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        102,
        Isotope {
            mass: 101.904349,
            abundance: 0.315500,
            neutrons: 102,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("S"),
        most_abundant_isotope: 32,
        most_abundant_mass: 31.972071,
        element_number: 32,
        ..Default::default()
    };
    elt.isotopes.insert(
        36,
        Isotope {
            mass: 35.967081,
            abundance: 0.000100,
            neutrons: 36,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        33,
        Isotope {
            mass: 32.971459,
            abundance: 0.007500,
            neutrons: 33,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        34,
        Isotope {
            mass: 33.967867,
            abundance: 0.042500,
            neutrons: 34,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        32,
        Isotope {
            mass: 31.972071,
            abundance: 0.949900,
            neutrons: 32,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Sb"),
        most_abundant_isotope: 121,
        most_abundant_mass: 120.903816,
        element_number: 121,
        ..Default::default()
    };
    elt.isotopes.insert(
        123,
        Isotope {
            mass: 122.904214,
            abundance: 0.427900,
            neutrons: 123,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        121,
        Isotope {
            mass: 120.903816,
            abundance: 0.572100,
            neutrons: 121,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Sc"),
        most_abundant_isotope: 45,
        most_abundant_mass: 44.955912,
        element_number: 45,
        ..Default::default()
    };
    elt.isotopes.insert(
        45,
        Isotope {
            mass: 44.955912,
            abundance: 1.000000,
            neutrons: 45,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Se"),
        most_abundant_isotope: 80,
        most_abundant_mass: 79.916521,
        element_number: 80,
        ..Default::default()
    };
    elt.isotopes.insert(
        74,
        Isotope {
            mass: 73.922476,
            abundance: 0.008900,
            neutrons: 74,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        77,
        Isotope {
            mass: 76.919914,
            abundance: 0.076300,
            neutrons: 77,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        76,
        Isotope {
            mass: 75.919214,
            abundance: 0.093700,
            neutrons: 76,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        82,
        Isotope {
            mass: 81.916699,
            abundance: 0.087300,
            neutrons: 82,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        78,
        Isotope {
            mass: 77.917309,
            abundance: 0.237700,
            neutrons: 78,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        80,
        Isotope {
            mass: 79.916521,
            abundance: 0.496100,
            neutrons: 80,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Sg"),
        most_abundant_isotope: 0,
        most_abundant_mass: 271.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 271.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Si"),
        most_abundant_isotope: 28,
        most_abundant_mass: 27.976927,
        element_number: 28,
        ..Default::default()
    };
    elt.isotopes.insert(
        30,
        Isotope {
            mass: 29.973770,
            abundance: 0.030920,
            neutrons: 30,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        29,
        Isotope {
            mass: 28.976495,
            abundance: 0.046850,
            neutrons: 29,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        28,
        Isotope {
            mass: 27.976927,
            abundance: 0.922230,
            neutrons: 28,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Sm"),
        most_abundant_isotope: 152,
        most_abundant_mass: 151.919732,
        element_number: 152,
        ..Default::default()
    };
    elt.isotopes.insert(
        144,
        Isotope {
            mass: 143.911999,
            abundance: 0.030700,
            neutrons: 144,
            neutron_shift: -8,
        },
    );
    elt.isotopes.insert(
        150,
        Isotope {
            mass: 149.917275,
            abundance: 0.073800,
            neutrons: 150,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        148,
        Isotope {
            mass: 147.914823,
            abundance: 0.112400,
            neutrons: 148,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        149,
        Isotope {
            mass: 148.917185,
            abundance: 0.138200,
            neutrons: 149,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        147,
        Isotope {
            mass: 146.914898,
            abundance: 0.149900,
            neutrons: 147,
            neutron_shift: -5,
        },
    );
    elt.isotopes.insert(
        154,
        Isotope {
            mass: 153.922209,
            abundance: 0.227500,
            neutrons: 154,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        152,
        Isotope {
            mass: 151.919732,
            abundance: 0.267500,
            neutrons: 152,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Sn"),
        most_abundant_isotope: 120,
        most_abundant_mass: 119.902195,
        element_number: 120,
        ..Default::default()
    };
    elt.isotopes.insert(
        115,
        Isotope {
            mass: 114.903342,
            abundance: 0.003400,
            neutrons: 115,
            neutron_shift: -5,
        },
    );
    elt.isotopes.insert(
        112,
        Isotope {
            mass: 111.904818,
            abundance: 0.009700,
            neutrons: 112,
            neutron_shift: -8,
        },
    );
    elt.isotopes.insert(
        114,
        Isotope {
            mass: 113.902779,
            abundance: 0.006600,
            neutrons: 114,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        122,
        Isotope {
            mass: 121.903439,
            abundance: 0.046300,
            neutrons: 122,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        124,
        Isotope {
            mass: 123.905274,
            abundance: 0.057900,
            neutrons: 124,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        117,
        Isotope {
            mass: 116.902952,
            abundance: 0.076800,
            neutrons: 117,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        119,
        Isotope {
            mass: 118.903308,
            abundance: 0.085900,
            neutrons: 119,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        116,
        Isotope {
            mass: 115.901741,
            abundance: 0.145400,
            neutrons: 116,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        118,
        Isotope {
            mass: 117.901603,
            abundance: 0.242200,
            neutrons: 118,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        120,
        Isotope {
            mass: 119.902195,
            abundance: 0.325800,
            neutrons: 120,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Sr"),
        most_abundant_isotope: 88,
        most_abundant_mass: 87.905612,
        element_number: 88,
        ..Default::default()
    };
    elt.isotopes.insert(
        84,
        Isotope {
            mass: 83.913425,
            abundance: 0.005600,
            neutrons: 84,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        87,
        Isotope {
            mass: 86.908877,
            abundance: 0.070000,
            neutrons: 87,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        86,
        Isotope {
            mass: 85.909260,
            abundance: 0.098600,
            neutrons: 86,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        88,
        Isotope {
            mass: 87.905612,
            abundance: 0.825800,
            neutrons: 88,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ta"),
        most_abundant_isotope: 181,
        most_abundant_mass: 180.947996,
        element_number: 181,
        ..Default::default()
    };
    elt.isotopes.insert(
        180,
        Isotope {
            mass: 179.947465,
            abundance: 0.000120,
            neutrons: 180,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        181,
        Isotope {
            mass: 180.947996,
            abundance: 0.999880,
            neutrons: 181,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Tb"),
        most_abundant_isotope: 159,
        most_abundant_mass: 158.925347,
        element_number: 159,
        ..Default::default()
    };
    elt.isotopes.insert(
        159,
        Isotope {
            mass: 158.925347,
            abundance: 1.000000,
            neutrons: 159,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Tc"),
        most_abundant_isotope: 0,
        most_abundant_mass: 98.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 98.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Te"),
        most_abundant_isotope: 130,
        most_abundant_mass: 129.906224,
        element_number: 130,
        ..Default::default()
    };
    elt.isotopes.insert(
        120,
        Isotope {
            mass: 119.904020,
            abundance: 0.000900,
            neutrons: 120,
            neutron_shift: -10,
        },
    );
    elt.isotopes.insert(
        123,
        Isotope {
            mass: 122.904270,
            abundance: 0.008900,
            neutrons: 123,
            neutron_shift: -7,
        },
    );
    elt.isotopes.insert(
        122,
        Isotope {
            mass: 121.903044,
            abundance: 0.025500,
            neutrons: 122,
            neutron_shift: -8,
        },
    );
    elt.isotopes.insert(
        124,
        Isotope {
            mass: 123.902818,
            abundance: 0.047400,
            neutrons: 124,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        125,
        Isotope {
            mass: 124.904431,
            abundance: 0.070700,
            neutrons: 125,
            neutron_shift: -5,
        },
    );
    elt.isotopes.insert(
        126,
        Isotope {
            mass: 125.903312,
            abundance: 0.188400,
            neutrons: 126,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        128,
        Isotope {
            mass: 127.904463,
            abundance: 0.317400,
            neutrons: 128,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        130,
        Isotope {
            mass: 129.906224,
            abundance: 0.340800,
            neutrons: 130,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Th"),
        most_abundant_isotope: 232,
        most_abundant_mass: 232.038055,
        element_number: 232,
        ..Default::default()
    };
    elt.isotopes.insert(
        232,
        Isotope {
            mass: 232.038055,
            abundance: 1.000000,
            neutrons: 232,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Ti"),
        most_abundant_isotope: 48,
        most_abundant_mass: 47.947946,
        element_number: 48,
        ..Default::default()
    };
    elt.isotopes.insert(
        49,
        Isotope {
            mass: 48.947870,
            abundance: 0.054100,
            neutrons: 49,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        50,
        Isotope {
            mass: 49.944791,
            abundance: 0.051800,
            neutrons: 50,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        47,
        Isotope {
            mass: 46.951763,
            abundance: 0.074400,
            neutrons: 47,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        46,
        Isotope {
            mass: 45.952632,
            abundance: 0.082500,
            neutrons: 46,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        48,
        Isotope {
            mass: 47.947946,
            abundance: 0.737200,
            neutrons: 48,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Tl"),
        most_abundant_isotope: 205,
        most_abundant_mass: 204.974427,
        element_number: 205,
        ..Default::default()
    };
    elt.isotopes.insert(
        203,
        Isotope {
            mass: 202.972344,
            abundance: 0.295200,
            neutrons: 203,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        205,
        Isotope {
            mass: 204.974427,
            abundance: 0.704800,
            neutrons: 205,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Tm"),
        most_abundant_isotope: 169,
        most_abundant_mass: 168.934213,
        element_number: 169,
        ..Default::default()
    };
    elt.isotopes.insert(
        169,
        Isotope {
            mass: 168.934213,
            abundance: 1.000000,
            neutrons: 169,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("U"),
        most_abundant_isotope: 238,
        most_abundant_mass: 238.050788,
        element_number: 238,
        ..Default::default()
    };
    elt.isotopes.insert(
        234,
        Isotope {
            mass: 234.040952,
            abundance: 0.000054,
            neutrons: 234,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        235,
        Isotope {
            mass: 235.043930,
            abundance: 0.007204,
            neutrons: 235,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        238,
        Isotope {
            mass: 238.050788,
            abundance: 0.992742,
            neutrons: 238,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Uuh"),
        most_abundant_isotope: 0,
        most_abundant_mass: 293.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 293.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Uuo"),
        most_abundant_isotope: 0,
        most_abundant_mass: 294.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 294.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Uup"),
        most_abundant_isotope: 0,
        most_abundant_mass: 288.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 288.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Uuq"),
        most_abundant_isotope: 0,
        most_abundant_mass: 289.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 289.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Uus"),
        most_abundant_isotope: 0,
        most_abundant_mass: 292.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 292.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Uut"),
        most_abundant_isotope: 0,
        most_abundant_mass: 284.000000,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 284.000000,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("V"),
        most_abundant_isotope: 51,
        most_abundant_mass: 50.943959,
        element_number: 51,
        ..Default::default()
    };
    elt.isotopes.insert(
        50,
        Isotope {
            mass: 49.947159,
            abundance: 0.002500,
            neutrons: 50,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        51,
        Isotope {
            mass: 50.943959,
            abundance: 0.997500,
            neutrons: 51,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("W"),
        most_abundant_isotope: 184,
        most_abundant_mass: 183.950931,
        element_number: 184,
        ..Default::default()
    };
    elt.isotopes.insert(
        180,
        Isotope {
            mass: 179.946704,
            abundance: 0.001200,
            neutrons: 180,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        183,
        Isotope {
            mass: 182.950223,
            abundance: 0.143100,
            neutrons: 183,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        182,
        Isotope {
            mass: 181.948204,
            abundance: 0.265000,
            neutrons: 182,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        186,
        Isotope {
            mass: 185.954364,
            abundance: 0.284300,
            neutrons: 186,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        184,
        Isotope {
            mass: 183.950931,
            abundance: 0.306400,
            neutrons: 184,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Xe"),
        most_abundant_isotope: 132,
        most_abundant_mass: 131.904154,
        element_number: 132,
        ..Default::default()
    };
    elt.isotopes.insert(
        124,
        Isotope {
            mass: 123.905893,
            abundance: 0.000952,
            neutrons: 124,
            neutron_shift: -8,
        },
    );
    elt.isotopes.insert(
        126,
        Isotope {
            mass: 125.904274,
            abundance: 0.000890,
            neutrons: 126,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        128,
        Isotope {
            mass: 127.903531,
            abundance: 0.019102,
            neutrons: 128,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        130,
        Isotope {
            mass: 129.903508,
            abundance: 0.040710,
            neutrons: 130,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        136,
        Isotope {
            mass: 135.907219,
            abundance: 0.088573,
            neutrons: 136,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        134,
        Isotope {
            mass: 133.905394,
            abundance: 0.104357,
            neutrons: 134,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        131,
        Isotope {
            mass: 130.905082,
            abundance: 0.212324,
            neutrons: 131,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        129,
        Isotope {
            mass: 128.904779,
            abundance: 0.264006,
            neutrons: 129,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        132,
        Isotope {
            mass: 131.904154,
            abundance: 0.269086,
            neutrons: 132,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Y"),
        most_abundant_isotope: 89,
        most_abundant_mass: 88.905848,
        element_number: 89,
        ..Default::default()
    };
    elt.isotopes.insert(
        89,
        Isotope {
            mass: 88.905848,
            abundance: 1.000000,
            neutrons: 89,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Yb"),
        most_abundant_isotope: 174,
        most_abundant_mass: 173.938862,
        element_number: 174,
        ..Default::default()
    };
    elt.isotopes.insert(
        168,
        Isotope {
            mass: 167.933897,
            abundance: 0.001300,
            neutrons: 168,
            neutron_shift: -6,
        },
    );
    elt.isotopes.insert(
        170,
        Isotope {
            mass: 169.934762,
            abundance: 0.030400,
            neutrons: 170,
            neutron_shift: -4,
        },
    );
    elt.isotopes.insert(
        176,
        Isotope {
            mass: 175.942572,
            abundance: 0.127600,
            neutrons: 176,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        171,
        Isotope {
            mass: 170.936326,
            abundance: 0.142800,
            neutrons: 171,
            neutron_shift: -3,
        },
    );
    elt.isotopes.insert(
        173,
        Isotope {
            mass: 172.938211,
            abundance: 0.161300,
            neutrons: 173,
            neutron_shift: -1,
        },
    );
    elt.isotopes.insert(
        172,
        Isotope {
            mass: 171.936382,
            abundance: 0.218300,
            neutrons: 172,
            neutron_shift: -2,
        },
    );
    elt.isotopes.insert(
        174,
        Isotope {
            mass: 173.938862,
            abundance: 0.318300,
            neutrons: 174,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Zn"),
        most_abundant_isotope: 64,
        most_abundant_mass: 63.929142,
        element_number: 64,
        ..Default::default()
    };
    elt.isotopes.insert(
        70,
        Isotope {
            mass: 69.925319,
            abundance: 0.006310,
            neutrons: 70,
            neutron_shift: 6,
        },
    );
    elt.isotopes.insert(
        67,
        Isotope {
            mass: 66.927127,
            abundance: 0.041020,
            neutrons: 67,
            neutron_shift: 3,
        },
    );
    elt.isotopes.insert(
        68,
        Isotope {
            mass: 67.924844,
            abundance: 0.190240,
            neutrons: 68,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        66,
        Isotope {
            mass: 65.926033,
            abundance: 0.279750,
            neutrons: 66,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        64,
        Isotope {
            mass: 63.929142,
            abundance: 0.482680,
            neutrons: 64,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("Zr"),
        most_abundant_isotope: 90,
        most_abundant_mass: 89.904704,
        element_number: 90,
        ..Default::default()
    };
    elt.isotopes.insert(
        96,
        Isotope {
            mass: 95.908273,
            abundance: 0.028000,
            neutrons: 96,
            neutron_shift: 6,
        },
    );
    elt.isotopes.insert(
        91,
        Isotope {
            mass: 90.905646,
            abundance: 0.112200,
            neutrons: 91,
            neutron_shift: 1,
        },
    );
    elt.isotopes.insert(
        92,
        Isotope {
            mass: 91.905041,
            abundance: 0.171500,
            neutrons: 92,
            neutron_shift: 2,
        },
    );
    elt.isotopes.insert(
        94,
        Isotope {
            mass: 93.906315,
            abundance: 0.173800,
            neutrons: 94,
            neutron_shift: 4,
        },
    );
    elt.isotopes.insert(
        90,
        Isotope {
            mass: 89.904704,
            abundance: 0.514500,
            neutrons: 90,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);

    let mut elt = Element {
        symbol: String::from("e*"),
        most_abundant_isotope: 0,
        most_abundant_mass: 0.000549,
        ..Default::default()
    };
    elt.isotopes.insert(
        0,
        Isotope {
            mass: 0.000549,
            abundance: 1.000000,
            neutrons: 0,
            neutron_shift: 0,
        },
    );
    elt.index_isotopes();
    table.add(elt);
}

pub static PERIODIC_TABLE: LazyLock<PeriodicTable> = LazyLock::new(|| {
    let mut t = PeriodicTable::new();
    populate_periodic_table(&mut t);
    t
});