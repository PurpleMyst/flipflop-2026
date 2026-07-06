# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "cyclopts",
#     "python-dotenv",
#     "requests",
#     "termcolor",
#     "tomlkit",
#     "tabulate",
# ]
# ///
import re
import shlex
import subprocess
import sys
import typing as t
from contextlib import chdir
from datetime import datetime
from functools import partial, wraps
from os import environ
from pathlib import Path

import tomlkit as toml
from cyclopts import App
from dotenv import load_dotenv
from termcolor import colored as c


app = App()
cb = partial(c, attrs=["bold"])

MAIN = """\
fn main() {{
    let (part1, part2, part3) = {crate}::solve();
    println!("{{part1}}");
    println!("{{part2}}");
    println!("{{part3}}");
}}\
"""

LIB = """\
use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    "TODO"
}

#[inline]
pub fn solve_part2() -> impl Display {
    "TODO"
}

#[inline]
pub fn solve_part3() -> impl Display {
    "TODO"
}\
"""

WORKSPACE_MANIFEST_PATH = Path(__file__).parent / "Cargo.toml"

TASKS_METADATA = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())["workspace"].get(
    "metadata", {}
).get("tasks", {})
PROBLEM_NAME = TASKS_METADATA.get("problem_name", "problem")
PROBLEM_DIGITS = TASKS_METADATA.get("problem_digits", 2)


load_dotenv()


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


def in_root_dir(f):
    @wraps(f)
    def inner(*args, **kwargs):
        with chdir(Path(__file__).parent):
            return f(*args, **kwargs)

    return inner


def find_next_problem_number() -> int:
    existing_numbers = []
    for path in Path().glob(f"{PROBLEM_NAME}*"):
        match = re.match(rf"{PROBLEM_NAME}(\d+)", path.name)
        if match:
            existing_numbers.append(int(match.group(1)))
    next_number = 1
    while next_number in existing_numbers:
        next_number += 1
    return next_number


@app.command(name="start_solve", alias="ss")
@in_root_dir
def start_solve(num: int | None = None) -> None:
    "Start solving a problem."

    if num is None:
        num = find_next_problem_number()

    crate = f"{PROBLEM_NAME}{int(num):0{PROBLEM_DIGITS}d}"
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
    (src / "main.rs").write_text(MAIN.format(crate=crate), newline="\n")
    (src / "lib.rs").write_text(LIB, newline="\n")

    benches = Path("benchmark", "benches")
    add_line(benches / "criterion.rs", f"    {crate},")

    run(("git", "add", crate))


@app.command(name="set_baseline", alias="sb")
@in_root_dir
def set_baseline(pattern: str = ".") -> None:
    "Run a criterion benchmark, setting its results as the new baseline."
    is_dirty = (
        run(("git", "status", "--porcelain"), capture_output=True, text=True).stdout.strip() != ""
    )
    if is_dirty:
        print(
            cb(
                "You have uncommitted changes. Please commit or stash them before setting a baseline.",
                "red",
            )
        )
        sys.exit(1)
    name = run(
        ("git", "rev-parse", "--short", "HEAD"), capture_output=True, text=True
    ).stdout.strip()
    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            pattern,
            "--save-baseline",
            name,
            "--verbose",
        )
    )


@app.command(name="compare", alias="cmp")
@in_root_dir
def compare(pattern: str = ".") -> None:
    "Run a criterion benchmark, comparing its results to the previous commit if there are no uncommitted changes, or to the current commit otherwise."
    is_dirty = (
        run(("git", "status", "--porcelain"), capture_output=True, text=True).stdout.strip() != ""
    )
    if is_dirty:
        name = run(
            ("git", "rev-parse", "--short", "HEAD"), capture_output=True, text=True
        ).stdout.strip()
    else:
        name = run(
            ("git", "rev-parse", "--short", "HEAD~1"), capture_output=True, text=True
        ).stdout.strip()

    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            pattern,
            "--baseline",
            name,
            "--verbose",
        )
    )


@app.command(name="compare_by_stashing", alias="cmp-stash")
@in_root_dir
def compare_by_stashing(pattern: str = ".") -> None:
    "Stash the current changes, set the baseline and then compare the new changes."
    run(("git", "stash", "push", "-m", "Stashing for benchmarking"))
    set_baseline(pattern=pattern)
    run(("git", "stash", "pop"))
    compare(pattern=pattern)


@app.command(name="measure_completion_time", alias="mct")
@in_root_dir
def measure_completion_time() -> None:
    "Measure completion time for all problems."
    from tabulate import tabulate

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())

    table = []
    for problem in Path().glob(f"{PROBLEM_NAME}*"):
        metadata = manifest["workspace"].get("metadata", {}).get(problem.name, {})  # type: ignore
        start_time = metadata.get("start_time")
        end_time = metadata.get("completion_time")
        if start_time is None or end_time is None:
            table.append((problem.name, "N/A"))
            continue
        completion_time = end_time - start_time
        table.append((problem.name, str(completion_time)))
    print(tabulate(table, headers=[PROBLEM_NAME.title(), "Completion Time"], tablefmt="fancy_grid"))


@app.command(name="set_completion_time", alias="sct")
def set_completion_time() -> None:
    "Set the completion time for the problem you're currently in."

    problem = Path.cwd().resolve().name
    if not problem.startswith(PROBLEM_NAME):
        print(cb(f"Not in a {PROBLEM_NAME} directory.", "red"))
        return

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    metadata = manifest["workspace"].setdefault("metadata", {})  # type: ignore
    problem_metadata = metadata.setdefault(problem, {})
    if "completion_time" in problem_metadata:
        print(cb("Completion time is already set.", "yellow"))
        return
    problem_metadata["completion_time"] = datetime.now()

    with WORKSPACE_MANIFEST_PATH.open("w") as manifest_f:
        toml.dump(manifest, manifest_f)


def main() -> None:
    environ["RUSTFLAGS"] = "-C target-cpu=native"
    app()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Bye!")
