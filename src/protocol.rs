#![allow(unused)]

use once_cell::sync::Lazy;
use std::collections::HashMap;

pub type ProtocolNum = u16;

pub const DEFAULT_PORT: u16 = 25565;

pub const MINECRAFT_1_7: ProtocolNum = 3;
pub const MINECRAFT_1_7_1: ProtocolNum = 3;

pub const MINECRAFT_1_7_2: ProtocolNum = 4;
pub const MINECRAFT_1_7_3: ProtocolNum = 4;
pub const MINECRAFT_1_7_4: ProtocolNum = 4;
pub const MINECRAFT_1_7_5: ProtocolNum = 4;

pub const MINECRAFT_1_7_6: ProtocolNum = 5;
pub const MINECRAFT_1_7_7: ProtocolNum = 5;
pub const MINECRAFT_1_7_8: ProtocolNum = 5;
pub const MINECRAFT_1_7_9: ProtocolNum = 5;
pub const MINECRAFT_1_7_10: ProtocolNum = 5;

pub const MINECRAFT_1_8: ProtocolNum = 47;
pub const MINECRAFT_1_8_1: ProtocolNum = 47;
pub const MINECRAFT_1_8_2: ProtocolNum = 47;
pub const MINECRAFT_1_8_3: ProtocolNum = 47;
pub const MINECRAFT_1_8_4: ProtocolNum = 47;
pub const MINECRAFT_1_8_5: ProtocolNum = 47;
pub const MINECRAFT_1_8_6: ProtocolNum = 47;
pub const MINECRAFT_1_8_7: ProtocolNum = 47;
pub const MINECRAFT_1_8_8: ProtocolNum = 47;
pub const MINECRAFT_1_8_9: ProtocolNum = 47;

pub const MINECRAFT_1_9: ProtocolNum = 107;
pub const MINECRAFT_1_9_1: ProtocolNum = 108;
pub const MINECRAFT_1_9_2: ProtocolNum = 109;
pub const MINECRAFT_1_9_3: ProtocolNum = 110;
pub const MINECRAFT_1_9_4: ProtocolNum = 110;

pub const MINECRAFT_1_10: ProtocolNum = 210;
pub const MINECRAFT_1_10_1: ProtocolNum = 210;
pub const MINECRAFT_1_10_2: ProtocolNum = 210;

pub const MINECRAFT_1_11: ProtocolNum = 315;
pub const MINECRAFT_1_11_1: ProtocolNum = 316;
pub const MINECRAFT_1_11_2: ProtocolNum = 316;

pub const MINECRAFT_1_12: ProtocolNum = 335;
pub const MINECRAFT_1_12_1: ProtocolNum = 338;
pub const MINECRAFT_1_12_2: ProtocolNum = 340;

pub const MINECRAFT_1_13: ProtocolNum = 393;
pub const MINECRAFT_1_13_1: ProtocolNum = 401;
pub const MINECRAFT_1_13_2: ProtocolNum = 404;

pub const MINECRAFT_1_14: ProtocolNum = 477;
pub const MINECRAFT_1_14_1: ProtocolNum = 480;
pub const MINECRAFT_1_14_2: ProtocolNum = 485;
pub const MINECRAFT_1_14_3: ProtocolNum = 490;
pub const MINECRAFT_1_14_4: ProtocolNum = 498;

pub const MINECRAFT_1_15: ProtocolNum = 573;
pub const MINECRAFT_1_15_1: ProtocolNum = 575;
pub const MINECRAFT_1_15_2: ProtocolNum = 578;

pub const MINECRAFT_1_16: ProtocolNum = 735;
pub const MINECRAFT_1_16_1: ProtocolNum = 736;
pub const MINECRAFT_1_16_2: ProtocolNum = 751;
pub const MINECRAFT_1_16_3: ProtocolNum = 753;
pub const MINECRAFT_1_16_4: ProtocolNum = 754;
pub const MINECRAFT_1_16_5: ProtocolNum = 754;

pub const MINECRAFT_1_17: ProtocolNum = 755;
pub const MINECRAFT_1_17_1: ProtocolNum = 756;

pub const MINECRAFT_1_18: ProtocolNum = 757;
pub const MINECRAFT_1_18_1: ProtocolNum = 757;
pub const MINECRAFT_1_18_2: ProtocolNum = 758;

pub const MINECRAFT_1_19: ProtocolNum = 759;
pub const MINECRAFT_1_19_1: ProtocolNum = 760;
pub const MINECRAFT_1_19_2: ProtocolNum = 760;
pub const MINECRAFT_1_19_3: ProtocolNum = 761;
pub const MINECRAFT_1_19_4: ProtocolNum = 762;

