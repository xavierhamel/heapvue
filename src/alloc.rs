use std::{
    process,
    io::{self, BufRead},
    collections::{HashMap, hash_map},
    thread,
    sync::mpsc,
    time,
    ops, fmt,
};

use crate::components::memory::{self, MAX_LINE_COUNT, BYTES_PER_LINE};

#[derive(PartialEq)]
pub enum Event {
    Alloc {
        ptr: u64,
        size: u64,
        identifier: String,
    },
    Free {
        ptr: u64,
        identifier: String,
    },
    Corrupted {
        ptr: u64
    }
}

impl Event {
    pub fn try_from_line(line: &str) -> Option<Self> {
        let (typ, data) = line.split_once(':')?;
        let mut parts = data.split(',');
        let ptr = u64::from_str_radix(parts.next()?, 16).ok()?;
        match typ {
            "m" => {
                let size = u64::from_str_radix(parts.next()?, 16).ok()?;
                let identifier = parts.collect::<Vec<&str>>().join(",");
                Some(Self::Alloc { ptr, size, identifier })
            }
            "f" => {
                let identifier = parts.collect::<Vec<&str>>().join(",");
                Some(Self::Free { ptr, identifier })
            }
            "c" => {
                Some(Self::Corrupted { ptr })
            }
            _ => None
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ChunkState {
    Ok,
    AlreadyUsed,
    AlreadyFreed,
    Corrupted,
}

impl ChunkState {
    pub fn to_color(&self) -> egui::Color32 {
        match self {
            ChunkState::Ok => memory::COLOR_USED,
            ChunkState::AlreadyUsed => memory::COLOR_ALREADY_USED,
            ChunkState::Corrupted => memory::COLOR_CORRUPTED,
            ChunkState::AlreadyFreed => memory::COLOR_CORRUPTED,
        }
    }
}

impl fmt::Display for ChunkState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkState::Ok => write!(f, "ok"),
            ChunkState::AlreadyUsed => write!(f, "already used"),
            ChunkState::Corrupted => write!(f, "corrupted"),
            ChunkState::AlreadyFreed => write!(f, "already freed"),
        }
    }
}

#[derive(Clone)]
pub struct ChunkLines {
    pub start: usize,
    pub count: usize,
}

impl ChunkLines {
    pub fn new(ptr: u64, size: u64) -> Self {
        let start = ptr / BYTES_PER_LINE;
        let start_x = ptr % BYTES_PER_LINE;
        let count = if size < BYTES_PER_LINE - start_x {
            1
        } else {
            (size - (BYTES_PER_LINE - start_x)) / BYTES_PER_LINE + 2
        };

        Self {
            start: start as usize,
            count: count as usize
        }
    }

    pub fn range(&self) -> ops::Range<usize> {
        self.start..self.start + self.count
    }
}

#[derive(Clone)]
pub struct Chunk {
    pub lines: ChunkLines,
    pub ptr: u64,
    pub size: u64,
    pub identifier: String,
    pub state: ChunkState,
}

impl Chunk {
    pub fn new(ptr: u64, size: u64, identifier: String) -> Self {
        Self {
            lines: ChunkLines::new(ptr, size),
            ptr,
            size,
            identifier,
            state: ChunkState::Ok,
        }
    }

    pub fn placeholder(ptr: u64, state: ChunkState) -> Self {
        let size = 100;
        Self {
            lines: ChunkLines::new(ptr, size),
            ptr,
            size,
            identifier: String::new(),
            state,
        }
    }

    fn is_solid(&self) -> bool {
        self.state != ChunkState::AlreadyFreed && self.state != ChunkState::Corrupted
    }

    pub fn is_colliding(&self, other: &Chunk) -> bool {
        (self.is_solid() && other.is_solid())
            && ((self.ptr >= other.ptr && self.ptr < other.ptr + other.size)
                || (other.ptr >= self.ptr && other.ptr < self.ptr + self.size))
    }

    pub fn set_state(&mut self, state: ChunkState) {
        self.state = state;
    }
}

pub struct Chunks {
    rx: mpsc::Receiver<Event>,
    chunks: HashMap<u64, Chunk>,
    line_lookup: [Vec<u64>; MAX_LINE_COUNT as usize],
    pub do_advance: bool,
}

impl Chunks {
    pub fn new(mut command: process::Command, context: egui::Context) -> Self {
        let (tx, rx) = mpsc::channel::<Event>();
        thread::spawn(move || {
            let mut process = command
                .stdout(process::Stdio::piped())
                .spawn()
                .unwrap();
            let mut stdout = io::BufReader::new(process.stdout.take().unwrap());
            loop {
                let mut input = String::new();
                if let Ok(_) = stdout.read_line(&mut input) {
                    if let Some(event) = Event::try_from_line(&input) {
                        let _ = tx.send(event);
                        context.request_repaint();
                    }
                }
            }
        });
        Self {
            rx,
            chunks: HashMap::default(),
            line_lookup: std::array::from_fn(|_| Vec::new()),
            do_advance: true,
        }
    }

    pub fn update(&mut self) -> bool {
        let mut events = Vec::new();
        if !self.do_advance {
            return false;
        }
        while let Ok(event) = self.rx.recv_timeout(time::Duration::from_secs(0)) {
            events.push(event);
        }
        let did_update = events.len() > 0;
        for event in events {
            match event {
                Event::Free { ptr, .. } => self.free(ptr),
                Event::Alloc { ptr, size, identifier } => self.alloc(ptr, size, identifier),
                Event::Corrupted { ptr } => { self.corrupted(ptr); }
            }
        }
        did_update
    }

    pub fn get(&self, ptr: u64) -> Option<&Chunk> {
        self.chunks.get(&ptr)
    }

    pub fn iter(&self) -> hash_map::Iter<'_, u64, Chunk> {
        self.chunks.iter()
    }

    fn is_chunk_colliding(&self, chunk: &Chunk) -> bool {
        chunk.lines.range()
            .into_iter()
            .any(|idx| {
                self.line_lookup[idx]
                    .iter()
                    .any(|ptr| {
                        self.chunks.get(ptr)
                            .map_or(false, |other| chunk.is_colliding(other))
                    })
            })
    }

    fn insert(&mut self, ptr: u64, chunk: Chunk) {
        chunk.lines.range()
            .into_iter()
            .for_each(|idx| {
                self.line_lookup[idx].push(ptr);
            });
        self.chunks.insert(ptr, chunk);
    }

    fn alloc(&mut self, ptr: u64, size: u64, identifier: String) {
        let mut chunk = Chunk::new(ptr, size, identifier);
        if self.is_chunk_colliding(&chunk) {
            chunk.set_state(ChunkState::AlreadyUsed);
        }
        self.insert(ptr, chunk);
    }

    fn free(&mut self, ptr: u64) {
        if let Some(chunk) = self.chunks.get(&ptr) {
            chunk.lines.range()
                .into_iter()
                .for_each(|idx| {
                    self.line_lookup[idx].retain(|&p| p != ptr);
                });
        } else {
            self.insert(ptr, Chunk::placeholder(ptr, ChunkState::AlreadyFreed));
        }
        self.chunks.remove(&ptr);
    }

    fn corrupted(&mut self, ptr: u64) {
        self.do_advance = false;
        if let Some(chunk) = self.chunks.get_mut(&ptr) {
            chunk.set_state(ChunkState::Corrupted);
        } else {
            self.insert(ptr, Chunk::placeholder(ptr, ChunkState::Corrupted));
        }
    }
}
