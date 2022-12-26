struct QuadraticEquation {
    coa: f32,
    cob: f32,
    coc: f32,
}

impl QuadraticEquation {
    fn unpack(&self) -> (f32, f32, f32) {
        return (self.coa, self.cob, self.coc);
    }

    fn solve(&self) -> (f32, f32) {
        let (a, b, c) = self.unpack();

        let bsqr_term = ((b * b) - (4.0*a*c)).sqrt();
        
        let pos_b = -(b)+bsqr_term;
        let neg_b = (b)-bsqr_term;

        return ((pos_b)/(2.0*a), (neg_b)/(2.0*a));
    }

    fn format(&self) -> String {
        let (a, b, c) = self.unpack();

        let equation = format!("{}x^2 + {}x + {}", a, b, c);

        return equation;
    } 
}

fn main() {
    let my_equation = QuadraticEquation {
        coa: 1.0,
        cob: 10.0,
        coc: 25.0
    };

    println!("{} solved is: {:?}", my_equation.format(), my_equation.solve());
}

