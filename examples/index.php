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
    // let ;

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

// gere um php com phpdoc
// gere tambem a nova syntax do match case
// entre outras syntax novas para testar meu LSP

/**
 * This is a simple example of a PHP program.
 *
 * @return void
 */
function main() {
    $three = 3;
    $meaning_of_life = $three * 14 + 1;

    echo "Hello, world!\n";
    echo "The meaning of life is...\n";

    // Match case
    $result = match ($meaning_of_life) {
        42 => "The meaning of life is 42",
        default => "...something we cannot know",
    };

    echo $result . "\n";

    echo "However, I can tell you that the factorial of 10 is...\n";
    echo factorial(10) . "\n";
}

// Adicone tambem sintax de if e endif 

/**
 * This is a simple example of a PHP program.
 *
 * @return void
 */
function main() {
    $three = 3;
    $meaning_of_life = $three * 14 + 1;

    echo "Hello, world!\n";
    echo "The meaning of life is...\n";

    // Match case
    $result = match ($meaning_of_life) {
        42 => "The meaning of life is 42",
        default => "...something we cannot know",
    };

    echo $result . "\n";

    echo "However, I can tell you that the factorial of 10 is...\n";
    echo factorial(10) . "\n";
}

main();