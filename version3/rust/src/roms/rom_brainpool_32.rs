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

/* Note that the original curve has been transformed to an isomorphic curve with A=-3 */

use brainpool::big::NLEN;
use super::arch::Chunk;
use std;
use types::{ModType, CurveType, CurvePairingType, SexticTwist, SignOfX};

// Base Bits= 28
// brainpool Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0xF6E5377, 0x13481D1, 0x6202820, 0xF623D52, 0xD726E3B, 0x909D838, 0xC3E660A, 0xA1EEA9B,
    0x9FB57DB, 0xA,
];
pub const R2MODP: [Chunk; NLEN] = [
    0xB9A3787, 0x9E04F49, 0x8F3CF49, 0x2931721, 0xF1DBC89, 0x54E8C3C, 0xF7559CA, 0xBB411A3,
    0x773E15F, 0x9,
];
pub const MCONST: Chunk = 0xEFD89B9;

// brainpool Curve
pub const CURVE_COF_I: isize = 1;
pub const CURVE_A: isize = -3;
pub const CURVE_B_I: isize = 0;
pub const CURVE_COF: [Chunk; NLEN] = [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B: [Chunk; NLEN] = [
    0xEE92B04, 0xE58101F, 0xF49256A, 0xEBC4AF2, 0x6B7BF93, 0x733D0B7, 0x4FE66A7, 0x30D84EA,
    0x62C61C4, 0x6,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x74856A7, 0x1E0E829, 0x1A6F790, 0x7AA3B56, 0xD718C39, 0x909D838, 0xC3E660A, 0xA1EEA9B,
    0x9FB57DB, 0xA,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xE1305F4, 0xA191562, 0xFBC2B79, 0x42C47AA, 0x149AFA1, 0xB23A656, 0x7732213, 0xC1CFE7B,
    0x3E8EB3C, 0xA,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0xB25C9BE, 0xABE8F35, 0x27001D, 0xB6DE39D, 0x17E69BC, 0xE146444, 0xD7F7B22, 0x3439C56,
    0xD996C82, 0x2,
];

pub const MODBYTES: usize = 32;
pub const BASEBITS: usize = 28;

pub const MODBITS: usize = 256;
pub const MOD8: usize = 7;
pub const MODTYPE: ModType = ModType::NOT_SPECIAL;
pub const SH: usize = std::cmp::min(14, BASEBITS*(1+((8*MODBYTES-1)/BASEBITS))-MODBITS);

pub const CURVETYPE: CurveType = CurveType::WEIERSTRASS;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::NOT;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::NOT;
pub const SIGN_OF_X: SignOfX = SignOfX::NOT;
pub const HASH_TYPE: usize = 32;
pub const AESKEY: usize = 16;
