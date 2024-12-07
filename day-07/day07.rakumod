unit module day07;

sub checkLinePart1($line) {
    if $line ~~ (m/^(\d+)':'([\s+(\d+)]+)$/) {
        my $target = $/.list[0].Int;
        my $nums =  $/.list[1].list[0]».Int;

        for 0..^(2**($nums.elems - 1)) -> $i {
            my $sum = $nums[0];
            for 1..^$nums.elems -> $j {
                if ($i +> ($j - 1)) +& 1 == 0 {
                    $sum += $nums[$j];
                } else {
                    $sum ×= $nums[$j];
                }
            }
            if $target == $sum {
                return $target;
            }
        }
    }
    return 0;
}

sub part1($input) is export {
    return lines($input).map(&checkLinePart1).sum;
}

sub checkLinePart2($line) {
    if $line ~~ (m/^(\d+)':'([\s+(\d+)]+)$/) {
        my $target = $/.list[0].Int;
        my @nums =  $/.list[1].list[0]».Int;

        my $first = @nums.shift;
        if checkLinePart2Rec($target, [$first], @nums) {
            return $target;
        }
    }
    return 0;
}

sub checkLinePart2Rec($target, @results, @remaining) {
    if @remaining.elems == 0 {
        return $target == @results.any;
    }
    my $num = @remaining.shift;
    my @new_results = @results.flatmap({ $_ + $num, $_ × $num, ($_.Str ~ $num.Str).Int }).grep({ $_ ≤ $target });
    return checkLinePart2Rec($target, @new_results, @remaining);
}

sub part2($input) is export {
    return lines($input).map(&checkLinePart2).sum;
}