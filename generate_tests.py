#!/usr/bin/python3

import re

intervals = [
    "]0.,0.[",  # Empty set
    "]-inf,+inf[",  # Infinity set
    "[0., 10.]",
    "[0., 10.[",
    "]0., 10.[",
    "]0., 10.]",
    "[10., 20.]",
    "[10., 20.[",
    "]10., 20.[",
    "]10., 20.]",
    "[5., 7.]",
    "[5., 7.[",
    "]5., 7.[",
    "]5., 7.]",
    "[5., 15.]",
    "[5., 15.[",
    "]5., 15.[",
    "]5., 15.]",
    "[15., 20.]",
    "[15., 20.[",
    "]15., 20.[",
    "]15., 20.]",
]

rpl = {
    re.compile(r"\]-inf"): "Unbound",
    re.compile(r"\+inf\["): "Unbound",
    re.compile(r"(\d+\.)\]"): r"Closed(\1)",
    re.compile(r"(\d+\.)\["): r"Open(\1)",
    re.compile(r"\](\d+\.)"): r"Open(\1)",
    re.compile(r"\[(\d+\.)"): r"Closed(\1)",
}

rust_def = []

for a in intervals:
    for regexp, replace in rpl.items():
        a = re.sub(regexp, replace, a)
    rust_def.append(a)

function = r"""
    let a = Interval::new({a});
    let b =  Interval::new({b});

    let c = a.union(b);
    println!("{{a}} U {{b}} = {{c}}");
"""

print(
    """
use interval::*;

fn main() {"""
)

i = 1
for a in rust_def:
    for b in rust_def:
        print(function.format(i=i, a=a, b=b))
        i += 1

print("")
print("}")
