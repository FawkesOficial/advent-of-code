#!/bin/sh

# Put your AoC cookie, choose the a template from the "templates" folder and put them inside ".env"
source .env

function get_puzzle_input() {
    day="${1:-$(date +'%-d')}"
    year="${2:-$(date +'%Y')}"
    response=$(curl --show-error --silent --cookie "session=$AOC_COOKIE" "https://adventofcode.com/$year/day/$day/input") \
        || echo "An error occurrend while downloading the puzzle input."

    case "$response" in
        "404 Not Found")
            echo "Puzzle input for day $day of $year was not found."
            ;;
        "Please don't repeatedly request this endpoint"*)
            echo "[$year] The puzzle input for day $day is not available yet!"
            ;;
        *)
            echo "$response" | tee input.txt && echo && echo "[$year] Successfully downloaded the puzzle input for day $day!"
            ;;
    esac
}

function setup_new_day() {
    cp "templates/$TEMPLATE" "solution.${TEMPLATE##*.}"
    get_puzzle_input "$@"
}

# setup_new_day "$@"
get_puzzle_input "$@"