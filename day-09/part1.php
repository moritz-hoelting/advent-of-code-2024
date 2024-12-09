<?php
    function part1($input){
        $chars = str_split($input);
        $disk = [];
        $idx = 0;
        foreach ($chars as $i => $val) {
            $val = intval($val);
            for ($j = 0; $j < $val; $j++) {
                if ($i % 2 == 0) {
                    $disk[$idx] = $i / 2;
                } else {
                    $disk[$idx] = '.';
                }
                $idx++;
            }
        }

        $disksize = count($disk);
        $takeIdx = $disksize - 1;
        $putIdx = 0;
        while ($takeIdx > $putIdx) {
            while ($disk[$takeIdx] === '.') {
                $takeIdx--;
            }
            while ($disk[$putIdx] !== '.') {
                $putIdx++;
            }
            $disk[$putIdx] = $disk[$takeIdx];
            $disk[$takeIdx] = '.';
            $takeIdx--;
            $putIdx++;
        }

        $sum = 0;

        foreach ($disk as $i => $val) {
            if ($val === '.') {
                break;
            }
            $sum += $val * $i;
        }

        return $sum;
    }
?>