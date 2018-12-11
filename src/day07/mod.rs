mod job;
pub use self::job::*;

use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn create_nodes(jobs: &[Job]) -> HashMap<char, Vec<char>> {
    let mut nodes: HashMap<char, Vec<char>> = HashMap::new();
    for edge in jobs.iter() {
        {
            let node = nodes.entry(edge.name).or_insert(vec![]);
            node.push(edge.prereq);
        } // force end of mutable borrow
        nodes.entry(edge.prereq).or_insert(vec![]);
    }

    nodes
}

pub fn topological_sort(jobs: &[Job]) -> String {
    let mut l: Vec<char> = vec![];

    let mut nodes = create_nodes(jobs);

    let mut s: Vec<char> = nodes
        .iter()
        .filter(|(_, edges)| edges.len() == 0)
        .map(|(node, _)| *node)
        .collect();

    while s.len() > 0 {
        s.sort();
        let n = s.remove(0);
        l.push(n);

        // nodes with edge starting at n
        let incoming: Vec<char> = nodes
            .iter()
            .filter(|(_, edges)| edges.iter().any(|e| *e == n))
            .map(|(node, _)| *node)
            .collect();

        for m in incoming {
            // get node from map
            let mut m_edges = nodes.get_mut(&m).unwrap();
            // find index of n
            let index = m_edges.iter().position(|e| *e == n).unwrap();
            // remove index
            m_edges.remove(index);
            // if node is now empty, add to S
            if m_edges.len() == 0 {
                s.push(m);
            }
        }
    }

    l.iter().collect()
}

pub fn read_input(filename: &str) -> Result<Vec<Job>, Box<Error>> {
    let s = fs::read_to_string(filename)?;
    let mut result = vec![];

    for line in s.lines() {
        let job: Job = line.parse()?;
        result.push(job);
    }

    Ok(result)
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    job: char,
    job_size: usize,
    current: usize,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            job: '.',
            job_size: 0,
            current: 1,
        }
    }

    fn give(&mut self, job: char, size: usize) {
        self.job = job;
        self.job_size = size;
        self.current = 0;
        // println!("W{}: received job {} for {}", self.id, job, size);
    }

    fn is_complete(&self) -> bool {
        self.current >= self.job_size
    }

    fn idle(&mut self) {
        self.job = '.';
        self.job_size = 0;
        self.current = 1;
        // println!("W{}: went idle", self.id);
    }

    fn is_idle(&self) -> bool {
        self.job == '.'
    }

    fn tick(&mut self) {
        if self.job == '.' {
            return;
        }

        self.current += 1;
        // println!("W{}: ticked to {}", self.id, self.current);
    }
}

struct WorkSim {
    tick_count: usize,
    workers: Vec<Worker>,
    queue: Vec<char>,
    done: Vec<char>,
    base_cost: usize,
    deps: HashMap<char, Vec<char>>,
}

impl WorkSim {
    fn new(jobs: &[Job], worker_count: usize, base_cost: usize) -> WorkSim {
        let mut workers = vec![];
        for i in 0..worker_count {
            workers.push(Worker::new(i));
        }

        let deps = create_nodes(jobs);

        let mut queue: Vec<char> = deps.iter().map(|(name, _)| *name).collect();
        queue.sort();

        let done = vec![];

        WorkSim {
            tick_count: 0,
            workers,
            queue,
            done,
            deps,
            base_cost,
        }
    }

    fn is_complete(&self) -> bool {
        self.queue.len() == 0 && self.workers.iter().all(|w| w.is_idle())
    }

    fn tick(&mut self) {
        // print!("{}\t", self.tick_count);
        // for i in 0..self.workers.len() {
        //     print!("{}\t", self.workers[i].job);
        // }
        // println!("{:?}", self.done);

        self.tick_step();

        self.tick_count += 1;
    }

    fn tick_step(&mut self) {
        for w in self.workers.iter_mut() {
            w.tick();

            if w.is_complete() && w.job != '.' {
                self.done.push(w.job);
                w.idle();
            }
        }

        for w in self.workers.iter_mut() {
            if w.is_idle() {
                if let Some(job) = WorkSim::next_available_job(&self.queue, &self.deps, &self.done)
                {
                    let index = self.queue.iter().position(|q| *q == job).unwrap();
                    self.queue.remove(index);
                    let cost = WorkSim::job_cost(job, self.base_cost);
                    w.give(job, cost);
                }
            }
        }
    }

    fn next_available_job(
        queue: &Vec<char>,
        deps: &HashMap<char, Vec<char>>,
        done: &Vec<char>,
    ) -> Option<char> {
        if queue.len() == 0 {
            return None;
        }

        for job in queue.iter() {
            let deps = deps.get(job).unwrap();

            // all dependencies are satisfied?
            if deps.iter().all(|d| done.iter().any(|q| q == d)) {
                return Some(*job);
            }
        }

        None
    }

    fn job_cost(job: char, base_cost: usize) -> usize {
        base_cost + 1 + ((job as u8 - 'A' as u8) as usize)
    }
}

pub fn work_length(jobs: &[Job], worker_count: usize, base_cost: usize) -> usize {
    let mut sim = WorkSim::new(jobs, worker_count, base_cost);
    while !sim.is_complete() {
        sim.tick();
    }

    sim.tick_count - 1
}

#[cfg(test)]
mod test {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn can_parse_job_structs() {
        let job: Job = "Step C must be finished before step A can begin."
            .parse()
            .unwrap();
        assert_that!(&job.name).is_equal_to('A');
        assert_that!(&job.prereq).is_equal_to('C');
    }

    #[test]
    fn topological_sort_sample() {
        let input: Vec<Job> = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            .lines()
            .map(|l| l.parse::<Job>().unwrap())
            .collect();

        let result = topological_sort(&input);
        assert_that!(&result.as_str()).is_equal_to("CABDFE");
    }

    #[test]
    fn topological_sort_examples() {
        let input = read_input("inputs\\day07.txt").unwrap();
        let result = topological_sort(&input);

        assert_that!(&result.as_str()).is_equal_to("BCADPVTJFZNRWXHEKSQLUYGMIO");
    }

    #[test]
    fn work_length_sample() {
        let input: Vec<Job> = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            .lines()
            .map(|l| l.parse::<Job>().unwrap())
            .collect();

        let result = work_length(&input, 2, 0);
        assert_that!(&result).is_equal_to(15);
    }

    #[test]
    fn work_length_input() {
        let input = read_input("inputs\\day07.txt").unwrap();
        let result = work_length(&input, 5, 60);
        assert_that!(&result).is_equal_to(973);
    }
}
