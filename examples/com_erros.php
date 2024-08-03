<?php

// Declaração de variáveis (PHP 5 vs. PHP 8.4)
$var1 = 'Olá, mundo!'; // String simples (PHP 5 e 8.4)
$var2 = "Usando aspas duplas para interpolação: $var1"; // Interpolação de strings (PHP 5 e 8.4)
$var3 = <<<EOT
Uma string heredoc
com múltiplas linhas.
EOT; // Heredoc (PHP 5 e 8.4)

// Tipos de dados (PHP 5 vs. PHP 8.4)
$inteiro = 42; // Inteiro (PHP 5 e 8.4)
$float = 3.14; // Float (PHP 5 e 8.4)
$booleano = true; // Booleano (PHP 5 e 8.4)
$array = [1, 2, 3]; // Array (PHP 5 e 8.4)
$objeto = new stdClass(); // Objeto (PHP 5 e 8.4)

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
$resultado = $objeto?->propriedade ?? 'Valor padrão';

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
$texto = match ($cor) {
    Cor::Vermelho => 'A cor é vermelho',
    Cor::Verde => 'A cor é verde',
    Cor::Azul => 'A cor é azul',
    default => 'Cor desconhecida'
};

// Named arguments (PHP 8.0+)
function saudacao(string $nome, string $saudação = 'Olá'): string {
    return "$saudação, $nome!";
}

// Constructor property promotion (PHP 8.0+)
class Carro {
    public function __construct(public string $modelo, public int $ano) {}
}

// Arrow functions (PHP 7.4+)
$dobrar = fn($numero) => $numero * 2;

// Erros de sintaxe introduzidos

// Falta de ponto e vírgula
$variavel_sem_ponto_e_virgula = 'Este código está faltando um ponto e vírgula'

// Chaves não fechadas
function funcaoComChaveNaoFechada($param) {
    echo $param;

// Parênteses não fechados
echo("Este é um teste de parênteses não fechados";

// String não fechada
$variavel_string_nao_fechada = 'Esta string não está fechada;

// Sintaxe incorreta em enum
enum Fruta {
    Maca
    Banana,
    Laranja
}

// Função com parâmetros incorretos
function somar($a, $b {
    return $a + $b;
}

// Erro em operador ternário
$ternario_errado = $condicao ? 'valor true' : 'valor false';

// Uso incorreto do operador de atribuição
$variavel =+ 10;

// Classe com erro de sintaxe
class ClasseComErro {
    public function metodoIncorreto()
        echo "Erro de sintaxe";
    }
}

// Match expression com erro
$texto_errado = match ($cor) {
    Cor::Vermelho => 'A cor é vermelho'
    Cor::Verde => 'A cor é verde',
    Cor::Azul => 'A cor é azul',
    default => 'Cor desconhecida'
};

// Funções e classes adicionais com erros de sintaxe
function subtrai($a $b) {
    return $a - $b
}

class Animal {
    public function som() {
        echo "Som de animal"
    }

    public function andar($distancia {
        echo "Animal andou $distancia metros";
    }
}
?>
