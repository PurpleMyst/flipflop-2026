# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "browser-cookie3",
#     "cyclopts",
#     "httpx",
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

TASKS_METADATA = (
    toml.parse(WORKSPACE_MANIFEST_PATH.read_text())["workspace"]
    .get("metadata", {})
    .get("tasks", {})
)
PROBLEM_NAME = TASKS_METADATA.get("problem_name", "problem")
PROBLEM_DIGITS = TASKS_METADATA.get("problem_digits", 2)
YEAR = TASKS_METADATA["year"]
FLIPFLOP_HOST = "flipflop.slome.org"


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


def flipflop_session() -> str:
    import browser_cookie3

    cookies = browser_cookie3.firefox(domain_name=FLIPFLOP_HOST)
    for cookie in cookies:
        if cookie.name == "PHPSESSID":
            return cookie.value
    raise RuntimeError(f"No PHPSESSID cookie found for {FLIPFLOP_HOST} in Firefox.")


def fetch_input(num: int) -> bytes:
    import httpx

    url = f"https://{FLIPFLOP_HOST}/{YEAR}/{num}/input"
    response = httpx.get(
        url,
        cookies={"PHPSESSID": flipflop_session()},
        headers={
            "Accept": "text/plain,*/*",
            "Referer": f"https://{FLIPFLOP_HOST}/{YEAR}/{num}",
        },
    )
    content = response.raise_for_status().content
    if not content or content.startswith(b"You must be logged in"):
        raise RuntimeError(
            "No input could be fetched with the Firefox PHPSESSID cookie."
        )
    return content


def current_problem_number() -> int:
    cwd = Path.cwd().resolve()
    root = WORKSPACE_MANIFEST_PATH.parent.resolve()
    for path in (cwd, *cwd.parents):
        match = re.fullmatch(rf"{re.escape(PROBLEM_NAME)}(\d+)", path.name)
        if match:
            return int(match.group(1))
        if path == root:
            break
    print(
        cb(
            f"Not in a {PROBLEM_NAME} directory and no problem number was provided.",
            "red",
        )
    )
    sys.exit(1)


def problem_crate(num: int) -> str:
    return f"{PROBLEM_NAME}{int(num):0{PROBLEM_DIGITS}d}"


def run_solution(num: int) -> list[str]:
    proc = run(
        ("cargo", "run", "--quiet", "-p", problem_crate(num)),
        cwd=WORKSPACE_MANIFEST_PATH.parent,
        stdout=subprocess.PIPE,
        text=True,
    )
    return proc.stdout.splitlines()


def infer_part(answers: list[str]) -> int:
    candidates = [
        part
        for part, answer in enumerate(answers[:3], start=1)
        if answer.strip() != "TODO"
    ]
    if not candidates:
        print(
            cb(
                "Could not infer a part: no non-TODO answer lines were produced.",
                "red",
            )
        )
        sys.exit(1)
    return candidates[-1]


def post_submission(num: int, part: int, answer: str) -> str:
    import httpx

    url = f"https://{FLIPFLOP_HOST}/{YEAR}/{num}/{part}/submit"
    response = httpx.post(
        url,
        data={"answer": answer},
        cookies={"PHPSESSID": flipflop_session()},
        headers={
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            "Origin": f"https://{FLIPFLOP_HOST}",
            "Referer": f"https://{FLIPFLOP_HOST}/{YEAR}/{num}",
        },
    )
    return response.raise_for_status().text


def parse_submission_result(html: str) -> tuple[str, str | None]:
    rank_match = re.search(r"You are ranked\s+(\d+)", html)
    rank = rank_match.group(1) if rank_match else None
    if "That is correct!" in html:
        return "correct", rank
    if "This answer is incorrect!" in html or "Your answer is incorrect." in html:
        return "incorrect", rank
    return "unknown", rank


