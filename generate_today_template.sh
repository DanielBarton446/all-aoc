year=$(date +"%Y")
day=$(date +"%-d")

root_dir=$(pwd $0)
script_dir="$root_dir/$(dirname $0)"
echo $script_dir

base_dir="$script_dir/aoc_$year/day_$day"
input_dir="$base_dir/input"

mkdir -p $base_dir
mkdir -p $input_dir
curl_status=curl "https://adventofcode.com/$year/day/$day/input" > "$input_dir/input.txt"

if [ $curl_status -ne 0 ]; then
    echo "Failed to download input"
    exit 1
fi
