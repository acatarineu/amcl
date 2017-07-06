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

//
//  rom.swift
//
//  Created by Michael Scott on 12/06/2015.
//  Copyright (c) 2015 Michael Scott. All rights reserved.
//

final public class ROM{
 
#if D32

// Base Bits= 28
//  Brainpool Modulus
static let Modulus:[Chunk] = [0xF6E5377,0x13481D1,0x6202820,0xF623D52,0xD726E3B,0x909D838,0xC3E660A,0xA1EEA9B,0x9FB57DB,0xA]
static let R2modp:[Chunk] = [0xB9A3787,0x9E04F49,0x8F3CF49,0x2931721,0xF1DBC89,0x54E8C3C,0xF7559CA,0xBB411A3,0x773E15F,0x9]
static let MConst:Chunk = 0xEFD89B9

//  Brainpool Curve
static let CURVE_A:Int = -3
static let CURVE_B_I:Int = 0
static let CURVE_B:[Chunk] = [0xEE92B04,0xE58101F,0xF49256A,0xEBC4AF2,0x6B7BF93,0x733D0B7,0x4FE66A7,0x30D84EA,0x62C61C4,0x6]
static public let CURVE_Order:[Chunk] = [0x74856A7,0x1E0E829,0x1A6F790,0x7AA3B56,0xD718C39,0x909D838,0xC3E660A,0xA1EEA9B,0x9FB57DB,0xA]
static public let CURVE_Gx:[Chunk] = [0xE1305F4,0xA191562,0xFBC2B79,0x42C47AA,0x149AFA1,0xB23A656,0x7732213,0xC1CFE7B,0x3E8EB3C,0xA]
static public let CURVE_Gy:[Chunk] = [0xB25C9BE,0xABE8F35,0x27001D,0xB6DE39D,0x17E69BC,0xE146444,0xD7F7B22,0x3439C56,0xD996C82,0x2]

#endif

#if D64

// Base Bits= 56
//  Brainpool  Modulus
static let Modulus:[Chunk] = [0x13481D1F6E5377,0xF623D526202820,0x909D838D726E3B,0xA1EEA9BC3E660A,0xA9FB57DB]
static let R2modp:[Chunk] = [0x9E04F49B9A3787,0x29317218F3CF49,0x54E8C3CF1DBC89,0xBB411A3F7559CA,0x9773E15F]
static let MConst:Chunk = 0xA75590CEFD89B9

//  Brainpool Curve
static let CURVE_A:Int = -3
static let CURVE_B_I:Int = 0
static let CURVE_B:[Chunk] = [0xE58101FEE92B04,0xEBC4AF2F49256A,0x733D0B76B7BF93,0x30D84EA4FE66A7,0x662C61C4]
static public let CURVE_Order:[Chunk] = [0x1E0E82974856A7,0x7AA3B561A6F790,0x909D838D718C39,0xA1EEA9BC3E660A,0xA9FB57DB]
static public let CURVE_Gx:[Chunk] = [0xA191562E1305F4,0x42C47AAFBC2B79,0xB23A656149AFA1,0xC1CFE7B7732213,0xA3E8EB3C]
static public let CURVE_Gy:[Chunk] = [0xABE8F35B25C9BE,0xB6DE39D027001D,0xE14644417E69BC,0x3439C56D7F7B22,0x2D996C82]


#endif

}

