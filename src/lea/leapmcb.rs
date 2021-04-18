#[doc = "Reader of register LEAPMCB"]
pub type R = crate::R<u32, super::LEAPMCB>;
#[doc = "Writer for register LEAPMCB"]
pub type W = crate::W<u32, super::LEAPMCB>;
#[doc = "Register LEAPMCB `reset()`'s with value 0"]
impl crate::ResetValue for super::LEAPMCB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "1:0\\]
LEA instruction handshake synchronization type flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEAITFLG_A {
    #[doc = "0: LEA command without any further indication"]
    LEAITFLG_0 = 0,
    #[doc = "1: LEA command with explicit result update"]
    LEAITFLG_1 = 1,
    #[doc = "2: LEA command with interrupt upon completion"]
    LEAITFLG_2 = 2,
    #[doc = "3: LEA command with interrupt and explicit result update"]
    LEAITFLG_3 = 3,
}
impl From<LEAITFLG_A> for u8 {
    #[inline(always)]
    fn from(variant: LEAITFLG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEAITFLG`"]
pub type LEAITFLG_R = crate::R<u8, LEAITFLG_A>;
impl LEAITFLG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAITFLG_A {
        match self.bits {
            0 => LEAITFLG_A::LEAITFLG_0,
            1 => LEAITFLG_A::LEAITFLG_1,
            2 => LEAITFLG_A::LEAITFLG_2,
            3 => LEAITFLG_A::LEAITFLG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEAITFLG_0`"]
    #[inline(always)]
    pub fn is_leaitflg_0(&self) -> bool {
        *self == LEAITFLG_A::LEAITFLG_0
    }
    #[doc = "Checks if the value of the field is `LEAITFLG_1`"]
    #[inline(always)]
    pub fn is_leaitflg_1(&self) -> bool {
        *self == LEAITFLG_A::LEAITFLG_1
    }
    #[doc = "Checks if the value of the field is `LEAITFLG_2`"]
    #[inline(always)]
    pub fn is_leaitflg_2(&self) -> bool {
        *self == LEAITFLG_A::LEAITFLG_2
    }
    #[doc = "Checks if the value of the field is `LEAITFLG_3`"]
    #[inline(always)]
    pub fn is_leaitflg_3(&self) -> bool {
        *self == LEAITFLG_A::LEAITFLG_3
    }
}
#[doc = "Write proxy for field `LEAITFLG`"]
pub struct LEAITFLG_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAITFLG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAITFLG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LEA command without any further indication"]
    #[inline(always)]
    pub fn leaitflg_0(self) -> &'a mut W {
        self.variant(LEAITFLG_A::LEAITFLG_0)
    }
    #[doc = "LEA command with explicit result update"]
    #[inline(always)]
    pub fn leaitflg_1(self) -> &'a mut W {
        self.variant(LEAITFLG_A::LEAITFLG_1)
    }
    #[doc = "LEA command with interrupt upon completion"]
    #[inline(always)]
    pub fn leaitflg_2(self) -> &'a mut W {
        self.variant(LEAITFLG_A::LEAITFLG_2)
    }
    #[doc = "LEA command with interrupt and explicit result update"]
    #[inline(always)]
    pub fn leaitflg_3(self) -> &'a mut W {
        self.variant(LEAITFLG_A::LEAITFLG_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "9:2\\]
These bits represent the LEA command to be invoked. See also the command table\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEACMD_A {
    #[doc = "0: Suspends ongoing action an enters Ready"]
    SUSPEND = 0,
    #[doc = "2: Resumes an previously suspended command execution"]
    RESUME = 2,
    #[doc = "4: Complex FFT on 16 bit fractional numbers fix scaling"]
    FFTCOMPLEXFIXEDSCALING = 4,
    #[doc = "6: Real FIR on 16 bit fractional numbers"]
    FIR = 6,
    #[doc = "8: Real vector polynomial calculations 16 args all fractional"]
    POLYNOMIAL = 8,
    #[doc = "10: Real FFT-extension on 16 bit fractional numbers"]
    FFT = 10,
    #[doc = "12: Real vector polynomial calculations 32 bit args all fractional"]
    POLYNOMIALLONG = 12,
    #[doc = "13: Real row oriented matrix multiply"]
    MPYMATRIXROW = 13,
    #[doc = "15: Real matrix multiply 16 with 16 to 16 bit fractional"]
    MPYMATRIX = 15,
    #[doc = "16: Real point wise matrix add of 16 and 16 to 16 bit number vector"]
    ADDMATRIX = 16,
    #[doc = "17: Real maximum value and position of 16 bit matrices"]
    MAXMATRIX = 17,
    #[doc = "18: Real minimum value and position of 16 bit matrices"]
    MINMATRIX = 18,
    #[doc = "19: Real second order biquad using DF1 with 16 bit fractional"]
    IIRBQ1 = 19,
    #[doc = "21: Real matrix MAC short with 16Bt to 16B fract"]
    MAC = 21,
    #[doc = "22: Split 16B vector even to even words"]
    DEINTERLEAVEEVENEVEN = 22,
    #[doc = "23: Split 16Bt vector even to odd words"]
    DEINTERLEAVEEVENODD = 23,
    #[doc = "24: Split 16B vector odd to even words"]
    DEINTERLEAVEODDEVEN = 24,
    #[doc = "25: Split 16B vector odd to odd words"]
    DEINTERLEAVEODDODD = 25,
    #[doc = "26: Complex Dot Product"]
    MACCOMPLEXMATRIX = 26,
    #[doc = "27: Complex conjugate Dot Product"]
    MACCOMPLEXCONJUGATEMATRIX = 27,
    #[doc = "28: Real point wise matrix Subtraction of 16 and 16 to 16 bit"]
    SUBMATRIX = 28,
    #[doc = "29: Real point wise matrix multiply 32 with 32 to 32 bit fractional"]
    MPYLONGMATRIX = 29,
    #[doc = "30: Complex point wise matrix multiply complex with complex"]
    MPYCOMPLEXMATRIX = 30,
    #[doc = "31: Real point wise matrix add of 32 and 32 to 32 bit number"]
    ADDLONGMATRIX = 31,
    #[doc = "32: List move 32 to 32 bit"]
    MOVELONGLIST = 32,
    #[doc = "33: Complex bit reversal for 16 bit fractional numbers even"]
    BITREVERSECOMPLEXEVEN = 33,
    #[doc = "34: Complex bit reversal for 16 bit fractional Numbers odd"]
    BITREVERSECOMPLEXODD = 34,
    #[doc = "36: Real second order biquad using DF2 with 16 bit fractional, extended to include bias and intermediate state min/max"]
    IIRBQ2EXTENDED = 36,
    #[doc = "39: Complex FFT on 32B bit fractional numbers, fix scaling"]
    FFTCOMPLEXLONG = 39,
    #[doc = "41: Real FFT-extension on 32 bit fractional numbers"]
    FFTLONG = 41,
    #[doc = "43: Complex bit reversal for 32 bit fractional numbers even"]
    BITREVERSECOMPLEXLONGEVEN = 43,
    #[doc = "45: Complex bit reversal for 16 bit fractional numbers odd"]
    BITREVERSECOMPLEXLONGODD = 45,
    #[doc = "47: Scalar Polynomial for math on 32bit fractional"]
    POLYNOMIALSCALAR = 47,
    #[doc = "48: Complex FFT on 16B bit fractional numbers with auto scaling for enhanced accuracy"]
    FFTCOMPLEXAUTOSCALING = 48,
    #[doc = "50: Real FIR on 32 bit fractional numbers"]
    FIRLONG = 50,
    #[doc = "52: Real block MAC on 32B fractional numbers"]
    MACLONGMATRIX = 52,
    #[doc = "53: Real point wise matrix Subtraction of 32 and 32 to 32 bit"]
    SUBLONGMATRIX = 53,
    #[doc = "54: Real maximum value and position of signed 32B matrices"]
    MAXLONGMATRIX = 54,
    #[doc = "55: Real minimum value and position of signed 32B matrices"]
    MINLONGMATRIX = 55,
    #[doc = "56: Complex FIR on 16B fractional numbers"]
    FIRCOMPLEX = 56,
    #[doc = "58: Real maximum value and position of unsigned 16B matrices"]
    MAXUNSIGNEDMATRIX = 58,
    #[doc = "59: Real minimum value and position of unsigned 32B matrices"]
    MINUNSIGNEDMATRIX = 59,
    #[doc = "64: Real Matrix MAC on 16B fractional"]
    MACMATRIX = 64,
    #[doc = "65: Vector maximum on 16B signed numbers"]
    MAX = 65,
    #[doc = "66: Vector minimum on 16B signed numbers"]
    MIN = 66,
    #[doc = "67: Vector maximum on 16B unsigned numbers"]
    MAXUNSIGNED = 67,
    #[doc = "68: Vector minimum on 16B unsigned numbers"]
    MINUNSIGNED = 68,
    #[doc = "69: Matrix maximum on 32B unsigned numbers"]
    MAXUNSIGNEDLONGMATRIX = 69,
    #[doc = "70: Matrix minimum on 32B unsigned numbers"]
    MINUNSIGNEDLONGMATRIX = 70,
    #[doc = "71: Real second order biquad using DF2 with 16 bit fractional"]
    IIRBQ2 = 71,
    #[doc = "73: Complex FIR on 32B fractional numbers"]
    FIRCOMPLEXLONG = 73,
    #[doc = "75: Split Function on 32B Vectors/Matrices"]
    DEINTERLEAVELONG = 75,
    #[doc = "76: In-place symmetrical window on 16B fractional numbers"]
    WINDOW = 76,
    #[doc = "77: Vector MAC at three points, real 16-bit with 32-bit result"]
    MAC3 = 77,
    #[doc = "78: Scaled vector multiply and accumulate (MAC)"]
    SCALEDMAC = 78,
    #[doc = "79: Scaled FIR, 16-bit real fractional numbers"]
    SCALEDFIR = 79,
}
impl From<LEACMD_A> for u8 {
    #[inline(always)]
    fn from(variant: LEACMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEACMD`"]
pub type LEACMD_R = crate::R<u8, LEACMD_A>;
impl LEACMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LEACMD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LEACMD_A::SUSPEND),
            2 => Val(LEACMD_A::RESUME),
            4 => Val(LEACMD_A::FFTCOMPLEXFIXEDSCALING),
            6 => Val(LEACMD_A::FIR),
            8 => Val(LEACMD_A::POLYNOMIAL),
            10 => Val(LEACMD_A::FFT),
            12 => Val(LEACMD_A::POLYNOMIALLONG),
            13 => Val(LEACMD_A::MPYMATRIXROW),
            15 => Val(LEACMD_A::MPYMATRIX),
            16 => Val(LEACMD_A::ADDMATRIX),
            17 => Val(LEACMD_A::MAXMATRIX),
            18 => Val(LEACMD_A::MINMATRIX),
            19 => Val(LEACMD_A::IIRBQ1),
            21 => Val(LEACMD_A::MAC),
            22 => Val(LEACMD_A::DEINTERLEAVEEVENEVEN),
            23 => Val(LEACMD_A::DEINTERLEAVEEVENODD),
            24 => Val(LEACMD_A::DEINTERLEAVEODDEVEN),
            25 => Val(LEACMD_A::DEINTERLEAVEODDODD),
            26 => Val(LEACMD_A::MACCOMPLEXMATRIX),
            27 => Val(LEACMD_A::MACCOMPLEXCONJUGATEMATRIX),
            28 => Val(LEACMD_A::SUBMATRIX),
            29 => Val(LEACMD_A::MPYLONGMATRIX),
            30 => Val(LEACMD_A::MPYCOMPLEXMATRIX),
            31 => Val(LEACMD_A::ADDLONGMATRIX),
            32 => Val(LEACMD_A::MOVELONGLIST),
            33 => Val(LEACMD_A::BITREVERSECOMPLEXEVEN),
            34 => Val(LEACMD_A::BITREVERSECOMPLEXODD),
            36 => Val(LEACMD_A::IIRBQ2EXTENDED),
            39 => Val(LEACMD_A::FFTCOMPLEXLONG),
            41 => Val(LEACMD_A::FFTLONG),
            43 => Val(LEACMD_A::BITREVERSECOMPLEXLONGEVEN),
            45 => Val(LEACMD_A::BITREVERSECOMPLEXLONGODD),
            47 => Val(LEACMD_A::POLYNOMIALSCALAR),
            48 => Val(LEACMD_A::FFTCOMPLEXAUTOSCALING),
            50 => Val(LEACMD_A::FIRLONG),
            52 => Val(LEACMD_A::MACLONGMATRIX),
            53 => Val(LEACMD_A::SUBLONGMATRIX),
            54 => Val(LEACMD_A::MAXLONGMATRIX),
            55 => Val(LEACMD_A::MINLONGMATRIX),
            56 => Val(LEACMD_A::FIRCOMPLEX),
            58 => Val(LEACMD_A::MAXUNSIGNEDMATRIX),
            59 => Val(LEACMD_A::MINUNSIGNEDMATRIX),
            64 => Val(LEACMD_A::MACMATRIX),
            65 => Val(LEACMD_A::MAX),
            66 => Val(LEACMD_A::MIN),
            67 => Val(LEACMD_A::MAXUNSIGNED),
            68 => Val(LEACMD_A::MINUNSIGNED),
            69 => Val(LEACMD_A::MAXUNSIGNEDLONGMATRIX),
            70 => Val(LEACMD_A::MINUNSIGNEDLONGMATRIX),
            71 => Val(LEACMD_A::IIRBQ2),
            73 => Val(LEACMD_A::FIRCOMPLEXLONG),
            75 => Val(LEACMD_A::DEINTERLEAVELONG),
            76 => Val(LEACMD_A::WINDOW),
            77 => Val(LEACMD_A::MAC3),
            78 => Val(LEACMD_A::SCALEDMAC),
            79 => Val(LEACMD_A::SCALEDFIR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == LEACMD_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == LEACMD_A::RESUME
    }
    #[doc = "Checks if the value of the field is `FFTCOMPLEXFIXEDSCALING`"]
    #[inline(always)]
    pub fn is_fftcomplexfixedscaling(&self) -> bool {
        *self == LEACMD_A::FFTCOMPLEXFIXEDSCALING
    }
    #[doc = "Checks if the value of the field is `FIR`"]
    #[inline(always)]
    pub fn is_fir(&self) -> bool {
        *self == LEACMD_A::FIR
    }
    #[doc = "Checks if the value of the field is `POLYNOMIAL`"]
    #[inline(always)]
    pub fn is_polynomial(&self) -> bool {
        *self == LEACMD_A::POLYNOMIAL
    }
    #[doc = "Checks if the value of the field is `FFT`"]
    #[inline(always)]
    pub fn is_fft(&self) -> bool {
        *self == LEACMD_A::FFT
    }
    #[doc = "Checks if the value of the field is `POLYNOMIALLONG`"]
    #[inline(always)]
    pub fn is_polynomiallong(&self) -> bool {
        *self == LEACMD_A::POLYNOMIALLONG
    }
    #[doc = "Checks if the value of the field is `MPYMATRIXROW`"]
    #[inline(always)]
    pub fn is_mpymatrixrow(&self) -> bool {
        *self == LEACMD_A::MPYMATRIXROW
    }
    #[doc = "Checks if the value of the field is `MPYMATRIX`"]
    #[inline(always)]
    pub fn is_mpymatrix(&self) -> bool {
        *self == LEACMD_A::MPYMATRIX
    }
    #[doc = "Checks if the value of the field is `ADDMATRIX`"]
    #[inline(always)]
    pub fn is_addmatrix(&self) -> bool {
        *self == LEACMD_A::ADDMATRIX
    }
    #[doc = "Checks if the value of the field is `MAXMATRIX`"]
    #[inline(always)]
    pub fn is_maxmatrix(&self) -> bool {
        *self == LEACMD_A::MAXMATRIX
    }
    #[doc = "Checks if the value of the field is `MINMATRIX`"]
    #[inline(always)]
    pub fn is_minmatrix(&self) -> bool {
        *self == LEACMD_A::MINMATRIX
    }
    #[doc = "Checks if the value of the field is `IIRBQ1`"]
    #[inline(always)]
    pub fn is_iirbq1(&self) -> bool {
        *self == LEACMD_A::IIRBQ1
    }
    #[doc = "Checks if the value of the field is `MAC`"]
    #[inline(always)]
    pub fn is_mac(&self) -> bool {
        *self == LEACMD_A::MAC
    }
    #[doc = "Checks if the value of the field is `DEINTERLEAVEEVENEVEN`"]
    #[inline(always)]
    pub fn is_deinterleaveeveneven(&self) -> bool {
        *self == LEACMD_A::DEINTERLEAVEEVENEVEN
    }
    #[doc = "Checks if the value of the field is `DEINTERLEAVEEVENODD`"]
    #[inline(always)]
    pub fn is_deinterleaveevenodd(&self) -> bool {
        *self == LEACMD_A::DEINTERLEAVEEVENODD
    }
    #[doc = "Checks if the value of the field is `DEINTERLEAVEODDEVEN`"]
    #[inline(always)]
    pub fn is_deinterleaveoddeven(&self) -> bool {
        *self == LEACMD_A::DEINTERLEAVEODDEVEN
    }
    #[doc = "Checks if the value of the field is `DEINTERLEAVEODDODD`"]
    #[inline(always)]
    pub fn is_deinterleaveoddodd(&self) -> bool {
        *self == LEACMD_A::DEINTERLEAVEODDODD
    }
    #[doc = "Checks if the value of the field is `MACCOMPLEXMATRIX`"]
    #[inline(always)]
    pub fn is_maccomplexmatrix(&self) -> bool {
        *self == LEACMD_A::MACCOMPLEXMATRIX
    }
    #[doc = "Checks if the value of the field is `MACCOMPLEXCONJUGATEMATRIX`"]
    #[inline(always)]
    pub fn is_maccomplexconjugatematrix(&self) -> bool {
        *self == LEACMD_A::MACCOMPLEXCONJUGATEMATRIX
    }
    #[doc = "Checks if the value of the field is `SUBMATRIX`"]
    #[inline(always)]
    pub fn is_submatrix(&self) -> bool {
        *self == LEACMD_A::SUBMATRIX
    }
    #[doc = "Checks if the value of the field is `MPYLONGMATRIX`"]
    #[inline(always)]
    pub fn is_mpylongmatrix(&self) -> bool {
        *self == LEACMD_A::MPYLONGMATRIX
    }
    #[doc = "Checks if the value of the field is `MPYCOMPLEXMATRIX`"]
    #[inline(always)]
    pub fn is_mpycomplexmatrix(&self) -> bool {
        *self == LEACMD_A::MPYCOMPLEXMATRIX
    }
    #[doc = "Checks if the value of the field is `ADDLONGMATRIX`"]
    #[inline(always)]
    pub fn is_addlongmatrix(&self) -> bool {
        *self == LEACMD_A::ADDLONGMATRIX
    }
    #[doc = "Checks if the value of the field is `MOVELONGLIST`"]
    #[inline(always)]
    pub fn is_movelonglist(&self) -> bool {
        *self == LEACMD_A::MOVELONGLIST
    }
    #[doc = "Checks if the value of the field is `BITREVERSECOMPLEXEVEN`"]
    #[inline(always)]
    pub fn is_bitreversecomplexeven(&self) -> bool {
        *self == LEACMD_A::BITREVERSECOMPLEXEVEN
    }
    #[doc = "Checks if the value of the field is `BITREVERSECOMPLEXODD`"]
    #[inline(always)]
    pub fn is_bitreversecomplexodd(&self) -> bool {
        *self == LEACMD_A::BITREVERSECOMPLEXODD
    }
    #[doc = "Checks if the value of the field is `IIRBQ2EXTENDED`"]
    #[inline(always)]
    pub fn is_iirbq2extended(&self) -> bool {
        *self == LEACMD_A::IIRBQ2EXTENDED
    }
    #[doc = "Checks if the value of the field is `FFTCOMPLEXLONG`"]
    #[inline(always)]
    pub fn is_fftcomplexlong(&self) -> bool {
        *self == LEACMD_A::FFTCOMPLEXLONG
    }
    #[doc = "Checks if the value of the field is `FFTLONG`"]
    #[inline(always)]
    pub fn is_fftlong(&self) -> bool {
        *self == LEACMD_A::FFTLONG
    }
    #[doc = "Checks if the value of the field is `BITREVERSECOMPLEXLONGEVEN`"]
    #[inline(always)]
    pub fn is_bitreversecomplexlongeven(&self) -> bool {
        *self == LEACMD_A::BITREVERSECOMPLEXLONGEVEN
    }
    #[doc = "Checks if the value of the field is `BITREVERSECOMPLEXLONGODD`"]
    #[inline(always)]
    pub fn is_bitreversecomplexlongodd(&self) -> bool {
        *self == LEACMD_A::BITREVERSECOMPLEXLONGODD
    }
    #[doc = "Checks if the value of the field is `POLYNOMIALSCALAR`"]
    #[inline(always)]
    pub fn is_polynomialscalar(&self) -> bool {
        *self == LEACMD_A::POLYNOMIALSCALAR
    }
    #[doc = "Checks if the value of the field is `FFTCOMPLEXAUTOSCALING`"]
    #[inline(always)]
    pub fn is_fftcomplexautoscaling(&self) -> bool {
        *self == LEACMD_A::FFTCOMPLEXAUTOSCALING
    }
    #[doc = "Checks if the value of the field is `FIRLONG`"]
    #[inline(always)]
    pub fn is_firlong(&self) -> bool {
        *self == LEACMD_A::FIRLONG
    }
    #[doc = "Checks if the value of the field is `MACLONGMATRIX`"]
    #[inline(always)]
    pub fn is_maclongmatrix(&self) -> bool {
        *self == LEACMD_A::MACLONGMATRIX
    }
    #[doc = "Checks if the value of the field is `SUBLONGMATRIX`"]
    #[inline(always)]
    pub fn is_sublongmatrix(&self) -> bool {
        *self == LEACMD_A::SUBLONGMATRIX
    }
    #[doc = "Checks if the value of the field is `MAXLONGMATRIX`"]
    #[inline(always)]
    pub fn is_maxlongmatrix(&self) -> bool {
        *self == LEACMD_A::MAXLONGMATRIX
    }
    #[doc = "Checks if the value of the field is `MINLONGMATRIX`"]
    #[inline(always)]
    pub fn is_minlongmatrix(&self) -> bool {
        *self == LEACMD_A::MINLONGMATRIX
    }
    #[doc = "Checks if the value of the field is `FIRCOMPLEX`"]
    #[inline(always)]
    pub fn is_fircomplex(&self) -> bool {
        *self == LEACMD_A::FIRCOMPLEX
    }
    #[doc = "Checks if the value of the field is `MAXUNSIGNEDMATRIX`"]
    #[inline(always)]
    pub fn is_maxunsignedmatrix(&self) -> bool {
        *self == LEACMD_A::MAXUNSIGNEDMATRIX
    }
    #[doc = "Checks if the value of the field is `MINUNSIGNEDMATRIX`"]
    #[inline(always)]
    pub fn is_minunsignedmatrix(&self) -> bool {
        *self == LEACMD_A::MINUNSIGNEDMATRIX
    }
    #[doc = "Checks if the value of the field is `MACMATRIX`"]
    #[inline(always)]
    pub fn is_macmatrix(&self) -> bool {
        *self == LEACMD_A::MACMATRIX
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == LEACMD_A::MAX
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == LEACMD_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAXUNSIGNED`"]
    #[inline(always)]
    pub fn is_maxunsigned(&self) -> bool {
        *self == LEACMD_A::MAXUNSIGNED
    }
    #[doc = "Checks if the value of the field is `MINUNSIGNED`"]
    #[inline(always)]
    pub fn is_minunsigned(&self) -> bool {
        *self == LEACMD_A::MINUNSIGNED
    }
    #[doc = "Checks if the value of the field is `MAXUNSIGNEDLONGMATRIX`"]
    #[inline(always)]
    pub fn is_maxunsignedlongmatrix(&self) -> bool {
        *self == LEACMD_A::MAXUNSIGNEDLONGMATRIX
    }
    #[doc = "Checks if the value of the field is `MINUNSIGNEDLONGMATRIX`"]
    #[inline(always)]
    pub fn is_minunsignedlongmatrix(&self) -> bool {
        *self == LEACMD_A::MINUNSIGNEDLONGMATRIX
    }
    #[doc = "Checks if the value of the field is `IIRBQ2`"]
    #[inline(always)]
    pub fn is_iirbq2(&self) -> bool {
        *self == LEACMD_A::IIRBQ2
    }
    #[doc = "Checks if the value of the field is `FIRCOMPLEXLONG`"]
    #[inline(always)]
    pub fn is_fircomplexlong(&self) -> bool {
        *self == LEACMD_A::FIRCOMPLEXLONG
    }
    #[doc = "Checks if the value of the field is `DEINTERLEAVELONG`"]
    #[inline(always)]
    pub fn is_deinterleavelong(&self) -> bool {
        *self == LEACMD_A::DEINTERLEAVELONG
    }
    #[doc = "Checks if the value of the field is `WINDOW`"]
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        *self == LEACMD_A::WINDOW
    }
    #[doc = "Checks if the value of the field is `MAC3`"]
    #[inline(always)]
    pub fn is_mac3(&self) -> bool {
        *self == LEACMD_A::MAC3
    }
    #[doc = "Checks if the value of the field is `SCALEDMAC`"]
    #[inline(always)]
    pub fn is_scaledmac(&self) -> bool {
        *self == LEACMD_A::SCALEDMAC
    }
    #[doc = "Checks if the value of the field is `SCALEDFIR`"]
    #[inline(always)]
    pub fn is_scaledfir(&self) -> bool {
        *self == LEACMD_A::SCALEDFIR
    }
}
#[doc = "Write proxy for field `LEACMD`"]
pub struct LEACMD_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Suspends ongoing action an enters Ready"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(LEACMD_A::SUSPEND)
    }
    #[doc = "Resumes an previously suspended command execution"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(LEACMD_A::RESUME)
    }
    #[doc = "Complex FFT on 16 bit fractional numbers fix scaling"]
    #[inline(always)]
    pub fn fftcomplexfixedscaling(self) -> &'a mut W {
        self.variant(LEACMD_A::FFTCOMPLEXFIXEDSCALING)
    }
    #[doc = "Real FIR on 16 bit fractional numbers"]
    #[inline(always)]
    pub fn fir(self) -> &'a mut W {
        self.variant(LEACMD_A::FIR)
    }
    #[doc = "Real vector polynomial calculations 16 args all fractional"]
    #[inline(always)]
    pub fn polynomial(self) -> &'a mut W {
        self.variant(LEACMD_A::POLYNOMIAL)
    }
    #[doc = "Real FFT-extension on 16 bit fractional numbers"]
    #[inline(always)]
    pub fn fft(self) -> &'a mut W {
        self.variant(LEACMD_A::FFT)
    }
    #[doc = "Real vector polynomial calculations 32 bit args all fractional"]
    #[inline(always)]
    pub fn polynomiallong(self) -> &'a mut W {
        self.variant(LEACMD_A::POLYNOMIALLONG)
    }
    #[doc = "Real row oriented matrix multiply"]
    #[inline(always)]
    pub fn mpymatrixrow(self) -> &'a mut W {
        self.variant(LEACMD_A::MPYMATRIXROW)
    }
    #[doc = "Real matrix multiply 16 with 16 to 16 bit fractional"]
    #[inline(always)]
    pub fn mpymatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MPYMATRIX)
    }
    #[doc = "Real point wise matrix add of 16 and 16 to 16 bit number vector"]
    #[inline(always)]
    pub fn addmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::ADDMATRIX)
    }
    #[doc = "Real maximum value and position of 16 bit matrices"]
    #[inline(always)]
    pub fn maxmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MAXMATRIX)
    }
    #[doc = "Real minimum value and position of 16 bit matrices"]
    #[inline(always)]
    pub fn minmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MINMATRIX)
    }
    #[doc = "Real second order biquad using DF1 with 16 bit fractional"]
    #[inline(always)]
    pub fn iirbq1(self) -> &'a mut W {
        self.variant(LEACMD_A::IIRBQ1)
    }
    #[doc = "Real matrix MAC short with 16Bt to 16B fract"]
    #[inline(always)]
    pub fn mac(self) -> &'a mut W {
        self.variant(LEACMD_A::MAC)
    }
    #[doc = "Split 16B vector even to even words"]
    #[inline(always)]
    pub fn deinterleaveeveneven(self) -> &'a mut W {
        self.variant(LEACMD_A::DEINTERLEAVEEVENEVEN)
    }
    #[doc = "Split 16Bt vector even to odd words"]
    #[inline(always)]
    pub fn deinterleaveevenodd(self) -> &'a mut W {
        self.variant(LEACMD_A::DEINTERLEAVEEVENODD)
    }
    #[doc = "Split 16B vector odd to even words"]
    #[inline(always)]
    pub fn deinterleaveoddeven(self) -> &'a mut W {
        self.variant(LEACMD_A::DEINTERLEAVEODDEVEN)
    }
    #[doc = "Split 16B vector odd to odd words"]
    #[inline(always)]
    pub fn deinterleaveoddodd(self) -> &'a mut W {
        self.variant(LEACMD_A::DEINTERLEAVEODDODD)
    }
    #[doc = "Complex Dot Product"]
    #[inline(always)]
    pub fn maccomplexmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MACCOMPLEXMATRIX)
    }
    #[doc = "Complex conjugate Dot Product"]
    #[inline(always)]
    pub fn maccomplexconjugatematrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MACCOMPLEXCONJUGATEMATRIX)
    }
    #[doc = "Real point wise matrix Subtraction of 16 and 16 to 16 bit"]
    #[inline(always)]
    pub fn submatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::SUBMATRIX)
    }
    #[doc = "Real point wise matrix multiply 32 with 32 to 32 bit fractional"]
    #[inline(always)]
    pub fn mpylongmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MPYLONGMATRIX)
    }
    #[doc = "Complex point wise matrix multiply complex with complex"]
    #[inline(always)]
    pub fn mpycomplexmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MPYCOMPLEXMATRIX)
    }
    #[doc = "Real point wise matrix add of 32 and 32 to 32 bit number"]
    #[inline(always)]
    pub fn addlongmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::ADDLONGMATRIX)
    }
    #[doc = "List move 32 to 32 bit"]
    #[inline(always)]
    pub fn movelonglist(self) -> &'a mut W {
        self.variant(LEACMD_A::MOVELONGLIST)
    }
    #[doc = "Complex bit reversal for 16 bit fractional numbers even"]
    #[inline(always)]
    pub fn bitreversecomplexeven(self) -> &'a mut W {
        self.variant(LEACMD_A::BITREVERSECOMPLEXEVEN)
    }
    #[doc = "Complex bit reversal for 16 bit fractional Numbers odd"]
    #[inline(always)]
    pub fn bitreversecomplexodd(self) -> &'a mut W {
        self.variant(LEACMD_A::BITREVERSECOMPLEXODD)
    }
    #[doc = "Real second order biquad using DF2 with 16 bit fractional, extended to include bias and intermediate state min/max"]
    #[inline(always)]
    pub fn iirbq2extended(self) -> &'a mut W {
        self.variant(LEACMD_A::IIRBQ2EXTENDED)
    }
    #[doc = "Complex FFT on 32B bit fractional numbers, fix scaling"]
    #[inline(always)]
    pub fn fftcomplexlong(self) -> &'a mut W {
        self.variant(LEACMD_A::FFTCOMPLEXLONG)
    }
    #[doc = "Real FFT-extension on 32 bit fractional numbers"]
    #[inline(always)]
    pub fn fftlong(self) -> &'a mut W {
        self.variant(LEACMD_A::FFTLONG)
    }
    #[doc = "Complex bit reversal for 32 bit fractional numbers even"]
    #[inline(always)]
    pub fn bitreversecomplexlongeven(self) -> &'a mut W {
        self.variant(LEACMD_A::BITREVERSECOMPLEXLONGEVEN)
    }
    #[doc = "Complex bit reversal for 16 bit fractional numbers odd"]
    #[inline(always)]
    pub fn bitreversecomplexlongodd(self) -> &'a mut W {
        self.variant(LEACMD_A::BITREVERSECOMPLEXLONGODD)
    }
    #[doc = "Scalar Polynomial for math on 32bit fractional"]
    #[inline(always)]
    pub fn polynomialscalar(self) -> &'a mut W {
        self.variant(LEACMD_A::POLYNOMIALSCALAR)
    }
    #[doc = "Complex FFT on 16B bit fractional numbers with auto scaling for enhanced accuracy"]
    #[inline(always)]
    pub fn fftcomplexautoscaling(self) -> &'a mut W {
        self.variant(LEACMD_A::FFTCOMPLEXAUTOSCALING)
    }
    #[doc = "Real FIR on 32 bit fractional numbers"]
    #[inline(always)]
    pub fn firlong(self) -> &'a mut W {
        self.variant(LEACMD_A::FIRLONG)
    }
    #[doc = "Real block MAC on 32B fractional numbers"]
    #[inline(always)]
    pub fn maclongmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MACLONGMATRIX)
    }
    #[doc = "Real point wise matrix Subtraction of 32 and 32 to 32 bit"]
    #[inline(always)]
    pub fn sublongmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::SUBLONGMATRIX)
    }
    #[doc = "Real maximum value and position of signed 32B matrices"]
    #[inline(always)]
    pub fn maxlongmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MAXLONGMATRIX)
    }
    #[doc = "Real minimum value and position of signed 32B matrices"]
    #[inline(always)]
    pub fn minlongmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MINLONGMATRIX)
    }
    #[doc = "Complex FIR on 16B fractional numbers"]
    #[inline(always)]
    pub fn fircomplex(self) -> &'a mut W {
        self.variant(LEACMD_A::FIRCOMPLEX)
    }
    #[doc = "Real maximum value and position of unsigned 16B matrices"]
    #[inline(always)]
    pub fn maxunsignedmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MAXUNSIGNEDMATRIX)
    }
    #[doc = "Real minimum value and position of unsigned 32B matrices"]
    #[inline(always)]
    pub fn minunsignedmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MINUNSIGNEDMATRIX)
    }
    #[doc = "Real Matrix MAC on 16B fractional"]
    #[inline(always)]
    pub fn macmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MACMATRIX)
    }
    #[doc = "Vector maximum on 16B signed numbers"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(LEACMD_A::MAX)
    }
    #[doc = "Vector minimum on 16B signed numbers"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(LEACMD_A::MIN)
    }
    #[doc = "Vector maximum on 16B unsigned numbers"]
    #[inline(always)]
    pub fn maxunsigned(self) -> &'a mut W {
        self.variant(LEACMD_A::MAXUNSIGNED)
    }
    #[doc = "Vector minimum on 16B unsigned numbers"]
    #[inline(always)]
    pub fn minunsigned(self) -> &'a mut W {
        self.variant(LEACMD_A::MINUNSIGNED)
    }
    #[doc = "Matrix maximum on 32B unsigned numbers"]
    #[inline(always)]
    pub fn maxunsignedlongmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MAXUNSIGNEDLONGMATRIX)
    }
    #[doc = "Matrix minimum on 32B unsigned numbers"]
    #[inline(always)]
    pub fn minunsignedlongmatrix(self) -> &'a mut W {
        self.variant(LEACMD_A::MINUNSIGNEDLONGMATRIX)
    }
    #[doc = "Real second order biquad using DF2 with 16 bit fractional"]
    #[inline(always)]
    pub fn iirbq2(self) -> &'a mut W {
        self.variant(LEACMD_A::IIRBQ2)
    }
    #[doc = "Complex FIR on 32B fractional numbers"]
    #[inline(always)]
    pub fn fircomplexlong(self) -> &'a mut W {
        self.variant(LEACMD_A::FIRCOMPLEXLONG)
    }
    #[doc = "Split Function on 32B Vectors/Matrices"]
    #[inline(always)]
    pub fn deinterleavelong(self) -> &'a mut W {
        self.variant(LEACMD_A::DEINTERLEAVELONG)
    }
    #[doc = "In-place symmetrical window on 16B fractional numbers"]
    #[inline(always)]
    pub fn window(self) -> &'a mut W {
        self.variant(LEACMD_A::WINDOW)
    }
    #[doc = "Vector MAC at three points, real 16-bit with 32-bit result"]
    #[inline(always)]
    pub fn mac3(self) -> &'a mut W {
        self.variant(LEACMD_A::MAC3)
    }
    #[doc = "Scaled vector multiply and accumulate (MAC)"]
    #[inline(always)]
    pub fn scaledmac(self) -> &'a mut W {
        self.variant(LEACMD_A::SCALEDMAC)
    }
    #[doc = "Scaled FIR, 16-bit real fractional numbers"]
    #[inline(always)]
    pub fn scaledfir(self) -> &'a mut W {
        self.variant(LEACMD_A::SCALEDFIR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | (((value as u32) & 0xff) << 2);
        self.w
    }
}
#[doc = "Reader of field `LEACTX`"]
pub type LEACTX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEACTX`"]
pub struct LEACTX_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
LEA instruction handshake synchronization type flag"]
    #[inline(always)]
    pub fn leaitflg(&self) -> LEAITFLG_R {
        LEAITFLG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:9 - 9:2\\]
These bits represent the LEA command to be invoked. See also the command table"]
    #[inline(always)]
    pub fn leacmd(&self) -> LEACMD_R {
        LEACMD_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Command context: This bit field may be set by the user together with invoking a command. This bit field is saved on SUSPEND; and restored from LEA Stack on RESUME. This bit field is used by SW to associate or synchronize LEA commands to a certain tasks and IDs."]
    #[inline(always)]
    pub fn leactx(&self) -> LEACTX_R {
        LEACTX_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
LEA instruction handshake synchronization type flag"]
    #[inline(always)]
    pub fn leaitflg(&mut self) -> LEAITFLG_W {
        LEAITFLG_W { w: self }
    }
    #[doc = "Bits 2:9 - 9:2\\]
These bits represent the LEA command to be invoked. See also the command table"]
    #[inline(always)]
    pub fn leacmd(&mut self) -> LEACMD_W {
        LEACMD_W { w: self }
    }
    #[doc = "Bits 20:31 - 31:20\\]
Command context: This bit field may be set by the user together with invoking a command. This bit field is saved on SUSPEND; and restored from LEA Stack on RESUME. This bit field is used by SW to associate or synchronize LEA commands to a certain tasks and IDs."]
    #[inline(always)]
    pub fn leactx(&mut self) -> LEACTX_W {
        LEACTX_W { w: self }
    }
}
