use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct City(usize);

pub struct Tour(Vec<City>);

#[derive(Default, Debug)]
pub struct Status {
    penalty_cost: u64,
    violations: Vec<String>,
}

pub trait Criterion {
    fn evaluate(&self, tour: &Tour, previous_status: Status) -> Status;
}

// distance

pub struct Distance {
    distances: HashMap<(City, City), u64>,
    penalty_per_km: u64,
}

impl Distance {
    fn new_example() -> Self {
        Distance {
            distances: [
                ((City(0), City(1)), 10),
                ((City(0), City(2)), 7),
                ((City(0), City(3)), 20),
                ((City(1), City(0)), 57),
                ((City(1), City(2)), 9),
                ((City(1), City(3)), 11),
                ((City(2), City(0)), 3),
                ((City(2), City(1)), 15),
                ((City(2), City(3)), 7),
                ((City(3), City(0)), 88),
                ((City(3), City(1)), 18),
                ((City(3), City(2)), 8),
            ]
            .into_iter()
            .collect(),
            penalty_per_km: 1,
        }
    }
}

impl Criterion for Distance {
    fn evaluate(&self, tour: &Tour, mut status: Status) -> Status {
        for (a, b) in tour.0.iter().zip(tour.0.iter().skip(1)) {
            let distance_km = self.distances.get(&(*a, *b)).unwrap();
            status.penalty_cost += self.penalty_per_km * distance_km;
        }
        status
    }
}

// capacity

pub struct Capacity {
    vehicle_capacity: u64,
    shipment_volumes: HashMap<City, i64>,
}

impl Capacity {
    fn new_example() -> Self {
        Self {
            vehicle_capacity: 10,
            shipment_volumes: [(City(0), 5), (City(1), 7), (City(2), -4), (City(3), -8)]
                .into_iter()
                .collect(),
        }
    }
}

impl Criterion for Capacity {
    fn evaluate(&self, tour: &Tour, mut status: Status) -> Status {
        let vehicle_capacity = self.vehicle_capacity as i64;
        let mut current_capacity = 0;
        for city in &tour.0 {
            let volume = self.shipment_volumes.get(city).unwrap();
            current_capacity += volume;
            if current_capacity > vehicle_capacity {
                let violation = format!(
                    "Capacity reached {} while vehicle capacity is {}.",
                    current_capacity, self.vehicle_capacity
                );
                status.violations.push(violation);
            }
        }
        status
    }
}

// precedence

pub struct Precedence {
    must_precede: Vec<(City, City)>,
    penalty_per_violation: u64,
}

impl Precedence {
    fn new_example() -> Self {
        Self {
            must_precede: vec![(City(2), City(0))],
            penalty_per_violation: 100,
        }
    }
}

impl Criterion for Precedence {
    fn evaluate(&self, tour: &Tour, mut status: Status) -> Status {
        for (a, b) in &self.must_precede {
            let pos_a = tour.0.iter().position(|x| x == a).unwrap();
            let pos_b = tour.0.iter().position(|x| x == b).unwrap();
            if pos_a > pos_b {
                status.penalty_cost += self.penalty_per_violation;
                let violation = format!("Precedence {}->{} is violated.", a.0, b.0);
                status.violations.push(violation);
            }
        }
        status
    }
}

// queue

orx_meta::define_queue!(
    elements => [ Criterion ];
    queue => [ StCriteria; CriteriaSingle, Criteria ];
);

impl<F: Criterion> Criterion for CriteriaSingle<F> {
    // identity: return the previous state
    fn evaluate(&self, tour: &Tour, status: Status) -> Status {
        self.f.evaluate(tour, status)
    }
}

impl<F: Criterion, B: StCriteria> Criterion for Criteria<F, B> {
    // composition: evaluate them both
    fn evaluate(&self, tour: &Tour, status: Status) -> Status {
        let status = self.f.evaluate(tour, status);
        self.b.evaluate(tour, status)
    }
}

// use cases

fn use_case1() {
    println!("\n\n# Use case with criteria [Distance]");
    let tour = Tour(vec![City(0), City(1), City(2), City(3)]);

    let criteria = Criteria::new(Distance::new_example());
    let status = criteria.evaluate(&tour, Status::default());
    println!("{status:?}");
}

fn use_case2() {
    println!("\n\n# Use case with criteria [Distance, Precedence]");
    let tour = Tour(vec![City(0), City(1), City(2), City(3)]);

    let criteria = Criteria::new(Distance::new_example()).push(Precedence::new_example());
    let status = criteria.evaluate(&tour, Status::default());
    println!("{status:?}");
}

fn use_case3() {
    println!("\n\n# Use case with criteria [Distance, Capacity]");
    let tour = Tour(vec![City(0), City(1), City(2), City(3)]);

    let criteria = Criteria::new(Distance::new_example()).push(Capacity::new_example());
    let status = criteria.evaluate(&tour, Status::default());
    println!("{status:?}");
}

fn use_case4() {
    println!("\n\n# Use case with criteria [Distance, Capacity, Precedence]");
    let tour = Tour(vec![City(0), City(1), City(2), City(3)]);

    let criteria = Criteria::new(Distance::new_example())
        .push(Capacity::new_example())
        .push(Precedence::new_example());
    let status = criteria.evaluate(&tour, Status::default());
    println!("{status:?}");
}

fn main() {
    use_case1();
    use_case2();
    use_case3();
    use_case4();
}
