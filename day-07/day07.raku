use v6;
use day07;

my $fh = open "input.txt", :r;
my $input = $fh.slurp;
$fh.close;

say sprintf("Part 1: %d", part1 $input);
say sprintf("Part 2: %d", part2 $input);