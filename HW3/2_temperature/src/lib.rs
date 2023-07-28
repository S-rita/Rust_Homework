// x < y
pub fn calc1(x:f32) -> f32  {
    return (5.0/9.0)*(x - 32.0);
}

pub fn calc2(x:f32, z:f32) -> f32  {
    return (5.0/9.0)*((x+(z*1.0)) - 32.0);
}

pub fn calc3(x:f32, z:f32) -> f32  {
    return (5.0/9.0)*((x+(z*2.0))-32.0);
}

pub fn calc4(y:f32, z:f32) -> f32  {
    return (5.0/9.0)*((y-(z*2.0))-32.0);
}

pub fn calc5(y:f32, z:f32) -> f32 {
    return (5.0/9.0)*((y-(z*1.0))-32.0);
}

pub fn calc6(y:f32) -> f32 {
    return (5.0/9.0)*(y-32.0);
}

// x > y

pub fn calc1_(x:f32) -> f32  {
    return (5.0/9.0)*(x - 32.0);
}

pub fn calc2_(x:f32, z:f32) -> f32  {
    return (5.0/9.0)*((x-(z*1.0)) - 32.0);
}

pub fn calc3_(x:f32, z:f32) -> f32  {
    return (5.0/9.0)*((x-(z*2.0))-32.0);
}

pub fn calc4_(y:f32, z:f32) -> f32  {
    return (5.0/9.0)*((y+(z*2.0))-32.0);
}

pub fn calc5_(y:f32, z:f32) -> f32 {
    return (5.0/9.0)*(y+(z*1.0)-32.0);
}

pub fn calc6_(y:f32) -> f32 {
    return (5.0/9.0)*(y-32.0);
}
