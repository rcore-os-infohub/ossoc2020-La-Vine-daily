// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade 
// is represented numerically (e.g. 1.0 -> 5.5). 
// However, the school also issues alphabetical grades (A+ -> F-) and needs 
// to be able to print both types of report card!

// Make the necessary code changes to support alphabetical report cards, thereby making 
// the second test pass.


pub struct ReportCard<T, V, K> {
    pub grade: T,
    pub student_name: V,
    pub student_age: K,
}

impl<T:std::fmt::Display,V:std::fmt::Display,K:std::fmt::Display> ReportCard<T, V, K> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}", 
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1, 
            student_name: "Tom Wriggle".to_string(), 
            student_age: 12,
        };
        assert_eq!(report_card.print(), "Tom Wriggle (12) - achieved a grade of 2.1");
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+", 
            student_name: "Gary Plotter".to_string(), 
            student_age: 11,
        };
        assert_eq!(report_card.print(), "Gary Plotter (11) - achieved a grade of A+");
    }
}