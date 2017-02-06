#include "arch.h"
#include "ecp_HIFIVE.h"

/* Curve HIFIVE */

#if CHUNK==32
const int CURVE_A_HIFIVE=1;
const BIG_336 CURVE_Order_HIFIVE= {0x1E9FA805,0x197CACB9,0x1E4EEA9E,0x17AD70F,0x1FA9850C,0x38A0A,0x0,0x0,0x0,0x0,0x0,0x4000};
const BIG_336 CURVE_B_HIFIVE= {0x2B67};
const BIG_336 CURVE_Gx_HIFIVE= {0xC};
const BIG_336 CURVE_Gy_HIFIVE= {0x5FE8632,0x15F63428,0xD976C4,0x1AACA194,0x35B6DB5,0x8E3F7A,0x52D1B0E,0xF0A7A36,0x1C161D00,0x8170C70,0x1185AD59,0x181B};
#endif

#if CHUNK==64
const int CURVE_A_HIFIVE=1;
const BIG_336 CURVE_Order_HIFIVE= {0xB2F95973E9FA805,0xC0BD6B87F93BAA7,0x71415FA9850,0x0,0x0,0x200000000};
const BIG_336 CURVE_B_HIFIVE= {0x2B67};
const BIG_336 CURVE_Gx_HIFIVE= {0xC};
const BIG_336 CURVE_Gy_HIFIVE= {0x2BEC68505FE8632,0x5D5650CA0365DB1,0x3811C7EF435B6DB,0x7853D1B14B46C,0x56502E18E1C161D,0xC0DC616B};
#endif


