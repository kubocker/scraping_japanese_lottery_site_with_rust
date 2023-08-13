// struct Figure {
//     num1: i32,
//     num2: String,
// }

// struct Result {
//     num: i32,
//     price: f64,
// }


// struct Numbers3Result {
//     straights: Result,
//     boxes: Result,
//     sets: Numbers3ResultSet,
// }

// struct Numbers3ResultSet {
//     straight: Result,
//     boxes: Result,
// }


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
    // figures: Figure,
    // results: Numbers3Result,
}

// pub type Numbers4 = Numbers3;


#[derive(Debug)]
pub struct Loto6 {
    pub no: String,
    pub date: String,
    // figures: Loto6Figure,
    // results: Loto6Result,
}
