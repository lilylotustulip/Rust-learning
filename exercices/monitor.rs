
fn main() {
    check_goal(1400, 1000, 2000);
}

fn check_goal(meal_1: i64, meal_2: i64, target_goal: i64) -> bool {

    let total_calories: i64 = meal_1 + meal_2;
    if total_calories >= target_goal {
        println!("the target goal met your total calories intake!");
        true
    }
    else {
        println!("your total calories are less then your target goal");
        false
    }
}