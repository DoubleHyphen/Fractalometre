pub trait Bloatable {
    type Dub;
    type Quat;
    type Oct;
    fn bloat_2(self) -> Self::Dub;
    fn bloat_4(self) -> Self::Quat;
    fn bloat_8(self) -> Self::Oct;
}

impl Bloatable for u8 {
    type Dub  = u16;
    type Quat = u32;
    type Oct  = u64;
    
    fn bloat_2(self) -> u16 {
        let mut result = u16::from(self);
        result |= result << 4;
        result &= 0x_0F_0F;
        result |= result << 2;
        result &= 0x_33_33;
        result |= result << 1;
        result &  0x_55_55
    }
    
    fn bloat_4(self) -> u32 {
        let mut result = u32::from(self);
        result |= result<<12;
        result |= result<<6;
        result &= 0x_03_03_03_03;
        result |= result<<3;
        result &  0x_11_11_11_11
    }
    
    fn bloat_8(self) -> u64 {
        let mut result = u64::from(self);
        result |= result<<28;
        result |= result<<14;
        result |= result<<7;
        result & 0x_01_01_01_01_01_01_01_01
    }
}

impl Bloatable for u16 {
    type Dub  =  u32;
    type Quat =  u64;
    //type Oct  = [u64; 2];
    type Oct = u8;
    
    fn bloat_2(self) -> u32 {
        let mut result = u32::from(self);
        result |= result << 8;
        result &= 0x_00_FF_00_FF;
        result |= result << 4;
        result &= 0x_0F_0F_0F_0F;
        result |= result << 2;
        result &= 0x_33_33_33_33;
        result |= result << 1;
        result &  0x_55_55_55_55
    }
    
    fn bloat_4(self) -> u64 {
        let mut result = u64::from(self);
        result |= result<<24;
        result |= result<<12;
        result &= 0x_00_0F_00_0F_00_0F_00_0F;
        result |= result<<6;
        result |= result<<3;
        result &  0x_11_11_11_11_11_11_11_11
    }
    
    fn bloat_8(self) -> u8{//-> [u64; 2] {
        let upper_byte = (self>>8) as u8;
        let lower_byte = (self & 0x00FF) as u8;
        let _result: [u64; 2] = [upper_byte.bloat_8(), lower_byte.bloat_8()];
        unimplemented!("Could not figure out how to let arrays implement PrimInt.");
    }
}

impl Bloatable for u32 {
    type Dub  =  u64;
    //type Quat = [u64;2];
    //type Oct  = [u64;4];
    type Quat = u8;
    type Oct = u8;
    
    fn bloat_2(self) -> u64 {
        let mut result = u64::from(self);
        result |= result << 16;
        result &= 0x_00_00_FF_FF_00_00_FF_FF;
        result |= result << 8;
        result &= 0x_00_FF_00_FF_00_FF_00_FF;
        result |= result << 4;
        result &= 0x_0F_0F_0F_0F_0F_0F_0F_0F;
        result |= result << 2;
        result &= 0x_33_33_33_33_33_33_33_33;
        result |= result << 1;
        result &  0x_55_55_55_55_55_55_55_55
    }
    
    fn bloat_4(self) -> u8 {// -> [u64;2] {
        let upper_u16 = (self>>16) as u16;
        let lower_u16 = (self & 0x_0000_FFFF) as u16;
        let _result: [u64; 2] = [upper_u16.bloat_4(), lower_u16.bloat_4()];
        unimplemented!("Could not figure out how to let arrays implement PrimInt.");
    }
    
    fn bloat_8(self) -> u8 {//-> [u64; 4] {
        let get_part = |x| (((self>>(x*8)) & (0x_0000_00FF as u32)) as u8).bloat_8();
        let _result: [u64;4] = [get_part(3), get_part(2), get_part(1), get_part(0)];
        unimplemented!("Could not figure out how to let arrays implement PrimInt.");
    }
}
