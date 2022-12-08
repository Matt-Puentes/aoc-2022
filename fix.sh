# [ -d "day_$1" ] && echo "Directory day_$1 exists." && exit

#mv day_0$1/src/main.rs day_0$1/src/solutions.rs
#mv day_0$1/src/parse_args.rs day_0$1/src/main.rs
cp day_10/example.txt day_$1/example.txt
cp day_10/input.txt day_$1/input.txt
#sed -i '' "s/main.rs/solutions.rs/g" day_0$1/Cargo.toml
#sed -i '' "s/day 0/day $1/g" day_$1/src/main.rs
#code day_$1
#cd day_$1
