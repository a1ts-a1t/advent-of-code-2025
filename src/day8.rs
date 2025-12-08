// i'm sure there's a better way to do this
// it might involve
// SLINK - https://github.com/battuzz/slink/blob/master/doc/sibson.pdf
// Randomized algorithm - https://www.sciencedirect.com/science/article/pii/S0890540185710498?via%3Dihub
// but w/e
// this code is also just generally a mess

use std::hash::Hash;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
    hash::Hasher,
    rc::Rc,
};

use itertools::Itertools;

// inputs are all integers
// and we won't be rooting the distances
#[derive(Clone)]
struct Vec3(i64, i64, i64);

impl Vec3 {
    fn distance(p: &Self, q: &Self) -> u64 {
        ((p.0 - q.0).pow(2) + (p.1 - q.1).pow(2) + (p.2 - q.2).pow(2)) as u64
    }
}

impl From<&str> for Vec3 {
    fn from(value: &str) -> Self {
        let v = value
            .splitn(3, ',')
            .map(|s| str::parse::<i64>(s).unwrap())
            .collect_vec();

        Vec3(v[0], v[1], v[2])
    }
}

#[derive(PartialEq, Eq, Debug)]
struct PointPair {
    idx: usize,
    idy: usize,
    distance: u64,
}

impl PartialOrd for PointPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl From<Vec<(usize, Vec3)>> for PointPair {
    fn from(value: Vec<(usize, Vec3)>) -> Self {
        Self {
            idx: value[0].0,
            idy: value[1].0,
            distance: Vec3::distance(&value[0].1, &value[1].1),
        }
    }
}

impl From<Vec<(usize, &Vec3)>> for PointPair {
    fn from(value: Vec<(usize, &Vec3)>) -> Self {
        Self {
            idx: value[0].0,
            idy: value[1].0,
            distance: Vec3::distance(value[0].1, value[1].1),
        }
    }
}

#[derive(Clone, Debug)]
struct Cluster(Rc<HashSet<usize>>);

impl Cluster {
    fn new(set: &HashSet<usize>) -> Self {
        Cluster(Rc::new(set.clone()))
    }

    fn merge(cluster_x: Option<Cluster>, cluster_y: Option<Cluster>) -> HashSet<usize> {
        match (cluster_x, cluster_y) {
            (None, None) => HashSet::new(),
            (None, Some(y)) => y.0.as_ref().clone(),
            (Some(x), None) => x.0.as_ref().clone(),
            (Some(x), Some(y)) => HashSet::union(&x.0, &y.0).cloned().collect(),
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl PartialEq for Cluster {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Cluster {}

impl Hash for Cluster {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        hasher.write_usize(Rc::as_ptr(&self.0) as usize);
    }
}

pub fn part1() {
    let body = read_to_string("inputs/day8.txt").unwrap();
    let point_pairs = body
        .split('\n')
        .map(Vec3::from)
        .enumerate()
        .combinations(2)
        .map(PointPair::from);

    let mut heap: BinaryHeap<PointPair> = BinaryHeap::new();
    for point_pair in point_pairs {
        heap.push(point_pair);
    }

    let mut cluster_map: HashMap<usize, Cluster> = HashMap::new();
    for _ in 0..1000 {
        let point_pair = heap.pop().unwrap();
        let cluster_x = cluster_map.remove(&point_pair.idx);
        let cluster_y = cluster_map.remove(&point_pair.idy);

        let mut merged = Cluster::merge(cluster_x, cluster_y);
        merged.extend(vec![point_pair.idx, point_pair.idy]);

        let cluster = Cluster::new(&merged);
        for idx in merged {
            cluster_map.insert(idx, cluster.clone());
        }
    }

    let product = cluster_map
        .values()
        .unique()
        .map(Cluster::len)
        .k_largest(3)
        .product::<usize>();

    print!("Part 1 answer: {}", product);
}

pub fn part2() {
    let body = read_to_string("inputs/day8.txt").unwrap();
    let vecs = body.split('\n').map(Vec3::from).collect_vec();

    let point_pairs = vecs.iter().enumerate().combinations(2).map(PointPair::from);

    let mut heap: BinaryHeap<PointPair> = BinaryHeap::new();
    for point_pair in point_pairs {
        heap.push(point_pair);
    }

    let mut cluster_map: HashMap<usize, Cluster> = HashMap::new();
    loop {
        let point_pair = heap.pop().unwrap();
        let cluster_x = cluster_map.remove(&point_pair.idx);
        let cluster_y = cluster_map.remove(&point_pair.idy);

        let mut merged = Cluster::merge(cluster_x, cluster_y);
        merged.extend(vec![point_pair.idx, point_pair.idy]);

        if merged.len() == vecs.len() {
            let x = vecs.get(point_pair.idx).unwrap();
            let y = vecs.get(point_pair.idy).unwrap();
            let product = x.0 * y.0;
            print!("Part 2 answer: {}", product);
            return;
        }

        let cluster = Cluster::new(&merged);
        for idx in merged {
            cluster_map.insert(idx, cluster.clone());
        }
    }
}
