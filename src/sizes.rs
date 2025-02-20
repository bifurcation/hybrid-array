//! Macros for defining various array sizes, and their associated invocations.

use super::{ArraySize, AssocArraySize};
use typenum::consts::*;

/// Additional typenum size aliases beyond what are normally provided.
///
/// These are defined using their component bits rather than `Add` to avoid conflicting impls.
#[rustfmt::skip]
pub mod extra_sizes {
    use typenum::{UInt, UTerm, B0, B1};

    pub type U1088 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type U1152 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type U1184 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type U1472 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type U1536 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type U1568 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type U1600 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type U1632 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type U2336 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type U2368 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type U2400 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type U3072 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type U3104 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type U3136 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type U3168 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
}

pub use extra_sizes::*;

macro_rules! impl_array_size {
    ($($len:expr => $ty:ident),+) => {
        $(
            unsafe impl ArraySize for $ty {
                type ArrayType<T> = [T; $len];
            }

            impl<T> AssocArraySize for [T; $len] {
                type Size = $ty;
            }
        )+
     };
}

impl_array_size! {
    0 => U0,
    1 => U1,
    2 => U2,
    3 => U3,
    4 => U4,
    5 => U5,
    6 => U6,
    7 => U7,
    8 => U8,
    9 => U9,
    10 => U10,
    11 => U11,
    12 => U12,
    13 => U13,
    14 => U14,
    15 => U15,
    16 => U16,
    17 => U17,
    18 => U18,
    19 => U19,
    20 => U20,
    21 => U21,
    22 => U22,
    23 => U23,
    24 => U24,
    25 => U25,
    26 => U26,
    27 => U27,
    28 => U28,
    29 => U29,
    30 => U30,
    31 => U31,
    32 => U32,
    33 => U33,
    34 => U34,
    35 => U35,
    36 => U36,
    37 => U37,
    38 => U38,
    39 => U39,
    40 => U40,
    41 => U41,
    42 => U42,
    43 => U43,
    44 => U44,
    45 => U45,
    46 => U46,
    47 => U47,
    48 => U48,
    49 => U49,
    50 => U50,
    51 => U51,
    52 => U52,
    53 => U53,
    54 => U54,
    55 => U55,
    56 => U56,
    57 => U57,
    58 => U58,
    59 => U59,
    60 => U60,
    61 => U61,
    62 => U62,
    63 => U63,
    64 => U64,
    65 => U65,
    66 => U66,
    67 => U67,
    68 => U68,
    69 => U69,
    70 => U70,
    71 => U71,
    72 => U72,
    73 => U73,
    74 => U74,
    75 => U75,
    76 => U76,
    77 => U77,
    78 => U78,
    79 => U79,
    80 => U80,
    81 => U81,
    82 => U82,
    83 => U83,
    84 => U84,
    85 => U85,
    86 => U86,
    87 => U87,
    88 => U88,
    89 => U89,
    90 => U90,
    91 => U91,
    92 => U92,
    93 => U93,
    94 => U94,
    95 => U95,
    96 => U96,
    97 => U97,
    98 => U98,
    99 => U99,
    100 => U100,
    101 => U101,
    102 => U102,
    103 => U103,
    104 => U104,
    105 => U105,
    106 => U106,
    107 => U107,
    108 => U108,
    109 => U109,
    110 => U110,
    111 => U111,
    112 => U112,
    113 => U113,
    114 => U114,
    115 => U115,
    116 => U116,
    117 => U117,
    118 => U118,
    119 => U119,
    120 => U120,
    121 => U121,
    122 => U122,
    123 => U123,
    124 => U124,
    125 => U125,
    126 => U126,
    127 => U127,
    128 => U128,
    129 => U129,
    130 => U130,
    131 => U131,
    132 => U132,
    133 => U133,
    134 => U134,
    135 => U135,
    136 => U136,
    137 => U137,
    138 => U138,
    139 => U139,
    140 => U140,
    141 => U141,
    142 => U142,
    143 => U143,
    144 => U144,
    145 => U145,
    146 => U146,
    147 => U147,
    148 => U148,
    149 => U149,
    150 => U150,
    151 => U151,
    152 => U152,
    153 => U153,
    154 => U154,
    155 => U155,
    156 => U156,
    157 => U157,
    158 => U158,
    159 => U159,
    160 => U160,
    161 => U161,
    162 => U162,
    163 => U163,
    164 => U164,
    165 => U165,
    166 => U166,
    167 => U167,
    168 => U168,
    169 => U169,
    170 => U170,
    171 => U171,
    172 => U172,
    173 => U173,
    174 => U174,
    175 => U175,
    176 => U176,
    177 => U177,
    178 => U178,
    179 => U179,
    180 => U180,
    181 => U181,
    182 => U182,
    183 => U183,
    184 => U184,
    185 => U185,
    186 => U186,
    187 => U187,
    188 => U188,
    189 => U189,
    190 => U190,
    191 => U191,
    192 => U192,
    193 => U193,
    194 => U194,
    195 => U195,
    196 => U196,
    197 => U197,
    198 => U198,
    199 => U199,
    200 => U200,
    201 => U201,
    202 => U202,
    203 => U203,
    204 => U204,
    205 => U205,
    206 => U206,
    207 => U207,
    208 => U208,
    209 => U209,
    210 => U210,
    211 => U211,
    212 => U212,
    213 => U213,
    214 => U214,
    215 => U215,
    216 => U216,
    217 => U217,
    218 => U218,
    219 => U219,
    220 => U220,
    221 => U221,
    222 => U222,
    223 => U223,
    224 => U224,
    225 => U225,
    226 => U226,
    227 => U227,
    228 => U228,
    229 => U229,
    230 => U230,
    231 => U231,
    232 => U232,
    233 => U233,
    234 => U234,
    235 => U235,
    236 => U236,
    237 => U237,
    238 => U238,
    239 => U239,
    240 => U240,
    241 => U241,
    242 => U242,
    243 => U243,
    244 => U244,
    245 => U245,
    246 => U246,
    247 => U247,
    248 => U248,
    249 => U249,
    250 => U250,
    251 => U251,
    252 => U252,
    253 => U253,
    254 => U254,
    255 => U255,
    256 => U256,
    272 => U272,
    288 => U288,
    304 => U304,
    320 => U320,
    336 => U336,
    352 => U352,
    368 => U368,
    384 => U384,
    400 => U400,
    416 => U416,
    432 => U432,
    448 => U448,
    464 => U464,
    480 => U480,
    496 => U496,
    512 => U512,
    528 => U528,
    544 => U544,
    560 => U560,
    576 => U576,
    592 => U592,
    608 => U608,
    624 => U624,
    640 => U640,
    656 => U656,
    672 => U672,
    688 => U688,
    704 => U704,
    720 => U720,
    736 => U736,
    752 => U752,
    768 => U768,
    784 => U784,
    800 => U800,
    816 => U816,
    832 => U832,
    848 => U848,
    864 => U864,
    880 => U880,
    896 => U896,
    912 => U912,
    928 => U928,
    944 => U944,
    960 => U960,
    976 => U976,
    992 => U992,
    1008 => U1008,
    1024 => U1024,
    1088 => U1088,
    1152 => U1152,
    1184 => U1184,
    1472 => U1472,
    1536 => U1536,
    1568 => U1568,
    1600 => U1600,
    1632 => U1632,
    2048 => U2048,
    2336 => U2336,
    2368 => U2368,
    2400 => U2400,
    3072 => U3072,
    3104 => U3104,
    3136 => U3136,
    3168 => U3168,
    4096 => U4096
}