@app.command(name="start_solve", alias="ss")
@in_root_dir
def start_solve(num: int | None = None) -> None:
    "Start solving a problem."

    if num is None:
        num = find_next_problem_number()

    crate = problem_crate(num)
    crate_path = Path(crate)

    if crate_path.exists():
        print(f"{crate} already exists.")
        return

    input_txt = fetch_input(num)

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    if crate not in manifest["workspace"]["members"]:
        manifest["workspace"]["members"].append(crate)

    metadata = manifest["workspace"].setdefault("metadata", {})
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
    (src / "input.txt").write_bytes(input_txt)

    benches = Path("benchmark", "benches")
    add_line(benches / "criterion.rs", f"    {crate},")

    run(("git", "add", crate))


@app.command(name="submit", alias="s")
def submit(
    part: int | None = None,
    answer: str | None = None,
    num: int | None = None,
) -> None:
    "Submit an answer for a problem part."
    if num is None:
        num = current_problem_number()

    answers: list[str] = []
    if part is None or answer is None:
        answers = run_solution(num)

    if part is None:
        part = infer_part(answers)
        print(f"Inferred part {part}.")

    if not 1 <= part <= 3:
        print(cb("Part must be 1, 2, or 3.", "red"))
        sys.exit(1)

    if answer is None:
        if len(answers) < part:
            print(cb(f"Solver produced only {len(answers)} answer lines.", "red"))
            sys.exit(1)
        answer = answers[part - 1]
        if answer.strip() == "TODO":
            print(cb(f"Part {part} still prints TODO.", "red"))
            sys.exit(1)

    crate = problem_crate(num)
    print(f"Submitting {crate} part {part}.")
    result, rank = parse_submission_result(post_submission(num, part, answer))

    if result == "correct":
        print(cb("Correct.", "green"))
        if rank is not None:
            print(f"Rank: {rank}")
        if part == 3:
            set_completion_time_for(crate)
        return

    if result == "incorrect":
        print(cb("Incorrect.", "red"))
        sys.exit(1)

    print(cb("Unknown submission response.", "yellow"))
    sys.exit(1)


@app.command(name="set_baseline", alias="sb")
@in_root_dir
def set_baseline(pattern: str = ".") -> None:
    "Run a criterion benchmark, setting its results as the new baseline."
    is_dirty = (
        run(
            ("git", "status", "--porcelain"), capture_output=True, text=True
        ).stdout.strip()
        != ""
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
        run(
            ("git", "status", "--porcelain"), capture_output=True, text=True
        ).stdout.strip()
        != ""
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
        metadata = manifest["workspace"].get("metadata", {}).get(problem.name, {})
        start_time = metadata.get("start_time")
        end_time = metadata.get("completion_time")
        if start_time is None or end_time is None:
            table.append((problem.name, "N/A"))
            continue
        completion_time = end_time - start_time
        table.append((problem.name, str(completion_time)))
    print(
        tabulate(
            table,
            headers=[PROBLEM_NAME.title(), "Completion Time"],
            tablefmt="fancy_grid",
        )
    )


def set_completion_time_for(problem: str) -> None:
    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    metadata = manifest["workspace"].setdefault("metadata", {})
    problem_metadata = metadata.setdefault(problem, {})
    if "completion_time" in problem_metadata:
        print(cb("Completion time is already set.", "yellow"))
        return
    problem_metadata["completion_time"] = datetime.now()

    with WORKSPACE_MANIFEST_PATH.open("w") as manifest_f:
        toml.dump(manifest, manifest_f)


@app.command(name="set_completion_time", alias="sct")
def set_completion_time() -> None:
    "Set the completion time for the problem you're currently in."

    problem = Path.cwd().resolve().name
    if not problem.startswith(PROBLEM_NAME):
        print(cb(f"Not in a {PROBLEM_NAME} directory.", "red"))
        return

    set_completion_time_for(problem)


def main() -> None:
    environ["RUSTFLAGS"] = "-C target-cpu=native"
    app()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Bye!")
