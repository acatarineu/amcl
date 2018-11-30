/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use bls383::big::NLEN;
use super::arch::Chunk;
use std;
use types::{ModType, CurveType, CurvePairingType, SexticTwist, SignOfX};

// Base Bits= 29
pub const MODULUS: [Chunk; NLEN] = [
    0x5AAB0AB, 0x11B8EB24, 0x19214AF6, 0x187E5314, 0x124F47A8, 0x1C00B4B0, 0x1446B0C6, 0x59E6CB4,
    0x4A0AD46, 0xFF5494, 0x81B6B71, 0x956DD6B, 0x16556956, 0x2A,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x116907F4, 0x405B700, 0x1752AC11, 0x67A9E7C, 0x1941C581, 0x1AEA38C4, 0xB1E4D22, 0xCE841AE,
    0xA0FC49B, 0xB4B1F48, 0x13852312, 0x1B3FDCED, 0x1FECE397, 0x26,
];
pub const MCONST: Chunk = 0x73435FD;
pub const FRA: [Chunk; NLEN] = [
    0x1311DAC1, 0x296B969, 0x19DCF806, 0x126901FC, 0xD8C8A36, 0x1A2572A8, 0xA1A0959, 0x1A47F743,
    0x110E4C6C, 0x1608DA97, 0xCE2E7F0, 0x4FED178, 0xACD5BF0, 0x11,
];
pub const FRB: [Chunk; NLEN] = [
    0x1298D5EA, 0xF2231BA, 0x1F4452F0, 0x6155117, 0x4C2BD72, 0x1DB4208, 0xA2CA76D, 0xB567571,
    0x139260D9, 0xAF679FC, 0x1B388380, 0x4580BF2, 0xB880D66, 0x19,
];

pub const CURVE_A: isize = 0;
pub const CURVE_COF_I: isize = 0;
pub const CURVE_COF: [Chunk; NLEN] = [
    0x15169EAB, 0xA82AB0A, 0xAAEFFED, 0x15558001, 0x555, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0,
];
pub const CURVE_B_I: isize = 15;
pub const CURVE_B: [Chunk; NLEN] = [
    0xF, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x1EBC0001, 0x1904CF5F, 0x834E5CE, 0xBE12B42, 0xB381DE0, 0xE40B4C, 0x270110, 0x10018017,
    0x1002001, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0x8734573, 0x623B9C8, 0x1D1DC11E, 0xBB7E107, 0x1E3445C5, 0x1D6C2578, 0x10B0BE1E, 0xED6103E,
    0x10F31D9F, 0x296ED82, 0x18E0D7D0, 0x12F3D9C9, 0x1FCBA55B, 0x20,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x3F224, 0x968B2F4, 0x1FE63F48, 0xFA93D90, 0x14D2DDE5, 0x54A56F5, 0x12441D4C, 0x18CD76C8,
    0x199D0DAD, 0xE18E236, 0x92BA73, 0x99F6600, 0x8F16727, 0x3,
];

pub const CURVE_BNX: [Chunk; NLEN] = [
    0x1001200, 0x400000, 0x40, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_CRU: [Chunk; NLEN] = [
    0xEAAC2A9, 0x61B3A81, 0x17D974B7, 0xBED0345, 0xA341BC2, 0x17A51A6F, 0x5738948, 0x69B7BAE,
    0x14605445, 0x374A43, 0x8116AD1, 0x956DD69, 0x16556956, 0x2A,
];
pub const CURVE_PXA: [Chunk; NLEN] = [
    0xD7F2D86, 0x1E59DB1, 0x17474F85, 0x1FB56CF2, 0x572EE81, 0xE487AB1, 0x96F51FC, 0x190A5AAE,
    0x6432501, 0x13E58F3A, 0x101E6425, 0xFD807D1, 0x34D2240, 0x3,
];
pub const CURVE_PXB: [Chunk; NLEN] = [
    0x452DE15, 0x1ECF20F6, 0x1FF9837B, 0x95651AA, 0xD5D75B5, 0x5D44749, 0x12277F66, 0x1DB3A0B9,
    0x1D24F498, 0x19441B0E, 0x1CDE9DC5, 0x2C975, 0xD78006, 0x18,
];
pub const CURVE_PYA: [Chunk; NLEN] = [
    0x1408CB41, 0x34785DC, 0x3586597, 0x13DBC9E4, 0x1A2E75B4, 0x1D65489, 0xCF9A25E, 0x1ACE7933,
    0x1B6E990E, 0x19FF31A3, 0x12527615, 0x1A44A68F, 0x1792CF93, 0x19,
];
pub const CURVE_PYB: [Chunk; NLEN] = [
    0x1F479093, 0x16C2321B, 0x1889218E, 0x87961BC, 0x1BC98B01, 0x197A24FB, 0xA3DEBC2, 0x88D67DF,
    0x1CE0D, 0x1E8AD3D7, 0x93B9EE9, 0x59B18D6, 0xE5247DD, 0x10,
];
pub const CURVE_W: [[Chunk; NLEN]; 2] = [
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
];
pub const CURVE_SB: [[[Chunk; NLEN]; 2]; 2] = [
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
];
pub const CURVE_WB: [[Chunk; NLEN]; 4] = [
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
];
pub const CURVE_BB: [[[Chunk; NLEN]; 4]; 4] = [
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
];

pub const USE_GLV: bool = true;
pub const USE_GS_G2: bool = true;
pub const USE_GS_GT: bool = true;
pub const GT_STRONG: bool = true;

pub const MODBYTES: usize = 48;
pub const BASEBITS: usize = 29;

pub const MODBITS: usize = 383;
pub const MOD8: usize = 3;
pub const MODTYPE: ModType = ModType::NOT_SPECIAL;
pub const SH: usize = std::cmp::min(14, BASEBITS*(1+((8*MODBYTES-1)/BASEBITS))-MODBITS);

pub const CURVETYPE: CurveType = CurveType::WEIERSTRASS;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::BLS;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::M_TYPE;
pub const SIGN_OF_X: SignOfX = SignOfX::POSITIVEX;
pub const HASH_TYPE: usize = 32;
pub const AESKEY: usize = 16;
