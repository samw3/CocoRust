use std::borrow::Borrow;
use std::ops::Deref;

#[derive(Clone)]
struct Range {
    from: i32,
    to: i32,
}

impl Range {
    pub fn new(from: i32, to: i32) -> Self {
        Range { from, to }
    }
}

pub struct CharSet {
    head: Vec<Range>,
}

impl CharSet {
    pub fn new() -> Self {
        CharSet { head: vec![] }
    }

    pub fn get(&self, i: i32) -> bool {
        for p in &self.head {
            if i < p.from {
                return false;
            } else if i <= p.to {
                return true;
            }
        }
        return false;
    }

    pub fn set(&mut self, i: i32) {
        let mut cur = 0;
        let mut v = &mut self.head;
        while cur < v.len() && i >= v[cur].from - 1 {
            if i <= v[cur].to + 1 {
                if i == v[cur].from - 1 {
                    v[cur].from -= 1
                } else if i == v[cur].to + 1 {
                    v[cur].to += 1;
                    let next = cur + 1;
                    if next < v.len() && (v[cur].to == v[next].from - 1) {
                        v[cur].to = v[next].to;
                        v.remove(next);
                    }
                }
                return;
            }
            cur += 1;
        }
        v.insert(cur, Range { from: i, to: i });
    }

    pub fn clone(&mut self) -> CharSet {
        let mut new_char_set = CharSet::new();
        for cur in &self.head {
            new_char_set.head.push(Range { from: cur.from, to: cur.to });
        }
        new_char_set
    }

    pub fn equals(&self, other: &CharSet) -> bool {
        if self.head.len() != other.head.len() {
            return false;
        }
        for (i, p) in self.head.iter().enumerate() {
            let q = &other.head[i];
            if p.from != q.from || p.to != q.to {
                return false;
            }
        }
        true
    }

    pub fn elements(&self) -> i32 {
        let mut n = 0;
        for p in &self.head {
            n += p.to - p.from + 1;
        }
        n
    }

    pub fn first(&self) -> i32 {
        if self.head.len() > 0 {
            return self.head[0].from;
        }
        return -1;
    }

    pub fn or(&mut self, other: &CharSet) {
        for p in &other.head {
            for i in p.from..=p.to {
                self.set(i)
            }
        }
    }

    pub fn and(&mut self, other: &CharSet) {
        let mut x = CharSet::new();
        for p in &self.head {
            for i in p.from..=p.to {
                if other.get(i) {
                    x.set(i);
                }
            }
        }
        self.head = x.head;
    }

    pub fn subtract(&mut self, other: &CharSet) {
        let mut x = CharSet::new();
        for p in &self.head {
            for i in p.from..=p.to {
                if !other.get(i) {
                    x.set(i);
                }
            }
        }
        self.head = x.head;
    }

    pub fn includes(&self, other: &CharSet) -> bool {
        for p in &other.head {
            for i in p.from..=p.to {
                if !self.get(i) {
                    return false;
                }
            }
        }
        true
    }

    pub fn intersects(&self, other: &CharSet) -> bool {
        for p in &other.head {
            for i in p.from..=p.to {
                if self.get(i) {
                    return true;
                }
            }
        }
        false
    }

    pub fn fill(&mut self) {
        self.head = vec![Range { from: 0, to: char::MAX as i32 }]
    }
}

