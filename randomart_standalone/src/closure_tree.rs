#![allow(non_camel_case_types)]
use crate::utils::Colour;
use std::sync::{Mutex, Arc};
use crate::node::Node;
use std::fmt;

pub trait ClosureNode: Fn(f32, f32) -> f32 + Send + Sync {}
impl<T: Fn(f32, f32) -> f32 + Send + Sync> ClosureNode for T {}

pub struct ClosureTree {
    pub r: CachedClosure,
    pub g: CachedClosure,
    pub b: CachedClosure,
}

impl ClosureTree {
    pub fn from_node(node: &Node) -> Self {
        match node {
            Node::Triple(r, g, b) => Self {
                r: r.to_cached_closure(),
                g: g.to_cached_closure(),
                b: b.to_cached_closure(),
            },
            _ => panic!("Expected Node::Triple at top level"),
        }
    }

    pub fn eval_rgb_cached(&self, x: f32, y: f32, xi: usize, yi: usize) -> Colour {
        Colour {
            r: self.r.eval(x, y, xi, yi),
            g: self.g.eval(x, y, xi, yi),
            b: self.b.eval(x, y, xi, yi),
        }
    }
}

pub enum Dependency {
    x,
    y,
    x_and_y,
    const_
}

pub enum CachedClosure {
    Const(f32),
    XCached {
        cache: Arc<Mutex<Option<Vec<f32>>>>,
        f: Arc<dyn Fn(f32) -> f32 + Send + Sync>,
    },
    YCached {
        cache: Arc<Mutex<Option<Vec<f32>>>>,
        f: Arc<dyn Fn(f32) -> f32 + Send + Sync>,
    },
    XY(Arc<dyn Fn(f32, f32) -> f32 + Send + Sync>),
}

impl CachedClosure {
    pub fn eval(&self, x: f32, y: f32, xi: usize, yi: usize) -> f32 {
        match self {
            CachedClosure::Const(v) => *v,
            CachedClosure::XCached { cache, f: _ } => {
                let guard = cache.lock().unwrap();
                if let Some(values) = &*guard {
                    values[xi]
                } else {
                    panic!("XCached: cache not populated");
                }
            }
            CachedClosure::YCached { cache, f: _ } => {
                let guard = cache.lock().unwrap();
                if let Some(values) = &*guard {
                    values[yi]
                } else {
                    panic!("YCached: cache not populated");
                }
            }
            CachedClosure::XY(f) => f(x, y),
        }
    }

    pub fn populate_x(&self, width: usize) {
        if let CachedClosure::XCached { cache, f } = self {
            let mut values = Vec::with_capacity(width);
            for xi in 0..width {
                let x = (xi as f32 / (width - 1) as f32) * 2.0 - 1.0;
                values.push(f(x));
            }
            *cache.lock().unwrap() = Some(values);
        }
    }

    pub fn populate_y(&self, height: usize) {
        if let CachedClosure::YCached { cache, f } = self {
            let mut values = Vec::with_capacity(height);
            for yi in 0..height {
                let y = (yi as f32 / (height - 1) as f32) * 2.0 - 1.0;
                values.push(f(y));
            }
            *cache.lock().unwrap() = Some(values);
        }
    }
}


impl fmt::Debug for CachedClosure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CachedClosure::Const(val) => f.debug_tuple("Const").field(val).finish(),
            CachedClosure::XCached { cache, .. } => {
                let cache_guard = cache.lock().unwrap();
                f.debug_struct("XCached")
                    .field("cache", &*cache_guard)
                    .field("f", &"<closure>")
                    .finish()
            }
            CachedClosure::YCached { cache, .. } => {
                let cache_guard = cache.lock().unwrap();
                f.debug_struct("YCached")
                    .field("cache", &*cache_guard)
                    .field("f", &"<closure>")
                    .finish()
            }
            CachedClosure::XY(_) => write!(f, "XY(<closure>)"),
        }
    }
}

impl fmt::Debug for ClosureTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClosureTree")
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .finish()
    }
}