#!/bin/bash

# Define o número de execuções concorrentes
num_execucoes_concorrentes=30

# Define o número total de execuções que deseja realizar
total_execucoes=10000

# Crie uma sequência de números de 1 a total_execucoes
seq $total_execucoes | xargs -n 1 -P $num_execucoes_concorrentes -I {} sh -c './vote.sh'
