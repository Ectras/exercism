"""Deletes all but the latest iteration of each exercise and moves it up one folder (e.g. from foo/2/ to foo/)."""

import argparse
from pathlib import Path
import shutil


def get_subdirs(dir: Path) -> list[Path]:
    return [subdir for subdir in dir.iterdir() if subdir.is_dir()]


def is_iteration_dir(path: Path) -> bool:
    return path.name.isdecimal()


def get_iteration(path: Path) -> int:
    return int(path.name)


def move_up_one_level(base_path: Path, relative_child_path: Path):
    assert base_path.is_dir()
    tmp_base_path = base_path.rename(base_path.with_name(base_path.name + "__tmp"))
    new_child_path = tmp_base_path / relative_child_path
    new_child_path.rename(base_path)
    tmp_base_path.rmdir()


def delete_iteration_folders(track_path: Path):
    for exercise_dir in get_subdirs(track_path):
        iteration_dirs = get_subdirs(exercise_dir)

        # Only consider the folder if it has subfolders and all subfolders have numeric names
        is_unprocessed = len(iteration_dirs) > 0 and all(
            is_iteration_dir(dir) for dir in iteration_dirs
        )
        if not is_unprocessed:
            continue

        # Find the index of the latest iteration
        (latest_iteration_index, latest_iteration_path) = max(
            enumerate(iteration_dirs), key=lambda tpl: get_iteration(tpl[1])
        )

        # Delete all other iteration folders
        for i, path in enumerate(iteration_dirs):
            if i == latest_iteration_index:
                continue

            print(f"Deleting {path}")
            shutil.rmtree(path)

        # Move remaining iteration folder up one level
        relative_path = latest_iteration_path.relative_to(exercise_dir)
        print(f"Moving {latest_iteration_path} to {exercise_dir}")
        move_up_one_level(exercise_dir, relative_path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="delete_iterations",
        description="Helper script to delete iteration folders (except the latest) created by the Exercism backup tool",
    )
    parser.add_argument("track", help="The name of the track to clean")

    args = parser.parse_args()
    track_path = Path("solutions") / args.track
    assert track_path.exists()

    delete_iteration_folders(track_path)
