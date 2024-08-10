<?php

// Declaração de variáveis (PHP 5 vs. PHP 8.4)
$var1 = 'Olá, mundo!'; // String simples (PHP 5 e 8.4)
$var2 = "Usando aspas duplas para interpolação: $var1"; // Interpolação de strings (PHP 5 e 8.4)
// $var3 = <<<EOT
// Uma string heredoc
// com múltiplas linhas.
// EOT;

// Comentario

# This is a comment 

// Tipos de dados (PHP 5 vs. PHP 8.4)
$inteiro = 42; // Inteiro (PHP 5 e 8.4)
$float = 3.14; // Float (PHP 5 e 8.4)
$booleano = true; // Booleano (PHP 5 e 8.4)
$array = [1, 2, 3]; // Array (PHP 5 e 8.4)
$objeto = new stdClass(); // Objeto (PHP 5 e 8.4)
$objeto = new Http("tyy", "cu", "prinquito"); // Objeto (PHP 5 e 8.4)

// Tipos escalares (PHP 8.0+)
$string = "Uma string";
$int = 42;
$float = 3.14;
$bool = true;

// Tipos compostos (PHP 8.0+)
$array_tipado = [1, 2, 3];
$objeto_tipado = new stdClass();

// Union types (PHP 8.0+)

function soma(int|float $a, int|float $b): int|float {
    return $a + $b;
}

// Nullsafe operator (PHP 8.0+)
// $resultado = $objeto?->propriedade ?? 'Valor padrão';

// Atributos (PHP 8.1+)
class Pessoa {
    public string $nome;
    public int $idade;
}

// Enums (PHP 8.1+)
enum Cor {
    Vermelho,
    Verde,
    Azul
}

// Match expression (PHP 8.0+)
$cor = Cor::Vermelho;
// $texto = match ($cor) {
//    Cor::Vermelho => 'A cor é vermelho',
//    Cor::Verde => 'A cor é verde',
//    Cor::Azul => 'A cor é azul',
//    default => 'Cor desconhecida'
// };

// Named arguments (PHP 8.0+)
function saudacao(string $nome, string $saudacao = 'Olá'): string {
    return "$saudação, $nome!";
}

// Constructor property promotion (PHP 8.0+)
class Carro {
    public function __construct(public string $modelo, public int $ano) {}
}

// Arrow functions (PHP 7.4+)
$dobrar = fn($numero) => $numero * 2;

// Funções e classes adicionais sem erros de sintaxe
function multiplica(int|float $a, int|float $b): int|float {
    return $a * $b;
}

class Livro {
    public string $titulo;
    public string $autor;

    public function __construct(string $titulo, string $autor) {
        $this->titulo = $titulo;
        $this->autor = $autor;
    }

    public function getDescricao(): string {
        return "$this->titulo por $this->autor";
    }
}