# P1337 — instrumentação pré-freeze

Na primeira execução bilateral pré-C, a serialização do recibo completo excedeu
ARG_MAX ao passar o patch como argumento ao processo apply_patch. A execução
terminou antes de criar o arquivo de resultados e antes de qualquer freeze.
Não se atribui RED, GREEN ou classificação a essa tentativa sem recibo.

Correção de transporte do runner ainda não congelado: enviar o mesmo patch
por stdin de apply_patch. Casos e políticas não mudaram. A execução completa
é repetida para gerar a evidência rastreável. Não é falha nem mutante do produto.
