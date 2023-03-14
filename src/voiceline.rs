
use crate::event::*;
use std::path::*;
use walkdir::*;
use serde::*;

use rand::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Voiceline  {
    pub id : usize,
    sucs : Vec<usize>,
    events : Vec<u32>,
    pub path : String,
    line : String,
    duration : u32,
    format : u16,
    init : bool,
    term : bool
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SoundStorage {
    pub store : Vec<Voiceline>
}

pub fn print_graph(gr : &Vec<Vec<usize>>) {
    for (i, e) in gr.iter().enumerate() {
        print!("{}: ", i);
        for j in e {
            print!("{}, ", j);
        }
        println!("");
    }
}

pub fn generate_voiceline(vls : &SoundStorage, e : &Event) -> String {

    let mut graph : Vec<Vec<usize>> = Vec::new();
    graph.resize(vls.store.len(), vec!{});

    for i in &vls.store {
        if i.events.contains(&e.id) {
            let mut tmp : Vec<usize> = Vec::new();
            tmp.push(0);
            for j in &i.sucs {
                tmp.push(*j);
            }
            graph[i.id] = tmp;
        }
    }

    let mut origins = Vec::new();
    for (i, _e) in graph.iter().enumerate() {
        if vls.store[i].init && vls.store[i].events.contains(&e.id) == true {
            origins.push(i);
        }
    }

    let mut rng = rand::thread_rng();
    let ri = rng.gen_range(0..origins.len());
    let mut cur = origins[ri];
    drop(ri);

    let mut result : String = String::new();

    result.push_str(&vls.store[cur].line);

    loop {

        let r;
        let node_len = graph[cur].len();
        if node_len == 1 || node_len == 0{
            break;
        } else if node_len == 2 {
            r = 1;
        } else {
            r = rng.gen_range(1..node_len);
        }
        result.push_str(&vls.store[graph[cur][r]].line);
        cur = graph[cur][r];
    }

    return result;
}

pub fn store_segments<'a>(file : &std::path::Path) ->
std::result::Result<SoundStorage, Box<dyn std::error::Error>> {
    let f = std::fs::File::open(file)?;
    let vls = SoundStorage { store: serde_yaml::from_reader(f)? };
    Ok(vls)
}
