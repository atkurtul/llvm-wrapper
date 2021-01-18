use std::collections::HashSet;
use std::{collections::HashMap, io::stdin};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Func {
    name: String,
    re: String,
    args: Vec<String>,
}

use inflector::Inflector;
use regex::Regex;
use std::fs::*;
use std::io::*;

fn main() {
    let name = Regex::new(r"^(\w+)").unwrap();
    let arg = Regex::new(r"\((.*?)\)").unwrap();
    let ret = Regex::new(r"\[(.*?)\]").unwrap();
    let mut xsts = HashSet::<String>::new();

    let mut out = vec![];

    let mut funcs = HashMap::<String, Func>::default();
    let mut xargs = HashMap::<String, Vec<String>>::default();
    let mut src = "".to_owned();
    stdin().read_to_string(&mut src).unwrap();
    let src = src.split("\n").collect::<Vec<_>>();
    for src in src {
        if let Some(cap) = name.captures(&src) {
            let cap = cap.get(1).unwrap().as_str().to_owned();
            if cap == "AddInternalizePassWithMustPreservePredicate" {
                continue;
            }
            let mut re = "".to_owned();
            if let Some(cap) = ret.captures(&src) {
                re = cap.get(1).unwrap().as_str().to_owned();
            }
            let mut args = vec![];
            for a in arg.captures_iter(&src) {
                let a = a.get(1).unwrap().as_str().to_owned();
                args.push(a.clone());
            }

            if let Some(o) = args.first() {
                if o.starts_with("LLVM") && !o.ends_with("Predicate"){
                    xargs.entry(o.clone()).or_default().push(cap.clone());
                    xsts.insert(o.trim_end().to_owned());
                }
            }
            let re = if re.is_empty() { "()".to_owned() } else { re };
            if let Some(other) = funcs.insert(
                cap.clone(),
                Func {
                    name: cap.clone(),
                    re,
                    args,
                },
            ) {
                eprintln!("Duplicate {} ", cap);
            }
        }
    }
    fn map_ty(t: &str, i: usize, xsts: &HashSet<String>) -> (String, String) {
        if xsts.get(t).is_some() {
            (format!("{}", &t[4..]), format!("a{}.0", i))
        } else {
            (format!("{}", t), format!("a{}", i))
        }
    }

    for (k, v) in xargs.iter() {
        out.push(format!("impl {} {{", &k[4..]));
        for v in v {
            let func = if let Some(func) = funcs.remove(v) {
                func
            } else {
                eprintln!("Duplicate type {} of {}", k, v);
                continue;
            };

            let (c0, c1): (Vec<_>, Vec<_>) = func
                .args
                .into_iter()
                .skip(1)
                .enumerate()
                .map(|(i, t)| {
                    let (s0, s1) = map_ty(&t, i, &xsts);
                    (format!("a{}: {}", i, s0), s1)
                })
                .unzip();

            if xargs.get(&func.re).is_some() {
                let re = &func.re[4..];
                out.push(format!(
                    "pub fn {}(self, {}) -> {}{{{}( unsafe {{LLVM{}(self.0,{})}})}}",
                    v.to_snake_case(),
                    c0.join(","),
                    re,
                    re,
                    func.name,
                    c1.join(","),
                ));
            } else {
                out.push(format!(
                    "pub fn {}(self, {}) -> {}{{unsafe {{LLVM{}(self.0,{})}}}}",
                    v.to_snake_case(),
                    c0.join(","),
                    func.re,
                    func.name,
                    c1.join(","),
                ));
            }
        }
        out.push(format!("}}"));
    }

    for (v, func) in funcs {
        let (c0, c1): (Vec<_>, Vec<_>) = func
            .args
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                let (s0, s1) = map_ty(&t, i, &xsts);
                (format!("a{}: {}", i, s0), s1)
            })
            .unzip();

        if xargs.get(&func.re).is_some() {
            let re = &func.re[4..];
            out.push(format!(
                "pub fn {}({}) -> {}{{{}( unsafe {{LLVM{}({})}})}}",
                v.to_snake_case(),
                c0.join(","),
                re,
                re,
                func.name,
                c1.join(","),
            ));
        } else {
            out.push(format!(
                "pub fn {}({}) -> {}{{unsafe {{LLVM{}({})}}}}",
                v.to_snake_case(),
                c0.join(","),
                func.re,
                func.name,
                c1.join(","),
            ));
        }
    }

    let mut out2 = vec![];
    let mods = vec![
        "analysis",
        "bit_reader",
        "bit_writer",
        "comdat",
        "core",
        "debuginfo",
        "disassembler",
        "error",
        "error_handling",
        "execution_engine",
        "initialization",
        "ir_reader",
        "linker",
        "object",
        "orc",
        "orc2",
        "prelude",
        "remarks",
        "support",
        "target",
        "target_machine",
        "transforms::aggressive_instcombine",
        "transforms::coroutines",
        "transforms::ipo",
        "transforms::pass_manager_builder",
        "transforms::scalar",
        "transforms::util",
        "transforms::vectorize",
    ];

    out2.push(format!("use llvm_sys::*;"));
    for f in mods.iter() {
        out2.push(format!("use llvm_sys::{}::*;", f));
    }

    for k in xsts {
        out.push(format!(
            "#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]"
        ));
        out.push(format!("pub struct {}(pub {});", &k[4..], k));
    }

    let out = format!("{}{}", out2.join("\n"), out.join("\n"));
    write("./src/lib.rs", out).unwrap();
}
