<?php 
    require __DIR__ . '/part1.php';
    require __DIR__ . '/part2.php';
    
    $inputFile = fopen("input.txt", "r") or die("Unable to open file!");
    $input = fread($inputFile,filesize("input.txt"));
    fclose($inputFile);


    $res1 = part1($input);
    echo "Part 1: $res1<br>";

    $res2 = part2($input);
    echo "Part 2: $res2<br>";
?>