pub const MINECRAFT_1_20: ProtocolNum = 763;
pub const MINECRAFT_1_20_1: ProtocolNum = 763;
pub const MINECRAFT_1_20_2: ProtocolNum = 764;
pub const MINECRAFT_1_20_3: ProtocolNum = 765;
pub const MINECRAFT_1_20_4: ProtocolNum = 765;
pub const MINECRAFT_1_20_5: ProtocolNum = 766;
pub const MINECRAFT_1_20_6: ProtocolNum = 766;

pub const MINECRAFT_1_21: ProtocolNum = 767;
pub const MINECRAFT_1_21_1: ProtocolNum = 767;
pub const MINECRAFT_1_21_2: ProtocolNum = 768;
pub const MINECRAFT_1_21_3: ProtocolNum = 768;
pub const MINECRAFT_1_21_4: ProtocolNum = 769;
pub const MINECRAFT_1_21_5: ProtocolNum = 770;
pub const MINECRAFT_1_21_6: ProtocolNum = 771;
pub const MINECRAFT_1_21_7: ProtocolNum = 772;
pub const MINECRAFT_1_21_8: ProtocolNum = 772;
pub const MINECRAFT_1_21_9: ProtocolNum = 773;
pub const MINECRAFT_1_21_10: ProtocolNum = 773;
pub const MINECRAFT_1_21_11: ProtocolNum = 774;

pub const LATEST: ProtocolNum = MINECRAFT_1_21_11;

static VERSION_TO_PROTONUM: Lazy<HashMap<&str, ProtocolNum>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert("1.7", MINECRAFT_1_7);
    m.insert("1.7.1", MINECRAFT_1_7_1);
    m.insert("1.7.2", MINECRAFT_1_7_2);
    m.insert("1.7.3", MINECRAFT_1_7_3);
    m.insert("1.7.4", MINECRAFT_1_7_4);
    m.insert("1.7.5", MINECRAFT_1_7_5);
    m.insert("1.7.6", MINECRAFT_1_7_6);
    m.insert("1.7.7", MINECRAFT_1_7_7);
    m.insert("1.7.8", MINECRAFT_1_7_8);
    m.insert("1.7.9", MINECRAFT_1_7_9);
    m.insert("1.7.10", MINECRAFT_1_7_10);

    m.insert("1.8", MINECRAFT_1_8);
    m.insert("1.8.1", MINECRAFT_1_8_1);
    m.insert("1.8.2", MINECRAFT_1_8_2);
    m.insert("1.8.3", MINECRAFT_1_8_3);
    m.insert("1.8.4", MINECRAFT_1_8_4);
    m.insert("1.8.5", MINECRAFT_1_8_5);
    m.insert("1.8.6", MINECRAFT_1_8_6);
    m.insert("1.8.7", MINECRAFT_1_8_7);
    m.insert("1.8.8", MINECRAFT_1_8_8);
    m.insert("1.8.9", MINECRAFT_1_8_9);

    m.insert("1.9", MINECRAFT_1_9);
    m.insert("1.9.1", MINECRAFT_1_9_1);
    m.insert("1.9.2", MINECRAFT_1_9_2);
    m.insert("1.9.3", MINECRAFT_1_9_3);
    m.insert("1.9.4", MINECRAFT_1_9_4);

    m.insert("1.10", MINECRAFT_1_10);
    m.insert("1.10.1", MINECRAFT_1_10_1);
    m.insert("1.10.2", MINECRAFT_1_10_2);

    m.insert("1.11", MINECRAFT_1_11);
    m.insert("1.11.1", MINECRAFT_1_11_1);
    m.insert("1.11.2", MINECRAFT_1_11_2);

    m.insert("1.12", MINECRAFT_1_12);
    m.insert("1.12.1", MINECRAFT_1_12_1);
    m.insert("1.12.2", MINECRAFT_1_12_2);

    m.insert("1.13", MINECRAFT_1_13);
    m.insert("1.13.1", MINECRAFT_1_13_1);
    m.insert("1.13.2", MINECRAFT_1_13_2);

    m.insert("1.14", MINECRAFT_1_14);
    m.insert("1.14.1", MINECRAFT_1_14_1);
    m.insert("1.14.2", MINECRAFT_1_14_2);
    m.insert("1.14.3", MINECRAFT_1_14_3);
    m.insert("1.14.4", MINECRAFT_1_14_4);

    m.insert("1.15", MINECRAFT_1_15);
    m.insert("1.15.1", MINECRAFT_1_15_1);
    m.insert("1.15.2", MINECRAFT_1_15_2);

    m.insert("1.16", MINECRAFT_1_16);
    m.insert("1.16.1", MINECRAFT_1_16_1);
    m.insert("1.16.2", MINECRAFT_1_16_2);
    m.insert("1.16.3", MINECRAFT_1_16_3);
    m.insert("1.16.4", MINECRAFT_1_16_4);
    m.insert("1.16.5", MINECRAFT_1_16_5);

    m.insert("1.17", MINECRAFT_1_17);
    m.insert("1.17.1", MINECRAFT_1_17_1);

    m.insert("1.18", MINECRAFT_1_18);
    m.insert("1.18.1", MINECRAFT_1_18_1);
    m.insert("1.18.2", MINECRAFT_1_18_2);

    m.insert("1.19", MINECRAFT_1_19);
    m.insert("1.19.1", MINECRAFT_1_19_1);
    m.insert("1.19.2", MINECRAFT_1_19_2);
    m.insert("1.19.3", MINECRAFT_1_19_3);
    m.insert("1.19.4", MINECRAFT_1_19_4);

    m.insert("1.20", MINECRAFT_1_20);
    m.insert("1.20.1", MINECRAFT_1_20_1);
    m.insert("1.20.2", MINECRAFT_1_20_2);
    m.insert("1.20.3", MINECRAFT_1_20_3);
    m.insert("1.20.4", MINECRAFT_1_20_4);
    m.insert("1.20.5", MINECRAFT_1_20_5);
    m.insert("1.20.6", MINECRAFT_1_20_6);

    m.insert("1.21", MINECRAFT_1_21);
    m.insert("1.21.1", MINECRAFT_1_21_1);
    m.insert("1.21.2", MINECRAFT_1_21_2);
    m.insert("1.21.3", MINECRAFT_1_21_3);
    m.insert("1.21.4", MINECRAFT_1_21_4);
    m.insert("1.21.5", MINECRAFT_1_21_5);
    m.insert("1.21.6", MINECRAFT_1_21_6);
    m.insert("1.21.7", MINECRAFT_1_21_7);
    m.insert("1.21.8", MINECRAFT_1_21_8);
    m.insert("1.21.9", MINECRAFT_1_21_9);
    m.insert("1.21.10", MINECRAFT_1_21_10);
    m.insert("1.21.11", MINECRAFT_1_21_11);

    m
});

