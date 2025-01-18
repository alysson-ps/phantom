<?php
    // Comentário de uma linha
    /* Comentário de várias linhas */

    namespace App\Baseline;

    class Pessoa {
        public $nome;
        private $idade;

        public function __construct($nome, $idade) {
            $this->nome = $nome;
            $this->idade = $idade;
        }

        public function saudacao() {
            echo "Olá, " . $this->nome;
        }
    }
?>