use std::collections::BinaryHeap;

fn main() {
    let mut jobs = BinaryHeap::new();
    jobs.push((100, "Reply to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));
    
    while let Some((_, task)) = jobs.pop() {
        println!("You need to: {task:?}");     
    }
}
