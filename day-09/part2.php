<?php
    function part2($input){
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

        $mem = array();
        
        for ($i = $disksize - 1; $i >= 0; $i--) {
            $char = $disk[$i];
            if ($char === '.') {
                continue;
            }

            $end = $i;
            while ($i > 0 && $disk[$i - 1] === $char) {
                $i--;
            }
            $start = $i;

            $len = $end - $start + 1;

            $startIndex = 0;
            foreach ($mem as $l => $s) {
                if ($l > $len) {
                    break;
                }
                $startIndex = $s;
            }
            
            $success = false;
            for ($j = $startIndex; $j < $disksize; $j++) {
                $err = false;
                for ($k = 0; $k < $len; $k++) {
                    if ($j + $k >= $disksize || $disk[$j + $k] !== '.') {
                        $err = true;
                        break;
                    }
                }
                if (!$err && $start > $j) {
                    $success = true;
                    $mem[$len] = $j;
                    for ($k = 0; $k < $len; $k++) {
                        $disk[$j + $k] = $char;
                    }
                    break;
                }
            }
            if ($err) {
                $mem[$len] = $j;
            }
            if ($success) {
                for ($j = $start; $j <= $end; $j++) {
                    $disk[$j] = '.';
                }
            }
        }

        $sum = 0;

        foreach ($disk as $i => $val) {
            if ($val === '.') {
                continue;
            }
            $sum += $val * $i;
        }

        return $sum;
    }
?>