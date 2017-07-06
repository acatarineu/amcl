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

/* Fixed Data in ROM - Field and Curve parameters */


package amcl.BN254CX;

public class ROM
{
// Base Bits= 28
	public static final int[] Modulus= {0xC1B55B3,0x6623EF5,0x93EE1BE,0xD6EE180,0x6D3243F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2};
	public static final int[] R2modp= {0x8A0800A,0x466A061,0x43056A3,0x2B3A225,0x9C6600,0x148515B,0x6BDF50,0xEC9EA56,0xC992E66,0x1};
	public static final int MConst= 0x9789E85;

	public static final int CURVE_A= 0;
	public static final int CURVE_B_I= 2;
	public static final int[] CURVE_B= {0x2,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0};
	public static final int[] CURVE_Order= {0x6EB1F6D,0x11C0A63,0x906CEBE,0xD6EE0CC,0x6D2C43F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2};
	public static final int[] CURVE_Gx= {0xC1B55B2,0x6623EF5,0x93EE1BE,0xD6EE180,0x6D3243F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2};
	public static final int[] CURVE_Gy= {0x1,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0};

	public static final int[] CURVE_Bnx= {0x3C012B1,0x0,0x40,0x0,0x0,0x0,0x0,0x0,0x0,0x0};
	public static final int[] CURVE_Cof= {0x1,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0};
	public static final int[] CURVE_Cru= {0x4235C97,0xE093179,0xF875631,0xDF6471E,0xF1440BD,0xCA83,0x480000,0x0,0x0,0x0};
	public static final int[] Fra= {0x5C80EA3,0xD908335,0x3F8215B,0x7326F17,0x8986867,0x8AACA71,0x4AFE18B,0xA63A016,0x359082F,0x1};
	public static final int[] Frb= {0x6534710,0x8D1BBC0,0x546C062,0x63C7269,0xE3ABBD8,0xD9CDBC4,0x900DC53,0x623628A,0xA6F7D0,0x1};
	public static final int[] CURVE_Pxa= {0xB5FAAF5,0xC3E806C,0x9362851,0xB8E61B9,0x2F0944F,0x73CB19C,0xFA7121C,0x7F9A9B4,0xAC95A5B,0x1};
	public static final int[] CURVE_Pxb= {0xC15F433,0xE50060F,0xA094DED,0x33A1ABD,0x64C1BBA,0xD1681F3,0x43F11C2,0x365660,0x53D3001,0x0};
	public static final int[] CURVE_Pya= {0x772A299,0x33216A9,0x3484E68,0xF0936EA,0x6479DF,0xFEF9184,0x2EE06BB,0xB09E6FB,0x5727970,0x1};
	public static final int[] CURVE_Pyb= {0xC49E8CD,0x1BC11F9,0x61ADC4,0x56091A,0x166D320,0x7F56070,0xFD57BD7,0x2AC189,0x1BCB56C,0x1};
	public static final int[][] CURVE_W= {{0x62FEB83,0x5463491,0x381200,0xB4,0x6000,0x0,0x0,0x0,0x0,0x0},{0x7802561,0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0}};
	public static final int[][][] CURVE_SB= {{{0xDB010E4,0x5463491,0x381280,0xB4,0x6000,0x0,0x0,0x0,0x0,0x0},{0x7802561,0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0}},{{0x7802561,0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0},{0xBB33EA,0xBD5D5D2,0x8CEBCBD,0xD6EE018,0x6D2643F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2}}};
	public static final int[][] CURVE_WB= {{0x67A84B0,0x1C21185,0x12B040,0x3C,0x2000,0x0,0x0,0x0,0x0,0x0},{0xE220475,0xCDF995B,0xA7F9A36,0x94EDA8C,0xA0DC07E,0x8702,0x300000,0x0,0x0,0x0},{0xF10B93,0x66FCCAE,0x53FCD3B,0x4A76D46,0x506E03F,0x4381,0x180000,0x0,0x0,0x0},{0xDFAAA11,0x1C21185,0x12B0C0,0x3C,0x2000,0x0,0x0,0x0,0x0,0x0}};
	public static final int[][][] CURVE_BB= {{{0x32B0CBD,0x11C0A63,0x906CE7E,0xD6EE0CC,0x6D2C43F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2},{0x32B0CBC,0x11C0A63,0x906CE7E,0xD6EE0CC,0x6D2C43F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2},{0x32B0CBC,0x11C0A63,0x906CE7E,0xD6EE0CC,0x6D2C43F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2},{0x7802562,0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0}},{{0x7802561,0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0},{0x32B0CBC,0x11C0A63,0x906CE7E,0xD6EE0CC,0x6D2C43F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2},{0x32B0CBD,0x11C0A63,0x906CE7E,0xD6EE0CC,0x6D2C43F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2},{0x32B0CBC,0x11C0A63,0x906CE7E,0xD6EE0CC,0x6D2C43F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2}},{{0x7802562,0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0},{0x7802561,0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0},{0x7802561,0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0},{0x7802561,0x0,0x80,0x0,0x0,0x0,0x0,0x0,0x0,0x0}},{{0x3C012B2,0x0,0x40,0x0,0x0,0x0,0x0,0x0,0x0,0x0},{0xF004AC2,0x0,0x100,0x0,0x0,0x0,0x0,0x0,0x0,0x0},{0xF6AFA0A,0x11C0A62,0x906CE3E,0xD6EE0CC,0x6D2C43F,0x647A636,0xDB0BDDF,0x8702A0,0x4000000,0x2},{0x3C012B2,0x0,0x40,0x0,0x0,0x0,0x0,0x0,0x0,0x0}}};

}