pub fn get_protocol_number_by_version(ver: &str) -> Option<ProtocolNum> {
    VERSION_TO_PROTONUM.get(ver).copied()
}

pub fn is_known_protocol_number(n: ProtocolNum) -> bool {
    matches!(
        n,
        MINECRAFT_1_7_1
            | MINECRAFT_1_7_5
            | MINECRAFT_1_7_10
            | MINECRAFT_1_8_9
            | MINECRAFT_1_9
            | MINECRAFT_1_9_1
            | MINECRAFT_1_9_2
            | MINECRAFT_1_9_4
            | MINECRAFT_1_10_2
            | MINECRAFT_1_11
            | MINECRAFT_1_11_2
            | MINECRAFT_1_12
            | MINECRAFT_1_12_1
            | MINECRAFT_1_12_2
            | MINECRAFT_1_13
            | MINECRAFT_1_13_1
            | MINECRAFT_1_13_2
            | MINECRAFT_1_14
            | MINECRAFT_1_14_1
            | MINECRAFT_1_14_2
            | MINECRAFT_1_14_3
            | MINECRAFT_1_14_4
            | MINECRAFT_1_15
            | MINECRAFT_1_15_1
            | MINECRAFT_1_15_2
            | MINECRAFT_1_16
            | MINECRAFT_1_16_1
            | MINECRAFT_1_16_2
            | MINECRAFT_1_16_3
            | MINECRAFT_1_16_5
            | MINECRAFT_1_17
            | MINECRAFT_1_17_1
            | MINECRAFT_1_18_1
            | MINECRAFT_1_18_2
            | MINECRAFT_1_19
            | MINECRAFT_1_19_2
            | MINECRAFT_1_19_3
            | MINECRAFT_1_19_4
            | MINECRAFT_1_20_1
            | MINECRAFT_1_20_2
            | MINECRAFT_1_20_4
            | MINECRAFT_1_20_6
            | MINECRAFT_1_21_1
            | MINECRAFT_1_21_3
            | MINECRAFT_1_21_4
            | MINECRAFT_1_21_5
            | MINECRAFT_1_21_6
            | MINECRAFT_1_21_8
            | MINECRAFT_1_21_10
            | MINECRAFT_1_21_11
    )
}
