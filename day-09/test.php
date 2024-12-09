<?php 
    require __DIR__ . '/part1.php';
    require __DIR__ . '/part2.php';

    $sample = '2333133121414131402';

    $res1 = part1($sample);
    echo "Part 1 Test: $res1<br>";
    assert($res1 === 1928, "Expected 1928, got $res1");

    $res2 = part2($sample);
    echo "Part 2 Test: $res2<br>";
    assert($res2 === 2858, "Expected 2858, got $res2");
?>