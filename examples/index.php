<?php

class Person {
    public $name;
    public $age;
}

function factorial($x) {
    // Conditionals are supported!
    if ($x == 0) {
        return 1;
    } else {
        return $x * factorial($x - 1);
    }
}

// The main function
function main() {
    $three = 3;
    $meaning_of_life = $three * 14 + 1;

    echo "Hello, world!\n";
    echo "The meaning of life is...\n";

    if ($meaning_of_life == 42) {
        echo $meaning_of_life . "\n";
    } else {
        echo "...something we cannot know\n";

        echo "However, I can tell you that the factorial of 10 is...\n";
        // Function calling
        echo factorial(10) . "\n";
    }
}

main();