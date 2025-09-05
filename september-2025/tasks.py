# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "typer",
#     "tomlkit",
#     "termcolor",
# ]
# ///
import os
import typing as t
import subprocess
import shlex
from pathlib import Path
from functools import partial

import typer
import tomlkit as toml
from datetime import datetime

from termcolor import colored as c

app = typer.Typer(context_settings={"help_option_names": ["-h", "--help"]})

cb = partial(c, attrs=["bold"])

PROBLEM_NAME = "problem-"

MAIN = """\
fn main() {{
    let output = {crate}::solve();
    println!("{{output}}");
}}\
"""

LIB = """\
use std::fmt::Display;

use common::*;

#[inline]
pub fn solve() -> impl Display {
    cases(
        include_str!("input.txt")
        .lines()
        .skip(1)
        .map(|line| {
            "TODO"
        })
     )
}\
"""

WORKSPACE_MANIFEST_PATH = Path(__file__).parent / "Cargo.toml"

def run(cmd: t.Sequence[str | Path], /, **kwargs) -> subprocess.CompletedProcess:
    check = kwargs.pop("check", True)
    print(
        cb("$", "green"),
        shlex.join(map(str, cmd)),
        c(f"(w/ options {kwargs})", "green") if kwargs else "",
    )
    proc = subprocess.run(cmd, **kwargs)
    if check and proc.returncode != 0:
        print(cb("Failed.", "red"))
        sys.exit(proc.returncode)
    return proc


def add_line(p: Path, l: str) -> None:
    ls = p.read_text().splitlines()
    ls.insert(-1, l)
    if ls[-1] != "":
        # add or keep trailing newline
        ls.append("")
    p.write_text("\n".join(ls), newline="\n")


@app.command()
def start_solve(problem_char: str) -> None:
    """Start solving a problem identified by a single character."""
    os.chdir(Path(__file__).parent)
    typer.echo(f"Starting to solve problem: {problem_char}")

    crate = f"{PROBLEM_NAME}{problem_char.lower()}"
    crate_path = Path(crate)

    if crate_path.exists():
        print(f"{crate} already exists.")
        return

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    if crate not in manifest["workspace"]["members"]:  # type: ignore
        manifest["workspace"]["members"].append(crate)  # type: ignore

    metadata = manifest["workspace"].setdefault("metadata", {})  # type: ignore
    metadata[crate] = {"start_time": datetime.now()}

    with WORKSPACE_MANIFEST_PATH.open("w") as manifest_f:
        toml.dump(manifest, manifest_f)

    run(("cargo", "new", "--bin", crate))
    run(
        (
            "cargo",
            "add",
            "--manifest-path",
            "benchmark/Cargo.toml",
            "--path",
            crate,
            crate,
        )
    )

    src = crate_path / "src"
    (src / "main.rs").write_text(MAIN.format(crate=crate.replace("-", "_")), newline="\n")
    (src / "lib.rs").write_text(LIB, newline="\n")

    run(
        (
            "cargo",
            "add",
            "--manifest-path",
            f"{crate}/Cargo.toml",
            "--path",
            Path(__file__).parent / "common",
            "common",
        )
    )

    benches = Path("benchmark", "benches")
    add_line(benches / "criterion.rs", f"    {crate},")

    run(("git", "add", crate))

@app.command()
def save_output() -> None:
    "Run problem solution and save output to output.txt"
    proc = run(("cargo", "run", "--release"), stdout=subprocess.PIPE)
    output = proc.stdout.decode().strip()
    (Path("output.txt")).write_text(output, newline="\n")


@app.command()
def measure_completion_time() -> None:
    "Measure completion time for all problems."
    from tabulate import tabulate

    os.chdir(Path(__file__).parent)

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())

    table = []
    for problem in Path().glob(f"{PROBLEM_NAME}*"):
        metadata = manifest["workspace"].get("metadata", {}).get(problem.name, {})  # type: ignore
        start_time = metadata.get("start_time")
        end_time = metadata.get("completion_time")
        src = problem / "src"
        if start_time is None:
            start_time = datetime.fromtimestamp((src / "input.txt").stat().st_ctime)
        if end_time is None:
            end_time = datetime.fromtimestamp(max(f.stat().st_mtime for f in src.glob("**/*.rs")))
        completion_time = end_time - start_time
        table.append((problem.name, str(completion_time)))
    print(tabulate(table, headers=[PROBLEM_NAME.title(), "Completion Time"], tablefmt="fancy_grid"))

@app.command()
def set_completion_time() -> None:
    "Set the completion time for the problem you're currently in."

    problem = Path.cwd().resolve().name
    if not problem.startswith(PROBLEM_NAME):
        print(cb(f"Not in a {PROBLEM_NAME} directory.", "red"))
        return

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    metadata = manifest["workspace"].setdefault("metadata", {})  # type: ignore
    metadata.setdefault(problem, {})["completion_time"] = datetime.now()

    with WORKSPACE_MANIFEST_PATH.open("w") as manifest_f:
        toml.dump(manifest, manifest_f)

app.command("ss")(start_solve)
app.command("sct")(set_completion_time)
app.command("mct")(measure_completion_time)

if __name__ == "__main__":
    app()
