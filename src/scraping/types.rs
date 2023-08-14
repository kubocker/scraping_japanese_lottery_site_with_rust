#[derive(Debug)]
pub struct Figure {
    pub num1: i32,
    pub num2: String,
}


#[derive(Debug)]
pub struct Result {
    pub num: String,
    pub price: String,
}


#[derive(Debug)]
pub struct Numbers3ResultSet {
    pub straight: Result,
    pub boxes: Result,
    pub mini: Result,
}


#[derive(Debug)]
pub struct Numbers3Result {
    pub straights: Result,
    pub boxes: Result,
    pub sets: Numbers3ResultSet,
}


#[derive(Debug)]
pub struct Numbers4ResultSet {
    pub straight: Result,
    pub boxes: Result,
}


#[derive(Debug)]
pub struct Numbers4Result {
    pub straights: Result,
    pub boxes: Result,
    pub sets: Numbers4ResultSet,
}


// struct Loto6Figure {
//     num1: i32,
//     num2: i32,
//     num3: i32,
//     num4: i32,
//     num5: i32,
//     num6: i32,
//     bonus: i32,
// }

// struct Loto6Result {
//     top1: Result,
//     top2: Result,
//     top3: Result,
//     top4: Result,
//     top5: Result,
//     carry_over: String,
// }

#[derive(Debug)]
pub struct Numbers3 {
    pub no: String,
    pub date: String,
    pub figures: Figure,
    pub results: Numbers3Result,
}

#[derive(Debug)]
pub struct  Numbers4 {
    pub no: String,
    pub date: String,
    pub figures: Figure,
    pub results: Numbers4Result,
}


#[derive(Debug)]
pub struct Loto6 {
    pub no: String,
    pub date: String,
    // figures: Loto6Figure,
    // results: Loto6Result,
}
