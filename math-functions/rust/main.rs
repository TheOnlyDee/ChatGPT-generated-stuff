// La libreria è chiamata "math"
pub mod math {
    // Calcola la radice quadrata di un numero
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    // Calcola la potenza di un numero
    pub fn pow(x: f64, y: f64) -> f64 {
        x.powf(y)
    }

    // Calcola il logaritmo di un numero
    pub fn log(x: f64) -> f64 {
        x.ln()
    }

    // Calcola il cubo di un numero
    pub fn cube(x: f64) -> f64 {
        x * x * x
    }

    // Calcola l'area di un cerchio dato il raggio
    pub fn circle_area(r: f64) -> f64 {
        std::f64::consts::PI * r * r
    }
}
